mod impls;

use serde::{Deserialize, Serialize};

pub type DefaultMeasureValue = f64;
pub const DEFAULT_NUMBER_OF_MEASUREMENTS: usize = 3;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Point(pub [f64; 2]);
