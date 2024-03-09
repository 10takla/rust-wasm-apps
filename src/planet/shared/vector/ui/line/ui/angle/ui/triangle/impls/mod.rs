mod display;
mod iterator;

use std::{fmt::Debug, rc::Rc};
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
impl<T: Copy, const N: usize> From<Triangle<T, N>> for [Vector<T, N>; 3] {
    fn from(value: Triangle<T, N>) -> Self {
        value.abc.into()
    }
}

impl<T: Copy, const N: usize> From<Triangle<T, N>> for [Point<T, N>; 3] {
    fn from(value: Triangle<T, N>) -> Self {
        value.abc.into()
    }
}

impl<T: Copy, const N: usize> From<&Triangle<T, N>> for [Point<T, N>; 3] {
    fn from(value: &Triangle<T, N>) -> Self {
        value.abc.clone().into()
    }
}

impl<T: Copy, const N: usize> From<[Angle<T, N>; 3]> for Triangle<T, N> {
    fn from(angles: [Angle<T, N>; 3]) -> Self {
        Self {
            abc: angles[0].clone(),
            bca: angles[1].clone(),
            cab: angles[2].clone(),
        }
    }
}

impl<T: Copy, const N: usize> From<[Line<T, N>; 3]> for Triangle<T, N> {
    fn from(points: [Line<T, N>; 3]) -> Self {
        let (ab, bc, ac) = (points[0].clone(), points[1].clone(), points[2].clone());
        Self {
            cab: [ab.clone(), ac.clone()].into(),
            abc: [ab.clone(), bc.clone()].into(),
            bca: [ac.clone(), bc.clone()].into(),
        }
    }
}


impl<T: Copy, const N: usize> From<[Rc<Vector<T, N>>; 3]> for Triangle<T, N> {
    fn from(vecs: [Rc<Vector<T, N>>; 3]) -> Self {
        Self {
            cab: [vecs[2].clone(), vecs[0].clone(), vecs[1].clone()].into(),
            abc: [vecs[0].clone(), vecs[1].clone(), vecs[2].clone()].into(),
            bca: [vecs[1].clone(), vecs[2].clone(), vecs[0].clone()].into(),
        }
    }
}

impl<T: Copy, const N: usize> From<[&Rc<Vector<T, N>>; 3]> for Triangle<T, N> {
    fn from(vecs: [&Rc<Vector<T, N>>; 3]) -> Self {
        Self {
            cab: [*vecs[2].clone(), *vecs[0].clone(), *vecs[1].clone()].into(),
            abc: [*vecs[0].clone(), *vecs[1].clone(), *vecs[2].clone()].into(),
            bca: [*vecs[1].clone(), *vecs[2].clone(), *vecs[0].clone()].into(),
        }
    }
}

impl<T: Copy, const N: usize> From<[Vector<T, N>; 3]> for Triangle<T, N> {
    fn from(vecs: [Vector<T, N>; 3]) -> Self {
        Self {
            cab: [Rc::new(vecs[2]), Rc::new(vecs[0]), Rc::new(vecs[1])].into(),
            abc: [Rc::new(vecs[0]), Rc::new(vecs[1]), Rc::new(vecs[2])].into(),
            bca: [Rc::new(vecs[1]), Rc::new(vecs[2]), Rc::new(vecs[0])].into(),
        }
    }
}

impl<T: Copy, const N: usize> From<[&Vector<T, N>; 3]> for Triangle<T, N> {
    fn from(vecs: [&Vector<T, N>; 3]) -> Self {
        Self {
            cab: [Rc::new(*vecs[2]), Rc::new(*vecs[0]), Rc::new(*vecs[1])].into(),
            abc: [Rc::new(*vecs[0]), Rc::new(*vecs[1]), Rc::new(*vecs[2])].into(),
            bca: [Rc::new(*vecs[1]), Rc::new(*vecs[2]), Rc::new(*vecs[0])].into(),
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

impl<T: Number, const N: usize> From<Vec<Rc<Vector<T, N>>>> for Triangle<T, N> {
    fn from(vecs: Vec<Rc<Vector<T, N>>>) -> Self {
        Self {
            cab: [vecs[2].clone(), vecs[0].clone(), vecs[1].clone()].into(),
            abc: [vecs[0].clone(), vecs[1].clone(), vecs[2].clone()].into(),
            bca: [vecs[1].clone(), vecs[2].clone(), vecs[0].clone()].into(),
        }
    }
}

impl<T: Number, const N: usize> From<Vec<Point<T, N>>> for Triangle<T, N> {
    fn from(vecs: Vec<Point<T, N>>) -> Self {
        Self {
            cab: [vecs[2], vecs[0], vecs[1]].into(),
            abc: [vecs[0], vecs[1], vecs[2]].into(),
            bca: [vecs[1], vecs[2], vecs[0]].into(),
        }
    }
}