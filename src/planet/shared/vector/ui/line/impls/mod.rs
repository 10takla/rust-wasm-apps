mod iterator;
mod ordering;

use super::Line;
use crate::planet::shared::{point::Point, vector::Vector};

impl<T: Copy> From<Line<T>> for [Point<T> ; 2] {
    fn from(value: Line<T>) -> Self {
        let vecs: [Vector<T>; 2] = value.into();
        [vecs[0].into(), vecs[1].into()]
    }
}

impl<T> From<Line<T>> for [Vector<T> ; 2] {
    fn from(line: Line<T> ) -> Self {
        [line.a, line.b]
    }
}


impl<T: Copy> From<[Vector<T>; 2]> for Line<T> {
    fn from(vecs: [Vector<T>; 2]) -> Self {
        Self {
            a: vecs[0],
            b: vecs[1],
        }
    }
}

impl<T: Copy> From<[Point<T>; 2]> for Line<T> {
    fn from(vecs: [Point<T>; 2]) -> Self {
        Self {
            a: vecs[0].into(),
            b: vecs[1].into(),
        }
    }
}