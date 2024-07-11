mod impls;
pub mod svg;
#[cfg(test)]
mod tests;
pub mod ui;

use super::point::{DefaultMeasureValue, Point, DEFAULT_MEASURE};
use super::traits::{As, Normalize, Projection};
use crate::traits::as_prim::AsPrim;
use macros::Deref;
use serde::ser::SerializeSeq;
use serde::Serialize;
use std::f64::consts::PI;
use std::fmt::{Debug, Display};
use std::rc::Rc;
use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

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
    + AsPrim
    + PartialOrd
    + PartialEq
    + Display
    + Debug
    + 'static
{
}

macro_rules! impl_Number_for_types {
    ($($t:ty),+) => {
        $(
            impl Number for $t {}
        )+
    };
}

impl_Number_for_types!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash, Deref)]
pub struct Vector<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE>(pub Point<T, N>);

pub type Vectors<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> = Vec<VectorType<T, N>>;

pub type VectorType<T = DefaultMeasureValue, const N: usize = DEFAULT_MEASURE> = Rc<Vector<T, N>>;

pub trait SphericalProjection {
    type Output;
    fn projection(&self) -> Self::Output;
}

impl<T: Number> SphericalProjection for Vector<T, 3> {
    type Output = Vector<T, 2>;
    fn projection(&self) -> Self::Output {
        let xy: Vector<T, 2> = Projection::projection(&self, &[2]);
        let xy_l = 2. * PI * xy.radius().as_::<f64>() * xy.tan().as_::<f64>();

        let xz: Vector<T, 2> = Projection::projection(&self, &[1]);
        let xz_l = 2. * PI * xz.radius().as_::<f64>() * xz.tan().as_::<f64>();

        Vector([xy_l, xz_l]).as_()
    }
}

impl<T: Number, const N: usize> Vector<T, N> {
    pub fn radius(&self) -> T {
        (self
            .iter()
            .map(|&measure| measure.as_::<f64>().powf(2.0))
            .sum::<f64>())
        .sqrt()
        .as_()
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
        let vector: Vector<f64, N> = self.as_();
        vector[1].atan2(vector[0])
    }

    pub fn angle(&self) -> T {
        self.atan().to_degrees().as_()
    }

    pub fn sum(&self) -> T {
        self.0.into_iter().sum()
    }
}
