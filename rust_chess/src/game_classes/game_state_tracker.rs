use std::collections::HashMap;

use crate::game_classes::{board_classes::board::Board, game_state::GameState, zobrist::Zobrist};

pub struct GameStateTracker {
    state_counts: HashMap<u64, u32>,
}

impl GameStateTracker {
    pub fn new() -> Self {
        Self{ state_counts: HashMap::new() }
    }

    pub fn clear(&mut self) {
        self.state_counts.clear();
    }

    pub fn record_position(&mut self, hash: u64) {
        let counter = self.state_counts.entry(hash).or_insert(0);
        *counter += 1;
    }

    pub fn is_threefold_repetition(&self, hash: u64) -> bool {
        self.state_counts.get(&hash).cloned().unwrap_or(0) >= 3
    }

}
