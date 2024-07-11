#[cfg(test)]
mod tests;
pub mod wasm;

use std::rc::Rc;
use crate::{
        planet::{
        point_distribution::{wasm::PointDistribution as PDWasm, PointDistribution},
        shared::{
            point::{DefaultMeasureValue, DEFAULT_MEASURE, Point},
            traits::{Alien, Has, Indices, Normalize, SvgStyle},
            vector::{
                svg::VectorStyle,
                ui::line::{
                    svg::LineStyle,
                    ui::angle::{ui::triangle::ui::hull::Hull, Angle},
                    Line, LineType,
                },
                Number, VectorType,
            },
        },
    },
    traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
    },
    utils::svg::draw_svg,
};
use macros::Deref;
use rand::{thread_rng, Rng};
use serde_wasm_bindgen::to_value;
use spherical_voronoi::{build, Point as Point3, Visitor};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_test::{console_log, wasm_bindgen_test};

#[derive(Default, Clone, Debug)]
struct SphericalVoronoi {
    pd: PointDistribution<f64, 3>,
    edges: Vec<LineType<f64, 3>>,
    other: (Vec<[usize; 3]>, usize),
}

impl SphericalVoronoi {

    pub fn generate(pd: PointDistribution<f64, 3>) -> Vec<Hull<f64, 3>> {
        Cells(Self::get_edges(pd)).find_cells()
    }

    pub fn get_edges(mut pd: PointDistribution<f64, 3>) -> Vec<Rc<Line<f64, 3>>> {
        let mut sv = SphericalVoronoi::default();

        build(
            &pd.iter_mut()
                .map(|vector| {
                    let mut v = (**vector).clone();
                    v.normalize();
                    Point3::new(v[0], v[1], v[2])
                })
                .collect::<Vec<Point3>>()[..],
            &mut sv,
        );

        sv.edges.clone()
    }

    pub fn new(mut pd: PointDistribution<f64, 3>) -> Self {
        let mut sv = SphericalVoronoi::default();

        build(
            &pd.iter_mut()
                .map(|vector| {
                    let mut v = (**vector).clone();
                    v.normalize();
                    Point3::new(v[0], v[1], v[2])
                })
                .collect::<Vec<Point3>>()[..],
            &mut sv,
        );

        sv
    }
}

impl Visitor for SphericalVoronoi {
    fn vertex(&mut self, point: Point3, cells: [usize; 3]) {
        self.other.0.push(cells);
        console_log!("{:?} {:?}", point, cells);
        self.pd.push(VectorType::of([point.x, point.y, point.z]))
    }
    fn edge(&mut self, vertices: [usize; 2]) {
        console_log!("{:?}", vertices);
        self.edges
            .push(LineType::of(vertices.map(|i| self.pd[i].clone())));
    }
    fn cell(&mut self) {
        self.other.1 += 1;
        console_log!("cell");
    }
}

#[derive(Deref)]
pub struct Cells<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE>(pub Vec<LineType<T, N>>);

impl<T: Number, const N: usize> Cells<T, N> {
    pub fn find_cells(&mut self) -> Vec<Hull<T, N>> {
        let mut cells = vec![];
        let mut lines_stack = vec![self[0].clone()];
        let mut queue = vec![self[0].clone()];

        let mut curr = queue[0].clone();

        while !queue.is_empty() {
            // let y = lines_stack.into_iter().map(|line| line).collect::<Vec<LineType<T>>>()
            // draw_svg(
            //     &VecSvg(vec![
            //         // Box::new(lines_stack.clone()),
            //         Box::new(SvgWithStyle(
            //             lines_stack
            //                 .clone()
            //                 .into_iter()
            //                 .map(|line| line.clone())
            //                 .collect::<Vec<LineType>>(),
            //             LineStyle::default()
            //                 .set_color("yellow")
            //                 .set_vecs(VectorStyle::default().set_color("yellow")),
            //         )),
            //     ]),
            //     "one_line",
            //     module_path!(),
            //     "",
            // );
            // dbg!();
            let finded = {
                let mut common_lines = self
                    .iter()
                    .filter(|&line| *line != curr && line.has(&curr.b))
                    .map(|line| {
                        if !Rc::ptr_eq(&curr.b, &line.a) {
                            return Rc::new(line.reverse());
                        }
                        line.clone()
                    })
                    .collect::<Vec<LineType<T, N>>>();
                if common_lines.len() > 1 {
                    {
                        let get_angle = |line: &LineType<T, N>| {
                            let angle = Angle::of([&curr.a, &curr.b, &line.b]).get_polar_angle();
                            // dbg!(&lines_stack);
                            angle
                        };

                        let is_reverse = {
                            lines_stack.len() > 1 && {
                                let t: [LineType<T, N>; 2] =
                                    lines_stack[..2].to_vec().try_into().unwrap();
                                Angle::of([&lines_stack[0].a, &lines_stack[0].b, &lines_stack[1].b])
                                    .get_polar_angle()
                            } < 0.0.as_()
                        };
                        let top = if lines_stack.len() > 1 {
                            Angle::of([&lines_stack[0].a, &lines_stack[0].b, &lines_stack[1].b])
                                .get_polar_angle()
                        } else {
                            0.0.as_()
                        };
                        dbg!("COMMON LINES");
                        dbg!((&common_lines, is_reverse, top));

                        common_lines.sort_by(|line_a, line_b| {
                            let mut t = get_angle(line_a).partial_cmp(&get_angle(line_b)).unwrap();

                            if is_reverse {
                                if get_angle(line_a) > 0.0.as_() && get_angle(line_b) > 0.0.as_() {
                                    return std::cmp::Ordering::Equal;
                                }
                                if get_angle(line_a) > 0.0.as_() && !(get_angle(line_b) > 0.0.as_())
                                {
                                    return std::cmp::Ordering::Greater;
                                }
                                if !(get_angle(line_a) > 0.0.as_()) && get_angle(line_b) > 0.0.as_()
                                {
                                    return std::cmp::Ordering::Less;
                                }
                                return get_angle(line_b).partial_cmp(&get_angle(line_a)).unwrap();
                            } else {
                                if get_angle(line_a) < 0.0.as_() && get_angle(line_b) < 0.0.as_() {
                                    return std::cmp::Ordering::Equal;
                                }
                                if get_angle(line_a) < 0.0.as_() && !(get_angle(line_b) < 0.0.as_())
                                {
                                    return std::cmp::Ordering::Greater;
                                }
                                if !(get_angle(line_a) < 0.0.as_()) && get_angle(line_b) < 0.0.as_()
                                {
                                    return std::cmp::Ordering::Less;
                                }
                                return get_angle(line_a).partial_cmp(&get_angle(line_b)).unwrap();
                            }
                        });
                    }

                    common_lines[1..].into_iter().for_each(|line| {
                        if cells.iter().flatten().all(|l: &LineType<T, N>| *l != *line) {
                            queue.push(line.clone());
                        }
                    });
                }
                common_lines[0].clone()
            };

            queue = queue.into_iter().filter(|line| *line != finded).collect();

            lines_stack.push(finded.clone());

            let t = lines_stack[0].clone();
            if Rc::ptr_eq(&lines_stack[lines_stack.len() - 1].b, &t.a) {
                cells.push(lines_stack);

                queue = queue.into_iter().filter(|line| *line != t).collect();

                if queue.is_empty() {
                    break;
                }

                curr = queue[0].clone();
                lines_stack = vec![curr.clone()];
                continue;
            }

            curr = finded;
        }

        cells.into_iter().map(|cell| Hull::of(cell)).collect()
    }
}
