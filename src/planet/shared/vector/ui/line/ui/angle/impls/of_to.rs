/* -------FROM------- */

use std::rc::Rc;

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

// from Line
impl<T: Copy, const N: usize> Of<[Line<T, N>; 2]> for Angle<T, N> {
    fn of(lines: [Line<T, N>; 2]) -> Self {
        Self {
            ba: Rc::new(lines[0].clone()),
            bc: Rc::new(lines[1].clone()),
        }
    }
}

impl<T: Copy, const N: usize> Of<[Rc<Line<T, N>>; 2]> for Angle<T, N> {
    fn of(lines: [Rc<Line<T, N>>; 2]) -> Self {
        Self {
            ba: lines[0].clone(),
            bc: lines[1].clone(),
        }
    }
}

// from Vector
impl<T: Copy, const N: usize> Of<[Vector<T, N>; 3]> for Angle<T, N> {
    fn of(vecs: [Vector<T, N>; 3]) -> Self {
        let [a, b, c] = [Rc::new(vecs[0]), Rc::new(vecs[1]), Rc::new(vecs[2])];
        Self {
            ba: Rc::new([b.clone(), a.clone()].to()),
            bc: Rc::new([b.clone(), c.clone()].to()),
        }
    }
}

impl<T: Copy, const N: usize> Of<[Rc<Vector<T, N>>; 3]> for Angle<T, N> {
    fn of(vecs: [Rc<Vector<T, N>>; 3]) -> Self {
        let [a, b, c] = [vecs[0].clone(), vecs[1].clone(), vecs[2].clone()];
        Self {
            ba: Rc::new([b.clone(), a.clone()].to()),
            bc: Rc::new([b.clone(), c.clone()].to()),
        }
    }
}

impl<T: Copy, const N: usize> Of<[&Rc<Vector<T, N>>; 3]> for Angle<T, N> {
    fn of(vecs: [&Rc<Vector<T, N>>; 3]) -> Self {
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
impl<T: Clone, const N: usize> Of<Angle<T, N>> for [Line<T, N>; 2] {
    fn of(angle: Angle<T, N>) -> Self {
        [(*angle.ba).clone(), (*angle.bc).clone()]
    }
}

impl<T: Copy, const N: usize> Of<Rc<Angle<T, N>>> for [Line<T, N>; 2] {
    fn of(angle: Rc<Angle<T, N>>) -> Self {
        [(*angle.ba).clone(), (*angle.bc).clone()]
    }
}

impl<T, const N: usize> Of<Angle<T, N>> for [Rc<Line<T, N>>; 2] {
    fn of(angle: Angle<T, N>) -> Self {
        [angle.ba, angle.bc]
    }
}

impl<T: Copy, const N: usize> Of<&Angle<T, N>> for [Rc<Line<T, N>>; 2] {
    fn of(angle: &Angle<T, N>) -> Self {
        [angle.ba.clone(), angle.bc.clone()]
    }
}

impl<T: Copy + Clone, const N: usize> Of<Rc<Angle<T, N>>> for [Rc<Line<T, N>>; 2] {
    fn of(angle: Rc<Angle<T, N>>) -> Self {
        [angle.ba.clone(), angle.bc.clone()]
    }
}

// for Vector
impl<T: Copy, const N: usize> Of<Angle<T, N>> for [Vector<T, N>; 3] {
    fn of(angle: Angle<T, N>) -> Self {
        [*angle.ba.b, *angle.ba.a, *angle.bc.b]
    }
}

impl<T: Copy, const N: usize> Of<&Angle<T, N>> for [Vector<T, N>; 3] {
    fn of(angle: &Angle<T, N>) -> Self {
        [*angle.ba.b, *angle.ba.a, *angle.bc.b]
    }
}

impl<T: Copy, const N: usize> Of<Angle<T, N>> for [Rc<Vector<T, N>>; 3] {
    fn of(angle: Angle<T, N>) -> Self {
        [angle.ba.b.clone(), angle.ba.a.clone(), angle.bc.b.clone()]
    }
}

impl<T: Copy, const N: usize> Of<Rc<Angle<T, N>>> for [Vector<T, N>; 3] {
    fn of(angle: Rc<Angle<T, N>>) -> Self {
        [*angle.ba.b, *angle.ba.a, *angle.bc.b]
    }
}

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

impl<T: Copy, const N: usize> Of<&Angle<T, N>> for [Point<T, N>; 3] {
    fn of(angle: &Angle<T, N>) -> Self {
        [(*angle.ba.b).to(), (*angle.ba.a).to(), (*angle.bc.b).to()]
    }
}
