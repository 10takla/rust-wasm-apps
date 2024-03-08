use super::Vector;
use std::cmp::Ordering;

// impl<T: PartialOrd> PartialOrd for Vector<T, N> {
//     fn partial_cmp(&self, other: &Vector<T, N>) -> Option<Ordering>{
//         let mut merge_vecs = self.iter().zip(other.iter());
//         if let None = merge_vecs.find(|(v1, v2)| !(v1 == v2)) {
//             return Some(Ordering::Equal);
//         }
//         if let None = merge_vecs.find(|(v1, v2)| !(v1 < v2)) {
//             return Some(Ordering::Less);
//         }
//         if let None = merge_vecs.find(|(v1, v2)| !(v1 > v2)) {
//             return Some(Ordering::Greater);
//         }
//         None
//     }
// }