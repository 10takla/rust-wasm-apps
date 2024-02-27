use crate::planet::shared::{point::Point, vector::ui::line::{ui::angle::Angle, Line}};

mod from_lines {
    use super::*;

    // #[test]
    // #[should_panic]
    // fn panic_have_3_common_points() {
    //     let points = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];
    
    //     let get_angle = |ab: [usize; 2], bc: [usize; 2]| {
    //         let (ab, bc) = (
    //             Line::from([points[ab[0]], points[ab[1]]]),
    //             Line::from([points[bc[0]], points[bc[1]]])
    //         );
    //         Angle::from([ab, bc])
    //     };
    
    //     // паника, когда 3 точки с одной координатой
    //     get_angle([0, 0], [0, 1]);
    // }
    
    // #[test]
    // #[should_panic]
    // fn panic_have_0_common_points() {
    //     let points = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [2.0, 1.0]];
    
    //     let get_angle = |ab: [usize; 2], bc: [usize; 2]| {
    //         let (ab, bc) = (
    //             Line::from([points[ab[0]], points[ab[1]]]),
    //             Line::from([points[bc[0]], points[bc[1]]])
    //         );
    //         Angle::from([ab, bc])
    //     };
    
    //     // паника, когда нет 2 общих точек
    //     get_angle([0, 1], [2, 3]);
    // }
}


#[test]
fn convert() {
    let angle = Angle::from([[1, 1], [2, 2], [3,3]]);
    let angle: Angle<f64> = (&angle).into();
    let angle: [Point; 3] = angle.into();
    assert_eq!(angle, [[1.0, 1.0], [2.0, 2.0], [3.0, 3.0]])
}