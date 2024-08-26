use rand::Rng;

pub fn generate_sequence(length: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen::<f64>()).collect()
}

// mod seed;

// 2 base pairs; a genome
// let _seq1: Vec<f64> = seed::generate_sequence(23);
// let _seq2: Vec<f64> = seed::generate_sequence(23);
