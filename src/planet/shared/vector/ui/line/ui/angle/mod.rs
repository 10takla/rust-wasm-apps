#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use crate::planet::shared::point::DefaultMeasureValue;
use super::super::{Line, Vector};

#[derive(Debug, Copy, Clone)]
pub struct Angle {
    pub ab: Line,
    pub bc: Line,
}

impl Angle {
    pub fn get_normal(&self) -> Vector {
        let ab = self.ab.get_vector();
        let bc = self.bc.get_vector();
        // ((ab - bc) / 2.0).angle()
        (ab - bc) / 2.0
    }
    
    pub fn get_angle(&self) -> DefaultMeasureValue {
        let ab = self.ab.get_vector();
        let bc = self.bc.get_vector();
        (ab.scalar(&bc) / (ab.radius() * bc.radius()))
            .acos()
            .to_degrees()
    }
}