#[cfg(test)]
mod tests;
mod impls;
mod ui;
use rand::Rng;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use std::cmp::Ordering;
use crate::{derive_deref, planet::shared::point::Points};
use crate::planet::shared::point::Point;
use super::shared::vector::{Vector, Vectors};



#[wasm_bindgen]
#[derive(Debug, Serialize, Clone)]
pub struct PointDistribution(Vectors);
derive_deref!(PointDistribution, 0, Vectors);


// getters setters
#[wasm_bindgen]
impl PointDistribution {
    #[wasm_bindgen(constructor)]
    pub fn from_points(points: JsValue) -> Self {
        let points: Points = from_value(points).unwrap();
        points.into()
    }

    pub fn set_random_points(point_count: usize, sizes: JsValue) -> Self {
        let sizes: Point = from_value(sizes).unwrap();
        let points: Points = (0..point_count)
            .map(|_| {
                let start = 0.0;
                let mut measures = [start; 2];
                for i in 0..2 {
                    measures[i] = if sizes[i] != start {
                        rand::thread_rng().gen_range(start..sizes[i])
                    } else {
                        start
                    }
                }
                measures
            })
            .collect();
        points.into()
    }
    
    #[wasm_bindgen(getter)]
    pub fn points(&self) -> JsValue {
        to_value(&self).unwrap()
    }
}

impl PointDistribution {
    pub fn get_max_point(&self) -> usize {
        self.sort_points_by_max()[0].0
    }
    pub fn get_max_point_by_axis(&self) -> usize {
        self.sort_points_by_max()[0].0
    }

    pub fn sort_points_by_max(&self) -> Vec<(usize, Vector)> {
        let mut new_points: Vec<(usize, Vector)> =
            self.iter().enumerate().map(|(i, &p)| (i, p)).collect();
        new_points.sort_by(|&(_, a), &(_, b)| {
            if *a > *b {
                return Ordering::Less;
            }
            if *a < *b {
                return Ordering::Greater;
            }
            Ordering::Equal
        });
        new_points
    }

    pub fn get_min_point(&self) -> usize {
        self.sort_points_by_min()[0].0
    }

    pub fn sort_points_by_min(&self) -> Vec<(usize, Vector)> {
        let mut new_points: Vec<(usize, Vector)> =
            self.iter().enumerate().map(|(i, &p)| (i, p)).collect();
        new_points.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        new_points
    }
}
