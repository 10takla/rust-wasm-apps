mod circumference;

use crate::planet::point_distribution::Point;

use super::point_distribution::PointDistribution;

struct Triangulate {
    points: PointDistribution,
    triangles: Vec<[usize; 3]>,
}


impl Triangulate {
    pub fn get_delone(&self) -> Vec<[usize; 3]> {
        let points = self.points.iter().enumerate();
        
        // self.points.
        vec![]
    }
}