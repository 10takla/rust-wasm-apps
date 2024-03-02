mod impls;

use super::super::Triangle;
use crate::planet::shared::{
    point::Point,
    vector::{ui::line::Line, Vector},
};
use std::{fmt::Debug, iter};

#[derive(Debug)]
pub struct Rectangle<'a, T> {
    a: Triangle<'a, T>,
    b: Triangle<'a, T>,
}

impl<'a, T: PartialEq + Copy + Eq + Debug> Rectangle<'a, T> {
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
    let vecs = [[0; 2], [1; 2], [2; 2], [3; 2]]
        .into_iter()
        .map(|p| Vector::<i32>::from(p))
        .collect::<Vec<Vector<i32>>>();
    let vecs: [&Vector<i32>; 4] = vecs.iter().collect::<Vec<&Vector<i32>>>().try_into().unwrap();

    let rec: Rectangle<i32> = dbg!(Rectangle::from(vecs));
    let points: [&Point<i32>; 4] = rec.into();
}
