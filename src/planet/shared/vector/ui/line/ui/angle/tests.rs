use crate::{
    planet::shared::{
        point::Point,
        vector::{ui::line::ui::angle::Angle, Number},
    },
    traits::of_to::Of,
};
use std::fmt::Debug;

#[test]
fn get_polar_angle() {
    fn check<T: Debug + Number, I: Number>(points: [Point<T>; 3], angle: I) {
        assert_eq!(Angle::of(points).get_polar_angle().as_::<I>(), angle);
    }

    check([[1, 0], [0, 0], [-1, 1]], 135);
    check([[-1, 1], [0, 0], [1, 0]], -135);
    check([[1, 0], [0, 0], [-1, -1]], -135);
    check([[-1, -1], [0, 0], [1, 0]], 135);

    check([[4, 4], [3, 2], [2, 1]], 161);
    check([[4, 4], [3, 2], [2, 3]], 71);

    check([[1.0, 0.0], [0.0, 0.0], [-2.0, 0.7]], 160);
    check([[2.0, 0.5], [1.0, 0.5], [2.0, 2.0]], 56);
    check([[1.0, 0.5], [2.0, 2.0], [2.5, 1.0]], 60);
    check([[1.0, 0.5], [2.0, 2.0], [1.0, 2.0]], -56);
}

#[test]
fn get_angle() {
    fn check<T: Debug + Number, I: Number, const N: usize>(points: [Point<T, N>; 3], angle: I) {
        assert_eq!(Angle::of(points).get_angle().as_::<I>(), angle);
    }
    check([[0, 0], [0, 0], [0, 0]], 0);
    check([[1, 0], [0, 0], [0, 0]], 0);

    check([[1, 0], [0, 0], [1, 1]], 45);

    check([[1, 0, 0], [0, 0, 0], [1, 1, 0]], 45);

    check([[1, 0, 1], [0, 0, 0], [1, 1, 0]], 60);

    check([[1.0, 0.0, 0.5], [0.0; 3], [1.0, 1.0, 0.0]], 50);
}

mod from_lines {
    // use super::*;

    // #[test]
    // #[should_panic]
    // fn panic_have_3_common_points() {
    //     let points = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];

    //     let get_angle = |ab: [usize; 2], bc: [usize; 2]| {
    //         let [ab, bc] = [
    //             Line::of([points[ab[0]], points[ab[1]]]),
    //             Line::of([points[bc[0]], points[bc[1]]])
    //         ];
    //         Angle::of([ab, bc])
    //     };

    //     // паника, когда 3 точки с одной координатой
    //     get_angle([0, 0], [0, 1]);
    // }

    // #[test]
    // #[should_panic]
    // fn panic_have_0_common_points() {
    //     let points = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [2.0, 1.0]];

    //     let get_angle = |ab: [usize; 2], bc: [usize; 2]| {
    //         let [ab, bc] = [
    //             Line::of([points[ab[0]], points[ab[1]]]),
    //             Line::of([points[bc[0]], points[bc[1]]])
    //         ];
    //         Angle::of([ab, bc])
    //     };

    //     // паника, когда нет 2 общих точек
    //     get_angle([0, 1], [2, 3]);
    // }
}

// convert_test!(Angle, 3, [1, 1], [2, 2], [3, 3]);
