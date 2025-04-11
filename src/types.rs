use crate::constants::{LAYERS, NEURONS_PER_LAYER, NUMBER_OF_WEIGHTS};

pub type WeightsVector = [f32; LAYERS * NEURONS_PER_LAYER];

pub type InputVector = [f32; NUMBER_OF_WEIGHTS];
