mod alien;
mod display;
mod has;
mod of_to;
mod ordering;
mod search;

use super::Triangle;
use crate::planet::point_distribution::PointDistribution;
use crate::planet::shared::traits::Svg;
use crate::planet::shared::vector::ui::line::Line;
use crate::planet::shared::vector::{Vector, VectorType};
use crate::traits::of_to::Of;
use crate::{
    planet::shared::{point::Point, vector::Number},
    traits::of_to::To,
};
use std::fmt::Debug;
use std::rc::Rc;
use svg::node::element::path::Parameters;
use svg::node::element::Polygon;
use svg::node::Value;
use svg::Document;

//Debug
impl<T: Number, const N: usize> Debug for Triangle<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vecs = self.to::<[Vector<T, N>; 3]>();
        f.debug_struct("Triangle")
            .field("a", &vecs[0])
            .field("b", &vecs[1])
            .field("c", &vecs[2])
            .finish()
    }
}