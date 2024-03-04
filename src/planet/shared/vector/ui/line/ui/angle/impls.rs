use super::Angle;
use crate::planet::shared::{
    point::Point,
    vector::{ui::line::Line, Vector},
};


impl<T> From<Angle<T>> for [Line<T>; 2] {
    fn from(value: Angle<T>) -> Self {
        [value.ba, value.bc]
    }
}

impl<T> From<Angle<T>> for [Vector<T>; 3] {
    fn from(value: Angle<T>) -> Self {
        [value.ba.b.into(), value.ba.a.into(), value.bc.b.into()]
    }
}

impl<T> From<Angle<T>> for [Point<T>; 3] {
    fn from(value: Angle<T>) -> Self {
        [value.ba.b.into(), value.ba.a.into(), value.bc.b.into()]
    }
}

impl<T: Copy> From<[Line<T>; 2]> for Angle<T> {
    fn from(lines: [Line<T>; 2]) -> Self {
        Self {
            ba: lines[0],
            bc: lines[1],
        }
    }
}

impl<T: Copy> From<[Vector<T>; 3]> for Angle<T> {
    fn from(vecs: [Vector<T>; 3]) -> Self {
        let (a, b, c) = (vecs[0], vecs[1], vecs[2]);
        Self {
            ba: [b, a].into(),
            bc: [b, c].into(),
        }
    }
}

impl<T: Copy> From<[Point<T>; 3]> for Angle<T> {
    fn from(vecs: [Point<T>; 3]) -> Self {
        let (a, b, c) = (vecs[0], vecs[1], vecs[2]);
        Self {
            ba: [b, a].into(),
            bc: [b, c].into(),
        }
    }
}