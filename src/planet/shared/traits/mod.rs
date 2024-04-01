use svg::Document;

use super::vector::Number;

pub trait Has<T> {
    fn has(&self, value: &T) -> bool;
}

pub trait Svg {
    fn to_svg(&self, document: &mut Document);
}

// impl<T: Svg> Svg for Vec<T> {
//     fn to_svg(&self, document: &mut Document) {
//         self.into_iter().for_each(|triangle| {
//             triangle.to_svg(document);
//         });
//     }
// }

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
