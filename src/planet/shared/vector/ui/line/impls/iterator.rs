use std::rc::Rc;

use crate::planet::shared::vector::{ui::line::Line, Vector};

impl<T, const N: usize> IntoIterator for Line<T, N> {
    type Item = Rc<Vector<T, N>>;
    type IntoIter = std::array::IntoIter<Self::Item, 2>;
    
    fn into_iter(self) -> Self::IntoIter {
        [self.a, self.b].into_iter()
    }
}

pub struct LineIterator<'a, T, const N: usize> {
    iter: &'a Line<T, N>,
    count: usize
}

impl<'a, T, const N: usize> Iterator for LineIterator<'a, T, N> {
    type Item = &'a Rc<Vector<T, N>>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.count {
            0 => {
                self.count += 1;
                Some(&self.iter.a)
            },
            1 => {
                self.count += 1;
                Some(&self.iter.b)
            },
            _ => None
        };
        result
    }
}

impl<T, const N: usize> Line<T, N> {
    pub fn iter(&self) -> LineIterator<T, N> {
        LineIterator {
            iter: self,
            count: 0
        }
    }
}