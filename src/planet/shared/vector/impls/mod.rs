mod arithmetic;
mod of_to;

use super::ui::line::Line;
use super::{Number, Vector};
use crate::planet::shared::point::Point;
use crate::planet::shared::traits::{As, Nearest, Svg};
use crate::traits::as_prim::AsPrim;
use crate::traits::of_to::{Of, To};
use std::rc::Rc;
use svg::node::element::Circle;
use svg::node::Value;
use svg::Document;

// Default
impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self([T::default(); N])
    }
}

// As
impl<T: Number, const N: usize> As for Vector<T, N> {
    type Output<I> = Vector<I, N>;

    fn as_<I: Number>(&self) -> Self::Output<I> {
        let point: Point<I, N> = self
            .into_iter()
            .map(|measure| measure.as_::<I>())
            .collect::<Vec<I>>()
            .try_into()
            .unwrap();
        point.to()
    }
}

// Svg
impl<T: Number + Into<Value>> Svg for Rc<Vector<T>> {
    fn to_svg(&self, document: &mut Document) {
        // let vector: Vector<T> = **self * 40.as_::<T>();
        *document = document.clone().add(
            Circle::new()
                .set("cx", self[0])
                .set("cy", self[1])
                .set("r", 0.3)
                .set("fill", "black")
        );
    }
}

// Nearest
impl Nearest<Rc<Vector>> for Vec<Rc<Vector>> {
    fn nearest(&self, vector: &Rc<Vector>) -> Rc<Vector> {
        self.clone()
            .into_iter()
            .min_by(|a, b| {
                let [a, b] = (Line::of([a, b]) - vector.clone())
                    .to::<[Rc<Vector>; 2]>()
                    .map(|vector| vector.radius().abs());
                a.partial_cmp(&b).unwrap()
            })
            .unwrap()
    }
}
