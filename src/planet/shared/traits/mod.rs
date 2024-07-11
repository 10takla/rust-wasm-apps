use std::{any::Any, collections::HashMap, ops::Range, rc::Rc};

use svg::Document;

use crate::traits::of_to::Of;

use super::{
    point::DefaultMeasureValue,
    vector::{
        svg::VectorStyle,
        ui::line::{svg::LineStyle, ui::angle::ui::triangle::Triangle, Line, LineType},
        Number, Vector, VectorType,
    },
};

pub trait Has<T> {
    fn has(&self, value: &T) -> bool;
}

pub trait Svg {
    fn to_svg(&self, document: &mut Document);
}

#[derive(Debug)]
pub struct SvgStyle<E: Svg, S>(pub E, pub S);

impl<E: Clone + Svg, S: Default> SvgStyle<E, S> {
    pub fn new(element: &E) -> Self {
        SvgStyle(element.clone(), S::default())
    }
}

impl Svg for Vec<Box<dyn Svg + 'static>> {
    fn to_svg(&self, document: &mut Document) {
        for el in self {
            el.to_svg(document)
        }
    }
}

impl<'a> Svg for Vec<&'a (dyn Svg + 'static)> {
    fn to_svg(&self, document: &mut Document) {
        for el in self {
            el.to_svg(document)
        }
    }
}

pub trait Nearest<I, O = I> {
    fn nearest(&self, el: &I) -> O;
}

pub trait As {
    type Output<I>;
    fn as_<I: Number>(&self) -> Self::Output<I>;
}

pub trait Alien<A, B = A> {
    fn alien(&self, one_own: &B) -> A;
}

pub trait SuperAlien<B> {
    fn alien<A>(&self, one_own: &B) -> A
    where
        Self: Alien<A, B>;
}

impl<T, B> SuperAlien<B> for T {
    fn alien<A>(&self, one_own: &B) -> A
    where
        T: Alien<A, B>,
    {
        T::alien(&self, &one_own)
    }
}

pub trait Find<A, B = A> {
    fn find(&self, one_own: &B) -> A;
}

pub trait SuperFind<B> {
    fn search<A>(&self, one_own: &B) -> A
    where
        Self: Find<A, B>;
}

impl<T, B> SuperFind<B> for T {
    fn search<A>(&self, one_own: &B) -> A
    where
        T: Find<A, B>,
    {
        T::find(&self, &one_own)
    }
}

pub trait Indices<T> {
    type Output;
    fn get_inds(&self, by: T) -> Self::Output;
}

pub trait Normalize<T: Number, const N: usize> {
    fn normalize(&mut self) -> &mut Self;
}

pub trait Projection<const N: usize> {
    type Output<const P: usize>;
    fn check_errors<const P: usize>(axises: &[usize]) {
        if axises.len() != N - P {
            panic!("Количество исключающих осей должно быть {} (N - P)", N - P)
        }

        let mut count_axises: HashMap<usize, usize> = HashMap::new();
        let mut non_existent_axes = vec![];
        for &axis in axises {
            if axis >= N {
                non_existent_axes.push(axis);
            }
            count_axises.insert(axis, {
                if let Some(&count) = count_axises.get(&axis) {
                    count + 1
                } else {
                    0
                }
            });
        }
        if non_existent_axes.len() > 0 {
            panic!("Оси {:?} не сущесвуют", non_existent_axes)
        }

        let panic_msg = count_axises
            .into_iter()
            .filter(|&(axis, count)| count > 0)
            .map(|(axis, count)| format!("   ось {axis} - {count} раз"))
            .collect::<Vec<String>>();
        if panic_msg.len() > 0 {
            panic!(
                "Оси должны быть уникальными.\nПовторяющиеся оси:\n{}",
                panic_msg.join("\n")
            )
        }
    }

    fn filter<const P: usize>(&self, axises: [usize; P]) -> Self::Output<P> {
        self.projection(
            &(0..N)
                .filter(|i| !axises.contains(i))
                .collect::<Vec<usize>>(),
        )
    }

    fn projection<const P: usize>(&self, axises: &[usize]) -> Self::Output<P>;
}
