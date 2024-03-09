#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use std::rc::Rc;

use crate::planet::shared::{point::DefaultMeasureValue, vector::Vector};

#[derive(Debug, Clone)]
pub struct Line<T = DefaultMeasureValue, const N: usize = 2> {
    pub a: Rc<Vector<T, N>>,
    pub b: Rc<Vector<T, N>>,
}

impl<T: Copy, const N: usize> Line<T, N> {
    pub fn reverse(&self) -> Line<T, N> {
        Line::from([self.b.clone(), self.a.clone()])
    }
}