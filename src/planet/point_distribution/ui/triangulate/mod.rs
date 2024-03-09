#[cfg(test)]
mod tests;

use crate::planet::shared::point::{DefaultMeasureValue, Point};
use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::ui::rectangle::Rectangle;
use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::Triangle;
use crate::planet::shared::vector::ui::line::ui::angle::Angle;
use crate::planet::shared::vector::Number;
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
                    if [a_i, b_i].contains(&c_i) {
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

impl PointDistribution {
    pub fn triangulate(self) {
        let non_ind_vecs = |vecs: &Vec<(usize, Vector)>| -> Vec<Vector> {
            vecs.into_iter().map(|(_, vec)| *vec).collect()
        };
        let points = self.sort_points_by_min();
        let start_triangle = Triangle::from(non_ind_vecs(&points[0..=2].to_vec()));
        let next_vec = points[3].1;

        let tri_points: [Vector; 3] = start_triangle.into();
        let nearest_vec = tri_points
            .into_iter()
            .min_by(|&a, &b| {
                let [a, b] = [a, b]
                    .map(|v| (next_vec - v).radius().abs())
                    .try_into()
                    .unwrap();
                a.partial_cmp(&b).unwrap()
            })
            .unwrap();

        tri_points
            .into_iter()
            .filter(|&v| v != nearest_vec)
            .for_each(|oth_v| {
                let angle = Angle::from([next_vec, nearest_vec, oth_v]);
                (Vector::from(angle.ba.reverse()) * Vector::from(angle.ba.reverse()));
                // let angle_value = dbg!(angle.get_angle());
            })

        // Rectangle::from((start_triangle, next_point.1));
    }
}

#[test]
fn delone() {
    let pd = PointDistribution::from(vec![[0, 0], [0, 5], [-2, 3], [6, 3]]).as_::<f64>();

    // let tries = pd.traiangulate();
}
