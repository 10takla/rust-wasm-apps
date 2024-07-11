mod impls;
pub mod svg;
#[cfg(test)]
mod tests;
pub mod ui;

use crate::planet::shared::vector::{Number, VectorType};
use crate::traits::of_to::To;
use crate::{
    planet::shared::{point::{DefaultMeasureValue, DEFAULT_MEASURE}, vector::Vector},
    traits::of_to::Of,
};
use macros::{extended_structure, Iterator};
use serde::Serialize;
use std::rc::Rc;

#[derive(Debug, Clone, Iterator)]
pub struct Line<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> {
    pub a: VectorType<T, N>,
    pub b: VectorType<T, N>,
}

pub type LineType<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> = Rc<Line<T, N>>;

impl<T: Number, const N: usize> Line<T, N> {
    pub fn reverse(&self) -> Line<T, N> {
        Line::of([&self.b, &self.a])
    }
}

impl<T: Number, const N: usize> Line<T, N> {
    pub fn length(&self) -> T {
        (*self.b - *self.a).radius()
    }


    pub fn map<F: Number, const P: usize>(
        &self,
        f: impl Fn(&Rc<Vector<T, N>>) -> VectorType<F, P>,
    ) -> LineType<F, P> {
        [&self.a, &self.b].map(f).to::<LineType<F, P>>()
    }
}
