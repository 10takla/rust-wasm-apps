use crate::planet::shared::vector::ui::line::{ui::angle::Angle, Line};

#[test]
fn from_line() {
    let points = [[0.0, 0.0], [1.0, 0.0], [0.0, 1.0]];
    let t = Angle::from([
        Line::from([points[0], points[1]]),
        Line::from([points[1], points[2]]),
    ]);
    dbg!(t);
}