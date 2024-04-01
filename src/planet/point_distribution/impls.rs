use std::rc::Rc;

use super::{PointDistribution, Points, Vectors};
use crate::planet::shared::traits::As;

use crate::traits::of_to::Of;
use crate::{
    planet::shared::vector::{Number, Vector},
    traits::of_to::To,
};

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

impl<T, const N: usize> Of<Vec<&Rc<Vector<T, N>>>> for PointDistribution<T, N> {
    fn of(vecs: Vec<&Rc<Vector<T, N>>>) -> Self {
        Self(
            vecs.into_iter()
                .map(|vector| (*vector).clone())
                .collect::<Vec<Rc<Vector<T, N>>>>(),
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
impl<T: Copy, const N: usize> Of<PointDistribution<T, N>> for Vec<Rc<Vector<T, N>>> {
    fn of(pd: PointDistribution<T, N>) -> Self {
        pd.0
    }
}

// for Points
impl<T: Copy, const N: usize> Of<PointDistribution<T, N>> for Points<T, N> {
    fn of(pd: PointDistribution<T, N>) -> Self {
        pd.0.into_iter().map(|v| (*v).0).collect()
    }
}
