use crate::ai::tuner::AiTuner;

/// Matchmaking & Lobby System with AI skill curve balancing
pub struct Matchmaker {
    base_elo_tolerance: u32,
    active_elo_tolerance: u32,
}

impl Matchmaker {
    pub fn new(base_elo_tolerance: u32) -> Self {
        Self { 
            base_elo_tolerance,
            active_elo_tolerance: base_elo_tolerance,
        }
    }

    /// Dynamically expands matchmaking ELO brackets during low player counts to prevent infinite lobby waits
    pub fn optimize_with_ai(&mut self, tuner: &AiTuner, active_player_pool: u32) {
        self.active_elo_tolerance = tuner.tune_matchmaking_elo_tolerance(active_player_pool, self.base_elo_tolerance);
    }

    pub fn find_match(&self, player_elo: u32, pool: &[u32]) -> Option<u32> {
        pool.iter()
            .find(|&&opponent_elo| opponent_elo.abs_diff(player_elo) <= self.active_elo_tolerance)
            .copied()
    }
}
