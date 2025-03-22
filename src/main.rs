use macroquad::{
    color::{BLACK, Color, GREEN, RED, WHITE},
    shapes::{draw_circle, draw_line},
    window::{Conf, clear_background, next_frame},
};
use rust_neural_network::{
    constants::{DATA_POINTS, SCREEN_HEIGHT, SCREEN_WIDTH},
    perceptron::Perceptron,
    point::Point,
    types::InputVector,
    utils::f,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Perceptron".to_owned(),
        window_width: SCREEN_WIDTH,
        window_height: SCREEN_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points: Vec<Point> = Vec::new();
    let mut perceptron: Perceptron = Perceptron::new();
    let training_index: usize = 0;
    let target_point_1: Point = Point::new_with_coordinates(-1.0, f(-1.0));
    let target_point_2: Point = Point::new_with_coordinates(1.0, f(1.0));

    for _ in 0..DATA_POINTS {
        points.push(Point::new());
    }

    loop {
        // Clear the screen with a color
        clear_background(WHITE);

        // Draw a line
        draw_line(
            target_point_1.pixel_x(),
            target_point_1.pixel_y(),
            target_point_2.pixel_x(),
            target_point_2.pixel_y(),
            1.0,
            BLACK,
        );

        //Draw dataset
        for point in &points {
            point.show();
        }

        for point in &points {
            let inputs: InputVector = [point.x, point.y];

            let guess: f32 = perceptron.guess(inputs);

            let mut color: Color = RED;

            if guess == point.target {
                color = GREEN;
            }

            draw_circle(point.pixel_x(), point.pixel_y(), 2.0, color);
        }

        let training: &Point = &points[training_index];
        let inputs: InputVector = [training.x, training.y];
        perceptron.train(inputs, training.target);

        // Wait for the next frame
        next_frame().await;
    }
}
