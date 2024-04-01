use crate::{
    planet::shared::{
        point::Point,
        vector::{ui::line::Line, Number, Vector},
    },
    traits::of_to::{Of, To},
};
use std::rc::Rc;

// from Vector
impl<T: Number, const N: usize> Of<Vec<Rc<Vector<T, N>>>> for Vec<Rc<Line<T, N>>> {
    fn of(vecs: Vec<Rc<Vector<T, N>>>) -> Self {
        vecs[0..vecs.len() - 1]
            .iter()
            .enumerate()
            .map(|(i, vector)| Rc::new(Line::of([&vector, &vecs[(i + 1) % vecs.len()]])))
            .collect()
    }
}

impl<T: Copy, const N: usize> Of<[Vector<T, N>; 2]> for Line<T, N> {
    fn of(vecs: [Vector<T, N>; 2]) -> Self {
        Self {
            a: Rc::new(vecs[0]),
            b: Rc::new(vecs[1]),
        }
    }
}

impl<T: Copy, const N: usize> Of<[Rc<Vector<T, N>>; 2]> for Line<T, N> {
    fn of(vecs: [Rc<Vector<T, N>>; 2]) -> Self {
        Self {
            a: vecs[0].clone(),
            b: vecs[1].clone(),
        }
    }
}

impl<T: Copy, const N: usize> Of<[&Rc<Vector<T, N>>; 2]> for Line<T, N> {
    fn of(vecs: [&Rc<Vector<T, N>>; 2]) -> Self {
        Self {
            a: vecs[0].clone(),
            b: vecs[1].clone(),
        }
    }
}

// from Point
impl<T: Number, const N: usize> Of<[Point<T, N>; 2]> for Line<T, N> {
    fn of(points: [Point<T, N>; 2]) -> Self {
        Self {
            a: Rc::new(points[0].to()),
            b: Rc::new(points[1].to()),
        }
    }
}

// for Vector
impl<T: Copy, const N: usize> Of<Line<T, N>> for [Vector<T, N>; 2] {
    fn of(line: Line<T, N>) -> Self {
        [*line.a, *line.b]
    }
}

impl<T, const N: usize> Of<Line<T, N>> for [Rc<Vector<T, N>>; 2] {
    fn of(line: Line<T, N>) -> Self {
        [line.a, line.b]
    }
}

impl<T, const N: usize> Of<&Line<T, N>> for [Rc<Vector<T, N>>; 2] {
    fn of(line: &Line<T, N>) -> Self {
        [line.a.clone(), line.b.clone()]
    }
}

impl<T, const N: usize> Of<Rc<Line<T, N>>> for [Rc<Vector<T, N>>; 2] {
    fn of(line: Rc<Line<T, N>>) -> Self {
        [line.a.clone(), line.b.clone()]
    }
}

impl<T: Number, const N: usize> Of<Line<T, N>> for Vector<T, N> {
    fn of(line: Line<T, N>) -> Self {
        *line.b - *line.a
    }
}

impl<T: Number, const N: usize> Of<&Line<T, N>> for Vector<T, N> {
    fn of(line: &Line<T, N>) -> Self {
        *line.b - *line.a
    }
}

impl<T: Number, const N: usize> Of<Rc<Line<T, N>>> for Vector<T, N> {
    fn of(line: Rc<Line<T, N>>) -> Self {
        *line.b - *line.a
    }
}

impl<T: Number, const N: usize> Of<&Rc<Line<T, N>>> for Vector<T, N> {
    fn of(line: &Rc<Line<T, N>>) -> Self {
        *line.b - *line.a
    }
}

impl<T: Copy + Number, const N: usize> Of<&Vec<Rc<Line<T, N>>>> for Vec<Rc<Vector<T, N>>> {
    fn of(triangles: &Vec<Rc<Line<T, N>>>) -> Self {
        (*triangles).clone()
            .into_iter()
            .map(|triangle| triangle.to::<[Rc<Vector<T, N>>; 2]>())
            .flatten()
            .fold(vec![], |mut acc, curr| {
                if !acc.contains(&curr) {
                    acc.push(curr);
                }
                acc
            })
    }
}

// for Point
impl<T: Copy, const N: usize> Of<Line<T, N>> for [Point<T, N>; 2] {
    fn of(line: Line<T, N>) -> Self {
        let vecs: [Rc<Vector<T, N>>; 2] = line.to();
        [(*vecs[0]).to(), (*vecs[1]).to()]
    }
}