use crate::planet::shared::{point::Point, vector::Vector};
use super::Triangle;

impl From<[Point; 3]> for Triangle {
    fn from(points: [Point; 3]) -> Self {
        let mut vecs = [Vector([0.0; 2]); 3];
        for (i, &p) in points.iter().enumerate() {
            vecs[i] = p.into();
        }
        Self{a: vecs[0], b: vecs[1], c: vecs[2]}
    }
}

impl From<[Vector; 3]> for Triangle {
    fn from(points: [Vector; 3]) -> Self {
        Self{a: points[0], b: points[1], c: points[2]}
    }
}