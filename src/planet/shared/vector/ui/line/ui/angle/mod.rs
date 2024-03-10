#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use std::rc::Rc;

use crate::{planet::shared::{point::DefaultMeasureValue, vector::Number}, traits::as_::As};
use super::super::{Line, Vector};
use crate::traits::of_to::{Of, To};

#[derive(Debug, Clone)]
pub struct Angle<T = DefaultMeasureValue, const N: usize = 2> {
    // линии расположны от центральной вершины к соседним
    pub ba: Rc<Line<T, N>>,
    pub bc: Rc<Line<T, N>>,
}

impl<T: Number, const N: usize> Angle<T, N> {
    pub fn angle_to_vector(angle: T) -> Vector<f64, 2> {
        [angle.as_::<f64>().cos(), angle.as_::<f64>().sin()].to()
    }
    
    pub fn get_normal(&self) -> Vector<T, N> {
        let ba = Vector::of(self.ba.clone());
        let bc = Vector::of(self.bc.clone());
        (bc + ba) / 2.as_::<T>()
    }
    
    pub fn get_angle(&self) -> T {
        let ab = Vector::of(self.ba.clone());
        let bc = Vector::of(self.bc.clone());
        (ab.scalar(&bc) / (ab.radius() * bc.radius())).as_::<f64>()
            .acos()
            .to_degrees().as_()
    }
}