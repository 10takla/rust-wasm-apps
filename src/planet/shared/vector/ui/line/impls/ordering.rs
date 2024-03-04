use crate::planet::shared::vector::ui::line::Line;


impl<T: PartialEq> PartialEq for Line<T> {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
        || self.a == other.b && self.b == other.a
    }
}