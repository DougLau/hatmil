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
    fn end(&'p mut self) -> &'p mut Page;
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
        // FIXME: only transparent content (not interactive, or "a")
        text_content!();
    };
}
element!("a", A, "Anchor", a_items());

// Abbr element
macro_rules! abbr_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("abbr", Abbr, "Abbreviation", abbr_items());

// Address element
macro_rules! address_items {
    ( $el:literal ) => {
        address_content!();
    };
}
element!("address", Address, "Contact Address", address_items());

// Article element
macro_rules! article_items {
    ( $el:literal ) => {
        flow_content!();
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
        // no content (void)
    };
}
element!("area", Area, "Image Map Area", area_items());

// Aside element
macro_rules! aside_items {
    ( $el:literal ) => {
        flow_content!();
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
        // NOTE: special content rules
        elem_method!(source, Source);
        elem_method!(track, Track);
        // FIXME: transparent content also allowed
        comment_raw_methods!();
    };
}
element!("audio", Audio, "Embed Audio", audio_items());

// B element
macro_rules! b_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("b", B, "Bring Attention To (Bold)", b_items());

// Bdi element
macro_rules! bdi_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("bdi", Bdi, "Bidirectional Isolate", bdi_items());

// Bdo element
macro_rules! bdo_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("bdo", Bdo, "Bidirectional Override", bdo_items());

// BlockQuote element
macro_rules! blockquote_items {
    ( $el:literal ) => {
        attribute!($el, cite);
        flow_content!();
    };
}
element!(
    "blockquote",
    BlockQuote,
    "Block Quotation",
    blockquote_items()
);

// Base element (void)
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
        // no content (void)
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
        non_interactive_phrasing_content!();
    };
}
element!("button", Button, "Button", button_items());

// Canvas element
macro_rules! canvas_items {
    ( $el:literal ) => {
        attribute!($el, height);
        attribute!($el, width);
        // FIXME: weird content rules
    };
}
element!("canvas", Canvas, "Graphics Canvas", canvas_items());

// Caption element
macro_rules! caption_items {
    ( $el:literal ) => {
        flow_content!();
    };
}
element!("caption", Caption, "Table Caption", caption_items());

// Cite element
macro_rules! cite_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("cite", Cite, "Citation", cite_items());

// Code element
macro_rules! code_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("code", Code, "Inline Code", code_items());

// Col element (void)
macro_rules! col_items {
    ( $el:literal ) => {
        attribute!($el, span);
        // no content (void)
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
        phrasing_content!();
    };
}
element!("data", Data, "Data", data_items());

// DataList element
macro_rules! datalist_items {
    ( $el:literal ) => {
        // NOTE: supposedly, phrasing content is allowed instead of options,
        //       but why?  (just use raw if you really need this)
        elem_method!(option, Option);
        comment_raw_methods!();
    };
}
element!("datalist", DataList, "Data List", datalist_items());

// Dd element
macro_rules! dd_items {
    ( $el:literal ) => {
        flow_content!();
    };
}
element!("dd", Dd, "Description Details", dd_items());

// Del element
macro_rules! del_items {
    ( $el:literal ) => {
        attribute!($el, cite);
        attribute!($el, datetime);
        // FIXME: transparent content
        text_content!();
    };
}
element!("del", Del, "Deleted Text", del_items());

// Details element
macro_rules! details_items {
    ( $el:literal ) => {
        attribute!($el, open, true);
        attribute!($el, name);
        // NOTE: summary must be first child
        elem_method!(summary, Summary);
        flow_content!();
    };
}
element!("details", Details, "Details Disclosure", details_items());

// Dfn element
macro_rules! dfn_items {
    ( $el:literal ) => {
        // NOTE: no dfn descendants allowed!
        phrasing_content!();
    };
}
element!("dfn", Dfn, "Definition", dfn_items());

// Dialog element
macro_rules! dialog_items {
    ( $el:literal ) => {
        attribute!($el, closedby);
        attribute!($el, open, true);
        flow_content!();
    };
}
element!("dialog", Dialog, "Dialog", dialog_items());

// Div element
macro_rules! div_items {
    ( $el:literal ) => {
        flow_content!();
    };
}
element!("div", Div, "Content Division", div_items());

// Dl element
macro_rules! dl_items {
    ( $el:literal ) => {
        // NOTE: dt/dd elements must be in pairs
        elem_method!(dt, Dt);
        elem_method!(dd, Dd);
        elem_method!(script, Script);
        elem_method!(template, Template);
        comment_raw_methods!();
    };
}
element!("dl", Dl, "Description List", dl_items());

// Dt element
macro_rules! dt_items {
    ( $el:literal ) => {
        // FIXME: no header, footer, sectioning or heading descendants
        flow_content!();
    };
}
element!("dt", Dt, "Description Term", dt_items());

// Em element
macro_rules! em_items {
    ( $el:literal ) => {
        phrasing_content!();
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
        // no content (void)
    };
}
element!("embed", Embed, "Embed External Content", embed_items());

// FeildSet element
macro_rules! fieldset_items {
    ( $el:literal ) => {
        attribute!($el, disabled, true);
        attribute!($el, form);
        attribute!($el, name);
        // NOTE: legend optional, but must be first child
        elem_method!(legend, Legend);
        flow_content!();
    };
}
element!("fieldset", FieldSet, "Field Set", fieldset_items());

// FigCaption element
macro_rules! figcaption_items {
    ( $el:literal ) => {
        flow_content!();
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
        elem_method!(figcaption, FigCaption);
        flow_content!();
    };
}
element!("figure", Figure, "Figure", figure_items());

// Footer element
macro_rules! footer_items {
    ( $el:literal ) => {
        // NOTE: descendant header/footer elements not allowed
        flow_content!();
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
        // FIXME: descendant form elements not allowed
        flow_content!();
    };
}
element!("form", Form, "Form", form_items());

// heading element
macro_rules! heading_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
// FIXME: MDN links broken; "Heading_Elements"
element!("h1", H1, "Section Heading 1", heading_items());
element!("h2", H2, "Section Heading 2", heading_items());
element!("h3", H3, "Section Heading 3", heading_items());
element!("h4", H4, "Section Heading 4", heading_items());
element!("h5", H5, "Section Heading 5", heading_items());
element!("h6", H6, "Section Heading 6", heading_items());

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
        // NOTE: descendant header/footer elements not allowed
        flow_content!();
    };
}
element!("header", Header, "Header", header_items());

// HGroup element
macro_rules! hgroup_items {
    ( $el:literal ) => {
        // NOTE: there are special ordering rules for these
        elem_method!(h1, H1);
        elem_method!(h2, H2);
        elem_method!(h3, H3);
        elem_method!(h4, H4);
        elem_method!(h5, H5);
        elem_method!(h6, H6);
        elem_method!(p, P);
    };
}
element!("hgroup", HGroup, "Heading Group", hgroup_items());

// Hr element (void)
macro_rules! hr_items {
    ( $el:literal ) => {
        // no content (void)
    };
}
element!("hr", Hr, "Horizontal Rule", hr_items());

// Html element
macro_rules! html_items {
    ( $el:literal ) => {
        elem_method!(head, Head);
        elem_method!(body, Body);
        comment_raw_methods!();
    };
}
element!("html", Html, "HTML Document Root", html_items());

// I element
macro_rules! i_items {
    ( $el:literal ) => {
        phrasing_content!();
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
        // no content allowed (nested browsing context)
    };
}
element!("iframe", IFrame, "Inline Frame", iframe_items());

// Img element (void)
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
        // no content (void)
    };
}
element!("img", Img, "Embedded Image", img_items());

// Input element (void)
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
        // no content (void)
    };
}
element!("input", Input, "Input", input_items());

// Ins element
macro_rules! ins_items {
    ( $el:literal ) => {
        attribute!($el, cite);
        attribute!($el, datetime);
        // FIXME: transparent content
        text_content!();
    };
}
element!("ins", Ins, "Inserted Text", ins_items());

// Kbd element
macro_rules! kbd_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("kbd", Kbd, "Keyboard Input", kbd_items());

// Label element
macro_rules! label_items {
    ( $el:literal ) => {
        attribute!($el, r#for, "for");
        // NOTE: no descendant label, etc.
        phrasing_content!();
    };
}
element!("label", Label, "Label", label_items());

// Legend element
macro_rules! legend_items {
    ( $el:literal ) => {
        elem_method!(h1, H1);
        elem_method!(h2, H2);
        elem_method!(h3, H3);
        elem_method!(h4, H4);
        elem_method!(h5, H5);
        elem_method!(h6, H6);
        phrasing_content!();
    };
}
element!("legend", Legend, "Field Set Legend", legend_items());

// Li element
macro_rules! li_items {
    ( $el:literal ) => {
        attribute!($el, value);
        flow_content!();
    };
}
element!("li", Li, "List Item", li_items());

// Link element (void)
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
        flow_content!();
    };
}
element!("main", Main, "Main", main_items());

// Mark element
macro_rules! mark_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("mark", Mark, "Mark Text", mark_items());

// Map element
macro_rules! map_items {
    ( $el:literal ) => {
        attribute!($el, name);
        // FIXME: transparent content
    };
}
element!("map", Map, "Image Map", map_items());

// Menu element
macro_rules! menu_items {
    ( $el:literal ) => {
        elem_method!(li, Li);
        elem_method!(script, Script);
        elem_method!(template, Template);
        comment_raw_methods!();
    };
}
element!("menu", Menu, "Menu", menu_items());

// Meta element (void)
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

// Meter element
macro_rules! meter_items {
    ( $el:literal ) => {
        attribute!($el, value);
        attribute!($el, min);
        attribute!($el, max);
        attribute!($el, low);
        attribute!($el, high);
        attribute!($el, optimum);
        // NOTE: no meter descendants allowed!
        phrasing_content!();
    };
}
element!("meter", Meter, "Meter", meter_items());

// Nav element
macro_rules! nav_items {
    ( $el:literal ) => {
        flow_content!();
    };
}
element!("nav", Nav, "Navigation Section", nav_items());

// NoScript element
macro_rules! noscript_items {
    ( $el:literal ) => {
        // FIXME: complex content rules (sometimes transparent)
        elem_method!(link, Link);
        elem_method!(style_elem, Style, "style"); // global attr
        elem_method!(meta, Meta);
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
        // FIXME: transparent content
    };
}
element!("object", Object, "External Object", object_items());

// Ol element
macro_rules! ol_items {
    ( $el:literal ) => {
        attribute!($el, reversed, true);
        attribute!($el, start);
        attribute!($el, r#type, "type");
        elem_method!(li, Li);
        elem_method!(script, Script);
        elem_method!(template, Template);
        comment_raw_methods!();
    };
}
element!("ol", Ol, "Ordered List", ol_items());

// OptGroup element
macro_rules! optgroup_items {
    ( $el:literal ) => {
        attribute!($el, disabled, true);
        attribute!($el, label);
        // NOTE: legend permitted in customizable select elements
        elem_method!(option, Option);
        elem_method!(legend, Legend);
        comment_raw_methods!();
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
        // NOTE: more permitted in customizable select elements
        text_methods!();
        comment_raw_methods!();
    };
}
element!("option", Option, "Option", option_items());

// Output element
macro_rules! output_items {
    ( $el:literal ) => {
        attribute!($el, r#for, "for");
        attribute!($el, form);
        attribute!($el, name);
        phrasing_content!();
    };
}
element!("output", Output, "Output", output_items());

// P element
macro_rules! p_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("p", P, "Paragraph", p_items());

// Picture element
macro_rules! picture_items {
    ( $el:literal ) => {
        elem_method!(source, Source);
        elem_method!(img, Img);
        comment_raw_methods!();
    };
}
element!("picture", Picture, "Picture", picture_items());

// Pre element
macro_rules! pre_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("pre", Pre, "Preformatted Text", pre_items());

// Progress element
macro_rules! progress_items {
    ( $el:literal ) => {
        attribute!($el, max);
        attribute!($el, value);
        // NOTE: no progress descendants allowed!
        phrasing_content!();
    };
}
element!("progress", Progress, "Progress Indicator", progress_items());

// Q element
macro_rules! q_items {
    ( $el:literal ) => {
        attribute!($el, cite);
        phrasing_content!();
    };
}
element!("q", Q, "Inline Quotation", q_items());

// Rp element
macro_rules! rp_items {
    ( $el:literal ) => {
        text_content!();
    };
}
element!("rp", Rp, "Ruby Fallback Parenthesis", rp_items());

// Rt element
macro_rules! rt_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("rt", Rt, "Ruby Text", rt_items());

// Ruby element
macro_rules! ruby_items {
    ( $el:literal ) => {
        // NOTE: content rules are complex
        elem_method!(rp, Rp);
        elem_method!(rt, Rt);
        phrasing_content!();
    };
}
element!("ruby", Ruby, "Ruby Annotation", ruby_items());

// S element
macro_rules! s_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("s", S, "Strikethrough", s_items());

// Samp element
macro_rules! samp_items {
    ( $el:literal ) => {
        phrasing_content!();
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
        text_methods!();
    };
}
element!("script", Script, "Script", script_items());

// Search element
macro_rules! search_items {
    ( $el:literal ) => {
        flow_content!();
    };
}
element!("search", Search, "Search", search_items());

// Section element
macro_rules! section_items {
    ( $el:literal ) => {
        flow_content!();
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
        // NOTE: more permitted in customizable select elements
        elem_method!(option, Option);
        elem_method!(optgroup, OptGroup);
        elem_method!(hr, Hr);
        comment_raw_methods!();
    };
}
element!("select", Select, "Select", select_items());

// Selectedcontent element
// TODO

// Slot element
macro_rules! slot_items {
    ( $el:literal ) => {
        attribute!($el, name);
        // FIXME: transparent content
        text_content!();
    };
}
element!("slot", Slot, "Web Component Slot", slot_items());

// Small element
macro_rules! small_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("small", Small, "Side Comment (Small)", small_items());

// Source element (void)
macro_rules! source_items {
    ( $el:literal ) => {
        attribute!($el, r#type, "type");
        attribute!($el, src);
        attribute!($el, srcset);
        attribute!($el, sizes);
        attribute!($el, media);
        attribute!($el, height);
        attribute!($el, width);
        // no content (void)
    };
}
element!("source", Source, "Media or Image Source", source_items());

// Span element
macro_rules! span_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("span", Span, "Content Span", span_items());

// Strong element
macro_rules! strong_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("strong", Strong, "Strong Importance", strong_items());

// Style element
macro_rules! style_items {
    ( $el:literal ) => {
        attribute!($el, blocking);
        attribute!($el, media);
        // NOTE: `text/css` content only
        text_content!();
    };
}
element!("style", Style, "Style Information", style_items());

// Sub element
macro_rules! sub_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("sub", Sub, "Subscript", sub_items());

// Summary element
macro_rules! summary_items {
    ( $el:literal ) => {
        // FIXME: also heading content (h1-h6 / hgroup)
        phrasing_content!();
    };
}
element!("summary", Summary, "Disclosure Summary", summary_items());

// Sup element
macro_rules! sup_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("sup", Sup, "Superscript", sup_items());

// TBody element
macro_rules! tbody_items {
    ( $el:literal ) => {
        elem_method!(tr, Tr);
        comment_raw_methods!();
    };
}
element!("tbody", TBody, "Table Body", tbody_items());

// TFoot element
macro_rules! tfoot_items {
    ( $el:literal ) => {
        elem_method!(tr, Tr);
        comment_raw_methods!();
    };
}
element!("tfoot", TFoot, "Table Foot", tfoot_items());

// THead element
macro_rules! thead_items {
    ( $el:literal ) => {
        elem_method!(tr, Tr);
        comment_raw_methods!();
    };
}
element!("thead", THead, "Table Head", thead_items());

// Table element
macro_rules! table_items {
    ( $el:literal ) => {
        // NOTE: additional ordering rules apply!
        elem_method!(caption, Caption);
        elem_method!(colgroup, ColGroup);
        elem_method!(thead, THead);
        elem_method!(tbody, TBody);
        elem_method!(tr, Tr);
        elem_method!(tfoot, TFoot);
        comment_raw_methods!();
    };
}
element!("table", Table, "Table", table_items());

// Td element
macro_rules! td_items {
    ( $el:literal ) => {
        attribute!($el, colspan);
        attribute!($el, headers);
        attribute!($el, rowspan);
        flow_content!();
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
        // NOTE: use raw to build template
        comment_raw_methods!();
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
        text_content!();
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
        // NOTE: no header, footer, sectioning, or heading content
        flow_content!();
    };
}
element!("th", Th, "Table Header", th_items());

// Time element
macro_rules! time_items {
    ( $el:literal ) => {
        attribute!($el, datetime);
        phrasing_content!();
    };
}
element!("time", Time, "Time / Date", time_items());

// Title element
macro_rules! title_content {
    ( $el:literal ) => {
        text_content!();
    };
}
element!("title", Title, "Document Title", title_content());

// Tr element
macro_rules! tr_items {
    ( $el:literal ) => {
        elem_method!(td, Td);
        elem_method!(th, Th);
        elem_method!(script, Script);
        elem_method!(template, Template);
        comment_raw_methods!();
    };
}
element!("tr", Tr, "Table Row", tr_items());

// Track element (void)
macro_rules! track_items {
    ( $el:literal ) => {
        attribute!($el, default);
        attribute!($el, kind);
        attribute!($el, label);
        attribute!($el, src);
        attribute!($el, srclang);
        // no content (void)
    };
}
element!("track", Track, "Embed Text Track", track_items());

// U element
macro_rules! u_items {
    ( $el:literal ) => {
        phrasing_content!();
    };
}
element!("u", U, "Unarticulated Annotation (Underline)", u_items());

// Ul element
macro_rules! ul_items {
    ( $el:literal ) => {
        elem_method!(li, Li);
        elem_method!(script, Script);
        elem_method!(template, Template);
        comment_raw_methods!();
    };
}
element!("ul", Ul, "Unordered List", ul_items());

// Var element
macro_rules! var_items {
    ( $el:literal ) => {
        phrasing_content!();
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
        // NOTE: special content rules
        elem_method!(source, Source);
        elem_method!(track, Track);
        // FIXME: transparent content also allowed
        comment_raw_methods!();
    };
}
element!("video", Video, "Embed Video", video_items());

// Wbr element (void)
macro_rules! wbr_items {
    ( $el:literal ) => {
        // no content (void)
    };
}
element!("wbr", Wbr, "Line Break Opportunity", wbr_items());
