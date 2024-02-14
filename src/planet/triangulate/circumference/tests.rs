use crate::planet::point_distribution::point::Point;

use super::Circle;

#[test]
fn get_center() {
    let circle = Circle{a: Point([0.0, 0.0]), b: Point([3.0, 3.0]), c: Point([6.0, 0.0])};

    assert_eq!(circle.get_center(), Point([3.0, 0.0]));
}