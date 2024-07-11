use crate::{
    planet::shared::{point::Point, traits::Normalize, vector::Vector},
    traits::of_to::{Of, To},
};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(Clone)]
#[wasm_bindgen]
pub struct PointDistribution(super::PointDistribution<f64, 3>);

impl PointDistribution {
    pub fn get_pd(&mut self) -> &mut super::PointDistribution<f64, 3> {
        &mut self.0
    }

    pub fn set_pd(pd: super::PointDistribution<f64, 3>) -> PointDistribution {
        Self(pd)
    }
}

#[wasm_bindgen]
impl PointDistribution {
    pub fn set_random_points(point_count: usize, sizes: JsValue) -> Self {
        let sizes = from_value::<[Point<f64, 3>; 2]>(sizes).unwrap();
        PointDistribution(super::PointDistribution::set_random_points(
            point_count,
            sizes,
        ))
    }
    pub fn set_spherical_random_points(point_count: usize, radius: f64) -> Self {
        let mut pd = super::PointDistribution::set_random_points(point_count, [[-1.;3], [1.;3]]);
        pd.normalize();
        Self(
            pd * radius
        )
    }

    pub fn set_points(points: JsValue) -> Self {
        PointDistribution(super::PointDistribution::of(
            from_value::<Vec<Point<f64, 3>>>(points).unwrap(),
        ))
    }

    pub fn normalize(&mut self) -> Self {
        self.0.normalize();
        self.clone()
    }

    #[wasm_bindgen(method, getter)]
    pub fn points(&self) -> JsValue {
        to_value(&self.0.clone().to::<Vec<Point<f64, 3>>>()).unwrap()
    }
}
