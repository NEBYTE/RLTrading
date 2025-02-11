use crate::core::types::OrderSide;

pub fn compute_reward(action: OrderSide, previous_price: f64, current_price: f64) -> f64 {
    match action {
        OrderSide::Buy => (current_price - previous_price),
        OrderSide::Sell => (previous_price - current_price),
        _ => 0.0,
    }
}