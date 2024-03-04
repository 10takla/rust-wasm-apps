use std::hash::Hash;
use crate::planet::shared::{point::Point, vector::{ui::line::ui::angle::ui::triangle::Triangle, Number, Vector}};

use super::Rectangle;

impl<T: PartialEq + Ord + Copy + Number + Hash> From<Rectangle<T>> for [Point<T>; 4] {
    fn from(value: Rectangle<T>) -> Self {
        let (a_points, b_points): ([Point<T>; 3], [Point<T>; 3]) = (value.a.into(), value.b.into());
        let points: Vec<Point<T>> = a_points.into_iter().chain(b_points.into_iter())
        .fold(vec![], |mut acc, point| {
            if !acc.contains(&point) {
                acc.push(point);
            }
            acc
        });
        points.try_into().unwrap()
    }
}


impl<T: Copy> From<[Vector<T>; 4]> for Rectangle<T> {
    fn from(vecs: [Vector<T>; 4]) -> Self {
        Rectangle{
            a: Triangle::from([vecs[0], vecs[1], vecs[2]]),
            b: Triangle::from([vecs[1], vecs[2], vecs[3]]),
        }
    }
}

impl<T: Copy> From<[Point<T>; 4]> for Rectangle<T> {
    fn from(vecs: [Point<T>; 4]) -> Self {
        Rectangle{
            a: Triangle::from([vecs[0], vecs[1], vecs[2]]),
            b: Triangle::from([vecs[1], vecs[2], vecs[3]]),
        }
    }
}