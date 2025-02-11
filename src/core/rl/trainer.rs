use tch::{nn, Tensor, Device, nn::OptimizerConfig};
use tch::nn::Module;
use crate::core::types::{MarketState};
use crate::core::rl::agent::{create_rl_model, select_action, timeframe_to_numeric};
use crate::core::rl::environment::compute_reward;

pub fn train(states: Vec<MarketState>) {
    let device = Device::cuda_if_available();
    let vs = nn::VarStore::new(device);
    let model = create_rl_model(&vs.root());
    let mut optimizer = nn::Adam::default().build(&vs, 1e-3).unwrap();

    for _epoch in 0..1000 {
        for (i, state) in states.iter().enumerate() {
            let action = select_action(&model, state);

            let next_state = if i < states.len() - 1 {
                &states[i + 1]
            } else {
                &states[i]
            };

            let reward = compute_reward(action, state.price, next_state.price);

            let input = Tensor::from_slice(&[
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
            ]).view([1, 12]);

            let target = reward as f32 + 0.9 * model.forward(&input).max();
            let loss = model.forward(&input).smooth_l1_loss(&Tensor::from(target), tch::Reduction::Mean, 0.0);
            
            optimizer.backward_step(&loss)
        }
    }
}