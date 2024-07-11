use super::*;
use crate::{
    planet::shared::{
        point::Point, traits::Svg, vector::ui::line::ui::angle::ui::triangle::ui::icosahedron::Path,
    },
    utils::svg::draw_svg,
};
use std::{iter::once, time::Instant};

impl Svg for Hull<f64, 2> {
    fn to_svg(&self, document: &mut svg::Document) {
        let mut points = self.to::<Vec<Point<f64, 2>>>();

        if points.len() == 0 {
            return;
        }

        let [start_x, start_y] = points.remove(0);
        let pa = points
            .iter()
            .chain(once(&[start_x, start_y]))
            .map(|[x, y]| format!("L {x} {y}"))
            .collect::<Vec<_>>()
            .join(" ");

        *document = document
            .clone()
            .add(svg::node::element::Path::new().set("d", format!("M {start_x} {start_y} {pa} Z")));
    }
}

impl Svg for Path<f64, 2> {
    fn to_svg(&self, document: &mut svg::Document) {
        let mut points = self.to::<Vec<Point<f64, 2>>>();

        if points.len() == 0 {
            return;
        }

        let [start_x, start_y] = points.remove(0);
        let pa = points
            .into_iter()
            .map(|[x, y]| format!("L {x} {y}"))
            .collect::<Vec<_>>()
            .join(" ");

        *document = document
            .clone()
            .add(svg::node::element::Path::new().set("d", format!("M {start_x} {start_y} {pa} Z")));
    }
}

#[test]
fn convex_hull() {
    let check = |points, inds: Vec<usize>| {
        let mut pd = PointDistribution::of(points);
        draw_svg(vec![&pd.0], "vecs", module_path!(), "");
        let [lines1, lines2] = [
            inds.into_iter()
                .map(|i| &pd[i])
                .collect::<Vec<_>>()
                .to::<Hull>(),
            pd.convex_hull(),
        ];
        draw_svg(vec![&lines2], "lines", module_path!(), "");
        assert_eq!(lines1, lines2);
    };

    check(
        vec![
            [0.0, 0.0],
            [1.0, -0.2],
            [2.0, 1.0],
            [1.8, 2.0],
            [0.7, 2.1],
            [0.2, 1.6],
        ],
        vec![0, 1, 2, 3, 4, 5],
    );

    check(
        vec![[0.0, 1.0], [2.0, 2.0], [2.0, 5.0], [2.0, 9.0]],
        vec![0, 1, 2, 3],
    );

    check(
        vec![[2.0, 1.0], [2.0, 3.0], [3.0, 2.0], [4.0, 1.0]],
        vec![0, 3, 2, 1],
    );

    check(vec![[0.0, 1.0], [2.0, 2.0]], vec![0, 1]);

    check(vec![[0.0, 1.0]], vec![0]);

    check(vec![], vec![]);
}

#[test]
fn get_angle() {
    let pd = PointDistribution::of(vec![[-1, 0], [0, 0], [1, 0]]);
    assert_eq!(
        pd.get_angle(&vec![pd[0].clone(), pd[1].clone()], &pd[2]),
        180
    );
}

#[test]
#[ignore]
fn prefomance_set_random_points() {
    let start = Instant::now();
    let mut pd = PointDistribution::set_random_points(100000, [[0.0, 0.0], [5.0, 5.0]]);

    let edges = pd.convex_hull();

    let end = Instant::now() - start;
    dbg!(end);
    let count_ident = edges.iter().filter(|x| !edges.contains(x)).count();
    assert_eq!(count_ident, 0);
}
