use super::{Number, VectorType};
use crate::{
    planet::shared::traits::{Svg, SvgStyle},
    traits::{
        as_prim::AsPrim,
        of_to::{Of, To},
    },
};
use macros::{setter, Default};
use svg::{
    node::{
        element::{Circle, Text},
        Value,
    },
    Document,
};

#[derive(Clone, Default, Debug)]
#[setter]
pub struct TextStyle {
    #[default_field = "white"]
    pub color: &'static str,

    #[default_field = "0.15px"]
    pub font_size: &'static str,

    #[default_field = 0.1]
    pub offset: f64,
}

#[derive(Clone, Default, Debug)]
#[setter]
pub struct VectorStyle {
    #[default_field = "black"]
    pub color: &'static str,

    #[default_field = 0.3]
    pub width: f64,

    pub text: Option<TextStyle>,
}

impl<T: Into<Value> + Number> Svg for VectorType<T> {
    fn to_svg(&self, document: &mut Document) {
        SvgStyle::new(self).to_svg(document);
    }
}

impl<T: Number + Into<Value>> Svg for Vec<VectorType<T>> {
    fn to_svg(&self, document: &mut Document) {
        SvgStyle::new(self).to_svg(document);
    }
}

impl<T: Into<Value> + Number> Svg for SvgStyle<VectorType<T>, VectorStyle> {
    fn to_svg(&self, document: &mut Document) {
        let SvgStyle(el, style) = self;

        let mut circle = Circle::new()
            .set("cx", el[0])
            .set("cy", el[1])
            .set("r", style.width)
            .set("fill", style.color);

        *document = document.clone().add(circle);
        
        if let Some(TextStyle {
            font_size,
            color,
            offset,
        }) = style.text
        {
            let text = Text::new(format!("({}, {})", el[0], el[1]))
                .set("x", el[0] + offset.as_())
                .set("y", el[1] - offset.as_())
                .set("font-size", font_size)
                .set("fill", color);
            
            *document = document.clone().add(text);
        }
    }
}

impl<T: Number + Into<Value>> Svg for SvgStyle<Vec<VectorType<T>>, VectorStyle> {
    fn to_svg(&self, document: &mut Document) {
        self.0
            .clone()
            .into_iter()
            .for_each(|vector| SvgStyle(vector, self.1.clone()).to_svg(document));
    }
}
