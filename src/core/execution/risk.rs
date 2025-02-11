pub fn stop_loss(entry_price: f64, current_price: f64, stop_loss_percent: f64) -> bool {
    let stop_loss_price = entry_price * (1.0 - stop_loss_percent / 100.0);
    current_price <= stop_loss_price
}

pub fn take_profit(entry_price: f64, current_price: f64, take_profit_percent: f64) -> bool {
    let take_profit_price = entry_price * (1.0 + take_profit_percent / 100.0);
    current_price >= take_profit_price
}

pub fn calculate_position_size(balance: f64, risk_percent: f64, stop_loss: f64) -> f64 {
    if stop_loss == 0.0 {
        return 0.0;
    }
    (balance * (risk_percent / 100.0)) / stop_loss
}
