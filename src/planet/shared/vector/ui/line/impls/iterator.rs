use crate::planet::shared::vector::{ui::line::Line, Vector};

impl<'a, T> IntoIterator for Line<'a, T> {
    type Item = &'a Vector<T>;
    type IntoIter = std::array::IntoIter<&'a Vector<T>, 2>;
    
    fn into_iter(self) -> Self::IntoIter {
        [self.a, self.b].into_iter()
    }
}

pub struct LineIterator<'a, T> {
    iter: &'a Line<'a, T>,
    count: usize
}

impl<'a, T> Iterator for LineIterator<'a, T> {
    type Item = &'a Vector<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.count {
            0 => {
                self.count += 1;
                Some(self.iter.a)
            },
            1 => {
                self.count += 1;
                Some(self.iter.b)
            },
            _ => None
        };
        result
    }
}

impl<'a, T> Line<'a, T> {
    pub fn iter(&self) -> LineIterator<T> {
        LineIterator {
            iter: self,
            count: 0
        }
    }
}