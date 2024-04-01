use std::rc::Rc;
use crate::{planet::shared::vector::{ui::line::ui::angle::{ui::triangle::Triangle, Angle}, Number}, traits::of_to::To};

impl<T: PartialEq + Number, const N: usize> PartialEq for Triangle<T, N> {
    fn eq(&self, other: &Self) -> bool {
        let mut other_angles = other.to::<[Rc<Angle<T, N>>; 3]>().to_vec();

        self.to::<[Rc<Angle<T, N>>; 3]>().into_iter().all(|angle| {
            if let Some(i) = other_angles.iter().position(|other_angle| {
                Rc::ptr_eq(other_angle, &angle)
            }) {
                other_angles.remove(i);
                return true;
            } else{
                return false
            }
        })
    }
}