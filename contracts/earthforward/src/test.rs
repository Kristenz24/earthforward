#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, token, Address, Env};

fn setup_test_env<'a>() -> (
    Env, 
    EarthForwardContractClient<'a>, 
    Address, 
    Address, 
    token::Client<'a>, 
    token::StellarAssetContractClient<'a>
) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, EarthForwardContract);
    let client = EarthForwardContractClient::new(&env, &contract_id);

    let ngo = Address::generate(&env);
    
    let token_admin = Address::generate(&env);
    let token_contract_id = env.register_stellar_asset_contract(token_admin);
    let token_client = token::Client::new(&env, &token_contract_id);
    let token_admin_client = token::StellarAssetContractClient::new(&env, &token_contract_id);

    (env, client, ngo, token_contract_id, token_client, token_admin_client)
}

#[test]
fn test_1_happy_path_donation_and_withdrawal() {
    let (env, client, ngo, token_id, token_client, token_admin) = setup_test_env();
    let donor = Address::generate(&env);
    
    token_admin.mint(&donor, &1000);
    client.initialize(&ngo, &token_id, &5000);
    
    // Test a normal donation
    client.donate(&donor, &400);
    assert_eq!(token_client.balance(&client.address), 400);
    assert_eq!(client.get_total_raised(), 400);
    
    // Test a normal withdrawal by the charity
    client.withdraw_funds(&ngo, &400);
    assert_eq!(token_client.balance(&ngo), 400);
    assert_eq!(token_client.balance(&client.address), 0);
}

#[test]
#[should_panic]
fn test_2_wrong_user_cannot_withdraw() {
    let (env, client, ngo, token_id, _, token_admin) = setup_test_env();
    let donor = Address::generate(&env);
    let wrong_user = Address::generate(&env);
    
    token_admin.mint(&donor, &1000);
    client.initialize(&ngo, &token_id, &5000);
    client.donate(&donor, &500);
    
    // Should fail: wrong_user is not the charity creator
    client.withdraw_funds(&wrong_user, &500);
}

#[test]
#[should_panic]
fn test_3_cannot_withdraw_more_than_vault_has() {
    let (env, client, ngo, token_id, _, token_admin) = setup_test_env();
    let donor = Address::generate(&env);
    
    token_admin.mint(&donor, &1000);
    client.initialize(&ngo, &token_id, &5000);
    client.donate(&donor, &300);
    
    // Should fail: trying to take out 400 when the vault only has 300
    client.withdraw_funds(&ngo, &400);
}

#[test]
fn test_4_state_verification() {
    let (env, client, ngo, token_id, _, _) = setup_test_env();
    
    client.initialize(&ngo, &token_id, &7500);
    
    assert_eq!(client.get_target_goal(), 7500);
    assert_eq!(client.get_total_raised(), 0);
}

#[test]
#[should_panic]
fn test_5_cannot_initialize_twice() {
    let (_, client, ngo, token_id, _, _) = setup_test_env();
    
    client.initialize(&ngo, &token_id, &5000);
    // Should fail: cannot change project settings once started
    client.initialize(&ngo, &token_id, &10000);
}