use crate::planet::shared::{point::Point, vector::{ui::line::Line, Vector}};

use super::Angle;

impl From<[Line; 2]> for Angle {
    fn from(lines: [Line; 2]) -> Self {
        if lines[0].a != lines[1].a {
            panic!("Lines not valid for Angle\n{:?}", dbg!(lines));
        }
        Self {
            ab: lines[0],
            bc: lines[1],
        }
    }
}

impl From<[Vector; 3]> for Angle {
    fn from(vecs: [Vector; 3]) -> Self {
        let (a, b, c) = (vecs[0], vecs[1], vecs[2]);
        Self {
            ab: [b, a].into(),
            bc: [b, c].into(),
        }
    }
}

impl From<[Point; 3]> for Angle {
    fn from(points: [Point; 3]) -> Self {
        let (a, b, c) = (points[0], points[1], points[2]);
        Self {
            ab: [b, a].into(),
            bc: [b, c].into(),
        }
    }
}