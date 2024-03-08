mod display;
mod iterator;
mod ordering;

use std::ops::SubAssign;

use super::Line;
use crate::planet::shared::{
    point::Point,
    vector::{Number, Vector},
};

impl<F: Number, const N: usize> Line<F, N> {
    fn as_<I: Number>(self) -> Line<I, N> {
        let new_line: [Vector<I, N>; 2] = self
            .into_iter()
            .map(|vector| vector.as_())
            .collect::<Vec<Vector<I, N>>>()
            .try_into()
            .unwrap();
        Line::from(new_line)
    }
}

impl<T: Copy, const N: usize> From<Line<T, N>> for [Point<T, N>; 2] {
    fn from(value: Line<T, N>) -> Self {
        let vecs: [Vector<T, N>; 2] = value.into();
        [vecs[0].into(), vecs[1].into()]
    }
}

impl<T, const N: usize> From<Line<T, N>> for [Vector<T, N>; 2] {
    fn from(line: Line<T, N>) -> Self {
        [line.a, line.b]
    }
}

impl<T: Copy + SubAssign, const N: usize> From<Line<T, N>> for Vector<T, N> {
    fn from(line: Line<T, N>) -> Self {
        line.b - line.a
    }
}

impl<T: Copy, const N: usize> From<[Vector<T, N>; 2]> for Line<T, N> {
    fn from(vecs: [Vector<T, N>; 2]) -> Self {
        Self {
            a: vecs[0],
            b: vecs[1],
        }
    }
}

impl<T: Number, const N: usize> From<[Point<T, N>; 2]> for Line<T, N> {
    fn from(vecs: [Point<T, N>; 2]) -> Self {
        Self {
            a: vecs[0].into(),
            b: vecs[1].into(),
        }
    }
}
