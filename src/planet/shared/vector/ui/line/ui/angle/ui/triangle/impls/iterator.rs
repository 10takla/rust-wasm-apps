use crate::planet::shared::vector::ui::line::ui::angle::Angle;
use super::Triangle;

impl<'a, T> IntoIterator for Triangle<'a, T> {
    type Item = Angle<'a, T>;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.cab, self.abc, self.bca].into_iter()
    }
}
pub struct TriangleIterator<'a, T> {
    iter: &'a Triangle<'a, T>,
    count: usize,
}

impl<'a, T> Iterator for TriangleIterator<'a, T> {
    type Item = &'a Angle<'a, T>;
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

impl<'a, T> Triangle<'a, T> {
    pub fn iter(&self) -> TriangleIterator<T> {
        TriangleIterator {
            iter: self,
            count: 0,
        }
    }
}