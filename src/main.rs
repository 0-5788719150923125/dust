use std::io::{self, Write};

mod ctx;
mod genome;
mod memory;
use ctx::Cortex;
use memory::MemoryCell;

fn main() {
    // 23 chromosomes
    let _seq1: Vec<f64> = genome::generate_sequence(23);
    let _seq2: Vec<f64> = genome::generate_sequence(23);

    let _worker = Cortex::new();

    let mut local_memory = MemoryCell::new();

    loop {
        local_memory.recall();
        print!(" INPUT: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("OUTPUT: {input}");
        local_memory.push(input);
    }
}
