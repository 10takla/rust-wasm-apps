use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

use crate::planet::point_distribution::{ui::triangulate::Triangles, PointDistribution};

#[wasm_bindgen_test]
fn get_delone() {
    let points = PointDistribution::set_random_points(40, to_value(&[10.0, 10.0]).unwrap());
    // let points: PointDistribution = vec![[1.0, 2.0], [2.0, 3.0], [2.0, 1.0], [4.0, 2.0]].into();
    // let points: PointDistribution = vec![[1.0, 2.0], [2.0, 3.0], [2.0, 1.0], [4.0, 2.0]].into();

    let triangles: Triangles = from_value(points.get_triangles()).unwrap();
    // console_log!("{:?}", triangles);
}

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
