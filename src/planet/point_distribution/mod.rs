mod impls;
#[cfg(test)]
mod tests;
pub mod ui;
mod wasm;

use super::shared::point::{DefaultMeasureValue, DEFAULT_MEASURE};
use super::shared::vector::ui::line::ui::angle::ui::triangle::{Triangle, TriangleType};
use super::shared::vector::ui::line::LineType;
use super::shared::vector::{Number, Vector, Vectors};
use crate::planet::shared::point::Point;
use crate::planet::shared::point::Points;
use crate::planet::shared::traits::As;
use crate::planet::shared::vector::VectorType;
use crate::traits::as_prim::AsPrim;
use crate::traits::of_to::{Of, To};
use macros::Deref;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, WeightedIndex};
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug, Clone, Default, Deref)]
pub struct PointDistribution<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE>(
    pub Vectors<T, N>,
);

pub trait Random<T> {
    fn set_random_points(count: Option<usize>, density: Option<f64>, tri: &T) -> Self;
}

impl<T: Number, const N: usize> Random<TriangleType<T, N>> for PointDistribution<T, N> {
    fn set_random_points(
        count: Option<usize>,
        density: Option<f64>,
        tri: &TriangleType<T, N>,
    ) -> Self {
        let count = match (count, density) {
            (Some(count), None) => count,
            (None, Some(density)) => (tri.area() * density).round() as usize,
            _ => panic!("it should be or count =  None or density = None"),
        };

        PointDistribution::of(
            (0..count)
                .map(|_| {
                    let mut r1 = rand::thread_rng().gen_range(0.0..1.);
                    let mut r2 = rand::thread_rng().gen_range(0.0..1.);
                    if r1 + r2 > 1. {
                        r1 = 1. - r1;
                        r2 = 1. - r2;
                    }

                    let [a, b, c] = tri.to::<[Vector<T, N>; 3]>().map(|v| v.as_::<f64>());
                    ((((a - b) * r1) + ((c - b) * r2)) + b).as_::<T>()
                })
                .collect::<Vec<Vector<T, N>>>(),
        )
    }
}

impl<T: Number, const N: usize> Random<Vec<TriangleType<T, N>>> for PointDistribution<T, N> {
    fn set_random_points(
        count: Option<usize>,
        density: Option<f64>,
        tries: &Vec<TriangleType<T, N>>,
    ) -> Self {
        let count = match (count, density) {
            (Some(count), None) => count,
            (None, Some(density)) => {
                let sum = tries.into_iter().map(|tri| tri.area()).sum::<f64>();
                (sum * density).round() as usize
            }
            _ => panic!("it should be or count =  None or density = None"),
        };

        (0..count)
            .map(|_| {
                let tri = {
                    let weights = tries.iter().map(|tri| tri.area()).collect::<Vec<f64>>();
                    let dist = WeightedIndex::new(&weights).unwrap();
                    let mut rng = rand::thread_rng();
                    &tries[dist.sample(&mut rng)]
                };

                <PointDistribution<T, N> as Random<TriangleType<T, N>>>::set_random_points(
                    Some(1),
                    None,
                    tri,
                )[0]
                .clone()
            })
            .collect::<Vec<VectorType<T, N>>>()
            .to()
    }
}

impl<T: Number, const N: usize> Random<LineType<T, N>> for PointDistribution<T, N> {
    fn set_random_points(
        count: Option<usize>,
        density: Option<f64>,
        line: &LineType<T, N>,
    ) -> Self {
        let count = match (count, density) {
            (Some(count), None) => count,
            (None, Some(density)) => (line.length().as_::<f64>() * density).round() as usize,
            _ => panic!("it should be or count =  None or density = None"),
        };

        (0..count)
            .map(|_| {
                let mut r1 = rand::thread_rng().gen_range(0.0..1.);
                ((*line.a - *line.b).as_::<f64>() * r1 + line.b.as_::<f64>()).as_::<T>()
            })
            .collect::<Vec<Vector<T, N>>>()
            .to()
    }
}

impl<T: Number, const N: usize> Random<Vec<LineType<T, N>>> for PointDistribution<T, N> {
    fn set_random_points(
        count: Option<usize>,
        density: Option<f64>,
        lines: &Vec<LineType<T, N>>,
    ) -> Self {
        let count = match (count, density) {
            (Some(count), None) => count,
            (None, Some(density)) => {
                let sum = lines
                    .into_iter()
                    .map(|line| line.length())
                    .sum::<T>()
                    .as_::<f64>();
                (sum * density).round() as usize
            }
            _ => panic!("it should be or count =  None or density = None"),
        };

        (0..count)
            .map(|_| {
                let line = {
                    let weights = lines
                        .iter()
                        .map(|line| line.length().as_::<f64>())
                        .collect::<Vec<f64>>();
                    let dist = WeightedIndex::new(&weights).unwrap();
                    let mut rng = rand::thread_rng();
                    &lines[dist.sample(&mut rng)]
                };

                <PointDistribution<T, N> as Random<LineType<T, N>>>::set_random_points(
                    Some(1),
                    None,
                    line,
                )[0]
                .clone()
            })
            .collect::<Vec<VectorType<T, N>>>()
            .to()
    }
}

impl<T: Number, const N: usize> PointDistribution<T, N> {
    pub fn set_random_points(point_count: usize, sizes: [Point<T, N>; 2]) -> Self
    where
        T: SampleUniform,
    {
        (0..point_count)
            .into_iter()
            .map(|_| {
                (0..N)
                    .map(|i| {
                        if sizes[0][i] != sizes[1][i] {
                            rand::thread_rng().gen_range(sizes[0][i]..sizes[1][i])
                        } else {
                            sizes[0][i]
                        }
                    })
                    .collect::<Vec<T>>()
                    .try_into()
                    .unwrap()
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

    pub fn get_max_point(&mut self) -> VectorType<T, N> {
        self.sort_points_by_max();
        self[0].clone()
    }

    pub fn sort_points_by_max(&mut self) -> &mut Self {
        self.sort_by(|a, b| match a.partial_cmp(&b) {
            Some(ordering) => match ordering {
                Ordering::Greater => Ordering::Less,
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            },
            None => Ordering::Equal,
        });
        self
    }

    pub fn get_min_point(&mut self) -> VectorType<T, N> {
        self.sort_points_by_min();
        self[0].clone()
    }

    pub fn sort_points_by_min(&mut self) -> &mut Self {
        self.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self
    }
}
