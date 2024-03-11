#[cfg(test)]
mod tests;
pub mod delone;

use std::rc::Rc;
use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::Triangle;
use crate::planet::{point_distribution::PointDistribution, shared::vector::Vector};
use crate::traits::of_to::Of;

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
    fn is_has_tries(triangles: &Triangles, a_i: usize, b_i: usize, c_i: usize) -> bool;
    fn triangulate(&self) -> Triangles;
}


impl Triangulate for PointDistribution {
    fn triangulate(&self) -> Triangles {
        let mut triangles: Triangles = vec![];
        let mut passed_tries: Triangles = vec![];
        let points: Vec<(usize, &Rc<Vector>)> = self.iter().enumerate().map(|(i, p)| (i, p)).collect();
        for &(a_i, a) in points.iter() {
            for &(b_i, b) in points.iter() {
                if a_i == b_i {
                    continue;
                }
                for &(c_i, c) in points.iter() {
                    if [a_i, b_i].contains(&c_i) {
                        continue;
                    } else {
                        passed_tries.push([a_i, b_i, c_i]);
                    }

                    let triangle = Triangle::of([*a.clone(), *b.clone(), *c.clone()]);
                    let circle = triangle.get_circle();
                    let radius = circle.radius();

                    let mut is_triangle = true;
                    for &(d_i, d) in points.iter() {
                        if [a_i, b_i, c_i].contains(&d_i) {
                            continue;
                        }
                        let distance = (**d - *circle.center).radius();
                        if distance < radius {
                            is_triangle = false;
                            break;
                        }
                    }
                    if is_triangle {
                        // console_log!("{is_triangle} for {} {} {}", a_i, b_i, c_i);
                        triangles.push([a_i, b_i, c_i]);
                    }
                }
            }
        }
        triangles
    }

    fn is_has_tries(triangles: &Triangles, a_i: usize, b_i: usize, c_i: usize) -> bool {
        triangles
            .iter()
            .find(|&t| Vec::from(t).every(&vec![a_i, b_i, c_i]))
            .is_some()
    }
}
