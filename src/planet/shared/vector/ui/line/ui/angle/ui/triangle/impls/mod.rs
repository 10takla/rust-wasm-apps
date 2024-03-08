mod display;
mod iterator;

use std::fmt::Debug;
use super::Triangle;
use crate::planet::shared::{
    point::Point,
    vector::{
        ui::line::{ui::angle::Angle, Line}, Number, Vector
    },
};

impl<T: Eq + Debug, const N: usize> From<Triangle<T, N>> for [Line<T, N>; 3] {
    fn from(value: Triangle<T, N>) -> Self {
        let mut lines: Vec<Line<T, N>> = value.into_iter().map(|angle| {
            let lines: [Line<T, N>; 2] = angle.into();
            lines
        }).flatten().collect();
        lines = lines.into_iter().fold(vec![], |mut acc, line| {
            // dbg!((&acc, &line));
            if !acc.contains(&line) {
                acc.push(line)
            }
            acc
        });
        lines.try_into().unwrap()
    }
}
impl<T, const N: usize> From<Triangle<T, N>> for [Vector<T, N>; 3] {
    fn from(value: Triangle<T, N>) -> Self {
        value.abc.into()
    }
}

impl<T, const N: usize> From<Triangle<T, N>> for [Point<T, N>; 3] {
    fn from(value: Triangle<T, N>) -> Self {
        value.abc.into()
    }
}

impl<T: Copy, const N: usize> From<[Angle<T, N>; 3]> for Triangle<T, N> {
    fn from(angles: [Angle<T, N>; 3]) -> Self {
        Self {
            abc: angles[0],
            bca: angles[1],
            cab: angles[2],
        }
    }
}

impl<T: Copy, const N: usize> From<[Line<T, N>; 3]> for Triangle<T, N> {
    fn from(points: [Line<T, N>; 3]) -> Self {
        let (ab, bc, ac) = (points[0], points[1], points[2]);
        Self {
            cab: [ab, ac].into(),
            abc: [ab, bc].into(),
            bca: [ac, bc].into(),
        }
    }
}

impl<T: Copy, const N: usize> From<[Vector<T, N>; 3]> for Triangle<T, N> {
    fn from(vecs: [Vector<T, N>; 3]) -> Self {
        Self {
            cab: [vecs[2], vecs[0], vecs[1]].into(),
            abc: [vecs[0], vecs[1], vecs[2]].into(),
            bca: [vecs[1], vecs[2], vecs[0]].into(),
        }
    }
}

impl<T: Number, const N: usize> From<[Point<T, N>; 3]> for Triangle<T, N> {
    fn from(vecs: [Point<T, N>; 3]) -> Self {
        Self {
            cab: [vecs[2], vecs[0], vecs[1]].into(),
            abc: [vecs[0], vecs[1], vecs[2]].into(),
            bca: [vecs[1], vecs[2], vecs[0]].into(),
        }
    }
}