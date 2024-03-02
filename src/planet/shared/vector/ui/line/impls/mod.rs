mod iterator;
mod ordering;

use super::Line;
use crate::planet::shared::{point::Point, vector::Vector};

impl<'a, T: Copy> From<Line<'a, T>> for [&'a Point<T> ; 2] {
    fn from(value: Line<'a, T>) -> Self {
        let vecs: [&Vector<T>; 2] = value.into();
        [vecs[0].into(), vecs[1].into()]
    }
}

impl<'a, T> From<Line<'a, T>> for [&'a Vector<T> ; 2] {
    fn from(line: Line<'a, T> ) -> Self {
        [line.a, line.b]
    }
}


impl<'a, T: Copy> From<[&'a Vector<T>; 2]> for Line<'a, T> {
    fn from(vecs: [&'a Vector<T>; 2]) -> Self {
        Self {
            a: vecs[0],
            b: vecs[1],
        }
    }
}