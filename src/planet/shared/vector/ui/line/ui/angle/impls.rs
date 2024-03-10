use std::rc::Rc;

use super::Angle;
use crate::planet::shared::{
    point::Point,
    vector::{ui::line::Line, Number, Vector},
};


/* -------FROM------- */

// from Line
impl<T: Copy, const N: usize> From<[Line<T, N>; 2]> for Angle<T, N> {
    fn from(lines: [Line<T, N>; 2]) -> Self {
        Self {
            ba: Rc::new(lines[0].clone()),
            bc: Rc::new(lines[1].clone()),
        }
    }
}

impl<T: Copy, const N: usize> From<[Rc<Line<T, N>>; 2]> for Angle<T, N> {
    fn from(lines: [Rc<Line<T, N>>; 2]) -> Self {
        Self {
            ba: lines[0].clone(),
            bc: lines[1].clone(),
        }
    }
}

// from Vector
impl<T: Copy, const N: usize> From<[Vector<T, N>; 3]> for Angle<T, N> {
    fn from(vecs: [Vector<T, N>; 3]) -> Self {
        let [a, b, c] = [Rc::new(vecs[0]), Rc::new(vecs[1]), Rc::new(vecs[2])];
        Self {
            ba: Rc::new([b.clone(), a.clone()].into()),
            bc:  Rc::new([b.clone(), c.clone()].into()),
        }
    }
}

impl<T: Copy, const N: usize> From<[Rc<Vector<T, N>>; 3]> for Angle<T, N> {
    fn from(vecs: [Rc<Vector<T, N>>; 3]) -> Self {
        let [a, b, c] = [vecs[0].clone(), vecs[1].clone(), vecs[2].clone()];
        Self {
            ba: Rc::new([b.clone(), a.clone()].into()),
            bc: Rc::new([b.clone(), c.clone()].into()),
        }
    }
}

// from Point
impl<T: Number, const N: usize> From<[Point<T, N>; 3]> for Angle<T, N> {
    fn from(vecs: [Point<T, N>; 3]) -> Self {
        let (a, b, c) = (vecs[0], vecs[1], vecs[2]);
        Self {
            ba: Rc::new([b, a].into()),
            bc: Rc::new([b, c].into()),
        }
    }
}

/* -------FOR------- */

// for Line
impl<T: Clone, const N: usize> From<Angle<T, N>> for [Line<T, N>; 2] {
    fn from(angle: Angle<T, N>) -> Self {
        [(*angle.ba).clone(), (*angle.bc).clone()]
    }
}

impl<T, const N: usize> From<Angle<T, N>> for [Rc<Line<T, N>>; 2] {
    fn from(angle: Angle<T, N>) -> Self {
        [angle.ba, angle.bc]
    }
}

// for Vector
impl<T: Copy, const N: usize> From<Angle<T, N>> for [Vector<T, N>; 3] {
    fn from(value: Angle<T, N>) -> Self {
        [*value.ba.b, *value.ba.a, *value.ba.b]
    }
}

impl<T: Copy, const N: usize> From<Angle<T, N>> for [Rc<Vector<T, N>>; 3] {
    fn from(value: Angle<T, N>) -> Self {
        [value.ba.b.clone(), value.ba.a.clone(), value.ba.b.clone()]
    }
}

// for Point
impl<T: Copy, const N: usize> From<Angle<T, N>> for [Point<T, N>; 3] {
    fn from(value: Angle<T, N>) -> Self {
        [(*value.ba.b).into(), (*value.ba.a).into(), (*value.ba.b).into()]
    }
}

impl<T: Copy, const N: usize> From<&Angle<T, N>> for [Point<T, N>; 3] {
    fn from(value: &Angle<T, N>) -> Self {
        [(*value.ba.b).into(), (*value.ba.a).into(), (*value.ba.b).into()]
    }
}