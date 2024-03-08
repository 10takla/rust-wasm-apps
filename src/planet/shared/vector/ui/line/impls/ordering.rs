use crate::planet::shared::vector::ui::line::Line;


impl<T: PartialEq, const N: usize> PartialEq for Line<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
        || self.a == other.b && self.b == other.a
    }
}