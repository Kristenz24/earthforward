#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contract]
pub struct EarthForwardContract;

#[contracttype]
#[derive(Clone)]
enum DataKey {
    Ngo,         // Wallet address of the charity creator
    Token,       // Digital dollar address (USDC)
    TargetGoal,  // Fundraising goal amount
    TotalRaised, // Total money collected so far
}

#[contractimpl]
impl EarthForwardContract {
    /// Starts a new fundraising campaign for a charity project
    pub fn initialize(env: Env, ngo: Address, token: Address, target_goal: i128) {
        if env.storage().instance().has(&DataKey::Ngo) {
            panic!("Campaign is already initialized");
        }
        assert!(target_goal > 0, "Target goal must be greater than zero");
        
        env.storage().instance().set(&DataKey::Ngo, &ngo);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::TargetGoal, &target_goal);
        env.storage().instance().set(&DataKey::TotalRaised, &0i128);
    }

    /// Lets anyone send a USDC donation to the active project
    pub fn donate(env: Env, donor: Address, amount: i128) {
        donor.require_auth();
        assert!(amount > 0, "Donation amount must be greater than zero");

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token_addr);

        // Move the donation money from the donor's account to the contract vault
        token_client.transfer(&donor, &env.current_contract_address(), &amount);

        let mut total_raised: i128 = env.storage().instance().get(&DataKey::TotalRaised).unwrap_or(0);
        total_raised += amount;
        env.storage().instance().set(&DataKey::TotalRaised, &total_raised);
    }

    /// Lets the verified charity creator withdraw the collected money
    pub fn withdraw_funds(env: Env, ngo: Address, amount: i128) {
        ngo.require_auth();
        
        let stored_ngo: Address = env.storage().instance().get(&DataKey::Ngo).unwrap();
        assert_eq!(ngo, stored_ngo, "Caller is not the verified campaign creator");

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token_addr);
        
        let contract_balance = token_client.balance(&env.current_contract_address());
        assert!(amount <= contract_balance, "Requested amount exceeds vault balance");

        // Send tokens from the contract vault directly to the charity's wallet
        token_client.transfer(&env.current_contract_address(), &ngo, &amount);
    }

    pub fn get_total_raised(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalRaised).unwrap_or(0)
    }

    pub fn get_target_goal(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TargetGoal).unwrap_or(0)
    }
}