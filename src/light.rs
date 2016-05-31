use std::vec::Vec;
use types::*;

pub trait Light: Sync {
    fn get_sample(&self) -> Color;
    fn get_samples(&self) -> Vec<Color>;
}
