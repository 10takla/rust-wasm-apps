mod arithmetic;
mod display;
mod iterator;
mod nearest;
mod of_to;
mod ordering;

use svg::node::element::path::{Data, Parameters};
use svg::node::Value;
use svg::Document;

use super::Line;
use crate::planet::shared::traits::{As, Has, Svg};
use crate::planet::shared::vector::{Number, Vector};
use crate::traits::of_to::{Of, To};
use std::rc::Rc;

// As
impl<F: Number, const N: usize> As for Line<F, N> {
    type Output<I> = Line<I, N>;
    fn as_<I: Number>(&self) -> Self::Output<I> {
        let new_line: [Rc<Vector<I, N>>; 2] = self
            .clone()
            .into_iter()
            .map(|vector| Rc::new(vector.as_()))
            .collect::<Vec<Rc<Vector<I, N>>>>()
            .try_into()
            .unwrap();
        Line::of(new_line)
    }
}

// Svg
impl<T: Number + Into<Parameters>> Svg for Vec<Line<T>> {
    fn to_svg(&self, document: &mut Document) {
        let mut path_data = {
            let first = *self[0].a;
            Data::new().move_to((first[0], first[1]))
        };
        for line in self.into_iter().clone() {
            let next = *line.b;
            path_data = path_data.clone().line_to((next[0], next[1]));
        }
        *document = document.clone().add(
            svg::node::element::Path::new()
                .set("d", path_data)
                .set("stroke", "black")
                .set("fill", "none"),
        );
    }
}

impl<T: Number + Into<Parameters> + Into<Value>> Svg for Vec<Rc<Line<T>>> {
    fn to_svg(&self, document: &mut Document) {
        self.to::<Vec<Rc<Vector<T>>>>()
            .into_iter()
            .for_each(|vector| {
                vector.to_svg(document);
            });

        self.into_iter().for_each(|line| {
            *document = document.clone().add(
                svg::node::element::Line::new()
                    .set("x1", line.a[0])
                    .set("y1", line.a[1])
                    .set("x2", line.b[0])
                    .set("y2", line.b[1])
                    .set("stroke", "black")
                    .set("stroke-width", "0.2"),
            );
        });
    }
}

// Has
impl<T, const N: usize> Has<Rc<Vector<T, N>>> for Rc<Line<T, N>> {
    fn has(&self, vector: &Rc<Vector<T, N>>) -> bool {
        self.iter().any(|vec| Rc::ptr_eq(&vec, vector))
    }
}
