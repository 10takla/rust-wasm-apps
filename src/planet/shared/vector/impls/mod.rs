mod arithmetic;
mod of_to;

use super::svg::VectorStyle;
use super::ui::line::Line;
use super::{Number, Vector};
use crate::planet::shared::point::Point;
use crate::planet::shared::traits::{As, Has, Indices, Nearest, Normalize, Svg};
use crate::planet::shared::vector::VectorType;
use crate::traits::as_prim::AsPrim;
use crate::traits::of_to::{Of, To};
use serde::ser::SerializeSeq;
use serde::Serialize;
use std::rc::Rc;
use svg::node::element::{Circle, Text};
use svg::node::Value;
use svg::Document;

// Normalize
impl<T: Number, const N: usize> Normalize<T, N> for Vector<T, N> {
    fn normalize(&mut self) -> &mut Self {
        let length = self.radius();
        self.iter_mut().for_each(|v| *v /= length);
        self
    }
}

// Default
impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self([T::default(); N])
    }
}

// As
impl<T: Number, const N: usize> As for Vector<T, N> {
    type Output<I> = Vector<I, N>;

    fn as_<I: Number>(&self) -> Self::Output<I> {
        let point: Point<I, N> = self
            .into_iter()
            .map(|measure| measure.as_::<I>())
            .collect::<Vec<I>>()
            .try_into()
            .unwrap();
        point.to()
    }
}

// Nearest
impl<T: Number, const N: usize> Nearest<VectorType<T, N>> for Vec<VectorType<T, N>> {
    fn nearest(&self, vector: &VectorType<T, N>) -> VectorType<T, N> {
        self.clone()
            .into_iter()
            .min_by(|a, b| {
                let [a, b] = (Line::of([a, b]) - vector.clone())
                    .to::<[VectorType<T, N>; 2]>()
                    .map(|vector| vector.radius());
                a.partial_cmp(&b).unwrap()
            })
            .unwrap()
    }
}

// Has
impl<T, const N: usize> Has<VectorType<T, N>> for VectorType<T, N> {
    fn has(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}

impl<T, const N: usize> Has<VectorType<T, N>> for Vec<VectorType<T, N>> {
    fn has(&self, other: &VectorType<T, N>) -> bool {
        self.into_iter().any(|vector| vector.has(other))
    }
}

// Serialize
impl<T, const N: usize> Serialize for Vector<T, N>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(N))?;
        for element in &self.0 {
            seq.serialize_element(element)?;
        }
        seq.end()
    }
}

// Indeces
impl<T: Number + Serialize, const N: usize> Indices<Vec<VectorType<T, N>>> for VectorType<T, N> {
    type Output = usize;
    fn get_inds(&self, vecs: Vec<VectorType<T, N>>) -> Self::Output {
        vecs.iter().position(|v| Rc::ptr_eq(v, self)).unwrap()
    }
}

impl<T: Number + Serialize, const N: usize> Indices<Vec<VectorType<T, N>>>
    for Vec<VectorType<T, N>>
{
    type Output = Vec<usize>;
    fn get_inds(&self, vecs: Vec<VectorType<T, N>>) -> Self::Output {
        self.into_iter()
            .map(|vector| vecs.iter().position(|v| Rc::ptr_eq(v, vector)).unwrap())
            .collect()
    }
}
