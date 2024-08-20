use std::io::{self, Write};

mod genome;

fn main() {
    // 23 chromosomes
    let _seq1: Vec<f64> = genome::generate_sequence(23);
    let _seq2: Vec<f64> = genome::generate_sequence(23);

    loop {
        print!(" INPUT: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("OUTPUT: {input}");
    }
}
