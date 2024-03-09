#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use crate::planet::shared::{point::DefaultMeasureValue, vector::Number};
use super::super::{Line, Vector};


#[derive(Debug, Clone)]
pub struct Angle<T = DefaultMeasureValue, const N: usize = 2> {
    // линии расположны от центральной вершины к соседним
    pub ba: Line<T, N>,
    pub bc: Line<T, N>,
}

impl<T: Number, const N: usize> Angle<T, N> {
    pub fn angle_to_vector(angle: T) -> Vector<f64, 2> {
        [angle.as_::<f64>().cos(), angle.as_::<f64>().sin()].into()
    }
    
    pub fn get_normal(&self) -> Vector<T, N> {
        let ba = Vector::from(self.ba.clone());
        let bc = Vector::from(self.bc.clone());
        (bc + ba) / T::from(2)
    }
    
    pub fn get_angle(&self) -> T {
        let ab = Vector::from(self.ba.clone());
        let bc = Vector::from(self.bc.clone());
        T::from(
            (ab.scalar(&bc) / (ab.radius() * bc.radius())).as_::<f64>()
            .acos()
            .to_degrees()
        )
    }
}