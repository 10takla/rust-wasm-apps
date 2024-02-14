use std::ops::{Deref, DerefMut};
use super::{Point, PointDistribution};


impl Deref for PointDistribution {
    type Target = Vec<Point>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for PointDistribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Point>> for PointDistribution {
    fn from(points: Vec<Point>) -> Self {
        Self(points)
    }
}