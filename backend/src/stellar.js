const {
  Keypair,
  Networks,
  TransactionBuilder,
  BASE_FEE,
  Contract,
  SorobanRpc,
  xdr,
  Address,
  nativeToScVal,
  scValToNative,
} = require("@stellar/stellar-sdk");

const CONTRACT_ID = process.env.CONTRACT_ID;
const DEV_SECRET  = process.env.DEV_SECRET_KEY;
const RPC_URL     = process.env.SOROBAN_RPC_URL || "https://soroban-testnet.stellar.org";
const PASSPHRASE  = process.env.NETWORK_PASSPHRASE || Networks.TESTNET;

const server   = new SorobanRpc.Server(RPC_URL, { allowHttp: false });
const devKeypair = Keypair.fromSecret(DEV_SECRET);

/**
 * Đọc dữ liệu từ contract (không tốn phí — simulate only).
 * @param {string} fnName  - tên hàm
 * @param {xdr.ScVal[]} args - mảng tham số
 */
async function simulateRead(fnName, args = []) {
  const contract = new Contract(CONTRACT_ID);
  const account  = await server.getAccount(devKeypair.publicKey());

  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: PASSPHRASE,
  })
    .addOperation(contract.call(fnName, ...args))
    .setTimeout(30)
    .build();

  const result = await server.simulateTransaction(tx);
  if (SorobanRpc.Api.isSimulationError(result)) {
    throw new Error(`Simulate error: ${result.error}`);
  }
  return scValToNative(result.result.retval);
}

/**
 * Gửi giao dịch thật lên mạng Stellar (Dev ký bằng SECRET_KEY).
 * @param {string} fnName
 * @param {xdr.ScVal[]} args
 */
async function sendTransaction(fnName, args = []) {
  const contract = new Contract(CONTRACT_ID);
  const account  = await server.getAccount(devKeypair.publicKey());

  // Bước 1: Build transaction
  let tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: PASSPHRASE,
  })
    .addOperation(contract.call(fnName, ...args))
    .setTimeout(30)
    .build();

  // Bước 2: Simulate để lấy resource fee (bắt buộc với Soroban)
  const sim = await server.simulateTransaction(tx);
  if (SorobanRpc.Api.isSimulationError(sim)) {
    throw new Error(`Simulation failed: ${sim.error}`);
  }

  // Bước 3: Assemble (thêm resource fee vào tx)
  tx = SorobanRpc.assembleTransaction(tx, sim).build();

  // Bước 4: Dev ký bằng Secret Key
  tx.sign(devKeypair);

  // Bước 5: Gửi lên mạng Stellar
  const response = await server.sendTransaction(tx);
  if (response.status === "ERROR") {
    throw new Error(`Submit failed: ${JSON.stringify(response.errorResult)}`);
  }

  // Bước 6: Polling chờ transaction confirm
  return await pollTxStatus(response.hash);
}

/**
 * Polling đợi transaction được confirm trên chain.
 */
async function pollTxStatus(hash) {
  let attempts = 0;
  while (attempts < 20) {
    await sleep(1500);
    const res = await server.getTransaction(hash);

    if (res.status === SorobanRpc.Api.GetTransactionStatus.SUCCESS) {
      return { success: true, hash, result: res };
    }
    if (res.status === SorobanRpc.Api.GetTransactionStatus.FAILED) {
      throw new Error(`Transaction failed: ${hash}`);
    }
    attempts++;
  }
  throw new Error(`Transaction timeout: ${hash}`);
}

function sleep(ms) {
  return new Promise((r) => setTimeout(r, ms));
}

// ── Public helpers ──────────────────────────────────────────────────────────

/**
 * Gọi consume_token trên Soroban contract.
 * @param {string} userAddress - địa chỉ ví Stellar của người dùng
 * @param {string} endpoint    - tên API endpoint
 */
async function consumeToken(userAddress, endpoint) {
  const args = [
    new Address(devKeypair.publicKey()).toScVal(), // dev
    new Address(userAddress).toScVal(),            // user
    nativeToScVal(endpoint, { type: "symbol" }),   // endpoint
  ];
  return await sendTransaction("consume_token", args);
}

/**
 * Lấy số dư GATE token của một user (read-only, không tốn phí).
 * @param {string} userAddress
 */
async function getGateBalance(userAddress) {
  const args = [new Address(userAddress).toScVal()];
  return await simulateRead("gate_balance", args);
}

/**
 * Lấy doanh thu của Dev (read-only).
 */
async function getDevRevenue() {
  const args = [new Address(devKeypair.publicKey()).toScVal()];
  return await simulateRead("get_dev_revenue", args);
}

/**
 * Dev rút toàn bộ doanh thu về ví.
 */
async function withdrawRevenue() {
  const args = [new Address(devKeypair.publicKey()).toScVal()];
  return await sendTransaction("withdraw_revenue", args);
}

module.exports = { consumeToken, getGateBalance, getDevRevenue, withdrawRevenue };
