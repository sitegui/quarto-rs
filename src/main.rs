mod board;
mod environment;
mod player;
mod simple_players;
mod train;
mod traits;

use environment::*;
use player::*;
use train::*;

fn main() {
    let mut env = Environment::new();
    let mut player = QLearningPlayer::new();
    train(&mut env, &mut player, 20_000, 1_000, 50, 0.1);
}
