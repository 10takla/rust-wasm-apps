use super::Rectangle;
use crate::planet::shared::{
    point::Point,
    vector::{ui::line::ui::angle::ui::triangle::Triangle, Number, Vector},
};
use std::{hash::Hash, rc::Rc};
use crate::traits::of_to::{Of, To};

// from Vector
impl<T: Copy, const N: usize> Of<[Rc<Vector<T, N>>; 4]> for Rectangle<T, N> {
    fn of(vecs: [Rc<Vector<T, N>>; 4]) -> Self {
        Rectangle {
            a: Rc::new(Triangle::of([
                vecs[0].clone(),
                vecs[1].clone(),
                vecs[2].clone(),
            ])),
            b: Rc::new(Triangle::of([
                vecs[1].clone(),
                vecs[2].clone(),
                vecs[3].clone(),
            ])),
        }
    }
}

impl<T: Copy, const N: usize> Of<[Vector<T, N>; 4]> for Rectangle<T, N> {
    fn of(vecs: [Vector<T, N>; 4]) -> Self {
        Rectangle {
            a: Rc::new(Triangle::of([
                Rc::new(vecs[0]),
                Rc::new(vecs[1]),
                Rc::new(vecs[2]),
            ])),
            b: Rc::new(Triangle::of([
                Rc::new(vecs[1]),
                Rc::new(vecs[2]),
                Rc::new(vecs[3]),
            ])),
        }
    }
}

// from Point
impl<T: Number, const N: usize> Of<[Point<T, N>; 4]> for Rectangle<T, N> {
    fn of(vecs: [Point<T, N>; 4]) -> Self {
        Rectangle {
            a: Rc::new(Triangle::of([vecs[0], vecs[1], vecs[2]])),
            b: Rc::new(Triangle::of([vecs[1], vecs[2], vecs[3]])),
        }
    }
}

// for Point
impl<T: PartialEq + Ord + Number + Hash, const N: usize> Of<Rectangle<T, N>>
    for [Point<T, N>; 4]
{
    fn of(rect: Rectangle<T, N>) -> Self {
        let [a_points, b_points]: [[Rc<Vector<T, N>>; 3]; 2] =
            [(*rect.a).clone().to(), (*rect.b).clone().to()];
        a_points
            .into_iter()
            .chain(b_points.into_iter())
            .fold(vec![], |mut acc, vector| {
                if !acc.contains(&vector) {
                    acc.push(vector);
                }
                acc
            })
            .into_iter()
            .map(|v| v.0)
            .collect::<Vec<Point<T, N>>>()
            .try_into()
            .unwrap()
    }
}

impl<T: PartialEq + Ord + Number + Hash, const N: usize> Of<&Rectangle<T, N>>
    for [Point<T, N>; 4]
{
    fn of(rect: &Rectangle<T, N>) -> Self {
        let [a_points, b_points]: [[Rc<Vector<T, N>>; 3]; 2] =
            [(*rect.a).clone().to(), (*rect.b).clone().to()];
        a_points
            .into_iter()
            .chain(b_points.into_iter())
            .fold(vec![], |mut acc, vector| {
                if !acc.contains(&vector) {
                    acc.push(vector);
                }
                acc
            })
            .into_iter()
            .map(|v| v.0)
            .collect::<Vec<Point<T, N>>>()
            .try_into()
            .unwrap()
    }
}