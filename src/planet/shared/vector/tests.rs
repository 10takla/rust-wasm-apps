use super::Vector;

#[test]
fn angle() {
    assert_eq!(Vector([1, 1]).angle(), 45);
    assert_eq!(Vector([-1, 1]).angle(), 135);
    assert_eq!(Vector([1, -1]).angle(), -45);
    assert_eq!(Vector([-1, -1]).angle(), -135);
}

#[test]
fn vector_from() {
    let vector: Vector<i32> = Vector::from([1; 2]);
    {
        let v_f64: Vector<f64> = vector.as_();
        assert_eq!(*v_f64, [1.0; 2]);
    }
}