#[cfg(test)]
mod tests;

use crate::planet::{point_distribution::PointDistribution, shared::vector::{ui::line::ui::angle::Angle, Number}};

impl<T: Number, const N: usize> PointDistribution<T, N> {
    pub fn convex_hull(&self) -> Vec<usize> {
        let mut hull_edges = vec![];
        loop {
            if self.len() == 0 || (hull_edges.len() > 1 && hull_edges[hull_edges.len() - 1] == hull_edges[0]) {
                break hull_edges;
            }
            let finded_p = {
                if hull_edges.len() == 0 {
                    self.get_min_point()
                } else {
                    self.get_next_point(&hull_edges)
                }
            };
            hull_edges.push(finded_p);
        }
    }

    fn get_next_point(&self, hull_edges: &Vec<usize>) -> usize {
        let hull_edges_len = hull_edges.len();
        let n_p_i = hull_edges[hull_edges_len - 1];
        let n_p = self[n_p_i];

        let points_width_ids = self.iter();

        let next_p = {
            if hull_edges_len >= 2 {
                points_width_ids
                    .enumerate()
                    .filter(|&(i, _)| {
                        if hull_edges_len >= 3 {
                            i == hull_edges[0] || !hull_edges.contains(&i)
                        } else {
                            !hull_edges.contains(&i)
                        }
                    })
                    .max_by(|&(b_i, _), &(c_i, _)| {
                        let (b_angle, c_angle) = (self.get_angle(hull_edges, b_i), self.get_angle(hull_edges, c_i));
                        b_angle.partial_cmp(&c_angle).unwrap()
                    })
                    .unwrap()
            } else {
                points_width_ids
                    .enumerate()
                    .filter(|&(i, _)| i != n_p_i)
                    .min_by(|&(_, &b), &(_, &c)| {
                        let [b_angle, c_angle] = [
                            (b - n_p).atan(),
                            (c - n_p).atan(),
                        ];
                        b_angle.partial_cmp(&c_angle).unwrap()
                    })
                    .unwrap()
            }
        };
        next_p.0
    }
    fn get_angle(&self, hull_edges: &Vec<usize>, p_i: usize) -> T {
        Angle::from([
            self[hull_edges[hull_edges.len() - 2]],
            self[hull_edges[hull_edges.len() - 1]],
            self[p_i],
        ]).get_angle()
    }
}