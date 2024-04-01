use std::rc::Rc;

use crate::{planet::shared::{
    traits::Alien,
    vector::{ui::line::{ui::angle::ui::triangle::Triangle, Line}, Number},
}, traits::of_to::To};

impl<T: Number, const N: usize> Alien<[Rc<Line<T, N>>; 2], Rc<Line<T, N>>> for Rc<Triangle<T, N>> {
    fn alien(&self, line: &Rc<Line<T, N>>) -> [Rc<Line<T, N>>; 2] {
        self.clone()
            .to::<[Rc<Line<T, N>>; 3]>()
            .into_iter()
            .filter(|line1| !(*line1).eq(line))
            .collect::<Vec<Rc<Line<T, N>>>>()
            .try_into()
            .unwrap()
    }
}