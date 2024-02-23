#[cfg(test)]
mod tests;
mod impls;
pub mod ui;

use serde::Serialize;

use crate::derive_deref;

use super::point::{DefaultMeasureValue, Point};

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Vector<T = DefaultMeasureValue>(pub Point<T>);
derive_deref!(Vector, 0, Point<T>, T);

pub type Vectors<T = DefaultMeasureValue> = Vec<Vector<T>>;

impl Vector{
    pub fn radius(&self) -> DefaultMeasureValue {
        (
            self.iter().map(|measure| measure.powf(2.0)).sum::<DefaultMeasureValue>()
        ).sqrt()
    }

    pub fn scalar(&self, other: &Self) -> DefaultMeasureValue {
        self.iter().zip(other.0.iter()).map(|(a, b)| a * b).sum()
    }
    
    pub fn cos(&self) -> DefaultMeasureValue {
        self[1] / self.radius()
    }

    pub fn sin(&self) -> DefaultMeasureValue {
        self[0] / self.radius()
    }

    pub fn tan(&self) -> DefaultMeasureValue {
        self[1].atan2(self[0])
    }

    pub fn angle(&self) -> DefaultMeasureValue {
        self.tan().to_degrees()
    }

    pub fn sum(&self) -> DefaultMeasureValue {
        self.iter().sum()
    }
}