use vella::trading::fix::FixClient;

#[test]
fn test_fix_message_checksum_and_format() {
    let mut client = FixClient::new("EXCHANGE", "VELLA_TRADER");
    
    // Build a mock Logon message
    let msg = client.build_message("A", "98=0\x01108=30\x01");
    
    // Verify standard FIX prefix and BodyLength
    assert!(msg.starts_with("8=FIX.4.4\x019="));
    
    // Verify it includes the standard SOH characters (converted to verify correctly)
    assert!(msg.contains("35=A\x01"));
    assert!(msg.contains("49=VELLA_TRADER\x01"));
    assert!(msg.contains("56=EXCHANGE\x01"));
    
    // Verify it ends with the checksum field and SOH
    assert!(msg.ends_with("\x01"));
    let parts: Vec<&str> = msg.split("\x01").collect();
    let checksum_part = parts[parts.len() - 2];
    assert!(checksum_part.starts_with("10="));
    assert_eq!(checksum_part.len(), 6); // "10=XXX"
}

#[tokio::test]
async fn test_fix_disconnected_error() {
    let mut client = FixClient::new("EXCHANGE", "VELLA_TRADER");
    
    // Try sending an order without connecting
    let result = client.send_order("AAPL", 100, 150.50).await;
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Not connected to exchange");
}
