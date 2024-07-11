use std::rc::Rc;
use crate::planet::shared::{traits::Alien, vector::{ui::line::Line, Number, VectorType}};

impl<T: Number, const N: usize> Alien<VectorType<T, N>> for Line<T, N> {
    fn alien(&self, one_own: &VectorType<T, N>) -> VectorType<T, N> {
        self.clone().into_iter().find(|vector| {
            !Rc::ptr_eq(vector, one_own)
        }).unwrap()
    }
}