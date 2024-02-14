use std::ops::{Deref, DerefMut};
use super::{point::Point, PointDistribution};


impl Deref for PointDistribution {
    type Target = Vec<(usize, Point)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for PointDistribution {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<[f64; 2]>> for PointDistribution {
    fn from(points: Vec<[f64; 2]>) -> Self {
        Self(points.into_iter().map(Point::from).enumerate().collect())
    }
}