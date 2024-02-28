#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use crate::planet::shared::{point::{DefaultMeasureValue, Point}, vector::{Number, Vector}};

#[derive(Debug, Copy, Clone)]
pub struct Line<T = DefaultMeasureValue> {
    pub a: Vector<T>,
    pub b: Vector<T>,
}

impl<T: Number> Line<T> {
    pub fn get_vector(&self) -> Vector<T> {
        self.b - self.a
    }
}

