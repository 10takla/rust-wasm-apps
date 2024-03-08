use crate::planet::shared::vector::{Number, Vector};
use super::{PointDistribution, Points, Vectors};

impl<T, const N: usize> From<Vectors<T, N>> for PointDistribution<T, N> {
    fn from(points: Vectors<T, N>) -> Self {
        Self(points)
    }
}

impl<T: Number, const N: usize> From<Points<T, N>> for PointDistribution<T, N> {
    fn from(points: Points<T, N>) -> Self {
        Self(points.into_iter().map(|point| point.into()).collect())
    }
}

impl<T: Clone, const N: usize> From<PointDistribution<T, N>> for Points<T, N> {
    fn from(pd: PointDistribution<T, N>) -> Self {
        <Vec<Vector<T, N>> as Clone>::clone(&pd).into_iter().map(|v| v.0).collect()
    }
}