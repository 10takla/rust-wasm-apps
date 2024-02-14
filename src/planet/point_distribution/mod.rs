#[cfg(test)]
mod tests;

mod impls;
pub mod point;

use std::cmp::Ordering;

use point::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PointDistribution(pub Vec<(usize, Point)>);

impl PointDistribution {
    pub fn get_max_point(&self) -> usize {
        self.sort_points_by_max()[0].0
    }

    pub fn sort_points_by_max(&self) -> PointDistribution {
        let mut new_points = self.0.clone();
        new_points.sort_by(|(_, a), (_, b)| {
            if a > b {
                return Ordering::Less;
            }
            if a < b {
                return Ordering::Greater;
            }
            Ordering::Equal
        });
        PointDistribution(new_points)
    }

    pub fn get_min_point(&self) -> usize {
        self.sort_points_by_min()[0].0
    }

    pub fn sort_points_by_min(&self) -> PointDistribution {
        let mut new_points = self.0.clone();
        new_points.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        PointDistribution(new_points)
       
    }
}
