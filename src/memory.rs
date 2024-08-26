use rand::seq::SliceRandom;

pub(crate) struct DataStore {
    store: Vec<String>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore { store: Vec::new() }
    }

    pub fn push(&mut self, item: String) {
        self.store.push(item);
    }

    pub fn get_focus(&self) -> &str {
        if self.store.is_empty() {
            ".blank"
        } else {
            self.store.choose(&mut rand::thread_rng()).unwrap()
        }
    }

    pub fn get_response(&self) -> &str {
        self.store.choose(&mut rand::thread_rng()).unwrap()
    }
}
