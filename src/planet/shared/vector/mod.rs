mod impls;
#[cfg(test)]
mod tests;
pub mod ui;

use super::point::{DefaultMeasureValue, Point};
use crate::derive_deref;
use serde::Serialize;
use std::fmt::Debug;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};
use crate::traits::as_::As;

pub trait Number:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + Copy
    + Sum
    + Default
    + Debug
    + As
{
}

macro_rules! impl_Number_for_types {
    ($($t:ty),+) => {
        $(
            impl Number for $t {}
        )+
    };
}

impl_Number_for_types!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
);

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Vector<T = DefaultMeasureValue>(pub Point<T>);
derive_deref!(Vector<T>, 0, Point<T>);

pub type Vectors<T = DefaultMeasureValue> = Vec<Vector<T>>;

impl<T> Vector<T>
where
    T: Number,
{
    pub fn radius(&self) -> T {
        let result = (self
            .iter()
            .map(|&measure| measure.as_::<f64>().powf(2.0))
            .sum::<f64>())
        .sqrt();
        result.as_()
    }

    pub fn scalar(&self, other: &Self) -> T {
        self.0.iter().zip(other.iter()).map(|(&a, &b)| a * b).sum()
    }

    pub fn cos(&self) -> T {
        self.0[1] / self.radius()
    }

    pub fn sin(&self) -> T {
        self.0[0] / self.radius()
    }

    pub fn tan(&self) -> T {
        self.0[1] / self.0[0]
    }

    pub fn atan(&self) -> f64 {
        let vector: Vector<f64> = self.as_();
        let result = vector[1].atan2(vector[0]);
        result
    }

    pub fn angle(&self) -> T {
        T::from(self.atan().to_degrees())
    }

    pub fn sum(&self) -> T {
        self.0.into_iter().sum()
    }
}
