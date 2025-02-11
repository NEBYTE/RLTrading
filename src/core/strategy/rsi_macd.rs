use crate::core::types::OrderSide;

pub fn calculate_rsi(prices: &[f64], period: usize) -> f64 {
    if prices.len() < period + 1 { return 50.0; }

    let mut gains = 0.0;
    let mut losses = 0.0;

    for i in 1..=period {
        let change = prices[i] - prices[i - 1];
        if change > 0.0 {
            gains += change;
        } else {
            losses -= change;
        }
    }

    if losses == 0.0 { return 100.0; }
    if gains == 0.0 { return 0.0; }

    let rs = gains / losses;
    100.0 - (100.0 / (1.0 + rs))
}

pub fn calculate_macd(prices: &[f64]) -> (f64, f64, f64) {
    let short_ema = super::moving_avg::calculate_ema(prices, 12);
    let long_ema = super::moving_avg::calculate_ema(prices, 26);
    let macd = short_ema - long_ema;
    let signal = super::moving_avg::calculate_ema(&[macd], 9);
    let histogram = macd - signal;

    (macd, signal, histogram)
}

pub fn generate_signal_rsi_macd(prices: &[f64]) -> OrderSide {
    let rsi = calculate_rsi(prices, 14);
    let (macd, signal, _) = calculate_macd(prices);

    if rsi < 30.0 && macd > signal {
        OrderSide::Buy
    } else if rsi > 70.0 && macd < signal {
        OrderSide::Sell
    } else {
        OrderSide::Hold
    }
}
