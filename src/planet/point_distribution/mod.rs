#[cfg(test)]
mod tests;

mod impls;

use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

pub type DefaultMeasureValue = f64;
pub type Point = [DefaultMeasureValue; 2];


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PointDistribution(pub Vec<Point>);

impl PointDistribution {
    pub fn get_max_point(&self) -> usize {
        self.sort_points_by_max()[0].0
    }

    pub fn sort_points_by_max(&self) -> Vec<(usize, Point)> {
        let mut new_points: Vec<(usize, Point)> = self.iter().enumerate().map(|(i, &p)| (i, p)).collect();
        new_points.sort_by(|(_, a), (_, b)| {
            if a > b {
                return Ordering::Less;
            }
            if a < b {
                return Ordering::Greater;
            }
            Ordering::Equal
        });
        new_points
    }

    pub fn get_min_point(&self) -> usize {
        self.sort_points_by_min()[0].0
    }

    pub fn sort_points_by_min(&self) -> Vec<(usize, Point)> {
        let mut new_points: Vec<(usize, Point)> = self.iter().enumerate().map(|(i, &p)| (i, p)).collect();
        new_points.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        new_points
    }
}
