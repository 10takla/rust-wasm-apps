#[cfg(test)]
mod tests;

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
                Vector,
            },
        },
    },
    traits::of_to::{Of, To},
};
use std::rc::Rc;

pub trait Delone {
    fn triangulate(&self) -> Vec<Rc<Triangle>>;
    fn get_start_point(&self) -> Vec<Rectangle>;
    fn get_scope_lines(lines: Vec<Rc<Line>>, next_vec: &Rc<Vector>) -> Vec<Rc<Line>>;
}

impl Delone for PointDistribution {
    fn triangulate(&self) -> Vec<Rc<Triangle>> {
        let sorted_pd = self.clone().sort_points_by_min();
        let mut rects = sorted_pd.get_start_point();

        for vector in sorted_pd.clone().iter().skip(4) {
            Self::get_next_point(vector, &mut rects);
        }

        rects
            .into_iter()
            .map(|rect| rect.to::<[Rc<Triangle>; 2]>())
            .flatten()
            .fold(vec![], |mut acc, curr| {
                if !acc.contains(&curr) {
                    acc.push(curr);
                }
                acc
            })
    }

    fn get_start_point(&self) -> Vec<Rectangle> {
        let start_tri = Triangle::of(self[0..=2].to_vec());

        let next_vec = self[3].clone();
        let scope_lines = Self::get_scope_lines(
            PointDistribution::of(start_tri.clone().to::<[Rc<Vector>; 3]>().clone().to_vec())
                .convex_hull(),
            &next_vec,
        );

        let mut rects: Vec<Rectangle> = vec![];

        // проведение треугольников к полю зрения
        if scope_lines.len() == 1 {
            let tri = Triangle::of([&scope_lines[0].a, &scope_lines[0].b, &next_vec]);
            let mut rect = Rectangle::of([&start_tri, &tri]);
            let v1 = start_tri.get_circle();
            if (*next_vec - *v1.center).radius() < v1.radius() {
                rect.reverse_tries();
            }
            rects.push(rect);
        } else if scope_lines.len() > 1 {
            let tri = Triangle::of([&scope_lines[0].a, &scope_lines[0].b, &next_vec]);
            let rect = Rectangle::of([&start_tri, &tri]);
            rects.push(rect);
            let tri = Triangle::of([&scope_lines[1].a, &scope_lines[1].b, &next_vec]);
            let rect = Rectangle::of([&start_tri, &tri]);
            rects.push(rect);
        }
        rects
    }

    fn get_scope_lines(lines: Vec<Rc<Line>>, next_vec: &Rc<Vector>) -> Vec<Rc<Line>> {
        let vecs = lines
            .into_iter()
            .map(|line| line.a.clone())
            .collect::<Vec<Rc<Vector>>>();

        let (center_ind, neares_vector) = {
            let neares_vector = vecs.nearest(next_vec);
            (
                vecs.clone()
                    .into_iter()
                    .position(|vector| vector == neares_vector)
                    .unwrap(),
                neares_vector,
            )
        };

        let [right, left] = {
            let maybe_right = vecs[center_ind + 1].clone();
            let angle = Angle::of([&next_vec, &neares_vector, &maybe_right]).get_polar_angle();
            if angle > 0.0 {
                [1_isize, -1]
            } else {
                [1, -1]
            }
        };

        let get_polar_angle = |center: &Rc<Vector>, right: &Rc<Vector>| {
            Angle::of([next_vec, center, right]).get_polar_angle()
        };

        let mut scope_vecs = vec![];
        for i in 0..vecs.len() - 1 {
            let len = vecs.len() as isize;
            let next_ind = (i as isize * left + center_ind as isize + len) % len;
            let next_id = (next_ind + left + len) % len;
            let [next_ind, next_id] = [next_ind as usize, next_id as usize];

            let angle = get_polar_angle(&vecs[next_ind], &vecs[next_id]);

            if angle < 0.0 {
                scope_vecs.push(vecs[next_id].clone());
            } else {
                break;
            }
        }
        scope_vecs.reverse();
        scope_vecs.push(neares_vector.clone());

        for i in 0..vecs.len() - 1 {
            let len = vecs.len() as isize;
            let next_ind = (i as isize * right + center_ind as isize + len) % len;
            let next_id = (next_ind + right + len) % len;
            let [next_ind, next_id] = [next_ind as usize, next_id as usize];

            let angle = get_polar_angle(&vecs[next_ind], &vecs[next_id]);
            if angle > 0.0 {
                scope_vecs.push(vecs[next_id].clone());
            } else {
                break;
            }
        }

        scope_vecs.to()
    }
}

impl PointDistribution {
    fn get_next_point(vector: &Rc<Vector>, rects: &mut Vec<Rectangle>) {
        let scope_lines = {
            let vecs = rects
                .clone()
                .into_iter()
                .map(|rect| rect.to::<[Rc<Vector>; 4]>())
                .flatten()
                .collect::<Vec<Rc<Vector>>>();
            Self::get_scope_lines(PointDistribution::of(vecs).convex_hull(), &vector)
        };

        

        scope_lines.iter().for_each(|boundary_line| {
            let find_tri = (*rects).search::<Vec<Rc<Triangle>>>(boundary_line)[0].clone();

            // fn recursive(rects: &Vec<Rectangle>, find_tri: &Rc<Triangle>, line: &Rc<Line>) {
            //     let any_lines = any_lines(find_tri, line);

            //     any_lines.map(|line| recursive(rects, &find_triangle(rects, &line), &line));
            // }

            let mut rect = Rectangle::of([
                &find_tri,
                &Rc::new(Triangle::of([&boundary_line.a, &boundary_line.b, &vector])),
            ]);

            let circle = find_tri.get_circle();
            if (**vector - *circle.center).radius() < circle.radius() {
                find_tri
                    .alien::<[Rc<Line>; 2]>(&boundary_line)
                    .map(|line| {
                        if let Some(rect) = rects
                            .into_iter()
                            .find(|rect| line.eq(&rect.get_common_line()))
                        {
                            let triangle = (*rect).alien::<Rc<Triangle>>(&find_tri);
                            *rect = Rectangle::of([
                                &triangle,
                                &Rc::new(Triangle::of([&line.a, &line.b, &vector])),
                            ]);
                        }
                    });
            }
            rect.set_delone();
            rects.push(rect);
        });
    }
}
