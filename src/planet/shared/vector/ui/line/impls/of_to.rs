use macros::of_to;

use crate::planet::shared::vector::ui::line::LineType;
use crate::planet::shared::vector::VectorType;
use crate::{
    planet::shared::{
        point::Point,
        vector::{ui::line::Line, Number, Vector},
    },
    traits::of_to::{Of, To},
};
use std::rc::Rc;

// from Vector
impl<T: Number, const N: usize> Of<Vec<VectorType<T, N>>> for Vec<LineType<T, N>> {
    fn of(vecs: Vec<VectorType<T, N>>) -> Self {
        if vecs.len() <= 1 {
            return vec![];
        }

        vecs[0..vecs.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, vector)| Rc::new(Line::of([&vector, &vecs[(i + 1) % vecs.len()]])))
            .collect()
    }
}

#[of_to]
impl<T: Number, const N: usize> Of<[Rc<Vector<T, N>>; 2]> for Line<T, N> {
    fn of(vecs: [VectorType<T, N>; 2]) -> Self {
        Self {
            a: vecs[0].clone(),
            b: vecs[1].clone(),
        }
    }
}

// from Point
#[of_to]
impl<T: Number, const N: usize> Of<[Point<T, N>; 2]> for Line<T, N> {
    fn of(points: [Point<T, N>; 2]) -> Self {
        Self {
            a: Rc::new(points[0].to()),
            b: Rc::new(points[1].to()),
        }
    }
}

// for LineType
impl<T, const N: usize> Of<Line<T, N>> for LineType<T, N> {
    fn of(line: Line<T, N>) -> Self {
        Rc::new(line)
    }
}

impl<T: Number, const N: usize> Of<&Line<T, N>> for LineType<T, N> {
    fn of(line: &Line<T, N>) -> Self {
        (*line).clone().to()
    }
}

impl<T: Copy + Number, const N: usize> Of<&Vec<Line<T, N>>> for Vec<LineType<T, N>> {
    fn of(lines: &Vec<Line<T, N>>) -> Self {
        lines.into_iter().map(|line| line.to()).collect()
    }
}

impl<T: Copy + Number, const N: usize> Of<&Vec<&Line<T, N>>> for Vec<LineType<T, N>> {
    fn of(lines: &Vec<&Line<T, N>>) -> Self {
        lines.into_iter().map(|&line| line.to()).collect()
    }
}

// for Line
impl<T: Copy + Number, const N: usize> Of<Vec<LineType<T, N>>> for Vec<Line<T, N>> {
    fn of(lines: Vec<LineType<T, N>>) -> Self {
        lines.into_iter().map(|line| (*line).clone()).collect()
    }
}

impl<T: Copy + Number, const N: usize> Of<&Vec<LineType<T, N>>> for Vec<Line<T, N>> {
    fn of(lines: &Vec<LineType<T, N>>) -> Self {
        lines.clone().to()
    }
}

impl<T: Copy + Number, const N: usize> Of<Vec<Line<T, N>>> for Vec<Line<T, N>> {
    fn of(lines: Vec<Line<T, N>>) -> Self {
        lines
            .into_iter()
            .map(|line| Rc::new(line))
            .collect::<Vec<LineType<T, N>>>()
            .to()
    }
}

impl<T: Copy + Number, const N: usize> Of<Vec<&Line<T, N>>> for Vec<Line<T, N>> {
    fn of(lines: Vec<&Line<T, N>>) -> Self {
        lines
            .into_iter()
            .map(|line| line.clone())
            .collect::<Vec<Line<T, N>>>()
            .to()
    }
}

// for Vector
#[of_to]
impl<T: Clone, const N: usize> Of<Line<T, N>> for [Rc<Vector<T, N>>; 2] {
    fn of(line: Line<T, N>) -> Self {
        [line.a, line.b]
    }
}

trait Map<T> {
    type Output;
    fn map(&self, handle: impl Fn(T) -> T) -> Self::Output;
}
impl<T, const N: usize> Map<Rc<Vector<T, N>>> for LineType<T, N> {
    type Output = [VectorType<T, N>; 2];
    fn map(&self, handle:  impl Fn(Rc<Vector<T, N>>) -> Rc<Vector<T, N>>) -> Self::Output {
        [self.a.clone(), self.b.clone()].map(handle)
    }
}

#[of_to]
impl<T: Copy + Number, const N: usize> Of<Vec<LineType<T, N>>> for Vec<VectorType<T, N>> {
    fn of(lines: Vec<LineType<T, N>>) -> Self {
        lines
            .clone()
            .into_iter()
            .map(|triangle| triangle.to::<[VectorType<T, N>; 2]>())
            .flatten()
            .fold(vec![], |mut acc, curr| {
                if !acc.contains(&curr) {
                    acc.push(curr);
                }
                acc
            })
    }
}

impl<T: Copy + Number, const N: usize> Of<Vec<Line<T, N>>> for Vec<VectorType<T, N>> {
    fn of(lines: Vec<Line<T, N>>) -> Self {
        lines
            .into_iter()
            .map(|line| Rc::new(line))
            .collect::<Vec<LineType<T, N>>>()
            .to()
    }
}

impl<T: Copy + Number, const N: usize> Of<Vec<&Line<T, N>>> for Vec<VectorType<T, N>> {
    fn of(lines: Vec<&Line<T, N>>) -> Self {
        lines
            .into_iter()
            .map(|line| line.clone())
            .collect::<Vec<Line<T, N>>>()
            .to()
    }
}

#[of_to]
impl<T: Number, const N: usize> Of<Rc<Line<T, N>>> for Vector<T, N> {
    fn of(line: Rc<Line<T, N>>) -> Self {
        *line.b - *line.a
    }
}

// for Point
#[of_to]
impl<T: Copy, const N: usize> Of<Line<T, N>> for [Point<T, N>; 2] {
    fn of(line: Line<T, N>) -> Self {
        let vecs: [VectorType<T, N>; 2] = line.to();
        [(*vecs[0]).to(), (*vecs[1]).to()]
    }
}


