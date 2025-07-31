# ₿ bitcoin-rust

A learning-driven, end‑to‑end cryptocurrency prototype in Rust—built to understand, from first principles, how keys, signatures, mining, difficulty, mempools, propagation, and incentives come together to form a working blockchain.

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-1A1A1A?style=for-the-badge&logo=tokio&logoColor=white)
![Actix%20Web](https://img.shields.io/badge/Actix%20Web-1F2E2E?style=for-the-badge&logo=rust&logoColor=white)
![WebSockets](https://img.shields.io/badge/WebSockets-02569B?style=for-the-badge&logo=websocket&logoColor=white)
![k256](https://img.shields.io/badge/k256-0B7285?style=for-the-badge&logo=ethereum&logoColor=white)
![SHA--256](https://img.shields.io/badge/SHA--256-333333?style=for-the-badge&logo=protonvpn&logoColor=white)
![Serde](https://img.shields.io/badge/Serde-4B5563?style=for-the-badge&logo=rust&logoColor=white)
![Cargo](https://img.shields.io/badge/Cargo-2F2F2F?style=for-the-badge&logo=rust&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-3B82F6?style=for-the-badge&logo=open-source-initiative&logoColor=white)

---

## 💡 Why I Built This (Passion)

I have always been deeply passionate about cryptocurrencies and the technology behind them. The idea that money can be decentralized, trustless, and governed purely by protocols has fascinated me for years. This project is my way of diving deep into how cryptocurrencies work at the protocol level—consensus, cryptography, mining, mempools, and incentives. I wanted to go beyond just reading about it and actually build the entire stack myself from scratch, experimenting, breaking things, and iterating until I truly understood how all the pieces fit together. This repository serves as my hands‑on study log, where I focus on writing clean, idiomatic Rust with explicit state, deterministic behavior, and small, testable components. I believe in learning in public, so feedback, reviews, and issues are always welcome as they help me refine my understanding of distributed systems and security.

---

## 🚀 Core Features
- **Proof‑of‑Work blockchain** with nonce search and dynamic difficulty targeting a configurable `MINE_RATE`.
- **Mining reward **: the block subsidy reward the miner.
- **Transactions & signatures**: ECDSA over secp256k1 (`k256`), outputs model, and per‑tx signatures over output hashes.
- **Mempool (TransactionPool)**: collect pending transactions, validate, and feed the miner.
- **Coinbase (reward) transactions**: distinct type alongside normal transfers.
- **Balance calculation**: scans chain with “last spend then forward credits” semantics per address.
- **P2P via WebSockets**: **explicitly uses WebSockets (tokio‑tungstenite)**—not raw TCP—to simplify peer connectivity and dev ergonomics.
- **Broadcast on add**: when a block is mined and appended, it is **immediately broadcast** to peers to converge network state.
- **HTTP API (Actix Web)**: endpoints for blocks, transactions, balances, and mining controls.
- **CORS enabled**: easy local experimentation with frontends and tools.

---

## 🧱 Architecture (High Level)
- **Node services**
  - HTTP server (Actix Web) for REST endpoints and quick tooling.
  - P2P subsystem (WebSockets) to connect, synchronize, and broadcast.
  - Blockchain state: `Block`, `Blockchain`, difficulty/PoW, halving-aware rewards.
  - Wallet: keypair (signing), balance computation, transaction creation.
  - Mempool: pending transactions with verification before inclusion.
  - Miner: assembles valid txs + coinbase, performs PoW, appends, **broadcasts added block**, clears mempools.
- **Data flow**
  - Wallet → Mempool: signed transaction submission.
  - Miner → Block: pick valid txs + reward, mine, append.
  - Node → Peers: **broadcast added block** (and clear mempool).
  - Peers → Node: receive `CHAIN` messages and reconcile via replace strategy.

---

## 🌐 P2P Protocol (WebSockets)
- **Message types**
  - `CHAIN`: send/receive full chain; peers may `replace_chain` if appropriate.
  - `TRANSACTION`: broadcast pending transactions to all peers.
  - `CLEAR_TRANSACTIONS`: signal mempool clear after block acceptance.
- **Connection behavior**
  - Auto‑connect to peers from `PEERS` on startup.
  - On connect, a node **sends its current chain** to the new peer.
  - On mining or chain update, nodes **broadcast the added block** and clear mempools.

---

## 📡 HTTP API (Essentials)
- `GET /blocks` — return full chain.
- `POST /mine` — mine a block from provided `data` (experimental).
- `GET /transactions` — list mempool transactions.
- `POST /transact` — create a signed tx `{ recipient, amount }`.
- `GET /mine-transactions` — mine current mempool to a new block.
- `GET /publickey` — node wallet public key.
- `GET /balance` — balance for node wallet.
- `GET /balance/{pubkey}` — balance for any address.
- `GET /startmining` / `GET /stopmining` — toggle background mining.
- `GET /mining-status` — check background mining status.

---

## 📝 What I Missed & Plan for Next Time

- **Transaction Fees**  
  - Currently, there are no transaction fees implemented. Adding fees will incentivize miners beyond the block subsidy and allow prioritization of transactions.

- **Full Chain Propagation**  
  - Right now, the entire chain is sent to peers whenever a block is created. While this works for a small test network, in real-world blockchains only the new block (or missing blocks) are propagated. Next time, I will optimize propagation to reduce unnecessary data transfer.

- **Improved P2P Communication**  
  - Currently, the peer-to-peer communication is built using `TcpStream` and `Listener` but wrapped via WebSockets for simplicity. Next iteration, I plan to experiment with raw TCP, as well as explore using **UDP** and **QUIC** (like Solana) to achieve lower latency and more robust networking.

---

## ⚙️ Configuration
- `HTTP_PORT` — HTTP server port (default `3001`).
- `P2P_PORT` — WebSocket peer port (default `5001`).
- `PEERS` — comma‑separated WebSocket URLs (e.g., `ws://127.0.0.1:5001,ws://127.0.0.1:5002`).
- `DIFFICULTY` — initial PoW difficulty (default `4`).
- `MINE_RATE` — target mining interval in ms (default `8000`).
- `MINING_REWARD` — initial block subsidy before **halving** applies.

---
## 🏃 Quickstart (Multi‑Node Local Net)

- **Clone the repository and build**
  - `git clone <repo_url>`
  - `cd bitcoin-rust`
  - `cargo build`

---

- **Run three nodes (one per terminal)**  
  - Terminal 1  
    - `cargo run --bin bitcoin_rust`
  - Terminal 2  
    - `HTTP_PORT=3002 P2P_PORT=5002 PEERS=ws://127.0.0.1:5001 cargo run --bin bitcoin_rust`
  - Terminal 3  
    - `HTTP_PORT=3003 P2P_PORT=5003 PEERS=ws://127.0.0.1:5001,ws://127.0.0.1:5002 cargo run --bin bitcoin_rust`

---

## 🖥️ Frontend (per‑node UI)

- `cd frontend`
- `npm install`
- Run three dev servers pointing to each node (one per terminal)
  - `VITE_API_BASE=http://localhost:3001 npm run dev -- --port 5173`
  - `VITE_API_BASE=http://localhost:3002 npm run dev -- --port 5174`
  - `VITE_API_BASE=http://localhost:3003 npm run dev -- --port 5175`


---

## 🖼️ Frontend Screenshots
- **Dashboard (per‑node overview)**
<img width="400" height="311" alt="Screenshot 2025-07-30 at 23 28 41" src="https://github.com/user-attachments/assets/9792aa3e-a6db-4a90-ba52-a027bef86e33" />

- **Start Mining (in every node)**
<img width="400" height="311" alt="Screenshot 2025-07-30 at 23 28 54" src="https://github.com/user-attachments/assets/3865b734-0190-4317-bb27-648c71845875" />

- **Block Dashboard (same in every node (Synchronization))**
<img width="400" height="311" alt="Screenshot 2025-07-30 at 23 29 13" src="https://github.com/user-attachments/assets/c0b05fe5-0717-40c6-b5f3-c55e4a509b86" />
<img width="400" height="311" alt="Screenshot 2025-07-30 at 23 29 13" src="https://github.com/user-attachments/assets/6139053e-6622-496c-83c0-bceb3a6fb764" />

- **Wallet**
<img width="400" height="311" alt="Screenshot 2025-07-30 at 23 30 05" src="https://github.com/user-attachments/assets/c54ef978-81ca-49b2-8710-8e2ce080354a" />

- **Transaction**
<img width="400" height="311" alt="Screenshot 2025-07-30 at 23 30 05" src="https://github.com/user-attachments/assets/6b208cda-1859-4622-bc86-99aa6f7655f2" />

- **Transaction Pool**
<img width="400" height="311" alt="Screenshot 2025-07-30 at 23 30 05" src="https://github.com/user-attachments/assets/33e00ccd-b4c6-4f11-96f4-8917de65b897" />

- **Direct Api Calls**
<img width="400" height="340" alt="Screenshot 2025-07-30 at 23 36 35" src="https://github.com/user-attachments/assets/7fc89c31-91bf-44bc-acb8-919d9c262cf9" />
<img width="400" height="340" alt="Screenshot 2025-07-30 at 23 37 03" src="https://github.com/user-attachments/assets/b7bf9622-3615-4109-b4ae-6c0abfeb9a81" />
<img width="400" height="340" alt="Screenshot 2025-07-30 at 23 37 22" src="https://github.com/user-attachments/assets/41950878-aab1-4a76-81b6-75fd82416318" />


---

## 🧪 Usage Tips
- Create a tx: `POST /transact` with `{ "recipient": "<pubkey>", "amount": 10 }`.
- Force a one‑off mine: `GET /mine-transactions`.
- Start background mining: `GET /startmining` (blocks will **broadcast on add**).
- Check balances: `GET /balance/{pubkey}` after mining.
- Observe P2P sync: logs show `sync_chains`, broadcasts, and mempool clears.

---

## 🧭 Learning Notes & Intent
- Clarity over complexity: minimal PoW + WebSockets keeps focus on fundamentals.
- Protocol literacy: see **signatures → mempool → mining → broadcast → reconciliation**.
- Observability: log points highlight mining loop, broadcasts, and chain replacement.
- Extensibility: room for UTXO sets, fees, stronger validation, and richer P2P.

---

## 🛡️ Limitations & Roadmap
- Strengthen validation: enforce PoW target, difficulty delta bounds, and **single coinbase with halving‑correct reward**.
- Improve mempool: handle multi‑spend conflicts within the same block.
- DoS hardening: size/rate limits on P2P messages.
- Cumulative‑work selection: prefer total work over length.
- Fees & prioritization: incentivize inclusion beyond subsidy.
- Peer management: retries, backoff, scoring.
- Tooling: CLI/GUI for keys, tx crafting, dashboards.
- Containerization: Docker setup for quick network spins.

---


## 📄 License
- MIT — permissive for learning, experimentation, and extension.

---

## 📬 Contact
- Email: **maheshkarri2109@gmail.com**
- LinkedIn: **https://www.linkedin.com/in/maheshkarri4444/**
