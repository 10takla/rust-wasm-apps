mod impls;

use super::super::Triangle;
use crate::planet::shared::{
    point::Point,
    vector::{ui::line::Line, Vector},
};
use std::{fmt::Debug, iter};

#[derive(Debug)]
pub struct Rectangle<T> {
    a: Triangle<T>,
    b: Triangle<T>,
}

impl<T: PartialEq + Copy + Eq + Debug> Rectangle<T> {
    fn get_common_line(&self) -> Line<T> {
        let (a_lines, b_lines): ([Line<T>; 3], [Line<T>; 3]) = (self.a.into(), self.b.into());
        let line = a_lines
            .into_iter()
            .find(|line| b_lines.contains(line))
            .unwrap();
        line
    }
    pub fn reverse_tries(&self) {
        let line = self.get_common_line();
        // let points: [Point; 4] = self.into();
    }
}

#[test]
fn rec() {
    let rec: Rectangle<i32> = dbg!(Rectangle::from([[0; 2], [1; 2], [2; 2], [3; 2]]));
    let points: [Point<i32>; 4] = rec.into();
}
