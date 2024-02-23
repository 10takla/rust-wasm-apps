use crate::planet::shared::{point::Point, vector::Vector};

use super::Triangle;

#[test]
fn angles() {
    let triangle: Triangle = [[0.0, 0.0], [9.0, 0.0], [0.0, 9.0]].into();
    println!("{}", triangle);
    assert_eq!(triangle.bac.get_angle(), 45.00000000000001);
    // assert_eq!(triangle.abc.get_angle(), 90.0);
    // assert_eq!(triangle.acb.get_angle(), 45.00000000000001);
}

#[test]
fn get_circle() {
    let check = |v1: [Point; 3], v| {
        // assert_eq!(Triangle::from(v1).get_circle().center, Vector(v));
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

// #[test]
// fn from_line() {
//     let points = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];
//     let t = Triangle::from([
//         Line::from([points[0], points[1]]),
//         Line::from([points[1], points[2]]),
//         Line::from([points[0], points[2]]),
//     ]);
//     dbg!(t);
// }