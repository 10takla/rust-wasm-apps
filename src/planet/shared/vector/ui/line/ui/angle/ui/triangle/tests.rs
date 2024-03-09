use crate::planet::shared::{
    point::Point,
    vector::{ui::line::ui::angle::ui::triangle::Triangle, Vector},
};

#[test]
fn get_circle() {
    let check = |v1: [Point; 3], v| {    
        assert_eq!(*Triangle::from(v1).get_circle().center, Vector(v));
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
