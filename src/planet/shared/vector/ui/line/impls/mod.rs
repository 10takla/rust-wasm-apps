mod alien;
mod arithmetic;
mod display;
mod nearest;
mod of_to;
mod ordering;

use super::svg::LineStyle;
use super::ui::angle::ui::triangle::TriangleType;
use super::Line;
use crate::planet::shared::point::Point;
use crate::planet::shared::traits::{As, Has, Svg};
use crate::planet::shared::traits::{Indices, Projection};
use crate::planet::shared::vector::ui::line::LineType;
use crate::planet::shared::vector::VectorType;
use crate::planet::shared::vector::{Number, Vector};
use crate::traits::of_to::{Of, To};
use serde::ser::SerializeSeq;
use serde::Serialize;
use std::ops::{Deref, Range, RangeBounds};
use std::rc::Rc;
use svg::node::element::path::{Data, Parameters};
use svg::node::Value;
use svg::Document;
use wasm_bindgen_test::console_log;

// As
impl<F: Number, const N: usize> As for Line<F, N> {
    type Output<I> = Line<I, N>;
    fn as_<I: Number>(&self) -> Self::Output<I> {
        let new_line: [Rc<Vector<I, N>>; 2] = self
            .clone()
            .into_iter()
            .map(|vector| Rc::new(vector.as_()))
            .collect::<Vec<Rc<Vector<I, N>>>>()
            .try_into()
            .unwrap();
        Line::of(new_line)
    }
}

// Has
impl<T, const N: usize> Has<VectorType<T, N>> for LineType<T, N> {
    fn has(&self, vector: &VectorType<T, N>) -> bool {
        self.iter().any(|vec| Rc::ptr_eq(&vec, vector))
    }
}

impl<T, const N: usize> Has<VectorType<T, N>> for Line<T, N> {
    fn has(&self, vector: &VectorType<T, N>) -> bool {
        self.iter().any(|vec| Rc::ptr_eq(vec, vector))
    }
}

// Serialize
impl<T: Number + Serialize, const N: usize> Serialize for Line<T, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&(*self.a).clone())?;
        seq.serialize_element(&(*self.b).clone())?;
        seq.end()
    }
}

// Index
impl<T: Number + Serialize, const N: usize> Indices<Vec<VectorType<T, N>>> for Vec<LineType<T, N>> {
    type Output = Vec<[usize; 2]>;
    fn get_inds(&self, vecs: Vec<VectorType<T, N>>) -> Self::Output {
        self.into_iter()
            .map(|line| {
                line.to::<[VectorType<T, N>; 2]>()
                    .map(|vector| vecs.iter().position(|v| Rc::ptr_eq(v, &vector)).unwrap())
            })
            .collect()
    }
}

impl<T: Number + Serialize, const N: usize> Indices<Vec<VectorType<T, N>>> for Line<T, N> {
    type Output = [usize; 2];
    fn get_inds(&self, vecs: Vec<VectorType<T, N>>) -> Self::Output {
        self.iter()
            .map(|vector| vecs.iter().position(|v| Rc::ptr_eq(v, vector)).unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap()
    }
}

// Projection
impl<const N: usize, I> Projection<N> for Vec<I>
where
    I: Projection<N>,
{
    type Output<const P: usize> = Vec<<I as Projection<N>>::Output<P>>;
    fn projection<const P: usize>(&self, axises: &[usize]) -> Self::Output<P> {
        self.iter()
            .map(|item| item.projection::<P>(axises))
            .collect()
    }
}

impl<T: Number, const N: usize> Projection<N> for TriangleType<T, N> {
    type Output<const P: usize> = TriangleType<T, P>;

    fn projection<const P: usize>(&self, axises: &[usize]) -> Self::Output<P> {
        self.to::<[LineType<T, N>; 3]>()
            .map(|line| line.projection::<P>(axises))
            .to()
    }
}

impl<T: Number, const N: usize> Projection<N> for LineType<T, N> {
    type Output<const P: usize> = LineType<T, P>;

    fn projection<const P: usize>(&self, axises: &[usize]) -> Self::Output<P> {
        self.map::<T, P>(|vector| vector.projection(axises))
    }
}

impl<T: Number, const N: usize> Projection<N> for Vector<T, N> {
    type Output<const P: usize> = Vector<T, P>;

    fn projection<const P: usize>(&self, axises: &[usize]) -> Self::Output<P> {
        Self::check_errors::<P>(axises);
        let point: Point<T, P> = self
            .0
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !axises.contains(i))
            .map(|(_, m)| m)
            .collect::<Vec<T>>()
            .try_into()
            .unwrap();
        Vector::of(point)
    }
}

impl<'a, T: Number, const N: usize> Projection<N> for &'a Vector<T, N> {
    type Output<const P: usize> = Vector<T, P>;
    fn projection<const P: usize>(&self, axises: &[usize]) -> Self::Output<P> {
        (*self).projection(axises)
    }
}

impl<T: Number, const N: usize> Projection<N> for VectorType<T, N> {
    type Output<const P: usize> = VectorType<T, P>;
    fn projection<const P: usize>(&self, axises: &[usize]) -> Self::Output<P> {
        Rc::new((**self).projection(axises))
    }
}

impl<T: Number, const N: usize> Projection<N> for &VectorType<T, N> {
    type Output<const P: usize> = VectorType<T, P>;
    fn projection<const P: usize>(&self, axises: &[usize]) -> Self::Output<P> {
        (**self).projection(axises)
    }
}
