mod old;
#[cfg(test)]
mod tests;
mod wasm;

use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::ui::hull::Hull;
use crate::planet::shared::vector::ui::line::LineType;
use crate::planet::shared::vector::VectorType;
use crate::{
    planet::{
        point_distribution::PointDistribution,
        shared::vector::{
            ui::line::{ui::angle::Angle, Line},
            Number, Vector,
        },
    },
    traits::of_to::{Of, To},
};
use std::{cmp::Ordering, ops::Deref, rc::Rc};

pub trait ConvexHull<T: Clone, const N: usize> {
    fn convex_hull(&mut self) -> Hull<T, N>;
}

impl<T: Number, const N: usize> ConvexHull<T, N> for PointDistribution<T, N> {
    fn convex_hull(&mut self) -> Hull<T, N> {

        if self.len() <= 2 {
            return Hull::of(self.0.clone());
        }

        let mut hull_edges: Vec<VectorType<T, N>> = vec![];

        let edge_point = self.get_min_point();
        hull_edges.push(edge_point.clone());

        self.sort_by_polar_angle(&edge_point);
        hull_edges.push(self[1].clone());

        loop {
            let finded_p = self.find_vector_by_angle(&hull_edges);
            if Rc::ptr_eq(&finded_p, &edge_point) {
                break Hull::of(&hull_edges);
            } else {
                hull_edges.push(finded_p);
            }
        }
    }
}

impl<T: Number, const N: usize> PointDistribution<T, N> {
    fn sort_by_polar_angle(&mut self, rel_vec: &VectorType<T, N>) {
        self.sort_by(|b, c| {
            if Rc::eq(c, rel_vec) {
                return Ordering::Greater;
            }
            let [b_angle, c_angle] = [b, c].map(|v| (**v - **rel_vec).atan());
            b_angle.partial_cmp(&c_angle).unwrap()
        });
    }

    fn find_vector_by_angle(&self, hull_edges: &Vec<VectorType<T, N>>) -> VectorType<T, N> {
        let vector = self
            .iter()
            .filter(|&vector| {
                if hull_edges.len() >= 3 {
                    Rc::ptr_eq(vector, &hull_edges[0])
                        || !hull_edges.iter().any(|e| Rc::ptr_eq(e, vector))
                } else {
                    !hull_edges.contains(&vector)
                }
            })
            .max_by(|&b, &c| {
                let [b_angle, c_angle] = [b, c].map(|v| self.get_angle(hull_edges, v));
                let t = b_angle.partial_cmp(&c_angle).unwrap();
                // dbg!((b, c, t));
                if let Ordering::Equal = t {
                    return Ordering::Greater;
                }
                t
            })
            .unwrap();
        (*vector).clone()
    }

    fn get_angle(&self, hull_edges: &Vec<VectorType<T, N>>, vector: &VectorType<T, N>) -> T {
        Angle::of([
            &hull_edges[hull_edges.len() - 2],
            &hull_edges[hull_edges.len() - 1],
            &vector,
        ])
        .get_angle()
    }
}
