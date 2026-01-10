// html.rs
//
// Copyright (C) 2025-2026  Douglas P Lau
//
//! HTML Elements -- _HyperText Markup Language_
use crate::page::{ElemType, Element, Page};
use crate::svg::Svg;
use crate::value::Value;

// A element
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
        // FIXME: only transparent content (not interactive, or "a")
        text_content!();
    };
}
html_elem!("a", A, "Anchor", a_items());

// Abbr element
macro_rules! abbr_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("abbr", Abbr, "Abbreviation", abbr_items());

// Address element
macro_rules! address_items {
    ( $el:literal ) => {
        address_content!();
    };
}
html_elem!("address", Address, "Contact Address", address_items());

// Article element
macro_rules! article_items {
    ( $el:literal ) => {
        flow_content!(abbr, cite, form);
    };
}
html_elem!("article", Article, "Article Contents", article_items());

// Area element (void)
macro_rules! area_items {
    ( $el:literal ) => {
        html_attr!($el, alt);
        html_attr!($el, coords);
        html_attr!($el, download);
        html_attr!($el, href);
        /* interestfor */
        html_attr!($el, ping);
        html_attr!($el, referrerpolicy);
        html_attr!($el, rel);
        html_attr!($el, shape);
        html_attr!($el, target);
        // no content (void)
    };
}
html_elem!(
    "area",
    Area,
    "Image Map Area",
    area_items(),
    ElemType::HtmlVoid
);

// Aside element
macro_rules! aside_items {
    ( $el:literal ) => {
        flow_content!(abbr, cite, form);
    };
}
html_elem!("aside", Aside, "Aside", aside_items());

// Audio element
macro_rules! audio_items {
    ( $el:literal ) => {
        html_attr!($el, autoplay, true);
        html_attr!($el, controls, true);
        html_attr!($el, controlslist);
        html_attr!($el, crossorigin);
        html_attr!($el, disableremoteplayback, true);
        html_attr!($el, r#loop, "loop", true);
        html_attr!($el, preload);
        html_attr!($el, src);
        // NOTE: special content rules
        elem_method!(source, Source);
        elem_method!(track, Track);
        // FIXME: transparent content also allowed
        comment_raw_methods!();
    };
}
html_elem!("audio", Audio, "Embed Audio", audio_items());

// B element
macro_rules! b_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("b", B, "Bring Attention To (Bold)", b_items());

// Bdi element
macro_rules! bdi_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("bdi", Bdi, "Bidirectional Isolate", bdi_items());

// Bdo element
macro_rules! bdo_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("bdo", Bdo, "Bidirectional Override", bdo_items());

// BlockQuote element
macro_rules! blockquote_items {
    ( $el:literal ) => {
        html_attr!($el, cite);
        flow_content!(abbr, cite_el, form);
    };
}
html_elem!(
    "blockquote",
    BlockQuote,
    "Block Quotation",
    blockquote_items()
);

// Base element (void)
macro_rules! base_items {
    ( $el:literal ) => {
        html_attr!($el, href);
        html_attr!($el, target);
        // no content (void)
    };
}
html_elem!("base", Base, "Base URL", base_items(), ElemType::HtmlVoid);

// Body element
macro_rules! body_items {
    ( $el:literal ) => {
        // FIXME: event attributes; onafterprint, onbeforeprint...
        flow_content!(abbr, cite, form);
    };
}
html_elem!("body", Body, "Document Body", body_items());

// Line break element (void)
macro_rules! br_items {
    ( $el:literal ) => {
        // no content (void)
    };
}
html_elem!("br", Br, "Line Break", br_items(), ElemType::HtmlVoid);

// Button element
macro_rules! button_items {
    ( $el:literal ) => {
        html_attr!($el, command);
        html_attr!($el, commandfor);
        html_attr!($el, disabled, true);
        html_attr!($el, form);
        html_attr!($el, formaction);
        html_attr!($el, formenctype);
        html_attr!($el, formmethod);
        html_attr!($el, formnovalidate, true);
        html_attr!($el, formtarget);
        /* interestfor */
        html_attr!($el, name);
        html_attr!($el, popovertarget);
        html_attr!($el, popovertargetaction);
        html_attr!($el, r#type, "type");
        html_attr!($el, value);
        non_interactive_phrasing_content!();
    };
}
html_elem!("button", Button, "Button", button_items());

// Canvas element
macro_rules! canvas_items {
    ( $el:literal ) => {
        html_attr!($el, height);
        html_attr!($el, width);
        // FIXME: weird content rules
    };
}
html_elem!("canvas", Canvas, "Graphics Canvas", canvas_items());

// Caption element
macro_rules! caption_items {
    ( $el:literal ) => {
        flow_content!(abbr, cite, form);
    };
}
html_elem!("caption", Caption, "Table Caption", caption_items());

// Cite element
macro_rules! cite_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("cite", Cite, "Citation", cite_items());

// Code element
macro_rules! code_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("code", Code, "Inline Code", code_items());

// Col element (void)
macro_rules! col_items {
    ( $el:literal ) => {
        html_attr!($el, span);
        // no content (void)
    };
}
html_elem!("col", Col, "Table Column", col_items(), ElemType::HtmlVoid);

// ColGroup element
macro_rules! colgroup_items {
    ( $el:literal ) => {
        html_attr!($el, span);
    };
}
html_elem!("colgroup", ColGroup, "Table Column Group", colgroup_items());

// Data element
macro_rules! data_items {
    ( $el:literal ) => {
        html_attr!($el, value);
        phrasing_content!(cite);
    };
}
html_elem!("data", Data, "Data", data_items());

// DataList element
macro_rules! datalist_items {
    ( $el:literal ) => {
        // NOTE: supposedly, phrasing content is allowed instead of options,
        //       but why?  (just use raw if you really need this)
        elem_method!(option, Option);
        comment_raw_methods!();
    };
}
html_elem!("datalist", DataList, "Data List", datalist_items());

// Dd element
macro_rules! dd_items {
    ( $el:literal ) => {
        flow_content!(abbr, cite, form);
    };
}
html_elem!("dd", Dd, "Description Details", dd_items());

// Del element
macro_rules! del_items {
    ( $el:literal ) => {
        html_attr!($el, cite);
        html_attr!($el, datetime);
        // FIXME: transparent content
        text_content!();
    };
}
html_elem!("del", Del, "Deleted Text", del_items());

// Details element
macro_rules! details_items {
    ( $el:literal ) => {
        html_attr!($el, open, true);
        html_attr!($el, name);
        // NOTE: summary must be first child
        elem_method!(summary, Summary);
        flow_content!(abbr, cite, form);
    };
}
html_elem!("details", Details, "Details Disclosure", details_items());

// Dfn element
macro_rules! dfn_items {
    ( $el:literal ) => {
        // NOTE: no dfn descendants allowed!
        phrasing_content!(cite);
    };
}
html_elem!("dfn", Dfn, "Definition", dfn_items());

// Dialog element
macro_rules! dialog_items {
    ( $el:literal ) => {
        html_attr!($el, closedby);
        html_attr!($el, open, true);
        flow_content!(abbr, cite, form);
    };
}
html_elem!("dialog", Dialog, "Dialog", dialog_items());

// Div element
macro_rules! div_items {
    ( $el:literal ) => {
        flow_content!(abbr, cite, form);
    };
}
html_elem!("div", Div, "Content Division", div_items());

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
html_elem!("dl", Dl, "Description List", dl_items());

// Dt element
macro_rules! dt_items {
    ( $el:literal ) => {
        // FIXME: no header, footer, sectioning or heading descendants
        flow_content!(abbr, cite, form);
    };
}
html_elem!("dt", Dt, "Description Term", dt_items());

// Em element
macro_rules! em_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("em", Em, "Emphasis", em_items());

// Embed element (void)
macro_rules! embed_items {
    ( $el:literal ) => {
        html_attr!($el, height);
        html_attr!($el, src);
        html_attr!($el, r#type, "type");
        html_attr!($el, width);
        // no content (void)
    };
}
html_elem!(
    "embed",
    Embed,
    "Embed External Content",
    embed_items(),
    ElemType::HtmlVoid
);

// FeildSet element
macro_rules! fieldset_items {
    ( $el:literal ) => {
        html_attr!($el, disabled, true);
        html_attr!($el, form);
        html_attr!($el, name);
        // NOTE: legend optional, but must be first child
        elem_method!(legend, Legend);
        flow_content!(abbr, cite, form_el);
    };
}
html_elem!("fieldset", FieldSet, "Field Set", fieldset_items());

// FigCaption element
macro_rules! figcaption_items {
    ( $el:literal ) => {
        flow_content!(abbr, cite, form);
    };
}
html_elem!(
    "figcaption",
    FigCaption,
    "Figure Caption",
    figcaption_items()
);

// Figure element
macro_rules! figure_items {
    ( $el:literal ) => {
        elem_method!(figcaption, FigCaption);
        flow_content!(abbr, cite, form);
    };
}
html_elem!("figure", Figure, "Figure", figure_items());

// Footer element
macro_rules! footer_items {
    ( $el:literal ) => {
        // NOTE: descendant header/footer elements not allowed
        flow_content!(abbr, cite, form);
    };
}
html_elem!("footer", Footer, "Footer", footer_items());

// Form element
macro_rules! form_items {
    ( $el:literal ) => {
        html_attr!($el, accept_charset, "accept-charset");
        html_attr!($el, autocomplete);
        html_attr!($el, name);
        html_attr!($el, rel);
        html_attr!($el, action);
        html_attr!($el, enctype);
        html_attr!($el, method);
        html_attr!($el, novalidate, true);
        html_attr!($el, target);
        // FIXME: descendant form elements not allowed
        flow_content!(abbr, cite, form);
    };
}
html_elem!("form", Form, "Form", form_items());

// heading element
macro_rules! heading_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
// FIXME: MDN links broken; "Heading_Elements"
html_elem!("h1", H1, "Section Heading 1", heading_items());
html_elem!("h2", H2, "Section Heading 2", heading_items());
html_elem!("h3", H3, "Section Heading 3", heading_items());
html_elem!("h4", H4, "Section Heading 4", heading_items());
html_elem!("h5", H5, "Section Heading 5", heading_items());
html_elem!("h6", H6, "Section Heading 6", heading_items());

// Head element
macro_rules! head_items {
    ( $el:literal ) => {
        metadata_content!();
    };
}
html_elem!("head", Head, "Header / Document Metadata", head_items());

// Header element
macro_rules! header_items {
    ( $el:literal ) => {
        // NOTE: descendant header/footer elements not allowed
        flow_content!(abbr, cite, form);
    };
}
html_elem!("header", Header, "Header", header_items());

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
html_elem!("hgroup", HGroup, "Heading Group", hgroup_items());

// Hr element (void)
macro_rules! hr_items {
    ( $el:literal ) => {
        // no content (void)
    };
}
html_elem!("hr", Hr, "Horizontal Rule", hr_items(), ElemType::HtmlVoid);

// Html element
macro_rules! html_items {
    ( $el:literal ) => {
        elem_method!(head, Head);
        elem_method!(body, Body);
        comment_raw_methods!();
    };
}
html_elem!("html", Html, "HTML Document Root", html_items());

// I element
macro_rules! i_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("i", I, "Idiomatic Text (Italic)", i_items());

// Iframe element
macro_rules! iframe_items {
    ( $el:literal ) => {
        html_attr!($el, allow);
        /* credentialless, csp */
        html_attr!($el, height);
        html_attr!($el, loading);
        html_attr!($el, name);
        /* privatetoken */
        html_attr!($el, referrerpolicy);
        html_attr!($el, sandbox);
        html_attr!($el, src);
        html_attr!($el, srcdoc);
        html_attr!($el, width);
        // no content allowed (nested browsing context)
    };
}
html_elem!("iframe", IFrame, "Inline Frame", iframe_items());

// Img element (void)
macro_rules! img_items {
    ( $el:literal ) => {
        html_attr!($el, alt);
        html_attr!($el, crossorigin);
        html_attr!($el, decoding);
        html_attr!($el, elementtiming);
        html_attr!($el, fetchpriority);
        html_attr!($el, height);
        html_attr!($el, ismap, true);
        html_attr!($el, loading);
        html_attr!($el, referrerpolicy);
        html_attr!($el, sizes);
        html_attr!($el, src);
        html_attr!($el, srcset);
        html_attr!($el, width);
        html_attr!($el, usemap);
        // no content (void)
    };
}
html_elem!(
    "img",
    Img,
    "Embedded Image",
    img_items(),
    ElemType::HtmlVoid
);

// Input element (void)
macro_rules! input_items {
    ( $el:literal ) => {
        html_attr!($el, accept);
        html_attr!($el, alpha);
        html_attr!($el, alt);
        html_attr!($el, autocomplete);
        html_attr!($el, capture);
        html_attr!($el, checked, true);
        html_attr!($el, colorspace);
        html_attr!($el, dirname);
        html_attr!($el, disabled, true);
        html_attr!($el, form);
        html_attr!($el, formaction);
        html_attr!($el, formenctype);
        html_attr!($el, formmethod);
        html_attr!($el, formnovalidate, true);
        html_attr!($el, formtarget);
        html_attr!($el, height);
        html_attr!($el, list);
        html_attr!($el, max);
        html_attr!($el, maxlength);
        html_attr!($el, min);
        html_attr!($el, minlength);
        html_attr!($el, multiple, true);
        html_attr!($el, name);
        html_attr!($el, pattern);
        html_attr!($el, placeholder);
        html_attr!($el, popovertarget);
        html_attr!($el, popovertargetaction);
        html_attr!($el, readonly, true);
        html_attr!($el, required, true);
        html_attr!($el, size);
        html_attr!($el, src);
        html_attr!($el, step);
        html_attr!($el, r#type, "type");
        html_attr!($el, value);
        html_attr!($el, width);
        // no content (void)
    };
}
html_elem!("input", Input, "Input", input_items(), ElemType::HtmlVoid);

// Ins element
macro_rules! ins_items {
    ( $el:literal ) => {
        html_attr!($el, cite);
        html_attr!($el, datetime);
        // FIXME: transparent content
        text_content!();
    };
}
html_elem!("ins", Ins, "Inserted Text", ins_items());

// Kbd element
macro_rules! kbd_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("kbd", Kbd, "Keyboard Input", kbd_items());

// Label element
macro_rules! label_items {
    ( $el:literal ) => {
        html_attr!($el, r#for, "for");
        // NOTE: no descendant label, etc.
        phrasing_content!(cite);
    };
}
html_elem!("label", Label, "Label", label_items());

// Legend element
macro_rules! legend_items {
    ( $el:literal ) => {
        elem_method!(h1, H1);
        elem_method!(h2, H2);
        elem_method!(h3, H3);
        elem_method!(h4, H4);
        elem_method!(h5, H5);
        elem_method!(h6, H6);
        phrasing_content!(cite);
    };
}
html_elem!("legend", Legend, "Field Set Legend", legend_items());

// Li element
macro_rules! li_items {
    ( $el:literal ) => {
        html_attr!($el, value);
        flow_content!(abbr, cite, form);
    };
}
html_elem!("li", Li, "List Item", li_items());

// Link element (void)
macro_rules! link_items {
    ( $el:literal ) => {
        html_attr!($el, r#as, "as");
        html_attr!($el, blocking);
        html_attr!($el, crossorigin);
        html_attr!($el, disabled, true);
        html_attr!($el, fetchpriority);
        html_attr!($el, href);
        html_attr!($el, hreflang);
        html_attr!($el, imagesize);
        html_attr!($el, imagesrcset);
        html_attr!($el, integrity);
        html_attr!($el, media);
        html_attr!($el, referrerpolicy);
        html_attr!($el, rel);
        html_attr!($el, sizes);
        html_attr!($el, r#type, "type");
        // NOTE: needed for standalone XML (SVG)
        html_attr!($el, xmlns);
        // no content (void)
    };
}
html_elem!(
    "link",
    Link,
    "External Resource Link",
    link_items(),
    ElemType::Xml
);

// Main element
macro_rules! main_items {
    ( $el:literal ) => {
        flow_content!(abbr, cite, form);
    };
}
html_elem!("main", Main, "Main", main_items());

// Mark element
macro_rules! mark_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("mark", Mark, "Mark Text", mark_items());

// Map element
macro_rules! map_items {
    ( $el:literal ) => {
        html_attr!($el, name);
        // FIXME: transparent content
    };
}
html_elem!("map", Map, "Image Map", map_items());

// Menu element
macro_rules! menu_items {
    ( $el:literal ) => {
        elem_method!(li, Li);
        elem_method!(script, Script);
        elem_method!(template, Template);
        comment_raw_methods!();
    };
}
html_elem!("menu", Menu, "Menu", menu_items());

// Meta element (void)
macro_rules! meta_items {
    ( $el:literal ) => {
        html_attr!($el, charset);
        html_attr!($el, content);
        /* http_equiv, */
        html_attr!($el, media);
        html_attr!($el, name);
        // no content (void)
    };
}
html_elem!("meta", Meta, "Metadata", meta_items(), ElemType::HtmlVoid);

// Meter element
macro_rules! meter_items {
    ( $el:literal ) => {
        html_attr!($el, value);
        html_attr!($el, min);
        html_attr!($el, max);
        html_attr!($el, low);
        html_attr!($el, high);
        html_attr!($el, optimum);
        // NOTE: no meter descendants allowed!
        phrasing_content!(cite);
    };
}
html_elem!("meter", Meter, "Meter", meter_items());

// Nav element
macro_rules! nav_items {
    ( $el:literal ) => {
        flow_content!(abbr, cite, form);
    };
}
html_elem!("nav", Nav, "Navigation Section", nav_items());

// NoScript element
macro_rules! noscript_items {
    ( $el:literal ) => {
        // FIXME: complex content rules (sometimes transparent)
        elem_method!(link, Link);
        elem_method!(style_el, Style); // global attr
        elem_method!(meta, Meta);
    };
}
html_elem!("noscript", NoScript, "NoScript", noscript_items());

// Object element
macro_rules! object_items {
    ( $el:literal ) => {
        html_attr!($el, data);
        html_attr!($el, form);
        html_attr!($el, height);
        html_attr!($el, name);
        html_attr!($el, r#type, "type");
        html_attr!($el, width);
        // FIXME: transparent content
    };
}
html_elem!("object", Object, "External Object", object_items());

// Ol element
macro_rules! ol_items {
    ( $el:literal ) => {
        html_attr!($el, reversed, true);
        html_attr!($el, start);
        html_attr!($el, r#type, "type");
        elem_method!(li, Li);
        elem_method!(script, Script);
        elem_method!(template, Template);
        comment_raw_methods!();
    };
}
html_elem!("ol", Ol, "Ordered List", ol_items());

// OptGroup element
macro_rules! optgroup_items {
    ( $el:literal ) => {
        html_attr!($el, disabled, true);
        html_attr!($el, label);
        // NOTE: legend permitted in customizable select elements
        elem_method!(option, Option);
        elem_method!(legend, Legend);
        comment_raw_methods!();
    };
}
html_elem!("optgroup", OptGroup, "Option Group", optgroup_items());

// Option element
macro_rules! option_items {
    ( $el:literal ) => {
        html_attr!($el, disabled, true);
        html_attr!($el, label);
        html_attr!($el, selected, true);
        html_attr!($el, value);
        // NOTE: more permitted in customizable select elements
        cdata_methods!();
        comment_raw_methods!();
    };
}
html_elem!("option", Option, "Option", option_items());

// Output element
macro_rules! output_items {
    ( $el:literal ) => {
        html_attr!($el, r#for, "for");
        html_attr!($el, form);
        html_attr!($el, name);
        phrasing_content!(cite);
    };
}
html_elem!("output", Output, "Output", output_items());

// P element
macro_rules! p_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("p", P, "Paragraph", p_items());

// Picture element
macro_rules! picture_items {
    ( $el:literal ) => {
        elem_method!(source, Source);
        elem_method!(img, Img);
        comment_raw_methods!();
    };
}
html_elem!("picture", Picture, "Picture", picture_items());

// Pre element
macro_rules! pre_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("pre", Pre, "Preformatted Text", pre_items());

// Progress element
macro_rules! progress_items {
    ( $el:literal ) => {
        html_attr!($el, max);
        html_attr!($el, value);
        // NOTE: no progress descendants allowed!
        phrasing_content!(cite);
    };
}
html_elem!("progress", Progress, "Progress Indicator", progress_items());

// Q element
macro_rules! q_items {
    ( $el:literal ) => {
        html_attr!($el, cite);
        phrasing_content!(cite_el);
    };
}
html_elem!("q", Q, "Inline Quotation", q_items());

// Rp element
macro_rules! rp_items {
    ( $el:literal ) => {
        text_content!();
    };
}
html_elem!("rp", Rp, "Ruby Fallback Parenthesis", rp_items());

// Rt element
macro_rules! rt_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("rt", Rt, "Ruby Text", rt_items());

// Ruby element
macro_rules! ruby_items {
    ( $el:literal ) => {
        // NOTE: content rules are complex
        elem_method!(rp, Rp);
        elem_method!(rt, Rt);
        phrasing_content!(cite);
    };
}
html_elem!("ruby", Ruby, "Ruby Annotation", ruby_items());

// S element
macro_rules! s_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("s", S, "Strikethrough", s_items());

// Samp element
macro_rules! samp_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("samp", Samp, "Sample Output", samp_items());

// Script element
macro_rules! script_items {
    ( $el:literal ) => {
        html_attr!($el, r#async, "async", true);
        html_attr!($el, blocking);
        html_attr!($el, crossorigin);
        html_attr!($el, defer, true);
        html_attr!($el, fetchpriority);
        html_attr!($el, integrity);
        html_attr!($el, nomodule, true);
        html_attr!($el, referrerpolicy);
        html_attr!($el, src);
        html_attr!($el, r#type, "type");
        cdata_methods!();
    };
}
html_elem!("script", Script, "Script", script_items());

// Search element
macro_rules! search_items {
    ( $el:literal ) => {
        flow_content!(abbr, cite, form);
    };
}
html_elem!("search", Search, "Search", search_items());

// Section element
macro_rules! section_items {
    ( $el:literal ) => {
        flow_content!(abbr, cite, form);
    };
}
html_elem!("section", Section, "Section", section_items());

// Select element
macro_rules! select_items {
    ( $el:literal ) => {
        html_attr!($el, autocomplete);
        html_attr!($el, disabled, true);
        html_attr!($el, form);
        html_attr!($el, multiple, true);
        html_attr!($el, name);
        html_attr!($el, required, true);
        html_attr!($el, size);
        // NOTE: more permitted in customizable select elements
        elem_method!(option, Option);
        elem_method!(optgroup, OptGroup);
        elem_method!(hr, Hr);
        comment_raw_methods!();
    };
}
html_elem!("select", Select, "Select", select_items());

// Selectedcontent element
// TODO

// Slot element
macro_rules! slot_items {
    ( $el:literal ) => {
        html_attr!($el, name);
        // FIXME: transparent content
        text_content!();
    };
}
html_elem!("slot", Slot, "Web Component Slot", slot_items());

// Small element
macro_rules! small_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("small", Small, "Side Comment (Small)", small_items());

// Source element (void)
macro_rules! source_items {
    ( $el:literal ) => {
        html_attr!($el, r#type, "type");
        html_attr!($el, src);
        html_attr!($el, srcset);
        html_attr!($el, sizes);
        html_attr!($el, media);
        html_attr!($el, height);
        html_attr!($el, width);
        // no content (void)
    };
}
html_elem!(
    "source",
    Source,
    "Media or Image Source",
    source_items(),
    ElemType::HtmlVoid
);

// Span element
macro_rules! span_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("span", Span, "Content Span", span_items());

// Strong element
macro_rules! strong_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("strong", Strong, "Strong Importance", strong_items());

// Style element
macro_rules! style_items {
    ( $el:literal ) => {
        html_attr!($el, blocking);
        html_attr!($el, media);
        // NOTE: `text/css` content only
        text_content!();
    };
}
html_elem!("style", Style, "Style Information", style_items());

// Sub element
macro_rules! sub_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("sub", Sub, "Subscript", sub_items());

// Summary element
macro_rules! summary_items {
    ( $el:literal ) => {
        // FIXME: also heading content (h1-h6 / hgroup)
        phrasing_content!(cite);
    };
}
html_elem!("summary", Summary, "Disclosure Summary", summary_items());

// Sup element
macro_rules! sup_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("sup", Sup, "Superscript", sup_items());

// TBody element
macro_rules! tbody_items {
    ( $el:literal ) => {
        elem_method!(tr, Tr);
        comment_raw_methods!();
    };
}
html_elem!("tbody", TBody, "Table Body", tbody_items());

// TFoot element
macro_rules! tfoot_items {
    ( $el:literal ) => {
        elem_method!(tr, Tr);
        comment_raw_methods!();
    };
}
html_elem!("tfoot", TFoot, "Table Foot", tfoot_items());

// THead element
macro_rules! thead_items {
    ( $el:literal ) => {
        elem_method!(tr, Tr);
        comment_raw_methods!();
    };
}
html_elem!("thead", THead, "Table Head", thead_items());

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
html_elem!("table", Table, "Table", table_items());

// Td element
macro_rules! td_items {
    ( $el:literal ) => {
        html_attr!($el, colspan);
        html_attr!($el, headers);
        html_attr!($el, rowspan);
        flow_content!(abbr, cite, form);
    };
}
html_elem!("td", Td, "Table Data Cell", td_items());

// Template element
macro_rules! template_items {
    ( $el:literal ) => {
        html_attr!($el, shadowrootmode);
        html_attr!($el, shadowrootclonable);
        html_attr!($el, shadowrootdelegatesfocus);
        html_attr!($el, shadowrootserializable);
        // NOTE: use raw to build template
        comment_raw_methods!();
    };
}
html_elem!("template", Template, "Content Template", template_items());

// TextArea element
macro_rules! textarea_items {
    ( $el:literal ) => {
        html_attr!($el, autocomplete);
        html_attr!($el, cols);
        html_attr!($el, dirname);
        html_attr!($el, disabled, true);
        html_attr!($el, form);
        html_attr!($el, maxlength);
        html_attr!($el, minlength);
        html_attr!($el, name);
        html_attr!($el, placeholder);
        html_attr!($el, readonly, true);
        html_attr!($el, required, true);
        html_attr!($el, rows);
        html_attr!($el, wrap);
        text_content!();
    };
}
html_elem!("textarea", TextArea, "Text Area", textarea_items());

// Th element
macro_rules! th_items {
    ( $el:literal ) => {
        html_attr!($el, abbr);
        html_attr!($el, colspan);
        html_attr!($el, headers);
        html_attr!($el, rowspan);
        html_attr!($el, scope);
        // NOTE: no header, footer, sectioning, or heading content
        flow_content!(abbr_el, cite, form);
    };
}
html_elem!("th", Th, "Table Header", th_items());

// Time element
macro_rules! time_items {
    ( $el:literal ) => {
        html_attr!($el, datetime);
        phrasing_content!(cite);
    };
}
html_elem!("time", Time, "Time / Date", time_items());

// Title element
macro_rules! title_content {
    ( $el:literal ) => {
        text_content!();
    };
}
html_elem!("title", Title, "Document Title", title_content());

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
html_elem!("tr", Tr, "Table Row", tr_items());

// Track element (void)
macro_rules! track_items {
    ( $el:literal ) => {
        html_attr!($el, default);
        html_attr!($el, kind);
        html_attr!($el, label);
        html_attr!($el, src);
        html_attr!($el, srclang);
        // no content (void)
    };
}
html_elem!(
    "track",
    Track,
    "Embed Text Track",
    track_items(),
    ElemType::HtmlVoid
);

// U element
macro_rules! u_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("u", U, "Unarticulated Annotation (Underline)", u_items());

// Ul element
macro_rules! ul_items {
    ( $el:literal ) => {
        elem_method!(li, Li);
        elem_method!(script, Script);
        elem_method!(template, Template);
        comment_raw_methods!();
    };
}
html_elem!("ul", Ul, "Unordered List", ul_items());

// Var element
macro_rules! var_items {
    ( $el:literal ) => {
        phrasing_content!(cite);
    };
}
html_elem!("var", Var, "Variable", var_items());

// Video element
macro_rules! video_items {
    ( $el:literal ) => {
        html_attr!($el, autoplay, true);
        html_attr!($el, controls, true);
        html_attr!($el, controlslist);
        html_attr!($el, crossorigin);
        html_attr!($el, disablepictureinpicture, true);
        html_attr!($el, disableremoteplayback, true);
        html_attr!($el, height);
        html_attr!($el, r#loop, "loop", true);
        html_attr!($el, muted, true);
        html_attr!($el, playsinline, true);
        html_attr!($el, poster);
        html_attr!($el, preload);
        html_attr!($el, src);
        html_attr!($el, width);
        // NOTE: special content rules
        elem_method!(source, Source);
        elem_method!(track, Track);
        // FIXME: transparent content also allowed
        comment_raw_methods!();
    };
}
html_elem!("video", Video, "Embed Video", video_items());

// Wbr element (void)
macro_rules! wbr_items {
    ( $el:literal ) => {
        // no content (void)
    };
}
html_elem!(
    "wbr",
    Wbr,
    "Line Break Opportunity",
    wbr_items(),
    ElemType::HtmlVoid
);
