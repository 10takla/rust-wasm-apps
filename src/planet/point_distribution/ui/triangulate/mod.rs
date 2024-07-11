pub mod delone;
pub mod spherical_triangulate;
#[cfg(test)]
mod tests;

use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::ui::hull::Hull;
use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::{Triangle, TriangleType};
use crate::planet::shared::vector::{Number, VectorType};
use crate::planet::{point_distribution::PointDistribution, shared::vector::Vector};
use crate::traits::of_to::{Of, To};
use std::rc::Rc;

type Triangles = Vec<[usize; 3]>;

trait Every<T: PartialEq> {
    fn every(&self, other: &Vec<T>) -> bool;
}

impl<T: PartialEq> Every<T> for Vec<T> {
    fn every(&self, other: &Vec<T>) -> bool {
        other.iter().filter(|i| self.contains(i)).count() == other.len()
    }
}

pub trait Triangulate {
    fn triangulate(&self) -> Vec<TriangleType>;
}

impl Triangulate for PointDistribution {
    fn triangulate(&self) -> Vec<TriangleType> {
        let mut triangles = vec![];
        for a in self.iter() {
            for b in self.iter() {
                if Rc::ptr_eq(a, b) {
                    continue;
                }
                for c in self.iter() {
                    if Rc::ptr_eq(a, c) || Rc::ptr_eq(b, c) {
                        continue;
                    }

                    let triangle = Triangle::of([a, b, c]);
                    let circle = triangle.get_circle();
                    let radius = circle.radius();

                    let mut id_delone = true;
                    for d in self.iter() {
                        if Rc::ptr_eq(a, d) || Rc::ptr_eq(b, d) || Rc::ptr_eq(c, d) {
                            continue;
                        }
                        let distance = (**d - *circle.center).radius();
                        if distance < radius {
                            id_delone = false;
                            break;
                        }
                    }
                    
                    if id_delone {
                        triangles.push(TriangleType::of([a, b, c]));
                    }
                }
            }
        }
        triangles
    }
}

// impl<T: Number, const N: usize> Triangulate for Hull<T, N> {
//     fn triangulate(&self) -> Triangles {
//         let mut vecs = self.to::<Vec<VectorType<T, N>>>();

//         let mut triangles = vec![];
//         let mut triangle = vec![];

//         let get_i = |offset| {
//             offset % vecs.len()
//         };

//         let (mut l, mut r) = (get_i(-1), get_i(1));

//         triangles.push(Triangle::of([vecs[l], vecs[r], vecs[0]]));

//         while l != r {
//             vecs[l]
//         }

//         vec![]
//     }
// }
