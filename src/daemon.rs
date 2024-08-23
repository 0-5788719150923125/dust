use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use crate::memory::MemoryCell;

pub struct Cortex {
    running: Arc<AtomicBool>,
    pub memory: MemoryCell,
}

impl Cortex {
    pub fn new() -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        thread::spawn(move || {
            while running_clone.load(Ordering::Relaxed) {
                // Your background task logic goes here
                // println!("Background thread is working...");

                // Sleep for 1 second to rate-limit the loop
                thread::sleep(Duration::from_secs(1));
            }
        });

        let memory = MemoryCell::new();

        Cortex { running, memory }
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

impl Drop for Cortex {
    fn drop(&mut self) {
        self.stop();
    }
}
