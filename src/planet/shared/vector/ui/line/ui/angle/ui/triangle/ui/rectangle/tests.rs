use std::rc::Rc;

use crate::{
    planet::shared::{
        point::Point,
        vector::{
            ui::line::{
                ui::angle::ui::triangle::{ui::rectangle::Rectangle, Triangle},
                Line,
            },
            Vector,
        },
    },
    traits::of_to::{Of, To},
};

#[test]
fn reverse_tries() {
    let rect = Rectangle::of([[0; 2], [1; 2], [1; 2], [3; 2]]);
    let rect = rect.reverse_tries();
    assert_eq!(
        rect.to::<[Point<i32>; 4]>(),
        [[1, 1], [0, 0], [3, 3], [1, 1]]
    );
}