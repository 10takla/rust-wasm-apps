use crate::{
    planet::shared::{
        point::Point,
        vector::{
            ui::line::{
                ui::angle::{ui::triangle::Triangle, Angle},
                Line,
            },
            Number, Vector,
        },
    },
    traits::of_to::{Of, To},
};
use std::fmt::Debug;
use std::rc::Rc;

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
impl<T: Number, const N: usize> Of<[Line<T, N>; 3]> for Triangle<T, N> {
    fn of(lines: [Line<T, N>; 3]) -> Self {
        let [ab, bc, ac] = [0, 1, 2].map(|ind| Rc::new(lines[ind].clone()));
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
        let lines: [Rc<Line<T, N>>; 3] = [[0, 1], [0, 2], [1, 2]]
            .map(|inds| Rc::new(Line::of([&vecs[inds[0]], &vecs[inds[1]]])));
        Self {
            cab: Rc::new([lines[0].clone(), lines[1].clone()].to()),
            abc: Rc::new([lines[0].clone(), lines[2].clone()].to()),
            bca: Rc::new([lines[1].clone(), lines[2].clone()].to()),
        }
    }
}

impl<T: Number, const N: usize> Of<[Vector<T, N>; 3]> for Triangle<T, N> {
    fn of(vecs: [Vector<T, N>; 3]) -> Self {
        let lines: [Rc<Line<T, N>>; 3] =
            [[0, 1], [0, 2], [1, 2]].map(|inds| Rc::new(Line::of([vecs[inds[0]], vecs[inds[1]]])));
        Self {
            cab: Rc::new([lines[0].clone(), lines[1].clone()].to()),
            abc: Rc::new([lines[0].clone(), lines[2].clone()].to()),
            bca: Rc::new([lines[1].clone(), lines[2].clone()].to()),
        }
    }
}

impl<T: Number, const N: usize> Of<[&Vector<T, N>; 3]> for Triangle<T, N> {
    fn of(vecs: [&Vector<T, N>; 3]) -> Self {
        let lines: [Rc<Line<T, N>>; 3] = [[0, 1], [0, 2], [1, 2]]
            .map(|inds| Rc::new(Line::of([*vecs[inds[0]], *vecs[inds[1]]])));
        Self {
            cab: Rc::new([lines[0].clone(), lines[1].clone()].to()),
            abc: Rc::new([lines[0].clone(), lines[2].clone()].to()),
            bca: Rc::new([lines[1].clone(), lines[2].clone()].to()),
        }
    }
}

impl<T: Number, const N: usize> Of<[Rc<Vector<T, N>>; 3]> for Triangle<T, N> {
    fn of(vecs: [Rc<Vector<T, N>>; 3]) -> Self {
        let lines: [Rc<Line<T, N>>; 3] = [[0, 1], [0, 2], [1, 2]]
            .map(|inds| Rc::new(Line::of([&vecs[inds[0]], &vecs[inds[1]]])));
        Self {
            cab: Rc::new([lines[0].clone(), lines[1].clone()].to()),
            abc: Rc::new([lines[0].clone(), lines[2].clone()].to()),
            bca: Rc::new([lines[1].clone(), lines[2].clone()].to()),
        }
    }
}

impl<T: Number, const N: usize> Of<[&Rc<Vector<T, N>>; 3]> for Triangle<T, N> {
    fn of(vecs: [&Rc<Vector<T, N>>; 3]) -> Self {
        let lines: [Rc<Line<T, N>>; 3] =
            [[0, 1], [0, 2], [1, 2]].map(|inds| Rc::new(Line::of([vecs[inds[0]], vecs[inds[1]]])));
        Self {
            cab: Rc::new([lines[0].clone(), lines[1].clone()].to()),
            abc: Rc::new([lines[0].clone(), lines[2].clone()].to()),
            bca: Rc::new([lines[1].clone(), lines[2].clone()].to()),
        }
    }
}

// from Point
impl<T: Number, const N: usize> Of<Vec<Point<T, N>>> for Triangle<T, N> {
    fn of(points: Vec<Point<T, N>>) -> Self {
        let lines: [Rc<Line<T, N>>; 3] = [[0, 1], [0, 2], [1, 2]]
            .map(|inds| Rc::new(Line::of([points[inds[0]], points[inds[1]]])));
        Self {
            cab: Rc::new([lines[0].clone(), lines[1].clone()].to()),
            abc: Rc::new([lines[0].clone(), lines[2].clone()].to()),
            bca: Rc::new([lines[1].clone(), lines[2].clone()].to()),
        }
    }
}

impl<T: Number, const N: usize> Of<[Point<T, N>; 3]> for Triangle<T, N> {
    fn of(points: [Point<T, N>; 3]) -> Self {
        let lines: [Rc<Line<T, N>>; 3] = [[0, 1], [0, 2], [1, 2]]
            .map(|inds| Rc::new(Line::of([points[inds[0]], points[inds[1]]])));
        Self {
            cab: Rc::new([lines[0].clone(), lines[1].clone()].to()),
            abc: Rc::new([lines[0].clone(), lines[2].clone()].to()),
            bca: Rc::new([lines[1].clone(), lines[2].clone()].to()),
        }
    }
}

/* -------FOR------- */

// for Angle
impl<T: Debug + Copy + Number, const N: usize> Of<Triangle<T, N>> for [Rc<Angle<T, N>>; 3] {
    fn of(triangle: Triangle<T, N>) -> Self {
        triangle
            .into_iter()
            .collect::<Vec<Rc<Angle<T, N>>>>()
            .try_into()
            .unwrap()
    }
}

impl<T: Debug + Copy + Number, const N: usize> Of<&Triangle<T, N>> for [Rc<Angle<T, N>>; 3] {
    fn of(triangle: &Triangle<T, N>) -> Self {
        triangle
            .clone()
            .into_iter()
            .collect::<Vec<Rc<Angle<T, N>>>>()
            .try_into()
            .unwrap()
    }
}

// for Line
impl<T: Debug + Copy + Number, const N: usize> Of<Triangle<T, N>> for [Line<T, N>; 3] {
    fn of(triangle: Triangle<T, N>) -> Self {
        Rc::new(triangle).to()
    }
}

impl<T: Debug + Copy + Number, const N: usize> Of<Triangle<T, N>> for [Rc<Line<T, N>>; 3] {
    fn of(triangle: Triangle<T, N>) -> Self {
        Rc::new(triangle).to()
    }
}

impl<T: Debug + Copy + Number, const N: usize> Of<Rc<Triangle<T, N>>> for [Line<T, N>; 3] {
    fn of(triangle: Rc<Triangle<T, N>>) -> Self {
        triangle.to::<[Rc<Line<T, N>>; 3]>().map(|line| (*line).clone())
    }
}

impl<T: Debug + Copy + Number, const N: usize> Of<Rc<Triangle<T, N>>> for [Rc<Line<T, N>>; 3] {
    fn of(triangle: Rc<Triangle<T, N>>) -> Self {
        (*triangle)
            .clone()
            .into_iter()
            .map(|angle| angle.to::<[Rc<Line<T, N>>; 2]>())
            .flatten()
            .into_iter()
            .fold(
                vec![],
                |mut acc: Vec<Rc<Line<T, N>>>, line: Rc<Line<T, N>>| {
                    if !acc.contains(&line) {
                        acc.push(line)
                    }
                    acc
                },
            )
            .try_into()
            .unwrap()
    }
}

impl<T: Debug + Copy + Number, const N: usize> Of<&Rc<Triangle<T, N>>> for [Rc<Line<T, N>>; 3] {
    fn of(triangle: &Rc<Triangle<T, N>>) -> Self {
        (*triangle).clone().to()
    }
}

impl<T: Copy + Number, const N: usize> Of<&Vec<Rc<Triangle<T, N>>>> for Vec<Rc<Line<T, N>>> {
    fn of(triangles: &Vec<Rc<Triangle<T, N>>>) -> Self {
        (*triangles)
            .clone()
            .into_iter()
            .map(|triangle| triangle.to::<[Rc<Line<T, N>>; 3]>())
            .flatten()
            .fold(vec![], |mut acc, line| {
                if !acc.contains(&line) {
                    acc.push(line);
                }
                acc
            })
    }
}

// for Vector
impl<T: Copy, const N: usize> Of<Triangle<T, N>> for [Vector<T, N>; 3] {
    fn of(triangle: Triangle<T, N>) -> Self {
        (*triangle.abc).clone().to()
    }
}

impl<T: Copy, const N: usize> Of<&Triangle<T, N>> for [Vector<T, N>; 3] {
    fn of(triangle: &Triangle<T, N>) -> Self {
        (*triangle.abc).clone().to()
    }
}

impl<T: Copy, const N: usize> Of<Triangle<T, N>> for [Rc<Vector<T, N>>; 3] {
    fn of(triangle: Triangle<T, N>) -> Self {
        (*triangle.abc).clone().to()
    }
}

impl<T: Copy, const N: usize> Of<Rc<Triangle<T, N>>> for [Vector<T, N>; 3] {
    fn of(triangle: Rc<Triangle<T, N>>) -> Self {
        triangle.abc.clone().to()
    }
}

impl<T: Copy, const N: usize> Of<Rc<Triangle<T, N>>> for [Rc<Vector<T, N>>; 3] {
    fn of(triangle: Rc<Triangle<T, N>>) -> Self {
        triangle.abc.clone().to()
    }
}

impl<T: Copy + Number, const N: usize> Of<Vec<Triangle<T, N>>> for Vec<Rc<Vector<T, N>>> {
    fn of(triangles: Vec<Triangle<T, N>>) -> Self {
        triangles
            .into_iter()
            .map(|triangle| triangle.to::<[Rc<Vector<T, N>>; 3]>())
            .flatten()
            .fold(vec![], |mut acc, vector| {
                if !acc.contains(&vector) {
                    acc.push(vector);
                }
                acc
            })
    }
}

impl<T: Copy + Number, const N: usize> Of<Vec<Rc<Triangle<T, N>>>> for Vec<Rc<Vector<T, N>>> {
    fn of(triangles: Vec<Rc<Triangle<T, N>>>) -> Self {
        triangles
            .into_iter()
            .map(|triangle| triangle.to::<[Rc<Vector<T, N>>; 3]>())
            .flatten()
            .fold(vec![], |mut acc, vector| {
                if !acc.contains(&vector) {
                    acc.push(vector);
                }
                acc
            })
    }
}

impl<T: Copy + Number, const N: usize> Of<&Vec<Rc<Triangle<T, N>>>> for Vec<Rc<Vector<T, N>>> {
    fn of(triangles: &Vec<Rc<Triangle<T, N>>>) -> Self {
        (*triangles)
            .clone()
            .into_iter()
            .map(|triangle| triangle.to::<[Rc<Vector<T, N>>; 3]>())
            .flatten()
            .fold(vec![], |mut acc, vector| {
                if !acc.contains(&vector) {
                    acc.push(vector);
                }
                acc
            })
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
