mod impls;

use crate::planet::shared::vector::VectorType;
use crate::planet::shared::{point::{DefaultMeasureValue, DEFAULT_MEASURE}, vector::Number};
use std::rc::Rc;

#[derive(Debug)]
pub struct Circle<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> {
    pub center: VectorType<T, N>,
    pub point: VectorType<T, N>,
}

impl<T: Number, const N: usize> Circle<T, N> {
    pub fn radius(&self) -> T {
        (*self.point - *self.center).radius()
    }
}
