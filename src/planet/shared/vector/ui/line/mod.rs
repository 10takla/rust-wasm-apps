mod impls;
#[cfg(test)]
mod tests;
pub mod ui;

use crate::{planet::shared::{point::DefaultMeasureValue, vector::Vector}, traits::of_to::Of};

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Line<T = DefaultMeasureValue, const N: usize = 2> {
    pub a: Rc<Vector<T, N>>,
    pub b: Rc<Vector<T, N>>,
}

impl<T: Copy, const N: usize> Line<T, N> {
    pub fn reverse(&self) -> Line<T, N> {
        Line::of([&self.b, &self.a])
    }
}
