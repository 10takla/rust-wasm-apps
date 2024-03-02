use crate::planet::shared::point::Point;

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

