use std::rc::Rc;

use super::Angle;
use crate::planet::shared::{
    point::Point,
    vector::{ui::line::Line, Number, Vector},
};


impl<T, const N: usize> From<Angle<T, N>> for [Line<T, N>; 2] {
    fn from(value: Angle<T, N>) -> Self {
        [value.ba, value.bc]
    }
}

impl<T: Copy, const N: usize> From<Angle<T, N>> for [Vector<T, N>; 3] {
    fn from(value: Angle<T, N>) -> Self {
        [*value.ba.b, *value.ba.a, *value.ba.b]
    }
}


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


impl<T: Copy, const N: usize> From<[Line<T, N>; 2]> for Angle<T, N> {
    fn from(lines: [Line<T, N>; 2]) -> Self {
        Self {
            ba: lines[0].clone(),
            bc: lines[1].clone(),
        }
    }
}

impl<T: Copy, const N: usize> From<[Rc<Vector<T, N>>; 3]> for Angle<T, N> {
    fn from(vecs: [Rc<Vector<T, N>>; 3]) -> Self {
        let [a, b, c] = [vecs[0].clone(), vecs[1].clone(), vecs[2].clone()];
        Self {
            ba: [b.clone(), a.clone()].into(),
            bc: [b.clone(), c.clone()].into(),
        }
    }
}
impl<T: Copy, const N: usize> From<[Vector<T, N>; 3]> for Angle<T, N> {
    fn from(vecs: [Vector<T, N>; 3]) -> Self {
        let [a, b, c] = [vecs[0].clone(), vecs[1].clone(), vecs[2].clone()];
        Self {
            ba: [Rc::new(b), Rc::new(a)].into(),
            bc: [Rc::new(b), Rc::new(c)].into(),
        }
    }
}


impl<T: Number, const N: usize> From<[Point<T, N>; 3]> for Angle<T, N> {
    fn from(vecs: [Point<T, N>; 3]) -> Self {
        let (a, b, c) = (vecs[0], vecs[1], vecs[2]);
        Self {
            ba: [b, a].into(),
            bc: [b, c].into(),
        }
    }
}