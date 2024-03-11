mod impls;
#[cfg(test)]
mod tests;
pub mod ui;
use super::shared::point::DefaultMeasureValue;
use super::shared::vector::{Number, Vector, Vectors};
use crate::planet::shared::point::Point;
use crate::traits::as_::As;
use crate::traits::of_to::To;
use crate::{derive_deref, planet::shared::point::Points};
use rand::distributions::uniform::SampleUniform;
use rand::Rng;
use std::cmp::Ordering;

// #[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct PointDistribution<T = DefaultMeasureValue, const N: usize = 2>(Vectors<T, N>);
derive_deref!(PointDistribution<T, N>, 0, Vectors<T, N>, <T, const N: usize>);

impl<T: Number, const N: usize> PointDistribution<T, N> {
    pub fn set_random_points(point_count: usize, sizes: Point<T, N>) -> Self
    where
        T: SampleUniform,
    {
        (0..point_count)
            .into_iter()
            .map(|_| {
                let start = 0_i32.as_();
                let mut measures = [start; N];
                for i in 0..N {
                    measures[i] = if sizes[i] != start {
                        rand::thread_rng().gen_range(start..sizes[i])
                    } else {
                        start
                    }
                }
                measures
            })
            .collect::<Points<T, N>>()
            .to()
    }

    pub fn get_box_boundary(&self) -> [Vector<T, N>; 2] {
        [self.get_min_vector(), self.get_max_vector()]
    }

    pub fn get_min_vector(&self) -> Vector<T, N> {
        let point: Point<T, N> = (0..N)
            .into_iter()
            .map(|i| {
                self.iter()
                    .min_by(|&a, &b| a[i].partial_cmp(&b[i]).unwrap())
                    .unwrap()[i]
            })
            .collect::<Vec<T>>()
            .try_into()
            .unwrap();
        point.to()
    }

    pub fn get_max_vector(&self) -> Vector<T, N> {
        let point: Point<T, N> = (0..N)
            .into_iter()
            .map(|i| {
                self.iter()
                    .max_by(|&a, &b| a[i].partial_cmp(&b[i]).unwrap())
                    .unwrap()[i]
            })
            .collect::<Vec<T>>()
            .try_into()
            .unwrap();
        point.to()
    }

    pub fn get_max_point(&self) -> usize {
        self.sort_points_by_max()[0].0
    }
    pub fn get_max_point_by_axis(&self) -> usize {
        self.sort_points_by_max()[0].0
    }

    pub fn sort_points_by_max(&self) -> Vec<(usize, Vector<T, N>)> {
        let mut new_points: Vec<(usize, Vector<T, N>)> =
            self.iter().enumerate().map(|(i, p)| (i, **p)).collect();
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

    pub fn sort_points_by_min(&self) -> Vec<(usize, Vector<T, N>)> {
        let mut new_points: Vec<(usize, Vector<T, N>)> =
            self.iter().enumerate().map(|(i, p)| (i, **p)).collect();
        new_points.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        new_points
    }
}
