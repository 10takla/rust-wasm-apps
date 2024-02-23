mod iterator;
mod display;

use crate::planet::shared::{point::Point, vector::{ui::line::{ui::angle::Angle, Line}, Vector}};
use super::Triangle;

impl From<[Angle; 3]> for Triangle {
    fn from(angles: [Angle; 3]) -> Self {
        Self {
            abc: angles[0],
            acb: angles[1],
            bac: angles[2],
        }
    }
}

impl From<[Line; 3]> for Triangle {
    fn from(points: [Line; 3]) -> Self {
        let (ab, bc, ac) = (points[0], points[1], points[2]);
        Self {
            abc: [ab, bc].into(),
            acb: [ac, bc].into(),
            bac: [ab, ac].into(),
        }
    }
}

impl From<[Vector; 3]> for Triangle {
    fn from(vecs: [Vector; 3]) -> Self {
        Self {
            abc: [vecs[0], vecs[1], vecs[2]].into(),
            acb: [vecs[0], vecs[2], vecs[1]].into(),
            bac: [vecs[1], vecs[0], vecs[2]].into(),
        }
    }
}

impl From<[Point; 3]> for Triangle {
    fn from(points: [Point; 3]) -> Self {
        Self {
            abc: [points[0], points[1], points[2]].into(),
            acb: [points[0], points[2], points[1]].into(),
            bac: [points[1], points[0], points[2]].into(),
        }
    }
}