use std::rc::Rc;

use crate::{
    planet::shared::{
        point::Point,
        traits::SuperAlien,
        vector::{ui::line::ui::angle::ui::triangle::ui::rectangle::Rectangle, Vector},
    },
    traits::of_to::Of,
};

#[test]
fn alien_vecs() {
    let check = |points: [Point<i32>; 4], inds: [usize; 2]| {
        let vecs = points.map(|p| Rc::new(Vector(p)));
        let rect = Rectangle::of(vecs.clone());

        assert!(rect
            .alien::<[Rc<Vector<i32>>; 2]>(&rect.get_common_line())
            .into_iter()
            .zip(inds.map(|i| vecs[i].clone()).into_iter())
            .all(|(a, b)| Rc::ptr_eq(&a, &b)))
    };

    check([[4, 9], [4, 7], [4, 9], [3, 9]], [0, 3]);
    check([[0, 0], [0, 0], [0, 0], [0, 0]], [0, 3]);
}
