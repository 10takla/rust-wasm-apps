use super::{Cells, SphericalVoronoi};
use crate::planet::shared::point::{DefaultMeasureValue, Point};
use crate::planet::shared::traits::{Normalize, Projection};
use crate::planet::shared::vector::Number;
use crate::traits::as_prim::AsPrim;
use crate::{
    planet::{
        point_distribution::PointDistribution,
        shared::{
            traits::{Has, Indices, SuperAlien, SvgStyle},
            vector::{
                svg::VectorStyle,
                ui::line::{
                    svg::LineStyle,
                    ui::angle::{
                        ui::triangle::ui::{hull::Hull, polyhedron::Polyhedron},
                        Angle,
                    },
                    LineType,
                },
                Vector, VectorType,
            },
        },
    },
    traits::of_to::{Of, To},
    utils::svg::draw_svg,
};
use rand::{thread_rng, Rng};
use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

fn get_start() -> (PointDistribution<f64, 3>, Vec<LineType<f64, 3>>) {
    let pd = PointDistribution::of(
        vec![
            [2.0, 0.5, 0.0],
            [1.0, 0.5, 0.0],
            [0.5, 1.0, 0.0],
            [1.0, 2.0, 0.0],
            [2.0, 2.0, 0.0],
            [2.5, 1.0, 0.0],
            [3.5, 1.0, 0.0],
            [3.5, 2.0, 0.0],
            [2.5, 2.5, 0.0],
            [2.0, 3.0, 0.0],
            [1.0, 3.0, 0.0],
        ]
        .into_iter()
        .map(|mut t| {
            t[2] = thread_rng().gen_range(0.0..2.0);
            t
        })
        .collect::<Vec<Point<f64, 3>>>(),
    );

    let mut edges = {
        let y = &pd[0..6];
        y.into_iter()
            .enumerate()
            .map(|(i, v)| LineType::of([v, &y[(i + 1) % y.len()]]))
            .collect::<Vec<LineType<f64, 3>>>()
    };
    {
        let t = {
            let mut t = pd.0.clone()[4..11].to_vec();
            t.push(pd[3].clone());
            t
        };
        let Hull(lines) = Hull::of(t);
        lines.iter().cloned().for_each(|line| edges.push(line));
    }
    edges.push(LineType::of([&pd[4], &pd[8]]));
    edges.push(LineType::of([&pd[1], &pd[4]]));
    edges.push(LineType::of([&pd[10], &pd[4]]));
    edges.push(LineType::of([&pd[5], &pd[8]]));

    edges.iter_mut().for_each(|edge| {
        if thread_rng().gen_bool(0.2) {
            edge.reverse();
        }
    });

    edges = edges.into_iter().fold(vec![], |mut acc, curr| {
        if (!acc.contains(&curr)) {
            acc.push(curr)
        }
        acc
    });
    (pd, edges)
}

#[test]
fn find_cells() {
    let (pd, edges) = get_start();
    let t = Cells(edges.clone()).find_cells();

    let lines = t
        .into_iter()
        .map(|hull| (**hull).clone())
        .flatten()
        .map(|line| line.clone())
        .map(|line| line.projection(&[0]))
        .collect::<Vec<LineType<f64, 2>>>();

    draw_svg(vec![&lines], "one_line", module_path!(), "");
}

#[test]
#[ignore]
fn random_cells() {
    let mut pd = PointDistribution::set_random_points(6, [[1.0; 3], [10.0; 3]]);
    pd.normalize();
    let lines = SphericalVoronoi::get_edges(pd)
        .into_iter()
        .map(|line| line.projection(&[0]))
        .collect::<Vec<LineType>>();

    draw_svg(vec![&lines], "one_line", module_path!(), "");

    // let sv = SphericalVoronoi::generate(pd);
    // dbg!(&sv);
}

#[test]
fn react_points() {
    let points = vec![
        [
            -0.4078103217716973,
            -0.026258082366354518,
            -0.0227071052118073,
        ],
        [
            -0.3238730135332224,
            -0.6079698701154288,
            -0.5432531641969152,
        ],
        [
            -0.12046400090683385,
            -0.8579926266125093,
            0.8105823689687242,
        ],
        [
            -0.5857918202790562,
            -0.8068825332656155,
            -0.9322883772809707,
        ],
        [
            0.19587175068949314,
            -0.9490122366897604,
            -0.7507328507212243,
        ],
        [0.8911931212775008, 0.4311191864444388, 0.2591225678053415],
    ];

    let pd = PointDistribution::of(points);

    let hulls = SphericalVoronoi::generate(pd);
    dbg!(hulls);
}
