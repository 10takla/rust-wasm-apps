mod display;
mod iterator;
mod ordering;

use std::{ops::{Deref, SubAssign}, rc::Rc};

use super::Line;
use crate::planet::shared::{
    point::Point,
    vector::{Number, Vector},
};

impl<F: Number, const N: usize> Line<F, N> {
    fn as_<I: Number>(self) -> Line<I, N> {
        let new_line: [Rc<Vector<I, N>>; 2] = self
            .into_iter()
            .map(|vector| Rc::new(vector.as_()))
            .collect::<Vec<Rc<Vector<I, N>>>>()
            .try_into()
            .unwrap();
        Line::from(new_line)
    }
}

impl<T: Copy, const N: usize> From<Line<T, N>> for [Point<T, N>; 2] {
    fn from(value: Line<T, N>) -> Self {
        let vecs: [Rc<Vector<T, N>>; 2] = value.into();
        [(*vecs[0]).into(), (*vecs[1]).into()]
    }
}

impl<T, const N: usize> From<Line<T, N>> for [Rc<Vector<T, N>>; 2] {
    fn from(line: Line<T, N>) -> Self {
        [line.a, line.b]
    }
}

impl<T: Copy, const N: usize> From<Line<T, N>> for [Vector<T, N>; 2] {
    fn from(line: Line<T, N>) -> Self {
        [*line.a, *line.b]
    }
}


impl<T: Number, const N: usize> From<Line<T, N>> for Vector<T, N> {
    fn from(line: Line<T, N>) -> Self {
        *line.b - *line.a
    }
}

impl<T: Copy, const N: usize> From<[Rc<Vector<T, N>>; 2]> for Line<T, N> {
    fn from(vecs: [Rc<Vector<T, N>>; 2]) -> Self {
        Self {
            a: vecs[0].clone(),
            b: vecs[1].clone(),
        }
    }
}

impl<T: Copy, const N: usize> From<[Vector<T, N>; 2]> for Line<T, N> {
    fn from(vecs: [Vector<T, N>; 2]) -> Self {
        Self {
            a: Rc::new(vecs[0]),
            b: Rc::new(vecs[1]),
        }
    }
}

impl<T: Number, const N: usize> From<[Point<T, N>; 2]> for Line<T, N> {
    fn from(vecs: [Point<T, N>; 2]) -> Self {
        Self {
            a: Rc::new(vecs[0].into()),
            b: Rc::new(vecs[1].into()),
        }
    }
}
