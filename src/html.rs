// html.rs
//
// Copyright (C) 2025  Douglas P Lau
//

/// Simple HTML builder
#[derive(Default)]
pub struct Html {
    html: String,
    stack: Vec<&'static str>,
}

/// Borrowed HTML element
pub struct Elem<'h> {
    html: &'h mut Html,
}

/// Borrowed HTML [Void] element
///
/// [Void]: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
pub struct VoidElem<'h> {
    html: &'h mut Html,
}

impl From<Html> for String {
    fn from(mut html: Html) -> Self {
        while let Some(elem) = html.stack.pop() {
            html.html.push_str("</");
            html.html.push_str(elem);
            html.html.push('>');
        }
        html.html
    }
}

impl Html {
    /// Create an HTML builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an HTML builder with a `doctype` preamble
    pub fn with_doctype() -> Self {
        let mut html = Html::default();
        html.raw("<!doctype html>");
        html
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
    ///
    /// The `text` will be truncated at `len` characters.
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
    ///
    /// **WARNING**: The text is used verbatim, with no escaping; do not call
    /// with untrusted content.
    pub fn raw(&mut self, text: impl AsRef<str>) -> &mut Self {
        self.html.push_str(text.as_ref());
        self
    }

    /// End the leaf element
    ///
    /// Add a closing tag (e.g. `</span>`).
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
    ///
    /// The characters `&` and `"` in `val` will automatically be escaped.
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
    ///
    /// The `text` will be truncated at `len` characters.
    pub fn text_len(self, text: impl AsRef<str>, len: usize) -> &'h mut Html {
        self.html.text_len(text, len)
    }

    /// End the element
    ///
    /// Adds the closing tag (e.g. `</span>`).
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

    /// End the element
    ///
    /// Since Void elements have no closing tags, this only returns the `Html`
    pub fn end(self) -> &'h mut Html {
        self.html
    }
}

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

global_attributes![class, dir, hidden, id, lang];

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

attributes![height, href, max, maxlength, min, size, src, value, width];

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
                #[doc = ") child element"]
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
                #[doc = ") child Void element"]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn div() {
        let mut html = Html::new();
        html.div();
        let html = String::from(html);
        assert_eq!(html, "<div></div>");
    }

    #[test]
    fn boolean() {
        let mut html = Html::new();
        html.div().id("test").attr_bool("spellcheck");
        let html = String::from(html);
        assert_eq!(html, "<div id=\"test\" spellcheck></div>");
    }

    #[test]
    fn paragraph() {
        let mut html = Html::new();
        html.p().text("This is a paragraph");
        let html = String::from(html);
        assert_eq!(html, "<p>This is a paragraph</p>");
    }

    #[test]
    fn escaping() {
        let mut html = Html::new();
        html.em().text("You & I");
        let html = String::from(html);
        assert_eq!(html, "<em>You &amp; I</em>");
    }

    #[test]
    fn raw_burger() {
        let mut html = Html::new();
        html.span().text("Raw").raw(" <em>Burger</em>!");
        let html = String::from(html);
        assert_eq!(html, "<span>Raw <em>Burger</em>!</span>");
    }

    #[test]
    fn void() {
        let mut html = Html::new();
        html.div().input().type_("text");
        let html = String::from(html);
        assert_eq!(html, "<div><input type=\"text\"></div>");
    }

    #[test]
    fn html() {
        let mut html = Html::new();
        html.ol();
        html.li().class("cat").text("nori").end();
        html.li().class("cat").text("chashu");
        let html = String::from(html);
        assert_eq!(
            html,
            "<ol><li class=\"cat\">nori</li><li class=\"cat\">chashu</li></ol>"
        );
    }

    #[test]
    fn build_html() {
        let mut html = Html::new();
        html.div().p().text("Paragraph Text").end();
        html.pre().text("Preformatted Text");
        let html = String::from(html);
        assert_eq!(
            &html,
            "<div><p>Paragraph Text</p><pre>Preformatted Text</pre></div>"
        );
    }

    #[test]
    fn html_builder() {
        let mut html = Html::new();
        html.html().lang("en");
        html.head().title().text("Title!").end().end();
        html.body().h1().text("Header!");
        let html = String::from(html);
        assert_eq!(
            html,
            "<html lang=\"en\"><head><title>Title!</title></head><body><h1>Header!</h1></body></html>"
        );
    }
}
