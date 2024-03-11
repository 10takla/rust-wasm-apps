mod impls;
#[cfg(test)]
mod tests;

use super::super::Triangle;
use crate::planet::shared::vector::Vector;
use crate::traits::of_to::Of;
use crate::{
    planet::shared::{
        point::{DefaultMeasureValue, Point},
        vector::{ui::line::Line, Number},
    },
    traits::of_to::To,
};
use std::hash::Hash;
use std::{fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub struct Rectangle<T = DefaultMeasureValue, const N: usize = 2> {
    a: Rc<Triangle<T, N>>,
    b: Rc<Triangle<T, N>>,
}

impl<T: PartialEq + Eq + Ord + Number + Hash, const N: usize> Rectangle<T, N> {
    pub fn reverse_tries(&self) -> Rectangle<T, N> {
        let common_line = self.get_common_line();
        let align_vecs = self.get_align_vecs();
        Rectangle::of([align_vecs[0].clone(), common_line.a.clone(), align_vecs[1].clone(), common_line.b.clone()])
    }

    fn get_common_line(&self) -> Rc<Line<T, N>> {
        let [a_lines, b_lines]: [[Rc<Line<T, N>>; 3]; 2] =
            [self.a.clone().to(), self.b.clone().to()];
        a_lines
            .into_iter()
            .find(|line| b_lines.contains(line))
            .unwrap()
    }

    fn get_align_vecs(&self) -> [Rc<Vector<T, N>>; 2] {
        let common_line: Rc<Line<T, N>> = self.get_common_line();
        ((*self).clone()).to::<[Rc<Vector<T, N>>; 4]>()
            .into_iter()
            .filter(|point| !common_line.clone().to::<[Rc<Vector<T, N>>; 2]>().contains(point))
            .collect::<Vec<Rc<Vector<T, N>>>>()
            .try_into()
            .unwrap()
    }
}

