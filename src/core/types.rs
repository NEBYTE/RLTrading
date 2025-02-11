use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct ExchangeData {
    pub name: Option<String>,
    pub exchange_id: String,
    pub exchange_name: Option<String>,
    pub exchange_url: Option<String>,
    pub exchange_logo: Option<String>,
    pub exchange_timezone: Option<String>,
    pub exchange_timezone_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Timeframe {
    M1,
    M5,
    M15,
    M30,
    H1,
    H4,
    D1,
    W1,
    m1,
    m6,
}

#[derive(Serialize, Deserialize)]
pub struct MarketState {
    pub price: f64,
    pub volume: f64,
    pub rsi: f64,
    pub macd: f64,
    pub moving_avg: f64,
    pub volatility: f64,
    pub high: f64,
    pub low: f64,
    pub ath: f64,
    pub atl: f64,
    pub timeframe: Timeframe,
    pub fear_and_greed: f64,
    pub trades: Vec<Trade>,
}

#[derive(Serialize, Deserialize)]
pub struct OrderBook {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderSide {
    Buy,
    Sell,
    Hold,
    Unknown,
}

#[derive(Serialize, Deserialize)]
pub struct Trade {
    pub id: u64,
    pub timestamp: DateTime<Utc>,
    pub amount: f64,
    pub price: f64,
    pub side: OrderSide,
    pub symbol: String,
    pub status: String,
    pub fee: String,
    pub liquidated: bool,
    pub liquidation_price: f64,
    pub leverage: Option<u8>,
    pub exchange_data: ExchangeData,
}