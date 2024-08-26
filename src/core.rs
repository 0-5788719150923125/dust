use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

use crate::memory::DataStore;

pub struct Simulator {
    running: Arc<AtomicBool>,
    pub memory: DataStore,
}

impl Simulator {
    pub fn new() -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let memory = DataStore::new();
        let cortex = Simulator { running, memory };
        cortex.spawn_background_thread();
        cortex
    }

    fn spawn_background_thread(&self) {
        let running = self.running.clone();
        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                // Background task logic
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

impl Drop for Simulator {
    fn drop(&mut self) {
        self.stop();
    }
}
