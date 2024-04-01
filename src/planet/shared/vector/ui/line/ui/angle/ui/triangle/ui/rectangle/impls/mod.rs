mod alien;
mod find;
mod has;
mod iterator;
mod of_to;

use super::Rectangle;
use crate::planet::shared::vector::{ui::line::ui::angle::ui::triangle::Triangle, Number};
use std::fmt::Debug;

// Debug
impl<T: Number, const N: usize> Debug for Rectangle<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Rectangle")
            .field("common_line", &self.get_common_line())
            .field("vectors", &self.get_align_vecs())
            .finish()
    }
}
