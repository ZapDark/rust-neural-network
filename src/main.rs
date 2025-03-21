use rand::Rng;
use rust_neural_network::types::WeightsVector;

pub struct Perceptron {
    weights: WeightsVector,
}

impl Perceptron {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut weights = WeightsVector::default();

        for i in 0..weights.len() {
            weights[i] = rng.gen_range(-1.0..1.0);
        }

        Perceptron { weights }
    }

    fn guess(&self, inputs: [f32; 2]) -> f32 {
        let mut sum = 0.0;

        for i in 0..self.weights.len() {
            sum += inputs[i] * self.weights[i];
        }

        return sum;
    }
}

fn main() {
    let perceptron = Perceptron::new();
    println!("{:?}", perceptron.weights);
}
