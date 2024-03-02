#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use crate::planet::shared::{point::DefaultMeasureValue, vector::Number};
use super::super::{Line, Vector};


#[derive(Debug, Copy, Clone)]
pub struct Angle<'a, T = DefaultMeasureValue> {
    // линии расположны от центральной вершины к соседним
    pub ba: Line<'a, T>,
    pub bc: Line<'a, T>,
}

impl<'a, T: Number> Angle<'a, T> {
    pub fn angle_to_vector(angle: T) -> Vector<T> {
        let v = Vector::<f64>::from([angle.into().cos(), angle.into().sin()]);
        (&v).into()
    }
    
    pub fn get_normal(&self) -> Vector<T> {
        let ba = self.ba.get_vector();
        let bc = self.bc.get_vector();
        (bc + ba) / T::from(2).unwrap()
    }
    
    pub fn get_angle(&self) -> T {
        let ab = self.ba.get_vector();
        let bc = self.bc.get_vector();
        T::from(
            (ab.scalar(&bc) / (ab.radius() * bc.radius())).into()
            .acos()
            .to_degrees()
        ).unwrap()
    }
}