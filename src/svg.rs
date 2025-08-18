// svg.rs
// Copyright (C) 2025  Douglas P Lau
//
use crate::html::{Elem, Html, VoidElem};

/// SVG element borrowed from [Html svg] method
///
/// [Html svg]: struct.Html.html#method.svg
pub struct Svg<'h> {
    html: &'h mut Html,
}

impl<'h> Svg<'h> {
    /// Create a new SVG element
    pub fn new(html: &'h mut Html) -> Self {
        Svg { html }
    }

    /// Add an attribute with value
    ///
    /// The characters `&` and `"` in `val` will automatically be escaped.
    pub fn attr(self, attr: &'static str, val: impl AsRef<str>) -> Self {
        self.html.attr(attr, val);
        self
    }

    /// End the element
    ///
    /// Adds the closing tag (e.g. `</svg>`)
    pub fn end(&mut self) {
        self.html.end();
    }

    /// Add [foreignObject] child element
    ///
    /// [foreignObject]: https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/foreignObject
    pub fn foreign_object(self) -> Elem<'h> {
        self.html.elem("foreignObject")
    }

    /// Add [link] child element (e.g. [CSS])
    ///
    /// [CSS]: https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorials/SVG_from_scratch/SVG_and_CSS
    /// [link]: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/link
    pub fn link(self) -> VoidElem<'h> {
        self.html.void_elem("link")
    }

    /// Add [use] child element
    ///
    /// NOTE: use `r#use()` to invoke
    ///
    /// [use]: https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/use
    pub fn r#use(self) -> Self {
        self.html.svg_elem("use")
    }
}

/// SVG element helper
macro_rules! svg_elements {
    ( $( $elem:ident $snake:ident ),* $(,)? ) => {
        impl<'h> Svg<'h> {
            $(
                #[doc = concat!("Add [", stringify!($elem), "](")]
                #[doc = concat!(
                    "https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/",
                     stringify!($elem)
                )]
                #[doc = ") child element"]
                pub fn $snake(self) -> Self {
                    self.html.svg_elem(stringify!($elem))
                }
            )*
        }
    }
}

svg_elements![
    a a,
    animate animate,
    animateMotion animate_motion,
    animateTransform animate_transform,
    circle circle,
    clipPath clip_path,
    defs defs,
    desc desc,
    ellipse ellipse,
    feBlend fe_blend,
    feComponentTransfer fe_component_transfer,
    feComposite fe_composite,
    feConvolveMatrix fe_convolve_matrix,
    feDiffuseLighting fe_diffuse_lighting,
    feDisplacementMap fe_displacement_map,
    feDistantLight fe_distant_light,
    feDropShadow fe_drop_shadow,
    feFlood fe_flood,
    feFuncA fe_func_a,
    feFuncB fe_func_b,
    feFuncG fe_func_g,
    feFuncR fe_func_r,
    feGaussianBlur fe_gaussian_blur,
    feImage fe_image,
    feMerge fe_merge,
    feMergeNode fe_merge_node,
    feMorphology fe_morphology,
    feOffset fe_offset,
    fePointLight fe_point_light,
    feSpecularLighting fe_specular_lighting,
    feSpotLight fe_spot_light,
    feTile fe_tile,
    feTurbulence fe_turbulence,
    filter filter,
    g g,
    image image,
    line line,
    linearGradient linear_gradient,
    marker marker,
    mask mask,
    metadata metadata,
    mpath mpath,
    path path,
    pattern pattern,
    polygon polygon,
    polyline polyline,
    radialGradient radial_gradient,
    rect rect,
    script script,
    set set,
    stop stop,
    style style,
    svg svg,
    switch switch,
    symbol symbol,
    text text,
    textPath text_path,
    title title,
    tspan tspan,
    view view,
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn svg() {
        let mut html = Html::new();
        html.svg();
        assert_eq!(html.to_string(), "<svg></svg>");
    }

    #[test]
    fn circle() {
        let mut html = Html::new();
        html.svg()
            .circle()
            .attr("cx", "50")
            .attr("cy", "25")
            .attr("r", "5");
        assert_eq!(
            html.to_string(),
            "<svg><circle cx=\"50\" cy=\"25\" r=\"5\"></circle></svg>"
        );
    }
}
