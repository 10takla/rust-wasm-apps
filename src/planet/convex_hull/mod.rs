#[cfg(test)]
mod tests;

use crate::planet::point_distribution::Point;
use crate::planet::vector::Vector;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

use super::point_distribution::PointDistribution;

type Points = Vec<Point>;

#[wasm_bindgen]
#[derive(Debug, Deserialize, Serialize)]
struct ConvexHull {
    point_distribution: PointDistribution,
    hull_edges: Vec<usize>,
}

#[wasm_bindgen]
impl ConvexHull {
    pub fn set_points(points: JsValue) -> ConvexHull {
        let post_points: Vec<[f64; 2]> = from_value(points).unwrap();
        let point_distribution: PointDistribution = post_points.into();
        ConvexHull {
            point_distribution,
            hull_edges: Vec::new(),
        }
    }

    pub fn set_random_ponts(point_count: usize, sizes: JsValue) -> ConvexHull {
        let post_sizes: Point = from_value(sizes).unwrap();
        let points: Vec<[f64; 2]> = (0..point_count)
            .map(|_| {
                let start = 0.0;
                let mut measures = [start; 2];
                for i in 0..2 {
                    measures[i] = if post_sizes[i] != start {
                        rand::thread_rng().gen_range(start..post_sizes[i])
                    } else {
                        start
                    }
                }
                measures
            })
            .collect();
        let point_distribution: PointDistribution = points.into();
        ConvexHull {
            point_distribution,
            hull_edges: Vec::new(),
        }
    }
    pub fn get_points(&self) -> JsValue {
        let points: Vec<[f64; 2]> = self
            .point_distribution
            .iter()
            .enumerate()
            .map(|(_, &p)| p)
            .collect();
        to_value(&points).unwrap()
    }

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
                            (Vector(b) - Vector(n_p)).tan(),
                            (Vector(c) - Vector(n_p)).tan(),
                        );
                        b_angle.partial_cmp(&c_angle).unwrap()
                    })
                    .unwrap()
            }
        };
        next_p.0
    }

    fn get_angle(&self, p_i: usize) -> f64 {
        let (a, b, c) = (
            Vector(
                self.point_distribution[self.hull_edges[self.hull_edges.len() - 2]],
            ),
            Vector(
                self.point_distribution[self.hull_edges[self.hull_edges.len() - 1]]
            ),
            Vector(self.point_distribution[p_i]),
        );
        let ab = a - b;
        let bc = c - b;

        let ab_length = ab.radius();
        let bc_length = bc.radius();

        let cos_theta = ab.scalar(&bc) / (ab_length * bc_length);

        cos_theta.acos()
    }
}
