use std::ops::{Sub, Mul, Add, Div};
use crate::planet::point_distribution::{DefaultMeasureValue, Point};

#[derive(Debug, Clone, Copy)]
pub struct Vector(pub [f64; 2]);

impl Vector {
    pub fn radius(&self) -> DefaultMeasureValue {
        (
            self.0.iter().map(|measure| measure.powf(2.0)).sum::<DefaultMeasureValue>()
        ).sqrt()
    }

    pub fn scalar(&self, other: &Self) -> DefaultMeasureValue {
        self.0.iter().zip(other.0.iter()).map(|(a, b)| a * b).sum()
    }
    
    pub fn cos(&self) -> DefaultMeasureValue {
        self.0[1] / self.radius()
    }

    pub fn sin(&self) -> DefaultMeasureValue {
        self.0[0] / self.radius()
    }

    pub fn tan(&self) -> DefaultMeasureValue {
        self.0[1].atan2(self.0[0])
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] += other.0[i];
        }
        Vector(new_vector)
    }
}
impl Sub for Vector {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] -= other.0[i];
        }
        Vector(new_vector)
    }
}
impl Mul for Vector {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] *= other.0[i];
        }
        Vector(new_vector)
    }
}
impl Div for Vector {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut new_vector = self.0;
        for i in 0..new_vector.len() {
            new_vector[i] /= other.0[i];
        }
        Vector(new_vector)
    }
}