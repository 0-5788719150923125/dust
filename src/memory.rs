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

    pub fn recall(&self) {
        println!("memories: {:?}", self.store);
    }
}
