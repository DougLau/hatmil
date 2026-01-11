// page.rs
//
// Copyright (C) 2025-2026  Douglas P Lau
//
use crate::html::Html;
use crate::value::Value;
use std::fmt;

/// Element type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElemType {
    /// HTML element
    Html,
    /// HTML void element
    HtmlVoid,
    /// XML element (SVG)
    Xml,
}

/// HTML page builder
#[derive(Default)]
pub struct Page {
    /// Include HTML `DOCTYPE` preamble
    doctype: bool,
    /// HTML document text
    doc: String,
    /// Stack of element tags
    stack: Vec<&'static str>,
    /// Leaf node element type
    tp: Option<ElemType>,
    /// Current tag empty
    empty: bool,
}

/// Element borrowed from a [Page]
pub trait Element<'p> {
    /// Element tag
    const TAG: &'static str;

    /// Element type
    const TP: ElemType;

    /// Make new "base" element
    fn new(page: &'p mut Page) -> Self;
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut void = self.tp == Some(ElemType::HtmlVoid);
        let mut self_closing = self.empty && self.tp == Some(ElemType::Xml);
        if self_closing && let Some(page) = self.doc.strip_suffix('>') {
            write!(f, "{}", page)?;
        } else {
            write!(f, "{}", self.doc)?;
            self_closing = false;
        }
        for tag in self.stack.iter().rev() {
            if self_closing {
                write!(f, " />")?;
            } else if !void {
                write!(f, "</{tag}>")?;
            }
            self_closing = false;
            void = false;
        }
        Ok(())
    }
}

impl From<Page> for String {
    fn from(mut page: Page) -> Self {
        // zero-copy alternative to fmt::Display
        page.close_to(1);
        page.doc
    }
}

impl Page {
    /// Create an HTML page builder
    ///
    /// ```rust
    /// use hatmil::Page;
    ///
    /// let mut page = Page::new().with_doctype();
    /// let mut html = page.html();
    /// let mut body = html.body();
    /// body.cdata("Page text");
    /// body.a().href("https://www.example.com/").cdata("Example link");
    /// assert_eq!(
    ///     page.to_string(),
    ///     "<!DOCTYPE html><html><body>Page text<a href=\"https://www.example.com/\">Example link</a></body></html>",
    /// );
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Include HTML `DOCTYPE` preamble
    pub fn with_doctype(mut self) -> Self {
        self.doctype = true;
        self
    }

    /// Convert page into a fragment
    ///
    /// - `E`: Element type, from either the [html] or [svg] modules
    ///
    /// ```rust
    /// use hatmil::{Page, html::A};
    ///
    /// let mut page = Page::default();
    /// page.frag::<A>().href("https://www.example.com/").cdata("Example link");
    /// assert_eq!(
    ///     page.to_string(),
    ///     "<a href=\"https://www.example.com/\">Example link</a>",
    /// );
    /// ```
    ///
    /// [html]: crate::html
    /// [svg]: crate::svg
    pub fn frag<'p, E>(&'p mut self) -> E
    where
        E: Element<'p>,
    {
        self.elem(E::TAG, E::TP);
        E::new(self)
    }

    /// Add `<html>` root element
    pub fn html(&mut self) -> Html<'_> {
        self.elem("html", ElemType::Html);
        Html::new(self)
    }

    /// Add an element
    ///
    /// - `tag`: Element tag
    /// - `tp`: Element type
    ///
    /// [Void]: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
    pub(crate) fn elem(&mut self, tag: &'static str, tp: ElemType) -> usize {
        if self.stack.is_empty() {
            self.doc.clear();
            if self.doctype {
                self.raw("<!DOCTYPE html>");
            }
        }
        self.doc.push('<');
        self.doc.push_str(tag);
        self.doc.push('>');
        self.empty = true;
        self.tp = Some(tp);
        self.stack.push(tag);
        self.stack.len()
    }

    /// Add an attribute with value
    ///
    /// These characters will be replaced with entities:
    ///
    /// | Char | Entity   |
    /// |------|----------|
    /// | `&`  | `&amp;`  |
    /// | `"`  | `&quot;` |
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

    /// Add a [Boolean] attribute
    ///
    /// [Boolean]: https://developer.mozilla.org/en-US/docs/Glossary/Boolean/HTML
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
    /// These characters will be replaced with entities:
    ///
    /// | Char | Entity     |
    /// |------|------------|
    /// | `-`  | `&hyphen;` |
    /// | `<`  | `&gt;`     |
    /// | `>`  | `&lt;`     |
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

    /// Add character data content
    pub(crate) fn cdata<'a, V>(&mut self, text: V) -> &mut Self
    where
        V: Into<Value<'a>>,
    {
        self.cdata_len(text, usize::MAX)
    }

    /// Add character data content with a maximum character limit
    pub(crate) fn cdata_len<'a, V>(&mut self, text: V, len: usize) -> &mut Self
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
    /// **WARNING**: `trusted` is used verbatim, with no escaping; do not call
    /// with untrusted content.
    pub fn raw(&mut self, trusted: impl AsRef<str>) -> &mut Self {
        self.doc.push_str(trusted.as_ref());
        self.empty = false;
        self
    }

    /// Close elements to the specified depth
    pub(crate) fn close_to(&mut self, depth: usize) -> &mut Self {
        while self.stack.len() >= depth {
            self.close();
        }
        self
    }

    /// Close the final open element
    ///
    /// Add a closing tag (e.g. `</span>`).
    pub fn close(&mut self) -> &mut Self {
        let tp = self.tp.take();
        if let Some(tag) = self.stack.pop() {
            let void = tp == Some(ElemType::HtmlVoid);
            let self_closing = self.empty && tp == Some(ElemType::Xml);
            if self_closing && self.doc.ends_with('>') {
                self.doc.pop();
                self.doc.push_str(" />");
            } else if !void {
                self.doc.push_str("</");
                self.doc.push_str(tag);
                self.doc.push('>');
            }
        }
        self.empty = false;
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::html::*;

    #[test]
    fn div() {
        let mut page = Page::default();
        page.frag::<Div>();
        assert_eq!(page.to_string(), "<div></div>");
    }

    #[test]
    fn boolean() {
        let mut page = Page::default();
        page.frag::<Div>().id("test").spellcheck(true);
        assert_eq!(
            page.to_string(),
            "<div id=\"test\" spellcheck=\"true\"></div>"
        );
    }

    #[test]
    fn paragraph() {
        let mut page = Page::default();
        page.frag::<P>().cdata("This is a paragraph");
        assert_eq!(page.to_string(), "<p>This is a paragraph</p>");
    }

    #[test]
    fn escaping() {
        let mut page = Page::default();
        page.frag::<Em>().cdata("You & I");
        assert_eq!(page.to_string(), "<em>You &amp; I</em>");
    }

    #[test]
    fn raw_burger() {
        let mut page = Page::default();
        page.frag::<Span>().cdata("Raw").raw(" <em>Burger</em>!");
        assert_eq!(page.to_string(), "<span>Raw <em>Burger</em>!</span>");
    }

    #[test]
    fn void() {
        let mut page = Page::default();
        page.frag::<Div>().input().r#type("text");
        assert_eq!(page.to_string(), "<div><input type=\"text\"></div>");
    }

    #[test]
    fn html() {
        let mut page = Page::default();
        let mut ol = page.frag::<Ol>();
        ol.li().class("cat").cdata("nori").close();
        ol.li().class("cat").cdata("chashu");
        assert_eq!(
            page.to_string(),
            "<ol><li class=\"cat\">nori</li><li class=\"cat\">chashu</li></ol>"
        );
    }

    #[test]
    fn build_html() {
        let mut page = Page::default();
        let mut div = page.frag::<Div>();
        div.p().cdata("Paragraph Text").close();
        div.pre().cdata("Preformatted Text");
        assert_eq!(
            page.to_string(),
            "<div><p>Paragraph Text</p><pre>Preformatted Text</pre></div>"
        );
    }

    #[test]
    fn html_builder() {
        let mut page = Page::default();
        let mut html = page.html();
        let mut head = html.lang("en").head();
        head.title_el().cdata("Title!");
        head.close();
        html.body().h1().cdata("Header!");
        assert_eq!(
            page.to_string(),
            "<html lang=\"en\"><head><title>Title!</title></head><body><h1>Header!</h1></body></html>"
        );
    }

    #[test]
    fn string_from() {
        let mut page = Page::new();
        let mut html = page.html();
        html.head().title_el().cdata("Head").close().close();
        html.body().cdata("Body");
        assert_eq!(
            String::from(page),
            "<html><head><title>Head</title></head><body>Body</body></html>"
        );
    }

    #[test]
    fn comment() {
        let mut page = Page::default();
        page.frag::<I>().comment("comment");
        assert_eq!(page.to_string(), "<i><!--comment--></i>");
    }

    #[test]
    fn comment_escape() {
        let mut page = Page::default();
        page.comment("<-->");
        assert_eq!(page.to_string(), "<!--&lt;&hyphen;&hyphen;&gt;-->");
    }

    #[test]
    fn xml() {
        let mut page = Page::default();
        page.frag::<Link>().rel("stylesheet").close();
        assert_eq!(page.to_string(), "<link rel=\"stylesheet\" />");
    }

    #[test]
    fn close() {
        let mut page = Page::default();
        page.frag::<Span>().id("gle").close();
        assert_eq!(page.to_string(), "<span id=\"gle\"></span>");
    }

    #[test]
    fn image() {
        let mut page = Page::default();
        page.frag::<Img>().width(100).height(50).close();
        assert_eq!(page.to_string(), "<img width=\"100\" height=\"50\">");
    }
}
