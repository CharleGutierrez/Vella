/// Matchmaking & Lobby System with AI skill curve balancing
pub struct Matchmaker {
    elo_tolerance: u32,
}

impl Matchmaker {
    pub fn new(elo_tolerance: u32) -> Self {
        Self { elo_tolerance }
    }

    pub fn find_match(&self, player_elo: u32, pool: &[u32]) -> Option<u32> {
        pool.iter()
            .find(|&&opponent_elo| opponent_elo.abs_diff(player_elo) <= self.elo_tolerance)
            .copied()
    }
}
