use vella::trading::fix::FixClient;
use vella::trading::fpga::FpgaCompiler;

fn main() {
    println!("--- Testing FIX Protocol (Vella) ---");
    let mut client = FixClient::new("TARGET", "SENDER");
    
    // Test FIX message building
    let msg = client.build_message("D", "11=ORD_123\x0155=AAPL\x0154=1\x0138=100\x0140=2\x0144=150.50\x01");
    println!("Generated FIX Message: {}", msg.replace('\x01', "|"));

    // Test FIX message parsing & checksum validation
    match FixClient::parse_message(&msg) {
        Ok(parsed) => {
            println!("Parsed Message:");
            println!("  MsgType: {}", parsed.msg_type);
            println!("  Valid Checksum: {}", parsed.valid_checksum);
            assert!(parsed.valid_checksum, "Checksum validation failed! The parsed checksum must be valid.");
        }
        Err(e) => {
            println!("Failed to parse message: {}", e);
            panic!("Message parsing should not fail");
        }
    }

    println!("\n--- Testing FPGA Compiler (Vella) ---");
    // Test FPGA Verilog generation
    match FpgaCompiler::compile_to_verilog("TestStrategy") {
        Ok(verilog) => {
            println!("Generated Verilog Snippet:");
            let snippet = verilog.lines().take(25).collect::<Vec<&str>>().join("\n");
            println!("{}\n...", snippet);
            
            assert!(verilog.contains("volatility"), "Verilog should contain volatility logic from the new TradeSignal logic.");
            assert!(verilog.contains("threshold"), "Verilog should contain threshold logic.");
        }
        Err(e) => {
            println!("FPGA Compilation failed: {}", e);
            panic!("FPGA Compilation should not fail");
        }
    }
}
