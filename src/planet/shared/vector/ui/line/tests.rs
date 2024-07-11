use std::time::Instant;

use super::Line;
use crate::planet::point_distribution::PointDistribution;
use crate::planet::shared::traits::Indices;
use crate::planet::shared::vector::ui::line::LineType;
use crate::traits::of_to::Of;

#[macro_export]
macro_rules! convert_test {
    ($name:ident, $l:tt, $([$($c:tt), +]), +) => {
        #[test]
        fn convert() {
            let line = $name::of([$([$($c), +],) +]);
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

#[test]
fn get_inds() {
    let vecs_count = 1000;
    let pd = PointDistribution::set_random_points(vecs_count, [[0, 0], [10, 10]]);

    let inds = (0..vecs_count)
        .map(|i| LineType::of([&pd[i], &pd[(i + 1) % vecs_count]]))
        .collect::<Vec<LineType<i32>>>()
        .get_inds(pd.0);

    assert_eq!(inds.len(), vecs_count);
    assert_eq!(
        inds,
        (0..vecs_count)
            .map(|i| [i, (i + 1) % vecs_count])
            .collect::<Vec<[usize; 2]>>()
    );
}
