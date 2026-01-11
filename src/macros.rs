// macros.rs
//
// Copyright (C) 2025-2026  Douglas P Lau

/// Create an HTML element
#[rustfmt::skip]
macro_rules! html_elem {
    ( $el:literal, $elem:ident, $desc:literal, $items:ident() ) => {
        html_elem!($el, $elem, $desc, $items(), ElemType::Html);
    };

    ( $el:literal, $elem:ident, $desc:literal, $items:ident(), $tp:expr ) => {
        #[doc = concat!(
            "`<",
            $el,
            ">`: [",
            $desc,
            "](",
            "https://developer.mozilla.org/en-US/docs/Web/HTML/",
            "Reference/Elements/",
            stringify!($elem),
            ") element",
        )]
        pub struct $elem<'p> {
            /// Borrowed Page
            pub(crate) page: &'p mut Page,
            /// Node depth
            pub(crate) depth: usize,
        }

        #[doc = concat!("`<", $el, ">` items")]
        impl<'p> $elem<'p> {
            $items!( $el );

            #[doc = "Close the element"]
            #[doc = ""]
            #[doc = concat!(
                "- Closes all child elements\n",
                "- Adds the closing tag if necessary (e.g. `</",
                $el,
                ">`)"
            )]
            pub fn close(&'p mut self) -> &'p mut Page {
                self.page.close_to(self.depth);
                self.page
            }
        }

        #[doc = "Global attributes"]
        impl<'p> $elem<'p> {
            global_attributes!();
        }

        impl<'p> Element<'p> for $elem<'p> {
            const TAG: &'static str = $el;
            const TP: ElemType = $tp;
            fn new(page: &'p mut Page) -> Self {
                $elem { page, depth: 1 }
            }
        }
    };
}

/// Make an HTML "value" attribute method
#[rustfmt::skip]
macro_rules! val_attr {
    ( $path:expr, $attr:ident, $raw_attr:expr ) => {
        #[doc = concat!(
            "Add [",
            $raw_attr,
            "](",
            "https://developer.mozilla.org/en-US/docs/",
            $path,
            $raw_attr,
            ") attribute",
        )]
        pub fn $attr<'a, V>(&mut self, val: V) -> &mut Self
        where
            V: Into<Value<'a>>,
        {
            self.page.attr($raw_attr, val);
            self
        }
    };
}

/// Make an HTML Boolean attribute method
#[rustfmt::skip]
macro_rules! bool_attr {
    ( $path:expr, $attr:ident, $raw_attr:expr ) => {
        #[doc = concat!(
            "Add [",
            $raw_attr,
            "](",
            "https://developer.mozilla.org/en-US/docs/",
            $path,
            $raw_attr,
            ") Boolean attribute",
        )]
        pub fn $attr(&mut self) -> &mut Self {
            self.page.attr_bool($raw_attr);
            self
        }
    };
}

/// Make an HTML attribute method
macro_rules! html_attr {
    // Make a non-global "value" attribute
    ( $el:expr, $attr:ident ) => {
        val_attr!(
            concat!("Web/HTML/Reference/Elements/", $el, "#"),
            $attr,
            stringify!($attr)
        );
    };

    // Make a non-global Boolean attribute
    ( $el:expr, $attr:ident, true ) => {
        bool_attr!(
            concat!("Web/HTML/Reference/Elements/", $el, "#"),
            $attr,
            stringify!($attr)
        );
    };

    // Make a non-global "value" attribute with raw-string name (e.g. r#type)
    ( $el:expr, $attr:ident, $raw_attr:literal ) => {
        val_attr!(
            concat!("Web/HTML/Reference/Elements/", $el, "#"),
            $attr,
            $raw_attr
        );
    };

    // Make a non-global Boolean attribute with raw-string name (e.g. r#loop)
    ( $el:expr, $attr:ident, $raw_attr:literal, true ) => {
        bool_attr!(
            concat!("Web/HTML/Reference/Elements/", $el, "#"),
            $attr,
            $raw_attr
        );
    };
}

/// Make a global HTML attribute method
macro_rules! global_attribute {
    // Make a global "value" attribute
    ( $attr:ident ) => {
        val_attr!(
            concat!("Web/HTML/Reference/", "Global_attributes/#"),
            $attr,
            stringify!($attr)
        );
    };

    // Make a global Boolean attribute
    ( $attr:ident, true ) => {
        bool_attr!(
            concat!("Web/HTML/Reference/", "Global_attributes/#"),
            $attr,
            stringify!($attr)
        );
    };
}

/// Global attributes
macro_rules! global_attributes {
    () => {
        global_attribute!(id);
        global_attribute!(class);
        // less-common...
        global_attribute!(accesskey);
        global_attribute!(autocapitalize);
        global_attribute!(autocorrect);
        global_attribute!(autofocus, true);
        global_attribute!(contenteditable);
        /* FIXME: data-* */
        global_attribute!(dir);
        global_attribute!(draggable);
        global_attribute!(enterkeyhint);
        global_attribute!(exportparts);
        /* FUTURE: headingoffset, headingreset... */
        global_attribute!(hidden);
        global_attribute!(inert, true);
        global_attribute!(inputmode);
        global_attribute!(is);
        global_attribute!(itemid);
        global_attribute!(itemprop);
        global_attribute!(itemref);
        global_attribute!(itemscope, true);
        global_attribute!(itemtype);
        global_attribute!(lang);
        global_attribute!(nonce);
        /* FIXME: event attributes: onauxclick, etc. */
        global_attribute!(part);
        global_attribute!(popover);
        global_attribute!(role);
        global_attribute!(slot);
        global_attribute!(spellcheck);
        global_attribute!(style);
        global_attribute!(tabindex);
        global_attribute!(title);
        global_attribute!(translate);
        /* FUTURE: virtualkeyboardpolicy, writingsuggestions */
    };
}

/// Create an element method (HTML or SVG)
macro_rules! elem_method {
    ( $meth:ident, $elem:ident ) => {
        #[doc = concat!("Add `", stringify!($elem), "` child element")]
        #[allow(clippy::self_named_constructors)]
        pub fn $meth(self: &mut Self) -> $elem<'_> {
            let depth = self.page.elem($elem::TAG, $elem::TP);
            $elem {
                page: self.page,
                depth,
            }
        }
    };
}

/// CData method
macro_rules! cdata_methods {
    () => {
        /// Add character data content
        ///
        /// These characters will be replaced with entities:
        ///
        /// | Char | Entity  |
        /// |------|---------|
        /// | `&`  | `&amp;` |
        /// | `<`  | `&lt;`  |
        /// | `>`  | `&gt;`  |
        pub fn cdata<'a, V>(&mut self, text: V) -> &mut Self
        where
            V: Into<Value<'a>>,
        {
            self.page.cdata(text);
            self
        }

        /// Add character data content with a maximum character limit
        ///
        /// | Char | Entity  |
        /// |------|---------|
        /// | `&`  | `&amp;` |
        /// | `<`  | `&lt;`  |
        /// | `>`  | `&gt;`  |
        pub fn cdata_len<'a, V>(&mut self, text: V, len: usize) -> &mut Self
        where
            V: Into<Value<'a>>,
        {
            self.page.cdata_len(text, len);
            self
        }
    };
}

/// Comment and raw methods
macro_rules! comment_raw_methods {
    () => {
        /// Add a comment
        ///
        /// These characters will be replaced with entities:
        ///
        /// | Char | Entity     |
        /// |------|------------|
        /// | `-`  | `&hyphen;` |
        /// | `<`  | `&gt;`     |
        /// | `>`  | `&lt;`     |
        pub fn comment<'v, V>(&mut self, com: V) -> &mut Self
        where
            V: Into<Value<'v>>,
        {
            self.page.comment(com);
            self
        }

        /// Add raw content
        ///
        /// **WARNING**: `trusted` is used verbatim, with no escaping;
        ///              do not call with untrusted content.
        pub fn raw(&mut self, trusted: impl AsRef<str>) -> &mut Self {
            self.page.raw(trusted);
            self
        }
    };
}

/// Metadata content
macro_rules! metadata_content {
    () => {
        elem_method!(base, Base);
        elem_method!(link, Link);
        elem_method!(meta, Meta);
        elem_method!(noscript, NoScript);
        elem_method!(script, Script);
        elem_method!(style_el, Style);
        elem_method!(template, Template);
        elem_method!(title_el, Title);
        comment_raw_methods!();
    };
}

/// Flow content
macro_rules! flow_content {
    ($abbr:ident, $cite:ident, $form:ident) => {
        cdata_methods!();
        elem_method!(a, A);
        elem_method!($abbr, Abbr);
        elem_method!(address, Address);
        elem_method!(article, Article);
        elem_method!(aside, Aside);
        elem_method!(audio, Audio);
        elem_method!(b, B);
        elem_method!(bdi, Bdi);
        elem_method!(bdo, Bdo);
        elem_method!(blockquote, BlockQuote);
        elem_method!(br, Br);
        elem_method!(button, Button);
        elem_method!(canvas, Canvas);
        elem_method!($cite, Cite);
        elem_method!(code, Code);
        elem_method!(data, Data);
        elem_method!(datalist, DataList);
        elem_method!(del, Del);
        elem_method!(details, Details);
        elem_method!(dfn, Dfn);
        elem_method!(dialog, Dialog);
        elem_method!(div, Div);
        elem_method!(dl, Dl);
        elem_method!(em, Em);
        elem_method!(embed, Embed);
        elem_method!(fieldset, FieldSet);
        elem_method!(figure, Figure);
        elem_method!(footer, Footer);
        elem_method!($form, Form);
        elem_method!(h1, H1);
        elem_method!(h2, H2);
        elem_method!(h3, H3);
        elem_method!(h4, H4);
        elem_method!(h5, H5);
        elem_method!(h6, H6);
        elem_method!(header, Header);
        elem_method!(hgroup, HGroup);
        elem_method!(hr, Hr);
        elem_method!(i, I);
        elem_method!(iframe, IFrame);
        elem_method!(img, Img);
        elem_method!(input, Input);
        elem_method!(ins, Ins);
        elem_method!(kbd, Kbd);
        elem_method!(label, Label);
        elem_method!(main, Main);
        elem_method!(map, Map);
        elem_method!(mark, Mark);
        // elem_method!(math, Math);
        elem_method!(menu, Menu);
        elem_method!(meter, Meter);
        elem_method!(nav, Nav);
        elem_method!(noscript, NoScript);
        elem_method!(object, Object);
        elem_method!(ol, Ol);
        elem_method!(output, Output);
        elem_method!(p, P);
        elem_method!(picture, Picture);
        elem_method!(pre, Pre);
        elem_method!(progress, Progress);
        elem_method!(q, Q);
        elem_method!(ruby, Ruby);
        elem_method!(s, S);
        elem_method!(samp, Samp);
        elem_method!(script, Script);
        elem_method!(search, Search);
        elem_method!(section, Section);
        elem_method!(select, Select);
        elem_method!(slot_el, Slot); // NOTE: global attr slot
        elem_method!(small, Small);
        elem_method!(span, Span);
        elem_method!(strong, Strong);
        elem_method!(sub, Sub);
        elem_method!(sup, Sup);
        elem_method!(svg, Svg);
        elem_method!(table, Table);
        elem_method!(template, Template);
        elem_method!(textarea, TextArea);
        elem_method!(time, Time);
        elem_method!(u, U);
        elem_method!(ul, Ul);
        elem_method!(var, Var);
        elem_method!(video, Video);
        elem_method!(wbr, Wbr);
        comment_raw_methods!();
    };
}

/// Phrasing content
macro_rules! phrasing_content {
    ($cite:ident) => {
        cdata_methods!();
        elem_method!(a, A); // NOTE: containing only phrasing content
        elem_method!(abbr, Abbr);
        elem_method!(area, Area); // NOTE: only descendants of <map>
        elem_method!(audio, Audio);
        elem_method!(b, B);
        elem_method!(bdi, Bdi);
        elem_method!(bdo, Bdo);
        elem_method!(br, Br);
        elem_method!(button, Button);
        elem_method!(canvas, Canvas);
        elem_method!($cite, Cite);
        elem_method!(code, Code);
        elem_method!(data, Data);
        elem_method!(datalist, DataList);
        elem_method!(del, Del); // NOTE: containing only phrasing content
        elem_method!(dfn, Dfn);
        elem_method!(em, Em);
        elem_method!(embed, Embed);
        elem_method!(i, I);
        elem_method!(iframe, IFrame);
        elem_method!(img, Img);
        elem_method!(input, Input);
        elem_method!(ins, Ins); // NOTE: containing only phrasing content
        elem_method!(kbd, Kbd);
        elem_method!(label, Label);
        elem_method!(link, Link); // NOTE: must have itemprop attribute
        elem_method!(map, Map); // NOTE: containing only phrasing content
        elem_method!(mark, Mark);
        // elem_method!(math, Math); // FIXME
        elem_method!(meta, Meta); // NOTE: must have itemprop attribute
        elem_method!(meter, Meter);
        elem_method!(noscript, NoScript);
        elem_method!(object, Object);
        elem_method!(output, Output);
        elem_method!(picture, Picture);
        elem_method!(progress, Progress);
        elem_method!(q, Q);
        elem_method!(ruby, Ruby);
        elem_method!(s, S);
        elem_method!(samp, Samp);
        elem_method!(script, Script);
        elem_method!(select, Select);
        elem_method!(slot_el, Slot);
        elem_method!(small, Small);
        elem_method!(span, Span);
        elem_method!(strong, Strong);
        elem_method!(sub, Sub);
        elem_method!(sup, Sup);
        elem_method!(svg, Svg);
        elem_method!(template, Template);
        elem_method!(textarea, TextArea);
        elem_method!(time, Time);
        elem_method!(u, U);
        elem_method!(var, Var);
        elem_method!(video, Video);
        elem_method!(wbr, Wbr);
        comment_raw_methods!();
    };
}

/// Non-interactive phrasing content
macro_rules! non_interactive_phrasing_content {
    () => {
        cdata_methods!();
        // a with href attribute is interactive
        elem_method!(abbr, Abbr);
        elem_method!(area, Area); // NOTE: only descendants of <map>
        // audio with controls attribute is interactive
        elem_method!(b, B);
        elem_method!(bdi, Bdi);
        elem_method!(bdo, Bdo);
        elem_method!(br, Br);
        // button is interactive
        elem_method!(canvas, Canvas);
        elem_method!(cite, Cite);
        elem_method!(code, Code);
        elem_method!(data, Data);
        elem_method!(datalist, DataList);
        elem_method!(del, Del); // NOTE: containing only phrasing content
        elem_method!(dfn, Dfn);
        elem_method!(em, Em);
        // embed is interactive
        elem_method!(i, I);
        // iframe is interactive
        elem_method!(img, Img); // with usemap attribute is interactive
        // input is interactive (if not hidden)
        elem_method!(ins, Ins); // NOTE: containing only phrasing content
        elem_method!(kbd, Kbd);
        // label is interactive
        elem_method!(link, Link); // NOTE: must have itemprop attribute
        elem_method!(map, Map); // NOTE: containing only phrasing content
        elem_method!(mark, Mark);
        // elem_method!(math, Math);
        elem_method!(meta, Meta); // NOTE: must have itemprop attribute
        elem_method!(meter, Meter);
        elem_method!(noscript, NoScript);
        elem_method!(object, Object); // with usemap attribute is interactive
        elem_method!(output, Output);
        elem_method!(picture, Picture);
        elem_method!(progress, Progress);
        elem_method!(q, Q);
        elem_method!(ruby, Ruby);
        elem_method!(s, S);
        elem_method!(samp, Samp);
        elem_method!(script, Script);
        // select is interactive
        elem_method!(slot_el, Slot);
        elem_method!(small, Small);
        elem_method!(span, Span);
        elem_method!(strong, Strong);
        elem_method!(sub, Sub);
        elem_method!(sup, Sup);
        elem_method!(svg, Svg);
        elem_method!(template, Template);
        // textarea is interactive
        elem_method!(time, Time);
        elem_method!(u, U);
        elem_method!(var, Var);
        // video with controls attribute is interactive
        elem_method!(wbr, Wbr);
        comment_raw_methods!();
    };
}

/// Text content
macro_rules! text_content {
    () => {
        cdata_methods!();
        comment_raw_methods!();
    };
}

/// Address content (flow, with some restrictions)
macro_rules! address_content {
    () => {
        cdata_methods!();
        elem_method!(a, A);
        elem_method!(abbr, Abbr);
        // address not allowed
        // article not allowed
        // aside not allowed
        elem_method!(audio, Audio);
        elem_method!(b, B);
        elem_method!(bdi, Bdi);
        elem_method!(bdo, Bdo);
        elem_method!(blockquote, BlockQuote);
        elem_method!(br, Br);
        elem_method!(button, Button);
        elem_method!(canvas, Canvas);
        elem_method!(cite, Cite);
        elem_method!(code, Code);
        elem_method!(data, Data);
        elem_method!(datalist, DataList);
        elem_method!(del, Del);
        elem_method!(details, Details);
        elem_method!(dfn, Dfn);
        elem_method!(dialog, Dialog);
        elem_method!(div, Div);
        elem_method!(dl, Dl);
        elem_method!(em, Em);
        elem_method!(embed, Embed);
        elem_method!(fieldset, FieldSet);
        elem_method!(figure, Figure);
        // footer not allowed
        elem_method!(form, Form);
        // h1 - h6 not allowed
        // header not allowed
        // hgroup not allowed
        elem_method!(hr, Hr);
        elem_method!(i, I);
        elem_method!(iframe, IFrame);
        elem_method!(img, Img);
        elem_method!(input, Input);
        elem_method!(ins, Ins);
        elem_method!(kbd, Kbd);
        elem_method!(label, Label);
        elem_method!(main, Main);
        elem_method!(map, Map);
        elem_method!(mark, Mark);
        // elem_method!(math, Math);
        elem_method!(menu, Menu);
        elem_method!(meter, Meter);
        // nav not allowed
        elem_method!(noscript, NoScript);
        elem_method!(object, Object);
        elem_method!(ol, Ol);
        elem_method!(output, Output);
        elem_method!(p, P);
        elem_method!(picture, Picture);
        elem_method!(pre, Pre);
        elem_method!(progress, Progress);
        elem_method!(q, Q);
        elem_method!(ruby, Ruby);
        elem_method!(s, S);
        elem_method!(samp, Samp);
        elem_method!(script, Script);
        elem_method!(search, Search);
        // section not allowed
        elem_method!(select, Select);
        elem_method!(slot_el, Slot);
        elem_method!(small, Small);
        elem_method!(span, Span);
        elem_method!(strong, Strong);
        elem_method!(sub, Sub);
        elem_method!(sup, Sup);
        elem_method!(svg, Svg);
        elem_method!(table, Table);
        elem_method!(template, Template);
        elem_method!(textarea, TextArea);
        elem_method!(time, Time);
        elem_method!(u, U);
        elem_method!(ul, Ul);
        elem_method!(var, Var);
        elem_method!(video, Video);
        elem_method!(wbr, Wbr);
        comment_raw_methods!();
    };
}

/// Create an SVG element
#[rustfmt::skip]
macro_rules! svg_elem {
    ( $el:literal, $elem:ident, $desc:literal, $items:ident() ) => {
        svg_elem!($el, $elem, $desc, $items(), ElemType::Xml);
    };

    ( $el:literal, $elem:ident, $desc:literal, $items:ident(), $tp:expr ) => {
        #[doc = concat!(
            "`<",
            $el,
            ">`: [",
            $desc,
            "](",
            "https://developer.mozilla.org/en-US/docs/Web/SVG/",
            "Reference/Element/",
            stringify!($elem),
            ") SVG element",
        )]
        pub struct $elem<'p> {
            /// Borrowed Page
            pub(crate) page: &'p mut Page,
            /// Node depth
            pub(crate) depth: usize,
        }

        #[doc = concat!("`<", $el, ">` items")]
        impl<'p> $elem<'p> {
            $items!( $el );

            #[doc = "Close the element"]
            #[doc = ""]
            #[doc = concat!(
                "- Closes all child elements\n",
                "- Adds the closing tag if necessary (e.g. `</",
                $el,
                ">`)"
            )]
            pub fn close(&'p mut self) -> &'p mut Page {
                self.page.close_to(self.depth);
                self.page
            }
        }

        #[doc = "Global SVG attributes"]
        impl<'p> $elem<'p> {
            svg_global_attributes!();
        }

        impl<'p> Element<'p> for $elem<'p> {
            const TAG: &'static str = $el;
            const TP: ElemType = $tp;
            fn new(page: &'p mut Page) -> Self {
                $elem { page, depth: 1 }
            }
        }
    }
}

/// Make an SVG attribute method
macro_rules! svg_attr {
    // Make an SVG attribute
    ( $attr:ident ) => {
        svg_attr!($attr, stringify!($attr));
    };

    // Make an SVG attribute with raw-string name (e.g. r#use)
    ( $attr:ident, $raw_attr:expr ) => {
        val_attr!("Web/SVG/Reference/Attribute/", $attr, $raw_attr);
    };

    // Make an SVG Boolean attribute
    ( $attr:ident, $raw_attr:expr, true ) => {
        bool_attr!("Web/SVG/Reference/Attribute/", $attr, $raw_attr);
    };
}

/// SVG global attributes
macro_rules! svg_global_attributes {
    () => {
        svg_attr!(id);
        // NOTE: only allowed on *most* SVG elements, for some reason
        svg_attr!(class);
        svg_attr!(style);
        // less common...
        svg_attr!(autofocus, "autofocus", true);
        /* FIXME: data-* */
        svg_attr!(lang);
        svg_attr!(tabindex);
        svg_attr!(transform);
    };
}

/// SVG support attributes
macro_rules! svg_support_attr {
    () => {
        svg_attr!(required_extensions, "requiredExtensions");
        svg_attr!(system_language, "systemLanguage");
    };
}

/// Graphics content
macro_rules! svg_graphics {
    () => {
        elem_method!(circle, Circle);
        elem_method!(ellipse, Ellipse);
        elem_method!(image, Image);
        elem_method!(line, Line);
        elem_method!(path, Path);
        elem_method!(polygon, Polygon);
        elem_method!(polyline, Polyline);
        elem_method!(rect, Rect);
        elem_method!(text, Text);
        elem_method!(r#use, Use);
    };
}

/// Container content
macro_rules! svg_container {
    () => {
        elem_method!(a, A);
        elem_method!(defs, Defs);
        elem_method!(g, G);
        elem_method!(marker, Marker);
        elem_method!(mask, Mask);
        elem_method!(pattern, Pattern);
        elem_method!(svg, Svg);
        elem_method!(switch, Switch);
        elem_method!(symbol, Symbol);
    };
}

/// Descriptive content
macro_rules! svg_descriptive {
    ($title:ident) => {
        elem_method!(desc, Desc);
        elem_method!(metadata, Metadata);
        elem_method!($title, Title);
    };
}

/// Gradient content
macro_rules! svg_gradient {
    () => {
        elem_method!(linear_gradient, LinearGradient);
        elem_method!(radial_gradient, RadialGradient);
        elem_method!(stop, Stop);
    };
}

/// Other content
macro_rules! svg_other {
    () => {
        elem_method!(clip_path, ClipPath);
        elem_method!(filter, Filter);
        elem_method!(foreign_object, ForeignObject);
        elem_method!(script, Script);
        elem_method!(style_el, Style);
        elem_method!(view, View);
    };
}

/// Animation content
macro_rules! svg_animation {
    () => {
        elem_method!(animate, Animate);
        elem_method!(animate_motion, AnimateMotion);
        elem_method!(animate_transform, AnimateTransform);
        elem_method!(mpath, MPath);
        elem_method!(set, Set);
    };
}

/// Svg content
macro_rules! svg_content {
    ($title:ident) => {
        svg_graphics!();
        svg_container!();
        svg_descriptive!($title);
        svg_gradient!();
        svg_other!();
        svg_animation!();
    };
}
