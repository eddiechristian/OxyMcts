use crate::{DefaultMcts, GameTrait};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use tracing::debug;

#[tracing::instrument]
pub fn mcts_uct_agent<Game: GameTrait + std::fmt::Debug >(state: &Game, playouts: usize, c: f64) -> Game::Move {
    let mut mcts = DefaultMcts::new(state);
    for i in 0..playouts {
        debug!("playout number: {:?}",i);
        mcts.execute(&c, ());
    }
    mcts.best_move(&c)
}

pub fn random_agent<Game: GameTrait>(state: &Game) -> Game::Move {
    state
        .legals_moves()
        .choose(&mut thread_rng())
        .unwrap()
        .clone()
}
