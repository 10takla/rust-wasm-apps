use crate::planet::shared::vector::Vector;
use super::{Points, PointDistribution};

impl From<Points> for PointDistribution {
    fn from(points: Points) -> Self {
        Self(points.iter().map(|&p| Vector(p)).collect())
    }
}