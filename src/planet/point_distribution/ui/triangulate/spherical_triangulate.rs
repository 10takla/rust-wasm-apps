use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

use super::Triangles;
use crate::planet::point_distribution::ui::triangulate::Triangulate;
use crate::planet::shared::point::Point;
use crate::planet::shared::traits::Normalize;
use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::{Triangle, TriangleType};
use crate::traits::of_to::Of;
use crate::{
    planet::{
        point_distribution::PointDistribution,
        shared::vector::{Number, SphericalProjection, Vector, VectorType},
    },
    traits::of_to::To,
};
use std::collections::HashMap;
use std::rc::Rc;

#[wasm_bindgen]
pub fn spherical_triangulate(points: JsValue) -> JsValue {
    let pd = from_value::<Vec<Point<f64, 3>>>(points)
        .unwrap()
        .to::<PointDistribution<f64, 3>>();
    let tries = pd.real();
    to_value(&[2]).unwrap()
}

pub trait SphericalTriangulate {
    fn triangulate(&self) -> Vec<Triangle<f64, 3>>;
    fn real(&self) -> Vec<TriangleType<f64, 3>>;
    fn set_spherical_random_points(count: usize) -> PointDistribution<f64, 3>;
}

impl SphericalTriangulate for PointDistribution<f64, 3> {
    fn set_spherical_random_points(count: usize) -> Self {
        let mut pd = PointDistribution::set_random_points(count, [[-1.; 3], [1.; 3]]);
        pd.normalize();
        pd
    }
    fn real(&self) -> Vec<TriangleType<f64, 3>> {
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

    fn triangulate(&self) -> Vec<Triangle<f64, 3>> {
        let mut tmp = HashMap::new();
        let pd = self
            .iter()
            .map(|point| {
                let v = <Vector<f64, 3> as SphericalProjection>::projection(&point);
                let v = Rc::new(v);
                tmp.insert(v.as_ptr(), point);
                v
            })
            .collect::<Vec<VectorType>>()
            .to::<PointDistribution>();

        let tries = pd.triangulate();

        tries
            .into_iter()
            .map(|tri| {
                Triangle::of({
                    let t: [VectorType<f64, 3>; 3] = tri
                        .to::<[VectorType; 3]>()
                        .into_iter()
                        .map(|vector| {
                            let v = tmp
                                .get(
                                    &pd.iter()
                                        .find(|&v| Rc::ptr_eq(v, &vector))
                                        .unwrap()
                                        .as_ptr(),
                                )
                                .unwrap();
                            (*v).clone()
                        })
                        .collect::<Vec<VectorType<f64, 3>>>()
                        .try_into()
                        .unwrap();
                    t
                })
            })
            .collect::<Vec<Triangle<f64, 3>>>()
    }
}
