use super::Angle;
use crate::planet::shared::{
    point::Point,
    vector::{ui::line::Line, Vector},
};


impl<'a, T> From<Angle<'a, T>> for [Line<'a, T>; 2] {
    fn from(value: Angle<'a, T>) -> Self {
        [value.ba, value.bc]
    }
}

impl<'a, T> From<Angle<'a, T>> for [&'a Vector<T>; 3] {
    fn from(value: Angle<'a, T>) -> Self {
        [value.ba.b.into(), value.ba.a.into(), value.bc.b.into()]
    }
}

impl<'a, T> From<Angle<'a, T>> for [&'a Point<T>; 3] {
    fn from(value: Angle<'a, T>) -> Self {
        [value.ba.b.into(), value.ba.a.into(), value.bc.b.into()]
    }
}

impl<'a, T: Copy> From<[Line<'a, T>; 2]> for Angle<'a, T> {
    fn from(lines: [Line<'a, T>; 2]) -> Self {
        Self {
            ba: lines[0],
            bc: lines[1],
        }
    }
}

impl<'a, T: Copy> From<[&'a Vector<T>; 3]> for Angle<'a, T> {
    fn from(vecs: [&'a Vector<T>; 3]) -> Self {
        let (a, b, c) = (vecs[0], vecs[1], vecs[2]);
        Self {
            ba: [b, a].into(),
            bc: [b, c].into(),
        }
    }
}