import { useState, useEffect, useRef } from 'react';
import { isConnected, getPublicKey } from '@stellar/freighter-api';

const BACKEND_URL = 'http://localhost:3001';

function App() {
  const [address, setAddress] = useState('');
  const [balance, setBalance] = useState('0');
  const [logs, setLogs] = useState([]);
  const [loading, setLoading] = useState(false);
  const [buyAmount, setBuyAmount] = useState('10');
  const consoleRef = useRef(null);

  // Auto-scroll console
  useEffect(() => {
    if (consoleRef.current) {
      consoleRef.current.scrollTop = consoleRef.current.scrollHeight;
    }
  }, [logs]);

  const addLog = (msg, type = 'normal') => {
    setLogs((prev) => [...prev, { time: new Date().toLocaleTimeString(), msg, type }].slice(-50));
  };

  const connectWallet = async () => {
    try {
      if (await isConnected()) {
        const pubKey = await getPublicKey();
        setAddress(pubKey);
        addLog(`Connected to Freighter: ${pubKey.substring(0, 6)}...${pubKey.substring(pubKey.length - 4)}`, 'highlight');
        fetchBalance(pubKey);
      } else {
        addLog("Freighter wallet extension not found.", "error");
        alert("Please install Freighter wallet extension.");
      }
    } catch (err) {
      addLog(`Wallet Connection Error: ${err.message}`, 'error');
    }
  };

  const fetchBalance = async (addr) => {
    try {
      const res = await fetch(`${BACKEND_URL}/balance/${addr}`);
      const data = await res.json();
      if (data.success) {
        setBalance(data.gate_balance);
        addLog(`Syncing on-chain state... Balance: ${data.gate_balance} GATE`);
      }
    } catch (err) {
      addLog(`RPC Sync Failed: ${err.message}`, 'error');
    }
  };

  const handleBuyTokens = async () => {
    if (!address) return alert("Connect wallet to initialize Soroban session.");
    addLog(`[TX_BUILD] Preparing 'buy_tokens' transaction for ${buyAmount} XLM...`, 'highlight');
    addLog(`[UX_NOTE] In production, Freighter would prompt for signature here.`);
    alert(`Testing Mode: Run this Soroban CLI command to mint tokens:\n\nstellar contract invoke --id YOUR_CONTRACT_ID --source ${address} --network testnet -- buy_tokens --buyer ${address} --xlm_amount ${buyAmount}0000000`);
  };

  const callApi = async (endpoint) => {
    if (!address) return alert("Connect wallet to authenticate.");
    setLoading(true);
    addLog(`[API_REQUEST] Sending request to endpoint: ${endpoint}...`);
    
    try {
      const res = await fetch(`${BACKEND_URL}${endpoint}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ userAddress: address })
      });
      
      const data = await res.json();
      if (data.success) {
        addLog(`[SOROBAN_TX_CONFIRMED] Hash: ${data.tx_hash.substring(0, 16)}...`, 'highlight');
        addLog(`[RESPONSE] ${JSON.stringify(data.data)}`);
        fetchBalance(address); 
      } else {
        addLog(`[API_REJECTED] ${data.error}`, 'error');
      }
    } catch (err) {
      addLog(`[NETWORK_FAULT] ${err.message}`, 'error');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="app-container">
      <header>
        <div className="logo-container">
          <h1>TokenGate</h1>
          <span className="text-muted">Decentralized Web3 API Gateway</span>
        </div>
        <button className={address ? 'outline' : ''} onClick={connectWallet}>
          {address ? `${address.substring(0, 4)}...${address.substring(address.length - 4)}` : 'Connect Freighter'}
        </button>
      </header>

      <div className="grid">
        {/* Left Column: Account & Gateway status */}
        <div className="card">
          <h2>My Account</h2>
          <div className="tag">Stellar Testnet</div>
          
          <div className="text-muted" style={{ marginTop: '1.5rem' }}>Available Credit</div>
          <div className="balance-display">
            {balance} <span className="balance-unit">GATE</span>
          </div>
          
          <div style={{ margin: '2rem 0', height: '1px', background: 'var(--border-color)' }} />
          
          <h2>Acquire Credits</h2>
          <p className="text-muted" style={{ marginBottom: '1rem' }}>
            Exchange Rate: 1 GATE = 0.1 XLM.<br/>Tokens are held in secure on-chain storage.
          </p>
          <div className="input-group">
            <input 
              type="number" 
              value={buyAmount} 
              onChange={(e) => setBuyAmount(e.target.value)} 
              placeholder="Amount in XLM"
            />
            <button className="buy-btn" onClick={handleBuyTokens}>Buy Tokens</button>
          </div>
        </div>

        {/* Right Column: API Services */}
        <div className="card">
          <h2>API Marketplace</h2>
          <p className="text-muted" style={{ marginBottom: '1.5rem' }}>
            Atomic micro-payments via Soroban Smart Contracts. No subscriptions required.
          </p>

          <div className="api-item">
            <div className="api-header">
              <span className="api-title">Weather Oracle Feed</span>
              <span className="tag">1 GATE / call</span>
            </div>
            <p className="text-muted" style={{ marginBottom: '1rem' }}>
              Real-time climatic data feed aggregated from decentralized sensors.
            </p>
            <button onClick={() => callApi('/api/weather')} disabled={loading || !address}>
              {loading ? 'Processing...' : 'Execute Request'}
            </button>
          </div>

          <div className="api-item">
            <div className="api-header">
              <span className="api-title">AI Text Synthesis</span>
              <span className="tag premium">5 GATE / call</span>
            </div>
            <p className="text-muted" style={{ marginBottom: '1rem' }}>
              High-performance inference engine for text generation tasks.
            </p>
            <button onClick={() => callApi('/api/ai-text')} disabled={loading || !address}>
              {loading ? 'Processing...' : 'Generate Text'}
            </button>
          </div>
        </div>
      </div>

      {/* Terminal UI */}
      <div className="console-wrapper">
        <div className="console-header">
          <div className="dot r"></div>
          <div className="dot y"></div>
          <div className="dot g"></div>
          <span style={{ marginLeft: '10px', fontSize: '0.85rem', color: 'var(--text-muted)' }}>
            soroban-rpc-terminal
          </span>
        </div>
        <div className="console" ref={consoleRef}>
          {logs.length === 0 ? '> Initializing Soroban context... Waiting for wallet connection.' : 
            logs.map((l, i) => (
              <div key={i} className={`console-line ${l.type}`}>
                <span className="time">[{l.time}]</span>
                <span>{l.msg}</span>
              </div>
          ))}
        </div>
      </div>
    </div>
  );
}

export default App;
