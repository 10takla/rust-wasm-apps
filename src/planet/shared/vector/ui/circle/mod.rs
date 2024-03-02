use std::iter::Sum;

use crate::planet::shared::{point::DefaultMeasureValue, vector::{Number, Vector}};

#[cfg(test)]
mod tests;
mod impls;

#[derive(Debug)]
pub struct Circle<T = DefaultMeasureValue> {
    pub center: Vector<T>,
    pub point: Vector<T>,
}

impl<T: Number + From<f64> + Into<f64> + Sum + Default> Circle<T> {
    pub fn radius(&self) -> T {
        (self.point - self.center).radius()
    }
}