#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::derive::contract]
pub trait LinkedList {
    #[view(getWaitingRoom)]
    #[storage_mapper("waiting_room")]
    fn waiting_room(&self) -> LinkedListMapper<ManagedAddress>;

    #[init]
    fn init(&self) {
    }

    #[endpoint]
    fn add(&self) -> LinkedListMapper<ManagedAddress> {

        let caller = self.blockchain().get_caller();

        self.waiting_room().push_back(caller);

        self.waiting_room()
    }
}
