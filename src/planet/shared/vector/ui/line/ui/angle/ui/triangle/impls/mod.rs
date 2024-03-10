mod display;
mod iterator;

use super::Triangle;
use crate::planet::shared::{
    point::Point,
    vector::{
        ui::line::{ui::angle::Angle, Line},
        Number, Vector,
    },
};
use std::{fmt::Debug, rc::Rc};

/* -------FROM------- */

// from Angle
impl<T: Copy, const N: usize> From<[Angle<T, N>; 3]> for Triangle<T, N> {
    fn from(angles: [Angle<T, N>; 3]) -> Self {
        Self {
            abc: Rc::new(angles[0].clone()),
            bca: Rc::new(angles[1].clone()),
            cab: Rc::new(angles[2].clone()),
        }
    }
}

// from Line
impl<T: Copy, const N: usize> From<[Line<T, N>; 3]> for Triangle<T, N> {
    fn from(points: [Line<T, N>; 3]) -> Self {
        let (ab, bc, ac) = (points[0].clone(), points[1].clone(), points[2].clone());
        Self {
            cab: Rc::new([ab.clone(), ac.clone()].into()),
            abc: Rc::new([ab.clone(), bc.clone()].into()),
            bca: Rc::new([ac.clone(), bc.clone()].into()),
        }
    }
}

// from Vector
impl<T: Number, const N: usize> From<Vec<Rc<Vector<T, N>>>> for Triangle<T, N> {
    fn from(vecs: Vec<Rc<Vector<T, N>>>) -> Self {
        Self {
            cab: Rc::new([vecs[2].clone(), vecs[0].clone(), vecs[1].clone()].into()),
            abc: Rc::new([vecs[0].clone(), vecs[1].clone(), vecs[2].clone()].into()),
            bca: Rc::new([vecs[1].clone(), vecs[2].clone(), vecs[0].clone()].into()),
        }
    }
}

impl<T: Copy, const N: usize> From<[Vector<T, N>; 3]> for Triangle<T, N> {
    fn from(vecs: [Vector<T, N>; 3]) -> Self {
        Self {
            cab: Rc::new([Rc::new(vecs[2]), Rc::new(vecs[0]), Rc::new(vecs[1])].into()),
            abc: Rc::new([Rc::new(vecs[0]), Rc::new(vecs[1]), Rc::new(vecs[2])].into()),
            bca: Rc::new([Rc::new(vecs[1]), Rc::new(vecs[2]), Rc::new(vecs[0])].into()),
        }
    }
}

impl<T: Copy, const N: usize> From<[&Vector<T, N>; 3]> for Triangle<T, N> {
    fn from(vecs: [&Vector<T, N>; 3]) -> Self {
        Self {
            cab: Rc::new([Rc::new(*vecs[2]), Rc::new(*vecs[0]), Rc::new(*vecs[1])].into()),
            abc: Rc::new([Rc::new(*vecs[0]), Rc::new(*vecs[1]), Rc::new(*vecs[2])].into()),
            bca: Rc::new([Rc::new(*vecs[1]), Rc::new(*vecs[2]), Rc::new(*vecs[0])].into()),
        }
    }
}

impl<T: Copy, const N: usize> From<[Rc<Vector<T, N>>; 3]> for Triangle<T, N> {
    fn from(vecs: [Rc<Vector<T, N>>; 3]) -> Self {
        Self {
            cab: Rc::new([vecs[2].clone(), vecs[0].clone(), vecs[1].clone()].into()),
            abc: Rc::new([vecs[0].clone(), vecs[1].clone(), vecs[2].clone()].into()),
            bca: Rc::new([vecs[1].clone(), vecs[2].clone(), vecs[0].clone()].into()),
        }
    }
}

impl<T: Copy, const N: usize> From<[&Rc<Vector<T, N>>; 3]> for Triangle<T, N> {
    fn from(vecs: [&Rc<Vector<T, N>>; 3]) -> Self {
        Self {
            cab: Rc::new([*vecs[2].clone(), *vecs[0].clone(), *vecs[1].clone()].into()),
            abc: Rc::new([*vecs[0].clone(), *vecs[1].clone(), *vecs[2].clone()].into()),
            bca: Rc::new([*vecs[1].clone(), *vecs[2].clone(), *vecs[0].clone()].into()),
        }
    }
}

// from Point
impl<T: Number, const N: usize> From<Vec<Point<T, N>>> for Triangle<T, N> {
    fn from(vecs: Vec<Point<T, N>>) -> Self {
        Self {
            cab: Rc::new([vecs[2], vecs[0], vecs[1]].into()),
            abc: Rc::new([vecs[0], vecs[1], vecs[2]].into()),
            bca: Rc::new([vecs[1], vecs[2], vecs[0]].into()),
        }
    }
}

impl<T: Number, const N: usize> From<[Point<T, N>; 3]> for Triangle<T, N> {
    fn from(vecs: [Point<T, N>; 3]) -> Self {
        Self {
            cab: Rc::new([vecs[2], vecs[0], vecs[1]].into()),
            abc: Rc::new([vecs[0], vecs[1], vecs[2]].into()),
            bca: Rc::new([vecs[1], vecs[2], vecs[0]].into()),
        }
    }
}

/* -------FOR------- */

// for Line
impl<T: Eq + Debug + Copy, const N: usize> From<Triangle<T, N>> for [Line<T, N>; 3] {
    fn from(value: Triangle<T, N>) -> Self {
        let lines: Vec<Line<T, N>> = value
            .into_iter()
            .map(|angle| {
                let lines: [Line<T, N>; 2] = (*angle).clone().into();
                lines
            })
            .flatten()
            .into_iter()
            .fold(vec![], |mut acc, line| {
                // dbg!((&acc, &line));
                if !acc.contains(&line) {
                    acc.push(line)
                }
                acc
            });
        lines.try_into().unwrap()
    }
}

// for Vector
impl<T: Copy, const N: usize> From<Triangle<T, N>> for [Vector<T, N>; 3] {
    fn from(value: Triangle<T, N>) -> Self {
        (*value.abc).clone().into()
    }
}

// for Point
impl<T: Copy, const N: usize> From<Triangle<T, N>> for [Point<T, N>; 3] {
    fn from(value: Triangle<T, N>) -> Self {
        (*value.abc).clone().into()
    }
}

impl<T: Copy, const N: usize> From<&Triangle<T, N>> for [Point<T, N>; 3] {
    fn from(value: &Triangle<T, N>) -> Self {
        (*value.abc).clone().into()
    }
}
