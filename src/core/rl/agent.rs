use tch::{nn, Tensor, Device, Kind};
use tch::nn::Module;
use crate::core::types::{MarketState, OrderSide, Timeframe};

pub fn timeframe_to_numeric(timeframe: &Timeframe) -> f64 {
    match timeframe {
        Timeframe::M1 => 1.0,
        Timeframe::M5 => 5.0,
        Timeframe::M15 => 15.0,
        Timeframe::H1 => 60.0,
        Timeframe::H4 => 240.0,
        Timeframe::D1 => 1440.0,
        Timeframe::W1 => 10080.0,
        Timeframe::m1 => 43200.0,
        Timeframe::m6 => 259200.0,
        _ => 0.0,
    }
}
pub fn create_rl_model(vs: &nn::Path) -> nn::Sequential {
    nn::seq()
        .add(nn::linear(vs, 12, 128, Default::default()))
        .add_fn(|x| x.relu())
        .add(nn::linear(vs, 128, 3, Default::default()))
}

pub fn select_action(model: &nn::Sequential, state: &MarketState) -> OrderSide {
    let input = Tensor::of_slice(&[
        state.price as f32,
        state.volume as f32,
        state.rsi as f32,
        state.macd as f32,
        state.moving_avg as f32,
        state.volatility as f32,
        state.high as f32,
        state.low as f32,
        state.ath as f32,
        state.atl as f32,
        state.fear_and_greed as f32,
        state.trades.len() as f32,
        timeframe_to_numeric(&state.timeframe) as f32,
    ]).view(&[1, 12]);

    let q_values = model.forward(&input);
    let action_index = q_values.argmax(1, false).int64_value(&[]);

    match action_index {
        0 => OrderSide::Buy,
        1 => OrderSide::Sell,
        _ => OrderSide::Hold,
    }
}