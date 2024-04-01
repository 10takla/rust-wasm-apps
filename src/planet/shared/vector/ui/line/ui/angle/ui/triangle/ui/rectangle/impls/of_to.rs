use std::rc::Rc;
use crate::{planet::shared::{point::Point, vector::{ui::line::ui::angle::ui::triangle::{ui::rectangle::Rectangle, Triangle}, Number, Vector}}, traits::of_to::{Of, To}};


// from Triangle
impl<T: Number, const N: usize> Of<[Triangle<T, N>; 2]> for Rectangle<T, N> {
    fn of(tries: [Triangle<T, N>; 2]) -> Self {
        Self {
            a: Rc::new(tries[0].clone()),
            b: Rc::new(tries[1].clone()),
        }
    }
}

impl<T: Number, const N: usize> Of<[&Triangle<T, N>; 2]> for Rectangle<T, N> {
    fn of(tries: [&Triangle<T, N>; 2]) -> Self {
        Self {
            a: Rc::new(tries[0].clone()),
            b: Rc::new(tries[1].clone()),
        }
    }
}

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
        Self {
            a: tries[0].clone(),
            b: tries[1].clone(),
        }
    }
}

// from Vector
impl<T: Number, const N: usize> Of<[Vector<T, N>; 4]> for Rectangle<T, N> {
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

impl<T: Number, const N: usize> Of<[Rc<Vector<T, N>>; 4]> for Rectangle<T, N> {
    fn of(vecs: [Rc<Vector<T, N>>; 4]) -> Self {
        Rectangle {
            a: Rc::new(Triangle::of([
                &vecs[0],
                &vecs[1],
                &vecs[2],
            ])),
            b: Rc::new(Triangle::of([
                &vecs[1],
                &vecs[2],
                &vecs[3],
            ])),
        }
    }
}

impl<T: Number, const N: usize> Of<[&Rc<Vector<T, N>>; 4]> for Rectangle<T, N> {
    fn of(vecs: [&Rc<Vector<T, N>>; 4]) -> Self {
        Rectangle {
            a: Rc::new(Triangle::of([
                vecs[0],
                vecs[1],
                vecs[2],
            ])),
            b: Rc::new(Triangle::of([
                vecs[1],
                vecs[2],
                vecs[3],
            ])),
        }
    }
}

// from Point
impl<T: Number, const N: usize> Of<[Point<T, N>; 4]> for Rectangle<T, N> {
    fn of(points: [Point<T, N>; 4]) -> Self {
        let vecs: [Rc<Vector<T, N>>; 4] = points.map(|p| Rc::new(Vector::of(p)));
        Rectangle {
            a: Rc::new(Triangle::of([
                &vecs[0],
                &vecs[1],
                &vecs[2],
            ])),
            b: Rc::new(Triangle::of([
                &vecs[1],
                &vecs[2],
                &vecs[3],
            ])),
        }
    }
}

// for Triangle
impl<T: Number, const N:usize> Of<Rectangle<T, N>> for [Rc<Triangle<T, N>>; 2] {
    fn of(rect: Rectangle<T, N>) -> Self {
        [rect.a, rect.b]
    }
}

// for Vector
impl<T: PartialEq + Ord + Number, const N: usize> Of<Rectangle<T, N>> for [Vector<T, N>; 4] {
    fn of(rect: Rectangle<T, N>) -> Self {
        let [a_points, b_points]: [[Rc<Vector<T, N>>; 3]; 2] =
            [rect.a.clone().to(), rect.b.clone().to()];
        // dbg!(a_points.clone(), b_points.clone());
        // dbg!(a_points.clone().iter().map(|x| x.as_ptr()).collect::<Vec<*const T>>(), b_points.clone().iter().map(|x| x.as_ptr()).collect::<Vec<*const T>>());
        a_points
            .into_iter()
            .chain(b_points.into_iter())
            .fold(
                vec![],
                |mut acc: Vec<Rc<Vector<T, N>>>, vector: Rc<Vector<T, N>>| {
                    if !acc.clone().iter().any(|v| Rc::ptr_eq(v, &vector)) {
                        acc.push(vector);
                    }
                    acc
                },
            )
            .into_iter()
            .map(|v| *v)
            .collect::<Vec<Vector<T, N>>>()
            .try_into()
            .unwrap()
    }
}

impl<T: Number, const N: usize> Of<Rectangle<T, N>> for [Rc<Vector<T, N>>; 4] {
    fn of(rect: Rectangle<T, N>) -> Self {
        let [a_points, b_points]: [[Rc<Vector<T, N>>; 3]; 2] =
            [rect.a.clone().to(), rect.b.clone().to()];

        a_points
            .into_iter()
            .chain(b_points.into_iter())
            .fold(vec![], |mut acc, vector| {
                if !acc.clone().iter().any(|v| Rc::ptr_eq(v, &vector)) {
                    acc.push(vector);
                }
                acc
            })
            .try_into()
            .unwrap()
    }
}

// for Point
impl<T: Number, const N: usize> Of<Rectangle<T, N>> for [Point<T, N>; 4] {
    fn of(rect: Rectangle<T, N>) -> Self {
        let [a_points, b_points]: [[Rc<Vector<T, N>>; 3]; 2] =
            [rect.a.clone().to(), rect.b.clone().to()];
        a_points
            .into_iter()
            .chain(b_points.into_iter())
            .fold(
                vec![],
                |mut acc: Vec<Rc<Vector<T, N>>>, vector: Rc<Vector<T, N>>| {
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
        let [a_points, b_points]: [[Rc<Vector<T, N>>; 3]; 2] =
            [rect.a.clone().to(), rect.b.clone().to()];
        a_points
            .into_iter()
            .chain(b_points.into_iter())
            .fold(
                vec![],
                |mut acc: Vec<Rc<Vector<T, N>>>, vector: Rc<Vector<T, N>>| {
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

impl<T: Number, const N: usize> Of<Rc<Rectangle<T, N>>> for [Point<T, N>; 4] {
    fn of(rect: Rc<Rectangle<T, N>>) -> Self {
        let [a_points, b_points]: [[Rc<Vector<T, N>>; 3]; 2] =
            [rect.a.clone().to(), rect.b.clone().to()];
        a_points
            .into_iter()
            .chain(b_points.into_iter())
            .fold(
                vec![],
                |mut acc: Vec<Rc<Vector<T, N>>>, vector: Rc<Vector<T, N>>| {
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