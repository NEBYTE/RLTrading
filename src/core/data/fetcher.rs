use serde_json::Value;
use crate::core::types::{ExchangeData, OrderBook, Trade};

pub async fn fetch_price(symbol: &str) -> Result<f64, reqwest::Error> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    let response = reqwest::get(&url).await?.json::<Value>().await;
    let price = response["price"].as_str().unwrap().parse::<f64>().unwrap();
    Ok(price)
}

pub async fn fetch_exchange_data() -> Result<ExchangeData, reqwest::Error> {
    let exchange_data = ExchangeData {
        name: None,
        exchange_name: Option::from("Binance".to_string()),
        exchange_url: None,
        exchange_logo: None,
        exchange_timezone: None,
        exchange_id: "1".to_string(),
        exchange_timezone_name: None,
    };
    Ok(exchange_data)
}

pub async fn fetch_orderbook(symbol: &str) -> Result<OrderBook, reqwest::Error> {
    let url = format!("https://api.binance.com/api/v3/depth?symbol={}&limit=10", symbol);
    let response = reqwest::get(&url).await?.json::<Value>().await;

    let bids = response["bids"].as_array()
        .unwrap()
        .iter()
        .map(|bid| (bid[0].as_str().unwrap().parse::<f64>().unwrap(), bid[1].as_str().unwrap().parse::<f64>().unwrap()))
        .collect();

    let asks = response["asks"].as_array()
        .unwrap()
        .iter()
        .map(|ask| (ask[0].as_str().unwrap().parse::<f64>().unwrap(), ask[1].as_str().unwrap().parse::<f64>().unwrap()))
        .collect();

    Ok(OrderBook { bids, asks })
}