mod impls;

use super::super::Triangle;
use crate::{planet::shared::{
    point::{DefaultMeasureValue, Point},
    vector::{ui::line::Line, Number},
}, traits::of_to::To};
use std::{fmt::Debug, rc::Rc};
use crate::traits::of_to::Of;

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
        Rectangle::of([points[0], line.a.0, points[1], line.b.0])
    }

    fn get_common_line(&self) -> Line<T, N> {
        let [a_lines, b_lines]: [[Line<T, N>; 3]; 2] = [(*self.a).clone().to(), (*self.b).clone().to()];
        let line = a_lines
            .into_iter()
            .find(|line| b_lines.contains(line))
            .unwrap();
        line
    }

    fn get_align_points(&self) -> [Point<T, N>; 2] {
        let line = self.get_common_line();
        let points: [Point<T, N>; 4] = (*self).clone().to();
        points
            .into_iter()
            .filter(|point| !Point::of(line.clone()).contains(point))
            .collect::<Vec<Point<T, N>>>()
            .try_into()
            .unwrap()
    }
}

#[test]
fn rec() {
    let rec: Rectangle<i32> = Rectangle::of([[0; 2], [1; 2], [2; 2], [3; 2]]);
    let rec = rec.reverse_tries();
    assert_eq!(rec.to::<[Point<i32>; 4]>(), [[0; 2], [1; 2], [3; 2], [2; 2]]);
}
