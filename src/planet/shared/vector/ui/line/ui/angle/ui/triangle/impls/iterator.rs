use std::rc::Rc;

use crate::planet::shared::vector::ui::line::ui::angle::Angle;
use super::Triangle;

impl<T, const N: usize> IntoIterator for Triangle<T, N> {
    type Item = Rc<Angle<T, N>>;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.cab, self.abc, self.bca].into_iter()
    }
}
pub struct TriangleIterator<'a, T, const N: usize> {
    iter: &'a Triangle<T, N>,
    count: usize,
}

impl<'a, T, const N: usize> Iterator for TriangleIterator<'a, T, N> {
    type Item = &'a Angle<T, N>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.count {
            0 => {
                self.count += 1;
                Some(&self.iter.cab)
            },
            1 => {
                self.count += 1;
                Some(&self.iter.abc)
            },
            2 => {
                self.count += 1;
                Some(&self.iter.bca)
            },
            _ => None
        }
    }
}

impl<T, const N: usize> Triangle<T, N> {
    pub fn iter(&self) -> TriangleIterator<T, N> {
        TriangleIterator {
            iter: self,
            count: 0,
        }
    }
}