// svg.rs
// Copyright (C) 2025  Douglas P Lau
//
//! SVG Elements
use crate::page::{Element, Page};
use crate::value::Value;

// A element (in SVG context)
macro_rules! a_items {
    ( $el:literal ) => {
        html_attr!($el, download);
        html_attr!($el, href);
        html_attr!($el, hreflang);
        /* interestfor */
        html_attr!($el, ping);
        html_attr!($el, referrerpolicy);
        html_attr!($el, rel);
        html_attr!($el, target);
        html_attr!($el, r#type, "type");
        svg_content!();
    };
}
svg_elem!("a", A, "Anchor", a_items());

// Animate attributes
macro_rules! animate_attr {
    () => {
        svg_attr!(href);
        // animation timing
        svg_attr!(dur);
        svg_attr!(begin);
        svg_attr!(end);
        svg_attr!(min);
        svg_attr!(max);
        svg_attr!(repeat_count, "repeatCount");
        svg_attr!(repeat_dur, "repeatDur");
        svg_attr!(restart);
        svg_attr!(fill);
        // animation value
        svg_attr!(values);
        svg_attr!(from);
        svg_attr!(to);
        svg_attr!(calc_mode, "calcMode");
        svg_attr!(key_points, "keyPoints");
        svg_attr!(key_times, "keyTimes");
        svg_attr!(key_splines, "keySplines");
        svg_attr!(by);
        // animation addition
        svg_attr!(additive);
        svg_attr!(accumulate);
    };
}

// Animate element
macro_rules! animate_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(attribute_name, "attributeName");
        // NOTE: attributeType is deprecated
        animate_attr!();
        svg_descriptive!();
    };
}
svg_elem!("animate", Animate, "Animate", animate_items());

// AnimateMotion element
macro_rules! animate_motion_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(path);
        svg_attr!(rotate);
        animate_attr!();
        svg_descriptive!();
        // FIXME: mpath element
    };
}
svg_elem!(
    "animateMotion",
    AnimateMotion,
    "Animate Motion",
    animate_motion_items()
);

// AnimateTransform element
macro_rules! animate_transform_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        animate_attr!();
        svg_descriptive!();
    };
}
svg_elem!(
    "animateTransform",
    AnimateTransform,
    "Animate Transform",
    animate_transform_items()
);

// Circle element
macro_rules! circle_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(cx);
        svg_attr!(cy);
        svg_attr!(r);
        svg_attr!(path_length, "pathLength");
        svg_animation!();
        svg_descriptive!();
    };
}
svg_elem!("circle", Circle, "Circle", circle_items());

// ClipPath element
macro_rules! clip_path_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(clip_path_units, "clipPathUnits");
        svg_animation!();
        svg_descriptive!();
        // svg_shape!();
        // FIXME: text, use elements
    };
}
svg_elem!("clipPath", ClipPath, "Clip Path", clip_path_items());

// Defs element
macro_rules! defs_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_content!();
    };
}
svg_elem!("defs", Defs, "Definitions", defs_items());

// ForeignObject element
macro_rules! foreign_object_items {
    ( $el:literal ) => {
        // FIXME
    };
}
svg_elem!(
    "foreignObject",
    ForeignObject,
    "Foreign Object",
    foreign_object_items()
);

// Link element
macro_rules! link_items {
    ( $el:literal ) => {
        // FIXME
    };
}
svg_elem!("link", Link, "Link", link_items());

// Svg element
macro_rules! svg_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(view_box, "viewBox");
        svg_attr!(height);
        svg_attr!(width);
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_content!();
    };
}
svg_elem!("svg", Svg, "Svg", svg_items());

// Use element
macro_rules! use_items {
    ( $el:literal ) => {
        // FIXME
    };
}
svg_elem!("use", Use, "Use", use_items());

/*
svg_elements![
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
    switch switch,
    symbol symbol,
    text text,
    textPath text_path,
    title title,
    tspan tspan,
    view view,
];

svg_attributes![
    "accumulate" accumulate,
    "additive" additive,
    "alignment-baseline" alignment_baseline,
    "amplitude" amplitude,
    "attributeName" attribute_name,
    "azimuth" azimuth,
    "baseFrequency" base_frequency,
    "baseline-shift" baseline_shift,
    "baseProfile" base_profile,
    "begin" begin,
    "bias" bias,
    "by" by,
    "calcMode" calc_mode,
    "class" class,
    "clip" clip,
    "clipPathUnits" clip_path_units,
    /* "clip-path" clip_path,*/
    "clip-rule" clip_rule,
    "color" color,
    "color-interpolation" color_interpolation,
    "color-interpolation-filters" color_interpolation_filters,
    "crossorigin" crossorigin,
    "cursor" cursor,
    "cx" cx,
    "cy" cy,
    "d" d,
    "decoding" decoding,
    "diffuseConstant" diffuse_constant,
    "direction" direction,
    "display" display,
    "divisor" divisor,
    "dominant-baseline" dominant_baseline,
    "dur" dur,
    "dx" dx,
    "dy" dy,
    "edgeMode" edge_mode,
    "elevation" elevation,
    /* "end" end, */
    "exponent" exponent,
    "fetchpriority" fetchpriority,
    "fill" fill,
    "fill-opacity" fill_opacity,
    "fill-rule" fill_rule,
    /* "filter" filter, */
    "filterUnits" filter_units,
    "flood-color" flood_color,
    "flood-opacity" flood_opacity,
    "font-family" font_family,
    "font-size" font_size,
    "font-size-adjust" font_size_adjust,
    "font-stretch" font_stretch,
    "font-style" font_style,
    "font-variant" font_variant,
    "font-weight" font_weight,
    "fr" fr,
    "from" from,
    "fx" fx,
    "fy" fy,
    "glyph-orientation-horizontal" glyph_orientation_horizontal,
    "glyph-orientation-vertical" glyph_orientation_vertical,
    "gradientTransform" gradient_transform,
    "gradientUnits" gradient_units,
    "height" height,
    "href" href,
    "hreflang" hreflang,
    "id" id,
    "image-rendering" image_rendering,
    "in" r#in,
    "in2" in2,
    "intercept" intercept,
    "k1" k1,
    "k2" k2,
    "k3" k3,
    "k4" k4,
    "kernelMatrix" kernel_matrix,
    "kernelUnitLength" kernel_unit_length,
    "keyPoints" key_points,
    "keySplines" key_splines,
    "keyTimes" key_times,
    "lang" lang,
    "lengthAdjust" length_adjust,
    "letter-spacing" letter_spacing,
    "lighting-color" lighting_color,
    "limitingConeAngle" limiting_cone_angle,
    "marker-end" marker_end,
    "marker-mid" marker_mid,
    "marker-start" marker_start,
    "markerHeight" marker_height,
    "markerUnits" marker_units,
    "markerWidth" marker_width,
    /* "mask" mask, */
    "maskContentUnits" mask_content_units,
    "maskUnits" mask_units,
    "max" max,
    "media" media,
    "method" method,
    "min" min,
    "mode" mode,
    "numOctaves" num_octaves,
    "offset" offset,
    "opacity" opacity,
    "operator" operator,
    "order" order,
    "orient" orient,
    "origin" origin,
    "overflow" overflow,
    "paint-order" paint_order,
    "path" path,
    "pathLength" path_length,
    "patternContentUnits" pattern_content_units,
    "patternTransform" pattern_transform,
    "patternUnits" pattern_units,
    "ping" ping,
    "pointer-events" pointer_events,
    "points" points,
    "pointsAtX" points_at_x,
    "pointsAtY" points_at_y,
    "pointsAtZ" points_at_z,
    "preserveAlpha" preserve_alpha,
    "preserveAspectRatio" preserve_aspect_ratio,
    "primitiveUnits" primitive_units,
    "r" r,
    "radius" radius,
    "referrerPolicy" referrer_policy,
    "refX" ref_x,
    "refY" ref_y,
    "rel" rel,
    "repeatCount" repeat_count,
    "repeatDur" repeat_dur,
    "requiredExtensions" required_extensions,
    "requiredFeatures" required_features,
    "restart" restart,
    "result" result,
    "rotate" rotate,
    "rx" rx,
    "ry" ry,
    "scale" scale,
    "seed" seed,
    "shape-rendering" shape_rendering,
    "side" side,
    "slope" slope,
    "spacing" spacing,
    "specularConstant" specular_constant,
    "specularExponent" specular_exponent,
    "spreadMethod" spread_method,
    "startOffset" start_offset,
    "stdDeviation" std_deviation,
    "stitchTiles" stitch_tiles,
    "stop-color" stop_color,
    "stop-opacity" stop_opacity,
    "stroke" stroke,
    "stroke-dasharray" stroke_dasharray,
    "stroke-dashoffset" stroke_dashoffset,
    "stroke-linecap" stroke_linecap,
    "stroke-linejoin" stroke_linejoin,
    "stroke-miterlimit" stroke_miterlimit,
    "stroke-opacity" stroke_opacity,
    "stroke-width" stroke_width,
    "style" style,
    "surfaceScale" surface_scale,
    "systemLanguage" system_language,
    "tabindex" tabindex,
    "tableValues" table_values,
    "target" target,
    "targetX" target_x,
    "targetY" target_y,
    "text-anchor" text_anchor,
    "text-decoration" text_decoration,
    "text-rendering" text_rendering,
    "textLength" text_length,
    "to" to,
    "transform" transform,
    "transform-origin" transform_origin,
    "type" r#type,
    "unicode-bidi" unicode_bidi,
    "values" values,
    "vector-effect" vector_effect,
    "version" version,
    "viewBox" view_box,
    "visibility" visibility,
    "width" width,
    "word-spacing" word_spacing,
    "writing-mode" writing_mode,
    "x" x,
    "x1" x1,
    "x2" x2,
    "xChannelSelector" x_channel_selector,
    "xlink:actuate" xlink_actuate,
    "xlink:arcrole" xlink_arcrole,
    "xlink:role" xlink_role,
    "xlink:show" xlink_show,
    "xlink:title" xlink_title,
    "xlink:type" xlink_type,
    "xml:lang" xml_lang,
    "xml:space" xml_space,
    "y" y,
    "y1" y1,
    "y2" y2,
    "yChannelSelector" y_channel_selector,
    "z" z,
    "zoomAndPan" zoom_and_pan,
];*/

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn svg() {
        let mut page = Page::default();
        let _svg = page.frag::<Svg>();
        assert_eq!(page.to_string(), "<svg />");
    }

    #[test]
    fn circle() {
        let mut page = Page::default();
        let mut svg = page.frag::<Svg>();
        svg.circle().cx("50").cy("25").r("5");
        assert_eq!(
            page.to_string(),
            "<svg><circle cx=\"50\" cy=\"25\" r=\"5\" /></svg>"
        );
    }

    #[test]
    fn path() {
        let mut page = Page::default();
        let mut svg = page.frag::<Svg>();
        let mut path = crate::PathDef::new();
        path.absolute(true)
            .move_to((0, 0))
            .line((100, 0))
            .line((50, 50))
            .close();
        svg.path().d(String::from(path));
        assert_eq!(
            page.to_string(),
            "<svg><path d=\"M0 0H100L50 50z\" /></svg>"
        );
    }
}
