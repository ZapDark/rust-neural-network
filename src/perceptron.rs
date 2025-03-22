use rand::Rng;

use crate::{
    types::{InputVector, WeightsVector},
    utils::sign,
};

pub struct Perceptron {
    weights: WeightsVector,
    learning_rate: f32,
}

impl Perceptron {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut weights = WeightsVector::default();

        for i in 0..weights.len() {
            weights[i] = rng.random_range(-1.0..1.0);
        }

        Perceptron {
            weights,
            learning_rate: 0.1,
        }
    }

    pub fn guess(&self, inputs: InputVector) -> f32 {
        let mut sum = 0.0;

        for i in 0..self.weights.len() {
            sum += inputs[i] * self.weights[i];
        }

        return sign(sum);
    }

    pub fn train(&mut self, inputs: InputVector, target: f32) {
        let guess: f32 = self.guess(inputs);

        let error: f32 = target - guess;

        for i in 0..self.weights.len() {
            self.weights[i] += error * inputs[i];
        }
    }
}
