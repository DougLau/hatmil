// css.rs
//
// Copyright (C) 2026  Douglas P Lau
//
//! CSS -- _Cascading Style Sheets_
use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;

/// CSS data value
pub enum Val<'a> {
    /// Borrowed string slice
    Borrowed(&'a str),
    /// Owned string
    Owned(String),
}

// NOTE: corner_shape() is behind `experimental` feature flag
#[allow(rustdoc::broken_intra_doc_links)]
/// Property list
///
/// ```rust
/// # use hatmil::css::Prop;
/// let mut prop = Prop::new();
/// prop.color("white").background_color("#234");
/// assert_eq!(String::from(prop), "color: white; background-color: #234;");
/// ```
///
/// Quotes must enclose `<string>` values:
///
/// ```rust
/// # use hatmil::css::Prop;
/// let mut prop = Prop::new();
/// prop.font_family("\"Liberation\"");
/// assert_eq!(String::from(prop), "font-family: \"Liberation\";");
/// ```
///
/// ## Property Categories
///
/// |                                    |                                |                          |                                   |
/// |------------------------------------|--------------------------------|--------------------------|-----------------------------------|
/// | [basic](Prop::all())               | [column](Prop::columns())      | [font](Prop::font())     | [math](Prop::math_depth())        |
/// | [alignment](Prop::align_content()) | [content area](Prop::height()) | [grid](Prop::grid())     | [padding](Prop::padding())        |
/// | [animation](Prop::animation())     | [containment](Prop::contain()) | [inset](Prop::inset())   | [scroll](Prop::scroll_behavior()) |
/// | [background](Prop::background())   | [corner](Prop::corner_shape()) | [margin](Prop::margin()) | [svg](Prop::clip_rule())          |
/// | [border](Prop::border())           | [flex](Prop::flex())           | [mask](Prop::mask())     | [text](Prop::text_autospace())    |
#[derive(Debug, Clone)]
pub struct Prop {
    /// Property separator
    sep: &'static str,
    /// Encoded values
    val: String,
}

/// CSS selector
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selector {
    // FIXME
}

/// Rule containing a selector and property list
#[derive(Debug, Clone)]
pub struct Rule {
    selector: Selector,
    prop: Prop,
}

impl Val<'_> {
    /// Get character iterator
    pub(crate) fn chars(&'_ self) -> impl Iterator<Item = char> {
        match &self {
            Val::Borrowed(s) => s.chars(),
            Val::Owned(s) => s.chars(),
        }
    }

    /// Return `<string>` value
    fn string(&self) -> Option<&str> {
        match &self {
            Val::Borrowed(s) => {
                if let Some(("", s)) = s.split_once('"')
                    && let Some((s, "")) = s.rsplit_once('"')
                {
                    return Some(s);
                }
            }
            Val::Owned(s) => {
                if let Some(("", s)) = s.split_once('"')
                    && let Some((s, "")) = s.rsplit_once('"')
                {
                    return Some(s);
                }
            }
        }
        None
    }

    /// Format a `<string>` value
    fn fmt_string(&self, f: &mut fmt::Formatter, s: &str) -> fmt::Result {
        write!(f, "\"")?;
        for c in s.chars() {
            match c {
                // NULL => REPLACEMENT CHARACTER
                '\0' => write!(f, "\u{FFFD}")?,
                '\u{0001}'..='\u{001f}' | '\u{007f}' => {
                    if let Ok(v) = u16::try_from(c) {
                        write!(f, "\\{v:X} ")?;
                    }
                }
                '"' => write!(f, r#"\""#)?,
                '\\' => write!(f, r"\\")?,
                _ => write!(f, "{c}")?,
            }
        }
        write!(f, "\"")
    }

    /// Format any non-`<string>` and non-`<ident>` value
    fn fmt_other(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in self.chars() {
            match c {
                // NULL => REPLACEMENT CHARACTER
                '\0' => write!(f, "\u{FFFD}")?,
                '\u{0001}'..='\u{001f}' | '\u{007f}' => {
                    if let Ok(v) = u16::try_from(c) {
                        write!(f, "\\{v:X} ")?;
                    }
                }
                '\\' => write!(f, r"\\")?,
                _ => write!(f, "{c}")?,
            }
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Val<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.string() {
            Some(s) => self.fmt_string(f, s),
            None => self.fmt_other(f),
        }
    }
}

impl<'c> From<&'c str> for Val<'c> {
    fn from(v: &'c str) -> Self {
        Val::Borrowed(v)
    }
}

impl From<String> for Val<'_> {
    fn from(v: String) -> Self {
        Val::Owned(v)
    }
}

impl<'c> From<&'c String> for Val<'c> {
    fn from(v: &'c String) -> Self {
        Val::Borrowed(v)
    }
}

impl<'c> From<Cow<'c, str>> for Val<'c> {
    fn from(v: Cow<'c, str>) -> Self {
        match v {
            Cow::Borrowed(v) => Val::Borrowed(v),
            Cow::Owned(v) => Val::Owned(v),
        }
    }
}

impl From<char> for Val<'_> {
    fn from(v: char) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<bool> for Val<'_> {
    fn from(v: bool) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<i8> for Val<'_> {
    fn from(v: i8) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<u8> for Val<'_> {
    fn from(v: u8) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<i16> for Val<'_> {
    fn from(v: i16) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<u16> for Val<'_> {
    fn from(v: u16) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<i32> for Val<'_> {
    fn from(v: i32) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<u32> for Val<'_> {
    fn from(v: u32) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<i64> for Val<'_> {
    fn from(v: i64) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<u64> for Val<'_> {
    fn from(v: u64) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<i128> for Val<'_> {
    fn from(v: i128) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<u128> for Val<'_> {
    fn from(v: u128) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<isize> for Val<'_> {
    fn from(v: isize) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<usize> for Val<'_> {
    fn from(v: usize) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<f32> for Val<'_> {
    fn from(v: f32) -> Self {
        Val::Owned(v.to_string())
    }
}

impl From<f64> for Val<'_> {
    fn from(v: f64) -> Self {
        Val::Owned(v.to_string())
    }
}

impl fmt::Display for Prop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.val.is_empty() {
            Ok(())
        } else {
            write!(f, "{};", self.val)
        }
    }
}

impl From<Prop> for String {
    fn from(mut prop: Prop) -> Self {
        // zero-copy alternative to fmt::Display
        if !prop.val.is_empty() {
            prop.val.push(';');
        }
        prop.val
    }
}

impl Default for Prop {
    fn default() -> Self {
        Self::new()
    }
}

impl Prop {
    /// Make a new property list
    pub fn new() -> Self {
        Prop {
            sep: " ",
            val: String::with_capacity(16),
        }
    }

    /// Push property separator
    fn push_sep(&mut self) {
        if !self.val.is_empty() {
            self.val.push(';');
            self.val.push_str(self.sep);
        }
    }

    /// Add a [custom] property (variable)
    ///
    /// ```rust
    /// # use hatmil::css::Prop;
    /// let mut prop = Prop::new();
    /// prop.custom("variable", "value");
    /// assert_eq!(String::from(prop), "--variable: value;");
    /// ```
    ///
    /// [custom]: https://developer.mozilla.org/en-US/docs/Web/CSS/Guides/Cascading_variables/Using_custom_properties
    pub fn custom<'a, V>(&mut self, name: &str, v: V) -> &mut Self
    where
        V: Into<Val<'a>>,
    {
        self.push_sep();
        self.val.push_str("--");
        // TODO: check name validity
        self.val.push_str(name);
        self.val.push_str(": ");
        let _ = write!(self.val, "{}", v.into());
        self
    }

    /// Add [!important] flag to the previous property
    ///
    /// ```rust
    /// # use hatmil::css::Prop;
    /// let mut prop = Prop::new();
    /// prop.color("rebeccapurple").important();
    /// assert_eq!(String::from(prop), "color: rebeccapurple !important;");
    /// ```
    ///
    /// [!important]: https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Values/important
    pub fn important(&mut self) -> &mut Self {
        if !self.val.is_empty() {
            self.val.push_str(" !important");
        }
        self
    }
}

/// **Basic Properties**
///
/// ---
impl Prop {
    css_prop!(all, "all");
    #[cfg(feature = "limited-availability")]
    css_prop!(accent_color, "accent-color");
    css_prop!(anchor_name, "anchor-name");
    css_prop!(anchor_scope, "anchor-scope");
    css_prop!(appearance, "appearance");
    css_prop!(aspect_ratio, "aspect-ratio");
    css_prop!(backdrop_filter, "backdrop-filter");
    css_prop!(backface_visibility, "backface-visibility");
    css_prop!(block_size, "block-size");
    #[cfg(feature = "limited-availability")]
    css_prop!(box_decoration_break, "box-decoration-break");
    css_prop!(box_shadow, "box-shadow");
    css_prop!(box_sizing, "box-sizing");
    css_prop!(break_after, "break-after");
    css_prop!(break_before, "break-before");
    css_prop!(break_inside, "break-inside");
    css_prop!(caption_side, "caption-side");
    #[cfg(feature = "experimental")]
    css_prop!(caret, "caret");
    #[cfg(feature = "experimental")]
    css_prop!(caret_animation, "caret-animation");
    css_prop!(caret_color, "caret-color");
    #[cfg(feature = "experimental")]
    css_prop!(caret_shape, "caret-shape");
    css_prop!(clear, "clear");
    css_prop!(clip_path, "clip-path");
    css_prop!(color, "color");
    css_prop!(color_scheme, "color-scheme");
    css_prop!(content, "content");
    css_prop!(counter_increment, "counter-increment");
    css_prop!(counter_reset, "counter-reset");
    css_prop!(counter_set, "counter-set");
    css_prop!(cursor, "cursor");
    css_prop!(direction, "direction");
    css_prop!(display, "display");
    #[cfg(feature = "limited-availability")]
    css_prop!(dynamic_range_limit, "dynamic-range-limit");
    css_prop!(empty_cells, "empty-cells");
    #[cfg(feature = "limited-availability")]
    css_prop!(field_sizing, "field-sizing");
    css_prop!(filter, "filter");
    css_prop!(float, "float");
    #[cfg(feature = "limited-availability")]
    css_prop!(forced_color_adjust, "forced-color-adjust");
    css_prop!(gap, "gap");
    #[cfg(feature = "limited-availability")]
    css_prop!(hanging_punctuation, "hanging-punctuation");
    css_prop!(hyphenate_character, "hyphenate-character");
    #[cfg(feature = "limited-availability")]
    css_prop!(hyphenate_limit_chars, "hyphenate-limit-chars");
    css_prop!(hyphens, "hyphens");
    css_prop!(image_orientation, "image-orientation");
    css_prop!(image_rendering, "image-rendering");
    #[cfg(feature = "experimental")]
    css_prop!(image_resolution, "image-resolution");
    #[cfg(feature = "limited-availability")]
    css_prop!(initial_letter, "initial-letter");
    css_prop!(inline_size, "inline-size");
    #[cfg(feature = "experimental")]
    css_prop!(interactivity, "interactivity");
    #[cfg(feature = "experimental")]
    css_prop!(interest_delay, "interest-delay");
    #[cfg(feature = "experimental")]
    css_prop!(interest_delay_start, "interest-delay-start");
    #[cfg(feature = "experimental")]
    css_prop!(interest_delay_end, "interest-delay-end");
    #[cfg(feature = "experimental")]
    css_prop!(interpolate_size, "interpolate-size");
    css_prop!(isolation, "isolation");
    css_prop!(letter_spacing, "letter-spacing");
    css_prop!(line_break, "line-break");
    #[cfg(feature = "limited-availability")]
    css_prop!(line_clamp, "line-clamp");
    css_prop!(line_height, "line-height");
    #[cfg(feature = "experimental")]
    css_prop!(line_height_step, "line-height-step");
    css_prop!(list_style, "list-style");
    css_prop!(list_style_image, "list-style-image");
    css_prop!(list_style_position, "list-style-position");
    css_prop!(list_style_type, "list-style-type");
    css_prop!(max_block_size, "max-block-size");
    css_prop!(max_inline_size, "max-inline-size");
    css_prop!(min_block_size, "min-block-size");
    css_prop!(min_inline_size, "min-inline-size");
    css_prop!(mix_blend_mode, "mix-blend-mode");
    css_prop!(object_fit, "object-fit");
    css_prop!(object_position, "object-position");
    #[cfg(feature = "experimental")]
    css_prop!(object_view_box, "object-view-box");
    css_prop!(opacity, "opacity");
    css_prop!(order, "order");
    #[cfg(feature = "limited-availability")]
    css_prop!(orphans, "orphans");
    css_prop!(overflow, "overflow");
    #[cfg(feature = "limited-availability")]
    css_prop!(overflow_anchor, "overflow-anchor");
    css_prop!(overflow_block, "overflow-block");
    #[cfg(feature = "limited-availability")]
    css_prop!(overflow_clip_margin, "overflow-clip-margin");
    css_prop!(overflow_inline, "overflow-inline");
    css_prop!(overflow_wrap, "overflow-wrap");
    css_prop!(overflow_x, "overflow-x");
    css_prop!(overflow_y, "overflow-y");
    #[cfg(feature = "experimental")]
    css_prop!(overlay, "overlay");
    css_prop!(page, "page");
    css_prop!(paint_order, "paint-order");
    css_prop!(perspective, "perspective");
    css_prop!(perspective_origin, "perspective-origin");
    css_prop!(pointer_events, "pointer-events");
    css_prop!(position, "position");
    #[cfg(feature = "limited-availability")]
    css_prop!(position_anchor, "position-anchor");
    css_prop!(position_area, "position-area");
    css_prop!(position_try, "position-try");
    css_prop!(position_try_fallbacks, "position-try-fallbacks");
    css_prop!(position_try_order, "position-try-order");
    css_prop!(position_visibility, "position-visibility");
    css_prop!(print_color_adjust, "print-color-adjust");
    css_prop!(quotes, "quotes");
    #[cfg(feature = "experimental")]
    css_prop!(reading_flow, "reading-flow");
    #[cfg(feature = "experimental")]
    css_prop!(reading_order, "reading-order");
    #[cfg(feature = "limited-availability")]
    css_prop!(resize, "resize");
    css_prop!(rotate, "rotate");
    css_prop!(row_gap, "row-gap");
    css_prop!(ruby_align, "ruby-align");
    #[cfg(feature = "limited-availability")]
    css_prop!(ruby_overhang, "ruby-overhang");
    css_prop!(ruby_position, "ruby-position");
    css_prop!(scale, "scale");
    css_prop!(scrollbar_color, "scrollbar-color");
    css_prop!(scrollbar_gutter, "scrollbar-gutter");
    css_prop!(scrollbar_width, "scrollbar-width");
    css_prop!(shape_image_threshold, "shape-image-threshold");
    css_prop!(shape_margin, "shape-margin");
    css_prop!(shape_outside, "shape-outside");
    #[cfg(feature = "experimental")]
    css_prop!(speak_as, "speak-as");
    css_prop!(tab_size, "tab-size");
    css_prop!(table_layout, "table-layout");
    css_prop!(touch_action, "touch-action");
    css_prop!(transform, "transform");
    css_prop!(transform_box, "transform-box");
    css_prop!(transform_origin, "transform-origin");
    css_prop!(transform_style, "transform-style");
    css_prop!(translate, "translate");
    css_prop!(unicode_bidi, "unicode-bidi");
    #[cfg(feature = "limited-availability")]
    css_prop!(user_select, "user-select");
    css_prop!(visibility, "visibility");
    css_prop!(white_space, "white-space");
    css_prop!(white_space_collapse, "white-space-collapse");
    #[cfg(feature = "limited-availability")]
    css_prop!(widows, "widows");
    css_prop!(will_change, "will-change");
    css_prop!(word_break, "word-break");
    css_prop!(word_spacing, "word-spacing");
    css_prop!(writing_mode, "writing-mode");
    css_prop!(z_index, "z-index");
    css_prop!(zoom, "zoom");
}

/// **Alignment and Justification**
///
/// ---
impl Prop {
    css_prop!(align_content, "align-content");
    css_prop!(align_items, "align-items");
    css_prop!(align_self, "align-self");
    css_prop!(justify_content, "justify-content");
    css_prop!(justify_items, "justify-items");
    css_prop!(justify_self, "justify-self");
    css_prop!(place_content, "place-content");
    css_prop!(place_items, "place-items");
    css_prop!(place_self, "place-self");
    css_prop!(text_align, "text-align");
    css_prop!(text_align_last, "text-align-last");
    #[cfg(feature = "limited-availability")]
    css_prop!(text_justify, "text-justify");
    css_prop!(vertical_align, "vertical-align");
    css_prop!(alignment_baseline, "alignment-baseline");
    css_prop!(baseline_shift, "baseline-shift");
    #[cfg(feature = "limited-availability")]
    css_prop!(baseline_source, "baseline-source");
    css_prop!(dominant_baseline, "dominant-baseline");
}

/// **Animation**
///
/// ---
impl Prop {
    css_prop!(animation, "animation");
    css_prop!(animation_composition, "animation-composition");
    css_prop!(animation_delay, "animation-delay");
    css_prop!(animation_direction, "animation-direction");
    css_prop!(animation_duration, "animation-duration");
    css_prop!(animation_fill_mode, "animation-fill-mode");
    css_prop!(animation_iteration_count, "animation-iteration-count");
    css_prop!(animation_name, "animation-name");
    css_prop!(animation_play_state, "animation-play-state");
    #[cfg(feature = "limited-availability")]
    css_prop!(animation_range_start, "animation-range-start");
    #[cfg(feature = "limited-availability")]
    css_prop!(animation_range_end, "animation-range-end");
    #[cfg(feature = "limited-availability")]
    css_prop!(animation_range, "animation-range");
    #[cfg(feature = "limited-availability")]
    css_prop!(animation_timeline, "animation-timeline");
    css_prop!(animation_timing_function, "animation-timing-function");
    css_prop!(offset, "offset");
    css_prop!(offset_anchor, "offset-anchor");
    css_prop!(offset_distance, "offset-distance");
    css_prop!(offset_path, "offset-path");
    css_prop!(offset_position, "offset-position");
    css_prop!(offset_rotate, "offset-rotate");
    #[cfg(feature = "limited-availability")]
    css_prop!(timeline_scope, "timeline-scope");
    css_prop!(transition, "transition");
    css_prop!(transition_behavior, "transition-behavior");
    css_prop!(transition_delay, "transition-delay");
    css_prop!(transition_duration, "transition-duration");
    css_prop!(transition_property, "transition-property");
    css_prop!(transition_timing_function, "transition-timing-function");
    #[cfg(feature = "limited-availability")]
    css_prop!(view_timeline, "view-timeline");
    #[cfg(feature = "limited-availability")]
    css_prop!(view_timeline_axis, "view-timeline-axis");
    #[cfg(feature = "limited-availability")]
    css_prop!(view_timeline_inset, "view-timeline-inset");
    #[cfg(feature = "limited-availability")]
    css_prop!(view_timeline_name, "view-timeline-name");
    css_prop!(view_transition_class, "view-transition-class");
    css_prop!(view_transition_name, "view-transition-name");
}

/// **Background**
///
/// ---
impl Prop {
    css_prop!(background, "background");
    css_prop!(background_attachment, "background-attachment");
    css_prop!(background_blend_mode, "background-blend-mode");
    css_prop!(background_clip, "background-clip");
    css_prop!(background_color, "background-color");
    css_prop!(background_image, "background-image");
    css_prop!(background_origin, "background-origin");
    css_prop!(background_position, "background-position");
    css_prop!(background_position_x, "background-position-x");
    css_prop!(background_position_y, "background-position-y");
    css_prop!(background_repeat, "background-repeat");
    #[cfg(feature = "experimental")]
    css_prop!(background_repeat_x, "background-repeat-x");
    #[cfg(feature = "experimental")]
    css_prop!(background_repeat_y, "background-repeat-y");
    css_prop!(background_size, "background-size");
}

/// **Border and Outline**
///
/// ---
impl Prop {
    css_prop!(border, "border");
    css_prop!(border_collapse, "border-collapse");
    css_prop!(border_color, "border-color");
    css_prop!(border_style, "border-style");
    css_prop!(border_spacing, "border-spacing");
    css_prop!(border_width, "border-width");
    css_prop!(border_bottom, "border-bottom");
    css_prop!(border_bottom_color, "border-bottom-color");
    css_prop!(border_bottom_style, "border-bottom-style");
    css_prop!(border_bottom_width, "border-bottom-width");
    css_prop!(border_left, "border-left");
    css_prop!(border_left_color, "border-left-color");
    css_prop!(border_left_style, "border-left-style");
    css_prop!(border_left_width, "border-left-width");
    css_prop!(border_right, "border-right");
    css_prop!(border_right_color, "border-right-color");
    css_prop!(border_right_style, "border-right-style");
    css_prop!(border_right_width, "border-right-width");
    css_prop!(border_top, "border-top");
    css_prop!(border_top_color, "border-top-color");
    css_prop!(border_top_style, "border-top-style");
    css_prop!(border_top_width, "border-top-width");
    css_prop!(border_radius, "border-radius");
    css_prop!(border_bottom_left_radius, "border-bottom-left-radius");
    css_prop!(border_bottom_right_radius, "border-bottom-right-radius");
    css_prop!(border_top_left_radius, "border-top-left-radius");
    css_prop!(border_top_right_radius, "border-top-right-radius");
    css_prop!(border_start_start_radius, "border-start-start-radius");
    css_prop!(border_start_end_radius, "border-start-end-radius");
    css_prop!(border_end_start_radius, "border-end-start-radius");
    css_prop!(border_end_end_radius, "border-end-end-radius");
    css_prop!(border_block, "border-block");
    css_prop!(border_block_color, "border-block-color");
    css_prop!(border_block_style, "border-block-style");
    css_prop!(border_block_width, "border-block-width");
    css_prop!(border_block_start, "border-block-start");
    css_prop!(border_block_start_color, "border-block-start-color");
    css_prop!(border_block_start_style, "border-block-start-style");
    css_prop!(border_block_start_width, "border-block-start-width");
    css_prop!(border_block_end, "border-block-end");
    css_prop!(border_block_end_color, "border-block-end-color");
    css_prop!(border_block_end_style, "border-block-end-style");
    css_prop!(border_block_end_width, "border-block-end-width");
    css_prop!(border_image, "border-image");
    css_prop!(border_image_outset, "border-image-outset");
    css_prop!(border_image_repeat, "border-image-repeat");
    css_prop!(border_image_slice, "border-image-slice");
    css_prop!(border_image_source, "border-image-source");
    css_prop!(border_image_width, "border-image-width");
    css_prop!(border_inline, "border-inline");
    css_prop!(border_inline_color, "border-inline-color");
    css_prop!(border_inline_style, "border-inline-style");
    css_prop!(border_inline_width, "border-inline-width");
    css_prop!(border_inline_start, "border-inline-start");
    css_prop!(border_inline_start_color, "border-inline-start-color");
    css_prop!(border_inline_start_style, "border-inline-start-style");
    css_prop!(border_inline_start_width, "border-inline-start-width");
    css_prop!(border_inline_end, "border-inline-end");
    css_prop!(border_inline_end_color, "border-inline-end-color");
    css_prop!(border_inline_end_style, "border-inline-end-style");
    css_prop!(border_inline_end_width, "border-inline-end-width");
    css_prop!(outline, "outline");
    css_prop!(outline_color, "outline-color");
    css_prop!(outline_offset, "outline-offset");
    css_prop!(outline_style, "outline-style");
    css_prop!(outline_width, "outline-width");
}

/// **Column**
///
/// ---
impl Prop {
    css_prop!(columns, "columns");
    css_prop!(column_count, "column-count");
    css_prop!(column_fill, "column-fill");
    css_prop!(column_gap, "column-gap");
    #[cfg(feature = "experimental")]
    css_prop!(column_height, "column-height");
    css_prop!(column_rule, "column-rule");
    css_prop!(column_rule_color, "column-rule-color");
    css_prop!(column_rule_style, "column-rule-style");
    css_prop!(column_rule_width, "column-rule-width");
    css_prop!(column_span, "column-span");
    css_prop!(column_width, "column-width");
    #[cfg(feature = "experimental")]
    css_prop!(column_wrap, "column-wrap");
}

/// **Content Area**
///
/// ---
impl Prop {
    css_prop!(height, "height");
    css_prop!(min_height, "min-height");
    css_prop!(max_height, "max-height");
    css_prop!(width, "width");
    css_prop!(max_width, "max-width");
    css_prop!(min_width, "min-width");
}

/// **Containment**
///
/// ---
impl Prop {
    css_prop!(contain, "contain");
    css_prop!(contain_intrinsic_block_size, "contain-intrinsic-block-size");
    css_prop!(contain_intrinsic_height, "contain-intrinsic-height");
    css_prop!(
        contain_intrinsic_inline_size,
        "contain-intrinsic-inline-size"
    );
    css_prop!(contain_intrinsic_size, "contain-intrinsic-size");
    css_prop!(contain_intrinsic_width, "contain-intrinsic-width");
    css_prop!(container, "container");
    css_prop!(container_name, "container-name");
    css_prop!(container_type, "container-type");
    css_prop!(content_visibility, "content-visibility");
}

#[cfg(feature = "experimental")]
/// **Corner**
///
/// ---
impl Prop {
    css_prop!(corner_shape, "corner-shape");
    css_prop!(corner_bottom_shape, "corner-bottom-shape");
    css_prop!(corner_left_shape, "corner-left-shape");
    css_prop!(corner_right_shape, "corner-right-shape");
    css_prop!(corner_top_shape, "corner-top-shape");
    css_prop!(corner_bottom_left_shape, "corner-bottom-left-shape");
    css_prop!(corner_bottom_right_shape, "corner-bottom-right-shape");
    css_prop!(corner_top_left_shape, "corner-top-left-shape");
    css_prop!(corner_top_right_shape, "corner-top-right-shape");
    css_prop!(corner_block_start_shape, "corner-block-start-shape");
    css_prop!(corner_block_end_shape, "corner-block-end-shape");
    css_prop!(corner_inline_end_shape, "corner-inline-end-shape");
    css_prop!(corner_inline_start_shape, "corner-inline-start-shape");
    css_prop!(corner_start_start_shape, "corner-start-start-shape");
    css_prop!(corner_start_end_shape, "corner-start-end-shape");
    css_prop!(corner_end_start_shape, "corner-end-start-shape");
    css_prop!(corner_end_end_shape, "corner-end-end-shape");
}

/// **Flex**
///
/// ---
impl Prop {
    css_prop!(flex, "flex");
    css_prop!(flex_basis, "flex-basis");
    css_prop!(flex_direction, "flex-direction");
    css_prop!(flex_flow, "flex-flow");
    css_prop!(flex_grow, "flex-grow");
    css_prop!(flex_shrink, "flex-shrink");
    css_prop!(flex_wrap, "flex-wrap");
}

/// **Font**
///
/// ---
impl Prop {
    css_prop!(font, "font");
    css_prop!(font_family, "font-family");
    css_prop!(font_feature_settings, "font-feature-settings");
    css_prop!(font_kerning, "font-kerning");
    #[cfg(feature = "limited-availability")]
    css_prop!(font_language_override, "font-language-override");
    css_prop!(font_optical_sizing, "font-optical-sizing");
    css_prop!(font_palette, "font-palette");
    css_prop!(font_size, "font-size");
    css_prop!(font_size_adjust, "font-size-adjust");
    css_prop!(font_stretch, "font-stretch");
    css_prop!(font_style, "font-style");
    css_prop!(font_synthesis, "font-synthesis");
    #[cfg(feature = "experimental")]
    css_prop!(font_synthesis_position, "font-synthesis-position");
    css_prop!(font_synthesis_small_caps, "font-synthesis-small-caps");
    css_prop!(font_synthesis_style, "font-synthesis-style");
    css_prop!(font_synthesis_weight, "font-synthesis-weight");
    css_prop!(font_variant, "font-variant");
    css_prop!(font_variant_alternates, "font-variant-alternates");
    css_prop!(font_variant_caps, "font-variant-caps");
    css_prop!(font_variant_east_asian, "font-variant-east-asian");
    #[cfg(feature = "limited-availability")]
    css_prop!(font_variant_emoji, "font-variant-emoji");
    css_prop!(font_variant_ligatures, "font-variant-ligatures");
    css_prop!(font_variant_numeric, "font-variant-numeric");
    css_prop!(font_variant_position, "font-variant-position");
    css_prop!(font_variation_settings, "font-variation-settings");
    css_prop!(font_weight, "font-weight");
    #[cfg(feature = "experimental")]
    css_prop!(font_width, "font-width");
}

/// **Grid**
///
/// ---
impl Prop {
    css_prop!(grid, "grid");
    css_prop!(grid_area, "grid-area");
    css_prop!(grid_auto_columns, "grid-auto-columns");
    css_prop!(grid_auto_flow, "grid-auto-flow");
    css_prop!(grid_auto_rows, "grid-auto-rows");
    css_prop!(grid_column, "grid-column");
    css_prop!(grid_column_start, "grid-column-start");
    css_prop!(grid_column_end, "grid-column-end");
    css_prop!(grid_row, "grid-row");
    css_prop!(grid_row_start, "grid-row-start");
    css_prop!(grid_row_end, "grid-row-end");
    css_prop!(grid_template, "grid-template");
    css_prop!(grid_template_areas, "grid-template-areas");
    css_prop!(grid_template_columns, "grid-template-columns");
    css_prop!(grid_template_rows, "grid-template-rows");
}

/// **Inset**
///
/// ---
impl Prop {
    css_prop!(inset, "inset");
    css_prop!(bottom, "bottom");
    css_prop!(left, "left");
    css_prop!(right, "right");
    css_prop!(top, "top");
    css_prop!(inset_block, "inset-block");
    css_prop!(inset_block_start, "inset-block-start");
    css_prop!(inset_block_end, "inset-block-end");
    css_prop!(inset_inline, "inset-inline");
    css_prop!(inset_inline_start, "inset-inline-start");
    css_prop!(inset_inline_end, "inset-inline-end");
}

/// **Margin**
///
/// ---
impl Prop {
    css_prop!(margin, "margin");
    css_prop!(margin_bottom, "margin-bottom");
    css_prop!(margin_left, "margin-left");
    css_prop!(margin_right, "margin-right");
    css_prop!(margin_top, "margin-top");
    css_prop!(margin_block, "margin-block");
    css_prop!(margin_block_start, "margin-block-start");
    css_prop!(margin_block_end, "margin-block-end");
    css_prop!(margin_inline, "margin-inline");
    css_prop!(margin_inline_start, "margin-inline-start");
    css_prop!(margin_inline_end, "margin-inline-end");
    #[cfg(feature = "experimental")]
    css_prop!(margin_trim, "margin-trim");
}

/// **Mask**
///
/// ---
impl Prop {
    css_prop!(mask, "mask");
    #[cfg(feature = "limited-availability")]
    css_prop!(mask_border, "mask-border");
    #[cfg(feature = "limited-availability")]
    css_prop!(mask_border_mode, "mask-border-mode");
    #[cfg(feature = "limited-availability")]
    css_prop!(mask_border_outset, "mask-border-outset");
    #[cfg(feature = "limited-availability")]
    css_prop!(mask_border_repeat, "mask-border-repeat");
    #[cfg(feature = "limited-availability")]
    css_prop!(mask_border_slice, "mask-border-slice");
    #[cfg(feature = "limited-availability")]
    css_prop!(mask_border_source, "mask-border-source");
    #[cfg(feature = "limited-availability")]
    css_prop!(mask_border_width, "mask-border-width");
    css_prop!(mask_clip, "mask-clip");
    css_prop!(mask_composite, "mask-composite");
    css_prop!(mask_image, "mask-image");
    css_prop!(mask_mode, "mask-mode");
    css_prop!(mask_origin, "mask-origin");
    css_prop!(mask_position, "mask-position");
    css_prop!(mask_repeat, "mask-repeat");
    css_prop!(mask_size, "mask-size");
}

/// **MathML**
///
/// ---
impl Prop {
    css_prop!(math_depth, "math-depth");
    css_prop!(math_shift, "math-shift");
    css_prop!(math_style, "math-style");
}

/// **Padding**
///
/// ---
impl Prop {
    css_prop!(padding, "padding");
    css_prop!(padding_bottom, "padding-bottom");
    css_prop!(padding_left, "padding-left");
    css_prop!(padding_right, "padding-right");
    css_prop!(padding_top, "padding-top");
    css_prop!(padding_block, "padding-block");
    css_prop!(padding_block_start, "padding-block-start");
    css_prop!(padding_block_end, "padding-block-end");
    css_prop!(padding_inline, "padding-inline");
    css_prop!(padding_inline_start, "padding-inline-start");
    css_prop!(padding_inline_end, "padding-inline-end");
}

/// **Scrolling Area**
///
/// ---
impl Prop {
    css_prop!(scroll_behavior, "scroll-behavior");
    #[cfg(feature = "experimental")]
    css_prop!(scroll_initial_target, "scroll-initial-target");
    #[cfg(feature = "experimental")]
    css_prop!(scroll_marker_group, "scroll-marker-group");
    css_prop!(scroll_margin, "scroll-margin");
    css_prop!(scroll_margin_bottom, "scroll-margin-bottom");
    css_prop!(scroll_margin_left, "scroll-margin-left");
    css_prop!(scroll_margin_right, "scroll-margin-right");
    css_prop!(scroll_margin_top, "scroll-margin-top");
    css_prop!(scroll_margin_block, "scroll-margin-block");
    css_prop!(scroll_margin_block_start, "scroll-margin-block-start");
    css_prop!(scroll_margin_block_end, "scroll-margin-block-end");
    css_prop!(scroll_margin_inline, "scroll-margin-inline");
    css_prop!(scroll_margin_inline_start, "scroll-margin-inline-start");
    css_prop!(scroll_margin_inline_end, "scroll-margin-inline-end");
    css_prop!(scroll_padding, "scroll-padding");
    css_prop!(scroll_padding_bottom, "scroll-padding-bottom");
    css_prop!(scroll_padding_left, "scroll-padding-left");
    css_prop!(scroll_padding_right, "scroll-padding-right");
    css_prop!(scroll_padding_top, "scroll-padding-top");
    css_prop!(scroll_padding_block, "scroll-padding-block");
    css_prop!(scroll_padding_block_start, "scroll-padding-block-start");
    css_prop!(scroll_padding_block_end, "scroll-padding-block-end");
    css_prop!(scroll_padding_inline, "scroll-padding-inline");
    css_prop!(scroll_padding_inline_start, "scroll-padding-inline-start");
    css_prop!(scroll_padding_inline_end, "scroll-padding-inline-end");
    css_prop!(scroll_snap_align, "scroll-snap-align");
    css_prop!(scroll_snap_stop, "scroll-snap-stop");
    css_prop!(scroll_snap_type, "scroll-snap-type");
    #[cfg(feature = "experimental")]
    css_prop!(scroll_target_group, "scroll-target-group");
    #[cfg(feature = "limited-availability")]
    css_prop!(scroll_timeline, "scroll-timeline");
    #[cfg(feature = "limited-availability")]
    css_prop!(scroll_timeline_axis, "scroll-timeline-axis");
    #[cfg(feature = "limited-availability")]
    css_prop!(scroll_timeline_name, "scroll-timeline-name");
    #[cfg(feature = "limited-availability")]
    css_prop!(overscroll_behavior, "overscroll-behavior");
    #[cfg(feature = "limited-availability")]
    css_prop!(overscroll_behavior_block, "overscroll-behavior-block");
    #[cfg(feature = "limited-availability")]
    css_prop!(overscroll_behavior_inline, "overscroll-behavior-inline");
    #[cfg(feature = "limited-availability")]
    css_prop!(overscroll_behavior_x, "overscroll-behavior-x");
    #[cfg(feature = "limited-availability")]
    css_prop!(overscroll_behavior_y, "overscroll-behavior-y");
}

/// **SVG**
///
/// ---
impl Prop {
    css_prop!(clip_rule, "clip-rule");
    css_prop!(color_interpolation, "color-interpolation");
    css_prop!(color_interpolation_filters, "color-interpolation-filters");
    css_prop!(cx, "cx");
    css_prop!(cy, "cy");
    #[cfg(feature = "limited-availability")]
    css_prop!(d, "d");
    css_prop!(fill, "fill");
    css_prop!(fill_opacity, "fill-opacity");
    css_prop!(fill_rule, "fill-rule");
    css_prop!(flood_color, "flood-color");
    css_prop!(flood_opacity, "flood-opacity");
    css_prop!(lighting_color, "lighting-color");
    css_prop!(marker, "marker");
    css_prop!(marker_start, "marker-start");
    css_prop!(marker_mid, "marker-mid");
    css_prop!(marker_end, "marker-end");
    css_prop!(mask_type, "mask-type");
    css_prop!(r, "r");
    css_prop!(rx, "rx");
    css_prop!(ry, "ry");
    css_prop!(shape_rendering, "shape-rendering");
    css_prop!(stop_color, "stop-color");
    css_prop!(stop_opacity, "stop-opacity");
    css_prop!(stroke, "stroke");
    css_prop!(stroke_dasharray, "stroke-dasharray");
    css_prop!(stroke_dashoffset, "stroke-dashoffset");
    css_prop!(stroke_linecap, "stroke-linecap");
    css_prop!(stroke_linejoin, "stroke-linejoin");
    css_prop!(stroke_miterlimit, "stroke-miterlimit");
    css_prop!(stroke_opacity, "stroke-opacity");
    css_prop!(stroke_width, "stroke-width");
    css_prop!(text_anchor, "text-anchor");
    css_prop!(vector_effect, "vector-effect");
    css_prop!(x, "x");
    css_prop!(y, "y");
}

/// **Text**
///
/// ---
impl Prop {
    css_prop!(text_autospace, "text-autospace");
    #[cfg(feature = "limited-availability")]
    css_prop!(text_box, "text-box");
    #[cfg(feature = "limited-availability")]
    css_prop!(text_box_edge, "text-box-edge");
    #[cfg(feature = "limited-availability")]
    css_prop!(text_box_trim, "text-box-trim");
    css_prop!(text_combine_upright, "text-combine-upright");
    css_prop!(text_decoration, "text-decoration");
    css_prop!(text_decoration_color, "text-decoration-color");
    #[cfg(feature = "experimental")]
    css_prop!(text_decoration_inset, "text-decoration-inset");
    css_prop!(text_decoration_line, "text-decoration-line");
    css_prop!(text_decoration_skip_ink, "text-decoration-skip-ink");
    css_prop!(text_decoration_style, "text-decoration-style");
    css_prop!(text_decoration_thickness, "text-decoration-thickness");
    css_prop!(text_emphasis, "text-emphasis");
    css_prop!(text_emphasis_color, "text-emphasis-color");
    css_prop!(text_emphasis_position, "text-emphasis-position");
    css_prop!(text_emphasis_style, "text-emphasis-style");
    css_prop!(text_indent, "text-indent");
    css_prop!(text_orientation, "text-orientation");
    css_prop!(text_overflow, "text-overflow");
    css_prop!(text_rendering, "text-rendering");
    css_prop!(text_shadow, "text-shadow");
    #[cfg(feature = "experimental")]
    css_prop!(text_size_adjust, "text-size-adjust");
    #[cfg(feature = "experimental")]
    css_prop!(text_spacing_trim, "text-spacing-trim");
    css_prop!(text_transform, "text-transform");
    css_prop!(text_underline_offset, "text-underline-offset");
    css_prop!(text_underline_position, "text-underline-position");
    css_prop!(text_wrap, "text-wrap");
    css_prop!(text_wrap_mode, "text-wrap-mode");
    css_prop!(text_wrap_style, "text-wrap-style");
}

impl Rule {
    /// Create a CSS rule
    pub fn new(selector: Selector) -> Self {
        let mut prop = Prop::new();
        prop.sep = "\n";
        Rule { selector, prop }
    }

    /// Get property list
    pub fn prop(&mut self) -> &mut Prop {
        &mut self.prop
    }
}
