mod display;
mod iterator;

use std::fmt::Debug;
use super::Triangle;
use crate::planet::shared::{
    point::Point,
    vector::{
        ui::line::{ui::angle::Angle, Line}, Vector,
    },
};

impl<'a, T: Eq + Debug> From<Triangle<'a, T>> for [Line<'a, T>; 3] {
    fn from(value: Triangle<'a, T>) -> Self {
        let mut lines: Vec<Line<'a, T>> = value.into_iter().map(|angle| {
            let lines: [Line<'a, T>; 2] = angle.into();
            lines
        }).flatten().collect();
        lines.dedup();
        lines.try_into().unwrap()
    }
}
impl<'a, T> From<Triangle<'a, T>> for [&'a Vector<T>; 3] {
    fn from(value: Triangle<'a, T>) -> Self {
        value.abc.into()
    }
}

impl<'a, T> From<Triangle<'a, T>> for [&'a Point<T>; 3] {
    fn from(value: Triangle<'a, T>) -> Self {
        value.abc.into()
    }
}

impl<'a, T: Copy> From<[Angle<'a, T>; 3]> for Triangle<'a, T> {
    fn from(angles: [Angle<'a, T>; 3]) -> Self {
        Self {
            abc: angles[0],
            bca: angles[1],
            cab: angles[2],
        }
    }
}

impl<'a, T: Copy> From<[Line<'a, T>; 3]> for Triangle<'a, T> {
    fn from(points: [Line<'a, T>; 3]) -> Self {
        let (ab, bc, ac) = (points[0], points[1], points[2]);
        Self {
            cab: [ab, ac].into(),
            abc: [ab, bc].into(),
            bca: [ac, bc].into(),
        }
    }
}

impl<'a, T: Copy> From<[&'a Vector<T>; 3]> for Triangle<'a, T> {
    fn from(vecs: [&'a Vector<T>; 3]) -> Self {
        Self {
            cab: [vecs[2], vecs[0], vecs[1]].into(),
            abc: [vecs[0], vecs[1], vecs[2]].into(),
            bca: [vecs[1], vecs[2], vecs[0]].into(),
        }
    }
}