use std::collections::VecDeque;

/// Delay Tolerant Networking (DTN) queue for deep space comms
pub struct DtnQueue {
    bundles: VecDeque<Vec<u8>>,
    max_latency_tolerance_secs: u64,
}

impl DtnQueue {
    pub fn new(max_latency_tolerance_secs: u64) -> Self {
        Self {
            bundles: VecDeque::new(),
            max_latency_tolerance_secs,
        }
    }

    pub fn enqueue_bundle(&mut self, payload: Vec<u8>) {
        self.bundles.push_back(payload);
    }

    pub fn transmit_when_ready(&mut self) -> Option<Vec<u8>> {
        // Logic for transmission during orbital windows
        self.bundles.pop_front()
    }
}
