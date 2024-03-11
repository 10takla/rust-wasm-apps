mod display;
mod iterator;
mod ordering;

use crate::traits::of_to::{Of, To};
use std::rc::Rc;
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
        Line::of(new_line)
    }
}

// from Vector
impl<T: Copy, const N: usize> Of<[Vector<T, N>; 2]> for Line<T, N> {
    fn of(vecs: [Vector<T, N>; 2]) -> Self {
        Self {
            a: Rc::new(vecs[0]),
            b: Rc::new(vecs[1]),
        }
    }
}

impl<T: Copy, const N: usize> Of<[Rc<Vector<T, N>>; 2]> for Line<T, N> {
    fn of(vecs: [Rc<Vector<T, N>>; 2]) -> Self {
        Self {
            a: vecs[0].clone(),
            b: vecs[1].clone(),
        }
    }
}

// from Point
impl<T: Number, const N: usize> Of<[Point<T, N>; 2]> for Line<T, N> {
    fn of(points: [Point<T, N>; 2]) -> Self {
        Self {
            a: Rc::new(points[0].to()),
            b: Rc::new(points[1].to()),
        }
    }
}

// for Point
impl<T: Copy, const N: usize> Of<Line<T, N>> for [Point<T, N>; 2] {
    fn of(line: Line<T, N>) -> Self {
        let vecs: [Rc<Vector<T, N>>; 2] = line.to();
        [(*vecs[0]).to(), (*vecs[1]).to()]
    }
}

// for Vector
impl<T: Copy, const N: usize> Of<Line<T, N>> for [Vector<T, N>; 2] {
    fn of(line: Line<T, N>) -> Self {
        [*line.a, *line.b]
    }
}

impl<T, const N: usize> Of<Line<T, N>> for [Rc<Vector<T, N>>; 2] {
    fn of(line: Line<T, N>) -> Self {
        [line.a, line.b]
    }
}

impl<T, const N: usize> Of<Rc<Line<T, N>>> for [Rc<Vector<T, N>>; 2] {
    fn of(line: Rc<Line<T, N>>) -> Self {
        [line.a.clone(), line.b.clone()]
    }
}

impl<T: Number, const N: usize> Of<Line<T, N>> for Vector<T, N> {
    fn of(line: Line<T, N>) -> Self {
        *line.b - *line.a
    }
}

impl<T: Number, const N: usize> Of<Rc<Line<T, N>>> for Vector<T, N> {
    fn of(line: Rc<Line<T, N>>) -> Self {
        *line.b - *line.a
    }
}