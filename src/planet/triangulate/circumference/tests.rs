use super::Circle;

#[test]
fn get_center() {
    let circle = Circle{a: [0.0, 0.0], b: [3.0, 3.0], c: [6.0, 0.0]};

    assert_eq!(circle.get_center(), [3.0, 0.0]);
}