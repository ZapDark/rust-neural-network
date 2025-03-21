extern crate rand;

use macroquad::prelude::*;
use rand::Rng;

struct Perceptron {
    weights: [f32; 2],
    learning_rate: f32,
}

//test

impl Perceptron {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Perceptron {
            weights: [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)],
            learning_rate: 0.05,
        }
    }

    fn predict(&self, x: f32) -> f32 {
        self.weights[0] * x + self.weights[1]
    }

    fn train(&mut self, dataset: &[(f32, f32)]) -> f32 {
        let mut gradient_w0 = 0.0;
        let mut gradient_w1 = 0.0;
        let mut total_error = 0.0;

        for &(x, y) in dataset {
            let prediction = self.predict(x);
            let error = prediction - y;

            gradient_w0 += error;
            gradient_w1 += error * x;
            total_error += error.powi(2);
        }

        let n = dataset.len() as f32;
        self.weights[0] -= self.learning_rate * gradient_w1 / n;
        self.weights[1] -= self.learning_rate * gradient_w0 / n;

        total_error
    }
}

fn generate_dataset() -> Vec<(f32, f32)> {
    let mut rng = rand::thread_rng();
    let m = 0.7;
    let b = 0.2;
    let mut points = Vec::new();

    for _ in 0..100 {
        let x = rng.gen_range(-1.0..1.0);
        let y = m * x + b + rng.gen_range(-0.1..0.1);
        points.push((x, y));
    }

    points
}

fn to_screen_coords(x: f32, y: f32) -> (f32, f32) {
    let screen_width = screen_width();
    let screen_height = screen_height();
    (
        (x + 1.0) / 2.0 * screen_width,
        screen_height - ((y + 1.0) / 2.0 * screen_height),
    )
}

#[macroquad::main("Linear Regression with Perceptron")]
async fn main() {
    let dataset = generate_dataset();
    let mut perceptron = Perceptron::new();
    let mut training = true;
    let true_m = 0.7;
    let true_b = 0.2;

    loop {
        clear_background(WHITE);

        // Draw dataset points
        for &(x, y) in &dataset {
            let (screen_x, screen_y) = to_screen_coords(x, y);
            draw_circle(screen_x, screen_y, 3.0, BLUE);
        }

        // Draw true line
        let (true_x1, true_y1) = to_screen_coords(-1.0, true_m * -1.0 + true_b);
        let (true_x2, true_y2) = to_screen_coords(1.0, true_m * 1.0 + true_b);
        draw_line(true_x1, true_y1, true_x2, true_y2, 2.0, GREEN);

        // Draw perceptron line
        let (perc_x1, perc_y1) = to_screen_coords(-1.0, perceptron.predict(-1.0));
        let (perc_x2, perc_y2) = to_screen_coords(1.0, perceptron.predict(1.0));
        draw_line(perc_x1, perc_y1, perc_x2, perc_y2, 2.0, RED);

        // Training logic
        let mut total_error = 0.0;
        if training {
            let previous_weights = perceptron.weights;
            total_error = perceptron.train(&dataset);

            // Check for convergence
            let dw0 = (perceptron.weights[0] - previous_weights[0]).abs();
            let dw1 = (perceptron.weights[1] - previous_weights[1]).abs();

            if dw0 < 1e-5 && dw1 < 1e-5 {
                training = false;
                println!("Converged!");
            }
        }

        // Display total error on the screen
        draw_text(
            &format!("Total Error: {:.4}", total_error),
            10.0,
            20.0,
            20.0,
            BLACK,
        );

        next_frame().await;
    }
}
