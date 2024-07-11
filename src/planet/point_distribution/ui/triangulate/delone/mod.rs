#[cfg(test)]
mod tests;

use crate::planet::shared::point::{DefaultMeasureValue, DEFAULT_MEASURE};
use crate::planet::shared::traits::{Find, Svg, SvgStyle};
use crate::planet::shared::vector::svg::VectorStyle;
use crate::planet::shared::vector::ui::line::svg::LineStyle;
use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::svg::TriangleStyle;
use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::ui::hull::Hull;
use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::ui::rectangle::svg::RectangleStyle;
use crate::planet::shared::vector::ui::line::ui::angle::ui::triangle::TriangleType;
use crate::planet::shared::vector::ui::line::LineType;
use crate::planet::shared::vector::VectorType;
use crate::utils::svg::draw_svg;
use crate::{
    planet::{
        point_distribution::{ui::convex_hull::ConvexHull, PointDistribution},
        shared::{
            traits::{Has, Nearest, SuperAlien, SuperFind},
            vector::{
                ui::line::{
                    ui::angle::{
                        ui::triangle::{ui::rectangle::Rectangle, Triangle},
                        Angle,
                    },
                    Line,
                },
                Number, Vector,
            },
        },
    },
    traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
    },
};
use std::cmp::Ordering;
use std::iter::once;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};
use svg::node::Value;

pub trait Delone<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> {
    fn triangulate(&mut self) -> Vec<TriangleType<T>>;
}

impl<T: Number + Into<Value>> Delone<T> for PointDistribution<T> {
    fn triangulate(&mut self) -> Vec<TriangleType<T>> {
        if self.len() < 3 {
            return vec![];
        }

        draw_svg(vec![&self.0], "tmp", module_path!(), "tests");

        self.sort_points_by_min();

        let mut tries = vec![self[0..3].to::<TriangleType<T>>()];

        for i in 3..self.len() {
            let next_vec = &self[i];
            let scope_lines = self.get_scope_lines(self[..i].to_vec().to(), next_vec);

            for line in scope_lines {
                let new_tri = Triangle::of([&line.a, &line.b, next_vec]);
            }

            break;
        }

        tries
    }
}

impl<T: Number, const N: usize> Nearest<Vec<LineType<T, N>>, LineType<T, N>> for VectorType<T, N> {
    fn nearest(&self, lines: &Vec<LineType<T, N>>) -> LineType<T, N> {
        lines
            .into_iter()
            .min_by(|&line_a, &line_b| {
                let length = |ab: &Rc<Line<T, N>>| {
                    let [ae, be]: [Line<T, N>; 2] =
                        [Line::of([&ab.a, self]), Line::of([&ab.b, self])];

                    let [ab_ae, ab_be] = [(**ab).clone() * ae.clone(), (**ab).clone() * be.clone()];
                    if ab_ae < 0.as_::<T>() {
                        ae.length()
                    } else if ab_be > 0.as_::<T>() {
                        be.length()
                    } else {
                        let [c, y] = [ab.to::<Vector<T, N>>(), ae.to::<Vector<T, N>>()];
                        ((c[0] * y[1] - c[1] * y[0]).as_::<f64>().abs().as_::<T>()) / ab.length()
                    }
                };

                let t = length(line_a).partial_cmp(&length(line_b)).unwrap();
                if let Ordering::Equal = t {
                    return length(&Rc::new(line_a.reverse())).partial_cmp(&length(line_b)).unwrap();
                }
                
                t
            })
            .unwrap()
            .clone()
    }
}

#[test]
fn nearest_line() {
    let vecs = vec![[1., 1.], [1.5, 2.], [1., 5.]].to::<Vec<VectorType>>();
    let vector = VectorType::of([2., 3.]);
    let lines: Vec<Rc<Line>> = Hull::of(vecs).0.0;
    let line = vector.nearest(&lines);

    draw_svg(
        vec![
            &SvgStyle(lines.clone(), LineStyle::default()),
            &SvgStyle(line.clone(), LineStyle::default().set_color("white")),
            &SvgStyle(vector.clone(), VectorStyle::default().set_color("red")),
        ],
        "nearest_line",
        module_path!(),
        "tests",
    );
}

impl<T: Number + Into<Value>> PointDistribution<T> {
    fn get_scope_lines(
        &self,
        mut pd: PointDistribution<T>,
        vector: &Rc<Vector<T>>,
    ) -> Vec<LineType<T>> {
        let convex_hull = pd.convex_hull();
        let near_vector = pd.nearest(vector);

        let line = vector.nearest(&convex_hull.0);

        let lines = convex_hull.0.find(&near_vector);
        dbg!(&lines);
        // dbg!(&vector, &near_vector);
        draw_svg(
            vec![
                &self.0,
                &SvgStyle(dbg!(line).clone(), LineStyle::default().set_color("white")),
                &SvgStyle(vector.clone(), VectorStyle::default().set_color("red")),
                &SvgStyle(
                    near_vector.clone(),
                    VectorStyle::default().set_color("yellow"),
                ),
            ],
            "tmp",
            module_path!(),
            "tests",
        );

        vec![]
    }
}
impl<T, const N: usize> Find<Vec<LineType<T, N>>, VectorType<T, N>> for Vec<LineType<T, N>> {
    fn find(&self, vector: &VectorType<T, N>) -> Vec<LineType<T, N>> {
        self.clone()
            .into_iter()
            .filter(|line| line.has(vector))
            .collect()
    }
}
// impl<T: Number, const N: usize> Nearest<VectorType<T, N>, LineType<T, N>> for Vec<LineType<T, N>> {
//     fn nearest(&self, vector: &VectorType<T, N>) -> LineType<T, N> {
//         self.clone()
//             .into_iter()
//             .min_by(|a, b| {
//                 a.0

//                 a.partial_cmp(&b).unwrap()
//             })
//             .unwrap()
//     }
// }
