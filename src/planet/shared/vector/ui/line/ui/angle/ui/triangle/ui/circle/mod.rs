use crate::planet::shared::{point::DefaultMeasureValue, vector::Vector};

#[cfg(test)]
mod tests;
mod impls;

#[derive(Debug)]
pub struct Circle {
    pub center: Vector,
    pub point: Vector,
}

impl Circle {
    pub fn radius(&self) -> DefaultMeasureValue {
        (self.point - self.center).radius()
    }
}