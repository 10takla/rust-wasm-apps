use crate::planet::shared::{point::Point, vector::Vector};

use super::Line;

impl From<[Vector; 2]> for Line {
    fn from(vecs: [Vector; 2]) -> Self {
        Self {
            a: vecs[0],
            b: vecs[1],
        }
    }
}

impl From<[Point; 2]> for Line {
    fn from(points: [Point; 2]) -> Self {
        Self {
            a: points[0].into(),
            b: points[1].into(),
        }
    }
}