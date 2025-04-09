use crate::constants::{LAYERS, NEURONS_PER_LAYER};

pub type WeightsVector = [f32; LAYERS * NEURONS_PER_LAYER];

pub type InputVector = [f32; 3];
