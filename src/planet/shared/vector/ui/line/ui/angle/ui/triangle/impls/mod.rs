mod display;
mod iterator;
mod of_to;
mod ordering;
mod alien;
mod search;

use super::Triangle;
use crate::planet::shared::traits::{Has, Svg};
use crate::planet::shared::vector::Vector;
use crate::{
    planet::shared::{
        point::Point,
        vector::{
            ui::line::{ui::angle::Angle, Line},
            Number,
        },
    },
    traits::of_to::To,
};
use std::fmt::Debug;
use std::rc::Rc;
use svg::node::element::path::Parameters;
use svg::node::element::Polygon;
use svg::node::Value;
use svg::Document;

//Debug
impl<T: Number, const N: usize> Debug for Triangle<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vecs = self.to::<[Vector<T, N>; 3]>();
        f.debug_struct("Rectangle")
            .field("a", &vecs[0])
            .field("b", &vecs[1])
            .field("c", &vecs[2])
            .finish()
    }
}

// Has
impl<T: Number, const N: usize> Has<Rc<Angle<T, N>>> for Triangle<T, N> {
    fn has(&self, angle: &Rc<Angle<T, N>>) -> bool {
        self.clone().into_iter().any(|angle1| angle1 == *angle)
    }
}

impl<T: Number, const N: usize> Has<Rc<Line<T, N>>> for Triangle<T, N> {
    fn has(&self, n_line: &Rc<Line<T, N>>) -> bool {
        self.clone()
            .to::<[Line<T, N>; 3]>()
            .into_iter()
            .any(|line| line == *n_line.clone())
    }
}

// Svg
impl<T: Number> Svg for Triangle<T> {
    fn to_svg(&self, document: &mut Document) {
        let points = self
            .to::<[Point<T>; 3]>()
            .map(|point| point.map(|m| m.to_string()).join(","))
            .join(" ");
        *document = document.clone().add(
            Polygon::new()
                .set("points", points)
                .set("fill", "rgba(255, 0, 0, 0.5)"),
        )
    }
}

impl<T: Number + Into<Value>> Svg for Rc<Triangle<T>> {
    fn to_svg(&self, document: &mut Document) {
        let points = (*self.as_ref())
            .clone()
            .to::<[Point<T>; 3]>()
            .map(|point| {
                point.map(|value| value.to_string()).join(",")
            })
            .join(" ");

        *document = document.clone().add(
            Polygon::new()
                .set("points", points)
                .set("fill", "rgba(255, 0, 0, 0.5)"),
        )
    }
}

impl<T: Number + Into<Value> + Into<Parameters>> Svg for Vec<Rc<Triangle<T>>> {
    fn to_svg(&self, document: &mut Document) {
        self.into_iter().for_each(|triangle| {
            triangle.to_svg(document);
        });

        self.to::<Vec<Rc<Line<T>>>>().to_svg(document);
    }
}
