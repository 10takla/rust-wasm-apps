use std::time::Instant;

use crate::planet::shared::point::Points;

use super::*;

#[test]
fn set_random_points() {
    let points_count = 10;
    let sizes = [5.0, 5.0];
    let pd = PointDistribution::set_random_points(points_count, sizes);
    
    assert_eq!(pd.len(), points_count);

    Points::of(pd).into_iter().for_each(|point| {
        point.into_iter().enumerate().for_each(|(i, measure)| {
            assert!(measure <= sizes[i]);
        })
    });
}

#[test]
fn convex_hull() {
    let pd = PointDistribution::<f64>::of(vec![
        [0.0, 0.0],
        [1.0, -0.2],
        [2.0, 1.0],
        [1.8, 2.0],
        [0.7, 2.1],
        [0.2, 1.6],
    ]);
    assert_eq!(pd.convex_hull(), vec![0, 1, 2, 3, 4, 5, 0]);
}

#[test]
fn get_angle() {
    let pd = PointDistribution::of(vec![[-1.0, 0.0], [0.0, 0.0], [1.0, 0.0]]);
    assert_eq!(pd.get_angle(&vec![0, 1], 2), 180.0);
}

#[test]
#[ignore]
fn prefomance_by_points() {
    let start = Instant::now();
    let pd = PointDistribution::set_random_points(100000, [5.0, 5.0]);

    let edges = pd.convex_hull();
    
    let end = Instant::now() - start;
    dbg!(end);
    let count_ident = edges.iter().filter(|x| !edges.contains(x)).count();
    assert_eq!(count_ident, 0);
}
