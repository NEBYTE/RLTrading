use crate::core::types::{MarketState, Trade, OrderSide};
use crate::core::execution::orders::place_order;
use crate::core::rl::agent::select_action;
use crate::core::data::fetcher::fetch_exchange_data;

use tch::nn;
use chrono::Utc;

pub fn run_backtest(states: &[MarketState]) -> Vec<Trade> {
    let mut simulated_trades = Vec::new();
    let vs = nn::VarStore::new(tch::Device::cuda_if_available());
    let model = crate::core::rl::agent::create_rl_model(&vs.root());

    for state in states {
        let action = select_action(&model, state);
        let trade = simulate_trade(state, action);
        simulated_trades.push(trade);
    }

    simulated_trades
}

pub fn simulate_trade(state: &MarketState, action: OrderSide) -> Trade {
    let price = state.price;
    let amount = 1.0;

    place_order(action.clone(), amount, price);

    Trade {
        id: Utc::now().timestamp() as u64,
        timestamp: Utc::now(),
        amount,
        price,
        side: action,
        symbol: "BTC/USDT".to_string(),
        status: "simulated".to_string(),
        fee: "0.001 BTC".to_string(),
        liquidated: false,
        liquidation_price: price,
        leverage: None,
        exchange_data: fetch_exchange_data(),
    }
}
