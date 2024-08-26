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

    pub fn get_table(&self) -> &Vec<String> {
        &self.table
    }

    pub fn get_random(&self) -> &str {
        if self.table.is_empty() {
            ".blank"
        } else {
            self.table.choose(&mut rand::thread_rng()).unwrap()
        }
    }
}
