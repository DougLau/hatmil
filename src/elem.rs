// elem.rs
//
// Copyright (C) 2025  Douglas P Lau
//
//! HTML Elements
use crate::html::Page;
use crate::value::Value;

/// Element borrowed from a [Page]
pub trait Element<'p> {
    /// End the element
    ///
    /// Adds the closing tag (e.g. `</span>`).
    fn end(self) -> &'p mut Page;
}

/// Create an HTML element
macro_rules! element {
    ( $el:literal, $elem:ident, $desc:literal, $attr:ident() ) => {
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

        #[doc = concat!("`<", $el, ">` attributes")]
        impl<'p> $elem<'p> {
            $attr!( $el );
        }

        #[doc = "Global attributes"]
        impl<'p> $elem<'p> {
            global_attributes!();
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
macro_rules! a_content {
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
element!("a", A, "Anchor", a_content());

// Abbr element
macro_rules! abbr_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("abbr", Abbr, "Abbreviation", abbr_content());

// Address element
macro_rules! address_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("address", Address, "Contact Address", address_content());

// Article element
macro_rules! article_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("article", Article, "Article Contents", article_content());

// Area element (void)
macro_rules! area_content {
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
element!("area", Area, "Image Map Area", area_content());

// Aside element
macro_rules! aside_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("aside", Aside, "Aside", aside_content());

// Audio element
macro_rules! audio_content {
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
element!("audio", Audio, "Embed Audio", audio_content());

// B element
macro_rules! b_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("b", B, "Bring Attention To (Bold)", b_content());

// Bdi element
macro_rules! bdi_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("bdi", Bdi, "Bidirectional Isolate", bdi_content());

// Bdo element
macro_rules! bdo_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("bdo", Bdo, "Bidirectional Override", bdo_content());

// BlockQuote element
macro_rules! blockquote_content {
    ( $el:literal ) => {
        attribute!($el, cite);
        // FIXME: content
    };
}
element!(
    "blockquote",
    BlockQuote,
    "Block Quotation",
    blockquote_content()
);

// Base element (void)
macro_rules! base_content {
    ( $el:literal ) => {
        attribute!($el, href);
        attribute!($el, target);
        // FIXME: content
    };
}
element!("base", Base, "Base URL", base_content());

// Body element
macro_rules! body_content {
    ( $el:literal ) => {
        // FIXME: event attributes; onafterprint, onbeforeprint...
        // FIXME: content
    };
}
element!("body", Body, "Document Body", body_content());

// Line break element (void)
macro_rules! br_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("br", Br, "Line Break", br_content());

// Button element
macro_rules! button_content {
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
element!("button", Button, "Button", button_content());

// Canvas element
macro_rules! canvas_content {
    ( $el:literal ) => {
        attribute!($el, height);
        attribute!($el, width);
        // FIXME: content
    };
}
element!("canvas", Canvas, "Graphics Canvas", canvas_content());

// Caption element
macro_rules! caption_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("caption", Caption, "Table Caption", caption_content());

// Cite element
macro_rules! cite_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("cite", Cite, "Citation", cite_content());

// Code element
macro_rules! code_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("code", Code, "Inline Code", code_content());

// Col element (void)
macro_rules! col_content {
    ( $el:literal ) => {
        attribute!($el, span);
        // FIXME: content
    };
}
element!("col", Col, "Table Column", col_content());

// ColGroup element
macro_rules! colgroup_content {
    ( $el:literal ) => {
        attribute!($el, span);
    };
}
element!(
    "colgroup",
    ColGroup,
    "Table Column Group",
    colgroup_content()
);

// Data element
macro_rules! data_content {
    ( $el:literal ) => {
        attribute!($el, value);
        // FIXME: content
    };
}
element!("data", Data, "Data", data_content());

// DataList element
macro_rules! datalist_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("datalist", DataList, "Data List", datalist_content());

// Dd element
macro_rules! dd_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("dd", Dd, "Description Details", dd_content());

// Del element
macro_rules! del_content {
    ( $el:literal ) => {
        attribute!($el, cite);
        attribute!($el, datetime);
        // FIXME: content
    };
}
element!("del", Del, "Deleted Text", del_content());

// Details element
macro_rules! details_content {
    ( $el:literal ) => {
        attribute!($el, open, true);
        attribute!($el, name);
        // FIXME: content
    };
}
element!("details", Details, "Details Disclosure", details_content());

// Dfn element
macro_rules! dfn_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("dfn", Dfn, "Definition", dfn_content());

// Dialog element
macro_rules! dialog_content {
    ( $el:literal ) => {
        attribute!($el, closedby);
        attribute!($el, open, true);
        // FIXME: content
    };
}
element!("dialog", Dialog, "Dialog", dialog_content());

// Div element
macro_rules! div_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("div", Div, "Content Division", div_content());

// Dl element
macro_rules! dl_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("dl", Dl, "Description List", dl_content());

// Dt element
macro_rules! dt_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("dt", Dt, "Description Term", dt_content());

// Em element
macro_rules! em_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("em", Em, "Emphasis", em_content());

// Embed element (void)
macro_rules! embed_content {
    ( $el:literal ) => {
        attribute!($el, height);
        attribute!($el, src);
        attribute!($el, r#type, "type");
        attribute!($el, width);
        // FIXME: content
    };
}
element!("embed", Embed, "Embed External Content", embed_content());

// FeildSet element
macro_rules! fieldset_content {
    ( $el:literal ) => {
        attribute!($el, disabled, true);
        attribute!($el, form);
        attribute!($el, name);
        // FIXME: content
    };
}
element!("fieldset", FieldSet, "Field Set", fieldset_content());

// FigCaption element
macro_rules! figcaption_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!(
    "figcaption",
    FigCaption,
    "Figure Caption",
    figcaption_content()
);

// Figure element
macro_rules! figure_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("figure", Figure, "Figure", figure_content());

// Footer element
macro_rules! footer_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("footer", Footer, "Footer", footer_content());

// Form element
macro_rules! form_content {
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
element!("form", Form, "Form", form_content());

// h1 element
macro_rules! h1_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
// FIXME: MDN links broken; "Heading_Elements"
element!("h1", H1, "Section Heading 1", h1_content());

// h2 element
macro_rules! h2_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("h2", H2, "Section Heading 2", h2_content());

// h3 element
macro_rules! h3_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("h3", H3, "Section Heading 3", h3_content());

// h4 element
macro_rules! h4_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("h4", H4, "Section Heading 4", h4_content());

// h5 element
macro_rules! h5_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("h5", H5, "Section Heading 5", h5_content());

// h6 element
macro_rules! h6_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("h6", H6, "Section Heading 6", h6_content());

// Head element
macro_rules! head_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("head", Head, "Header / Document Metadata", head_content());

// Header element
macro_rules! header_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("header", Header, "Header", header_content());

// HGroup element
macro_rules! hgroup_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("hgroup", HGroup, "Heading Group", hgroup_content());

// Hr element
macro_rules! hr_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("hr", Hr, "Horizontal Rule", hr_content());

// Html element
macro_rules! html_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("html", Html, "HTML Document Root", html_content());

// I element
macro_rules! i_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("i", I, "Idiomatic Text (Italic)", i_content());

// Iframe element
macro_rules! iframe_content {
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
element!("iframe", IFrame, "Inline Frame", iframe_content());

// Img element
macro_rules! img_content {
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
element!("img", Img, "Embedded Image", img_content());

// Input element
macro_rules! input_content {
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
element!("input", Input, "Input", input_content());

// Ins element
macro_rules! ins_content {
    ( $el:literal ) => {
        attribute!($el, cite);
        attribute!($el, datetime);
        // FIXME: content
    };
}
element!("ins", Ins, "Inserted Text", ins_content());

// Kbd element
macro_rules! kbd_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("kbd", Kbd, "Keyboard Input", kbd_content());

// Label element
macro_rules! label_content {
    ( $el:literal ) => {
        attribute!($el, r#for, "for");
        // FIXME: content
    };
}
element!("label", Label, "Label", label_content());

// Legend element
macro_rules! legend_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("legend", Legend, "Field Set Legend", legend_content());

// Li element
macro_rules! li_content {
    ( $el:literal ) => {
        attribute!($el, value);
        // FIXME: content
    };
}
element!("li", Li, "List Item", li_content());

// Link element
macro_rules! link_content {
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
        // FIXME: content
    };
}
element!("link", Link, "External Resource Link", link_content());

// Main element
macro_rules! main_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("main", Main, "Main", main_content());

// Mark element
macro_rules! mark_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("mark", Mark, "Mark Text", mark_content());

// Map element
macro_rules! map_content {
    ( $el:literal ) => {
        attribute!($el, name);
        // FIXME: content
    };
}
element!("map", Map, "Image Map", map_content());

// Menu element
macro_rules! menu_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("menu", Menu, "Menu", menu_content());

// Meter element
macro_rules! meter_content {
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
element!("meter", Meter, "Meter", meter_content());

// Meta element
macro_rules! meta_content {
    ( $el:literal ) => {
        attribute!($el, charset);
        attribute!($el, content);
        /* http_equiv, */
        attribute!($el, media);
        attribute!($el, name);
        // FIXME: content
    };
}
element!("meta", Meta, "Metadata", meta_content());

// Nav element
macro_rules! nav_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("nav", Nav, "Navigation Section", nav_content());

// NoScript element
macro_rules! noscript_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("noscript", NoScript, "NoScript", noscript_content());

// Object element
macro_rules! object_content {
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
element!("object", Object, "External Object", object_content());

// Ol element
macro_rules! ol_content {
    ( $el:literal ) => {
        attribute!($el, reversed, true);
        attribute!($el, start);
        attribute!($el, r#type, "type");
        // FIXME: content
    };
}
element!("ol", Ol, "Ordered List", ol_content());

// OptGroup element
macro_rules! optgroup_content {
    ( $el:literal ) => {
        attribute!($el, disabled, true);
        attribute!($el, label);
        // FIXME: content
    };
}
element!("optgroup", OptGroup, "Option Group", optgroup_content());

// Option element
macro_rules! option_content {
    ( $el:literal ) => {
        attribute!($el, disabled, true);
        attribute!($el, label);
        attribute!($el, selected, true);
        attribute!($el, value);
        // FIXME: content
    };
}
element!("option", Option, "Option", option_content());

// Output element
macro_rules! output_content {
    ( $el:literal ) => {
        attribute!($el, r#for, "for");
        attribute!($el, form);
        attribute!($el, name);
        // FIXME: content
    };
}
element!("output", Output, "Output", output_content());

// P element
macro_rules! p_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("p", P, "Paragraph", p_content());

// Picture element
macro_rules! picture_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("picture", Picture, "Picture", picture_content());

// Pre element
macro_rules! pre_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("pre", Pre, "Preformatted Text", pre_content());

// Progress element
macro_rules! progress_content {
    ( $el:literal ) => {
        attribute!($el, max);
        attribute!($el, value);
        // FIXME: content
    };
}
element!(
    "progress",
    Progress,
    "Progress Indicator",
    progress_content()
);

// Q element
macro_rules! q_content {
    ( $el:literal ) => {
        attribute!($el, cite);
        // FIXME: content
    };
}
element!("q", Q, "Inline Quotation", q_content());

// Rp element
macro_rules! rp_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("rp", Rp, "Ruby Fallback Parenthesis", rp_content());

// Rt element
macro_rules! rt_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("rt", Rt, "Ruby Text", rt_content());

// Ruby element
macro_rules! ruby_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("ruby", Ruby, "Ruby Annotation", ruby_content());

// S element
macro_rules! s_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("s", S, "Strikethrough", s_content());

// Samp element
macro_rules! samp_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("samp", Samp, "Sample Output", samp_content());

// Script element
macro_rules! script_content {
    ( $el:literal ) => {
        attribute!($el, r#async, "async", true);
        attribute!($el, blocking);
        attribute!($el, crossorigin);
        attribute!($el, defer, true);
        attribute!($el, fetchpriority);
        attribute!($el, integrity);
        attribute!($el, nomodule, true);
        attribute!($el, nonce);
        attribute!($el, referrerpolicy);
        attribute!($el, src);
        attribute!($el, r#type, "type");
        // FIXME: content
    };
}
element!("script", Script, "Script", script_content());

// Search element
macro_rules! search_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("search", Search, "Search", search_content());

// Section element
macro_rules! section_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("section", Section, "Section", section_content());

// Select element
macro_rules! select_content {
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
element!("select", Select, "Select", select_content());

// Selectedcontent element
// TODO

// Slot element
macro_rules! slot_content {
    ( $el:literal ) => {
        attribute!($el, name);
        // FIXME: content
    };
}
element!("slot", Slot, "Web Component Slot", slot_content());

// Small element
macro_rules! small_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("small", Small, "Side Comment (Small)", small_content());

// Source element
macro_rules! source_content {
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
element!("source", Source, "Media or Image Source", source_content());

// Span element
macro_rules! span_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("span", Span, "Content Span", span_content());

// Strong element
macro_rules! strong_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("strong", Strong, "Strong Importance", strong_content());

// Style element
macro_rules! style_content {
    ( $el:literal ) => {
        attribute!($el, blocking);
        attribute!($el, media);
        attribute!($el, nonce);
        attribute!($el, title);
    };
}
element!("style", Style, "Style Information", style_content());

// Sub element
macro_rules! sub_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("sub", Sub, "Subscript", sub_content());

// Summary element
macro_rules! summary_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("summary", Summary, "Disclosure Summary", summary_content());

// Sup element
macro_rules! sup_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("sup", Sup, "Superscript", sup_content());

// TBody element
macro_rules! tbody_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("tbody", TBody, "Table Body", tbody_content());

// TFoot element
macro_rules! tfoot_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("tfoot", TFoot, "Table Foot", tfoot_content());

// THead element
macro_rules! thead_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("thead", THead, "Table Head", thead_content());

// Table element
macro_rules! table_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("table", Table, "Table", table_content());

// Td element
macro_rules! td_content {
    ( $el:literal ) => {
        attribute!($el, colspan);
        attribute!($el, headers);
        attribute!($el, rowspan);
        // FIXME: content
    };
}
element!("td", Td, "Table Data Cell", td_content());

// Template element
macro_rules! template_content {
    ( $el:literal ) => {
        attribute!($el, shadowrootmode);
        attribute!($el, shadowrootclonable);
        attribute!($el, shadowrootdelegatesfocus);
        attribute!($el, shadowrootserializable);
    };
}
element!("template", Template, "Content Template", template_content());

// TextArea element
macro_rules! textarea_content {
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
element!("textarea", TextArea, "Text Area", textarea_content());

// Th element
macro_rules! th_content {
    ( $el:literal ) => {
        attribute!($el, abbr);
        attribute!($el, colspan);
        attribute!($el, headers);
        attribute!($el, rowspan);
        attribute!($el, scope);
        // FIXME: content
    };
}
element!("th", Th, "Table Header", th_content());

// Time element
macro_rules! time_content {
    ( $el:literal ) => {
        attribute!($el, datetime);
        // FIXME: content
    };
}
element!("time", Time, "Time / Date", time_content());

// Title element
macro_rules! title_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("title", Title, "Document Title", title_content());

// Tr element
macro_rules! tr_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("tr", Tr, "Table Row", tr_content());

// Track element
macro_rules! track_content {
    ( $el:literal ) => {
        attribute!($el, default);
        attribute!($el, kind);
        attribute!($el, label);
        attribute!($el, src);
        attribute!($el, srclang);
        // FIXME: content
    };
}
element!("track", Track, "Embed Text Track", track_content());

// U element
macro_rules! u_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("u", U, "Unarticulated Annotation (Underline)", u_content());

// Ul element
macro_rules! ul_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("ul", Ul, "Unordered List", ul_content());

// Var element
macro_rules! var_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("var", Var, "Variable", var_content());

// Video element
macro_rules! video_content {
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
element!("video", Video, "Embed Video", video_content());

// Wbr element
macro_rules! wbr_content {
    ( $el:literal ) => {
        // FIXME: content
    };
}
element!("wbr", Wbr, "Line Break Opportunity", wbr_content());
