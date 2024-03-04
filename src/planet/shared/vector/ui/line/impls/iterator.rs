use crate::planet::shared::vector::{ui::line::Line, Vector};

impl<T> IntoIterator for Line<T> {
    type Item = Vector<T>;
    type IntoIter = std::array::IntoIter<Vector<T>, 2>;
    
    fn into_iter(self) -> Self::IntoIter {
        [self.a, self.b].into_iter()
    }
}

pub struct LineIterator<'a, T> {
    iter: &'a Line<T>,
    count: usize
}

impl<'a, T> Iterator for LineIterator<'a, T> {
    type Item = &'a Vector<T>;

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

impl<T> Line<T> {
    pub fn iter(&self) -> LineIterator<T> {
        LineIterator {
            iter: self,
            count: 0
        }
    }
}