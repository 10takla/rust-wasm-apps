use crate::planet::shared::{
    point::Point,
    vector::{ui::line::ui::angle::ui::triangle::Triangle, Vector},
};

#[test]
fn get_circle() {
    let check = |v1: [Point; 3], v| {
        let vecs: Vec<Vector> = v1.into_iter()
        .map(|p| p.into())
        .collect::<Vec<Vector>>();
        
        let default_vector = Vector::default();
        let mut t = [&default_vector; 3];
        for i in 0..v1.len() {
            t[i] = &vecs[i]
        }
        
        assert_eq!(Triangle::from(t).get_circle().center, Vector(v));
    };
    check([[0.0, 0.0], [3.0, 3.0], [6.0, 0.0]], [3.0, 0.0]);
    check([[1.5, 1.5], [2.5, 2.5], [2.5, 0.5]], [2.5, 1.5]);
    check([[2.0, 1.0], [6.0, 3.0], [9.0, 2.0]], [6.0, -2.0]);
    check([[1.0, 2.0], [2.0, 3.0], [2.0, 1.0]], [2.0, 2.0]);

    check([[1.0, 2.0], [2.0, 3.0], [2.0, 1.0]], [2.0, 2.0]);
    check([[1.0, 2.0], [2.0, 3.0], [2.0, 1.0]], [2.0, 2.0]);
    check([[1.0, 2.0], [2.0, 3.0], [4.0, 2.0]], [2.5, 1.5]);
    check([[2.0, 3.0], [1.0, 2.0], [4.0, 2.0]], [2.5, 1.5]);
}
