use super::{Number, Vector};
use crate::planet::shared::point::Point;
use std::{iter::Sum, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

impl<'a, T> From<&'a Vector<T>> for &'a Point<T> {
    fn from(value: &'a Vector<T>) -> Self {
        &value.0
    }
}

impl<T> From<Vector<T>> for Point<T> {
    fn from(value: Vector<T>) -> Self {
        value.0
    }
}

impl<T> From<Point<T>> for Vector<T> {
    fn from(point: Point<T>) -> Self {
        Self(point)
    }
}

impl<F, I> From<&Vector<F>> for Vector<I>
where
    F: Number,
    I: Default + Number,
{
    fn from(vector: &Vector<F>) -> Self {
        let mut new_vector = Vector::default();
        for i in 0..vector.len() {
            new_vector[i] = I::from(vector[i]).unwrap();
        }
        new_vector
    }
}

impl<T:> Default for Vector<T>
where
    T: Default + Copy,
{
    fn default() -> Self {
        let default_value = T::default();
        Self([default_value, default_value])
    }
}

impl<T: AddAssign<T> + Copy > Add<T> for Vector<T> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] += other;
        }
        Self(new_vector)
    }
}

impl<T: AddAssign<T> + Copy > Add for Vector<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] += other[i];
        }
        Self(new_vector)
    }
}

impl<T: SubAssign<T> + Copy > Sub<T> for Vector<T> {
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] -= other;
        }
        Self(new_vector)
    }
}

impl<T: SubAssign<T> + Copy > Sub for Vector<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] -= other[i];
        }
        Self(new_vector)
    }
}

impl<T: MulAssign<T> + Copy > Mul<T> for Vector<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] *= other;
        }
        Self(new_vector)
    }
}

impl<T: MulAssign<T> + Copy > Mul for Vector<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] *= other[i];
        }
        Self(new_vector)
    }
}

impl<T: DivAssign<T> + Copy > Div<T> for Vector<T> {
    type Output = Self;
    fn div(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] /= other;
        }
        Self(new_vector)
    }
}

impl<T: DivAssign<T> + Copy > Div for Vector<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] /= other[i];
        }
        Self(new_vector)
    }
}

impl Sum for Vector {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector([0.0,  0.0]), |acc, v| {
            acc + v
        })
    }
}