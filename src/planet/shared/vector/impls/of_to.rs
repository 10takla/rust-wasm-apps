use std::rc::Rc;

use crate::{planet::shared::{
    point::Point,
    vector::{Number, Vector},
}, traits::of_to::Of};

// from point
impl<T: Number, const N: usize> Of<Point<T, N>> for Vector<T, N> {
    fn of(point: Point<T, N>) -> Self {
        Self(point)
    }
}

impl<T: Number, const N: usize> Of<&Point<T, N>> for Vector<T, N> {
    fn of(point: &Point<T, N>) -> Self {
        Self(*point)
    }
}

impl<T: Number, const N: usize> Of<Point<T, N>> for Rc<Vector<T, N>> {
    fn of(point: Point<T, N>) -> Self {
        Rc::new(Vector(point))
    }
}
// for point
impl<T, const N: usize> Of<Vector<T, N>> for Point<T, N> {
    fn of(value: Vector<T, N>) -> Self {
        value.0
    }
}

impl<T: Copy, const N: usize> Of<&Vector<T, N>> for Point<T, N> {
    fn of(value: &Vector<T, N>) -> Self {
        value.0
    }
}

impl<'a, T, const N: usize> Of<&'a Vector<T, N>> for &'a Point<T, N> {
    fn of(value: &'a Vector<T, N>) -> Self {
        &value.0
    }
}
