#[cfg(test)]
mod tests;

use crate::planet::shared::vector::VectorType;
use crate::{
    planet::shared::{
        traits::{Alien, Has, SuperAlien},
        vector::{
            ui::line::{
                ui::angle::ui::triangle::{ui::rectangle::Rectangle, Triangle},
                Line,
            },
            Number, Vector,
        },
    },
    traits::of_to::To,
};
use std::rc::Rc;
use crate::planet::shared::vector::ui::line::LineType;

impl<T: Number, const N: usize> Alien<Rc<Triangle<T, N>>> for Rectangle<T, N> {
    fn alien(&self, triangle: &Rc<Triangle<T, N>>) -> Rc<Triangle<T, N>> {
        self.clone().into_iter().find(|t| *t != *triangle).unwrap()
    }
}

impl<T: Number, const N: usize> Alien<VectorType<T, N>, Rc<Triangle<T, N>>> for Rectangle<T, N> {
    fn alien(&self, triangle: &Rc<Triangle<T, N>>) -> VectorType<T, N> {
        let common_line = self.get_common_line();
        SuperAlien::<Rc<Triangle<T, N>>>::alien::<Rc<Triangle<T, N>>>(self, &triangle)
            .to::<[VectorType<T, N>; 3]>()
            .into_iter()
            .find(|vector| !common_line.has(vector))
            .unwrap()
            .clone()
    }
}

impl<T: Number, const N: usize> Alien<[VectorType<T, N>; 2], LineType<T, N>> for Rectangle<T, N> {
    fn alien(&self, line: &LineType<T, N>) -> [VectorType<T, N>; 2] {
        ((*self).clone())
            .to::<[VectorType<T, N>; 4]>()
            .into_iter()
            .filter(|vector| {
                !line
                    .clone()
                    .to::<[VectorType<T, N>; 2]>()
                    .to_vec()
                    .has(vector)
            })
            .collect::<Vec<VectorType<T, N>>>()
            .try_into()
            .unwrap()
    }
}
