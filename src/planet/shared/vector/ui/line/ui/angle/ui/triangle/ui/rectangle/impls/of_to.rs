use crate::{
    planet::shared::{
        point::Point,
        traits::Has,
        vector::{
            ui::line::ui::angle::ui::triangle::{ui::rectangle::Rectangle, Triangle},
            Number, Vector,
        },
    },
    traits::of_to::{Of, To},
};
use std::rc::Rc;
use crate::planet::shared::vector::VectorType;

// from Triangle
impl<T: Number, const N: usize> Of<[Rc<Triangle<T, N>>; 2]> for Rectangle<T, N> {
    fn of(tries: [Rc<Triangle<T, N>>; 2]) -> Self {
        Self {
            a: tries[0].clone(),
            b: tries[1].clone(),
        }
    }
}

impl<T: Number, const N: usize> Of<[&Rc<Triangle<T, N>>; 2]> for Rectangle<T, N> {
    fn of(tries: [&Rc<Triangle<T, N>>; 2]) -> Self {
        tries.map(|triangle| (*triangle).clone()).to()
    }
}

impl<T: Number, const N: usize> Of<[Triangle<T, N>; 2]> for Rectangle<T, N> {
    fn of(tries: [Triangle<T, N>; 2]) -> Self {
        tries.map(|triangle| Rc::new(triangle)).to()
    }
}

impl<T: Number, const N: usize> Of<[&Triangle<T, N>; 2]> for Rectangle<T, N> {
    fn of(tries: [&Triangle<T, N>; 2]) -> Self {
        tries.map(|triangle| (*triangle).clone()).to()
    }
}

// from Vector
impl<T: Number, const N: usize> Of<[VectorType<T, N>; 4]> for Rectangle<T, N> {
    fn of(vecs: [VectorType<T, N>; 4]) -> Self {
        Rectangle {
            a: Rc::new(Triangle::of([&vecs[0], &vecs[1], &vecs[2]])),
            b: Rc::new(Triangle::of([&vecs[1], &vecs[2], &vecs[3]])),
        }
    }
}

impl<T: Number, const N: usize> Of<[&VectorType<T, N>; 4]> for Rectangle<T, N> {
    fn of(vecs: [&VectorType<T, N>; 4]) -> Self {
        vecs.map(|vector| (*vector).clone()).to()
    }
}

impl<T: Number, const N: usize> Of<[Vector<T, N>; 4]> for Rectangle<T, N> {
    fn of(vecs: [Vector<T, N>; 4]) -> Self {
        vecs.map(|vec| Rc::new(vec)).to()
    }
}

// from Point
impl<T: Number, const N: usize> Of<[Point<T, N>; 4]> for Rectangle<T, N> {
    fn of(points: [Point<T, N>; 4]) -> Self {
        let vecs: [VectorType<T, N>; 4] = points.map(|p| Rc::new(Vector::of(p)));
        Rectangle {
            a: Rc::new(Triangle::of([&vecs[0], &vecs[1], &vecs[2]])),
            b: Rc::new(Triangle::of([&vecs[1], &vecs[2], &vecs[3]])),
        }
    }
}

// for Triangle
impl<T: Number, const N: usize> Of<Rectangle<T, N>> for [Rc<Triangle<T, N>>; 2] {
    fn of(rect: Rectangle<T, N>) -> Self {
        [rect.a, rect.b]
    }
}

impl<T: Number, const N: usize> Of<Vec<Rectangle<T, N>>> for Vec<Rc<Triangle<T, N>>> {
    fn of(rects: Vec<Rectangle<T, N>>) -> Self {
        rects
            .into_iter()
            .map(|rect| rect.to::<[Rc<Triangle<T, N>>; 2]>())
            .flatten()
            .fold(vec![], |mut acc, curr| {
                if !acc.contains(&curr) {
                    acc.push(curr);
                }
                acc
            })
    }
}

// for Vector
impl<T: Number, const N: usize> Of<Rectangle<T, N>> for [VectorType<T, N>; 4] {
    fn of(rect: Rectangle<T, N>) -> Self {
        let [a_points, b_points]: [[VectorType<T, N>; 3]; 2] =
            [rect.a.clone().to(), rect.b.clone().to()];
        a_points
            .into_iter()
            .chain(b_points.into_iter())
            .fold(
                vec![],
                |mut acc: Vec<VectorType<T, N>>, vector: VectorType<T, N>| {
                    if !acc.has(&vector) {
                        acc.push(vector);
                    }
                    acc
                },
            )
            .try_into()
            .unwrap()
    }
}

impl<T: Number, const N: usize> Of<Rectangle<T, N>> for [Vector<T, N>; 4] {
    fn of(rect: Rectangle<T, N>) -> Self {
        rect.to::<[VectorType<T, N>; 4]>().map(|vector| *vector)
    }
}

// for Point
impl<T: Number, const N: usize> Of<Rectangle<T, N>> for [Point<T, N>; 4] {
    fn of(rect: Rectangle<T, N>) -> Self {
        let [a_points, b_points]: [[VectorType<T, N>; 3]; 2] =
            [rect.a.clone().to(), rect.b.clone().to()];
        a_points
            .into_iter()
            .chain(b_points.into_iter())
            .fold(
                vec![],
                |mut acc: Vec<VectorType<T, N>>, vector: VectorType<T, N>| {
                    if !acc.clone().iter().any(|v| Rc::ptr_eq(v, &vector)) {
                        acc.push(vector);
                    }
                    acc
                },
            )
            .into_iter()
            .map(|v| v.0)
            .collect::<Vec<Point<T, N>>>()
            .try_into()
            .unwrap()
    }
}

impl<T: Number, const N: usize> Of<&Rectangle<T, N>> for [Point<T, N>; 4] {
    fn of(rect: &Rectangle<T, N>) -> Self {
        (*rect).clone().to()
    }
}

impl<T: Number, const N: usize> Of<Rc<Rectangle<T, N>>> for [Point<T, N>; 4] {
    fn of(rect: Rc<Rectangle<T, N>>) -> Self {
        (*rect).clone().to()
    }
}
