use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::Duration;

pub struct Cortex {
    running: Arc<AtomicBool>,
}

impl Cortex {
    pub fn new() -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        thread::spawn(move || {
            while running_clone.load(Ordering::Relaxed) {
                // Your background task logic goes here
                println!("Background thread is working...");

                // Sleep for 1 second to rate-limit the loop
                thread::sleep(Duration::from_secs(1));
            }
        });

        Cortex { running }
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
