use crate::planet::shared::{point::Point, vector::{Number, Vector}};

// from point
impl<T: Number, const N: usize> From<Point<T, N>> for Vector<T, N> {
    fn from(point: Point<T, N>) -> Self {
        Self(point)
    }
}

impl<T: Number, const N: usize> From<&Point<T, N>> for Vector<T, N> {
    fn from(point: &Point<T, N>) -> Self {
        Self(*point)
    }
}

// for point
impl<T, const N: usize> From<Vector<T, N>> for Point<T, N> {
    fn from(value: Vector<T, N>) -> Self {
        value.0
    }
}

impl<T: Copy, const N: usize> From<&Vector<T, N>> for Point<T, N> {
    fn from(value: &Vector<T, N>) -> Self {
        value.0
    }
}

impl<'a, T, const N: usize> From<&'a Vector<T, N>> for &'a Point<T, N> {
    fn from(value: &'a Vector<T, N>) -> Self {
        &value.0
    }
}