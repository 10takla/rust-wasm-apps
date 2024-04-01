use macros::svg_test;

use crate::{planet::shared::{
    point::Point,
    vector::{ui::line::ui::angle::ui::triangle::Triangle, Vector},
}, traits::of_to::Of};

#[test]
fn get_circle() {
    let check = |tri_points: [Point; 3], point| {
        assert_eq!(**Triangle::of(tri_points).get_circle().center, point);
    };

    check([[0.0, 0.0], [3.0, 3.0], [6.0, 0.0]], [3.0, 0.0]);
    check([[1.5, 1.5], [2.5, 2.5], [2.5, 0.5]], [2.5, 1.5]);
    check([[2.0, 1.0], [6.0, 3.0], [9.0, 2.0]], [6.0, -2.0]);
    check([[1.0, 2.0], [2.0, 3.0], [2.0, 1.0]], [2.0, 2.0]);

    check([[1.0, 2.0], [2.0, 3.0], [2.0, 1.0]], [2.0, 2.0]);
    check([[1.0, 2.0], [2.0, 3.0], [2.0, 1.0]], [2.0, 2.0]);
    check([[1.0, 2.0], [2.0, 3.0], [4.0, 2.0]], [2.5, 1.5]);
    check([[2.0, 3.0], [1.0, 2.0], [4.0, 2.0]], [2.5, 1.5]);
}

#[svg_test]
fn triangle_svg() {
    Triangle::of([[0, 0], [1, 2], [2, 0]])
}
