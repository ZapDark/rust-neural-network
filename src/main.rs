use rand::Rng;
use rust_neural_network::types::WeightsArray;

pub struct Perceptron {
    weights: WeightsArray,
}

impl Perceptron {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Perceptron {
            weights: [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)],
        }
    }
}

fn main() {
    let perceptron = Perceptron::new();
    println!("{:?}", perceptron.weights);
}
