use super::{PointDistribution, Random};
use crate::{
    planet::{
        point_distribution::ui::triangulate::Triangulate,
        shared::{
            point::{Point, Points},
            traits::{Normalize, Svg, SvgStyle},
            vector::{
                svg::VectorStyle,
                ui::line::{
                    svg::LineStyle,
                    ui::angle::ui::triangle::{Triangle, TriangleType},
                    LineType,
                },
                Vector, VectorType,
            },
        },
    },
    traits::of_to::{Of, To},
    utils::svg::draw_svg,
};

#[test]
fn normalize() {
    let mut pd = PointDistribution::of(vec![[0.4, 0.], [0., 0.2]]);
    pd.normalize();
    let t = pd.to::<Vec<Point>>();
    assert_eq!(t, vec![[1., 0.], [0., 1.]]);
}

mod random {
    use super::*;
    
    #[test]
    fn set_random_points() {
        let check = |points_count, sizes| {
            let pd = PointDistribution::set_random_points(points_count, sizes);

            assert_eq!(pd.len(), points_count);

            Points::of(pd).into_iter().for_each(|point| {
                point.into_iter().enumerate().for_each(|(i, measure)| {
                    assert!(sizes[0][i] <= measure && measure <= sizes[1][i]);
                })
            });
        };

        check(10, [[0., 0.], [5., 5.]]);
        check(100, [[-1., -1.], [1., 1.]]);
        check(100, [[-1., -1.], [-1., -1.]]);
        check(100, [[0., 0.], [0., 0.]]);

        draw_svg(
            vec![&SvgStyle(
                PointDistribution::set_random_points(50, [[1.; 2], [10.; 2]]).0,
                VectorStyle::default().set_text(None).set_color("red").set_width(0.15),
            )],
            "points",
            module_path!(),
            "random",
        );
    }

    #[test]
    fn set_random_points_from_tri() {
        let pd = PointDistribution::set_random_points(3, [[0.0; 2], [10.0; 2]]);
        let tri = TriangleType::of([pd[0].clone(), pd[1].clone(), pd[2].clone()]);

        let pd: PointDistribution =
            Random::<TriangleType>::set_random_points(Some(100), None, &tri);

        draw_svg(
            vec![&tri, &SvgStyle(pd.0, VectorStyle::default().set_text(None))],
            "tri",
            module_path!(),
            "random",
        );
    }

    #[test]
    fn set_random_points_from_tries() {
        let tries = PointDistribution::set_random_points(10, [[0.0; 2], [10.0; 2]]).triangulate();

        let pd: PointDistribution =
            Random::<Vec<TriangleType>>::set_random_points(None, Some(0.5), &tries);

        draw_svg(vec![&tries, &pd.0], "tries", module_path!(), "random");
    }

    #[test]
    fn set_random_points_from_line() {
        let pd = PointDistribution::set_random_points(2, [[0.0; 2], [10.0; 2]]);

        let line = LineType::of([pd[0].clone(), pd[1].clone()]);

        let pd: PointDistribution = Random::<LineType>::set_random_points(Some(10), None, &line);

        draw_svg(
            vec![
                &SvgStyle(line, LineStyle::default().set_color("red")),
                &SvgStyle(pd.0, VectorStyle::default().set_text(None)),
            ],
            "line",
            module_path!(),
            "random",
        );
    }

    #[test]
    fn set_random_points_from_lines() {
        let tries = PointDistribution::set_random_points(10, [[0.0; 2], [10.0; 2]]).triangulate();
        let lines = tries
            .into_iter()
            .map(|tri| tri.to::<[LineType; 3]>())
            .flatten()
            .collect::<Vec<LineType>>();

        let pd: PointDistribution =
            Random::<Vec<LineType>>::set_random_points(Some(40), None, &lines);

        draw_svg(
            vec![
                &SvgStyle(lines, LineStyle::default().set_color("red")),
                &SvgStyle(pd.0, VectorStyle::default().set_text(None)),
            ],
            "lines",
            module_path!(),
            "random",
        );
    }
}

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
                inds.into_iter()
                    .map(|i| pd[i].clone())
                    .collect::<Vec<Rc<Vector>>>(),
                pd.clone().sort_points_by_max().0,
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
        let mut points = PointDistribution::of(vec![[3.0, 0.0], [0.0, 0.0], [4.0, 0.0]]);
        assert_eq!(points[1].clone(), points.get_min_point());
    }

    #[test]
    fn sort_points_by_min() {
        let mut pd = PointDistribution::of(vec![[3.0, 0.0], [0.0, 0.0], [4.0, 0.0]]);
        assert_eq!(
            vec![pd[1].clone(), pd[0].clone(), pd[2].clone()],
            pd.sort_points_by_min().0,
        );
    }
}
