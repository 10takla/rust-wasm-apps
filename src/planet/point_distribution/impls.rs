use super::{PointDistribution, Points, Vectors};
use crate::planet::shared::point::Point;
use crate::planet::shared::traits::As;
use crate::planet::shared::traits::Normalize;
use crate::planet::shared::vector::VectorType;
use crate::traits::of_to::Of;
use crate::{
    planet::shared::vector::{Number, Vector},
    traits::of_to::To,
};
use std::ops::Deref;
use std::ops::Mul;
use std::rc::Rc;

// As
impl<F: Number, const N: usize> As for PointDistribution<F, N> {
    type Output<I> = PointDistribution<I, N>;
    fn as_<I: Number>(&self) -> Self::Output<I> {
        self.iter()
            .map(|vector| Rc::new(vector.as_()))
            .collect::<Vec<Rc<Vector<I, N>>>>()
            .to()
    }
}

// Of
// of Vector
impl<T, const N: usize> Of<Vectors<T, N>> for PointDistribution<T, N> {
    fn of(vecs: Vectors<T, N>) -> Self {
        Self(vecs)
    }
}

impl<T, const N: usize> Of<Vec<&VectorType<T, N>>> for PointDistribution<T, N> {
    fn of(vecs: Vec<&VectorType<T, N>>) -> Self {
        Self(
            vecs.into_iter()
                .map(|vector| (*vector).clone())
                .collect::<Vec<VectorType<T, N>>>(),
        )
    }
}

impl<T, const N: usize> Of<Vec<Vector<T, N>>> for PointDistribution<T, N> {
    fn of(vecs: Vec<Vector<T, N>>) -> Self {
        Self(vecs.into_iter().map(|vector| Rc::new(vector)).collect())
    }
}

// of Points
impl<T: Number, const N: usize> Of<Points<T, N>> for PointDistribution<T, N> {
    fn of(vecs: Points<T, N>) -> Self {
        Self(
            vecs.into_iter()
                .map(|vector| Rc::new(vector.to()))
                .collect(),
        )
    }
}

// for Vectors
impl<T: Copy, const N: usize> Of<PointDistribution<T, N>> for Vec<VectorType<T, N>> {
    fn of(pd: PointDistribution<T, N>) -> Self {
        pd.0
    }
}

impl<T: Copy, const N: usize> Of<&PointDistribution<T, N>> for Vec<VectorType<T, N>> {
    fn of(pd: &PointDistribution<T, N>) -> Self {
        (*pd).clone().to()
    }
}

// for Points
impl<T: Copy, const N: usize> Of<PointDistribution<T, N>> for Vec<Point<T, N>> {
    fn of(pd: PointDistribution<T, N>) -> Self {
        pd.0.into_iter().map(|v| (*v).0).collect()
    }
}

impl<T: Copy, const N: usize> Of<&PointDistribution<T, N>> for Vec<Point<T, N>> {
    fn of(pd: &PointDistribution<T, N>) -> Self {
        pd.clone().to()
    }
}

impl<T: Copy, const N: usize> Of<&mut PointDistribution<T, N>> for Vec<Point<T, N>> {
    fn of(pd: &mut PointDistribution<T, N>) -> Self {
        pd.deref().to()
    }
}

// Normalize
// impl<T: Number, const N: usize> Normalize for PointDistribution<T, N> {
//     fn normalize(&mut self) -> &mut Self {
//         self.0.iter_mut().for_each(|v| {
//             let mut vector = (**v).clone();
//             vector.normalize();
//             *v = Rc::new(vector);
//         });
//         self
//     }
// }

impl<T: Number, const N: usize> Mul<T> for PointDistribution<T, N> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.iter().map(|vec| Rc::new(**vec * rhs)).collect())
    }
}
