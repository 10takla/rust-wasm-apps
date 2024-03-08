#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use crate::planet::shared::{point::DefaultMeasureValue, vector::Vector};

#[derive(Debug, Copy, Clone)]
pub struct Line<T = DefaultMeasureValue, const N: usize = 2> {
    pub a: Vector<T, N>,
    pub b: Vector<T, N>,
}

impl<T: Copy, const N: usize> Line<T, N> {
    pub fn reverse(&self) -> Line<T, N> {
        Line::from([self.b, self.a])
    }
}