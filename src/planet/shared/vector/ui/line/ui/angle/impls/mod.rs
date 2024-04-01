mod iterator;
mod of_to;
mod ordering;

use super::Angle;

use crate::{
    planet::shared::{
        traits::Has,
        vector::{ui::line::Line, Number, Vector},
    },
    traits::of_to::To,
};
use std::rc::Rc;
use std::fmt::Debug;

// Debug
impl<T: Number, const N: usize> Debug for Angle<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vecs = self.to::<[Vector<T, N>; 3]>();
        f.debug_struct("Rectangle")
            .field("left", &vecs[0])
            .field("center", &vecs[1])
            .field("right", &vecs[2])
            .finish()
    }
}

// Has
impl<T: Number, const N: usize> Has<Rc<Line<T, N>>> for Angle<T, N> {
    fn has(&self, line: &Rc<Line<T, N>>) -> bool {
        self.clone()
            .into_iter()
            .any(|angle_line| Rc::ptr_eq(&angle_line, &line.clone()))
    }
}