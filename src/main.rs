// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

mod genome;

fn main() {
    // Generate 23 chromosomes
    let seq1: Vec<f64> = genome::generate_sequence(23);
    let seq2: Vec<f64> = genome::generate_sequence(23);

    println!("Sequence1: {:?}", seq1);
    println!("Sequence2: {:?}", seq2);

    // println!("Guess the number!");

    // let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    // loop {
    //     println!("Please input your guess.");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You guessed: {guess}");

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }
}
