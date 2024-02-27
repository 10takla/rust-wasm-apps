use crate::planet::shared::point::Point;

use super::Line;

#[test]
fn convert() {
    let line = Line::from([[1, 1], [2, 2]]);
    let line = Line::<f64>::from(&line);
    let points: [Point; 2] = line.into();
    assert_eq!(points, [[1.0, 1.0], [2.0, 2.0]]);
}