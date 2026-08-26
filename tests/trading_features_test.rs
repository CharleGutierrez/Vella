use vella::trading::fix::FixEngine;
use vella::trading::backtest::BacktestSandbox;
use vella::trading::matching::MatchingEngine;
use vella::trading::fpga::FpgaCompiler;
use vella::ai::tuner::AiTuner;

#[tokio::test]
async fn test_trading_fix_protocol_execution() {
    let fix = FixEngine::new("NASDAQ", "VELLA_FUND");
    let order_id = fix.send_order("AAPL", 1000, 150.25).await.unwrap();
    assert!(order_id.contains("AAPL"));
}

#[test]
fn test_trading_backtest_sandbox() {
    let sandbox = BacktestSandbox::new("/data/historical/spy_2010_2020.csv");
    let report = sandbox.run_simulation("MeanReversion_V1").unwrap();
    assert!(report.contains("Sharpe Ratio"));
}

#[test]
fn test_trading_matching_engine() {
    let lob = MatchingEngine::new("BTC-USD");
    // Just verifying it runs without panicking
    lob.submit_order("BUY", 65000.0, 2);
}

#[test]
fn test_trading_fpga_compiler() {
    let verilog = FpgaCompiler::compile_to_verilog("HftArbBot").unwrap();
    assert!(verilog.contains("module HftArbBot"));
    assert!(verilog.contains("always @(posedge clk)"));
}

#[test]
fn test_ai_tuner_lob_and_fix_circuit_breaker() {
    let tuner = AiTuner::new();
    
    // Normal volume
    assert_eq!(tuner.tune_lob_matching_batch_size(500_000), 1);
    
    // Insane volume (batch mode)
    assert_eq!(tuner.tune_lob_matching_batch_size(2_000_000), 100);

    // Normal Market
    assert!(!tuner.predict_market_volatility_circuit_breaker(15.0, 0.01));

    // Flash Crash (VIX > 40)
    assert!(tuner.predict_market_volatility_circuit_breaker(45.0, 0.05));
}

#[test]
fn test_ai_tuner_advanced_web3() {
    let tuner = AiTuner::new();

    // FHE Tuning
    assert_eq!(tuner.tune_fhe_encryption_depth(50.0), 8192); // Normal
    assert_eq!(tuner.tune_fhe_encryption_depth(95.0), 4096); // CPU constrained

    // Oracle Slippage Tuning
    assert_eq!(tuner.tune_cross_chain_oracle_slippage(5_000_000.0), 0.5); // High liquidity
    assert_eq!(tuner.tune_cross_chain_oracle_slippage(500_000.0), 3.0); // Low liquidity
}
