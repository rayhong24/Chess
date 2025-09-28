use std::collections::HashMap;

pub struct GameStateTracker {
    state_counts: HashMap<u64, u32>, // Zobrist hash -> count
}

impl GameStateTracker {
    pub fn new() -> Self {
        Self { state_counts: HashMap::new() }
    }

    pub fn record_position(&mut self, hash: u64) {
        let counter = self.state_counts.entry(hash).or_insert(0);
        *counter += 1;
    }

    pub fn is_threefold_repetition(&self, hash: u64) -> bool {
        self.state_counts.get(&hash).cloned().unwrap_or(0) >= 3
    }
}
