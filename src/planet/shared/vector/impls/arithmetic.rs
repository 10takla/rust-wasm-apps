use std::{
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
};
use super::{Number, Vector};

macro_rules! arithmetic_traits {
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
            arithmetic_traits!($trait => $y => $op => Vector<T, N>, &Vector<T, N>);
        )+
    };
}
arithmetic_traits!(
    Add => add => +, 
    Sub => sub => -, 
    Mul => mul => *, 
    Div => div => /
);

impl<T: Number, const N: usize> Sum for Vector<T, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vector::<T, N>::default(), |acc, v| acc + v)
    }
}
