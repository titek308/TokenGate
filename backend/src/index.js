require("dotenv").config();
const express = require("express");
const cors    = require("cors");
const { consumeToken, getGateBalance, getDevRevenue, withdrawRevenue } = require("./stellar");

const app  = express();
const PORT = process.env.PORT || 3001;

app.use(cors());
app.use(express.json());

if (!process.env.DEV_SECRET_KEY || !process.env.CONTRACT_ID) {
  console.error("ERROR: Set DEV_SECRET_KEY and CONTRACT_ID in .env");
  process.exit(1);
}

// Health check
app.get("/", (req, res) => {
  const { Keypair } = require("@stellar/stellar-sdk");
  res.json({
    service: "TokenGate API Backend",
    network: process.env.STELLAR_NETWORK || "testnet",
    contract: process.env.CONTRACT_ID,
    dev_address: Keypair.fromSecret(process.env.DEV_SECRET_KEY).publicKey(),
    endpoints: ["/balance/:address", "/api/weather", "/api/ai-text", "/dev/revenue", "/dev/withdraw"],
  });
});

// GET /balance/:address
app.get("/balance/:address", async (req, res) => {
  try {
    const balance = await getGateBalance(req.params.address);
    res.json({ success: true, address: req.params.address, gate_balance: balance.toString() });
  } catch (err) {
    res.status(400).json({ success: false, error: err.message });
  }
});

// POST /api/weather  — 1 GATE token
app.post("/api/weather", async (req, res) => {
  const { userAddress } = req.body;
  if (!userAddress) return res.status(400).json({ success: false, error: "userAddress required" });

  try {
    const balance = await getGateBalance(userAddress);
    if (BigInt(balance) < 1n) {
      return res.status(402).json({ success: false, error: "Insufficient GATE tokens. Buy tokens first." });
    }

    console.log(`[consume_token] user=${userAddress} endpoint=weather`);
    const tx = await consumeToken(userAddress, "weather");

    res.json({
      success: true,
      tx_hash: tx.hash,
      endpoint: "weather",
      cost: "1 GATE token",
      data: {
        city: "Ho Chi Minh City",
        temperature_c: 32,
        humidity_pct: 75,
        condition: "Partly Cloudy",
        wind_kmh: 14,
        uv_index: 7,
      },
    });
  } catch (err) {
    console.error("[/api/weather]", err.message);
    if (err.message.includes("Insufficient") || err.message.includes("allowance")) {
      return res.status(402).json({ success: false, error: "No GATE tokens or approval not set." });
    }
    res.status(500).json({ success: false, error: err.message });
  }
});

// POST /api/ai-text  — 5 GATE tokens
app.post("/api/ai-text", async (req, res) => {
  const { userAddress, prompt } = req.body;
  if (!userAddress || !prompt) {
    return res.status(400).json({ success: false, error: "userAddress and prompt required" });
  }

  try {
    const balance = await getGateBalance(userAddress);
    if (BigInt(balance) < 5n) {
      return res.status(402).json({ success: false, error: "Need at least 5 GATE tokens for this endpoint." });
    }

    console.log(`[consume_token] user=${userAddress} endpoint=ai-text`);
    const tx = await consumeToken(userAddress, "ai-text");

    res.json({
      success: true,
      tx_hash: tx.hash,
      endpoint: "ai-text",
      cost: "5 GATE tokens",
      data: {
        prompt,
        response: `You asked: "${prompt}". This response was generated and billed automatically via a Soroban smart contract on Stellar. Each request atomically burns GATE tokens from your wallet and transfers XLM revenue to the developer — no subscription, no invoice, just on-chain micro-payments.`,
        tokens_consumed: 5,
      },
    });
  } catch (err) {
    console.error("[/api/ai-text]", err.message);
    if (err.message.includes("Insufficient") || err.message.includes("allowance")) {
      return res.status(402).json({ success: false, error: "No GATE tokens or approval not set." });
    }
    res.status(500).json({ success: false, error: err.message });
  }
});

// GET /dev/revenue
app.get("/dev/revenue", async (req, res) => {
  try {
    const stroops = await getDevRevenue();
    res.json({
      success: true,
      revenue_stroops: stroops.toString(),
      revenue_xlm: (Number(stroops) / 10_000_000).toFixed(7),
    });
  } catch (err) {
    res.status(500).json({ success: false, error: err.message });
  }
});

// POST /dev/withdraw
app.post("/dev/withdraw", async (req, res) => {
  try {
    console.log("[withdraw_revenue] initiating...");
    const result = await withdrawRevenue();
    res.json({ success: true, tx_hash: result.hash });
  } catch (err) {
    console.error("[/dev/withdraw]", err.message);
    res.status(500).json({ success: false, error: err.message });
  }
});

app.listen(PORT, () => {
  const { Keypair } = require("@stellar/stellar-sdk");
  console.log(`\nTokenGate Backend → http://localhost:${PORT}`);
  console.log(`Contract : ${process.env.CONTRACT_ID}`);
  console.log(`Dev      : ${Keypair.fromSecret(process.env.DEV_SECRET_KEY).publicKey()}\n`);
});
