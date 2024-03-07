#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use crate::planet::shared::{point::DefaultMeasureValue, vector::Vector};

#[derive(Debug, Copy, Clone)]
pub struct Line<T = DefaultMeasureValue> {
    pub a: Vector<T>,
    pub b: Vector<T>,
}

impl<T: Copy> Line<T> {
    pub fn reverse(&self) -> Line<T> {
        Line::from([self.b, self.a])
    }
}