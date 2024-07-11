use std::rc::Rc;
use svg::{
    node::{element::Polygon, Value},
    Document,
};
use super::{Triangle, TriangleType};
use crate::{
    planet::shared::{
        point::Point,
        traits::{Svg, SvgStyle},
        vector::{
            svg::VectorStyle,
            ui::line::{svg::LineStyle, LineType},
            Number, VectorType,
        },
    },
    traits::of_to::To,
};
use macros::{setter, Default};

#[derive(Clone, Default, Debug)]
#[setter]
pub struct TriangleStyle {
    #[default_field = "rgba(255, 0, 0, 0.5)"]
    pub color: &'static str,

    #[default_field = "black"]
    pub stroke_color: &'static str,

    #[default_field = 0.12]
    stroke_width: f64,

    pub vecs: VectorStyle,
}

// Svg
impl<T: Number> Svg for SvgStyle<TriangleType<T>, TriangleStyle> {
    fn to_svg(&self, document: &mut Document) {
        let SvgStyle(el, style) = self;

        let points = el
            .as_ref()
            .clone()
            .to::<[Point<T>; 3]>()
            .map(|point| point.map(|value| value.to_string()).join(","))
            .join(" ");

        *document = document.clone().add(
            Polygon::new()
                .set("points", points)
                .set("stroke", style.stroke_color)
                .set("stroke-width", style.stroke_width)
                .set("fill", style.color),
        )
    }
}

impl<T: Number + Into<Value>> Svg for SvgStyle<Vec<Rc<Triangle<T>>>, TriangleStyle> {
    fn to_svg(&self, document: &mut Document) {
        self.0.clone().into_iter().for_each(|triangle| {
            SvgStyle(triangle, self.1.clone()).to_svg(document);
        });
        SvgStyle(self.0.clone().to::<Vec<VectorType<T>>>(), self.1.vecs.clone()).to_svg(document);
    }
}

impl<T: Number> Svg for TriangleType<T> {
    fn to_svg(&self, document: &mut Document) {
        SvgStyle::new(self).to_svg(document);
    }
}

impl<T: Number> Svg for Triangle<T> {
    fn to_svg(&self, document: &mut Document) {
        SvgStyle::new(&self.to::<TriangleType<T>>()).to_svg(document);
    }
}

impl<T: Number + Into<Value>> Svg for Vec<Rc<Triangle<T>>> {
    fn to_svg(&self, document: &mut Document) {
        self.into_iter().for_each(|triangle| {
            SvgStyle::new(triangle).to_svg(document);
        });

        SvgStyle::new(&self.to::<Vec<LineType<T>>>()).to_svg(document);
    }
}
