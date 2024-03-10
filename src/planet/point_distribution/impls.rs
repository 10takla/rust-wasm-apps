use std::rc::Rc;

use super::{PointDistribution, Points, Vectors};
use crate::{planet::shared::vector::{Number, Vector}, traits::of_to::To};
use crate::traits::of_to::Of;

impl<F: Number, const N: usize> PointDistribution<F, N> {
    pub fn as_<I: Number>(&self) -> PointDistribution<I, N> {
        self.iter()
            .map(|vector| Rc::new(vector.as_()))
            .collect::<Vec<Rc<Vector<I, N>>>>()
            .to()
    }
}

impl<T, const N: usize> Of<Vectors<T, N>> for PointDistribution<T, N> {
    fn of(points: Vectors<T, N>) -> Self {
        Self(points)
    }
}

impl<T, const N: usize> Of<Vec<Vector<T, N>>> for PointDistribution<T, N> {
    fn of(points: Vec<Vector<T, N>>) -> Self {
        Self(points.into_iter().map(|point| Rc::new(point)).collect())
    }
}

impl<T: Number, const N: usize> Of<Points<T, N>> for PointDistribution<T, N> {
    fn of(points: Points<T, N>) -> Self {
        Self(points.into_iter().map(|point| Rc::new(point.to())).collect())
    }
}

impl<T: Copy, const N: usize> Of<PointDistribution<T, N>> for Points<T, N> {
    fn of(pd: PointDistribution<T, N>) -> Self {
        pd.0
            .into_iter()
            .map(|v| (*v).0)
            .collect()
    }
}
