#[cfg(test)]
mod tests;

use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::Triangle;
use crate::planet::{point_distribution::PointDistribution, shared::vector::Vector};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test;

type Triangles = Vec<[usize; 3]>;

trait Dota<T: PartialEq> {
    fn every(&self, other: &Vec<T>) -> bool;
}

impl<T: PartialEq> Dota<T> for Vec<T> {
    fn every(&self, other: &Vec<T>) -> bool {
        other.iter().filter(|i| self.contains(i)).count() == other.len()
    }
}

// #[wasm_bindgen]
impl PointDistribution {
    pub fn get_triangles(&self) -> JsValue {
        let mut triangles: Triangles = vec![];
        let mut passed_tries: Triangles = vec![];
        let points: Vec<(usize, Vector)> = self.iter().enumerate().map(|(i, &p)| (i, p)).collect();
        for &(a_i, a) in points.iter() {
            for &(b_i, b) in points.iter() {
                if a_i == b_i {
                    continue;
                }
                for &(c_i, c) in points.iter() {
                    if [a_i, b_i].contains(&c_i)
                    // || PointDistribution::is_has_tries(&passed_tries, a_i, b_i, c_i)
                    {
                        continue;
                    } else {
                        passed_tries.push([a_i, b_i, c_i]);
                    }

                    let triangle = Triangle::from([a, b, c]);
                    let circle = triangle.get_circle();
                    let radius = circle.radius();

                    let mut is_triangle = true;
                    for &(d_i, d) in points.iter() {
                        if [a_i, b_i, c_i].contains(&d_i) {
                            continue;
                        }
                        let distance = (d - circle.center).radius();
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
        to_value(&triangles).unwrap()
    }
}

impl PointDistribution {
    pub fn is_has_tries(triangles: &Triangles, a_i: usize, b_i: usize, c_i: usize) -> bool {
        triangles
            .iter()
            .find(|&t| Vec::from(t).every(&vec![a_i, b_i, c_i]))
            .is_some()
    }
}

struct Delone {
    points: PointDistribution,
    triangles: Triangles,
}

impl Delone {
    pub fn tick(&mut self) {
        if self.triangles.len() == 0 {
            // self.points = self.points.sort_points_by_min();
        } else {
        }
    }
}

#[wasm_bindgen_test]
fn delone() {
    // let delone = Delone{
    //     points: PointDistribution::set_random_points(10, to_value(&[1.0, 1.0]).unwrap()),
    //     triangles: vec![]
    // };
    // delone.tick();
}
