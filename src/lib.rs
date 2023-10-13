#![no_std]

use soroban_sdk::{contract, contractimpl, Address};


pub trait MissingValueContractTrait {

    fn repeat_address(factory: Address) -> Address;
   
}

#[contract]
struct MissingValueContract;

#[contractimpl]
impl MissingValueContractTrait for MissingValueContract {

    fn repeat_address(address: Address) -> Address {
        address
    }

}

mod test;
