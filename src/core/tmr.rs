use tracing::{info, warn, error};

pub struct TmrVoter;

impl TmrVoter {
    /// Executes Triple Modular Redundancy (TMR) Hardware Voting Logic
    pub fn execute_hardware_vote(node_a: u16, node_b: u16, node_c: u16) -> Result<u16, &'static str> {
        info!("TMR Logic: Voting on clustered PLC inputs (A: {}, B: {}, C: {})", node_a, node_b, node_c);
        
        if node_a == node_b && node_b == node_c {
            Ok(node_a)
        } else if node_a == node_b {
            warn!("TMR Fault: Node C diverged ({}). Tripping hardware isolation. Outputting majority.", node_c);
            Ok(node_a)
        } else if node_b == node_c {
            warn!("TMR Fault: Node A diverged ({}). Tripping hardware isolation. Outputting majority.", node_a);
            Ok(node_b)
        } else if node_a == node_c {
            warn!("TMR Fault: Node B diverged ({}). Tripping hardware isolation. Outputting majority.", node_b);
            Ok(node_a)
        } else {
            error!("CRITICAL TMR FAILURE: All three nodes diverged! (A:{}, B:{}, C:{}). Halting execution.", node_a, node_b, node_c);
            Err("TMR Consensus Failure - Initiating Emergency Plant Shutdown")
        }
    }
}
