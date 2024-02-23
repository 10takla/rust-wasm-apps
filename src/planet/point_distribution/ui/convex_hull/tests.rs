use super::*;
use std::f64::consts::PI;
use wasm_bindgen_test::*;

mod set_points_tests {
    use super::*;

    #[wasm_bindgen_test]
    fn set_points() {
        let to_points = vec![[0.0, 0.0], [1.0, 1.0], [1.0, -1.0], [0.8, -1.0]];
        let hull = {
            let point_distribution = PointDistribution::from_points(to_value(&to_points).unwrap());
            ConvexHull::from_distribution(to_value(&point_distribution).unwrap())
        };

        let points: Points = from_value(hull.point_distribution.points()).unwrap();
        assert_eq!(points, to_points);
    }

    #[wasm_bindgen_test]
    fn set_random_points() {
        let points_count = 10;
        let sizes = [5.0, 5.0, 5.0];
        let hull = {
            let point_distribution =
                PointDistribution::set_random_points(points_count, to_value(&sizes).unwrap());
            ConvexHull::from_distribution(to_value(&point_distribution).unwrap())
        };

        let points: Points = from_value(hull.point_distribution.points()).unwrap();
        assert_eq!(points.len(), points_count);

        points.iter().for_each(|point| {
            point.iter().enumerate().for_each(|(i, &measure)| {
                assert!(measure <= sizes[i]);
            })
        });
    }
}

mod convex_hull {
    use super::*;

    #[wasm_bindgen_test]
    fn convex_hull() {
        let mut hull = {
            let point_distribution = PointDistribution::from_points(
                to_value(&vec![
                    [0.0, 0.0, 0.0],
                    [1.0, -0.2, 0.0],
                    [2.0, 1.0, 0.0],
                    [1.8, 2.0, 0.0],
                    [0.7, 2.1, 0.0],
                    [0.2, 1.6, 0.0],
                ])
                .unwrap(),
            );
            ConvexHull::from_distribution(to_value(&point_distribution).unwrap())
        };
        let mut check_tick = |edge| {
            assert_eq!(hull.tick(), edge);
        };

        [0, 1, 2, 3, 4, 5, 0].iter().for_each(|&i| {
            check_tick(Some(i));
        });
        check_tick(None);
    }

    #[wasm_bindgen_test]
    fn tick_with_empty_points() {
        let mut hull = ConvexHull {
            point_distribution: PointDistribution(vec![]),
            hull_edges: vec![],
        };

        let edge = hull.tick();
        assert_eq!(edge, None);
    }
}

#[wasm_bindgen_test]
fn get_angle() {
    let point_distribution = PointDistribution::from(vec![[-1.0, 0.0], [0.0, 0.0], [1.0, 0.0]]);
    let hull = ConvexHull {
        point_distribution,
        hull_edges: vec![0, 1],
    };
    assert_eq!(hull.get_angle(2), 180.0);
}

#[wasm_bindgen_test]
#[ignore]
fn prefomance_by_points() {
    let points_count = 10000;
    let sizes = [5.0, 5.0, 5.0];
    let mut hull = {
        let point_distribution =
            PointDistribution::set_random_points(points_count, to_value(&sizes).unwrap());
        ConvexHull::from_distribution(to_value(&point_distribution).unwrap())
    };

    let edges = hull.get_convex_hull();
    let edges: HullEdges = from_value(edges).unwrap();
    let count_ident = edges.iter().filter(|x| !edges.contains(x)).count();
    assert_eq!(count_ident, 0);
}
