use crate::{
    planet::{
        point_distribution::{
            ui::triangulate::{
                delone::Delone, spherical_triangulate::SphericalTriangulate, Triangles, Triangulate,
            },
            PointDistribution,
        },
        shared::traits::{As, Normalize},
    },
    traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
    },
    utils::svg::draw_svg,
};
use serde_wasm_bindgen::from_value;
use wasm_bindgen_test::{console_log, wasm_bindgen_test};
use crate::planet::shared::traits::Projection;

#[test]
fn delone() {
    let pd = PointDistribution::of(vec![[0, 0], [0, 5], [-2, 3], [6, 3]]).as_::<f64>();
    // let tries = pd.delone();
}

#[test]
fn triagulate() {
    let pd = PointDistribution::set_random_points(20, [[0.; 2], [10.; 2]]);

    draw_svg(vec![&pd.triangulate()], "triagulate", module_path!(), "")
}

#[test]
fn spherical_triangulate() {
    let mut pd = PointDistribution::set_random_points(10, [[2.; 3], [30.; 3]]);
    pd.normalize();
    let tries = dbg!(SphericalTriangulate::real(&pd));
    draw_svg(vec![&tries.projection(&[0])], "spherical_triagulate", module_path!(), "")

}
