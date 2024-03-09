use std::{hash::Hash, rc::Rc};
use crate::planet::shared::{point::Point, vector::{ui::line::ui::angle::ui::triangle::Triangle, Number, Vector}};

use super::Rectangle;

impl<T: PartialEq + Ord + Number + Hash, const N: usize> From<Rectangle<T, N>> for [Point<T, N>; 4] {
    fn from(value: Rectangle<T, N>) -> Self {
        let (a_points, b_points): ([Point<T, N>; 3], [Point<T, N>; 3]) = (value.a.into(), value.b.into());
        let points: Vec<Point<T, N>> = a_points.into_iter().chain(b_points.into_iter())
        .fold(vec![], |mut acc, point| {
            if !acc.contains(&point) {
                acc.push(point);
            }
            acc
        });
        points.try_into().unwrap()
    }
}

impl<T: PartialEq + Ord + Number + Hash, const N: usize> From<&Rectangle<T, N>> for [Point<T, N>; 4] {
    fn from(value: &Rectangle<T, N>) -> Self {
        let (a_points, b_points): ([Point<T, N>; 3], [Point<T, N>; 3]) = (value.a.clone().into(), value.b.clone().into());
        let points: Vec<Point<T, N>> = a_points.into_iter().chain(b_points.into_iter())
        .fold(vec![], |mut acc, point| {
            if !acc.contains(&point) {
                acc.push(point);
            }
            acc
        });
        points.try_into().unwrap()
    }
}


impl<T: Copy, const N: usize> From<[Rc<Vector<T, N>>; 4]> for Rectangle<T, N> {
    fn from(vecs: [Rc<Vector<T, N>>; 4]) -> Self {
        Rectangle{
            a: Triangle::from([vecs[0].clone(), vecs[1].clone(), vecs[2].clone()]),
            b: Triangle::from([vecs[1].clone(), vecs[2].clone(), vecs[3].clone()]),
        }
    }
}

impl<T: Copy, const N: usize> From<[Vector<T, N>; 4]> for Rectangle<T, N> {
    fn from(vecs: [Vector<T, N>; 4]) -> Self {
        Rectangle{
            a: Triangle::from([Rc::new(vecs[0]), Rc::new(vecs[1]), Rc::new(vecs[2])]),
            b: Triangle::from([Rc::new(vecs[1]), Rc::new(vecs[2]), Rc::new(vecs[3])]),
        }
    }
}

impl<T: Number, const N: usize> From<[Point<T, N>; 4]> for Rectangle<T, N> {
    fn from(vecs: [Point<T, N>; 4]) -> Self {
        Rectangle{
            a: Triangle::from([vecs[0], vecs[1], vecs[2]]),
            b: Triangle::from([vecs[1], vecs[2], vecs[3]]),
        }
    }
}