use std::collections::HashSet;
use std::hash::Hash;
use crate::planet::shared::{point::Point, vector::{ui::line::ui::angle::ui::triangle::Triangle, Number, Vector}};

use super::Rectangle;

impl<'a, T: PartialEq + Ord + Copy + Number + Hash> From<Rectangle<'a, T>> for [&'a Point<T>; 4] {
    fn from(value: Rectangle<'a, T>) -> Self {
        let (a_points, b_points): ([&Point<T>; 3], [&Point<T>; 3]) = (value.a.into(), value.b.into());
        let points: Vec<&Point<T>> = a_points.into_iter().chain(b_points.into_iter())
        .enumerate()
        .fold(vec![], |mut acc, (i, point)| {
            if !acc.contains(&point) {
                acc.push(point);
            }
            acc
        });
        points.try_into().unwrap()
    }
}


impl<'a, T: Copy> From<[&'a Vector<T>; 4]> for Rectangle<'a, T> {
    fn from(vecs: [&'a Vector<T>; 4]) -> Self {
        Rectangle{
            a: Triangle::from([vecs[0], vecs[1], vecs[2]]),
            b: Triangle::from([vecs[1], vecs[2], vecs[3]]),
        }
    }
}