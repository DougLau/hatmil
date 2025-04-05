// html.rs
//
// Copyright (C) 2025  Douglas P Lau
//
use std::fmt::Display;

/// Get optional `str` reference
pub fn opt_ref(val: &Option<impl AsRef<str>>) -> &str {
    match val {
        Some(v) => v.as_ref(),
        None => "",
    }
}

/// Get optional String
pub fn opt_str(val: Option<impl Display>) -> String {
    match val {
        Some(v) => v.to_string(),
        None => String::new(),
    }
}

/// Simple HTML builder
///
/// Elements can be created using methods with a matching name, such as
/// [a](#method.a), [body](#method.body), [div](#method.div),
/// [html](#method.html), and [ol](#method.ol).
/// These methods return an [Elem](struct.Elem.html), which borrows
/// from the `Html`, and can be closed with the [end](#method.end) method.
/// [Void] elements, like [img](#method.img) and [input](#method.input),
/// do not need to be closed.
///
/// Text content can be added using the [text](#method.text) or
/// [text_len](#method.text_len) methods.  Characters such as `&`, `<`,
/// and `>` will automatically be escaped.  For content which has
/// already been escaped, use the [raw](#method.raw) method.
///
/// After creating all elements, use [build](#method.build) to get the
/// HTML as a `String`.
///
/// [Void]: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
#[derive(Default)]
pub struct Html {
    html: String,
    stack: Vec<&'static str>,
}

/// Borrowed HTML element
pub struct Elem<'h> {
    html: &'h mut Html,
}

/// Borrowed HTML Void element
pub struct VoidElem<'h> {
    html: &'h mut Html,
}

impl Html {
    /// Create an HTML builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the HTML into a `String`
    pub fn build(mut self) -> String {
        while let Some(elem) = self.stack.pop() {
            self.html.push_str("</");
            self.html.push_str(elem);
            self.html.push('>');
        }
        self.html
    }

    /// Add an element
    fn elem(&mut self, elem: &'static str) -> Elem {
        self.html.push('<');
        self.html.push_str(elem);
        self.html.push('>');
        self.stack.push(elem);
        Elem { html: self }
    }

    /// Add a Void element
    fn void_elem(&mut self, elem: &'static str) -> VoidElem {
        self.html.push('<');
        self.html.push_str(elem);
        self.html.push('>');
        VoidElem { html: self }
    }

    /// Add an attribute with value
    fn attr(&mut self, attr: &'static str, val: impl AsRef<str>) {
        match self.html.pop() {
            Some(gt) => assert_eq!(gt, '>'),
            None => unreachable!(),
        }
        self.html.push(' ');
        self.html.push_str(attr);
        self.html.push_str("=\"");
        for c in val.as_ref().chars() {
            match c {
                '&' => self.html.push_str("&amp;"),
                '"' => self.html.push_str("&quot;"),
                _ => self.html.push(c),
            }
        }
        self.html.push_str("\">");
    }

    /// Add a Boolean attribute
    fn attr_bool(&mut self, attr: &'static str) {
        match self.html.pop() {
            Some(gt) => assert_eq!(gt, '>'),
            None => unreachable!(),
        }
        self.html.push(' ');
        self.html.push_str(attr);
        self.html.push('>');
    }

    /// Add text content
    pub fn text(&mut self, text: impl AsRef<str>) -> &mut Self {
        self.text_len(text, usize::MAX)
    }

    /// Add text content with a character limit
    pub fn text_len(&mut self, text: impl AsRef<str>, len: usize) -> &mut Self {
        for c in text.as_ref().chars().take(len) {
            match c {
                '&' => self.html.push_str("&amp;"),
                '<' => self.html.push_str("&lt;"),
                '>' => self.html.push_str("&gt;"),
                _ => self.html.push(c),
            }
        }
        self
    }

    /// Add raw content
    pub fn raw(&mut self, text: impl AsRef<str>) -> &mut Self {
        self.html.push_str(text.as_ref());
        self
    }

    /// End the current element
    pub fn end(&mut self) -> &mut Self {
        if let Some(elem) = self.stack.pop() {
            self.html.push_str("</");
            self.html.push_str(elem);
            self.html.push('>');
        }
        self
    }
}

impl<'h> Elem<'h> {
    /// Add an attribute with value
    pub fn attr(self, attr: &'static str, val: impl AsRef<str>) -> Self {
        self.html.attr(attr, val);
        self
    }

    /// Add a Boolean attribute
    pub fn attr_bool(self, attr: &'static str) -> Self {
        self.html.attr_bool(attr);
        self
    }

    /// Add a `type` attribute
    pub fn type_(self, val: impl AsRef<str>) -> Self {
        self.html.attr("type", val);
        self
    }

    /// Add a `for` attribute
    pub fn for_(self, val: impl AsRef<str>) -> Self {
        self.html.attr("for", val);
        self
    }

    /// Add text content
    pub fn text(self, text: impl AsRef<str>) -> &'h mut Html {
        self.html.text_len(text, usize::MAX)
    }

    /// Add text content with a character limit
    pub fn text_len(self, text: impl AsRef<str>, len: usize) -> &'h mut Html {
        self.html.text_len(text, len)
    }

    /// End the element
    pub fn end(self) -> &'h mut Html {
        self.html.end()
    }
}

impl<'h> VoidElem<'h> {
    /// Add an attribute with value
    pub fn attr(self, attr: &'static str, val: impl AsRef<str>) -> Self {
        self.html.attr(attr, val);
        self
    }

    /// Add a Boolean attribute
    pub fn attr_bool(self, attr: &'static str) -> Self {
        self.html.attr_bool(attr);
        self
    }

    /// Add a `type` attribute
    pub fn type_(self, val: impl AsRef<str>) -> Self {
        self.html.attr("type", val);
        self
    }

    /// Add a `for` attribute
    pub fn for_(self, val: impl AsRef<str>) -> Self {
        self.html.attr("for", val);
        self
    }

    /// Add text content
    pub fn text(self, text: impl AsRef<str>) -> &'h mut Html {
        self.html.text_len(text, usize::MAX)
    }

    /// Add text content with a character limit
    pub fn text_len(self, text: impl AsRef<str>, len: usize) -> &'h mut Html {
        self.html.text_len(text, len)
    }
}

/// HTML element helper
macro_rules! elements {
    ( $( $elem:ident ),* ) => {
        impl Html {
            $(
                #[doc = concat!("Add [", stringify!($elem), "](")]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($elem))]
                #[doc = ") element"]
                pub fn $elem(&mut self) -> Elem {
                    self.elem(stringify!($elem))
                }
            )*
        }
        impl<'h> Elem<'h> {
            $(
                #[doc = concat!("Add [", stringify!($elem), "](")]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($elem))]
                #[doc = ") element"]
                pub fn $elem(self) -> Self {
                    self.html.elem(stringify!($elem))
                }
            )*
        }
    }
}

elements![
    a, abbr, address, article, aside, audio, b, bdi, bdo, blockquote, body, br,
    button, canvas, caption, cite, code, colgroup, data, datalist, dd, del,
    details, dfn, dialog, div, dl, dt, em, fieldset, figcaption, figure,
    footer, form, h1, h2, h3, h4, h5, h6, head, header, hgroup, html, i,
    iframe, ins, kbd, label, legend, li, main, map, mark, menu, meter, nav,
    noscript, object, ol, optgroup, option, output, p, picture, pre, progress,
    q, rp, rt, ruby, s, samp, script, search, section, select, slot, small,
    span, strong, style, sub, summary, sup, table, tbody, td, template,
    textarea, tfoot, th, thead, time, title, tr, u, ul, var, video
];

/// HTML Void element helper
macro_rules! void_elements {
    ( $( $elem:ident ),* ) => {
        impl Html {
            $(
                #[doc = concat!("Add [", stringify!($elem), "](")]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($elem))]
                #[doc = ") Void element"]
                pub fn $elem(&mut self) -> VoidElem {
                    self.void_elem(stringify!($elem))
                }
            )*
        }
        impl<'h> Elem<'h> {
            $(
                #[doc = concat!("Add [", stringify!($elem), "](")]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($elem))]
                #[doc = ") Void element"]
                pub fn $elem(self) -> VoidElem<'h> {
                    self.html.void_elem(stringify!($elem))
                }
            )*
        }
    }
}

void_elements![
    area, base, col, embed, hr, img, input, link, meta, source, track, wbr
];

/// HTML global attribute helper
macro_rules! global_attributes {
    ( $( $attr:ident ),* ) => {
        impl<'h> Elem<'h> {
            $(
                #[doc = concat!("Add `", stringify!($attr), "` ")]
                #[doc = "global [attribute]("]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/", stringify!($attr))]
                #[doc = ")"]
                pub fn $attr(self, val: impl AsRef<str>) -> Self {
                    self.html.attr(stringify!($attr), val);
                    self
                }
            )*
        }
        impl<'h> VoidElem<'h> {
            $(
                #[doc = concat!("Add `", stringify!($attr), "` ")]
                #[doc = "global [attribute]("]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/", stringify!($attr))]
                #[doc = ")"]
                pub fn $attr(self, val: impl AsRef<str>) -> Self {
                    self.html.attr(stringify!($attr), val);
                    self
                }
            )*
        }
    }
}

global_attributes![class, id];

/// HTML attribute helper
macro_rules! attributes {
    ( $( $attr:ident ),* ) => {
        impl<'h> Elem<'h> {
            $(
                #[doc = concat!("Add `", stringify!($attr), "` ")]
                #[doc = "[attribute]("]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/", stringify!($attr))]
                #[doc = ")"]
                pub fn $attr(self, val: impl AsRef<str>) -> Self {
                    self.html.attr(stringify!($attr), val);
                    self
                }
            )*
        }
        impl<'h> VoidElem<'h> {
            $(
                #[doc = concat!("Add `", stringify!($attr), "` ")]
                #[doc = "[attribute]("]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/", stringify!($attr))]
                #[doc = ")"]
                pub fn $attr(self, val: impl AsRef<str>) -> Self {
                    self.html.attr(stringify!($attr), val);
                    self
                }
            )*
        }
    }
}

attributes![height, href, max, min, size, src, value, width];

/// HTML Boolean attribute helper
macro_rules! boolean_attributes {
    ( $( $attr:ident ),* ) => {
        impl<'h> Elem<'h> {
            $(
                #[doc = concat!("Add `", stringify!($attr), "` ")]
                #[doc = "Boolean [attribute]("]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/", stringify!($attr))]
                #[doc = ")"]
                pub fn $attr(self) -> Self {
                    self.html.attr_bool(stringify!($attr));
                    self
                }
            )*
        }
        impl<'h> VoidElem<'h> {
            $(
                #[doc = concat!("Add `", stringify!($attr), "` ")]
                #[doc = "Boolean [attribute]("]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/", stringify!($attr))]
                #[doc = ")"]
                pub fn $attr(self) -> Self {
                    self.html.attr_bool(stringify!($attr));
                    self
                }
            )*
        }
    }
}

boolean_attributes![autofocus, checked, disabled, inert, readonly, required];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn html() {
        let mut html = Html::new();
        html.div();
        assert_eq!(html.build(), String::from("<div></div>"));
        let mut html = Html::new();
        html.div().id("test").attr_bool("spellcheck");
        assert_eq!(
            html.build(),
            String::from("<div id=\"test\" spellcheck></div>")
        );
        let mut html = Html::new();
        html.p().text("This is a paragraph");
        assert_eq!(html.build(), String::from("<p>This is a paragraph</p>"));
        let mut html = Html::new();
        html.em().text("You & I");
        assert_eq!(html.build(), String::from("<em>You &amp; I</em>"));
        let mut html = Html::new();
        html.div().span().text("Test").end().raw("&quot;");
        assert_eq!(
            html.build(),
            String::from("<div><span>Test</span>&quot;</div>")
        );
    }

    #[test]
    fn ol() {
        let mut html = Html::new();
        html.ol();
        html.li().class("cat").text("nori").end();
        html.li().class("cat").text("chashu");
        assert_eq!(
            html.build(),
            String::from(
                "<ol><li class=\"cat\">nori</li><li class=\"cat\">chashu</li></ol>"
            )
        );
    }

    #[test]
    fn void() {
        let mut html = Html::new();
        html.div().input().type_("text").text("Stuff");
        assert_eq!(
            html.build(),
            String::from("<div><input type=\"text\">Stuff</div>")
        );
    }
}
