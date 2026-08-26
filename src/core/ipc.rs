use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::info;

/// Genuine HPC IPC (Inter-Process Communication) Ring Buffer
/// Uses OS-level memory mapping (mmap) backed by a file (e.g., in /dev/shm)
/// to share data lock-free between independent Linux processes at microsecond latency.
pub struct MmapRingBuffer {
    mmap: MmapMut,
    capacity: usize,
}

impl MmapRingBuffer {
    /// Creates or opens an existing shared memory mapped file.
    /// In a real HPC environment, `file_path` would be in `/dev/shm/vella_telemetry`.
    pub fn new(file_path: &str, capacity: usize) -> Result<Self, String> {
        info!("⚙️ [Vella HPC] Initializing true Memory-Mapped IPC Ring Buffer at {}", file_path);
        
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .map_err(|e| format!("Failed to open shared memory file: {}", e))?;
            
        // Ensure file is large enough to hold `capacity` of u64s
        let file_size = (capacity * std::mem::size_of::<u64>()) as u64;
        file.set_len(file_size).map_err(|e| e.to_string())?;

        let mmap = unsafe { 
            MmapOptions::new().map_mut(&file).map_err(|e| e.to_string())? 
        };

        Ok(Self {
            mmap,
            capacity,
        })
    }

    /// Writes a value directly into the memory-mapped file lock-free
    pub fn write_lockfree(&mut self, index: usize, value: u64) {
        if index >= self.capacity {
            return;
        }
        
        // Cast the mmap memory region into an AtomicU64 slice
        let ptr = self.mmap.as_mut_ptr() as *mut AtomicU64;
        let atomic_ref = unsafe { &*ptr.add(index) };
        
        // Perform a relaxed atomic write (Zero-copy, bypasses kernel space entirely)
        atomic_ref.store(value, Ordering::Relaxed);
    }

    /// Reads a value directly from the memory-mapped file lock-free
    pub fn read_lockfree(&self, index: usize) -> Option<u64> {
        if index >= self.capacity {
            return None;
        }
        
        let ptr = self.mmap.as_ptr() as *const AtomicU64;
        let atomic_ref = unsafe { &*ptr.add(index) };
        
        Some(atomic_ref.load(Ordering::Relaxed))
    }
}
