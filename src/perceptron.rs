use rand::Rng;

use crate::{constants::LEARNING_RATE, types::InputVector, utils::sign};

pub struct Perceptron {
    pub weights: Vec<f32>,
    learning_rate: f32,
}

impl Perceptron {
    pub fn new(n: usize) -> Self {
        let mut rng = rand::rng();
        let mut weights = vec![0.0; n];

        for i in 0..n {
            weights[i] = rng.random_range(-1.0..1.0);
            println!("Initial Weights: {:?}", weights[i]);
        }

        Perceptron {
            weights,
            learning_rate: LEARNING_RATE,
        }
    }

    pub fn guess(&self, inputs: InputVector) -> f32 {
        let mut sum: f32 = 0.0;

        for i in 0..self.weights.len() {
            sum += inputs[i] * self.weights[i];
        }

        let output: f32 = sign(sum);
        return output;
    }

    pub fn guess_y(&self, x: f32) -> f32 {
        let w0: f32 = self.weights[0];
        let w1: f32 = self.weights[1];
        let w2: f32 = self.weights[2];

        return -(w2 / w1) - (w0 / w1) * x;
    }

    pub fn train(&mut self, inputs: InputVector, target: f32) {
        let guess: f32 = self.guess(inputs);

        let error: f32 = target - guess;

        for i in 0..self.weights.len() {
            self.weights[i] += error * inputs[i] * self.learning_rate;
        }
    }
}
