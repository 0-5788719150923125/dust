use rand::Rng;

pub fn generate_sequence(length: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..length).map(|_| rng.gen::<f64>()).collect()
}
