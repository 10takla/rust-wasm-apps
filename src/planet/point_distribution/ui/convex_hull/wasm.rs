use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::planet::point_distribution::{
    ui::convex_hull::ConvexHull as CH, wasm::PointDistribution, PointDistribution as PD,
};
use crate::planet::shared::traits::Indices;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen]
struct ConvexHull;

#[wasm_bindgen]
impl ConvexHull {
    pub fn convex_hull(pd: &mut PointDistribution) -> JsValue {
        let pd = pd.get_pd();
        to_value(&pd.convex_hull().get_inds(pd.0.clone())).unwrap()
    }
}

#[wasm_bindgen_test]
fn convex_hull() {
    let mut pd = PointDistribution::set_pd(PD::set_random_points(1000, [[0.0; 3], [1.0; 3]]));
    ConvexHull::convex_hull(&mut pd);
}
