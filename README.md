# Title

Pay-Per-Use API Gate — Token-Gated API Monetization on Stellar

---

# Description

Pay-Per-Use API Gate is a decentralized application built on the Stellar blockchain using Soroban smart contracts. It enables API developers to monetize their services through a token-gated access model, where users must hold and spend GATE tokens to consume API endpoints. Unlike traditional payment processors such as Stripe or PayPal that impose minimum transaction fees making micro-payments economically unviable, this system leverages Stellar's near-zero transaction cost (approximately $0.00001 per operation) to make per-request billing practical at any scale.

The core mechanism is a single Soroban smart contract that serves two roles simultaneously: a fungible token registry that tracks GATE token balances on-chain, and an API gateway that enforces token-gated access. Users purchase GATE tokens by depositing XLM directly into the contract. Each token purchase is an actual on-chain XLM transfer held in escrow by the contract. When a developer's backend processes a successful API request, it calls the smart contract to burn one GATE token from the user's balance and record the equivalent XLM value as revenue for the developer. The developer can then withdraw their accumulated XLM revenue at any time. Everything is verifiable, tamper-proof, and requires no centralized billing infrastructure.

The project is structured as a three-layer system: a Soroban Rust smart contract handling all on-chain state, a Node.js Express backend that signs and submits consume_token transactions on behalf of the developer, and an HTML/CSS/JavaScript frontend that allows users to connect their Freighter wallet, purchase GATE tokens, and interact with available API endpoints.

---

# Project Vision

The vision of Pay-Per-Use API Gate is to become a foundational payment primitive for the decentralized API economy. The goal is to allow any developer anywhere in the world to publish an API endpoint and receive real-time per-request revenue in XLM, with no bank account, no payment processor approval, and no monthly minimums. By making micro-payment billing economically viable through Stellar's infrastructure, this project opens API monetization to markets and use cases that are entirely inaccessible under the traditional subscription or flat-fee billing model.

---

# Features

**GATE Token — Internal Fungible Token**

The contract maintains a complete internal token system including per-address balances stored in persistent contract storage, a global total supply counter that decreases with each token burn, and an allowance mechanism that lets users pre-authorize the contract to deduct tokens on their behalf without requiring the user to sign each individual API call. Users can also transfer GATE tokens directly between wallets, making the token tradeable peer-to-peer on Stellar.

**Token Purchase via XLM Deposit**

The buy_tokens function accepts XLM from a user's wallet and mints an equivalent number of GATE tokens to that user's on-chain balance. The exchange rate is fixed at 1 GATE token per 0.1 XLM (one million stroops). The function requires the buyer's wallet signature via require_auth and enforces a minimum purchase of 0.1 XLM. The XLM is held in the contract as escrow and later transferred to the developer when they withdraw their revenue.

**Approve and Consume Pattern**

Before using any API, users call the approve function once to grant the contract permission to burn a specified number of GATE tokens on their behalf up to a chosen ledger expiration. This is analogous to the ERC-20 allowance pattern in Ethereum. Once approved, the developer's backend can call consume_token for each successful API response without requiring the user to sign every individual transaction, enabling seamless automatic billing.

**Backend-Initiated Credit Consumption**

The consume_token function is called exclusively by the developer's backend server, authenticated by the developer's secret key. It burns the configured number of GATE tokens from the user's balance and allowance, decreases the total supply permanently (deflationary burn), and records the equivalent XLM value as revenue for the calling developer. Custom per-endpoint pricing is supported: premium endpoints such as AI generation can cost five or ten tokens per call while basic endpoints cost one.

**Configurable Per-Endpoint API Pricing**

The admin can call set_api_price to assign a custom GATE token cost to any named API endpoint. Endpoints not explicitly configured default to one GATE token per call. This allows a single contract deployment to serve multiple API tiers with different pricing — basic lookups, standard data feeds, and premium AI-powered endpoints — all governed by the same on-chain billing contract.

**Developer Revenue Tracking and Withdrawal**

Each call to consume_token accumulates the equivalent XLM value (in stroops) into the developer's on-chain revenue record. The developer can call withdraw_revenue at any time to transfer the entire accumulated XLM balance from the contract to their personal Stellar wallet. This function requires the developer's wallet signature and resets the revenue counter to zero after the transfer completes.

**Node.js Backend with Full Soroban Integration**

The backend (backend/src/stellar.js) implements the complete Soroban transaction lifecycle: building a transaction, simulating it to determine the resource fee, assembling the final transaction with fees applied, signing with the developer's keypair, submitting to the Soroban RPC endpoint, and polling for confirmation. The Express server (backend/src/index.js) exposes REST API endpoints for balance checks, API calls with automatic credit deduction, revenue queries, and developer withdrawal — all wired to actual on-chain transactions on Stellar Testnet.

**14 Unit Tests**

The contract includes 14 unit tests using Soroban's built-in test environment covering: double initialization guard, minimum purchase enforcement, multi-deposit accumulation, token transfer between wallets, consume with default and custom endpoint pricing, consume without approval, insufficient balance rejection, revenue withdrawal, zero revenue rejection, and a complete end-to-end flow from token purchase through revenue withdrawal.

---

# Tech Stack

Smart Contract: Rust, Soroban SDK 20.0, deployed on Stellar Testnet

Backend: Node.js, Express 4, @stellar/stellar-sdk 12 (SorobanRpc, Contract, TransactionBuilder)

Frontend: HTML, CSS, JavaScript, @stellar/freighter-api

Wallet: Freighter Browser Extension (Testnet)


# Contract Methods

The TokenGate smart contract exposes the following public functions. All state-changing functions require a valid Stellar wallet signature from the authorized caller. Read-only functions are free to call and do not consume any gas.

**initialize(admin: Address, xlm_token: Address)**

One-time setup function that must be called immediately after deployment. Registers the admin address (the account authorized to configure API pricing) and the XLM native token address used for payments. Calling this function a second time will panic with "Already initialized". After this call, the contract stores the admin, token address, and initializes the total GATE token supply to zero.

**buy_tokens(buyer: Address, xlm_amount: i128)**

Allows any user to purchase GATE tokens by depositing XLM into the contract. Requires the buyer's wallet signature. Transfers the specified XLM amount (in stroops) from the buyer's wallet into the contract escrow using the native Stellar token interface. Mints the equivalent number of GATE tokens at a fixed rate of 1 GATE per 1,000,000 stroops (0.1 XLM). Enforces a minimum purchase of 1,000,000 stroops. The total supply counter increases with each successful purchase.

**approve(from: Address, amount: i128, expiration_ledger: u32)**

Grants the contract itself permission to burn up to a specified number of GATE tokens from the caller's balance on future consume_token calls. Requires the caller's wallet signature. The approval is stored in temporary contract storage and expires at the given ledger sequence number. This must be called before using any API endpoint for the first time. Without an active approval, all consume_token calls for that user will panic.

**consume_token(dev: Address, user: Address, endpoint: String)**

Called exclusively by the developer's backend server after each successful API response. Requires the developer's wallet signature. Deducts the configured GATE token price for the named endpoint from the user's allowance and balance simultaneously. Burns the tokens permanently by reducing the total supply (deflationary mechanism). Records the equivalent XLM value (token price multiplied by 1,000,000 stroops) as accumulated revenue for the developer. If the user has insufficient allowance or balance, the function panics and the backend returns a 402 response to the API caller.

**set_api_price(admin: Address, endpoint: String, price: u32)**

Admin-only function to configure how many GATE tokens an API endpoint costs per call. Requires the admin wallet signature and verifies the caller matches the stored admin address. Endpoints not configured with this function default to a cost of 1 GATE token per call. Setting "weather" to 1 and "ai-text" to 5 means the AI endpoint is five times more expensive than the basic weather lookup.

**withdraw_revenue(dev: Address)**

Allows a developer to transfer all accumulated XLM revenue from the contract to their personal Stellar wallet. Requires the developer's wallet signature. Reads the total accumulated revenue in stroops from persistent storage, performs a real XLM transfer from the contract to the developer's address using the native token interface, and resets the revenue counter to zero. Panics with "No revenue to withdraw" if the counter is zero.

**transfer(from: Address, to: Address, amount: i128)**

Standard peer-to-peer GATE token transfer between wallets. Requires the sender's wallet signature. Validates that the sender has sufficient balance and that the amount is positive before updating both balances in persistent storage. Enables GATE tokens to be traded directly between users outside of the API gate mechanism.

**gate_balance(user: Address) → i128**

Read-only. Returns the current GATE token balance of any wallet address. Returns zero if the address has never purchased tokens. No signature required, no gas cost.

**total_supply() → i128**

Read-only. Returns the total number of GATE tokens currently in circulation across all wallets. Decreases over time as tokens are burned through consume_token calls, making this a deflationary token system.

**allowance(from: Address) → i128**

Read-only. Returns the remaining GATE token approval granted by a wallet to this contract. Returns zero if no active approval exists or if the approval has expired past its ledger deadline.

**get_api_price(endpoint: String) → u32**

Read-only. Returns the configured GATE token cost for a named API endpoint. Returns 1 (the default price) if the endpoint has not been explicitly configured by the admin.

**get_dev_revenue(dev: Address) → i128**

Read-only. Returns the accumulated XLM revenue in stroops for a given developer address. Divide by 10,000,000 to convert to XLM denomination.

---

# Backend API Endpoints

GET /balance/:address — Returns the current GATE token balance of a Stellar wallet address without spending any gas.

POST /api/weather — Accepts userAddress in the request body, deducts 1 GATE token from the user's balance on-chain, and returns mock weather data for Ho Chi Minh City. Returns 402 Payment Required if the user has insufficient tokens.

POST /api/ai-text — Accepts userAddress and prompt. Deducts 5 GATE tokens (premium endpoint). Returns a mock AI-generated response. Returns 402 if the user has fewer than 5 tokens.

GET /dev/revenue — Returns the developer's current accumulated XLM revenue in both stroops and XLM denomination.

POST /dev/withdraw — Triggers a withdraw_revenue transaction on the Stellar contract, transferring all accumulated XLM to the developer's wallet.

---

# Project Structure

contracts/token_gate/src/lib.rs — Single Soroban contract with token and gateway logic

contracts/token_gate/src/test.rs — 14 unit tests

contracts/token_gate/Cargo.toml — Package configuration

contracts/Cargo.toml — Workspace configuration

backend/src/stellar.js — Soroban transaction builder, signer, and poller

backend/src/index.js — Express REST API server

backend/.env.example — Environment variable template

frontend/src/App.jsx — React Frontend with Cyberpunk/Web3 Dark Mode UI, Freighter wallet integration, and real-time transaction terminal.

---

# How to Run

### 1. Smart Contract
Ensure you have Rust and Soroban CLI installed.
```bash
cd contracts
cargo test
soroban contract build
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/token_gate.wasm --source YOUR_ACCOUNT --network testnet
```

### 2. Backend Server
The backend handles the execution of `consume_token` transactions.
```bash
cd backend
npm install
cp .env.example .env
```
Edit the `.env` file to include your newly deployed `CONTRACT_ID` and the `DEV_SECRET_KEY` (Must start with an 'S', e.g., SAXYZ...).
```bash
npm run dev
```
The backend will start on `http://localhost:3001`.

### 3. Frontend WebApp
The frontend is built with React and Vite, featuring a responsive Glassmorphism UI.
```bash
cd frontend
npm install
npm run dev
```
The web app will be available at `http://localhost:5173`. Connect your Freighter wallet to interact.

---

# Future Scopes

**Stellar DEX Integration for GATE Token**

Register the GATE token as a Stellar Asset Contract (SAC) asset to make it natively visible and tradeable on the Stellar decentralized exchange. Users could buy GATE tokens using any Stellar-supported asset including USDC, not just XLM, enabling broader adoption without requiring users to hold XLM specifically.

**Multi-Developer Marketplace**

Extend the contract with a developer registration system where multiple API providers can list their endpoints, set their own pricing, and receive independent revenue streams. A platform fee (in basis points) could be added to each consume_token call, automatically splitting revenue between the API developer and the marketplace operator.

**Dynamic Token Pricing Oracle**

Integrate an off-chain price oracle to adjust the XLM-to-GATE exchange rate based on real-time XLM market prices, ensuring that the dollar-equivalent cost per API call remains stable regardless of XLM price volatility. This makes pricing predictable for enterprise API consumers.

**Trial Token Allocation**

Allow the admin to airdrop a small number of free GATE tokens to new wallet addresses as a freemium onboarding mechanism. New users could try the API before committing XLM, lowering the adoption barrier significantly.

**Subscription Bundles with Volume Discounts**

Add tiered purchase rates where users buying above certain XLM thresholds receive bonus GATE tokens. For example, purchasing 10 XLM at once could yield 120 GATE instead of 100, incentivizing bulk purchases and longer-term usage commitments.

**Real-World Asset Data Verification**

Extend the consume_token model to serve as a paid oracle for Real World Asset verification workflows. A tokenized real estate platform could use each contract call to pay for a verified property valuation hash stored permanently on-chain, creating a tamper-proof audit trail where the cost of verification is funded transparently by the requesting party.

**Cross-Platform API Credential System**

Issue a non-fungible Soroban token (soul-bound) as an API key credential tied to a user's GATE token balance. Backend services across multiple platforms could verify API access by checking credential ownership on-chain rather than maintaining centralized API key databases.

**Enterprise Chargeback Accounting**

Deploy the contract as an internal billing system for large organizations running microservice architectures. Each internal team pre-loads a quarterly GATE token budget; the central API platform charges per call. All usage is recorded immutably on Stellar, replacing spreadsheet-based billing reconciliation with transparent on-chain accounting.

---

# Expansion Ideas

The Pay-Per-Use API Gate architecture is a general-purpose billing primitive applicable across many industries. The core pattern — deposit XLM, receive tokens, spend tokens per action, developer withdraws XLM — can be adopted directly or with minor modifications in the following contexts.

**AI Inference Services:** Indie developers hosting self-managed AI models for text generation, image captioning, or code review can wrap their inference endpoints behind this gate and receive per-generation revenue automatically without Stripe accounts or monthly invoicing.

**Environmental and IoT Data Feeds:** Independent weather station operators, satellite imagery providers, and sensor network owners can monetize their data on a per-query basis, reaching paying consumers in regions where traditional payment infrastructure is unavailable.

**Legal Document Timestamping:** Notarial services can charge one GATE token per document hash verification, producing a Stellar transaction as a permanent, publicly auditable proof of existence for legal documents.

**Academic Dataset Access:** Universities and research institutions can open proprietary datasets to external researchers on a pay-per-query basis, with research budgets converted to GATE tokens consumed as queries are executed.

**Freelance Compute Marketplaces:** Developers offering rendering, data transformation, or batch processing jobs can replace informal payment agreements with this contract, where clients pre-fund with tokens and the backend charges per completed job unit.

**Web3 Game Economies:** Browser games requiring server-side computation per player action can charge GATE tokens for AI opponent responses, procedural content generation, or loot drop calculations, funding backend costs directly from player spending.

---

# Profile

Name: Le Van Quyen

Role: Web3 Developer / Smart Contract Engineer

Skills: Rust, Soroban SDK, Stellar Blockchain, Node.js, JavaScript, Express.js, Smart Contract Development, Decentralized Application Architecture, Micro-payment Systems

Bootcamp: Rise In x Stellar University Tour — Vietnam

License: MIT License
---

## Contract Detail

**ID:** `CCL7RRR73KERUIJLWBUXAAG575M2M3ILY2H3WO7WB4IVO3OXFE32E2GA`

**Explorer:** [View on Stellar Expert](https://stellar.expert/explorer/testnet/contract/CCL7RRR73KERUIJLWBUXAAG575M2M3ILY2H3WO7WB4IVO3OXFE32E2GA?filter=history)

<img width="1917" height="839" alt="image" src="https://github.com/user-attachments/assets/2e43a025-3385-4e3e-ad7b-779b96ba89e1" />
<img width="1920" height="907" alt="image" src="https://github.com/user-attachments/assets/f5d47a38-e993-458c-a74b-f4605aae5b07" />



---
