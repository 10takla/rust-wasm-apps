use crate::planet::shared::{point::Point, vector::Vector};
use super::{PointDistribution, Points, Vectors};

impl<T> From<Vectors<T>> for PointDistribution<T> {
    fn from(points: Vectors<T>) -> Self {
        Self(points)
    }
}

impl<T> From<Points<T>> for PointDistribution<T> {
    fn from(points: Points<T>) -> Self {
        Self(points.into_iter().map(|p| Vector::from(p)).collect())
    }
}

impl<T: Clone> From<PointDistribution<T>> for Points<T> {
    fn from(pd: PointDistribution<T>) -> Self {
        <Vec<Vector<T>> as Clone>::clone(&pd).into_iter().map(|v| v.0).collect()
    }
}