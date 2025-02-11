pub mod core;

use crate::core::backtest::simulator::run_backtest;
use crate::core::backtest::results::analyze_backtest;
use crate::core::database::storage::{save_trade};
use crate::core::execution::orders::execute_trade;
use crate::core::types::{MarketState, Timeframe};

use tokio::runtime::Runtime;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let api_key = env::var("API_KEY").expect("API_KEY not found");

    println!("Database URL: {}", database_url);
    println!("API Key: {}", api_key);

    let rt = Runtime::new().unwrap();

    println!("Welcome to the RL Trading Bot!");
    println!("Please select an option:");
    println!("1: Run Backtest");
    println!("2: Start Live Trading (doesn't work on v0.1.0-pre.alpha.1)");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();

    match choice {
        "1" => run_backtest_mode(),
        "2" => rt.block_on(run_trading_bot()),
        _ => println!("Invalid choice, exiting..."),
    }
}

fn load_market_data() -> Vec<MarketState> {
    let mut states = Vec::new();

    for i in 0..100 {
        states.push(MarketState {
            price: 30000.0 + (i as f64 * 10.0),
            volume: 1000.0,
            rsi: 50.0,
            macd: 0.0,
            moving_avg: 29500.0,
            volatility: 2.0,
            high: 31000.0,
            low: 29000.0,
            ath: 69000.0,
            atl: 3000.0,
            timeframe: Timeframe::H1,
            fear_and_greed: 50.0,
            trades: Vec::new(),
        });
    }

    states
}

async fn run_trading_bot() {
    let market_states = load_market_data();

    for state in market_states {
        if let Some(trade) = execute_trade(&state) {
            println!("Trade executed: {:?}", trade.id);
            save_trade(&trade).await;
        }
    }

    println!("Live trading completed.");
}

fn run_backtest_mode() {
    let market_states = load_market_data();
    let trades = run_backtest(&market_states);

    analyze_backtest(&trades);
}