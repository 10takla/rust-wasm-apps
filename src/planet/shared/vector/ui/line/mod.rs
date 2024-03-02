#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use crate::planet::shared::{point::DefaultMeasureValue, vector::{Number, Vector}};

#[derive(Debug, Copy, Clone)]
pub struct Line<'a, T = DefaultMeasureValue> {
    pub a: &'a Vector<T>,
    pub b: &'a Vector<T>,
}

impl<'a, T: Number> Line<'a, T> {
    pub fn get_vector(&self) -> Vector<T> {
        *self.b - *self.a
    }
}

