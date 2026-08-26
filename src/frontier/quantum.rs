use num_complex::Complex;
use tracing::info;

/// Vella Quantum Computing State Vector Simulator
/// Physically computes Qubit superposition states using complex probability amplitudes.
pub struct QuantumEmulator {
    // A single qubit is represented by two complex probability amplitudes (alpha, beta)
    // where |alpha|^2 + |beta|^2 = 1.0
    state_vector: [Complex<f64>; 2],
}

impl QuantumEmulator {
    /// Initializes a qubit in the |0> state
    pub fn new() -> Self {
        Self {
            state_vector: [
                Complex::new(1.0, 0.0), // |0> amplitude
                Complex::new(0.0, 0.0), // |1> amplitude
            ]
        }
    }

    /// Applies the Hadamard Gate (H) to place the qubit into perfect superposition
    pub fn apply_hadamard(&mut self) {
        info!("⚛️ [Vella Quantum] Applying Hadamard Gate matrix multiplication...");
        let inv_sqrt2 = 1.0 / std::f64::consts::SQRT_2;
        
        let a = self.state_vector[0];
        let b = self.state_vector[1];
        
        // H = 1/sqrt(2) * [ 1  1 ]
        //                 [ 1 -1 ]
        self.state_vector[0] = (a + b) * inv_sqrt2;
        self.state_vector[1] = (a - b) * inv_sqrt2;
    }

    /// Applies the Pauli-X Gate (NOT) to flip the qubit state
    pub fn apply_pauli_x(&mut self) {
        info!("⚛️ [Vella Quantum] Applying Pauli-X (NOT) Gate...");
        let temp = self.state_vector[0];
        self.state_vector[0] = self.state_vector[1];
        self.state_vector[1] = temp;
    }

    /// Measures the probability of the qubit collapsing to the |1> state
    pub fn measure_probability_one(&self) -> f64 {
        let prob_one = self.state_vector[1].norm_sqr();
        info!("✨ [Vella Quantum] Computed physical collapse probability: {:.2}% chance of |1⟩", prob_one * 100.0);
        prob_one
    }
}
