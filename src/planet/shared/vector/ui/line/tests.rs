use super::Line;

#[macro_export]
macro_rules! convert_test {
    ($name:ident, $l:tt, $([$($c:tt), +]), +) => {
        #[test]
        fn convert() {
            let line = $name::from([$([$($c), +],) +]);
            let line: $name<f64> = (&line).into();

            let points: [Point; $l] = line.into();
            assert_eq!(points, [$([$($c as f64), +],) +]);
        }
    };
}

// convert_test!(Line, 2, [1, 1], [2, 2]);

#[test]
fn display() {
    let line = Line::from([[0, 8], [8, 0]]);
    println!("{}", line);
    let line = Line::from([[0, 0], [8, 4]]);
    println!("{}", line);
}

#[test]
fn from() {
    let points = [[2.0; 3]; 2];
    let line = Line::from(points);
}
