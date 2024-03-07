mod display;
mod iterator;
mod ordering;

use std::ops::SubAssign;

use super::Line;
use crate::planet::shared::{
        point::Point,
        vector::{Number, Vector},
    };

impl<F: Number> Line<F> {
    fn as_<I: Number>(self) -> Line<I> {
        let new_line: [Vector<I>; 2] = self
            .into_iter()
            .map(|p| p.as_())
            .collect::<Vec<Vector<I>>>()
            .try_into()
            .unwrap();
        Line::from(new_line)
    }
}

impl<T: Copy> From<Line<T>> for [Point<T>; 2] {
    fn from(value: Line<T>) -> Self {
        let vecs: [Vector<T>; 2] = value.into();
        [vecs[0].into(), vecs[1].into()]
    }
}

impl<T> From<Line<T>> for [Vector<T>; 2] {
    fn from(line: Line<T>) -> Self {
        [line.a, line.b]
    }
}

impl<T: Copy + SubAssign> From<Line<T>> for Vector<T> {
    fn from(line: Line<T>) -> Self {
        line.b - line.a
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
