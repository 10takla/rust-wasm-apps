mod ordering;

use super::{Number, Vector};
use crate::planet::shared::point::Point;
use std::{iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self([T::default(); N])
    }
}

impl<T: Number, const N: usize> Vector<T, N> {
    pub fn as_<I: Number>(self) -> Vector<I, N> {
        let point: Point<I, N> = self.into_iter().map(|measure| measure.as_::<I>()).collect::<Vec<I>>().try_into().unwrap();
        Vector::from(point)
    }
}

//Froms
impl<'a, T, const N: usize> From<&'a Vector<T, N>> for &'a Point<T, N> {
    fn from(value: &'a Vector<T, N>) -> Self {
        &value.0
    }
}

impl<T, const N: usize> From<Vector<T, N>> for Point<T, N> {
    fn from(value: Vector<T, N>) -> Self {
        value.0
    }
}

impl<F: Number, I: Number, const N: usize> From<Point<F, N>> for Vector<I, N> {
    fn from(point: Point<F, N>) -> Self {
        Self(
            point.into_iter().map(|measure| measure.as_()).collect::<Vec<I>>().try_into().unwrap()
        )
    }
}

// Arithmetic
impl<T: AddAssign<T> + Copy, const N: usize> Add<T> for Vector<T, N> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] += other;
        }
        Self(new_vector)
    }
}

impl<T: AddAssign<T> + Copy, const N: usize> Add for Vector<T, N> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] += other[i];
        }
        Self(new_vector)
    }
}

impl<T: SubAssign<T> + Copy, const N: usize> Sub<T> for Vector<T, N> {
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] -= other;
        }
        Self(new_vector)
    }
}

impl<T: SubAssign<T> + Copy, const N: usize> Sub for Vector<T, N> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] -= other[i];
        }
        Self(new_vector)
    }
}

impl<T: MulAssign<T> + Copy, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] *= other;
        }
        Self(new_vector)
    }
}

impl<T: MulAssign<T> + Copy, const N: usize> Mul for Vector<T, N> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] *= other[i];
        }
        Self(new_vector)
    }
}

impl<T: DivAssign<T> + Copy, const N: usize> Div<T> for Vector<T, N> {
    type Output = Self;
    fn div(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] /= other;
        }
        Self(new_vector)
    }
}

impl<T: DivAssign<T> + Copy, const N: usize> Div for Vector<T, N> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] /= other[i];
        }
        Self(new_vector)
    }
}

impl<T: Number, const N: usize> Sum for Vector<T, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector::<T, N>::default(), |acc, v| {
            acc + v
        })
    }
}