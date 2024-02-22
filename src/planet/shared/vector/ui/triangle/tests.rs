use crate::planet::shared::{point::Point, vector::Vector};

use super::Triangle;

#[test]
fn angles() {
    let triangle: Triangle = [[2.0, 1.0], [1.0, 1.0], [1.0, 2.0]].into();
    assert_eq!(triangle.get_angles(), [
        45.00000000000001,
        90.0,
        45.00000000000001,
    ]);
}

#[test]
fn get_center() {
    let check = |v1: [Point; 3], v| {
        assert_eq!(Triangle::from(v1).get_center(), Vector(v));
    };
    // check(([0.0, 0.0], [3.0, 3.0], [6.0, 0.0]), [3.0, 0.0]);
    // check(([1.5, 1.5], [2.5, 2.5], [2.5, 0.5]), [2.5, 1.5]);
    // check(([2.0, 1.0], [6.0, 3.0], [9.0, 2.0]), [6.0, -2.0]);
    // check(([1.0, 2.0], [2.0, 3.0], [2.0, 1.0]), [2.0, 2.0]);

    // check(([1.0, 2.0], [2.0, 3.0], [2.0, 1.0]), [2.0, 2.0]);
    // check(([1.0, 2.0], [2.0, 3.0], [2.0, 1.0]), [2.0, 2.0]);
    check([[1.0, 2.0], [2.0, 3.0], [4.0, 2.0]], [2.5, 1.5]);
    // check(([2.0, 3.0], [1.0, 2.0], [4.0, 2.0]), [2.5, 1.5]);
}