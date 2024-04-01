use std::rc::Rc;
use crate::planet::shared::{traits::Has, vector::{ui::line::{ui::angle::ui::triangle::{ui::rectangle::Rectangle, Triangle}, Line}, Number}};

impl<T: Number, const N: usize> Has<Rc<Triangle<T, N>>> for Rectangle<T, N> {
    fn has(&self, triangle: &Rc<Triangle<T, N>>) -> bool {
        self.clone().into_iter().any(|tri| tri == *triangle)
    }
}

impl<T: Number, const N: usize> Has<Rc<Line<T, N>>> for Rectangle<T, N> {
    fn has(&self, line: &Rc<Line<T, N>>) -> bool {
        self.clone().into_iter().any(|tri| tri.has(line))
    }
}