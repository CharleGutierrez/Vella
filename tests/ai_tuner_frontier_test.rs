use vella::ai::tuner::AiTuner;
use vella::space::dtn::DtnQueue;
use vella::robotics::slam::SlamOffloader;
use vella::gaming::matchmaking::Matchmaker;

#[test]
fn test_ai_tuned_dtn_space_weather() {
    let tuner = AiTuner::new();
    let base_tolerance = 3600; // 1 hour
    
    // Case 1: Normal solar weather (Index 3.0)
    let mut dtn_normal = DtnQueue::new(base_tolerance);
    dtn_normal.optimize_with_ai(&tuner, 3.0);
    assert_eq!(dtn_normal.get_tolerance(), 3600, "Tolerance should remain 1 hour under normal weather");

    // Case 2: Extreme solar flare (Index 8.5)
    let mut dtn_storm = DtnQueue::new(base_tolerance);
    dtn_storm.optimize_with_ai(&tuner, 8.5);
    assert_eq!(
        dtn_storm.get_tolerance(),
        3600 + 14400, // base + 4 hours
        "Tolerance should expand by 4 hours during extreme solar weather"
    );
}

#[test]
fn test_ai_tuned_slam_downsampling() {
    let tuner = AiTuner::new();
    
    // Case 1: Hovering / Slow movement (5 m/s)
    let mut slam_slow = SlamOffloader::new();
    slam_slow.optimize_with_ai(&tuner, 5.0);
    slam_slow.ingest_lidar_scan("drone_1", 1000); // Should ingest 100% (1000 points)
    assert_eq!(slam_slow.compile_global_map(), 1000, "Should not downsample during slow movement");

    // Case 2: Extreme velocity / Transiting (25 m/s)
    let mut slam_fast = SlamOffloader::new();
    slam_fast.optimize_with_ai(&tuner, 25.0);
    slam_fast.ingest_lidar_scan("drone_1", 1000); // Should ingest 25% (250 points) due to 4x downsample
    assert_eq!(slam_fast.compile_global_map(), 250, "Should downsample 4x during extreme velocity");
}

#[test]
fn test_ai_tuned_matchmaking() {
    let tuner = AiTuner::new();
    let base_elo_tolerance = 50;
    
    // Player pool of 1600 ELO and 1530 ELO (Difference of 70)
    let pool = vec![1600, 1530];
    
    // Case 1: High server population (10,000 players online)
    let mut matchmaker_peak = Matchmaker::new(base_elo_tolerance);
    matchmaker_peak.optimize_with_ai(&tuner, 10_000);
    
    // Player with 1500 ELO tries to match. Nearest in pool is 1530 (diff 30), which is <= 50.
    // Wait, let's test a diff of 70. Player ELO = 1600, searching for someone. 1530 is 70 ELO away.
    let matched_peak = matchmaker_peak.find_match(1600, &pool);
    // Base tolerance is 50. 1600 - 1530 = 70. Should return None because 70 > 50.
    // However, 1600 - 1600 = 0. We need to filter out self or just provide a pool without exact matches.
    let opponent_pool = vec![1530]; 
    assert_eq!(matchmaker_peak.find_match(1600, &opponent_pool), None, "Should reject match due to strict base ELO tolerance");

    // Case 2: 3:00 AM Low server population (200 players online)
    let mut matchmaker_dead_hours = Matchmaker::new(base_elo_tolerance);
    matchmaker_dead_hours.optimize_with_ai(&tuner, 200);
    
    // AI expands tolerance from 50 to 100.
    // 1600 - 1530 = 70, which is now <= 100. Should match!
    assert_eq!(matchmaker_dead_hours.find_match(1600, &opponent_pool), Some(1530), "Should approve match due to AI-expanded ELO tolerance");
}
