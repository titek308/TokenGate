#![no_std]

//! # TokenGate — Pay-Per-Use API Access via GATE Token
//!
//! Một contract duy nhất đảm nhiệm hai vai trò:
//!   1. **Token nội bộ (GATE)**: lưu balance, allowance, total supply trực tiếp trong storage.
//!   2. **API Gate**: bán GATE token bằng XLM, trừ token khi dùng API, rút doanh thu cho Dev.
//!
//! Không cần deploy nhiều contract hay cross-contract call.

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String};

// ══════════════════════════════════════════════════════════════════════════════
// Storage Keys
// ══════════════════════════════════════════════════════════════════════════════

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    // --- Setup ---
    Admin,
    XlmToken,

    // --- GATE Token (nội bộ) ---
    GateBalance(Address),
    Allowance(AllowanceKey),
    TotalSupply,

    // --- API Gate ---
    ApiPrice(String),        // credits (GATE) per call
    DevRevenue(Address),     // XLM stroops earned by each dev
}

#[contracttype]
#[derive(Clone)]
pub struct AllowanceKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
#[derive(Clone)]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

/// 1 GATE token = 0.1 XLM = 1_000_000 stroops
pub const STROOPS_PER_GATE: i128 = 1_000_000;

// ══════════════════════════════════════════════════════════════════════════════
// Contract
// ══════════════════════════════════════════════════════════════════════════════

#[contract]
pub struct TokenGate;

#[contractimpl]
impl TokenGate {

    // ── 1. Initialize ─────────────────────────────────────────────────────────

    /// Khởi tạo một lần. `admin` có thể set giá API.
    /// `xlm_token` là địa chỉ của native XLM asset trên mạng Stellar.
    pub fn initialize(env: Env, admin: Address, xlm_token: Address) {
        admin.require_auth();
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::XlmToken, &xlm_token);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
    }

    // ── 2. API Gate: Mua GATE token bằng XLM ─────────────────────────────────

    /// User gửi XLM vào contract để nhận GATE token.
    /// Tỉ giá: 1 GATE = 0.1 XLM (= 1_000_000 stroops).
    pub fn buy_tokens(env: Env, buyer: Address, xlm_amount: i128) {
        buyer.require_auth();
        if xlm_amount < STROOPS_PER_GATE {
            panic!("Minimum 0.1 XLM per purchase");
        }

        // Chuyển XLM từ ví buyer vào contract (escrow)
        let xlm_addr: Address = env.storage().instance().get(&DataKey::XlmToken).unwrap();
        token::Client::new(&env, &xlm_addr)
            .transfer(&buyer, &env.current_contract_address(), &xlm_amount);

        // Mint GATE token cho buyer
        let gate_amount = xlm_amount / STROOPS_PER_GATE;
        Self::add_gate_balance(&env, &buyer, gate_amount);
        let sup: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(sup + gate_amount));
    }

    // ── 3. User approve contract trừ token ───────────────────────────────────

    /// User cấp quyền cho contract tự trừ GATE token khi backend gọi API.
    pub fn approve(env: Env, from: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();
        let spender = env.current_contract_address();
        let key = DataKey::Allowance(AllowanceKey { from, spender });
        env.storage()
            .temporary()
            .set(&key, &AllowanceValue { amount, expiration_ledger });
    }

    // ── 4. Backend gọi sau mỗi API request thành công ────────────────────────

    /// `dev` là ví của nhà phát triển sở hữu API endpoint.
    /// `user` là người dùng đang dùng API.
    /// `endpoint` xác định giá (GATE tokens). Mặc định = 1 GATE/lần.
    pub fn consume_token(env: Env, dev: Address, user: Address, endpoint: String) {
        dev.require_auth();

        let price: i128 = env
            .storage()
            .persistent()
            .get::<DataKey, u32>(&DataKey::ApiPrice(endpoint))
            .unwrap_or(1) as i128;

        // Trừ allowance của user
        Self::spend_allowance(&env, &user, price);

        // Burn GATE token của user
        let bal = Self::read_gate_balance(&env, &user);
        if bal < price {
            panic!("Insufficient GATE token balance");
        }
        Self::set_gate_balance(&env, &user, bal - price);

        // Giảm total supply (deflationary burn)
        let sup: i128 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(sup - price));

        // Cộng doanh thu XLM cho dev
        let rev: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::DevRevenue(dev.clone()))
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::DevRevenue(dev), &(rev + price * STROOPS_PER_GATE));
    }

    // ── 5. Dev rút doanh thu XLM ──────────────────────────────────────────────

    pub fn withdraw_revenue(env: Env, dev: Address) {
        dev.require_auth();

        let revenue: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::DevRevenue(dev.clone()))
            .unwrap_or(0);
        if revenue == 0 {
            panic!("No revenue to withdraw");
        }

        let xlm_addr: Address = env.storage().instance().get(&DataKey::XlmToken).unwrap();
        token::Client::new(&env, &xlm_addr)
            .transfer(&env.current_contract_address(), &dev, &revenue);

        env.storage()
            .persistent()
            .set(&DataKey::DevRevenue(dev), &0i128);
    }

    // ── 6. Admin: Đặt giá endpoint ────────────────────────────────────────────

    pub fn set_api_price(env: Env, admin: Address, endpoint: String, price: u32) {
        admin.require_auth();
        Self::require_admin(&env, &admin);
        env.storage()
            .persistent()
            .set(&DataKey::ApiPrice(endpoint), &price);
    }

    // ── 7. GATE Token: Transfer ───────────────────────────────────────────────

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        if amount <= 0 { panic!("Amount must be positive"); }
        let bal = Self::read_gate_balance(&env, &from);
        if bal < amount { panic!("Insufficient balance"); }
        Self::set_gate_balance(&env, &from, bal - amount);
        let to_bal = Self::read_gate_balance(&env, &to);
        Self::set_gate_balance(&env, &to, to_bal + amount);
    }

    // ── 8. Read-only getters ──────────────────────────────────────────────────

    pub fn gate_balance(env: Env, user: Address) -> i128 {
        Self::read_gate_balance(&env, &user)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }

    pub fn allowance(env: Env, from: Address) -> i128 {
        let spender = env.current_contract_address();
        let key = DataKey::Allowance(AllowanceKey { from, spender });
        match env.storage().temporary().get::<DataKey, AllowanceValue>(&key) {
            Some(v) if v.expiration_ledger >= env.ledger().sequence() => v.amount,
            _ => 0,
        }
    }

    pub fn get_api_price(env: Env, endpoint: String) -> u32 {
        env.storage()
            .persistent()
            .get(&DataKey::ApiPrice(endpoint))
            .unwrap_or(1)
    }

    pub fn get_dev_revenue(env: Env, dev: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::DevRevenue(dev))
            .unwrap_or(0)
    }

    // ── Internals ─────────────────────────────────────────────────────────────

    fn read_gate_balance(env: &Env, addr: &Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::GateBalance(addr.clone()))
            .unwrap_or(0)
    }

    fn set_gate_balance(env: &Env, addr: &Address, amount: i128) {
        env.storage()
            .persistent()
            .set(&DataKey::GateBalance(addr.clone()), &amount);
    }

    fn add_gate_balance(env: &Env, addr: &Address, delta: i128) {
        let cur = Self::read_gate_balance(env, addr);
        Self::set_gate_balance(env, addr, cur + delta);
    }

    fn spend_allowance(env: &Env, from: &Address, amount: i128) {
        let spender = env.current_contract_address();
        let key = DataKey::Allowance(AllowanceKey { from: from.clone(), spender });
        let val = env
            .storage()
            .temporary()
            .get::<DataKey, AllowanceValue>(&key)
            .unwrap_or(AllowanceValue { amount: 0, expiration_ledger: 0 });
        let cur = if val.expiration_ledger >= env.ledger().sequence() { val.amount } else { 0 };
        if cur < amount { panic!("Insufficient allowance"); }
        env.storage().temporary().set(
            &key,
            &AllowanceValue { amount: cur - amount, expiration_ledger: val.expiration_ledger },
        );
    }

    fn require_admin(env: &Env, caller: &Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if *caller != admin { panic!("Caller is not admin"); }
    }
}

mod test;
