use std::rc::Rc;

use super::{PointDistribution, Points, Vectors};
use crate::planet::shared::vector::{Number, Vector};

impl<F: Number, const N: usize> PointDistribution<F, N> {
    pub fn as_<I: Number>(&self) -> PointDistribution<I, N> {
        self.iter()
            .map(|vector| Rc::new(vector.as_()))
            .collect::<Vec<Rc<Vector<I, N>>>>()
            .into()
    }
}

impl<T, const N: usize> From<Vectors<T, N>> for PointDistribution<T, N> {
    fn from(points: Vectors<T, N>) -> Self {
        Self(points)
    }
}

impl<T, const N: usize> From<Vec<Vector<T, N>>> for PointDistribution<T, N> {
    fn from(points: Vec<Vector<T, N>>) -> Self {
        Self(points.into_iter().map(|point| Rc::new(point)).collect())
    }
}

impl<T: Number, const N: usize> From<Points<T, N>> for PointDistribution<T, N> {
    fn from(points: Points<T, N>) -> Self {
        Self(points.into_iter().map(|point| Rc::new(point.into())).collect())
    }
}

impl<T: Copy, const N: usize> From<PointDistribution<T, N>> for Points<T, N> {
    fn from(pd: PointDistribution<T, N>) -> Self {
        pd.0
            .into_iter()
            .map(|v| (*v).0)
            .collect()
    }
}
