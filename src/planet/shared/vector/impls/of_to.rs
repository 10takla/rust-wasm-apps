use macros::of_to;

use crate::planet::shared::vector::VectorType;
use crate::traits::of_to::To;
use crate::{
    planet::shared::{
        point::Point,
        vector::{Number, Vector},
    },
    traits::of_to::Of,
};
use std::rc::Rc;

// from Point
#[of_to]
impl<T: Number, const N: usize> Of<Point<T, N>> for Vector<T, N> {
    fn of(point: Point<T, N>) -> Self {
        Self(point)
    }
}

impl<T: Number, const N: usize> Of<Vec<Point<T, N>>> for Vec<VectorType<T, N>> {
    fn of(points: Vec<Point<T, N>>) -> Self {
        points.into_iter().map(|point| point.to()).collect()
    }
}

// for Point
#[of_to]
impl<T: Clone, const N: usize> Of<Vector<T, N>> for Point<T, N> {
    fn of(value: Vector<T, N>) -> Self {
        value.0
    }
}

impl<T: Number, const N: usize, F> Of<Vec<F>> for Vec<Point<T, N>>
where
    Point<T, N>: Of<F>,
{
    fn of(points: Vec<F>) -> Self {
        points
            .into_iter()
            .map(|point| point.to::<Point<T, N>>())
            .collect()
    }
}

impl<'a, T, const N: usize> Of<&'a Vector<T, N>> for &'a Point<T, N> {
    fn of(value: &'a Vector<T, N>) -> Self {
        &value.0
    }
}
