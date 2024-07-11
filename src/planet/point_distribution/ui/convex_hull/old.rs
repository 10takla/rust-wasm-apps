// #[cfg(test)]
// mod tests;
// mod wasm;

// use crate::planet::shared::vector::ui::line::LineType;
// use crate::planet::shared::vector::VectorType;
// use crate::{
//     planet::{
//         point_distribution::PointDistribution,
//         shared::vector::{
//             ui::line::{ui::angle::Angle, Line},
//             Number, Vector,
//         },
//     },
//     traits::of_to::{Of, To},
// };
// use std::{cmp::Ordering, ops::Deref, rc::Rc};

// pub trait ConvexesHull<T, const N: usize> {
//     fn convex_hull(&mut self) -> Vec<LineType<T, N>>;
// }

// impl<T: Number, const N: usize> ConvexesHull<T, N> for PointDistribution<T, N> {
//     fn convex_hull(&mut self) -> Vec<LineType<T, N>> {
//         if self.len() == 0 {
//             return vec![];
//         }

//         if self.len() < 3 {
//             return PointDistribution::point_chain(&self.0);
//         }

//         let mut hull_edges: Vec<VectorType<T, N>> = vec![];
        
//         loop {
//             let finded_p = if hull_edges.len() == 0 {
//                 self.get_min_point()
//             } else {
//                 self.get_next_point_convex_hull(&hull_edges)
//             };

//             if hull_edges.len() > 1 && Rc::ptr_eq(&finded_p, &hull_edges[0]) {
//                 break PointDistribution::point_chain(&hull_edges);
//             } else {
//                 hull_edges.push(finded_p);
//             }
//         }
//     }
// }

// impl<T: Number, const N: usize> PointDistribution<T, N> {
//     fn point_chain(hull_edges: &Vec<VectorType<T, N>>) -> Vec<LineType<T, N>> {
//         let mut lines = hull_edges.clone().to::<Vec<LineType<T, N>>>();
//         lines.push(Rc::new(Line::of([
//             &hull_edges[hull_edges.len() - 1],
//             &hull_edges[0],
//         ])));
//         lines
//     }

//     fn get_next_point_convex_hull(
//         &mut self,
//         hull_edges: &Vec<VectorType<T, N>>,
//     ) -> VectorType<T, N> {
//         if hull_edges.len() >= 2 {
//             self.find_vector_by_angle(hull_edges)
//         } else {
//             self.get_nearest_extreme_point(hull_edges)
//         }
//         .clone()
//     }

//     fn get_angle(&self, hull_edges: &Vec<VectorType<T, N>>, vector: &VectorType<T, N>) -> T {
//         Angle::of([
//             &hull_edges[hull_edges.len() - 2],
//             &hull_edges[hull_edges.len() - 1],
//             &vector,
//         ])
//         .get_angle()
//     }

//     fn find_vector_by_angle(&self, hull_edges: &Vec<VectorType<T, N>>) -> VectorType<T, N> {
//         let vector = self
//             .iter()
//             .filter(|&i| {
//                 if hull_edges.len() >= 3 {
//                     Rc::ptr_eq(i, &hull_edges[0]) || !hull_edges.iter().any(|e| Rc::ptr_eq(e, i))
//                 } else {
//                     !hull_edges.contains(&i)
//                 }
//             })
//             .max_by(|&b, &c| {
//                 let [b_angle, c_angle] = [b, c].map(|v| self.get_angle(hull_edges, v));
//                 let t = b_angle.partial_cmp(&c_angle).unwrap();
//                 // dbg!((b, c, t));
//                 if let Ordering::Equal = t {
//                     return Ordering::Greater;
//                 }
//                 t
//             })
//             .unwrap();
//         (*vector).clone()
//     }

//     fn get_nearest_extreme_point(
//         &mut self,
//         hull_edges: &Vec<VectorType<T, N>>,
//     ) -> VectorType<T, N> {
//         let next_vec = hull_edges[hull_edges.len() - 1].clone();

//         self.sort_by(|b, c| {
//             if Rc::eq(c, &next_vec) {
//                 return Ordering::Greater;
//             }
//             let [b_angle, c_angle] = [b, c].map(|v| (**v - *next_vec).atan());
//             b_angle.partial_cmp(&c_angle).unwrap()
//         });
//         self[1].clone()
//     }
// }
