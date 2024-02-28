mod impls;
#[cfg(test)]
mod tests;
pub mod ui;

use std::fmt::Debug;
use super::point::{DefaultMeasureValue, Point};
use crate::derive_deref;
use num::{FromPrimitive, NumCast};
use serde::Serialize;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};


pub trait FromAll: From<f64> + Into<f64> + Sum + Default {}


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
    + NumCast
    + Sum 
    + Default 
    + Into<f64>
    + Debug
{
}

impl Number for i32 {}
impl Number for f32 {}
impl Number for f64 {}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Vector<T = DefaultMeasureValue>(pub Point<T>);
derive_deref!(Vector, 0, Point<T>, T);


pub type Vectors<T = DefaultMeasureValue> = Vec<Vector<T>>;

#[macro_export]
macro_rules! vector_as {
    ($v:expr, $type:ty) => {{
        let mut tmp = [0 as $type; 2];
        for (i, coo) in $v.into_iter().enumerate() {
            tmp[i] = coo.into();
        }
        Vector::from(tmp)
    }};
}

impl<T> Vector<T>
where 
    T: Number,
{
    pub fn radius(&self) -> T {
        let result = (self
            .iter()
            .map(|&measure| {
                let t: f64 = measure.into();
                t.powf(2.0)
            })
            .sum::<f64>())
        .sqrt();
        NumCast::from(result).unwrap()
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
        let vector: Vector<f64> = self.into();
        let result = vector[1].atan2(vector[0]);
        result
    }

    pub fn angle(&self) -> T {
        NumCast::from(self.atan().to_degrees()).unwrap()
    }

    pub fn sum(&self) -> T {
        self.0.into_iter().sum()
    }
}
