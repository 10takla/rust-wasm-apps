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

impl<T: Eq + Debug> From<Triangle<T>> for [Line<T>; 3] {
    fn from(value: Triangle<T>) -> Self {
        let mut lines: Vec<Line<T>> = value.into_iter().map(|angle| {
            let lines: [Line<T>; 2] = angle.into();
            lines
        }).flatten().collect();
        lines.dedup();
        lines.try_into().unwrap()
    }
}
impl<T> From<Triangle<T>> for [Vector<T>; 3] {
    fn from(value: Triangle<T>) -> Self {
        value.abc.into()
    }
}

impl<T> From<Triangle<T>> for [Point<T>; 3] {
    fn from(value: Triangle<T>) -> Self {
        value.abc.into()
    }
}

impl<T: Copy> From<[Angle<T>; 3]> for Triangle<T> {
    fn from(angles: [Angle<T>; 3]) -> Self {
        Self {
            abc: angles[0],
            bca: angles[1],
            cab: angles[2],
        }
    }
}

impl<T: Copy> From<[Line<T>; 3]> for Triangle<T> {
    fn from(points: [Line<T>; 3]) -> Self {
        let (ab, bc, ac) = (points[0], points[1], points[2]);
        Self {
            cab: [ab, ac].into(),
            abc: [ab, bc].into(),
            bca: [ac, bc].into(),
        }
    }
}

impl<T: Copy> From<[Vector<T>; 3]> for Triangle<T> {
    fn from(vecs: [Vector<T>; 3]) -> Self {
        Self {
            cab: [vecs[2], vecs[0], vecs[1]].into(),
            abc: [vecs[0], vecs[1], vecs[2]].into(),
            bca: [vecs[1], vecs[2], vecs[0]].into(),
        }
    }
}

impl<T: Copy> From<[Point<T>; 3]> for Triangle<T> {
    fn from(vecs: [Point<T>; 3]) -> Self {
        Self {
            cab: [vecs[2], vecs[0], vecs[1]].into(),
            abc: [vecs[0], vecs[1], vecs[2]].into(),
            bca: [vecs[1], vecs[2], vecs[0]].into(),
        }
    }
}