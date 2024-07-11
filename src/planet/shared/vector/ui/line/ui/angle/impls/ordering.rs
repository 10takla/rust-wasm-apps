use crate::{
    planet::shared::vector::{
        ui::line::{ui::angle::Angle, Line},
        Number,
    },
    traits::of_to::To,
};
use std::rc::Rc;
use crate::planet::shared::vector::ui::line::LineType;

impl<T: PartialEq + Number, const N: usize> PartialEq for Angle<T, N> {
    fn eq(&self, other: &Self) -> bool {
        let mut other_lines = other.to::<[LineType<T, N>; 2]>().to_vec();

        self.to::<[LineType<T, N>; 2]>().into_iter().all(|line| {
            if let Some(i) = other_lines
                .iter()
                .position(|other_line| Rc::ptr_eq(other_line, &line))
            {
                other_lines.remove(i);
                return true;
            } else {
                return false;
            }
        })
    }
}
