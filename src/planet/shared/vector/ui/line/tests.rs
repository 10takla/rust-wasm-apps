use super::Line;
use crate::traits::of_to::Of;

#[macro_export]
macro_rules! convert_test {
    ($name:ident, $l:tt, $([$($c:tt), +]), +) => {
        #[test]
        fn convert() {
            let line = $name::from([$([$($c), +],) +]);
            let line: $name<f64> = (&line).to();

            let points: [Point; $l] = line.to();
            assert_eq!(points, [$([$($c as f64), +],) +]);
        }
    };
}

// convert_test!(Line, 2, [1, 1], [2, 2]);

#[test]
fn display() {
    let line = Line::of([[0, 8], [8, 0]]);
    println!("{}", line);
    let line = Line::of([[0, 0], [8, 4]]);
    println!("{}", line);
}