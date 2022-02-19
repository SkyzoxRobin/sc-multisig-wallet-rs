use crate::action::{Action, EgldTransferData, EsdtTransferData};

elrond_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[elrond_wasm::module]
pub trait MultisigProposeModule: crate::multisig_state::MultisigStateModule {
    fn propose_action(&self, action: Action<Self::Api>) -> SCResult<usize> {
        let (caller_id, caller_role) = self.get_caller_id_and_role();
        require!(
            caller_role.can_propose(),
            "only board members and proposers can propose"
        );

        let action_id = self.action_mapper().push(&action);
        if caller_role.can_sign() {
            // also sign
            // since the action is newly created, the caller can be the only signer
            self.action_signer_ids(action_id).insert(caller_id);
        }

        Ok(action_id)
    }

    /// Initiates board member addition process.
    /// Can also be used to promote a proposer to board member.
    #[endpoint(proposeAddBoardMember)]
    fn propose_add_board_member(&self, board_member_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::AddBoardMember(board_member_address))
    }

    /// Initiates proposer addition process..
    /// Can also be used to demote a board member to proposer.
    #[endpoint(proposeAddProposer)]
    fn propose_add_proposer(&self, proposer_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::AddProposer(proposer_address))
    }

    /// Removes user regardless of whether it is a board member or proposer.
    #[endpoint(proposeRemoveUser)]
    fn propose_remove_user(&self, user_address: ManagedAddress) -> SCResult<usize> {
        self.propose_action(Action::RemoveUser(user_address))
    }

    #[endpoint(proposeChangeQuorum)]
    fn propose_change_quorum(&self, new_quorum: usize) -> SCResult<usize> {
        self.propose_action(Action::ChangeQuorum(new_quorum))
    }

    /// Propose a transaction in which the contract will perform a transfer-execute call.
    /// Can send EGLD without calling anything.
    /// Can call smart contract endpoints directly.
    /// Doesn't really work with builtin functions.
    #[endpoint(proposeEgldTransfer)]
    fn propose_transfer_execute(
        &self,
        to: ManagedAddress,
        egld_amount: BigUint,
    ) -> SCResult<usize> {
        let call_data = self.prepare_call_data(to, egld_amount);
        self.propose_action(Action::SendEgldTransfer(call_data))
    }

    fn prepare_call_data(
        &self,
        to: ManagedAddress,
        egld_amount: BigUint,
    ) -> EgldTransferData<Self::Api> {
        EgldTransferData {
            to,
            egld_amount,
        }
    }

    /// Propose a transaction in which the contract will perform a transfer of ESDT.
    /// Can send ESDT without calling anything.
    /// Can call smart contract endpoints directly.
    /// Doesn't really work with builtin functions.
    #[endpoint(proposeEsdtTransfer)]
    fn propose_esdt_transfer_execute(
        &self,
        to: ManagedAddress,
        token: TokenIdentifier,
        amount: BigUint,
    ) -> SCResult<usize> {
        let call_data = self.prepare_transfer_esdt_data(to, token, amount);
        self.propose_action(Action::SendEsdtTransfer(call_data))
    }

    fn prepare_transfer_esdt_data(
        &self,
        to: ManagedAddress,
        token: TokenIdentifier,
        amount: BigUint,
    ) -> EsdtTransferData<Self::Api> {
        EsdtTransferData {
            to,
            token,
            amount,
        }
    }
}
