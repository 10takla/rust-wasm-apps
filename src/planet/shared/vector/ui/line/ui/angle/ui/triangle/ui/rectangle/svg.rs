use super::Rectangle;
use crate::planet::shared::{
    traits::{Svg, SvgStyle},
    vector::{
        svg::VectorStyle,
        ui::line::{svg::LineStyle, ui::angle::ui::triangle::svg::TriangleStyle},
        Number,
    },
};
use macros::{setter, Default};

#[setter]
#[derive(Default, Clone)]
pub struct RectangleStyle {
    #[default_field = "blue"]
    pub color: &'static str,

    pub tries: TriangleStyle,
    pub lines: LineStyle,
    pub vecs: VectorStyle,
}

impl<T: Number> Svg for Rectangle<T> {
    fn to_svg(&self, document: &mut svg::Document) {
        self.clone().into_iter().for_each(|tri| {
            SvgStyle(
                tri,
                TriangleStyle::default().set_color(RectangleStyle::default().color),
            )
            .to_svg(document);
        })
    }
}
