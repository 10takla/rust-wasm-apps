mod impls;
#[cfg(test)]
mod tests;
pub mod ui;
use super::shared::point::DefaultMeasureValue;
use super::shared::vector::{Number, Vector, Vectors};
use crate::planet::shared::point::Point;
use crate::traits::as_prim::AsPrim;
use crate::traits::of_to::To;
use crate::{derive_deref, planet::shared::point::Points};
use rand::distributions::uniform::SampleUniform;
use rand::Rng;
use std::cmp::Ordering;
use std::rc::Rc;

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

    pub fn get_max_point(&mut self) -> Rc<Vector<T, N>> {
        self.sort_points_by_max()[0].clone()
    }

    pub fn sort_points_by_max(&mut self) -> Self {
        let mut vecs = self.0.clone();
        vecs.sort_by(|a, b| match a.partial_cmp(&b) {
            Some(ordering) => match ordering {
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            },
            None => Ordering::Equal,
        });
        vecs.to()
    }

    pub fn get_min_point(&self) -> Rc<Vector<T, N>> {
        self.sort_points_by_min()[0].clone()
    }

    pub fn sort_points_by_min(&self) -> Self {
        let mut vecs = self.0.clone();
        vecs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        vecs.to()
    }
}
