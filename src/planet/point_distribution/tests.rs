use super::PointDistribution;

mod max {
    use std::rc::Rc;

    use super::*;
    use crate::{
        planet::{point_distribution::Points, shared::vector::Vector},
        traits::of_to::Of,
    };

    #[test]
    fn get_max_point() {
        assert_eq!(
            PointDistribution::of(vec![[3.0, 1.9], [8.0, 0.4], [4.0, 4.5], [4.0, 0.0]])
                .get_max_point()
                .0,
            [8.0, 0.4]
        );
        assert_eq!(
            PointDistribution::of(vec![[0.0, 0.0], [0.0, 0.0]])
                .get_max_point()
                .0,
            [0.0, 0.0]
        );
        assert_eq!(
            PointDistribution::of(vec![[1.0, 0.0], [0.0, 0.0]])
                .get_max_point()
                .0,
            [1.0, 0.0]
        );
        assert_eq!(
            PointDistribution::of(vec![[0.0, 0.1], [0.0, 0.0]])
                .get_max_point()
                .0,
            [0.0, 0.1]
        );
        assert_eq!(
            PointDistribution::of(vec![[0.0, 0.1], [0.0, 0.2]])
                .get_max_point()
                .0,
            [0.0, 0.2]
        );
        assert_eq!(
            PointDistribution::of(vec![[1.0, 0.1], [1.0, 0.0]])
                .get_max_point()
                .0,
            [1.0, 0.1]
        );
        assert_eq!(
            PointDistribution::of(vec![[1.0, 0.1], [1.0, 0.7]])
                .get_max_point()
                .0,
            [1.0, 0.7]
        );
        assert_eq!(
            PointDistribution::of(vec![[0.0, 2.0], [1.0, 0.0]])
                .get_max_point()
                .0,
            [1.0, 0.0]
        );
    }

    #[test]
    fn sort_points_by_max() {
        let check = |points: Points, inds: Vec<usize>| {
            let pd = PointDistribution::of(points);
            assert_eq!(
                pd.clone().sort_points_by_max().0,
                inds.into_iter()
                    .map(|i| pd[i].clone())
                    .collect::<Vec<Rc<Vector>>>()
            );
        };
        check(
            vec![[3.0, 1.9], [8.0, 0.4], [4.0, 4.5], [4.0, 0.0]],
            vec![1, 2, 3, 0],
        );
        check(vec![[10.0, 1.9], [10.0, 0.2], [10.0, 0.4]], vec![0, 2, 1]);
    }
}

mod min {

    use crate::traits::of_to::Of;

    use super::*;

    #[test]
    fn get_min_point() {
        let points = PointDistribution::of(vec![[3.0, 0.0], [0.0, 0.0], [4.0, 0.0]]);
        assert_eq!(points.get_min_point(), points[1]);
    }

    #[test]
    fn sort_points_by_min() {
        let pd = PointDistribution::of(vec![[3.0, 0.0], [0.0, 0.0], [4.0, 0.0]]);
        assert_eq!(
            pd.sort_points_by_min().0,
            vec![pd[1].clone(), pd[0].clone(), pd[2].clone()]
        );
    }
}
