# Web3 Valet — Voice-First Web3 Assistant

A modular, voice-first Web3 assistant platform that integrates voice interaction, decentralized identity, and NFT functionality — powered by **Rust**, **Node.js**, and **React**.

---

## 🚀 Overview

**Web3 Valet** is a multi-component system that leverages MCP (Model Context Protocol) for multi-agent orchestration, enabling users to interact with the blockchain using natural language and voice commands.  
It brings together speech processing, multi-agent orchestration, and on-chain operations (wallet, NFT minting, token transfers) through a unified backend and voice interface.

Unlike traditional assistants, Web3 Valet focuses on **voice-first decentralized interaction**, built from the ground up with extensibility, modularity, and EVM compatibility in mind.

---

## ✨ Features

- 🎤 **Voice Input** — Speak commands or upload audio files to control Web3 actions  
- 🧠 **Multi-Agent Architecture** — Modular design for task-specific agents (minting, wallet, transaction)  
- 🗣️ **Speech Synthesis** — Text-to-speech for real-time voice responses  
- ⛓️ **Web3 Integration** — Wallet management, NFT minting, and token transfers  
- ⚙️ **Rust Core Server** — High-performance backend for blockchain communication  
- 🔗 **Node API Layer** — Middleware between frontend and Rust server  
- 💻 **React Frontend** — Clean UI for interacting with voice commands and blockchain data  
- 🌐 **Multi-Chain Support** — Works across Ethereum and EVM-compatible chains  

---

## ⚙️ Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/Meherajs/web3-valet.git
   cd web3-valet
2. **Setup Environment variables**
   ```bash
   cp .env.example .env
3. **Install dependencies**
   ```bash
   cd frontend && npm install
   cd ../mcp-api && cargo build
   cd ../mcp-server && cargo build
4. **Start the services**
   ```bash
   # In separate terminals
    cd mcp-server && cargo run
    cd ../mcp-api && cargo run
    cd ../frontend && npm start
5. **Open the application**
   
   Navigate to http://localhost:5173

    Speak or type a command like:
    
    “Check my wallet balance”
    “Mint a new NFT named ValetPass”
    “Send 0.1 ETH to 0xABC…”
     
