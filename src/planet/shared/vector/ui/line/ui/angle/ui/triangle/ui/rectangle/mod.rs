mod impls;
#[cfg(test)]
mod tests;
pub mod svg;

use super::super::Triangle;
use crate::planet::shared::traits::SuperAlien;
use crate::planet::shared::vector::Vector;
use crate::planet::shared::vector::VectorType;
use crate::traits::of_to::Of;
use crate::{
    planet::shared::{
        point::{DefaultMeasureValue, DEFAULT_MEASURE},
        vector::{ui::line::Line, Number},
    },
    traits::of_to::To,
};
use std::rc::Rc;
use crate::planet::shared::vector::ui::line::LineType;

#[derive(Clone)]
pub struct Rectangle<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> {
    pub a: Rc<Triangle<T, N>>,
    pub b: Rc<Triangle<T, N>>,
}

pub type RectangleType<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> = Rc<Rectangle<T, N>>;

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

    pub fn get_common_line(&self) -> LineType<T, N> {
        let [a_lines, b_lines]: [[LineType<T, N>; 3]; 2] =
            [self.a.clone().to(), self.b.clone().to()];
        a_lines
            .into_iter()
            .find(|line| b_lines.contains(line))
            .unwrap()
    }

    pub fn get_align_vecs(&self) -> [VectorType<T, N>; 2] {
        self.alien::<[VectorType<T, N>; 2]>(&self.get_common_line())
    }
}

impl<T: Number> Rectangle<T> {
    pub fn is_delone(&self) -> bool {
        let circle = self.a.get_circle();

        let vector = (*self).alien::<Rc<Vector<T>>>(&self.a);

        (*vector - *circle.center).radius() >= circle.radius()
    }

    pub fn set_delone(&mut self) {
        if !self.is_delone() {
            self.reverse_tries();
        }
    }
}
