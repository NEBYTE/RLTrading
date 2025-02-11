use crate::core::types::Trade;

pub fn calculate_total_profit(trades: &[Trade]) -> f64 {
    let mut total_profit = 0.0;
    for i in 0..trades.len() - 1 {
        if trades[i].side == trades[i + 1].side {
            continue;
        }

        let price_diff = trades[i + 1].price - trades[i].price;
        total_profit += match trades[i].side {
            crate::core::types::OrderSide::Buy => price_diff,
            crate::core::types::OrderSide::Sell => -price_diff,
            _ => 0.0,
        };
    }
    total_profit
}

pub fn calculate_win_rate(trades: &[Trade]) -> f64 {
    let mut wins = 0;
    let mut total = 0;

    for i in 0..trades.len() - 1 {
        if trades[i].side == trades[i + 1].side {
            continue;
        }

        let price_diff = trades[i + 1].price - trades[i].price;
        if (trades[i].side == crate::core::types::OrderSide::Buy && price_diff > 0.0)
            || (trades[i].side == crate::core::types::OrderSide::Sell && price_diff < 0.0)
        {
            wins += 1;
        }
        total += 1;
    }

    if total == 0 {
        0.0
    } else {
        (wins as f64 / total as f64) * 100.0
    }
}

pub fn analyze_backtest(trades: &[Trade]) {
    let total_profit = calculate_total_profit(trades);
    let win_rate = calculate_win_rate(trades);

    println!("Backtest Results:");
    println!("Total Profit: {:.2} USD", total_profit);
    println!("Win Rate: {:.2}%", win_rate);
}
