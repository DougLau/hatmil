// elem.rs
//
// Copyright (C) 2025  Douglas P Lau
//
//! HTML Elements
use crate::html::Page;
use crate::value::Value;

/// Element borrowed from a [Page]
pub trait Element<'p> {
    /// Element tag
    const TAG: &'static str;

    /// Make a new element
    fn new(page: &'p mut Page) -> Self;

    /// End the element
    ///
    /// Adds the closing tag (e.g. `</span>`).
    fn end(self) -> &'p mut Page;
}

/// Create an HTML element
macro_rules! element {
    ( $el:literal, $elem:ident, $desc:literal, $items:ident() ) => {
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
            page: &'p mut Page,
        }

        #[doc = concat!("`<", $el, ">` items")]
        impl<'p> $elem<'p> {
            $items!( $el );
        }

        #[doc = "Global attributes"]
        impl<'p> $elem<'p> {
            global_attributes!();
        }

        impl<'p> Element<'p> for $elem<'p> {
            const TAG: &'static str = $el;
            fn new(page: &'p mut Page) -> Self {
                $elem { page }
            }
            fn end(self) -> &'p mut Page {
                self.page
            }
        }
    };
}

/// Make an HTML attribute method
macro_rules! attribute {
    // Make a non-global "value" attribute
    ( $el:literal, $attr:ident ) => {
        attribute!( concat!("Elements/", $el), $attr );
    };

    // Make a non-global Boolean attribute
    ( $el:literal, $attr:ident, true ) => {
        attribute!( concat!("Elements/", $el), $attr, true );
    };

    // Make a non-global "value" attribute with raw-string name (e.g. r#type)
    ( $el:literal, $attr:ident, $raw_attr:literal ) => {
        attribute!( concat!("Elements/", $el), $attr ); // FIXME
    };

    // Make a non-global Boolean attribute with raw-string name (e.g. r#loop)
    ( $el:literal, $attr:ident, $raw_attr:literal, true ) => {
        attribute!( concat!("Elements/", $el), $attr, true ); // FIXME
    };

    // Make a global "value" attribute
    ( $attr:ident ) => {
        attribute!( concat!("Global_attributes/", ""), $attr );
    };

    // Make a global Boolean attribute
    ( $attr:ident, true ) => {
        attribute!( concat!("Global_attributes/", ""), $attr, true);
    };

    // Make a "value" attribute
    ( $path:expr, $attr:ident ) => {
        #[doc = concat!(
                    "Add [",
                    stringify!($attr),
                    "](",
                    "https://developer.mozilla.org/en-US/docs/Web/HTML/",
                    "Reference/",
                    $path,
                    "#",
                    stringify!($attr),
                    ") attribute",
                )]
        pub fn $attr<'a, V>(self, val: V) -> Self
        where
            V: Into<Value<'a>>,
        {
            self.page.attr(stringify!($attr), val);
            self
        }
    };

    // Make a Boolean attribute
    ( $path:expr, $attr:ident, true ) => {
        #[doc = concat!(
                    "Add [",
                    stringify!($attr),
                    "](",
                    "https://developer.mozilla.org/en-US/docs/Web/HTML/",
                    "Reference/",
                    $path,
                    "#",
                    stringify!($attr),
                    ") Boolean attribute",
                )]
        pub fn $attr(self) -> Self {
            self.page.attr_bool(stringify!($attr));
            self
        }
    };
}

/// Global attributes
macro_rules! global_attributes {
    () => {
        attribute!(accesskey);
        attribute!(autocapitalize);
        attribute!(autocorrect);
        attribute!(autofocus, true);
        attribute!(class);
        attribute!(contenteditable);
        /* data-* */
        attribute!(dir);
        attribute!(draggable);
        attribute!(enterkeyhint);
        attribute!(exportparts);
        attribute!(hidden);
        attribute!(id);
        attribute!(inert, true);
        attribute!(is);
        attribute!(inputmode);
        /* itemid, itemprop, itemref, itemscope, itemtype */
        attribute!(lang);
        attribute!(nonce);
        attribute!(part);
        attribute!(popover);
        attribute!(role);
        attribute!(slot);
        attribute!(spellcheck);
        attribute!(style);
        attribute!(tabindex);
        attribute!(title);
        attribute!(translate);
        /* virtualkeyboardpolicy, writingsuggestions */
    };
}

// A element
macro_rules! a_items {
    ( $el:literal ) => {
        attribute!($el, download);
        attribute!($el, href);
        attribute!($el, hreflang);
        /* interestfor */
        attribute!($el, ping);
        attribute!($el, referrerpolicy);
        attribute!($el, rel);
        attribute!($el, target);
        attribute!($el, r#type, "type");
        // FIXME: content
    };
}
element!("a", A, "Anchor", a_items());

// Abbr element
macro_rules! abbr_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("abbr", Abbr, "Abbreviation", abbr_items());

// Address element
macro_rules! address_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("address", Address, "Contact Address", address_items());

// Article element
macro_rules! article_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("article", Article, "Article Contents", article_items());

// Area element (void)
macro_rules! area_items {
    ( $el:literal ) => {
        attribute!($el, alt);
        attribute!($el, coords);
        attribute!($el, download);
        attribute!($el, href);
        /* interestfor */
        attribute!($el, ping);
        attribute!($el, referrerpolicy);
        attribute!($el, rel);
        attribute!($el, shape);
        attribute!($el, target);
        // FIXME: content
    };
}
element!("area", Area, "Image Map Area", area_items());

// Aside element
macro_rules! aside_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("aside", Aside, "Aside", aside_items());

// Audio element
macro_rules! audio_items {
    ( $el:literal ) => {
        attribute!($el, autoplay, true);
        attribute!($el, controls, true);
        attribute!($el, controlslist);
        attribute!($el, crossorigin);
        attribute!($el, disableremoteplayback, true);
        attribute!($el, r#loop, "loop", true);
        attribute!($el, preload);
        attribute!($el, src);
        // FIXME: content
    };
}
element!("audio", Audio, "Embed Audio", audio_items());

// B element
macro_rules! b_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("b", B, "Bring Attention To (Bold)", b_items());

// Bdi element
macro_rules! bdi_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("bdi", Bdi, "Bidirectional Isolate", bdi_items());

// Bdo element
macro_rules! bdo_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("bdo", Bdo, "Bidirectional Override", bdo_items());

// BlockQuote element
macro_rules! blockquote_items {
    ( $el:literal ) => {
        attribute!($el, cite);
        // FIXME: content
    };
}
element!(
    "blockquote",
    BlockQuote,
    "Block Quotation",
    blockquote_items()
);

// Base element
macro_rules! base_items {
    ( $el:literal ) => {
        attribute!($el, href);
        attribute!($el, target);
        // no content (void)
    };
}
element!("base", Base, "Base URL", base_items());

// Body element
macro_rules! body_items {
    ( $el:literal ) => {
        // FIXME: event attributes; onafterprint, onbeforeprint...
        flow_content!();
    };
}
element!("body", Body, "Document Body", body_items());

// Line break element (void)
macro_rules! br_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("br", Br, "Line Break", br_items());

// Button element
macro_rules! button_items {
    ( $el:literal ) => {
        attribute!($el, command);
        attribute!($el, commandfor);
        attribute!($el, disabled, true);
        attribute!($el, form);
        attribute!($el, formaction);
        attribute!($el, formenctype);
        attribute!($el, formmethod);
        attribute!($el, formnovalidate, true);
        attribute!($el, formtarget);
        /* interestfor */
        attribute!($el, name);
        attribute!($el, popovertarget);
        attribute!($el, popovertargetaction);
        attribute!($el, r#type, "type");
        attribute!($el, value);
        // FIXME: content
    };
}
element!("button", Button, "Button", button_items());

// Canvas element
macro_rules! canvas_items {
    ( $el:literal ) => {
        attribute!($el, height);
        attribute!($el, width);
        // FIXME: content
    };
}
element!("canvas", Canvas, "Graphics Canvas", canvas_items());

// Caption element
macro_rules! caption_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("caption", Caption, "Table Caption", caption_items());

// Cite element
macro_rules! cite_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("cite", Cite, "Citation", cite_items());

// Code element
macro_rules! code_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("code", Code, "Inline Code", code_items());

// Col element (void)
macro_rules! col_items {
    ( $el:literal ) => {
        attribute!($el, span);
        // FIXME: content
    };
}
element!("col", Col, "Table Column", col_items());

// ColGroup element
macro_rules! colgroup_items {
    ( $el:literal ) => {
        attribute!($el, span);
    };
}
element!("colgroup", ColGroup, "Table Column Group", colgroup_items());

// Data element
macro_rules! data_items {
    ( $el:literal ) => {
        attribute!($el, value);
        // FIXME: content
    };
}
element!("data", Data, "Data", data_items());

// DataList element
macro_rules! datalist_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("datalist", DataList, "Data List", datalist_items());

// Dd element
macro_rules! dd_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("dd", Dd, "Description Details", dd_items());

// Del element
macro_rules! del_items {
    ( $el:literal ) => {
        attribute!($el, cite);
        attribute!($el, datetime);
        // FIXME: content
    };
}
element!("del", Del, "Deleted Text", del_items());

// Details element
macro_rules! details_items {
    ( $el:literal ) => {
        attribute!($el, open, true);
        attribute!($el, name);
        // FIXME: content
    };
}
element!("details", Details, "Details Disclosure", details_items());

// Dfn element
macro_rules! dfn_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("dfn", Dfn, "Definition", dfn_items());

// Dialog element
macro_rules! dialog_items {
    ( $el:literal ) => {
        attribute!($el, closedby);
        attribute!($el, open, true);
        // FIXME: content
    };
}
element!("dialog", Dialog, "Dialog", dialog_items());

// Div element
macro_rules! div_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("div", Div, "Content Division", div_items());

// Dl element
macro_rules! dl_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("dl", Dl, "Description List", dl_items());

// Dt element
macro_rules! dt_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("dt", Dt, "Description Term", dt_items());

// Em element
macro_rules! em_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("em", Em, "Emphasis", em_items());

// Embed element (void)
macro_rules! embed_items {
    ( $el:literal ) => {
        attribute!($el, height);
        attribute!($el, src);
        attribute!($el, r#type, "type");
        attribute!($el, width);
        // FIXME: content
    };
}
element!("embed", Embed, "Embed External Content", embed_items());

// FeildSet element
macro_rules! fieldset_items {
    ( $el:literal ) => {
        attribute!($el, disabled, true);
        attribute!($el, form);
        attribute!($el, name);
        // FIXME: content
    };
}
element!("fieldset", FieldSet, "Field Set", fieldset_items());

// FigCaption element
macro_rules! figcaption_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!(
    "figcaption",
    FigCaption,
    "Figure Caption",
    figcaption_items()
);

// Figure element
macro_rules! figure_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("figure", Figure, "Figure", figure_items());

// Footer element
macro_rules! footer_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("footer", Footer, "Footer", footer_items());

// Form element
macro_rules! form_items {
    ( $el:literal ) => {
        attribute!($el, accept_charset, "accept-charset");
        attribute!($el, autocomplete);
        attribute!($el, name);
        attribute!($el, rel);
        attribute!($el, action);
        attribute!($el, enctype);
        attribute!($el, method);
        attribute!($el, novalidate, true);
        attribute!($el, target);
        // FIXME: content
    };
}
element!("form", Form, "Form", form_items());

// h1 element
macro_rules! h1_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
// FIXME: MDN links broken; "Heading_Elements"
element!("h1", H1, "Section Heading 1", h1_items());

// h2 element
macro_rules! h2_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("h2", H2, "Section Heading 2", h2_items());

// h3 element
macro_rules! h3_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("h3", H3, "Section Heading 3", h3_items());

// h4 element
macro_rules! h4_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("h4", H4, "Section Heading 4", h4_items());

// h5 element
macro_rules! h5_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("h5", H5, "Section Heading 5", h5_items());

// h6 element
macro_rules! h6_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("h6", H6, "Section Heading 6", h6_items());

// Head element
macro_rules! head_items {
    ( $el:literal ) => {
        metadata_content!();
    };
}
element!("head", Head, "Header / Document Metadata", head_items());

// Header element
macro_rules! header_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("header", Header, "Header", header_items());

// HGroup element
macro_rules! hgroup_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("hgroup", HGroup, "Heading Group", hgroup_items());

// Hr element
macro_rules! hr_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("hr", Hr, "Horizontal Rule", hr_items());

// Html element
macro_rules! html_items {
    ( $el:literal ) => {
        elem_method!(head, Head);
        elem_method!(body, Body);
        comment_method!();
    };
}
element!("html", Html, "HTML Document Root", html_items());

// I element
macro_rules! i_items {
    ( $el:literal ) => {
        comment_method!();
    };
}
element!("i", I, "Idiomatic Text (Italic)", i_items());

// Iframe element
macro_rules! iframe_items {
    ( $el:literal ) => {
        attribute!($el, allow);
        /* credentialless, csp */
        attribute!($el, height);
        attribute!($el, loading);
        attribute!($el, name);
        /* privatetoken */
        attribute!($el, referrerpolicy);
        attribute!($el, sandbox);
        attribute!($el, src);
        attribute!($el, srcdoc);
        attribute!($el, width);
        // FIXME: content
    };
}
element!("iframe", IFrame, "Inline Frame", iframe_items());

// Img element
macro_rules! img_items {
    ( $el:literal ) => {
        attribute!($el, alt);
        attribute!($el, crossorigin);
        attribute!($el, decoding);
        attribute!($el, elementtiming);
        attribute!($el, fetchpriority);
        attribute!($el, height);
        attribute!($el, ismap, true);
        attribute!($el, loading);
        attribute!($el, referrerpolicy);
        attribute!($el, sizes);
        attribute!($el, src);
        attribute!($el, srcset);
        attribute!($el, width);
        attribute!($el, usemap);
        // FIXME: content
    };
}
element!("img", Img, "Embedded Image", img_items());

// Input element
macro_rules! input_items {
    ( $el:literal ) => {
        attribute!($el, accept);
        attribute!($el, alpha);
        attribute!($el, alt);
        attribute!($el, autocomplete);
        attribute!($el, capture);
        attribute!($el, checked, true);
        attribute!($el, colorspace);
        attribute!($el, dirname);
        attribute!($el, disabled, true);
        attribute!($el, form);
        attribute!($el, formaction);
        attribute!($el, formenctype);
        attribute!($el, formmethod);
        attribute!($el, formnovalidate, true);
        attribute!($el, formtarget);
        attribute!($el, height);
        attribute!($el, list);
        attribute!($el, max);
        attribute!($el, maxlength);
        attribute!($el, min);
        attribute!($el, minlength);
        attribute!($el, multiple, true);
        attribute!($el, name);
        attribute!($el, pattern);
        attribute!($el, placeholder);
        attribute!($el, popovertarget);
        attribute!($el, popovertargetaction);
        attribute!($el, readonly, true);
        attribute!($el, required, true);
        attribute!($el, size);
        attribute!($el, src);
        attribute!($el, step);
        attribute!($el, r#type, "type");
        attribute!($el, value);
        attribute!($el, width);
        // FIXME: content
    };
}
element!("input", Input, "Input", input_items());

// Ins element
macro_rules! ins_items {
    ( $el:literal ) => {
        attribute!($el, cite);
        attribute!($el, datetime);
        // FIXME: content
    };
}
element!("ins", Ins, "Inserted Text", ins_items());

// Kbd element
macro_rules! kbd_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("kbd", Kbd, "Keyboard Input", kbd_items());

// Label element
macro_rules! label_items {
    ( $el:literal ) => {
        attribute!($el, r#for, "for");
        // FIXME: content
    };
}
element!("label", Label, "Label", label_items());

// Legend element
macro_rules! legend_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("legend", Legend, "Field Set Legend", legend_items());

// Li element
macro_rules! li_items {
    ( $el:literal ) => {
        attribute!($el, value);
        // FIXME: content
    };
}
element!("li", Li, "List Item", li_items());

// Link element
macro_rules! link_items {
    ( $el:literal ) => {
        attribute!($el, r#as, "as");
        attribute!($el, blocking);
        attribute!($el, crossorigin);
        attribute!($el, disabled, true);
        attribute!($el, fetchpriority);
        attribute!($el, href);
        attribute!($el, hreflang);
        attribute!($el, imagesize);
        attribute!($el, imagesrcset);
        attribute!($el, integrity);
        attribute!($el, media);
        attribute!($el, referrerpolicy);
        attribute!($el, rel);
        attribute!($el, sizes);
        attribute!($el, r#type, "type");
        // no content (void)
    };
}
element!("link", Link, "External Resource Link", link_items());

// Main element
macro_rules! main_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("main", Main, "Main", main_items());

// Mark element
macro_rules! mark_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("mark", Mark, "Mark Text", mark_items());

// Map element
macro_rules! map_items {
    ( $el:literal ) => {
        attribute!($el, name);
        // FIXME: content
    };
}
element!("map", Map, "Image Map", map_items());

// Menu element
macro_rules! menu_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("menu", Menu, "Menu", menu_items());

// Meter element
macro_rules! meter_items {
    ( $el:literal ) => {
        attribute!($el, value);
        attribute!($el, min);
        attribute!($el, max);
        attribute!($el, low);
        attribute!($el, high);
        attribute!($el, optimum);
        // FIXME: content
    };
}
element!("meter", Meter, "Meter", meter_items());

// Meta element
macro_rules! meta_items {
    ( $el:literal ) => {
        attribute!($el, charset);
        attribute!($el, content);
        /* http_equiv, */
        attribute!($el, media);
        attribute!($el, name);
        // no content (void)
    };
}
element!("meta", Meta, "Metadata", meta_items());

// Nav element
macro_rules! nav_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("nav", Nav, "Navigation Section", nav_items());

// NoScript element
macro_rules! noscript_items {
    ( $el:literal ) => {
        // FIXME: content (complex rules)
    };
}
element!("noscript", NoScript, "NoScript", noscript_items());

// Object element
macro_rules! object_items {
    ( $el:literal ) => {
        attribute!($el, data);
        attribute!($el, form);
        attribute!($el, height);
        attribute!($el, name);
        attribute!($el, r#type, "type");
        attribute!($el, width);
        // FIXME: content
    };
}
element!("object", Object, "External Object", object_items());

// Ol element
macro_rules! ol_items {
    ( $el:literal ) => {
        attribute!($el, reversed, true);
        attribute!($el, start);
        attribute!($el, r#type, "type");
        // FIXME: content
    };
}
element!("ol", Ol, "Ordered List", ol_items());

// OptGroup element
macro_rules! optgroup_items {
    ( $el:literal ) => {
        attribute!($el, disabled, true);
        attribute!($el, label);
        // FIXME: content
    };
}
element!("optgroup", OptGroup, "Option Group", optgroup_items());

// Option element
macro_rules! option_items {
    ( $el:literal ) => {
        attribute!($el, disabled, true);
        attribute!($el, label);
        attribute!($el, selected, true);
        attribute!($el, value);
        // FIXME: content
    };
}
element!("option", Option, "Option", option_items());

// Output element
macro_rules! output_items {
    ( $el:literal ) => {
        attribute!($el, r#for, "for");
        attribute!($el, form);
        attribute!($el, name);
        // FIXME: content
    };
}
element!("output", Output, "Output", output_items());

// P element
macro_rules! p_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("p", P, "Paragraph", p_items());

// Picture element
macro_rules! picture_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("picture", Picture, "Picture", picture_items());

// Pre element
macro_rules! pre_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("pre", Pre, "Preformatted Text", pre_items());

// Progress element
macro_rules! progress_items {
    ( $el:literal ) => {
        attribute!($el, max);
        attribute!($el, value);
        // FIXME: content
    };
}
element!("progress", Progress, "Progress Indicator", progress_items());

// Q element
macro_rules! q_items {
    ( $el:literal ) => {
        attribute!($el, cite);
        // FIXME: content
    };
}
element!("q", Q, "Inline Quotation", q_items());

// Rp element
macro_rules! rp_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("rp", Rp, "Ruby Fallback Parenthesis", rp_items());

// Rt element
macro_rules! rt_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("rt", Rt, "Ruby Text", rt_items());

// Ruby element
macro_rules! ruby_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("ruby", Ruby, "Ruby Annotation", ruby_items());

// S element
macro_rules! s_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("s", S, "Strikethrough", s_items());

// Samp element
macro_rules! samp_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("samp", Samp, "Sample Output", samp_items());

// Script element
macro_rules! script_items {
    ( $el:literal ) => {
        attribute!($el, r#async, "async", true);
        attribute!($el, blocking);
        attribute!($el, crossorigin);
        attribute!($el, defer, true);
        attribute!($el, fetchpriority);
        attribute!($el, integrity);
        attribute!($el, nomodule, true);
        attribute!($el, referrerpolicy);
        attribute!($el, src);
        attribute!($el, r#type, "type");
        // FIXME: content
    };
}
element!("script", Script, "Script", script_items());

// Search element
macro_rules! search_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("search", Search, "Search", search_items());

// Section element
macro_rules! section_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("section", Section, "Section", section_items());

// Select element
macro_rules! select_items {
    ( $el:literal ) => {
        attribute!($el, autocomplete);
        attribute!($el, disabled, true);
        attribute!($el, form);
        attribute!($el, multiple, true);
        attribute!($el, name);
        attribute!($el, required, true);
        attribute!($el, size);
    };
}
element!("select", Select, "Select", select_items());

// Selectedcontent element
// TODO

// Slot element
macro_rules! slot_items {
    ( $el:literal ) => {
        attribute!($el, name);
        // FIXME: content
    };
}
element!("slot", Slot, "Web Component Slot", slot_items());

// Small element
macro_rules! small_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("small", Small, "Side Comment (Small)", small_items());

// Source element
macro_rules! source_items {
    ( $el:literal ) => {
        attribute!($el, r#type, "type");
        attribute!($el, src);
        attribute!($el, srcset);
        attribute!($el, sizes);
        attribute!($el, media);
        attribute!($el, height);
        attribute!($el, width);
    };
}
element!("source", Source, "Media or Image Source", source_items());

// Span element
macro_rules! span_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("span", Span, "Content Span", span_items());

// Strong element
macro_rules! strong_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("strong", Strong, "Strong Importance", strong_items());

// Style element
macro_rules! style_items {
    ( $el:literal ) => {
        attribute!($el, blocking);
        attribute!($el, media);
        // FIXME: allow `text/css` content only
    };
}
element!("style", Style, "Style Information", style_items());

// Sub element
macro_rules! sub_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("sub", Sub, "Subscript", sub_items());

// Summary element
macro_rules! summary_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("summary", Summary, "Disclosure Summary", summary_items());

// Sup element
macro_rules! sup_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("sup", Sup, "Superscript", sup_items());

// TBody element
macro_rules! tbody_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("tbody", TBody, "Table Body", tbody_items());

// TFoot element
macro_rules! tfoot_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("tfoot", TFoot, "Table Foot", tfoot_items());

// THead element
macro_rules! thead_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("thead", THead, "Table Head", thead_items());

// Table element
macro_rules! table_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("table", Table, "Table", table_items());

// Td element
macro_rules! td_items {
    ( $el:literal ) => {
        attribute!($el, colspan);
        attribute!($el, headers);
        attribute!($el, rowspan);
        // FIXME: content
    };
}
element!("td", Td, "Table Data Cell", td_items());

// Template element
macro_rules! template_items {
    ( $el:literal ) => {
        attribute!($el, shadowrootmode);
        attribute!($el, shadowrootclonable);
        attribute!($el, shadowrootdelegatesfocus);
        attribute!($el, shadowrootserializable);
        // FIXME: is any content allowed?
    };
}
element!("template", Template, "Content Template", template_items());

// TextArea element
macro_rules! textarea_items {
    ( $el:literal ) => {
        attribute!($el, autocomplete);
        attribute!($el, cols);
        attribute!($el, dirname);
        attribute!($el, disabled, true);
        attribute!($el, form);
        attribute!($el, maxlength);
        attribute!($el, minlength);
        attribute!($el, name);
        attribute!($el, placeholder);
        attribute!($el, readonly, true);
        attribute!($el, required, true);
        attribute!($el, rows);
        attribute!($el, wrap);
        // FIXME: content
    };
}
element!("textarea", TextArea, "Text Area", textarea_items());

// Th element
macro_rules! th_items {
    ( $el:literal ) => {
        attribute!($el, abbr);
        attribute!($el, colspan);
        attribute!($el, headers);
        attribute!($el, rowspan);
        attribute!($el, scope);
        // FIXME: content
    };
}
element!("th", Th, "Table Header", th_items());

// Time element
macro_rules! time_items {
    ( $el:literal ) => {
        attribute!($el, datetime);
        // FIXME: content
    };
}
element!("time", Time, "Time / Date", time_items());

// Title element
macro_rules! title_items {
    ( $el:literal ) => {
        // FIXME: only text content allowed
    };
}
element!("title", Title, "Document Title", title_items());

// Tr element
macro_rules! tr_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("tr", Tr, "Table Row", tr_items());

// Track element
macro_rules! track_items {
    ( $el:literal ) => {
        attribute!($el, default);
        attribute!($el, kind);
        attribute!($el, label);
        attribute!($el, src);
        attribute!($el, srclang);
        // FIXME: content
    };
}
element!("track", Track, "Embed Text Track", track_items());

// U element
macro_rules! u_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("u", U, "Unarticulated Annotation (Underline)", u_items());

// Ul element
macro_rules! ul_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("ul", Ul, "Unordered List", ul_items());

// Var element
macro_rules! var_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("var", Var, "Variable", var_items());

// Video element
macro_rules! video_items {
    ( $el:literal ) => {
        attribute!($el, autoplay, true);
        attribute!($el, controls, true);
        attribute!($el, controlslist);
        attribute!($el, crossorigin);
        attribute!($el, disablepictureinpicture, true);
        attribute!($el, disableremoteplayback, true);
        attribute!($el, height);
        attribute!($el, r#loop, "loop", true);
        attribute!($el, muted, true);
        attribute!($el, playsinline, true);
        attribute!($el, poster);
        attribute!($el, preload);
        attribute!($el, src);
        attribute!($el, width);
        // FIXME: content
    };
}
element!("video", Video, "Embed Video", video_items());

// Wbr element
macro_rules! wbr_items {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("wbr", Wbr, "Line Break Opportunity", wbr_items());
