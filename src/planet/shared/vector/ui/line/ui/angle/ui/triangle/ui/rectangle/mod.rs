mod impls;

use super::super::Triangle;
use crate::planet::shared::{
    point::{DefaultMeasureValue, Point},
    vector::{ui::line::Line, Number},
};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle<T = DefaultMeasureValue, const N: usize = 2> {
    a: Triangle<T, N>,
    b: Triangle<T, N>,
}
use std::hash::Hash;

impl<T: PartialEq + Eq + Debug + Ord + Number + Hash, const N: usize> Rectangle<T, N> {
    pub fn reverse_tries(&self) -> Rectangle<T, N> {
        let line = self.get_common_line();
        let points = self.get_align_points();
        Rectangle::from([points[0], *line.a, points[1], *line.b])
    }

    fn get_common_line(&self) -> Line<T, N> {
        let (a_lines, b_lines): ([Line<T, N>; 3], [Line<T, N>; 3]) = (self.a.into(), self.b.into());
        let line = a_lines
            .into_iter()
            .find(|line| b_lines.contains(line))
            .unwrap();
        line
    }

    fn get_align_points(&self) -> [Point<T, N>; 2] {
        let line = self.get_common_line();
        let points: [Point<T, N>; 4] = (*self).into();
        points
            .into_iter()
            .filter(|point| !Point::from(line).contains(point))
            .collect::<Vec<Point<T, N>>>()
            .try_into()
            .unwrap()
    }
}

#[test]
fn rec() {
    let rec: Rectangle<i32> = Rectangle::from([[0; 2], [1; 2], [2; 2], [3; 2]]);
    let rec = rec.reverse_tries();
    assert_eq!(<Rectangle<i32> as Into<[Point<i32>; 4]>>::into(rec), [[0; 2], [1; 2], [3; 2], [2; 2]]);
}
