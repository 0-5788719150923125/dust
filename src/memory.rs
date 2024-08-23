use rand::seq::SliceRandom;

pub(crate) struct MemoryCell {
    store: Vec<String>,
}

impl MemoryCell {
    pub fn new() -> MemoryCell {
        MemoryCell { store: Vec::new() }
    }

    pub fn push(&mut self, item: String) {
        self.store.push(item);
    }

    // pub fn recall(&self) {
    //     println!("memories: {:?}", self.store);
    // }

    pub fn get_focus(&self) -> &str {
        if self.store.is_empty() {
            "No memories yet"
        } else {
            self.store.choose(&mut rand::thread_rng()).unwrap()
        }
    }

    pub fn get_response(&self) -> &str {
        self.store.choose(&mut rand::thread_rng()).unwrap()
    }
}
