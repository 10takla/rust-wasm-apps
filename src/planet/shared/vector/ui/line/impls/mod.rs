mod iterator;
mod ordering;

use super::Line;
use crate::planet::shared::{point::Point, vector::{Number, Vector}};

impl<T: Copy> From<Line<T>> for [Point<T> ; 2] {
    fn from(value: Line<T>) -> Self {
        let vecs: [Vector<T>; 2] = value.into();
        [vecs[0].into(), vecs[1].into()]
    }
}

impl<T> From<Line<T> > for [Vector<T> ; 2] {
    fn from(value: Line<T> ) -> Self {
        [value.a, value.b]
    }
}

impl<F, I> From<&Line<F>> for Line<I>
where
    F: Copy + Into<I> + Number,
    I: Copy + Number,
{
    fn from(line: &Line<F>) -> Self {
        let (a, b): (Vector<I>, Vector<I>) = ((&line.a).into(), (&line.b).into());
        let line: Line<I> = Line::from([a, b]);
        line
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
    fn from(points: [Point<T>; 2]) -> Self {
        Self {
            a: points[0].into(),
            b: points[1].into(),
        }
    }
}
