use crate::planet::shared::{
    traits::{Find, Has},
    vector::{
        ui::line::{ui::angle::ui::triangle::Triangle, Line},
        Number,
    },
};
use std::rc::Rc;
use crate::planet::shared::vector::ui::line::LineType;

impl<T: Number, const N: usize> Find<Vec<Rc<Triangle<T, N>>>, LineType<T, N>>
    for Vec<Rc<Triangle<T, N>>>
{
    fn find(&self, line: &LineType<T, N>) -> Vec<Rc<Triangle<T, N>>> {
        self.into_iter()
            .filter(|&triangle| triangle.has(line))
            .map(|t| (*t).clone())
            .collect()
    }
}
