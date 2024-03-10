mod impls;

use super::super::Triangle;
use crate::planet::shared::{
    point::{DefaultMeasureValue, Point},
    vector::{ui::line::Line, Number},
};
use std::{fmt::Debug, rc::Rc};

#[derive(Debug, Clone)]
pub struct Rectangle<T = DefaultMeasureValue, const N: usize = 2> {
    a: Rc<Triangle<T, N>>,
    b: Rc<Triangle<T, N>>,
}
use std::hash::Hash;

impl<T: PartialEq + Eq + Ord + Number + Hash, const N: usize> Rectangle<T, N> {
    pub fn reverse_tries(&self) -> Rectangle<T, N> {
        let line = self.get_common_line();
        let points = self.get_align_points();
        Rectangle::from([points[0], line.a.0, points[1], line.b.0])
    }

    fn get_common_line(&self) -> Line<T, N> {
        let (a_lines, b_lines): ([Line<T, N>; 3], [Line<T, N>; 3]) = ((*self.a).clone().into(), (*self.b).clone().into());
        let line = a_lines
            .into_iter()
            .find(|line| b_lines.contains(line))
            .unwrap();
        line
    }

    fn get_align_points(&self) -> [Point<T, N>; 2] {
        let line = self.get_common_line();
        let points: [Point<T, N>; 4] = (*self).clone().into();
        points
            .into_iter()
            .filter(|point| !Point::from(line.clone()).contains(point))
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
