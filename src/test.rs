#![cfg(test)]
extern crate std;

use soroban_sdk::testutils::{Address as _};
use soroban_sdk::{Env, Address, token};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

use crate::{MissingValueContract, MissingValueContractClient};


fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let contract_address = e.register_stellar_asset_contract(admin.clone());
    (
        TokenClient::new(e, &contract_address),
        TokenAdminClient::new(e, &contract_address),
    )
}


fn create_soroswap_library_contract<'a>(e: &Env) -> MissingValueContractClient<'a> {
    MissingValueContractClient::new(e, &e.register_contract(None, MissingValueContract {}))
}

// Test Functions
struct MissingValueContractTest<'a> {
    contract: MissingValueContractClient<'a>,
}

impl<'a> MissingValueContractTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let contract = create_soroswap_library_contract(&env);
        MissingValueContractTest {
            contract,
        }
    }
}



#[test]
fn test_repeats_works() {
    let test = MissingValueContractTest::setup();
    let repeat_address = test.contract.repeat_address(&test.contract.address);
    assert_eq!(repeat_address, test.contract.address);

}



#[test]
fn test_mint_works() {
    let e: Env = Default::default();
    e.mock_all_auths(); // TODO: can we test otherwise?
    
    let admin = Address::random(&e);
    let (token, token_admin_client) = create_token_contract(&e, &admin);
    let test = MissingValueContractTest::setup();

    token_admin_client.mint(&test.contract.address, &1000);
    assert_eq!(token.balance(&test.contract.address), 1000);
    
}

#[test]
fn test_is_failing() {
    let e: Env = Default::default();
    e.mock_all_auths(); // TODO: can we test otherwise?
    
    let admin = Address::random(&e);
    let (token, _token_admin_client) = create_token_contract(&e, &admin);
    let test = MissingValueContractTest::setup();

   let repeat_address = test.contract.repeat_address(&token.address);

   assert_eq!(repeat_address, token.address);
    
}