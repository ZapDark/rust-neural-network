use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Perceptron Visualization".to_owned(),
        window_width: 800,
        window_height: 600,
        window_resizable: false,
        ..Default::default()
    }
}

// Mock neural network data (replace with your actual implementation)
struct Perceptron {
    weights: [f32; 3],
    bias: f32,
    position: Vec2,
}

impl Perceptron {
    fn new() -> Self {
        Perceptron {
            weights: [0.5, -0.7, 0.3], // Example weights
            bias: 0.2,
            position: vec2(0.0, 0.0),
        }
    }

    // Add your activation logic here
    fn activate(&self, inputs: &[f32]) -> f32 {
        // Simple step function for demonstration
        let sum = inputs
            .iter()
            .zip(self.weights.iter())
            .map(|(i, w)| i * w)
            .sum::<f32>()
            + self.bias;

        if sum >= 0.0 { 1.0 } else { 0.0 }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let perceptron = Perceptron::new();

    loop {
        clear_background(BLACK);

        // Calculate dynamic positions based on screen size
        let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);

        // Input neurons positions (left third of the screen)
        let input_positions = [
            vec2(screen_width() * 0.25, screen_center.y - 100.0),
            vec2(screen_width() * 0.25, screen_center.y),
            vec2(screen_width() * 0.25, screen_center.y + 100.0),
        ];

        // Output neuron position (right third of the screen)
        let output_position = vec2(screen_width() * 0.75, screen_center.y);

        // Bias position (bottom center)
        let bias_position = vec2(screen_center.x, screen_height() - 100.0);

        // Draw connections
        for (i, &input_pos) in input_positions.iter().enumerate() {
            let weight = perceptron.weights[i];
            let color = if weight >= 0.0 { GREEN } else { RED };
            draw_line(
                input_pos.x,
                input_pos.y,
                output_position.x,
                output_position.y,
                3.0,
                color,
            );

            // Draw weight values
            let text_pos = input_pos.lerp(output_position, 0.5);
            draw_text(
                &format!("{:.2}", weight),
                text_pos.x - 20.0,
                text_pos.y + 5.0,
                20.0,
                WHITE,
            );
        }

        // Draw input neurons
        for (i, &pos) in input_positions.iter().enumerate() {
            draw_circle(pos.x, pos.y, 20.0, BLUE);
            draw_text(
                &format!("Input {}", i + 1),
                pos.x - 30.0,
                pos.y - 30.0,
                20.0,
                WHITE,
            );
        }

        // Draw output neuron
        let activation = perceptron.activate(&[1.0, 0.5, 0.7]); // Example inputs
        let output_color = if activation >= 0.5 { GREEN } else { RED };
        draw_circle(output_position.x, output_position.y, 25.0, output_color);
        draw_text(
            "Output",
            output_position.x - 40.0,
            output_position.y - 40.0,
            20.0,
            WHITE,
        );

        // Draw bias
        draw_circle(bias_position.x, bias_position.y, 15.0, YELLOW);
        draw_text(
            &format!("Bias: {:.2}", perceptron.bias),
            bias_position.x - 40.0,
            bias_position.y - 30.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}

fn draw_neuron(pos: Vec2, color: Color) {
    draw_circle(pos.x, pos.y, 20.0, color);
}
