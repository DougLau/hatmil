// svg.rs
// Copyright (C) 2025-2026  Douglas P Lau
//
//! SVG Elements -- _Scalable Vector Graphics_
use crate::definition::PathDefBuilder;
use crate::html::Link;
use crate::page::{ElemType, Element, Page};
use crate::poly::PolyPointBuilder;
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
        svg_support_attr!();
        svg_content!(title);
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
        svg_attr!(to);
        svg_attr!(from);
        svg_attr!(values);
        svg_attr!(calc_mode, "calcMode");
        svg_attr!(key_points, "keyPoints");
        svg_attr!(key_times, "keyTimes");
        svg_attr!(key_splines, "keySplines");
        svg_attr!(by);
        // animation addition
        svg_attr!(additive);
        svg_attr!(accumulate);
        svg_support_attr!();
    };
}

// Animate element
macro_rules! animate_items {
    ( $el:literal ) => {
        svg_attr!(attribute_name, "attributeName");
        // NOTE: attributeType is deprecated
        animate_attr!();
        svg_descriptive!(title);
    };
}
svg_elem!("animate", Animate, "Animate", animate_items());

// AnimateMotion element
macro_rules! animate_motion_items {
    ( $el:literal ) => {
        svg_attr!(path);
        svg_attr!(rotate);
        svg_attr!(origin);
        animate_attr!();
        elem_method!(mpath, MPath);
        svg_descriptive!(title);
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
        svg_attr!(attribute_name, "attributeName");
        html_attr!($el, r#type, "type");
        // NOTE: attributeType is deprecated
        animate_attr!();
        svg_descriptive!(title);
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
        svg_attr!(cx);
        svg_attr!(cy);
        svg_attr!(r);
        svg_attr!(path_length, "pathLength");
        svg_support_attr!();
        svg_descriptive!(title);
        svg_animation!();
    };
}
svg_elem!("circle", Circle, "Circle", circle_items());

// ClipPath element
macro_rules! clip_path_items {
    ( $el:literal ) => {
        svg_attr!(clip_path_units, "clipPathUnits");
        svg_support_attr!();
        svg_graphics!();
        svg_descriptive!(title);
        svg_animation!();
    };
}
svg_elem!("clipPath", ClipPath, "Clip Path", clip_path_items());

// Defs element
macro_rules! defs_items {
    ( $el:literal ) => {
        svg_content!(title);
        svg_support_attr!();
    };
}
svg_elem!("defs", Defs, "Definitions", defs_items());

// Desc element
macro_rules! desc_items {
    ( $el:literal ) => {
        text_content!();
    };
}
svg_elem!("desc", Desc, "Description", desc_items());

// Ellipse element
macro_rules! ellipse_items {
    ( $el:literal ) => {
        svg_attr!(cx);
        svg_attr!(cy);
        svg_attr!(rx);
        svg_attr!(ry);
        svg_attr!(path_length, "pathLength");
        svg_support_attr!();
        svg_descriptive!(title);
        svg_animation!();
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
    };
}

// FeBlend element
macro_rules! fe_blend_items {
    ( $el:literal ) => {
        svg_attr!(r#in, "in");
        svg_attr!(in2);
        svg_attr!(mode);
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(set, Set);
    };
}
svg_elem!("feBlend", FeBlend, "Filter Effect: Blend", fe_blend_items());

// FeColorMatrix element
macro_rules! fe_color_matrix_items {
    ( $el:literal ) => {
        svg_attr!(r#in, "in");
        svg_attr!(r#type, "type");
        svg_attr!(values);
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(r#in, "in");
        filter_attr!();
        elem_method!(fe_func_a, FeFuncA);
        elem_method!(fe_func_r, FeFuncR);
        elem_method!(fe_func_g, FeFuncG);
        elem_method!(fe_func_b, FeFuncB);
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
        svg_attr!(r#in, "in");
        svg_attr!(in2);
        svg_attr!(operator);
        svg_attr!(k1);
        svg_attr!(k2);
        svg_attr!(k3);
        svg_attr!(k4);
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(r#in, "in");
        svg_attr!(surface_scale, "surfaceScale");
        svg_attr!(diffuse_constant, "diffuseConstant");
        svg_attr!(kernel_unit_length, "kernelUnitLength");
        filter_attr!();
        // NOTE: only one light allowed!
        elem_method!(fe_distant_light, FeDistantLight);
        elem_method!(fe_point_light, FePointLight);
        elem_method!(fe_spot_light, FeSpotLight);
        svg_descriptive!(title);
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
        svg_attr!(r#in, "in");
        svg_attr!(in2);
        svg_attr!(scale);
        svg_attr!(x_channel_selector, "xChannelSelector");
        svg_attr!(y_channel_selector, "yChannelSelector");
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(azimuth);
        svg_attr!(elevation);
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(r#in, "in");
        svg_attr!(dx);
        svg_attr!(dy);
        svg_attr!(std_deviation, "stdDeviation");
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(set, Set);
        elem_method!(script, Script);
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
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(set, Set);
    };
}
svg_elem!("feFlood", FeFlood, "Filter Effect: Flood", fe_flood_items());

// FeFunc[RGBA] element
macro_rules! fe_func_items {
    ( $el:literal ) => {
        // transfer type: identity, table, discrete, linear, gamma
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
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(r#in, "in");
        svg_attr!(std_deviation, "stdDeviation");
        svg_attr!(edge_mode, "edgeMode");
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(href);
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_attr!(crossorigin);
        // NOTE: fetchpriority (future)
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(animate_transform, AnimateTransform);
        elem_method!(set, Set);
    };
}
svg_elem!("feImage", FeImage, "Filter Effect: Image", fe_image_items());

// FeMerge element
macro_rules! fe_merge_items {
    ( $el:literal ) => {
        filter_attr!();
        elem_method!(fe_merge_node, FeMergeNode);
    };
}
svg_elem!("feMerge", FeMerge, "Filter Effect: Merge", fe_merge_items());

// FeMergeNode element
macro_rules! fe_merge_node_items {
    ( $el:literal ) => {
        svg_attr!(r#in, "in");
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(r#in, "in");
        svg_attr!(operator);
        svg_attr!(radius);
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(r#in, "in");
        svg_attr!(dx);
        svg_attr!(dy);
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(z);
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(r#in, "in");
        svg_attr!(surface_scale, "surfaceScale");
        svg_attr!(specular_constant, "specularConstant");
        svg_attr!(specular_exponent, "specularExponent");
        svg_attr!(kernel_unit_length, "kernelUnitLength");
        filter_attr!();
        // NOTE: only one light allowed!
        elem_method!(fe_distant_light, FeDistantLight);
        elem_method!(fe_point_light, FePointLight);
        elem_method!(fe_spot_light, FeSpotLight);
        svg_descriptive!(title);
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
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(z);
        svg_attr!(points_at_x, "pointsAtX");
        svg_attr!(points_at_y, "pointsAtY");
        svg_attr!(points_at_z, "pointsAtZ");
        svg_attr!(specular_exponent, "specularExponent");
        svg_attr!(limiting_cone_angle, "limitingConeAngle");
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(r#in, "in");
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(set, Set);
    };
}
svg_elem!("feTile", FeTile, "Filter Effect: Tile", fe_tile_items());

// FeTurbulence element
macro_rules! fe_turbulence_items {
    ( $el:literal ) => {
        svg_attr!(base_frequency, "baseFrequency");
        svg_attr!(num_octaves, "numOctaves");
        svg_attr!(seed);
        svg_attr!(stitch_tiles, "stitchTiles");
        svg_attr!(r#type, "type");
        filter_attr!();
        elem_method!(animate, Animate);
        elem_method!(set, Set);
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
        svg_attr!(filter_units, "filterUnits");
        svg_attr!(primitive_units, "primitiveUnits");
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_descriptive!(title);
        elem_method!(fe_blend, FeBlend);
        elem_method!(fe_color_matrix, FeColorMatrix);
        elem_method!(fe_component_transfer, FeComponentTransfer);
        elem_method!(fe_composite, FeComposite);
        elem_method!(fe_convolve_matrix, FeConvolveMatrix);
        elem_method!(fe_diffuse_lighting, FeDiffuseLighting);
        elem_method!(fe_displacement_map, FeDisplacementMap);
        elem_method!(fe_drop_shadow, FeDropShadow);
        elem_method!(fe_flood, FeFlood);
        elem_method!(fe_func_a, FeFuncA);
        elem_method!(fe_func_r, FeFuncR);
        elem_method!(fe_func_g, FeFuncG);
        elem_method!(fe_func_b, FeFuncB);
        elem_method!(fe_gaussian_blur, FeGaussianBlur);
        elem_method!(fe_image, FeImage);
        elem_method!(fe_merge, FeMerge);
        elem_method!(fe_merge_node, FeMergeNode);
        elem_method!(fe_morphology, FeMorphology);
        elem_method!(fe_offset, FeOffset);
        elem_method!(fe_specular_lighting, FeSpecularLighting);
        elem_method!(fe_tile, FeTile);
        elem_method!(fe_turbulence, FeTurbulence);
        elem_method!(animate, Animate);
        elem_method!(set, Set);
    };
}
svg_elem!("filter", Filter, "Filter", filter_items());

// ForeignObject element
macro_rules! foreign_object_items {
    ( $el:literal ) => {
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_support_attr!();
        svg_content!(title);
        text_content!();
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
        svg_support_attr!();
        svg_content!(title);
    };
}
svg_elem!("g", G, "Group", g_items());

// Image element
macro_rules! image_items {
    ( $el:literal ) => {
        svg_attr!(href);
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_attr!(crossorigin);
        svg_attr!(decoding);
        // NOTE: fetchpriority (future)
        svg_support_attr!();
        svg_descriptive!(title);
        svg_animation!();
        elem_method!(script, Script);
        elem_method!(style_el, Style);
    };
}
svg_elem!("image", Image, "Image", image_items());

// Line element
macro_rules! line_items {
    ( $el:literal ) => {
        svg_attr!(x1);
        svg_attr!(y1);
        svg_attr!(x2);
        svg_attr!(y2);
        svg_attr!(path_length, "pathLength");
        svg_support_attr!();
        svg_descriptive!(title);
        svg_animation!();
    };
}
svg_elem!("line", Line, "Line", line_items());

// LinearGradient element
macro_rules! linear_gradient_items {
    ( $el:literal ) => {
        svg_attr!(href);
        svg_attr!(x1);
        svg_attr!(y1);
        svg_attr!(x2);
        svg_attr!(y2);
        svg_attr!(gradient_units, "gradientUnits");
        svg_attr!(gradient_transform, "gradientTransform");
        svg_attr!(spread_method, "spreadMethod");
        svg_descriptive!(title);
        elem_method!(animate, Animate);
        elem_method!(animate_transform, AnimateTransform);
        elem_method!(set, Set);
        elem_method!(stop, Stop);
        elem_method!(script, Script);
        elem_method!(style_el, Style);
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
        svg_attr!(marker_width, "markerWidth");
        svg_attr!(marker_height, "markerHeight");
        svg_attr!(marker_units, "markerUnits");
        svg_attr!(orient);
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_attr!(ref_x, "refX");
        svg_attr!(ref_y, "refY");
        svg_attr!(view_box, "viewBox");
        svg_content!(title);
    };
}
svg_elem!("marker", Marker, "Marker", marker_items());

// Mask element
macro_rules! mask_items {
    ( $el:literal ) => {
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(mask_content_units, "maskContentUnits");
        svg_attr!(mask_units, "maskUnits");
        svg_support_attr!();
        svg_content!(title);
    };
}
svg_elem!("mask", Mask, "Mask", mask_items());

// Metadata element
macro_rules! metadata_items {
    ( $el:literal ) => {
        svg_content!(title);
        text_content!();
    };
}
svg_elem!("metadata", Metadata, "Metadata", metadata_items());

// MPath element
macro_rules! mpath_items {
    ( $el:literal ) => {
        svg_attr!(href);
        svg_descriptive!(title);
    };
}
svg_elem!("mpath", MPath, "Motion Path", mpath_items());

impl Path<'_> {
    /// Make a path definition builder
    pub fn def_builder() -> PathDefBuilder {
        PathDefBuilder::new()
    }
}

// Path element
macro_rules! path_items {
    ( $el:literal ) => {
        svg_attr!(d);
        svg_attr!(path_length, "pathLength");
        svg_support_attr!();
        svg_descriptive!(title);
        svg_animation!();
    };
}
svg_elem!("path", Path, "Path", path_items());

// Pattern element
macro_rules! pattern_items {
    ( $el:literal ) => {
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
        svg_support_attr!();
        svg_content!(title);
    };
}
svg_elem!("pattern", Pattern, "Pattern", pattern_items());

impl Polygon<'_> {
    /// Make a polygon point builder
    pub fn point_builder() -> PolyPointBuilder {
        PolyPointBuilder::new()
    }
}

// Polygon element
macro_rules! polygon_items {
    ( $el:literal ) => {
        svg_attr!(points);
        svg_attr!(path_length, "pathLength");
        svg_support_attr!();
        svg_descriptive!(title);
        svg_animation!();
    };
}
svg_elem!("polygon", Polygon, "Polygon", polygon_items());

impl Polyline<'_> {
    /// Make a polyline point builder
    pub fn point_builder() -> PolyPointBuilder {
        PolyPointBuilder::new()
    }
}

// PolyLine element
macro_rules! polyline_items {
    ( $el:literal ) => {
        svg_attr!(points);
        svg_attr!(path_length, "pathLength");
        svg_support_attr!();
        svg_descriptive!(title);
        svg_animation!();
    };
}
svg_elem!("polyline", Polyline, "Polyline", polyline_items());

// RadialGradient element
macro_rules! radial_gradient_items {
    ( $el:literal ) => {
        svg_attr!(href);
        svg_attr!(cx);
        svg_attr!(cy);
        svg_attr!(fr);
        svg_attr!(fx);
        svg_attr!(fy);
        svg_attr!(r);
        svg_attr!(gradient_units, "gradientUnits");
        svg_attr!(gradient_transform, "gradientTransform");
        svg_attr!(spread_method, "spreadMethod");
        svg_descriptive!(title);
        elem_method!(animate, Animate);
        elem_method!(animate_transform, AnimateTransform);
        elem_method!(set, Set);
        elem_method!(stop, Stop);
        elem_method!(script, Script);
        elem_method!(style_el, Style);
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
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(rx);
        svg_attr!(ry);
        svg_attr!(path_length, "pathLength");
        svg_support_attr!();
        svg_descriptive!(title);
        svg_animation!();
    };
}
svg_elem!("rect", Rect, "Rectangle", rect_items());

// Script element
macro_rules! script_items {
    ( $el:literal ) => {
        svg_attr!(href);
        svg_attr!(r#type, "type");
        svg_attr!(crossorigin);
        // NOTE: fetchpriority (future)
        svg_content!(title);
        text_content!();
    };
}
svg_elem!("script", Script, "Script", script_items());

// Set element
macro_rules! set_items {
    ( $el:literal ) => {
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
        // other
        svg_attr!(to);
        svg_attr!(attribute_name, "attributeName");
        // NOTE: attributeType is deprecated
        svg_attr!(key_points, "keyPoints");
        svg_support_attr!();
        svg_descriptive!(title);
    };
}
svg_elem!("set", Set, "Set Value", set_items());

// Stop element
macro_rules! stop_items {
    ( $el:literal ) => {
        svg_attr!(offset);
        elem_method!(animate, Animate);
        elem_method!(set, Set);
        elem_method!(script, Script);
        elem_method!(style_el, Style);
    };
}
svg_elem!("stop", Stop, "Gradient Stop", stop_items());

// Style element
macro_rules! style_items {
    ( $el:literal ) => {
        svg_attr!(r#type, "type");
        svg_attr!(media);
        svg_attr!(title);
        svg_content!(title_el);
        text_content!();
    };
}
svg_elem!("style", Style, "Style Information", style_items());

// Svg element
macro_rules! svg_items {
    ( $el:literal ) => {
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(view_box, "viewBox");
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_attr!(xmlns);
        svg_support_attr!();
        svg_content!(title);
        elem_method!(link, Link);
    };
}
svg_elem!("svg", Svg, "Svg", svg_items());

// Switch element
macro_rules! switch_items {
    ( $el:literal ) => {
        svg_support_attr!();
        svg_content!(title);
    };
}
svg_elem!("switch", Switch, "Switch", switch_items());

// Symbol element
macro_rules! symbol_items {
    ( $el:literal ) => {
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_attr!(ref_x, "refX");
        svg_attr!(ref_y, "refY");
        svg_attr!(view_box, "viewBox");
        svg_content!(title);
    };
}
svg_elem!("symbol", Symbol, "Symbol", symbol_items());

// Text element
macro_rules! text_items {
    ( $el:literal ) => {
        text_content!();
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(dx);
        svg_attr!(dy);
        svg_attr!(rotate);
        svg_attr!(length_adjust, "lengthAdjust");
        svg_attr!(text_length, "textLength");
        svg_attr!(text_anchor, "text-anchor");
        svg_support_attr!();
        elem_method!(tspan, TSpan);
        elem_method!(text_path, TextPath);
        elem_method!(a, A);
        svg_descriptive!(title);
        svg_animation!();
    };
}
svg_elem!("text", Text, "Text", text_items());

// TextPath element
macro_rules! text_path_items {
    ( $el:literal ) => {
        text_content!();
        svg_attr!(href);
        svg_attr!(method);
        svg_attr!(length_adjust, "lengthAdjust");
        svg_attr!(spacing);
        svg_attr!(start_offset, "startOffset");
        svg_attr!(text_length, "textLength");
        svg_attr!(text_anchor, "text-anchor");
        svg_attr!(path); // NOTE: experimental
        svg_attr!(side); // NOTE: experimental
        svg_support_attr!();
        elem_method!(tspan, TSpan);
        elem_method!(a, A);
        svg_descriptive!(title);
        elem_method!(animate, Animate);
        elem_method!(set, Set);
    };
}
svg_elem!("textPath", TextPath, "Text Path", text_path_items());

// Title element
macro_rules! title_items {
    ( $el:literal ) => {
        svg_content!(title);
        text_content!();
    };
}
svg_elem!("title", Title, "Title", title_items());

// TSpan element
macro_rules! tspan_items {
    ( $el:literal ) => {
        text_content!();
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(dx);
        svg_attr!(dy);
        svg_attr!(rotate);
        svg_attr!(length_adjust, "lengthAdjust");
        svg_attr!(text_length, "textLength");
        svg_attr!(text_anchor, "text-anchor");
        svg_support_attr!();
        elem_method!(tspan, TSpan);
        svg_descriptive!(title);
        elem_method!(animate, Animate);
        elem_method!(set, Set);
    };
}
svg_elem!("tspan", TSpan, "Text Span", tspan_items());

// Use element
macro_rules! use_items {
    ( $el:literal ) => {
        svg_attr!(href);
        svg_attr!(x);
        svg_attr!(y);
        svg_attr!(width);
        svg_attr!(height);
        svg_support_attr!();
        svg_descriptive!(title);
        svg_animation!();
    };
}
svg_elem!("use", Use, "Use", use_items());

// View element
macro_rules! view_items {
    ( $el:literal ) => {
        svg_attr!(view_box, "viewBox");
        svg_attr!(preserve_aspect_ratio, "preserveAspectRatio");
        svg_descriptive!(title);
    };
}
svg_elem!("view", View, "View", view_items());

// Extra Presentation attributes (FIXME: Use CSS, or add?):
// - "alignment-baseline" alignment_baseline
// - "baseline-shift" baseline_shift
// - "clip-path" clip_path
// - "clip-rule" clip_rule
// - "color" color
// - "color-interpolation" color_interpolation
// - "color-interpolation-filters" color_interpolation_filters
// - "cursor" cursor
// - "direction" direction
// - "display" display
// - "dominant-baseline" dominant_baseline
// - "fill" fill (not the animation version!)
// - "fill-opacity" fill_opacity
// - "fill-rule" fill_rule
// - "filter" filter
// - "flood-color" flood_color
// - "flood-opacity" flood_opacity
// - "font-family" font_family
// - "font-size" font_size
// - "font-size-adjust" font_size_adjust
// - "font-style" font_style
// - "font-variant" font_variant
// - "font-weight" font_weight
// - "image-rendering" image_rendering
// - "letter-spacing" letter_spacing
// - "lighting-color" lighting_color
// - "marker-end" marker_end
// - "marker-mid" marker_mid
// - "marker-start" marker_start
// - "mask" mask
// - "mask-type" mask_type
// - "opacity" opacity
// - "overflow" overflow
// - "paint-order" paint_order
// - "pointer-events" pointer_events
// - "shape-rendering" shape_rendering
// - "stop-color" stop_color
// - "stop-opacity" stop_opacity
// - "stroke" stroke
// - "stroke-dasharray" stroke_dasharray
// - "stroke-dashoffset" stroke_dashoffset
// - "stroke-linecap" stroke_linecap
// - "stroke-linejoin" stroke_linejoin
// - "stroke-miterlimit" stroke_miterlimit
// - "stroke-opacity" stroke_opacity
// - "stroke-width" stroke_width
// - "text-decoration" text_decoration
// - "text-overflow" text_overflow
// - "text-rendering" text_rendering
// - "transform-origin" transform_origin
// - "unicode-bidi" unicode_bidi
// - "vector-effect" vector_effect
// - "visibility" visibility
// - "white-space" white_space
// - "word-spacing" word_spacing
// - "writing-mode" writing_mode

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
        let mut path = Path::def_builder();
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
