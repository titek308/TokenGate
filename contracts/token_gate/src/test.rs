#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::StellarAssetClient,
    token::Client as XlmClient,
    Address, Env, String,
};

// ── Setup helper ──────────────────────────────────────────────────────────────

fn setup() -> (Env, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    // Deploy mock XLM
    let xlm_admin = Address::generate(&env);
    let xlm_contract = env.register_stellar_asset_contract_v2(xlm_admin);
    let xlm_addr = xlm_contract.address();

    // Deploy TokenGate
    let admin = Address::generate(&env);
    let contract_id = env.register(TokenGate, ());
    let client = TokenGateClient::new(&env, &contract_id);
    client.initialize(&admin, &xlm_addr);

    (env, contract_id, xlm_addr, admin)
}

fn mint_xlm(env: &Env, xlm_addr: &Address, to: &Address, amount: i128) {
    StellarAssetClient::new(env, xlm_addr).mint(to, &amount);
}

// ── Test: initialize ──────────────────────────────────────────────────────────

#[test]
fn test_initialize_success() {
    let (env, contract_id, _, _) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    assert_eq!(client.total_supply(), 0);
}

#[test]
#[should_panic(expected = "Already initialized")]
fn test_double_initialize_panics() {
    let (env, contract_id, xlm_addr, admin) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    client.initialize(&admin, &xlm_addr);
}

// ── Test: buy_tokens ──────────────────────────────────────────────────────────

#[test]
fn test_buy_tokens_correct_rate() {
    let (env, contract_id, xlm_addr, _) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    let user = Address::generate(&env);
    mint_xlm(&env, &xlm_addr, &user, 10_000_000); // 1 XLM

    client.buy_tokens(&user, &10_000_000i128); // 1 XLM = 10 GATE

    assert_eq!(client.gate_balance(&user), 10);
    assert_eq!(client.total_supply(), 10);
}

#[test]
fn test_buy_tokens_multiple_times_accumulates() {
    let (env, contract_id, xlm_addr, _) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    let user = Address::generate(&env);
    mint_xlm(&env, &xlm_addr, &user, 30_000_000); // 3 XLM

    client.buy_tokens(&user, &10_000_000i128);
    client.buy_tokens(&user, &10_000_000i128);
    client.buy_tokens(&user, &10_000_000i128);

    assert_eq!(client.gate_balance(&user), 30);
    assert_eq!(client.total_supply(), 30);
}

#[test]
#[should_panic(expected = "Minimum 0.1 XLM")]
fn test_buy_tokens_too_small_panics() {
    let (env, contract_id, xlm_addr, _) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    let user = Address::generate(&env);
    mint_xlm(&env, &xlm_addr, &user, 500_000);
    client.buy_tokens(&user, &500_000i128);
}

// ── Test: approve + consume_token ────────────────────────────────────────────

#[test]
fn test_consume_token_default_price() {
    let (env, contract_id, xlm_addr, _) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    let user = Address::generate(&env);
    let dev  = Address::generate(&env);
    mint_xlm(&env, &xlm_addr, &user, 10_000_000);

    client.buy_tokens(&user, &10_000_000i128); // 10 GATE
    client.approve(&user, &10i128, &(env.ledger().sequence() + 10000));

    let endpoint = String::from_str(&env, "weather");
    client.consume_token(&dev, &user, &endpoint);

    assert_eq!(client.gate_balance(&user), 9);
    assert_eq!(client.total_supply(), 9);           // supply giảm (deflationary burn)
    assert_eq!(client.get_dev_revenue(&dev), 1_000_000i128); // 0.1 XLM
}

#[test]
fn test_consume_token_custom_price() {
    let (env, contract_id, xlm_addr, admin) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    let user = Address::generate(&env);
    let dev  = Address::generate(&env);
    mint_xlm(&env, &xlm_addr, &user, 50_000_000);

    client.buy_tokens(&user, &50_000_000i128); // 50 GATE
    client.approve(&user, &50i128, &(env.ledger().sequence() + 10000));

    // Admin set "image_gen" = 5 GATE/call
    let endpoint = String::from_str(&env, "image_gen");
    client.set_api_price(&admin, &endpoint, &5u32);

    client.consume_token(&dev, &user, &endpoint);

    assert_eq!(client.gate_balance(&user), 45);
    assert_eq!(client.get_dev_revenue(&dev), 5 * 1_000_000i128);
}

#[test]
#[should_panic(expected = "Insufficient allowance")]
fn test_consume_without_approve_panics() {
    let (env, contract_id, xlm_addr, _) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    let user = Address::generate(&env);
    let dev  = Address::generate(&env);
    mint_xlm(&env, &xlm_addr, &user, 10_000_000);
    client.buy_tokens(&user, &10_000_000i128);
    // No approve call
    client.consume_token(&dev, &user, &String::from_str(&env, "weather"));
}

// ── Test: withdraw_revenue ────────────────────────────────────────────────────

#[test]
fn test_withdraw_revenue_success() {
    let (env, contract_id, xlm_addr, _) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    let user = Address::generate(&env);
    let dev  = Address::generate(&env);
    mint_xlm(&env, &xlm_addr, &user, 10_000_000);

    client.buy_tokens(&user, &10_000_000i128);
    client.approve(&user, &10i128, &(env.ledger().sequence() + 10000));

    let endpoint = String::from_str(&env, "ai");
    for _ in 0..10 {
        client.consume_token(&dev, &user, &endpoint);
    }

    client.withdraw_revenue(&dev);

    let dev_xlm = XlmClient::new(&env, &xlm_addr).balance(&dev);
    assert_eq!(dev_xlm, 10_000_000); // 1 XLM về ví dev
    assert_eq!(client.get_dev_revenue(&dev), 0);
}

#[test]
#[should_panic(expected = "No revenue to withdraw")]
fn test_withdraw_zero_revenue_panics() {
    let (env, contract_id, _, _) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    let dev = Address::generate(&env);
    client.withdraw_revenue(&dev);
}

// ── Test: transfer GATE token ─────────────────────────────────────────────────

#[test]
fn test_transfer_gate_token() {
    let (env, contract_id, xlm_addr, _) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    let alice = Address::generate(&env);
    let bob   = Address::generate(&env);
    mint_xlm(&env, &xlm_addr, &alice, 10_000_000);

    client.buy_tokens(&alice, &10_000_000i128); // 10 GATE
    client.transfer(&alice, &bob, &4i128);

    assert_eq!(client.gate_balance(&alice), 6);
    assert_eq!(client.gate_balance(&bob), 4);
}

// ── Test: Full end-to-end ─────────────────────────────────────────────────────

#[test]
fn test_full_flow() {
    let (env, contract_id, xlm_addr, admin) = setup();
    let client = TokenGateClient::new(&env, &contract_id);
    let user = Address::generate(&env);
    let dev  = Address::generate(&env);
    mint_xlm(&env, &xlm_addr, &user, 50_000_000); // 5 XLM

    // 1. Mua 50 GATE token
    client.buy_tokens(&user, &50_000_000i128);
    assert_eq!(client.gate_balance(&user), 50);

    // 2. Admin set giá "premium_ai" = 5 GATE/call
    let endpoint = String::from_str(&env, "premium_ai");
    client.set_api_price(&admin, &endpoint, &5u32);

    // 3. User approve contract trừ 50 GATE
    client.approve(&user, &50i128, &(env.ledger().sequence() + 100000));

    // 4. Dev gọi API 6 lần × 5 GATE = 30 GATE burned
    for _ in 0..6 {
        client.consume_token(&dev, &user, &endpoint);
    }
    assert_eq!(client.gate_balance(&user), 20);
    assert_eq!(client.total_supply(), 20);
    assert_eq!(client.get_dev_revenue(&dev), 30 * 1_000_000i128); // 3 XLM

    // 5. Dev rút 3 XLM
    client.withdraw_revenue(&dev);
    let dev_xlm = XlmClient::new(&env, &xlm_addr).balance(&dev);
    assert_eq!(dev_xlm, 30_000_000); // 3 XLM
}
