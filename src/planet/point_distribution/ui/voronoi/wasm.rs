use std::ops::DerefMut;

use super::SphericalVoronoi as SV;
use crate::planet::point_distribution::PointDistribution;
use crate::planet::shared::point::Point;
use crate::planet::shared::traits::Indices;
use crate::planet::{
    point_distribution::wasm::PointDistribution as PD, shared::vector::VectorType,
};
use crate::traits::of_to::To;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
#[derive(Clone)]
struct SphericalVoronoi;

#[wasm_bindgen]
impl SphericalVoronoi {
    pub fn generate(mut pd: PD) -> JsValue {
        let pd = pd.get_pd();
        let sv = SV::generate(pd.clone());
        let cells = sv
            .into_iter()
            .map(|hull| {
                hull.0
                    .to::<Vec<VectorType<f64, 3>>>()
                    .get_inds(pd.0.clone())
            })
            .collect::<Vec<Vec<usize>>>();

        to_value(&((*pd).clone().to::<Vec<Point<f64, 3>>>(), cells)).unwrap()
    }

    pub fn get_edges(mut pd: PD) -> JsValue {
        let pd = pd.get_pd();
        let edges = SV::get_edges(pd.clone());
        let wasm_edges = edges
            .into_iter()
            .map(|edge| edge.to())
            .collect::<Vec<[Point<f64, 3>; 2]>>();

        to_value(&wasm_edges).unwrap()
    }

    pub fn get_other(mut pd: PD) -> JsValue {
        let t = (*pd.get_pd()).clone();
        let sv = SV::new(t);

        to_value(&(
            sv.pd.to::<Vec<Point<f64, 3>>>(),
            sv.edges
                .into_iter()
                .map(|edge| edge.to())
                .collect::<Vec<[Point<f64, 3>; 2]>>(),
            sv.other.0,
            sv.other.1,
        ))
        .unwrap()
    }
}
