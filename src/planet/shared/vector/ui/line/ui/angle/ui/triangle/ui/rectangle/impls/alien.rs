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

impl<T: Number, const N: usize> Alien<Rc<Triangle<T, N>>> for Rectangle<T, N> {
    fn alien(&self, triangle: &Rc<Triangle<T, N>>) -> Rc<Triangle<T, N>> {
        self.clone().into_iter().find(|t| *t != *triangle).unwrap()
    }
}

impl<T: Number, const N: usize> Alien<Rc<Vector<T, N>>, Rc<Triangle<T, N>>> for Rectangle<T, N> {
    fn alien(&self, triangle: &Rc<Triangle<T, N>>) -> Rc<Vector<T, N>> {
        let common_line = self.get_common_line();
        SuperAlien::<Rc<Triangle<T, N>>>::alien::<Rc<Triangle<T, N>>>(self, &triangle)
            .to::<[Rc<Vector<T, N>>; 3]>()
            .into_iter()
            .find(|vector| !common_line.has(vector))
            .unwrap()
            .clone()
    }
}

impl<T: Number, const N: usize> Alien<[Rc<Vector<T, N>>; 2], Rc<Line<T, N>>> for Rectangle<T, N> {
    fn alien(&self, line: &Rc<Line<T, N>>) -> [Rc<Vector<T, N>>; 2] {
        ((*self).clone())
            .to::<[Rc<Vector<T, N>>; 4]>()
            .into_iter()
            .filter(|point| !line.clone().to::<[Rc<Vector<T, N>>; 2]>().contains(point))
            .collect::<Vec<Rc<Vector<T, N>>>>()
            .try_into()
            .unwrap()
    }
}
