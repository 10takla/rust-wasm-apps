use crate::planet::shared::vector::VectorType;
use crate::{
    planet::shared::vector::{ui::line::Line, Number, Vector},
    traits::of_to::{Of, To},
};
use std::{
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

macro_rules! arithmetic_traits {
    ($( $trait:ident => $method:ident => $operator:tt),+) => {
        $(
            impl<T: Number, const N: usize> $trait<T> for Line<T, N> {
                type Output = Line<T, N>;
                fn $method(self, rhs: T) -> Self::Output {
                    Line::of(
                        self.to::<[Vector<T, N>; 2]>().map(|vector| vector $operator rhs)
                    )
                }
            }
        )+
    };
}

arithmetic_traits!(Add => add => +, Sub => sub => -, Mul => mul => *, Div => div => /);

impl<T: Number, const N: usize> Sub<VectorType<T, N>> for Line<T, N> {
    type Output = Self;
    fn sub(self, rhs: VectorType<T, N>) -> Self::Output {
        Line::of(
            self.to::<[VectorType<T, N>; 2]>()
                .map(|vector| *vector - *rhs),
        )
    }
}

impl<T: Number, const N: usize> Mul for Line<T, N> {
    type Output = T;
    fn mul(self, rhs: Self) -> Self::Output {
        let [ab, bc] = [self, rhs].map(|line| line.to::<Vector<T, N>>());
        ab[0] * bc[0] + ab[1] * bc[1]
    }
}
