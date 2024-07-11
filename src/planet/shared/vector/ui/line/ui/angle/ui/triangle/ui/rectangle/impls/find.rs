use std::rc::Rc;
use crate::{
    planet::shared::{
        traits::{Find, Has},
        vector::{
            ui::line::{
                ui::angle::ui::triangle::{ui::rectangle::Rectangle, Triangle},
                Line,
            },
            Number,
        },
    },
    traits::of_to::To,
};
use crate::planet::shared::vector::ui::line::LineType;

impl<T: Number, const N: usize> Find<Vec<Rectangle<T, N>>, LineType<T, N>>
    for Vec<Rectangle<T, N>>
{
    fn find(&self, line: &LineType<T, N>) -> Vec<Rectangle<T, N>> {
        self.clone()
            .into_iter()
            .filter(|rect| rect.has(line))
            .collect()
    }
}

impl<T: Number, const N: usize> Find<Vec<Rc<Triangle<T, N>>>, LineType<T, N>>
    for Vec<Rectangle<T, N>>
{
    fn find(&self, line: &LineType<T, N>) -> Vec<Rc<Triangle<T, N>>> {
        self.clone()
            .into_iter()
            .map(|rect| rect.to::<[Rc<Triangle<T, N>>; 2]>())
            .flatten()
            .filter(|triangle| triangle.has(line))
            .collect()
    }
}
