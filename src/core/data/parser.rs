use serde_json::Value;
use crate::core::data::fetcher::fetch_exchange_data;
use crate::core::types::{Trade, OrderSide};

pub fn parse_trade_history(response: &Value) -> Vec<Trade> {
    response.as_array().unwrap().iter().map(|trade| Trade {
        id: trade["id"].as_u64().unwrap(),
        timestamp: chrono::Utc::now(),
        amount: trade["amount"].as_str().unwrap().parse::<f64>().unwrap(),
        price: trade["price"].as_str().unwrap().parse::<f64>().unwrap(),
        side: match trade["side"].as_str().unwrap() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => OrderSide::Unknown,
        },
        symbol: trade["symbol"].as_str().unwrap().to_string(),
        status: trade["status"].as_str().unwrap().to_string(),
        fee: trade["fee"].as_str().unwrap().to_string(),
        liquidated: trade["liquidated"].as_bool().unwrap(),
        liquidation_price: trade["liquidation_price"].as_f64().unwrap(),
        leverage: trade["leverage"].as_u64().map(|x| x as u8),
        exchange_data: fetch_exchange_data(),
    }).collect()
}