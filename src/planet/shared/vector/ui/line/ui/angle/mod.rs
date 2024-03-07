#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use crate::planet::shared::{point::DefaultMeasureValue, vector::Number};
use super::super::{Line, Vector};


#[derive(Debug, Copy, Clone)]
pub struct Angle<T = DefaultMeasureValue> {
    // линии расположны от центральной вершины к соседним
    pub ba: Line<T>,
    pub bc: Line<T>,
}

impl<T: Number> Angle<T> {
    pub fn angle_to_vector(angle: T) -> Vector {
        Vector::<f64>::from([angle.as_::<f64>().cos(), angle.as_::<f64>().sin()])
    }
    
    pub fn get_normal(&self) -> Vector<T> {
        let ba = Vector::from(self.ba);
        let bc = Vector::from(self.bc);
        (bc + ba) / T::from(2)
    }
    
    pub fn get_angle(&self) -> T {
        let ab = Vector::from(self.ba);
        let bc = Vector::from(self.bc);
        T::from(
            (ab.scalar(&bc) / (ab.radius() * bc.radius())).as_::<f64>()
            .acos()
            .to_degrees()
        )
    }
}