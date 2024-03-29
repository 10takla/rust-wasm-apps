use super::PointDistribution;


mod max {
    use crate::{planet::point_distribution::Points, traits::of_to::Of};
    use super::*;

    #[test]
    fn get_max_point() {
        assert_eq!(
            PointDistribution::of(vec![[3.0, 1.9], [8.0, 0.4], [4.0, 4.5], [4.0, 0.0]])
                .get_max_point(),
            1
        );
        assert_eq!(
            PointDistribution::of(vec![[0.0, 0.0], [0.0, 0.0]]).get_max_point(),
            0
        );
        assert_eq!(
            PointDistribution::of(vec![[1.0, 0.0], [0.0, 0.0]]).get_max_point(),
            0
        );
        assert_eq!(
            PointDistribution::of(vec![[0.0, 0.1], [0.0, 0.0]]).get_max_point(),
            0
        );
        assert_eq!(
            PointDistribution::of(vec![[0.0, 0.1], [0.0, 0.2]]).get_max_point(),
            1
        );
        assert_eq!(
            PointDistribution::of(vec![[1.0, 0.1], [1.0, 0.0]]).get_max_point(),
            0
        );
        assert_eq!(
            PointDistribution::of(vec![[1.0, 0.1], [1.0, 0.7]]).get_max_point(),
            1
        );
        assert_eq!(
            PointDistribution::of(vec![[0.0, 2.0], [1.0, 0.0]]).get_max_point(),
            1
        );
    }

    #[test]
    fn sort_points_by_max() {
        let check = |points: Points, v2| {
            let sorted_points: Points = PointDistribution::of(points)
                .sort_points_by_max()
                .iter()
                .map(|&(_, p)| *p)
                .collect();
            assert_eq!(sorted_points, v2);
        };
        check(
            vec![[3.0, 1.9], [8.0, 0.4], [4.0, 4.5], [4.0, 0.0]],
            vec![[8.0, 0.4], [4.0, 4.5], [4.0, 0.0], [3.0, 1.9]],
        );
        check(
            vec![[10.0, 1.9], [10.0, 0.2], [10.0, 0.4]],
            vec![[10.0, 1.9], [10.0, 0.4], [10.0, 0.2]],
        );
    }
}

mod min {
    use crate::{planet::point_distribution::Points, traits::of_to::Of};

    use super::*;

    #[test]
    fn get_min_point() {
        let points = PointDistribution::of(vec![[3.0, 0.0], [0.0, 0.0], [4.0, 0.0]]);
        assert_eq!(points.get_min_point(), 1);
    }

    #[test]
    fn sort_points_by_min() {
        let points = PointDistribution::of(vec![[3.0, 0.0], [0.0, 0.0], [4.0, 0.0]]);
        let sorted_points: Points = points
            .sort_points_by_min()
            .iter()
            .map(|&(_, p)| *p)
            .collect();
        assert_eq!(sorted_points, vec![[0.0, 0.0], [3.0, 0.0], [4.0, 0.0]]);
    }
}
