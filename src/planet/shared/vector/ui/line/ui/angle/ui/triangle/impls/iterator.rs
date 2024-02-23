use crate::planet::shared::vector::ui::line::ui::angle::Angle;
use super::Triangle;

impl IntoIterator for Triangle {
    type Item = Angle;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.bac, self.abc, self.acb].into_iter()
    }
}
pub struct TriangleIterator<'a> {
    iter: &'a Triangle,
    count: usize,
}

impl<'a> Iterator for TriangleIterator<'a> {
    type Item = &'a Angle;
    fn next(&mut self) -> Option<Self::Item> {
        match self.count {
            0 => {
                self.count += 1;
                Some(&self.iter.bac)
            },
            1 => {
                self.count += 1;
                Some(&self.iter.abc)
            },
            2 => {
                self.count += 1;
                Some(&self.iter.acb)
            },
            _ => None
        }
    }
}

impl Triangle {
    pub fn iter(&self) -> TriangleIterator {
        TriangleIterator {
            iter: self,
            count: 0,
        }
    }
}