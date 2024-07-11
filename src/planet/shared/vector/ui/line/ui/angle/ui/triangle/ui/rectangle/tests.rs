use std::rc::Rc;

use crate::{
    planet::shared::{
        point::Point,
        vector::{
            ui::line::{ui::angle::ui::triangle::ui::rectangle::Rectangle, Line},
            Vector,
        },
    },
    traits::of_to::{Of, To},
};

#[test]
fn reverse_tries() {
    let mut rect = Rectangle::of([[0, 1], [1, 1], [0, 0], [1, 0]]);
    rect.reverse_tries();
    assert_eq!(
        rect.to::<[Point<i32>; 4]>(),
        [[0, 1], [1, 1], [1, 0], [0, 0]]
    );
}

#[test]
fn set_delone() {
    let check = |points: [Point; 4], inds: [usize; 2]| {
        let vecs = points.map(|point| point.to::<Rc<Vector>>());
        let mut rect = Rectangle::of(vecs.clone());
        rect.set_delone();
        assert_eq!(
            *rect.get_common_line().as_ref(),
            Line::of(inds.map(|i| &vecs[i]))
        );
    };
    check([[0.25, 0.75], [1.0, 1.0], [0.0, 0.0], [0.75, 0.25]], [0, 3]);
    check([[0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]], [1, 2]);
}
