// html.rs
//
// Copyright (C) 2025  Douglas P Lau
//
use crate::html::{Element, Html};
use crate::value::Value;
use std::fmt;

/// User-friendly HTML builder
#[derive(Default)]
pub struct Page {
    /// Include HTML `doctype` preamble
    doctype: bool,
    /// XML compatibility (self-closing tags include `/`)
    xml_compatible: bool,
    /// HTML document text
    doc: String,
    /// Stack of (element tag, void flags)
    stack: Vec<(&'static str, bool)>,
    /// Current tag empty + XML compatible
    empty: bool,
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
        for (tag, _void) in self.stack.iter().rev() {
            if empty {
                write!(f, " />")?;
            } else {
                write!(f, "</{tag}>")?;
            }
            empty = false;
        }
        Ok(())
    }
}

impl From<Page> for String {
    fn from(mut page: Page) -> Self {
        // zero-copy alternative to fmt::Display
        while let Some((tag, _void)) = page.stack.pop() {
            page.doc.push_str("</");
            page.doc.push_str(tag);
            page.doc.push('>');
        }
        page.doc
    }
}

impl Page {
    /// Create an HTML page builder
    ///
    /// - `doctype`: Include HTML `doctype` preamble
    ///
    /// ```rust
    /// use hatmil::Page;
    ///
    /// let mut page = Page::new(true);
    /// let mut html = page.html();
    /// let mut body = html.body();
    /// body.text("Page text");
    /// body.a().href("https://www.example.com/").text("Example link");
    /// assert_eq!(
    ///     page.to_string(),
    ///     "<!doctype html><html><body>Page text<a href=\"https://www.example.com/\">Example link</a></body></html>",
    /// );
    /// ```
    pub fn new(doctype: bool) -> Self {
        Page {
            doctype,
            ..Default::default()
        }
    }

    /// Convert page into a fragment
    ///
    /// ```rust
    /// use hatmil::{Page, elem::A};
    ///
    /// let mut page = Page::default();
    /// page.frag::<A>().href("https://www.example.com/").text("Example link");
    /// assert_eq!(
    ///     page.to_string(),
    ///     "<a href=\"https://www.example.com/\">Example link</a>",
    /// );
    /// ```
    pub fn frag<'p, E>(&'p mut self) -> E
    where
        E: Element<'p>,
    {
        self.doc.clear();
        // FIXME: void or not?
        self.elem(E::TAG, false);
        E::new(self)
    }

    /// Make the page XML-compatible
    ///
    /// This causes `/` to be included in self-closing tags.
    pub fn xml_compatible(mut self) -> Self {
        self.xml_compatible = true;
        self
    }

    /// Add `<html>` root element
    pub fn html(&mut self) -> Html<'_> {
        self.doc.clear();
        if self.doctype {
            self.raw("<!doctype html>");
        }
        self.elem("html", false);
        Html::new(self)
    }

    /// Add an element
    ///
    /// - `tag`: Element tag
    /// - `void`: [Void] element
    ///
    /// [Void]: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
    pub(crate) fn elem(&mut self, tag: &'static str, void: bool) {
        self.doc.push('<');
        self.doc.push_str(tag);
        self.doc.push('>');
        self.stack.push((tag, void));
        self.empty = self.xml_compatible && !void;
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

    /// Add text content
    pub(crate) fn text<'a, V>(&mut self, text: V) -> &mut Self
    where
        V: Into<Value<'a>>,
    {
        self.text_len(text, usize::MAX)
    }

    /// Add text content with a maximum character limit
    pub(crate) fn text_len<'a, V>(&mut self, text: V, len: usize) -> &mut Self
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

    /// End the leaf element
    ///
    /// Add a closing tag (e.g. `</span>`).
    pub fn end(&mut self) -> &mut Self {
        if let Some((tag, _void)) = self.stack.pop() {
            if self.empty && self.doc.ends_with('>') {
                self.doc.pop();
                self.doc.push_str(" />");
            } else {
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
    use crate::elem::*;

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
        page.frag::<P>().text("This is a paragraph");
        assert_eq!(page.to_string(), "<p>This is a paragraph</p>");
    }

    #[test]
    fn escaping() {
        let mut page = Page::default();
        page.frag::<Em>().text("You & I");
        assert_eq!(page.to_string(), "<em>You &amp; I</em>");
    }

    #[test]
    fn raw_burger() {
        let mut page = Page::default();
        page.frag::<Span>().text("Raw").raw(" <em>Burger</em>!");
        assert_eq!(page.to_string(), "<span>Raw <em>Burger</em>!</span>");
    }
    /*
        #[test]
        fn void() {
            let mut page = Page::default();
            page.frag::<Div>().input().r#type("text");
            assert_eq!(page.to_string(), "<div><input type=\"text\"></div>");
        }
    */
    #[test]
    fn html() {
        let mut page = Page::default();
        let mut ol = page.frag::<Ol>();
        ol.li().class("cat").text("nori").end();
        ol.li().class("cat").text("chashu");
        assert_eq!(
            page.to_string(),
            "<ol><li class=\"cat\">nori</li><li class=\"cat\">chashu</li></ol>"
        );
    }

    #[test]
    fn build_html() {
        let mut page = Page::default();
        let mut div = page.frag::<Div>();
        div.p().text("Paragraph Text").end();
        div.pre().text("Preformatted Text");
        assert_eq!(
            page.to_string(),
            "<div><p>Paragraph Text</p><pre>Preformatted Text</pre></div>"
        );
    }

    #[test]
    fn html_builder() {
        let mut page = Page::default();
        let mut html = page.html();
        html.lang("en")
            .head()
            .title_elem()
            .text("Title!")
            .end()
            .end();
        html.body().h1().text("Header!");
        assert_eq!(
            page.to_string(),
            "<html lang=\"en\"><head><title>Title!</title></head><body><h1>Header!</h1></body></html>"
        );
    }

    #[test]
    fn string_from() {
        let mut page = Page::new(false);
        let mut html = page.html();
        html.head().title_elem().text("Head").end().end();
        html.body().text("Body");
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
        let mut page = Page::default().xml_compatible();
        page.frag::<Link>().rel("stylesheet").end();
        assert_eq!(page.to_string(), "<link rel=\"stylesheet\" />");
    }

    #[test]
    fn end() {
        let mut page = Page::default();
        page.frag::<Span>().id("gle").end();
        assert_eq!(page.to_string(), "<span id=\"gle\"></span>");
    }
    /*
    #[test]
    fn image() {
        let mut page = Page::default();
        page.frag::<Img>().width(100).height(50).end();
        assert_eq!(page.to_string(), "<img width=\"100\" height=\"50\">");
    }*/
}
