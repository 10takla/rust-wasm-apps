mod ordering;
mod from;

use super::{Number, Vector};
use crate::planet::shared::point::Point;
use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};
use crate::traits::of_to::To;

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
        let point: Point<I, N> = self
            .into_iter()
            .map(|measure| measure.as_::<I>())
            .collect::<Vec<I>>()
            .try_into()
            .unwrap();
        point.to()
    }
}

// Arithmetic
macro_rules! fast {
    ($trait:ident => $y:tt => $op:tt => $($t:ty),+) => {
        $(
            impl<T: Number, const N: usize> $trait for $t {
                type Output = Vector<T, N>;
                fn $y(self, other: Self) -> Self::Output {
                    Vector(
                        self
                            .into_iter()
                            .zip(other.into_iter())
                            .map(|(a, b)| a $op b)
                            .collect::<Vec<T>>()
                            .try_into()
                            .unwrap(),
                    )
                }
            }
            impl<T: Number, const N: usize> $trait<T> for $t {
                type Output = Vector<T, N>;
                fn $y(self, other: T) -> Self::Output {
                    Vector(
                        self
                            .into_iter()
                            .map(|a| a $op other)
                            .collect::<Vec<T>>()
                            .try_into()
                            .unwrap(),
                    )
                }
            }
        )+
    };
    ($($trait:ident => $y:tt => $op:tt),+) => {
        $(
            fast!($trait => $y => $op => Vector<T, N>, &Vector<T, N>);
        )+
    };
}
fast!(Add => add => +, Sub => sub => -, Mul => mul => *, Div => div => /);

impl<T: Number, const N: usize> Sum for Vector<T, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector::<T, N>::default(), |acc, v| acc + v)
    }
}