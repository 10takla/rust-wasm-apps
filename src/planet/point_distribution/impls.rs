use crate::planet::shared::vector::Vector;
use super::{PointDistribution, Points, Vectors};

impl From<Vectors> for PointDistribution {
    fn from(points: Vectors) -> Self {
        Self(points)
    }
}

impl From<Points> for PointDistribution {
    fn from(points: Points) -> Self {
        Self(points.iter().map(|&p| Vector::from(p)).collect())
    }
}