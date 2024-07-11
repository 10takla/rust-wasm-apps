use crate::{
    planet::shared::{
        traits::Has,
        vector::{
            ui::line::{
                ui::angle::{ui::triangle::Triangle, Angle},
                Line,
            },
            Number,
        },
    },
    traits::of_to::To,
};
use std::rc::Rc;
use crate::planet::shared::vector::ui::line::LineType;

impl<T: Number, const N: usize> Has<Rc<Angle<T, N>>> for Triangle<T, N> {
    fn has(&self, angle: &Rc<Angle<T, N>>) -> bool {
        self.clone().into_iter().any(|angle1| angle1 == *angle)
    }
}

impl<T: Number, const N: usize> Has<LineType<T, N>> for Triangle<T, N> {
    fn has(&self, n_line: &LineType<T, N>) -> bool {
        self.clone()
            .to::<[Line<T, N>; 3]>()
            .into_iter()
            .any(|line| line == *n_line.clone())
    }
}

impl<T: Number, const N: usize> Has<LineType<T, N>> for Rc<Triangle<T, N>> {
    fn has(&self, n_line: &LineType<T, N>) -> bool {
        self.as_ref().has(n_line)
    }
}
