use macroquad::{
    color::{BLACK, Color, GRAY},
    shapes::draw_circle,
};
use rand::Rng;

use crate::{
    constants::{SCREEN_HEIGHT, SCREEN_WIDTH},
    utils::{f, map_range},
};

pub struct Point {
    pub x: f32,
    pub y: f32,
    pub bias: f32,
    pub target: f32,
}

impl Point {
    pub fn new() -> Self {
        let mut rng = rand::rng();

        let x: f32 = rng.random_range(-1.0..1 as f32);
        let y: f32 = rng.random_range(-1.0..1 as f32);

        let line_y: f32 = f(x);

        let target: f32 = if y > line_y { 1.0 } else { -1.0 };

        Point {
            x,
            y,
            target,
            bias: 1.0,
        }
    }

    pub fn new_with_coordinates(x: f32, y: f32) -> Self {
        let line_y: f32 = f(x);

        let target: f32 = if y > line_y { 1.0 } else { -1.0 };

        Point {
            x,
            y,
            target,
            bias: 1.0,
        }
    }

    pub fn pixel_x(&self) -> f32 {
        return map_range(self.x, -1.0, 1.0, 0.0, SCREEN_WIDTH as f32);
    }

    pub fn pixel_y(&self) -> f32 {
        return map_range(self.y, -1.0, 1.0, SCREEN_HEIGHT as f32, 0.0);
    }

    pub fn show(&self) {
        let mut color: Color = GRAY;
        if self.target == 1.0 {
            color = BLACK;
        }

        draw_circle(self.pixel_x(), self.pixel_y(), 4.0, color);
    }
}
