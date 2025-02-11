use crate::core::types::OrderSide;

pub fn detect_levels(prices: &[f64]) -> (f64, f64) {
    if prices.is_empty() {
        return (0.0, 0.0);
    }

    let support = prices.iter().cloned().fold(f64::INFINITY, f64::min);
    let resistance = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    (support, resistance)
}

pub fn generate_signal_levels(prices: &[f64]) -> OrderSide {
    if prices.len() < 10 {
        return OrderSide::Hold;
    }

    let current_price = *prices.last().unwrap();
    let (support, resistance) = detect_levels(prices);

    if current_price <= support * 1.02 {
        OrderSide::Buy
    } else if current_price >= resistance * 0.98 {
        OrderSide::Sell
    } else {
        OrderSide::Hold
    }
}
