mod display;
mod iterator;

use super::Triangle;
use crate::{planet::shared::{
    point::Point,
    vector::{
        ui::line::{ui::angle::Angle, Line},
        Number, Vector,
    },
}, traits::of_to::To};
use std::{fmt::Debug, rc::Rc};
use crate::traits::of_to::Of;

/* -------FROM------- */

// from Angle
impl<T: Copy, const N: usize> Of<[Angle<T, N>; 3]> for Triangle<T, N> {
    fn of(angles: [Angle<T, N>; 3]) -> Self {
        Self {
            abc: Rc::new(angles[0].clone()),
            bca: Rc::new(angles[1].clone()),
            cab: Rc::new(angles[2].clone()),
        }
    }
}

// from Line
impl<T: Copy, const N: usize> Of<[Line<T, N>; 3]> for Triangle<T, N> {
    fn of(lines: [Line<T, N>; 3]) -> Self {
        let [ab, bc, ac] = [lines[0].clone(), lines[1].clone(), lines[2].clone()];
        Self {
            cab: Rc::new([ab.clone(), ac.clone()].to()),
            abc: Rc::new([ab.clone(), bc.clone()].to()),
            bca: Rc::new([ac.clone(), bc.clone()].to()),
        }
    }
}

// from Vector
impl<T: Number, const N: usize> Of<Vec<Rc<Vector<T, N>>>> for Triangle<T, N> {
    fn of(vecs: Vec<Rc<Vector<T, N>>>) -> Self {
        Self {
            cab: Rc::new([vecs[2].clone(), vecs[0].clone(), vecs[1].clone()].to()),
            abc: Rc::new([vecs[0].clone(), vecs[1].clone(), vecs[2].clone()].to()),
            bca: Rc::new([vecs[1].clone(), vecs[2].clone(), vecs[0].clone()].to()),
        }
    }
}

impl<T: Copy, const N: usize> Of<[Vector<T, N>; 3]> for Triangle<T, N> {
    fn of(vecs: [Vector<T, N>; 3]) -> Self {
        Self {
            cab: Rc::new([Rc::new(vecs[2]), Rc::new(vecs[0]), Rc::new(vecs[1])].to()),
            abc: Rc::new([Rc::new(vecs[0]), Rc::new(vecs[1]), Rc::new(vecs[2])].to()),
            bca: Rc::new([Rc::new(vecs[1]), Rc::new(vecs[2]), Rc::new(vecs[0])].to()),
        }
    }
}

impl<T: Copy, const N: usize> Of<[&Vector<T, N>; 3]> for Triangle<T, N> {
    fn of(vecs: [&Vector<T, N>; 3]) -> Self {
        Self {
            cab: Rc::new([Rc::new(*vecs[2]), Rc::new(*vecs[0]), Rc::new(*vecs[1])].to()),
            abc: Rc::new([Rc::new(*vecs[0]), Rc::new(*vecs[1]), Rc::new(*vecs[2])].to()),
            bca: Rc::new([Rc::new(*vecs[1]), Rc::new(*vecs[2]), Rc::new(*vecs[0])].to()),
        }
    }
}

impl<T: Copy, const N: usize> Of<[Rc<Vector<T, N>>; 3]> for Triangle<T, N> {
    fn of(vecs: [Rc<Vector<T, N>>; 3]) -> Self {
        Self {
            cab: Rc::new([vecs[2].clone(), vecs[0].clone(), vecs[1].clone()].to()),
            abc: Rc::new([vecs[0].clone(), vecs[1].clone(), vecs[2].clone()].to()),
            bca: Rc::new([vecs[1].clone(), vecs[2].clone(), vecs[0].clone()].to()),
        }
    }
}

impl<T: Copy, const N: usize> Of<[&Rc<Vector<T, N>>; 3]> for Triangle<T, N> {
    fn of(vecs: [&Rc<Vector<T, N>>; 3]) -> Self {
        Self {
            cab: Rc::new([*vecs[2].clone(), *vecs[0].clone(), *vecs[1].clone()].to()),
            abc: Rc::new([*vecs[0].clone(), *vecs[1].clone(), *vecs[2].clone()].to()),
            bca: Rc::new([*vecs[1].clone(), *vecs[2].clone(), *vecs[0].clone()].to()),
        }
    }
}

// from Point
impl<T: Number, const N: usize> Of<Vec<Point<T, N>>> for Triangle<T, N> {
    fn of(points: Vec<Point<T, N>>) -> Self {
        Self {
            cab: Rc::new([points[2], points[0], points[1]].to()),
            abc: Rc::new([points[0], points[1], points[2]].to()),
            bca: Rc::new([points[1], points[2], points[0]].to()),
        }
    }
}

impl<T: Number, const N: usize> Of<[Point<T, N>; 3]> for Triangle<T, N> {
    fn of(points: [Point<T, N>; 3]) -> Self {
        Self {
            cab: Rc::new([points[2], points[0], points[1]].to()),
            abc: Rc::new([points[0], points[1], points[2]].to()),
            bca: Rc::new([points[1], points[2], points[0]].to()),
        }
    }
}

/* -------FOR------- */

// for Line
impl<T: Eq + Debug + Copy, const N: usize> Of<Triangle<T, N>> for [Line<T, N>; 3] {
    fn of(triangle: Triangle<T, N>) -> Self {
        let lines: Vec<Line<T, N>> = triangle
            .into_iter()
            .map(|angle| {
                let lines: [Line<T, N>; 2] = (*angle).clone().to();
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
impl<T: Copy, const N: usize> Of<Triangle<T, N>> for [Vector<T, N>; 3] {
    fn of(triangle: Triangle<T, N>) -> Self {
        (*triangle.abc).clone().to()
    }
}

impl<T: Copy, const N: usize> Of<Triangle<T, N>> for [Rc<Vector<T, N>>; 3] {
    fn of(triangle: Triangle<T, N>) -> Self {
        (*triangle.abc).clone().to()
    }
}

// for Point
impl<T: Copy, const N: usize> Of<Triangle<T, N>> for [Point<T, N>; 3] {
    fn of(triangle: Triangle<T, N>) -> Self {
        (*triangle.abc).clone().to()
    }
}

impl<T: Copy, const N: usize> Of<&Triangle<T, N>> for [Point<T, N>; 3] {
    fn of(triangle: &Triangle<T, N>) -> Self {
        (*triangle.abc).clone().to()
    }
}