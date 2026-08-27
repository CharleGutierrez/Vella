use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use vella::net::UdpTelemetryListener; // assuming vella is the crate name

fn main() {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9091";
    let mut listener = UdpTelemetryListener::new(addr);
    listener.listen_for_telemetry().unwrap();

    // Start a thread to send mock F1 telemetry data
    thread::spawn(move || {
        let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
        for i in 0..10 {
            let tire_temp = 90.0 + (i as f32) * 1.5;
            let rpm = 11000 + i * 200;
            let msg = format!("F1_TELEMETRY: TireTemp={:.1}C, RPM={}", tire_temp, rpm);
            sender.send_to(msg.as_bytes(), addr).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    println!("Listening for F1 Telemetry...");
    let mut count = 0;
    while count < 10 {
        if let Some(packet) = listener.receive() {
            if let Ok(msg) = String::from_utf8(packet) {
                println!("Received: {}", msg);
                count += 1;
            }
        } else {
            thread::sleep(Duration::from_millis(50));
        }
    }
    println!("F1 Telemetry test completed successfully.");
}
