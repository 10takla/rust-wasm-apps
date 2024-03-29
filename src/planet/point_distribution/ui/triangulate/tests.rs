use serde_wasm_bindgen::from_value;
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

use crate::{planet::point_distribution::{ui::triangulate::{Triangles, Triangulate}, PointDistribution}, traits::of_to::Of};

#[wasm_bindgen_test]
fn get_delone() {
    let pd = PointDistribution::set_random_points(40, [10.0, 10.0]);
    // let points: PointDistribution = vec![[1.0, 2.0], [2.0, 3.0], [2.0, 1.0], [4.0, 2.0]].to();
    // let points: PointDistribution = vec![[1.0, 2.0], [2.0, 3.0], [2.0, 1.0], [4.0, 2.0]].to();

    // let triangles: Triangles = from_value(pd.get_triangles()).unwrap();
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


#[test]
fn delone() {
    let pd = PointDistribution::of(vec![[0, 0], [0, 5], [-2, 3], [6, 3]]).as_::<f64>();

    // let tries = pd.delone();
}
