use elrond_wasm::{
    api::ManagedTypeApi,
    types::{
         BigUint, ManagedAddress, ManagedVec, TokenIdentifier,
    },
};

elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TypeAbi)]
pub struct EgldTransferData<M: ManagedTypeApi> {
    pub to: ManagedAddress<M>,
    pub egld_amount: BigUint<M>,
}

#[derive(NestedEncode, NestedDecode, TypeAbi)]
pub struct EsdtTransferData<M: ManagedTypeApi> {
    pub to: ManagedAddress<M>,
    pub token: TokenIdentifier<M>,
    pub amount: BigUint<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum Action<M: ManagedTypeApi> {
    Nothing,
    AddBoardMember(ManagedAddress<M>),
    AddProposer(ManagedAddress<M>),
    RemoveUser(ManagedAddress<M>),
    ChangeQuorum(usize),
    SendEgldTransfer(EgldTransferData<M>),
    SendEsdtTransfer(EsdtTransferData<M>),
}

impl<M: ManagedTypeApi> Action<M> {
    /// Only pending actions are kept in storage,
    /// both executed and discarded actions are removed (converted to `Nothing`).
    /// So this is equivalent to `action != Action::Nothing`.
    pub fn is_pending(&self) -> bool {
        !matches!(*self, Action::Nothing)
    }
}

/// Not used internally, just to retrieve results via endpoint.
#[derive(TopEncode, TypeAbi)]
pub struct ActionFullInfo<M: ManagedTypeApi> {
    pub action_id: usize,
    pub action_data: Action<M>,
    pub signers: ManagedVec<M, ManagedAddress<M>>,
}
