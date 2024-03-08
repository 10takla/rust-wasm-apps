mod impls;

use crate::planet::shared::{point::DefaultMeasureValue, vector::{Number, Vector}};

#[derive(Debug)]
pub struct Circle<T = DefaultMeasureValue, const N: usize = 2> {
    pub center: Vector<T, N>,
    pub point: Vector<T, N>,
}

impl<T: Number, const N: usize> Circle<T, N> {
    pub fn radius(&self) -> T {
        (self.point - self.center).radius()
    }
}