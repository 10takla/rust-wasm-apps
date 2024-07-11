use std::rc::Rc;

use super::{Line, LineType};
use crate::{
    planet::shared::{
        traits::{Svg, SvgStyle},
        vector::{svg::VectorStyle, Number, VectorType},
    },
    traits::of_to::{Of, To},
};
use macros::{setter, Default};
use svg::{node::Value, Document};

#[derive(Clone, Default, Debug)]
#[setter]
pub struct LineStyle {
    #[default_field = "black"]
    pub color: &'static str,

    #[default_field = 0.1]
    pub width: f64,

    pub vecs: VectorStyle,
}

impl<T: Number + Into<Value>> Svg for LineType<T> {
    fn to_svg(&self, document: &mut Document) {
        SvgStyle::new(self).to_svg(document);
    }
}

impl<T: Number + Into<Value>> Svg for Line<T> {
    fn to_svg(&self, document: &mut Document) {
        SvgStyle::new(&Rc::new(self.clone())).to_svg(document);
    }
}

impl<T: Number + Into<Value>> Svg for Vec<LineType<T>> {
    fn to_svg(&self, document: &mut Document) {
        SvgStyle::new(self).to_svg(document);
    }
}


impl<T: Number + Into<Value>> Svg for Vec<Line<T>> {
    fn to_svg(&self, document: &mut Document) {
        SvgStyle::new(&self.to::<Vec<LineType<T>>>()).to_svg(document);
    }
}

impl<T: Number + Into<Value>> Svg for SvgStyle<LineType<T>, LineStyle> {
    fn to_svg(&self, document: &mut Document) {
        let SvgStyle(el, style) = self;
        *document = document.clone().add(
            svg::node::element::Line::new()
                .set("x1", el.a[0])
                .set("y1", el.a[1])
                .set("x2", el.b[0])
                .set("y2", el.b[1])
                .set("stroke", style.color)
                .set("stroke-width", style.width),
        );
        el.iter().for_each(|vector| {
            SvgStyle(vector.clone(), style.vecs.clone()).to_svg(document);
        });
    }
}

impl<T: Number + Into<Value>> Svg for SvgStyle<Vec<LineType<T>>, LineStyle> {
    fn to_svg(&self, document: &mut Document) {
        let SvgStyle(el, style) = self;
        el.clone().into_iter().for_each(|line| {
            *document = document.clone().add(
                svg::node::element::Line::new()
                    .set("x1", line.a[0])
                    .set("y1", line.a[1])
                    .set("x2", line.b[0])
                    .set("y2", line.b[1])
                    .set("stroke", style.color)
                    .set("stroke-width", style.width),
            );
        });

        el.to::<Vec<VectorType<T>>>()
            .into_iter()
            .for_each(|vector| {
                SvgStyle(vector.clone(), style.vecs.clone()).to_svg(document);
            });
    }
}

impl<T: Number + Into<Value>> Svg for SvgStyle<Vec<Line<T>>, LineStyle> {
    fn to_svg(&self, document: &mut Document) {
        SvgStyle(
            self.0
                .iter()
                .map(|line| Rc::new(line.clone()))
                .collect::<Vec<LineType<T>>>(),
            self.1.clone(),
        )
        .to_svg(document);
    }
}