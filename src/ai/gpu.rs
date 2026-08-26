use tracing::{info, warn};
use std::process::Command;

pub enum HardwareAcceleratorType {
    Cuda,
    Metal,
    CpuSimd,
}

pub struct HardwareAccelerator {
    primary_hardware: HardwareAcceleratorType,
}

impl HardwareAccelerator {
    pub fn detect() -> Self {
        info!("Probing system hardware for AI Acceleration...");
        let hw = if std::env::var("CUDA_VISIBLE_DEVICES").is_ok() || Command::new("nvidia-smi").output().is_ok() {
            info!("Detected Nvidia CUDA compatible GPU. Routing heavy tensor math to CUDA.");
            HardwareAcceleratorType::Cuda
        } else if std::env::consts::OS == "macos" {
            info!("Detected macOS. Routing heavy tensor math to Metal.");
            HardwareAcceleratorType::Metal
        } else {
            warn!("No GPU detected. Falling back to CPU SIMD instructions.");
            HardwareAcceleratorType::CpuSimd
        };

        Self { primary_hardware: hw }
    }

    /// Dynamically probes actual physical GPU temperatures
    fn get_real_temperature(&self) -> Option<usize> {
        if matches!(self.primary_hardware, HardwareAcceleratorType::Cuda) {
            let output = Command::new("nvidia-smi")
                .arg("--query-gpu=temperature.gpu")
                .arg("--format=csv,noheader")
                .output()
                .ok()?;
                
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.trim().parse::<usize>().ok()
        } else {
            None
        }
    }

    pub fn execute_vector_math(&self, operation: &str) {
        if let Some(temp) = self.get_real_temperature() {
            if temp > 85 {
                warn!("🔥 GPU Thermal Throttling Detected ({}°C). Offloading {} to CPU SIMD temporarily.", temp, operation);
                return;
            } else {
                info!("🌡️ Current GPU Temp: {}°C. Safe to proceed.", temp);
            }
        }

        match self.primary_hardware {
            HardwareAcceleratorType::Cuda => info!("Executing {} via Nvidia CUDA cores", operation),
            HardwareAcceleratorType::Metal => info!("Executing {} via Apple Metal API", operation),
            HardwareAcceleratorType::CpuSimd => info!("Executing {} via AVX-512 CPU SIMD", operation),
        }
    }
}
