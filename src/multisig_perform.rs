use crate::{
    action::Action,
    user_role::UserRole,
};

elrond_wasm::imports!();

/// Gas required to finsh transaction after transfer-execute.
const PERFORM_ACTION_FINISH_GAS: u64 = 300_000;

fn usize_add_isize(value: &mut usize, delta: isize) {
    *value = (*value as isize + delta) as usize;
}

/// Contains all events that can be emitted by the contract.
#[elrond_wasm::module]
pub trait MultisigPerformModule: crate::multisig_state::MultisigStateModule {
    fn gas_for_transfer_exec(&self) -> u64 {
        let gas_left = self.blockchain().get_gas_left();
        if gas_left <= PERFORM_ACTION_FINISH_GAS {
            Self::Api::error_api_impl().signal_error(b"insufficient gas for call");
        }
        gas_left - PERFORM_ACTION_FINISH_GAS
    }

    /// Can be used to:
    /// - create new user (board member / proposer)
    /// - remove user (board member / proposer)
    /// - reactivate removed user
    /// - convert between board member and proposer
    /// Will keep the board size and proposer count in sync.
    fn change_user_role(&self, user_address: ManagedAddress, new_role: UserRole) {
        let user_id = self.user_mapper().get_or_create_user(&user_address);
        let user_id_to_role_mapper = self.user_id_to_role(user_id);
        let old_role = user_id_to_role_mapper.get();
        user_id_to_role_mapper.set(&new_role);

        // update board size
        let mut board_members_delta = 0isize;
        if old_role == UserRole::BoardMember {
            board_members_delta -= 1;
        }
        if new_role == UserRole::BoardMember {
            board_members_delta += 1;
        }
        if board_members_delta != 0 {
            self.num_board_members()
                .update(|value| usize_add_isize(value, board_members_delta));
        }

        let mut proposers_delta = 0isize;
        if old_role == UserRole::Proposer {
            proposers_delta -= 1;
        }
        if new_role == UserRole::Proposer {
            proposers_delta += 1;
        }
        if proposers_delta != 0 {
            self.num_proposers()
                .update(|value| usize_add_isize(value, proposers_delta));
        }
    }

    /// Returns `true` (`1`) if `getActionValidSignerCount >= getQuorum`.
    #[view(quorumReached)]
    fn quorum_reached(&self, action_id: usize) -> bool {
        let quorum = self.quorum().get();
        let valid_signers_count = self.get_action_valid_signer_count(action_id);
        valid_signers_count >= quorum
    }

    fn clear_action(&self, action_id: usize) {
        self.action_mapper().clear_entry_unchecked(action_id);
        self.action_signer_ids(action_id).clear();
    }

    /// Proposers and board members use this to launch signed actions.
    #[endpoint(performAction)]
    fn perform_action_endpoint(
        &self,
        action_id: usize,
    ) {
        let (_, caller_role) = self.get_caller_id_and_role();
        require!(
            caller_role.can_perform_action(),
            "only board members and proposers can perform actions"
        );
        require!(
            self.quorum_reached(action_id),
            "quorum has not been reached"
        );

        self.perform_action(action_id)
    }

    fn perform_action(&self, action_id: usize) {
        let action = self.action_mapper().get(action_id);

        // clean up storage
        // happens before actual execution, because the match provides the return on each branch
        self.clear_action(action_id);

        match action {
            Action::Nothing => {},
            Action::AddBoardMember(board_member_address) => {
                self.change_user_role(board_member_address, UserRole::BoardMember);
            },
            Action::AddProposer(proposer_address) => {
                self.change_user_role(proposer_address, UserRole::Proposer);

                // validation required for the scenario when a board member becomes a proposer
                require!(
                    self.quorum().get() <= self.num_board_members().get(),
                    "quorum cannot exceed board size"
                );
            },
            Action::RemoveUser(user_address) => {
                self.change_user_role(user_address, UserRole::None);
                let num_board_members = self.num_board_members().get();
                let num_proposers = self.num_proposers().get();
                require!(
                    num_board_members + num_proposers > 0,
                    "cannot remove all board members and proposers"
                );
                require!(
                    self.quorum().get() <= num_board_members,
                    "quorum cannot exceed board size"
                );
            },
            Action::ChangeQuorum(new_quorum) => {
                require!(
                    new_quorum <= self.num_board_members().get(),
                    "quorum cannot exceed board size"
                );
                self.quorum().set(new_quorum);
            },
            Action::SendEgldTransfer(call_data) => {
                self.send().direct_egld(
                    &call_data.to,
                    &call_data.egld_amount,
                    &[]
                );
            },
            Action::SendEsdtTransfer(call_data) => {
                self.send().direct(
                    &call_data.to,
                    &call_data.token,
                    0,
                    &call_data.amount,
                    &[]
                )
            },
        }
    }
}
