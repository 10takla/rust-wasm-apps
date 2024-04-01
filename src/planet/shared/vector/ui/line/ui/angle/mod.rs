mod impls;
#[cfg(test)]
mod tests;
pub mod ui;

use std::rc::Rc;

use super::super::{Line, Vector};
use crate::traits::of_to::{Of, To};
use crate::{
    planet::shared::{point::DefaultMeasureValue, vector::Number},
    traits::as_prim::AsPrim,
};

#[derive(Clone)]
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
        let ba = Vector::of(&self.ba);
        let bc = Vector::of(&self.bc);
        (bc + ba) / 2.as_::<T>()
    }

    pub fn get_angle(&self) -> T {
        let ba = Vector::of(&self.ba);
        let bc = Vector::of(&self.bc);
        (ba.scalar(&bc) / (ba.radius() * bc.radius()))
            .as_::<f64>()
            .acos()
            .to_degrees()
            .as_()
    }

    pub fn get_polar_angle(&self) -> T {
        let ba = Vector::of(&self.ba);
        let bc = Vector::of(&self.bc);
        Self::normalize_angle((bc.atan() - ba.atan()).to_degrees()).as_()
    }

    fn normalize_angle(angle: f64) -> f64 {
        let mut normalized_angle = angle % 360.0;
        if normalized_angle > 180.0 {
            normalized_angle -= 360.0;
        } else if normalized_angle <= -180.0 {
            normalized_angle += 360.0;
        }
        normalized_angle
    }
}
