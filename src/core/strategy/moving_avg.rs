use crate::core::types::OrderSide;

pub fn calculate_sma(prices: &[f64], period: usize) -> f64 {
    if prices.len() < period {
        return prices.iter().sum::<f64>() / prices.len() as f64;
    }

    let sum = prices.iter().take(period).sum::<f64>();
    sum / period as f64
}

pub fn calculate_ema(prices: &[f64], period: usize) -> f64 {
    if prices.is_empty() { return 0.0; }

    let smoothing = 2.0 / (period as f64 + 1.0);
    let mut ema = prices[0];

    for &price in prices.iter().skip(1) {
        ema = (price * smoothing) + (ema * (1.0 - smoothing))
    }

    ema
}

pub fn generate_signal(prices: &[f64]) -> OrderSide {
    if prices.len() < 20 { return OrderSide::Hold; }

    let short_sma = calculate_sma(prices, 10);
    let long_sma = calculate_sma(prices, 20);

    if short_sma > long_sma {
        OrderSide::Buy
    } else if short_sma < long_sma {
        OrderSide::Sell
    } else {
        OrderSide::Hold
    }
}