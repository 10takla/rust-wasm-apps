use std::rc::Rc;

use crate::planet::shared::vector::ui::line::Line;
use super::Angle;

impl<T, const N: usize> IntoIterator for Angle<T, N> {
    type Item = Rc<Line<T, N>>;
    type IntoIter = std::array::IntoIter<Self::Item, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.ba, self.bc].into_iter()
    }
}
pub struct TriangleIterator<'a, T, const N: usize> {
    iter: &'a Angle<T, N>,
    count: usize,
}

impl<'a, T, const N: usize> Iterator for TriangleIterator<'a, T, N> {
    type Item = &'a Line<T, N>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.count {
            0 => {
                self.count += 1;
                Some(&self.iter.ba)
            },
            1 => {
                self.count += 1;
                Some(&self.iter.bc)
            }
            _ => None
        }
    }
}

impl<T, const N: usize> Angle<T, N> {
    pub fn iter(&self) -> TriangleIterator<T, N> {
        TriangleIterator {
            iter: self,
            count: 0,
        }
    }
}