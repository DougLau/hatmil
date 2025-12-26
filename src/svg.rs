// svg.rs
// Copyright (C) 2025  Douglas P Lau
//
//! SVG Elements -- _Scalable Vector Graphics_
use crate::html::Link;
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

// Desc element
macro_rules! desc_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        // FIXME: character data content
    };
}
svg_elem!("desc", Desc, "Description", desc_items());

// Ellipse element
macro_rules! ellipse_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(cx);
        svg_attr!(cy);
        svg_attr!(rx);
        svg_attr!(ry);
        svg_attr!(path_length, "pathLength");
        svg_animation!();
        svg_descriptive!();
    };
}
svg_elem!("ellipse", Ellipse, "Ellipse", ellipse_items());

// Filter primitive attributes
macro_rules! filter_attr {
    () => {
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(result);
        svg_attr!(color_interpolation_filters, "color-interpolation-filters");
    };
}

// FeBlend element
macro_rules! fe_blend_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        svg_attr!(in2);
        svg_attr!(mode);
        filter_attr!();
        // FIXME: animate, set
    };
}
svg_elem!("feBlend", FeBlend, "Filter Effect: Blend", fe_blend_items());

// FeColorMatrix element
macro_rules! fe_color_matrix_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        svg_attr!(r#type, "type");
        svg_attr!(values);
        filter_attr!();
        // FIXME: animate, set
    };
}
svg_elem!(
    "feColorMatrix",
    FeColorMatrix,
    "Filter Effect: Color Matrix",
    fe_color_matrix_items()
);

// FeComponentTransfer element
macro_rules! fe_component_transfer_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        filter_attr!();
        // FIXME: feFuncA, feFuncR, feFuncG, feFuncB
    };
}
svg_elem!(
    "feComponentTransfer",
    FeComponentTransfer,
    "Filter Effect: Component Transfer",
    fe_component_transfer_items()
);

// FeComposite element
macro_rules! fe_composite_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        svg_attr!(in2);
        svg_attr!(operator);
        svg_attr!(k1);
        svg_attr!(k2);
        svg_attr!(k3);
        svg_attr!(k4);
        filter_attr!();
        // FIXME: animate, set
    };
}
svg_elem!(
    "feComposite",
    FeComposite,
    "Filter Effect: Composite",
    fe_composite_items()
);

// FeConvolveMatrix element
macro_rules! fe_convolve_matrix_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        svg_attr!(order);
        svg_attr!(kernel_matrix, "kernelMatrix");
        svg_attr!(divisor);
        svg_attr!(bias);
        svg_attr!(target_x, "targetX");
        svg_attr!(target_y, "targetY");
        svg_attr!(edge_mode, "edgeMode");
        svg_attr!(kernel_unit_length, "kernelUnitLength");
        svg_attr!(preserve_alpha, "preserveAlpha");
        filter_attr!();
        // FIXME: animate, set
    };
}
svg_elem!(
    "feConvolveMatrix",
    FeConvolveMatrix,
    "Filter Effect: Convolve Matrix",
    fe_convolve_matrix_items()
);

// FeDiffuseLighting element
macro_rules! fe_diffuse_lighting_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        svg_attr!(surface_scale, "surfaceScale");
        svg_attr!(diffuse_constant, "diffuseConstant");
        svg_attr!(kernel_unit_length, "kernelUnitLength");
        filter_attr!();
        // FIXME: one light source!
        svg_descriptive!();
    };
}
svg_elem!(
    "feDiffuseLighting",
    FeDiffuseLighting,
    "Filter Effect: Diffuse Lighting",
    fe_diffuse_lighting_items()
);

// FeDisplacementMap element
macro_rules! fe_displacement_map_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        svg_attr!(in2);
        svg_attr!(scale);
        svg_attr!(x_channel_selector, "xChannelSelector");
        svg_attr!(y_channel_selector, "yChannelSelector");
        filter_attr!();
        // FIXME: animate, set
    };
}
svg_elem!(
    "feDisplacementMap",
    FeDisplacementMap,
    "Filter Effect: Displacement Map",
    fe_displacement_map_items()
);

// FeDistantLight element
macro_rules! fe_distant_light_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(azimuth);
        svg_attr!(elevation);
        // FIXME: animate, set
    };
}
svg_elem!(
    "feDistantLight",
    FeDistantLight,
    "Filter Effect: Distant Light",
    fe_distant_light_items()
);

// FeDropShadow element
macro_rules! fe_drop_shadow_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        svg_attr!(dx);
        svg_attr!(dy);
        svg_attr!(std_deviation, "stdDeviation");
        svg_attr!(flood_color, "flood-color");
        svg_attr!(flood_opacity, "flood-opacity");
        // FIXME: animate, script, set
    };
}
svg_elem!(
    "feDropShadow",
    FeDropShadow,
    "Filter Effect: Drop Shadow",
    fe_drop_shadow_items()
);

// FeFlood element
macro_rules! fe_flood_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(flood_color, "flood-color");
        svg_attr!(flood_opacity, "flood-opacity");
        filter_attr!();
        // FIXME: animate, set
    };
}
svg_elem!("feFlood", FeFlood, "Filter Effect: Flood", fe_flood_items());

// Transfer function attributes
macro_rules! transfer_func_attr {
    () => {
        // transfer func: identity, table, discrete, linear, gamma
        svg_attr!(r#type, "type");
        // discrete/table func attributes
        svg_attr!(table_values, "tableValues");
        // linear func attributes
        svg_attr!(slope);
        svg_attr!(intercept);
        // gamma func attributes
        svg_attr!(amplitude);
        svg_attr!(exponent);
        svg_attr!(offset);
    };
}

// FeFunc[RGBA] element
macro_rules! fe_func_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        transfer_func_attr!();
        // FIXME: animate, set
    };
}
svg_elem!(
    "feFuncA",
    FeFuncA,
    "Filter Effect: Alpha Transfer",
    fe_func_items()
);
svg_elem!(
    "feFuncB",
    FeFuncB,
    "Filter Effect: Blue Transfer",
    fe_func_items()
);
svg_elem!(
    "feFuncG",
    FeFuncG,
    "Filter Effect: Green Transfer",
    fe_func_items()
);
svg_elem!(
    "feFuncR",
    FeFuncR,
    "Filter Effect: Red Transfer",
    fe_func_items()
);

// FeGaussianBlur element
macro_rules! fe_gaussian_blur_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        svg_attr!(std_deviation, "stdDeviation");
        svg_attr!(edge_mode, "edgeMode");
        filter_attr!();
        // FIXME: animate, set
    };
}
svg_elem!(
    "feGaussianBlur",
    FeGaussianBlur,
    "Filter Effect: Gaussian Blur",
    fe_gaussian_blur_items()
);

// FeImage element
macro_rules! fe_image_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(href);
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_attr!(crossorigin);
        // FIXME: fetchpriority
        filter_attr!();
        // FIXME: animate, animateTransform, set
    };
}
svg_elem!("feImage", FeImage, "Filter Effect: Image", fe_image_items());

// FeMerge element
macro_rules! fe_merge_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        filter_attr!();
        // FIXME: feMergeNode
    };
}
svg_elem!("feMerge", FeMerge, "Filter Effect: Merge", fe_merge_items());

// FeMergeNode element
macro_rules! fe_merge_node_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        // FIXME: animate, set
    };
}
svg_elem!(
    "feMergeNode",
    FeMergeNode,
    "Filter Effect: Merge Node",
    fe_merge_node_items()
);

// FeMorphology element
macro_rules! fe_morphology_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        svg_attr!(operator);
        svg_attr!(radius);
        filter_attr!();
        // FIXME: animate, set
    };
}
svg_elem!(
    "feMorphology",
    FeMorphology,
    "Filter Effect: Morphology",
    fe_morphology_items()
);

// FeOffset element
macro_rules! fe_offset_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        svg_attr!(dx);
        svg_attr!(dy);
        filter_attr!();
        // FIXME: animate, set
    };
}
svg_elem!(
    "feOffset",
    FeOffset,
    "Filter Effect: Offset",
    fe_offset_items()
);

// FePointLight element
macro_rules! fe_point_light_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(z);
        // FIXME: animate, set
    };
}
svg_elem!(
    "fePointLight",
    FePointLight,
    "Filter Effect: Point Light",
    fe_point_light_items()
);

// FeSpecularLighting element
macro_rules! fe_specular_lighting_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        svg_attr!(surface_scale, "surfaceScale");
        svg_attr!(specular_constant, "specularConstant");
        svg_attr!(specular_exponent, "specularExponent");
        svg_attr!(kernel_unit_length, "kernelUnitLength");
        filter_attr!();
        // FIXME: feDistantLight, fePointLight, feSpotLight (only one)
        svg_descriptive!();
    };
}
svg_elem!(
    "feSpecularLighting",
    FeSpecularLighting,
    "Filter Effect: Specular Lighting",
    fe_specular_lighting_items()
);

// FeSpotLight element
macro_rules! fe_spot_light_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(z);
        svg_attr!(points_at_x, "pointsAtX");
        svg_attr!(points_at_y, "pointsAtY");
        svg_attr!(points_at_z, "pointsAtZ");
        svg_attr!(specular_exponent, "specularExponent");
        svg_attr!(limiting_cone_angle, "limitingConeAngle");
        // FIXME: animate, set
    };
}
svg_elem!(
    "feSpotLight",
    FeSpotLight,
    "Filter Effect: Spot Light",
    fe_spot_light_items()
);

// FeTile element
macro_rules! fe_tile_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#in, "in");
        filter_attr!();
        // FIXME: animate, set
    };
}
svg_elem!("feTile", FeTile, "Filter Effect: Tile", fe_tile_items());

// FeTurbulence element
macro_rules! fe_turbulence_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(base_frequency, "baseFrequency");
        svg_attr!(num_octaves, "numOctaves");
        svg_attr!(seed);
        svg_attr!(stitch_tiles, "stitchTiles");
        svg_attr!(r#type, "type");
        filter_attr!();
        // FIXME: animate, set
    };
}
svg_elem!(
    "feTurbulence",
    FeTurbulence,
    "Filter Effect: Turbulence",
    fe_turbulence_items()
);

// Filter element
macro_rules! filter_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(filter_units, "filterUnits");
        svg_attr!(primitive_units, "primitiveUnits");
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_descriptive!();
        // FIXME: filter primitive elements
        // FIXME: animate, set
    };
}
svg_elem!("filter", Filter, "Filter", filter_items());

// ForeignObject element
macro_rules! foreign_object_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        // FIXME: any elements or character data
    };
}
svg_elem!(
    "foreignObject",
    ForeignObject,
    "Foreign Object",
    foreign_object_items()
);

// G element
macro_rules! g_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_content!();
    };
}
svg_elem!("g", G, "Group", g_items());

// Image element
macro_rules! image_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(href);
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_attr!(crossorigin);
        svg_attr!(decoding);
        // FIXME: fetchpriority
        svg_animation!();
        svg_descriptive!();
        // FIXME: script, style
    };
}
svg_elem!("image", Image, "Image", image_items());

// Line element
macro_rules! line_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(x1);
        svg_attr!(y1);
        svg_attr!(x2);
        svg_attr!(y2);
        svg_attr!(path_length, "pathLength");
        svg_animation!();
        svg_descriptive!();
    };
}
svg_elem!("line", Line, "Line", line_items());

// LinearGradient element
macro_rules! linear_gradient_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(x1);
        svg_attr!(y1);
        svg_attr!(x2);
        svg_attr!(y2);
        svg_attr!(gradient_units, "gradientUnits");
        svg_attr!(gradient_transform, "gradientTransform");
        svg_attr!(spread_method, "spreadMethod");
        svg_attr!(href);
        svg_descriptive!();
        // FIXME: animate, animateTransform, script, set, stop, style
    };
}
svg_elem!(
    "linearGradient",
    LinearGradient,
    "Linear Gradient",
    linear_gradient_items()
);

// Marker element
macro_rules! marker_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(marker_width, "markerWidth");
        svg_attr!(marker_height, "markerHeight");
        svg_attr!(marker_units, "markerUnits");
        svg_attr!(orient);
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_attr!(ref_x, "refX");
        svg_attr!(ref_y, "refY");
        svg_attr!(view_box, "viewBox");
        svg_content!();
    };
}
svg_elem!("marker", Marker, "Marker", marker_items());

// Mask element
macro_rules! mask_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(mask_type, "mask-type");
        svg_attr!(mask_content_units, "maskContentUnits");
        svg_attr!(mask_units, "maskUnits");
        svg_content!();
    };
}
svg_elem!("mask", Mask, "Mask", mask_items());

// Metadata element
macro_rules! metadata_items {
    ( $el:literal ) => {
        // FIXME: any elements or character data
    };
}
svg_elem!("metadata", Metadata, "Metadata", metadata_items());

// MPath element
macro_rules! mpath_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(href);
        svg_descriptive!();
    };
}
svg_elem!("mpath", MPath, "Motion Path", mpath_items());

// Path element
macro_rules! path_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(d);
        svg_attr!(path_length, "pathLength");
        svg_animation!();
        svg_descriptive!();
    };
}
svg_elem!("path", Path, "Path", path_items());

// Pattern element
macro_rules! pattern_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(href);
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(pattern_content_units, "patternContentUnits");
        svg_attr!(pattern_units, "patternUnits");
        svg_attr!(pattern_transform, "patternTransform");
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_attr!(view_box, "viewBox");
        svg_content!();
    };
}
svg_elem!("pattern", Pattern, "Pattern", pattern_items());

// Polygon element
macro_rules! polygon_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(points);
        svg_attr!(path_length, "pathLength");
        svg_animation!();
        svg_descriptive!();
    };
}
svg_elem!("polygon", Polygon, "Polygon", polygon_items());

// PolyLine element
macro_rules! polyline_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(points);
        svg_attr!(path_length, "pathLength");
        svg_animation!();
        svg_descriptive!();
    };
}
svg_elem!("polyline", Polyline, "Polyline", polyline_items());

// RadialGradient element
macro_rules! radial_gradient_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(cx);
        svg_attr!(cy);
        svg_attr!(fr);
        svg_attr!(fx);
        svg_attr!(fy);
        svg_attr!(r);
        svg_attr!(gradient_units, "gradientUnits");
        svg_attr!(gradient_transform, "gradientTransform");
        svg_attr!(spread_method, "spreadMethod");
        svg_attr!(href);
        svg_descriptive!();
        // FIXME: animate, animateTransform, script, set, stop, style
    };
}
svg_elem!(
    "radialGradient",
    RadialGradient,
    "Radial Gradient",
    radial_gradient_items()
);

// Rect element
macro_rules! rect_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(rx);
        svg_attr!(ry);
        svg_attr!(path_length, "pathLength");
        svg_animation!();
        svg_descriptive!();
    };
}
svg_elem!("rect", Rect, "Rectangle", rect_items());

// Script element
macro_rules! script_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(crossorigin);
        // FIXME: fetchpriority
        svg_attr!(href);
        svg_attr!(r#type, "type");
        // FIXME: any elements or character data
    };
}
svg_elem!("script", Script, "Script", script_items());

// Set element
macro_rules! set_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(attribute_name, "attributeName");
        svg_attr!(to);
        svg_attr!(dur);
        svg_attr!(begin);
        svg_attr!(end);
        svg_attr!(min);
        svg_attr!(max);
        svg_attr!(repeat_count, "repeatCount");
        svg_attr!(repeat_dur, "repeatDur");
        svg_attr!(restart);
        svg_attr!(fill);
        svg_descriptive!();
    };
}
svg_elem!("set", Set, "Set Value", set_items());

// Stop element
macro_rules! stop_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(offset);
        svg_attr!(stop_color, "stop-color");
        svg_attr!(stop_opacity, "stop-opacity");
        // FIXME: animate, script, set, style
    };
}
svg_elem!("stop", Stop, "Gradient Stop", stop_items());

// Style element
macro_rules! style_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(r#type, "type");
        svg_attr!(media);
        svg_attr!(title);
        // FIXME: any elements or character data
    };
}
svg_elem!("style", Style, "Style Information", style_items());

// Svg element
macro_rules! svg_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(view_box, "viewBox");
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_attr!(xmlns);
        svg_content!();
        elem_method!(link, Link);
    };
}
svg_elem!("svg", Svg, "Svg", svg_items());

// Switch element
macro_rules! switch_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(required_extensions, "requiredExtensions");
        svg_attr!(system_language, "systemLanguage");
        svg_content!();
    };
}
svg_elem!("switch", Switch, "Switch", switch_items());

// Symbol element
macro_rules! symbol_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_attr!(ref_x, "refX");
        svg_attr!(ref_y, "refY");
        svg_attr!(view_box, "viewBox");
        svg_content!();
    };
}
svg_elem!("symbol", Symbol, "Symbol", symbol_items());

// Text element
macro_rules! text_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(dx);
        svg_attr!(dy);
        svg_attr!(rotate);
        svg_attr!(length_adjust, "lengthAdjust");
        svg_attr!(text_length, "textLength");
        svg_animation!();
        svg_descriptive!();
        // FIXME: text content + a (anchor)
    };
}
svg_elem!("text", Text, "Text", text_items());

// TextPath element
macro_rules! text_path_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(href);
        svg_attr!(method);
        svg_attr!(length_adjust, "lengthAdjust");
        svg_attr!(text_length, "textLength");
        svg_attr!(spacing);
        svg_attr!(start_offset, "startOffset");
        // FIXME: path, side
        svg_descriptive!();
        // FIXME: text content, a, animate, set, tspan
    };
}
svg_elem!("textPath", TextPath, "Text Path", text_path_items());

// Title element
macro_rules! title_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        // FIXME: any elements or character data
    };
}
svg_elem!("title", Title, "Title", title_items());

// TSpan element
macro_rules! tspan_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(dx);
        svg_attr!(dy);
        svg_attr!(rotate);
        svg_attr!(length_adjust, "lengthAdjust");
        svg_attr!(text_length, "textLength");
        svg_descriptive!();
        // FIXME: text content + animate, set, tspan
    };
}
svg_elem!("tspan", TSpan, "Text Span", tspan_items());

// Use element
macro_rules! use_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(href);
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_animation!();
        svg_descriptive!();
    };
}
svg_elem!("use", Use, "Use", use_items());

// View element
macro_rules! view_items {
    ( $el:literal ) => {
        // FIXME: global attributes: id
        svg_attr!(view_box, "viewBox");
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_descriptive!();
    };
}
svg_elem!("view", View, "View", view_items());

/*
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
