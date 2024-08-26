use rand::seq::SliceRandom;

pub(crate) struct DataStore {
    table: Vec<String>,
}

impl DataStore {
    pub fn new() -> DataStore {
        DataStore { table: Vec::new() }
    }

    pub fn push(&mut self, item: String) {
        self.table.push(item);
    }

    pub fn get_focus(&self) -> &str {
        if self.table.is_empty() {
            ".blank"
        } else {
            self.table.choose(&mut rand::thread_rng()).unwrap()
        }
    }

    pub fn get_response(&self) -> &str {
        self.table.choose(&mut rand::thread_rng()).unwrap()
    }
}
