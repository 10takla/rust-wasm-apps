use crate::planet::shared::point::{DefaultMeasureValue, Point};
use super::Vector;
use std::ops::{Sub, Mul, Add, Div};

impl From<Point> for Vector {
    fn from(point: Point) -> Self {
        Self(point)
    }
}

impl Add<DefaultMeasureValue> for Vector {
    type Output = Self;

    fn add(self, other: DefaultMeasureValue) -> Self {
        let mut new_vector = self.0;
        for i in  0..new_vector.len() {
            new_vector[i] += other;
        }
        Vector(new_vector)
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] += other[i];
        }
        Vector(new_vector)
    }
}

impl Sub<DefaultMeasureValue> for Vector {
    type Output = Self;
    fn sub(self, other: DefaultMeasureValue) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] -= other;
        }
        Vector(new_vector)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] -= other[i];
        }
        Vector(new_vector)
    }
}

impl Mul<DefaultMeasureValue> for Vector {
    type Output = Self;
    fn mul(self, other: DefaultMeasureValue) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] *= other;
        }
        Vector(new_vector)
    }
}


impl Mul for Vector {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] *= other[i];
        }
        Vector(new_vector)
    }
}

impl Div<DefaultMeasureValue> for Vector {
    type Output = Self;
    fn div(self, other: DefaultMeasureValue) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] /= other;
        }
        Vector(new_vector)
    }
}

impl Div for Vector {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] /= other[i];
        }
        Vector(new_vector)
    }
}