use std::rc::Rc;

use crate::planet::shared::traits::Has;
use crate::planet::shared::vector::ui::line::LineType;
use crate::{
    planet::shared::{
        traits::Alien,
        vector::{
            ui::line::{ui::angle::ui::triangle::Triangle, Line},
            Number, VectorType,
        },
    },
    traits::of_to::To,
};

impl<T: Number, const N: usize> Alien<[LineType<T, N>; 2], LineType<T, N>> for Rc<Triangle<T, N>> {
    fn alien(&self, line: &LineType<T, N>) -> [LineType<T, N>; 2] {
        self.clone()
            .to::<[LineType<T, N>; 3]>()
            .into_iter()
            .filter(|line1| !(*line1).eq(line))
            .collect::<Vec<LineType<T, N>>>()
            .try_into()
            .unwrap()
    }
}

impl<T: Number, const N: usize> Alien<LineType<T, N>, VectorType<T, N>> for Rc<Triangle<T, N>> {
    fn alien(&self, vector: &VectorType<T, N>) -> LineType<T, N> {
        self.clone()
            .to::<[LineType<T, N>; 3]>()
            .into_iter()
            .find(|line1| !line1.has(&vector))
            .unwrap()
    }
}
