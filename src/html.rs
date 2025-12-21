// html.rs
//
// Copyright (C) 2025  Douglas P Lau
//
use crate::svg::Svg;
use crate::value::Value;
use std::fmt;

/// User-friendly HTML builder
///
/// All common HTML elements are available as methods returning an [Elem].
#[derive(Default)]
pub struct Page {
    /// XML compatibility (self-closing tags include `/`)
    xml_compatible: bool,
    /// HTML document text
    doc: String,
    /// Tag stack
    stack: Vec<&'static str>,
    /// Current tag empty + XML compatible
    empty: bool,
}

/// Element borrowed from a [Page]
pub struct Elem<'h> {
    page: &'h mut Page,
}

/// [Void] element borrowed from a [Page]
///
/// [Void]: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
pub struct VoidElem<'h> {
    page: &'h mut Page,
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut empty = self.empty;
        if empty && let Some(page) = self.doc.strip_suffix('>') {
            write!(f, "{}", page)?;
        } else {
            write!(f, "{}", self.doc)?;
            empty = false;
        }
        for elem in self.stack.iter().rev() {
            if empty {
                write!(f, " />")?;
            } else {
                write!(f, "</{elem}>")?;
            }
            empty = false;
        }
        Ok(())
    }
}

impl From<Page> for String {
    fn from(mut page: Page) -> Self {
        // zero-copy alternative to fmt::Display
        while let Some(elem) = page.stack.pop() {
            page.doc.push_str("</");
            page.doc.push_str(elem);
            page.doc.push('>');
        }
        page.doc
    }
}

impl Page {
    /// Create an HTML page builder
    ///
    /// ```rust
    /// use hatmil::Page;
    ///
    /// let mut page = Page::new();
    /// page.a().href("https://www.example.com/").text("Example link");
    /// assert_eq!(
    ///     page.to_string(),
    ///     "<a href=\"https://www.example.com/\">Example link</a>",
    /// );
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Create an XML-compatible page builder
    ///
    /// This causes `/` to be included in self-closing tags.
    pub fn new_xml_compatible() -> Self {
        Page {
            xml_compatible: true,
            ..Default::default()
        }
    }

    /// Create a page builder with a `doctype` preamble
    ///
    /// ```rust
    /// use hatmil::Page;
    ///
    /// let mut page = Page::with_doctype();
    /// page.html().body().text("Page text");
    /// assert_eq!(
    ///     page.to_string(),
    ///     "<!doctype html><html><body>Page text</body></html>",
    /// );
    /// ```
    pub fn with_doctype() -> Self {
        let mut page = Page::default();
        page.raw("<!doctype html>");
        page
    }

    /// Add an element
    pub(crate) fn elem(&mut self, elem: &'static str) -> Elem<'_> {
        self.doc.push('<');
        self.doc.push_str(elem);
        self.doc.push('>');
        self.stack.push(elem);
        self.empty = self.xml_compatible;
        Elem { page: self }
    }

    /// Add a Void element
    pub(crate) fn void_elem(&mut self, elem: &'static str) -> VoidElem<'_> {
        self.doc.push('<');
        self.doc.push_str(elem);
        self.doc.push('>');
        self.empty = false;
        VoidElem { page: self }
    }

    /// Add an SVG element
    pub(crate) fn svg_elem(&mut self, elem: &'static str) -> Svg<'_> {
        self.doc.push('<');
        self.doc.push_str(elem);
        self.doc.push('>');
        self.stack.push(elem);
        self.empty = true;
        Svg::new(self)
    }

    /// Add an SVG element
    pub fn svg(&mut self) -> Svg<'_> {
        self.svg_elem("svg")
    }

    /// Add an attribute with value
    pub(crate) fn attr<'a, V>(&mut self, attr: &'static str, val: V)
    where
        V: Into<Value<'a>>,
    {
        match self.doc.pop() {
            Some(gt) => assert_eq!(gt, '>'),
            None => unreachable!(),
        }
        self.doc.push(' ');
        self.doc.push_str(attr);
        self.doc.push_str("=\"");
        for c in val.into().chars() {
            match c {
                '&' => self.doc.push_str("&amp;"),
                '"' => self.doc.push_str("&quot;"),
                _ => self.doc.push(c),
            }
        }
        self.doc.push_str("\">");
    }

    /// Add a Boolean attribute
    pub(crate) fn attr_bool(&mut self, attr: &'static str) {
        match self.doc.pop() {
            Some(gt) => assert_eq!(gt, '>'),
            None => unreachable!(),
        }
        self.doc.push(' ');
        self.doc.push_str(attr);
        self.doc.push('>');
    }

    /// Add a comment
    ///
    /// The characters `-`, `<` and `>` in `com` will automatically be
    /// escaped.
    pub fn comment<'a, V>(&mut self, com: V) -> &mut Self
    where
        V: Into<Value<'a>>,
    {
        self.doc.push_str("<!--");
        for c in com.into().chars() {
            match c {
                '-' => self.doc.push_str("&hyphen;"),
                '<' => self.doc.push_str("&lt;"),
                '>' => self.doc.push_str("&gt;"),
                _ => self.doc.push(c),
            }
        }
        self.doc.push_str("-->");
        self.empty = false;
        self
    }

    /// Add text content
    ///
    /// The characters `&`, `<` and `>` in `text` will automatically be
    /// escaped.
    pub fn text<'a, V>(&mut self, text: V) -> &mut Self
    where
        V: Into<Value<'a>>,
    {
        self.text_len(text, usize::MAX)
    }

    /// Add text content with a maximum character limit
    ///
    /// The characters `&`, `<` and `>` in `text` will automatically be
    /// escaped.
    pub fn text_len<'a, V>(&mut self, text: V, len: usize) -> &mut Self
    where
        V: Into<Value<'a>>,
    {
        for c in text.into().chars().take(len) {
            match c {
                '&' => self.doc.push_str("&amp;"),
                '<' => self.doc.push_str("&lt;"),
                '>' => self.doc.push_str("&gt;"),
                _ => self.doc.push(c),
            }
        }
        self.empty = false;
        self
    }

    /// Add raw content
    ///
    /// **WARNING**: `text` is used verbatim, with no escaping; do not call
    /// with untrusted content.
    pub fn raw(&mut self, text: impl AsRef<str>) -> &mut Self {
        self.doc.push_str(text.as_ref());
        self.empty = false;
        self
    }

    /// End the leaf element
    ///
    /// Add a closing tag (e.g. `</span>`).
    pub fn end(&mut self) -> &mut Self {
        if let Some(elem) = self.stack.pop() {
            if self.empty && self.doc.ends_with('>') {
                self.doc.pop();
                self.doc.push_str(" />");
            } else {
                self.doc.push_str("</");
                self.doc.push_str(elem);
                self.doc.push('>');
            }
        }
        self.empty = false;
        self
    }
}

impl<'h> Elem<'h> {
    /// Add an attribute with value
    ///
    /// NOTE: dedicated methods such as [id] or [name] should be used when
    ///       available.
    ///
    /// The characters `&` and `"` in `val` will automatically be escaped.
    ///
    /// [id]: #method.id
    /// [name]: #method.name
    pub fn attr<'a, V>(self, attr: &'static str, val: V) -> Self
    where
        V: Into<Value<'a>>,
    {
        self.page.attr(attr, val);
        self
    }

    /// Add a [Boolean] attribute
    ///
    /// [Boolean]: https://developer.mozilla.org/en-US/docs/Glossary/Boolean/HTML
    pub fn attr_bool(self, attr: &'static str) -> Self {
        self.page.attr_bool(attr);
        self
    }

    ///  Add [type](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/type) attribute
    ///
    /// NOTE: use `r#type(...)` to invoke
    pub fn r#type<'a, V>(self, val: V) -> Self
    where
        V: Into<Value<'a>>,
    {
        self.page.attr("type", val);
        self
    }

    ///  Add [for](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/for) attribute
    ///
    /// NOTE: use `r#for(...)` to invoke
    pub fn r#for<'a, V>(self, val: V) -> Self
    where
        V: Into<Value<'a>>,
    {
        self.page.attr("for", val);
        self
    }

    /// Add text content
    ///
    /// The characters `&`, `<` and `>` in `text` will automatically be
    /// escaped.
    pub fn text<'a, V>(self, text: V) -> &'h mut Page
    where
        V: Into<Value<'a>>,
    {
        self.page.text_len(text, usize::MAX)
    }

    /// Add text content with a maximum character limit
    ///
    /// The characters `&`, `<` and `>` in `text` will automatically be
    /// escaped.
    pub fn text_len<'a, V>(self, text: V, len: usize) -> &'h mut Page
    where
        V: Into<Value<'a>>,
    {
        self.page.text_len(text, len)
    }

    /// End the element
    ///
    /// Adds the closing tag (e.g. `</span>`).
    pub fn end(self) -> &'h mut Page {
        self.page.end()
    }
}

impl<'h> VoidElem<'h> {
    /// Add an attribute with value
    ///
    /// The characters `&` and `"` in `val` will automatically be escaped.
    pub fn attr<'a, V>(self, attr: &'static str, val: V) -> Self
    where
        V: Into<Value<'a>>,
    {
        self.page.attr(attr, val);
        self
    }

    /// Add a [Boolean] attribute
    ///
    /// [Boolean]: https://developer.mozilla.org/en-US/docs/Glossary/Boolean/HTML
    pub fn attr_bool(self, attr: &'static str) -> Self {
        self.page.attr_bool(attr);
        self
    }

    ///  Add [type](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/type) attribute
    ///
    /// NOTE: use `r#type(...)` to invoke
    pub fn r#type<'a, V>(self, val: V) -> Self
    where
        V: Into<Value<'a>>,
    {
        self.page.attr("type", val);
        self
    }

    /// End the element
    ///
    /// Since Void elements have no closing tags, this simply returns the
    /// [Page] to allow chaining method calls.
    pub fn end(self) -> &'h mut Page {
        let page = self.page;
        if page.xml_compatible {
            match page.doc.pop() {
                Some(gt) => assert_eq!(gt, '>'),
                None => unreachable!(),
            }
            page.doc.push_str(" />");
        }
        page
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
                pub fn $attr<'a, V>(self, val: V) -> Self
                    where V: Into<Value<'a>>
                {
                    self.page.attr(stringify!($attr), val);
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
                pub fn $attr<'a, V>(self, val: V) -> Self
                    where V: Into<Value<'a>>
                {
                    self.page.attr(stringify!($attr), val);
                    self
                }
            )*
        }
    }
}

global_attributes![
    accesskey,
    autocapitalize,
    /* autocorrect, */
    /* autofocus, */
    class,
    contenteditable,
    /* data-* */
    dir,
    draggable,
    enterkeyhint,
    exportparts,
    hidden,
    id,
    /* inert, */
    /* is, */
    inputmode,
    lang,
    nonce,
    part,
    popover,
    role,
    /* slot, */
    spellcheck,
    /* style, */
    tabindex,
    /* title, */
    translate
];

/// HTML attribute helper
macro_rules! attributes {
    ( $( $attr:ident ),* ) => {
        impl<'h> Elem<'h> {
            $(
                #[doc = concat!("Add `", stringify!($attr), "` ")]
                #[doc = "[attribute]("]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/", stringify!($attr))]
                #[doc = ")"]
                pub fn $attr<'a, V>(self, val: V) -> Self
                    where V: Into<Value<'a>>
                {
                    self.page.attr(stringify!($attr), val);
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
                pub fn $attr<'a, V>(self, val: V) -> Self
                    where V: Into<Value<'a>>
                {
                    self.page.attr(stringify!($attr), val);
                    self
                }
            )*
        }
    }
}

attributes![
    accept,
    action,
    allow,
    alt,
    autocomplete,
    autoplay,
    cols,
    colspan,
    content,
    controls,
    coords,
    crossorigin,
    datetime,
    decoding,
    height,
    high,
    href,
    low,
    max,
    maxlength,
    min,
    name,
    optimum,
    placeholder,
    rel,
    rows,
    size,
    src,
    step,
    target,
    value,
    width
];

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
                    self.page.attr_bool(stringify!($attr));
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
                    self.page.attr_bool(stringify!($attr));
                    self
                }
            )*
        }
    }
}

boolean_attributes![
    autofocus, /* also global */
    checked, disabled, inert, /* also global */
    multiple, readonly, required, selected
];

/// HTML element helper
macro_rules! elements {
    ( $( $elem:ident ),* ) => {
        impl Page {
            $(
                #[doc = concat!("Add [", stringify!($elem), "](")]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($elem))]
                #[doc = ") element"]
                pub fn $elem(&mut self) -> Elem<'_> {
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
                    self.page.elem(stringify!($elem))
                }
            )*
        }
    }
}

/// HTML Void element helper
macro_rules! void_elements {
    ( $( $elem:ident ),* ) => {
        impl Page {
            $(
                #[doc = concat!("Add [", stringify!($elem), "](")]
                #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($elem))]
                #[doc = ") Void element"]
                pub fn $elem(&mut self) -> VoidElem<'_> {
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
                    self.page.void_elem(stringify!($elem))
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

void_elements![
    area, base, col, embed, hr, img, input, link, meta, source, track, wbr
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn div() {
        let mut page = Page::new();
        page.div();
        assert_eq!(page.to_string(), "<div></div>");
    }

    #[test]
    fn boolean() {
        let mut page = Page::new();
        page.div().id("test").attr_bool("spellcheck");
        assert_eq!(page.to_string(), "<div id=\"test\" spellcheck></div>");
    }

    #[test]
    fn paragraph() {
        let mut page = Page::new();
        page.p().text("This is a paragraph");
        assert_eq!(page.to_string(), "<p>This is a paragraph</p>");
    }

    #[test]
    fn escaping() {
        let mut page = Page::new();
        page.em().text("You & I");
        assert_eq!(page.to_string(), "<em>You &amp; I</em>");
    }

    #[test]
    fn raw_burger() {
        let mut page = Page::new();
        page.span().text("Raw").raw(" <em>Burger</em>!");
        assert_eq!(page.to_string(), "<span>Raw <em>Burger</em>!</span>");
    }

    #[test]
    fn void() {
        let mut page = Page::new();
        page.div().input().r#type("text");
        assert_eq!(page.to_string(), "<div><input type=\"text\"></div>");
    }

    #[test]
    fn html() {
        let mut page = Page::new();
        page.ol();
        page.li().class("cat").text("nori").end();
        page.li().class("cat").text("chashu");
        assert_eq!(
            page.to_string(),
            "<ol><li class=\"cat\">nori</li><li class=\"cat\">chashu</li></ol>"
        );
    }

    #[test]
    fn build_html() {
        let mut page = Page::new();
        page.div().p().text("Paragraph Text").end();
        page.pre().text("Preformatted Text");
        assert_eq!(
            page.to_string(),
            "<div><p>Paragraph Text</p><pre>Preformatted Text</pre></div>"
        );
    }

    #[test]
    fn html_builder() {
        let mut page = Page::new();
        page.html().lang("en");
        page.head().title().text("Title!").end().end();
        page.body().h1().text("Header!");
        assert_eq!(
            page.to_string(),
            "<html lang=\"en\"><head><title>Title!</title></head><body><h1>Header!</h1></body></html>"
        );
    }

    #[test]
    fn string_from() {
        let mut page = Page::new();
        page.html();
        page.head().title().text("Head").end().end();
        page.body().text("Body");
        assert_eq!(
            String::from(page),
            "<html><head><title>Head</title></head><body>Body</body></html>"
        );
    }

    #[test]
    fn comment() {
        let mut page = Page::new();
        page.comment("comment");
        assert_eq!(page.to_string(), "<!--comment-->");
    }

    #[test]
    fn comment_escape() {
        let mut page = Page::new();
        page.comment("<-->");
        assert_eq!(page.to_string(), "<!--&lt;&hyphen;&hyphen;&gt;-->");
    }

    #[test]
    fn xml() {
        let mut page = Page::new_xml_compatible();
        page.link().rel("stylesheet").end();
        assert_eq!(page.to_string(), "<link rel=\"stylesheet\" />");
    }

    #[test]
    fn end() {
        let mut page = Page::new();
        page.span().name("a span").end();
        assert_eq!(page.to_string(), "<span name=\"a span\"></span>");
    }

    #[test]
    fn image() {
        let mut page = Page::new();
        page.img().width(100).height(50).end();
        assert_eq!(page.to_string(), "<img width=\"100\" height=\"50\">");
    }
}
