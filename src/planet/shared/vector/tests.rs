use super::Vector;

#[test]
fn angle() {
    assert_eq!(Vector([1.0, 1.0]).angle(), 45.0);
    assert_eq!(Vector([-1.0, 1.0]).angle(), 135.0);
    assert_eq!(Vector([1.0, -1.0]).angle(), -45.0);
    assert_eq!(Vector([-1.0, -1.0]).angle(), -135.0);
}