mod impls;

use crate::{derive_deref, planet::shared::{point::Point, vector::ui::line::Line}};
use super::super::Triangle;

pub struct Plane<T> {
    a: Triangle<T>,
    b: Triangle<T>
}

impl<T: PartialEq + Copy> Plane<T> {
    // fn get_common_line(&self) -> Line<T> {
    //     let (a_lines, b_lines): ([Line<T>; 3], [Line<T>; 3]) = (self.a.into(), self.b.into());
    //     let line = a_lines.into_iter().find(|line| b_lines.contains(line)).unwrap();
    //     line
    // }
    // pub fn reverse_tries(&self) {
    //     let line = self.get_common_line();
    //     // let points: [Point; 4] = self.into();
    // }
}