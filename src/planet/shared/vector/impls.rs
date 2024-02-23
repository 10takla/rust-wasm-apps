use crate::planet::shared::point::Point;
use super::Vector;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl<T> From<Point<T>> for Vector<T> {
    fn from(point: Point<T>) -> Self {
        Self(point)
    }
}

impl<T: AddAssign<T> + Copy> Add<T> for Vector<T>{
    type Output = Self;

    fn add(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in  0..new_vector.len() {
            new_vector[i] += other;
        }
        Vector(new_vector)
    }
}

impl<T: AddAssign<T> + Copy> Add for Vector<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] += other[i];
        }
        Vector(new_vector)
    }
}

impl<T: SubAssign<T> + Copy> Sub<T> for Vector<T> {
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] -= other;
        }
        Vector(new_vector)
    }
}

impl<T: SubAssign<T> + Copy> Sub for Vector<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] -= other[i];
        }
        Vector(new_vector)
    }
}

impl<T: MulAssign<T> + Copy> Mul<T> for Vector<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] *= other;
        }
        Vector(new_vector)
    }
}


impl<T: MulAssign<T> + Copy> Mul for Vector<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] *= other[i];
        }
        Vector(new_vector)
    }
}

impl<T: DivAssign<T> + Copy> Div<T> for Vector<T> {
    type Output = Self;
    fn div(self, other: T) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] /= other;
        }
        Vector(new_vector)
    }
}

impl<T: DivAssign<T> + Copy> Div for Vector<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut new_vector = *self;
        for i in 0..new_vector.len() {
            new_vector[i] /= other[i];
        }
        Vector(new_vector)
    }
}