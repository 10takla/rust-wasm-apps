use crate::{planet::shared::vector::{ui::line::Line, Vector}, traits::of_to::To};
use std::rc::Rc;

impl<T: PartialEq, const N: usize> PartialEq for Line<T, N> {
    fn eq(&self, other: &Self) -> bool {
        let mut other_vecs = other.to::<[Rc<Vector<T, N>>; 2]>().to_vec();
        
        self.to::<[Rc<Vector<T, N>>; 2]>().into_iter().all(|vec| {
            if let Some(i) = other_vecs
                .iter()
                .position(|other_vec| Rc::ptr_eq(other_vec, &vec))
            {
                other_vecs.remove(i);
                return true;
            } else {
                return false;
            }
        })
    }
}