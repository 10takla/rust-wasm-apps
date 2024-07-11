use crate::planet::shared::vector::ui::line::LineType;
use crate::planet::shared::vector::VectorType;
use crate::{
    planet::shared::{
        point::Point,
        vector::{
            ui::line::{ui::angle::Angle, Line},
            Number, Vector,
        },
    },
    traits::of_to::{Of, To},
};
use macros::of_to;
use std::rc::Rc;

// from Line
#[of_to]
impl<T: Number, const N: usize> Of<[LineType<T, N>; 2]> for Angle<T, N> {
    fn of(lines: [LineType<T, N>; 2]) -> Self {
        Self {
            ba: lines[0].clone(),
            bc: lines[1].clone(),
        }
    }
}

// from Vector
#[of_to]
impl<T: Number, const N: usize> Of<[Rc<Vector<T, N>>; 3]> for Angle<T, N> {
    fn of(vecs: [Rc<Vector<T, N>>; 3]) -> Self {
        let [a, b, c] = [vecs[0].clone(), vecs[1].clone(), vecs[2].clone()];
        Self {
            ba: Rc::new([b.clone(), a.clone()].to()),
            bc: Rc::new([b.clone(), c.clone()].to()),
        }
    }
}

// from Point
impl<T: Number, const N: usize> Of<[Point<T, N>; 3]> for Angle<T, N> {
    fn of(points: [Point<T, N>; 3]) -> Self {
        let [a, b, c] = [points[0], points[1], points[2]];
        Self {
            ba: Rc::new([b, a].to()),
            bc: Rc::new([b, c].to()),
        }
    }
}

/* -------FOR------- */

// for Line
#[of_to]
impl<T: Copy + Clone, const N: usize> Of<Rc<Angle<T, N>>> for [Rc<Line<T, N>>; 2] {
    fn of(angle: Rc<Angle<T, N>>) -> Self {
        [angle.ba.clone(), angle.bc.clone()]
    }
}

// for Vector
#[of_to]
impl<T: Copy, const N: usize> Of<Rc<Angle<T, N>>> for [Rc<Vector<T, N>>; 3] {
    fn of(angle: Rc<Angle<T, N>>) -> Self {
        [angle.ba.b.clone(), angle.ba.a.clone(), angle.bc.b.clone()]
    }
}

// for Point
impl<T: Copy, const N: usize> Of<Angle<T, N>> for [Point<T, N>; 3] {
    fn of(angle: Angle<T, N>) -> Self {
        [(*angle.ba.b).to(), (*angle.ba.a).to(), (*angle.bc.b).to()]
    }
}
