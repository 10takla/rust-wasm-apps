mod impls;

use std::{ops::Deref, rc::Rc};
use crate::planet::shared::{point::DefaultMeasureValue, vector::{Number, Vector}};

#[derive(Debug)]
pub struct Circle<T = DefaultMeasureValue, const N: usize = 2> {
    pub center: Rc<Vector<T, N>>,
    pub point: Rc<Vector<T, N>>,
}

impl<T: Number, const N: usize> Circle<T, N> {
    pub fn radius(&self) -> T {
        (*self.point- *self.center).radius()
    }
}