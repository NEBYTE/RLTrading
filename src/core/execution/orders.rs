use crate::core::types::{MarketState, OrderSide, Trade};
use crate::core::data::fetcher::fetch_exchange_data;
use crate::core::rl::agent::{create_rl_model, select_action};

use tch::nn;
use chrono::Utc;

pub fn place_order(action: OrderSide, amount: f64, price: f64) -> bool {
    println!(
        "{}: Placing {} order of {} at price {}",
        Utc::now(),
        match action {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
            _ => "HOLD",
        },
        amount,
        price
    );

    true
}
pub fn execute_trade(market_state: &MarketState) -> Option<Trade> {
    let vs = nn::VarStore::new(tch::Device::cuda_if_available());
    let action = select_action(&create_rl_model(&vs.root()), market_state);

    if let OrderSide::Buy | OrderSide::Sell = action {
        let price = market_state.price;
        let amount = 1.0;
        let action_clone = action.clone();

        let leverage = Some(1.0);
        let liquid_price = match leverage {
            Some(l) if l > 1.0 => {
                let entry_price = price;
                let margin = entry_price / l;
                if action_clone == OrderSide::Buy {
                    entry_price - margin
                } else {
                    entry_price + margin
                }
            }
            _ => 0.0,
        };

        place_order(action, amount, price);

        Some(Trade {
            id: Utc::now().timestamp() as u64,
            timestamp: Utc::now(),
            amount,
            price,
            side: action_clone,
            symbol: "BTC/USDT".to_string(),
            status: "executed".to_string(),
            fee: "0.001 BTC".to_string(),
            liquidated: false,
            liquidation_price: liquid_price,
            leverage: None,
            exchange_data: fetch_exchange_data(),
        })
    } else {
        None
    }
}