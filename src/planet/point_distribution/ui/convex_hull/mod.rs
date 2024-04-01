#[cfg(test)]
mod tests;

use std::rc::Rc;

use crate::{
    planet::{
        point_distribution::PointDistribution,
        shared::vector::{
            ui::line::{ui::angle::Angle, Line},
            Number, Vector,
        },
    },
    traits::of_to::{Of, To},
};

pub trait ConvexHull<T, const N: usize> {
    fn convex_hull(&self) -> Vec<Rc<Line<T, N>>>;
    fn get_next_point(&self, hull_edges: &Vec<Rc<Vector<T, N>>>) -> Rc<Vector<T, N>>;
    fn get_angle(&self, hull_edges: &Vec<Rc<Vector<T, N>>>, p_i: &Rc<Vector<T, N>>) -> T;
}

impl<T: Number, const N: usize> ConvexHull<T, N> for PointDistribution<T, N> {
    fn convex_hull(&self) -> Vec<Rc<Line<T, N>>> {
        let mut hull_edges: Vec<Rc<Vector<T, N>>> = vec![];
        loop {
            let finded_p = {
                if hull_edges.len() == 0 {
                    self.get_min_point()
                } else {
                    self.get_next_point(&hull_edges)
                }
            };
            if self.len() == 0 || (hull_edges.len() > 1 && Rc::ptr_eq(&finded_p, &hull_edges[0])) {
                break {
                    let mut lines = hull_edges.clone().to::<Vec<Rc<Line<T, N>>>>();
                    lines.push(Rc::new(Line::of([
                        &hull_edges[hull_edges.len() - 1],
                        &hull_edges[0],
                    ])));
                    lines
                };
            } else {
                hull_edges.push(finded_p);
            }
        }
    }

    fn get_next_point(&self, hull_edges: &Vec<Rc<Vector<T, N>>>) -> Rc<Vector<T, N>> {
        let hull_edges_len = hull_edges.len();
        let n_p_i = hull_edges[hull_edges_len - 1].clone();

        let points_width_ids = self.iter();

        let next_p = {
            if hull_edges_len >= 2 {
                points_width_ids
                    .filter(|&i| {
                        if hull_edges_len >= 3 {
                            Rc::ptr_eq(i, &hull_edges[0])
                                || !hull_edges.iter().any(|e| Rc::ptr_eq(e, i))
                        } else {
                            !hull_edges.contains(&i)
                        }
                    })
                    .max_by(|&b_i, &c_i| {
                        let [b_angle, c_angle] = [
                            self.get_angle(hull_edges, b_i),
                            self.get_angle(hull_edges, c_i),
                        ];
                        b_angle.partial_cmp(&c_angle).unwrap()
                    })
                    .unwrap()
            } else {
                points_width_ids
                    .filter(|&i| !Rc::eq(i, &n_p_i))
                    .min_by(|&b, &c| {
                        let [b_angle, c_angle] = [(**b - *n_p_i).atan(), (**c - *n_p_i).atan()];
                        b_angle.partial_cmp(&c_angle).unwrap()
                    })
                    .unwrap()
            }
        };
        (*next_p).clone()
    }
    fn get_angle(&self, hull_edges: &Vec<Rc<Vector<T, N>>>, p_i: &Rc<Vector<T, N>>) -> T {
        Angle::of([
            &hull_edges[hull_edges.len() - 2],
            &hull_edges[hull_edges.len() - 1],
            &p_i,
        ])
        .get_angle()
    }
}
