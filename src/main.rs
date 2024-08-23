use std::io::{self, Write};

mod daemon;
mod genome;
mod memory;
use daemon::Cortex;

fn main() {
    // 23 chromosomes
    let _seq1: Vec<f64> = genome::generate_sequence(23);
    let _seq2: Vec<f64> = genome::generate_sequence(23);

    let mut daemon = Cortex::new();

    loop {
        daemon.memory.recall();
        print!(" INPUT: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("OUTPUT: {input}");
        daemon.memory.push(input);
    }
}
