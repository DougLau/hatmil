// html.rs
//
// Copyright (C) 2025  Douglas P Lau
//
use crate::value::Value;
use crate::svg::Svg;
use std::fmt;

/// Simple HTML builder
#[derive(Default)]
pub struct Html {
    /// XML compatibility (self-closing tags include `/`)
    xml_compatible: bool,
    /// HTML document text
    html: String,
    /// Tag stack
    stack: Vec<&'static str>,
    /// Current tag empty + XML compatible
    empty: bool,
}

/// Element borrowed from an [Html]
pub struct Elem<'h> {
    html: &'h mut Html,
}

/// [Void] element borrowed from an [Html]
///
/// [Void]: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
pub struct VoidElem<'h> {
    html: &'h mut Html,
}

impl fmt::Display for Html {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut empty = self.empty;
        if empty && let Some(html) = self.html.strip_suffix('>') {
            write!(f, "{}", html)?;
        } else {
            write!(f, "{}", self.html)?;
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

impl From<Html> for String {
    fn from(mut html: Html) -> Self {
        // zero-copy alternative to fmt::Display
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

    /// Create an XML-compatible HTML builder
    pub fn new_xml_compatible() -> Self {
        Html {
            xml_compatible: true,
            ..Default::default()
        }
    }

    /// Create an HTML builder with a `doctype` preamble
    pub fn with_doctype() -> Self {
        let mut html = Html::default();
        html.raw("<!doctype html>");
        html
    }

    /// Add an element
    pub(crate) fn elem(&mut self, elem: &'static str) -> Elem<'_> {
        self.html.push('<');
        self.html.push_str(elem);
        self.html.push('>');
        self.stack.push(elem);
        self.empty = self.xml_compatible;
        Elem { html: self }
    }

    /// Add a Void element
    pub(crate) fn void_elem(&mut self, elem: &'static str) -> VoidElem<'_> {
        self.html.push('<');
        self.html.push_str(elem);
        self.html.push('>');
        self.empty = false;
        VoidElem { html: self }
    }

    /// Add an SVG element
    pub(crate) fn svg_elem(&mut self, elem: &'static str) -> Svg<'_> {
        self.html.push('<');
        self.html.push_str(elem);
        self.html.push('>');
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
        match self.html.pop() {
            Some(gt) => assert_eq!(gt, '>'),
            None => unreachable!(),
        }
        self.html.push(' ');
        self.html.push_str(attr);
        self.html.push_str("=\"");
        for c in val.into().chars() {
            match c {
                '&' => self.html.push_str("&amp;"),
                '"' => self.html.push_str("&quot;"),
                _ => self.html.push(c),
            }
        }
        self.html.push_str("\">");
    }

    /// Add a Boolean attribute
    pub(crate) fn attr_bool(&mut self, attr: &'static str) {
        match self.html.pop() {
            Some(gt) => assert_eq!(gt, '>'),
            None => unreachable!(),
        }
        self.html.push(' ');
        self.html.push_str(attr);
        self.html.push('>');
    }

    /// Add a comment
    ///
    /// The characters `-`, `<` and `>` in `com` will automatically be
    /// escaped.
    pub fn comment<'a, V>(&mut self, com: V) -> &mut Self
    where
        V: Into<Value<'a>>,
    {
        self.html.push_str("<!--");
        for c in com.into().chars() {
            match c {
                '-' => self.html.push_str("&hyphen;"),
                '<' => self.html.push_str("&lt;"),
                '>' => self.html.push_str("&gt;"),
                _ => self.html.push(c),
            }
        }
        self.html.push_str("-->");
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
                '&' => self.html.push_str("&amp;"),
                '<' => self.html.push_str("&lt;"),
                '>' => self.html.push_str("&gt;"),
                _ => self.html.push(c),
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
        self.html.push_str(text.as_ref());
        self.empty = false;
        self
    }

    /// End the leaf element
    ///
    /// Add a closing tag (e.g. `</span>`).
    pub fn end(&mut self) -> &mut Self {
        if let Some(elem) = self.stack.pop() {
            if self.empty && self.html.ends_with('>') {
                self.html.pop();
                self.html.push_str(" />");
            } else {
                self.html.push_str("</");
                self.html.push_str(elem);
                self.html.push('>');
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
        self.html.attr(attr, val);
        self
    }

    /// Add a [Boolean] attribute
    ///
    /// [Boolean]: https://developer.mozilla.org/en-US/docs/Glossary/Boolean/HTML
    pub fn attr_bool(self, attr: &'static str) -> Self {
        self.html.attr_bool(attr);
        self
    }

    ///  Add [type](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/type) attribute
    ///
    /// NOTE: use `r#type(...)` to invoke
    pub fn r#type<'a, V>(self, val: V) -> Self
    where
        V: Into<Value<'a>>,
    {
        self.html.attr("type", val);
        self
    }

    ///  Add [for](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/for) attribute
    ///
    /// NOTE: use `r#for(...)` to invoke
    pub fn r#for<'a, V>(self, val: V) -> Self
    where
        V: Into<Value<'a>>,
    {
        self.html.attr("for", val);
        self
    }

    /// Add text content
    ///
    /// The characters `&`, `<` and `>` in `text` will automatically be
    /// escaped.
    pub fn text<'a, V>(self, text: V) -> &'h mut Html
    where
        V: Into<Value<'a>>,
    {
        self.html.text_len(text, usize::MAX)
    }

    /// Add text content with a maximum character limit
    ///
    /// The characters `&`, `<` and `>` in `text` will automatically be
    /// escaped.
    pub fn text_len<'a, V>(self, text: V, len: usize) -> &'h mut Html
    where
        V: Into<Value<'a>>,
    {
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
    ///
    /// The characters `&` and `"` in `val` will automatically be escaped.
    pub fn attr<'a, V>(self, attr: &'static str, val: V) -> Self
    where
        V: Into<Value<'a>>,
    {
        self.html.attr(attr, val);
        self
    }

    /// Add a [Boolean] attribute
    ///
    /// [Boolean]: https://developer.mozilla.org/en-US/docs/Glossary/Boolean/HTML
    pub fn attr_bool(self, attr: &'static str) -> Self {
        self.html.attr_bool(attr);
        self
    }

    ///  Add [type](https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/type) attribute
    ///
    /// NOTE: use `r#type(...)` to invoke
    pub fn r#type<'a, V>(self, val: V) -> Self
    where
        V: Into<Value<'a>>,
    {
        self.html.attr("type", val);
        self
    }

    /// End the element
    ///
    /// Since Void elements have no closing tags, this simply returns the
    /// [Html] to allow chaining method calls.
    pub fn end(self) -> &'h mut Html {
        let html = self.html;
        if html.xml_compatible {
            match html.html.pop() {
                Some(gt) => assert_eq!(gt, '>'),
                None => unreachable!(),
            }
            html.html.push_str(" />");
        }
        html
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
                pub fn $attr<'a, V>(self, val: V) -> Self
                    where V: Into<Value<'a>>
                {
                    self.html.attr(stringify!($attr), val);
                    self
                }
            )*
        }
    }
}

global_attributes![
    accesskey,
    autocapitalize,
    /* autofocus, */
    class,
    contenteditable,
    dir,
    draggable,
    enterkeyhint,
    hidden,
    id,
    /* inert, */
    inputmode,
    lang,
    role,
    /* style, */
    /* title, */
    tabindex
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
                pub fn $attr<'a, V>(self, val: V) -> Self
                    where V: Into<Value<'a>>
                {
                    self.html.attr(stringify!($attr), val);
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

boolean_attributes![
    autofocus, /* also global */
    checked, disabled, inert, /* also global */
    multiple, readonly, required, selected
];

/// HTML element helper
macro_rules! elements {
    ( $( $elem:ident ),* ) => {
        impl Html {
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
                    self.html.elem(stringify!($elem))
                }
            )*
        }
    }
}

/// HTML Void element helper
macro_rules! void_elements {
    ( $( $elem:ident ),* ) => {
        impl Html {
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
                    self.html.void_elem(stringify!($elem))
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
        let mut html = Html::new();
        html.div();
        assert_eq!(html.to_string(), "<div></div>");
    }

    #[test]
    fn boolean() {
        let mut html = Html::new();
        html.div().id("test").attr_bool("spellcheck");
        assert_eq!(html.to_string(), "<div id=\"test\" spellcheck></div>");
    }

    #[test]
    fn paragraph() {
        let mut html = Html::new();
        html.p().text("This is a paragraph");
        assert_eq!(html.to_string(), "<p>This is a paragraph</p>");
    }

    #[test]
    fn escaping() {
        let mut html = Html::new();
        html.em().text("You & I");
        assert_eq!(html.to_string(), "<em>You &amp; I</em>");
    }

    #[test]
    fn raw_burger() {
        let mut html = Html::new();
        html.span().text("Raw").raw(" <em>Burger</em>!");
        assert_eq!(html.to_string(), "<span>Raw <em>Burger</em>!</span>");
    }

    #[test]
    fn void() {
        let mut html = Html::new();
        html.div().input().r#type("text");
        assert_eq!(html.to_string(), "<div><input type=\"text\"></div>");
    }

    #[test]
    fn html() {
        let mut html = Html::new();
        html.ol();
        html.li().class("cat").text("nori").end();
        html.li().class("cat").text("chashu");
        assert_eq!(
            html.to_string(),
            "<ol><li class=\"cat\">nori</li><li class=\"cat\">chashu</li></ol>"
        );
    }

    #[test]
    fn build_html() {
        let mut html = Html::new();
        html.div().p().text("Paragraph Text").end();
        html.pre().text("Preformatted Text");
        assert_eq!(
            html.to_string(),
            "<div><p>Paragraph Text</p><pre>Preformatted Text</pre></div>"
        );
    }

    #[test]
    fn html_builder() {
        let mut html = Html::new();
        html.html().lang("en");
        html.head().title().text("Title!").end().end();
        html.body().h1().text("Header!");
        assert_eq!(
            html.to_string(),
            "<html lang=\"en\"><head><title>Title!</title></head><body><h1>Header!</h1></body></html>"
        );
    }

    #[test]
    fn string_from() {
        let mut html = Html::new();
        html.html();
        html.head().title().text("Head").end().end();
        html.body().text("Body");
        assert_eq!(
            String::from(html),
            "<html><head><title>Head</title></head><body>Body</body></html>"
        );
    }

    #[test]
    fn comment() {
        let mut html = Html::new();
        html.comment("comment");
        assert_eq!(html.to_string(), "<!--comment-->");
    }

    #[test]
    fn comment_escape() {
        let mut html = Html::new();
        html.comment("<-->");
        assert_eq!(html.to_string(), "<!--&lt;&hyphen;&hyphen;&gt;-->");
    }

    #[test]
    fn xml() {
        let mut html = Html::new_xml_compatible();
        html.link().rel("stylesheet").end();
        assert_eq!(html.to_string(), "<link rel=\"stylesheet\" />");
    }

    #[test]
    fn end() {
        let mut html = Html::new();
        html.span().name("a span").end();
        assert_eq!(html.to_string(), "<span name=\"a span\"></span>");
    }

    #[test]
    fn image() {
        let mut html = Html::new();
        html.img().width(100).height(50).end();
        assert_eq!(html.to_string(), "<img width=\"100\" height=\"50\">");
    }
}
