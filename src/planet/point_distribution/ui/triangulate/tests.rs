use serde_wasm_bindgen::from_value;
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

use crate::{
    planet::{
        point_distribution::{
            ui::triangulate::{delone::Delone, Triangles, Triangulate},
            PointDistribution,
        },
        shared::traits::As,
    },
    traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
    },
};

#[test]
fn is_has_tries() {
    let triangles = vec![[0, 1, 2]];
    assert!(PointDistribution::is_has_tries(&triangles, 0, 1, 2));

    let triangles = vec![[1, 2, 0]];
    assert!(PointDistribution::is_has_tries(&triangles, 0, 1, 2));

    let triangles = vec![[2, 0, 1]];
    assert!(PointDistribution::is_has_tries(&triangles, 0, 1, 2));

    let triangles = vec![[2, 0, 3]];
    assert!(!PointDistribution::is_has_tries(&triangles, 0, 1, 2));
}

fn delone() {
    let pd = PointDistribution::of(vec![[0, 0], [0, 5], [-2, 3], [6, 3]]).as_::<f64>();
    // let tries = pd.delone();
}
