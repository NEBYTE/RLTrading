<img src="https://l7mozmkiwy.ufs.sh/f/HKemhjN71TyOBDSBjRYWE0OaYPF9Vq4jUDItmN6JuXrkiTAe" alt="Crypto Trading Bot">

# RL Trading (v0.1.0-pre.alpha.1)

[![Maintainer](https://img.shields.io/badge/maintainer-rustyspottedcatt-blue)](https://github.com/carlos-crypto)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-GNU_AGPLv3-blue)](https://choosealicense.com/licenses/agpl-3.0/)

> [!NOTE]
> This is a **fully autonomous AI-powered crypto trading bot** built in Rust. It utilizes **Reinforcement Learning (RL)** to optimize trading decisions. For educational purposes.

The AI Crypto Trading Bot leverages **Deep Q-Networks (DQN)** and **technical indicators (RSI, MACD, Moving Averages)** to execute trades with **high efficiency**. It supports **experimental trading, backtesting, and risk management**.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Live Trading](#live-trading)
  - [Backtesting](#backtesting)
- [Technical Overview](#technical-overview)
- [Dependencies](#dependencies)
- [License](#license)

---

## Features

> [!WARNING]
> **This bot is experimental.** While it uses AI for decision-making, it is not guaranteed to be profitable. Use at your own risk!
> This bot has not been tested before, it is not even connected to any sort of exchange API. This is only for learning purposes.

- **Fully Autonomous AI Trading** - Uses RL to trade **without manual intervention**.
- **Backtesting System** - Simulates past trades to evaluate performance.
- **Live Trading Mode** - Executes real trades via exchange APIs. (Needs to be implemented)
- **Multi-Indicator Strategy** - Uses **RSI, MACD, Moving Averages, and Support/Resistance**.
- **Risk Management** - Implements **stop-loss and take-profit**.

---

## Installation

### Prerequisites

- Rust (latest stable)
- Cargo package manager
- PostgreSQL (for trade history storage, check src/core/database/storage.rs)

### Clone the Repository

```sh
git clone https://github.com/NEBYTE/RLTrading.git
cd RLTrading
```

### Setup .env variables

```env
DATABASE_URL=postgres://user:password@localhost/trade_history
API_KEY= # BINANCE API KEY -> More exchanges will be implemented in the future
```

### Build the Project

```sh
cargo build --release
```

---

## Usage

### Live Trading

```sh
cargo run --release
```

---

### Backtesting

To evaluate AI performance on past market data:

```sh
cargo run --release -- backtest
```

---

## Technical Overview

### 🏦 **Reinforcement Learning Model**
- **Deep Q-Network (DQN)**
- **State** → Market indicators (price, volume, RSI, MACD, moving average, etc.)
- **Actions** → Buy, Sell, Hold
- **Reward Function** → Based on profit/loss

### 📈 **Trading Strategies**
- **Moving Average Crossover (SMA/EMA)**
- **RSI & MACD Signals**
- **Support & Resistance Level Strategy**
- **Fear & Greed Index (TODO)**

### 🛠 **Risk Management**
- **Stop-Loss & Take-Profit**

### 📊 **Backtesting Engine**
- **Simulates trades on past market data**
- **Calculates total profit & win rate**
- **Stores results in PostgreSQL database**

---

## Dependencies

```toml
[dependencies]
tokio = { version = "1.28", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "blocking"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tch = "0.19.0"
chrono = { version = "0.4", features = ["serde"]}
rust_decimal = "1.29"
sqlx = { version = "0.6", features = ["runtime-tokio-rustls", "postgres", "macros"] }
log = "0.4"
env_logger = "0.10"
```

---

## License

Distributed under the [GNU AGPLv3](https://choosealicense.com/licenses/agpl-3.0/) license.