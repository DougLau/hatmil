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

    /// Add a boolean attribute
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

    /// Add a boolean attribute
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

    /// Add a boolean attribute
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
macro_rules! element {
    ($elem:ident) => {
        impl Html {
            #[doc = "Add `"]
            #[doc = stringify!($elem)]
            #[doc = "` [element]("]
            #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($elem))]
            #[doc = ")"]
            pub fn $elem(&mut self) -> Elem {
                self.elem(stringify!($elem))
            }
        }
        impl<'h> Elem<'h> {
            #[doc = "Add `"]
            #[doc = stringify!($elem)]
            #[doc = "` [element]("]
            #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($elem))]
            #[doc = ")"]
            pub fn $elem(self) -> Self {
                self.html.elem(stringify!($elem))
            }
        }
    }
}

/// HTML Void element helper
macro_rules! void_element {
    ($elem:ident) => {
        impl Html {
            #[doc = "Add `"]
            #[doc = stringify!($elem)]
            #[doc = "` [element]("]
            #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($elem))]
            #[doc = ")"]
            pub fn $elem(&mut self) -> VoidElem {
                self.void_elem(stringify!($elem))
            }
        }
        impl<'h> Elem<'h> {
            #[doc = "Add `"]
            #[doc = stringify!($elem)]
            #[doc = "` [element]("]
            #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Element/", stringify!($elem))]
            #[doc = ")"]
            pub fn $elem(self) -> VoidElem<'h> {
                self.html.void_elem(stringify!($elem))
            }
        }
    }
}

element!(a);
element!(abbr);
element!(address);
void_element!(area);
element!(article);
element!(aside);
element!(audio);
element!(b);
void_element!(base);
element!(bdi);
element!(bdo);
element!(blockquote);
element!(body);
element!(br);
element!(button);
element!(canvas);
element!(caption);
element!(cite);
element!(code);
void_element!(col);
element!(colgroup);
element!(data);
element!(datalist);
element!(dd);
element!(del);
element!(details);
element!(dfn);
element!(dialog);
element!(div);
element!(dl);
element!(dt);
element!(em);
void_element!(embed);
element!(fieldset);
element!(figcaption);
element!(figure);
element!(footer);
element!(form);
element!(h1);
element!(h2);
element!(h3);
element!(h4);
element!(h5);
element!(h6);
element!(head);
element!(header);
element!(hgroup);
void_element!(hr);
element!(html);
element!(i);
element!(iframe);
void_element!(img);
void_element!(input);
element!(ins);
element!(kbd);
element!(label);
element!(legend);
element!(li);
void_element!(link);
element!(main);
element!(map);
element!(mark);
element!(menu);
void_element!(meta);
element!(meter);
element!(nav);
element!(noscript);
element!(object);
element!(ol);
element!(optgroup);
element!(option);
element!(output);
element!(p);
element!(picture);
element!(pre);
element!(progress);
element!(q);
element!(rp);
element!(rt);
element!(ruby);
element!(s);
element!(samp);
element!(script);
element!(search);
element!(section);
element!(select);
element!(slot);
element!(small);
void_element!(source);
element!(span);
element!(strong);
element!(style);
element!(sub);
element!(summary);
element!(sup);
element!(table);
element!(tbody);
element!(td);
element!(template);
element!(textarea);
element!(tfoot);
element!(th);
element!(thead);
element!(time);
element!(title);
element!(tr);
void_element!(track);
element!(u);
element!(ul);
element!(var);
element!(video);
void_element!(wbr);

/// HTML global attribute helper
macro_rules! global_attribute {
    ($attr:ident) => {
        impl<'h> Elem<'h> {
            #[doc = "Add `"]
            #[doc = stringify!($attr)]
            #[doc = "` [attribute]("]
            #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/", stringify!($attr))]
            #[doc = ")"]
            pub fn $attr(self, val: impl AsRef<str>) -> Self {
                self.html.attr(stringify!($attr), val);
                self
            }
        }
        impl<'h> VoidElem<'h> {
            #[doc = "Add `"]
            #[doc = stringify!($attr)]
            #[doc = "` [attribute]("]
            #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/", stringify!($attr))]
            #[doc = ")"]
            pub fn $attr(self, val: impl AsRef<str>) -> Self {
                self.html.attr(stringify!($attr), val);
                self
            }
        }
    }
}

global_attribute!(class);
global_attribute!(id);

/// HTML attribute helper
macro_rules! attribute {
    ($attr:ident) => {
        impl<'h> Elem<'h> {
            #[doc = "Add `"]
            #[doc = stringify!($attr)]
            #[doc = "` [attribute]("]
            #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/", stringify!($attr))]
            #[doc = ")"]
            pub fn $attr(self, val: impl AsRef<str>) -> Self {
                self.html.attr(stringify!($attr), val);
                self
            }
        }
        impl<'h> VoidElem<'h> {
            #[doc = "Add `"]
            #[doc = stringify!($attr)]
            #[doc = "` [attribute]("]
            #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/", stringify!($attr))]
            #[doc = ")"]
            pub fn $attr(self, val: impl AsRef<str>) -> Self {
                self.html.attr(stringify!($attr), val);
                self
            }
        }
    }
}

attribute!(height);
attribute!(href);
attribute!(max);
attribute!(min);
attribute!(size);
attribute!(src);
attribute!(value);
attribute!(width);

/// HTML Boolean attribute helper
macro_rules! boolean_attribute {
    ($attr:ident) => {
        impl<'h> Elem<'h> {
            #[doc = "Add `"]
            #[doc = stringify!($attr)]
            #[doc = "` [attribute]("]
            #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/", stringify!($attr))]
            #[doc = ")"]
            pub fn $attr(self) -> Self {
                self.html.attr_bool(stringify!($attr));
                self
            }
        }
        impl<'h> VoidElem<'h> {
            #[doc = "Add `"]
            #[doc = stringify!($attr)]
            #[doc = "` [attribute]("]
            #[doc = concat!("https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes/", stringify!($attr))]
            #[doc = ")"]
            pub fn $attr(self) -> Self {
                self.html.attr_bool(stringify!($attr));
                self
            }
        }
    }
}

boolean_attribute!(disabled);

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
