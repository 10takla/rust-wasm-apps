mod impls;
#[cfg(test)]
mod tests;

use super::super::Triangle;
use crate::planet::shared::traits::SuperAlien;
use crate::planet::shared::vector::Vector;

use crate::traits::of_to::Of;
use crate::{
    planet::shared::{
        point::DefaultMeasureValue,
        vector::{ui::line::Line, Number},
    },
    traits::of_to::To,
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Rectangle<T = DefaultMeasureValue, const N: usize = 2> {
    pub a: Rc<Triangle<T, N>>,
    pub b: Rc<Triangle<T, N>>,
}

impl<T: Number, const N: usize> Rectangle<T, N> {
    pub fn reverse_tries(&mut self) {
        let common_line = self.get_common_line();
        let align_vecs = self.get_align_vecs();
        *self = Rectangle::of([
            &common_line.a,
            &align_vecs[0],
            &align_vecs[1],
            &common_line.b,
        ]);
    }

    pub fn get_common_line(&self) -> Rc<Line<T, N>> {
        let [a_lines, b_lines]: [[Rc<Line<T, N>>; 3]; 2] =
            [self.a.clone().to(), self.b.clone().to()];
        a_lines
            .into_iter()
            .find(|line| b_lines.contains(line))
            .unwrap()
    }

    pub fn get_align_vecs(&self) -> [Rc<Vector<T, N>>; 2] {
        self.alien::<[Rc<Vector<T, N>>; 2]>(&self.get_common_line())
    }
}

impl<T: Number> Rectangle<T> {
    pub fn set_delone(&mut self) {
        let circle = self.a.get_circle();
        let vector = (*self).alien::<Rc<Vector<T>>>(&self.a);

        if (*vector - *circle.center).radius() < circle.radius() {
            self.reverse_tries();
        }
    }
}
