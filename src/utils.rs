use macroquad::{
    color::{Color, GRAY},
    shapes::draw_line,
};

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH, SHIFT, SLOPE};

pub fn sign(n: f32) -> f32 {
    if n >= 0.0 {
        return 1.0;
    }
    return -1.0;
}

pub fn map_range(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min)
}

pub fn f(x: f32) -> f32 {
    return SLOPE * x + SHIFT;
}

pub fn draw_cartesian_grid(step: f32, grid_color: Color) {
    // Draw vertical grid lines
    for x in (0..SCREEN_WIDTH).step_by(step as usize) {
        draw_line(
            x as f32,
            0.0,
            x as f32,
            SCREEN_HEIGHT as f32,
            1.0,
            grid_color,
        );
    }

    // Draw horizontal grid lines
    for y in (0..SCREEN_HEIGHT).step_by(step as usize) {
        draw_line(
            0.0,
            y as f32,
            SCREEN_WIDTH as f32,
            y as f32,
            1.0,
            grid_color,
        );
    }

    // Draw the Cartesian axes
    draw_line(
        (SCREEN_WIDTH / 2) as f32,
        0.0,
        (SCREEN_WIDTH / 2) as f32,
        SCREEN_HEIGHT as f32,
        1.0,
        GRAY,
    ); // Y-axis
    draw_line(
        0.0,
        (SCREEN_HEIGHT / 2) as f32,
        SCREEN_WIDTH as f32,
        (SCREEN_HEIGHT / 2) as f32,
        1.0,
        GRAY,
    ); // X-axis
}
