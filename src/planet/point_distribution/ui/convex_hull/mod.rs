#[cfg(test)]
mod tests;

use crate::planet::{point_distribution::{PointDistribution, Points}, shared::vector::{ui::line::ui::angle::ui::triangle::Triangle, Vector}};
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

type HullEdges = Vec<usize>;

#[wasm_bindgen]
#[derive(Debug, Serialize)]
struct ConvexHull {
    point_distribution: PointDistribution,
    hull_edges: HullEdges,
}

//setters getters
#[wasm_bindgen]
impl ConvexHull {
    #[wasm_bindgen(constructor)]
    pub fn from_distribution(points: JsValue) -> Self {
        let points: Points = from_value(points).unwrap();
        Self {
            point_distribution: points.into(),
            hull_edges: Vec::new(),
        }
    }
}

#[wasm_bindgen]
impl ConvexHull {
    pub fn get_convex_hull(&mut self) -> JsValue {
        loop {
            let edge = self.tick();
            if let None = edge {
                break to_value(&self.hull_edges).unwrap();
            }
        }
    }

    pub fn tick(&mut self) -> Option<usize> {
        if self.point_distribution.len() == 0 || {
            let hull_edges_len = self.hull_edges.len();
            hull_edges_len > 1 && self.hull_edges[hull_edges_len - 1] == self.hull_edges[0]
        } {
            return None;
        }
        let finded_p = {
            if self.hull_edges.len() == 0 {
                self.point_distribution.get_min_point()
            } else {
                self.get_next_point()
            }
        };
        self.hull_edges.push(finded_p);
        Some(finded_p)
    }

    fn get_next_point(&self) -> usize {
        let hull_edges_len = self.hull_edges.len();
        let n_p_i = self.hull_edges[hull_edges_len - 1];
        let n_p = self.point_distribution[n_p_i];

        let points_width_ids = self.point_distribution.iter();

        let next_p = {
            if hull_edges_len >= 2 {
                points_width_ids
                    .enumerate()
                    .filter(|&(i, _)| {
                        if hull_edges_len >= 3 {
                            i == self.hull_edges[0] || !self.hull_edges.contains(&i)
                        } else {
                            !self.hull_edges.contains(&i)
                        }
                    })
                    .max_by(|&(b_i, _), &(c_i, _)| {
                        let (b_angle, c_angle) = (self.get_angle(b_i), self.get_angle(c_i));
                        b_angle.partial_cmp(&c_angle).unwrap()
                    })
                    .unwrap()
            } else {
                points_width_ids
                    .enumerate()
                    .filter(|&(i, _)| i != n_p_i)
                    .min_by(|&(_, &b), &(_, &c)| {
                        let (b_angle, c_angle) = (
                            (b - n_p).atan(),
                            (c - n_p).atan(),
                        );
                        b_angle.partial_cmp(&c_angle).unwrap()
                    })
                    .unwrap()
            }
        };
        next_p.0
    }

    fn get_angle(&self, p_i: usize) -> f64 {
        let triangle: Triangle = Triangle::from([
            &self.point_distribution[self.hull_edges[self.hull_edges.len() - 2]],
            &self.point_distribution[self.hull_edges[self.hull_edges.len() - 1]],
            &self.point_distribution[p_i],
        ]);
        triangle.abc.get_angle()
    }
}
