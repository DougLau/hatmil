// content.rs
//
// Copyright (C) 2025  Douglas P Lau
//

/// Text method
macro_rules! text_methods {
    () => {
        /// Add text content
        ///
        ///  - `text`: Text content; these characters will be replaced with
        ///            entities:
        ///  <br> `&` &xrarr; `&amp;`
        ///  <br> `<` &xrarr; `&lt;`
        ///  <br> `>` &xrarr; `&gt;`
        pub fn text<'a, V>(&mut self, text: V) -> &mut Self
        where
            V: Into<Value<'a>>,
        {
            self.page.text(text);
            self
        }

        /// Add text content with a maximum character limit
        ///
        ///  - `text`: Text content; these characters will be replaced with
        ///            entities:
        ///  <br> `&` &xrarr; `&amp;`
        ///  <br> `<` &xrarr; `&lt;`
        ///  <br> `>` &xrarr; `&gt;`
        ///  - `len`: Maximum number of characters
        pub fn text_len<'a, V>(&mut self, text: V, len: usize) -> &mut Self
        where
            V: Into<Value<'a>>,
        {
            self.page.text_len(text, len);
            self
        }
    };
}

/// Comment method
macro_rules! comment_method {
    () => {
        /// Add a comment
        ///
        ///  - `com`: Comment text; these characters will be replaced with
        ///           entities:
        ///  <br> `-` &xrarr; `&hyphen;`
        ///  <br> `<` &xrarr; `&gt;`
        ///  <br> `>` &xrarr; `&lt;`
        pub fn comment<'v, V>(&mut self, com: V) -> &mut Self
        where
            V: Into<Value<'v>>,
        {
            self.page.comment(com);
            self
        }
    };
}

/// Create an element method
macro_rules! elem_method {
    ( $meth:ident, $elem:ident ) => {
        #[doc = concat!("Add `<", stringify!($meth), ">` child element")]
        pub fn $meth(self: &mut Self) -> $elem<'_> {
            $elem { page: self.page }
        }
    };

    ( $meth:ident, $elem:ident, $el:literal ) => {
        #[doc = concat!("Add `<", $el, ">` child element")]
        pub fn $meth(self: &mut Self) -> $elem<'_> {
            $elem { page: self.page }
        }
    };
}

/// Metadata content
macro_rules! metadata_content {
    () => {
        elem_method!(base, Base);
        elem_method!(link, Link);
        elem_method!(meta, Meta);
        elem_method!(noscript, NoScript);
        elem_method!(script, Script);
        elem_method!(style_elem, Style, "style");
        elem_method!(template, Template);
        elem_method!(title_elem, Title, "title");
        comment_method!();
    };
}

/// Flow content
macro_rules! flow_content {
    () => {
        text_methods!();
        elem_method!(a, A);
        elem_method!(abbr, Abbr);
        elem_method!(address, Address);
        elem_method!(article, Article);
        elem_method!(aside, Aside);
        elem_method!(audio, Audio);
        elem_method!(b, B);
        elem_method!(bdi, Bdi);
        elem_method!(bdo, Bdo);
        elem_method!(blockquote, BlockQuote);
        elem_method!(br, Br);
        elem_method!(button, Button);
        elem_method!(canvas, Canvas);
        elem_method!(cite, Cite);
        elem_method!(code, Code);
        elem_method!(data, Data);
        elem_method!(datalist, DataList);
        elem_method!(del, Del);
        elem_method!(details, Details);
        elem_method!(dfn, Dfn);
        elem_method!(dialog, Dialog);
        elem_method!(div, Div);
        elem_method!(dl, Dl);
        elem_method!(em, Em);
        elem_method!(embed, Embed);
        elem_method!(fieldset, FieldSet);
        elem_method!(figure, Figure);
        elem_method!(footer, Footer);
        elem_method!(form, Form);
        elem_method!(h1, H1);
        elem_method!(h2, H2);
        elem_method!(h3, H3);
        elem_method!(h4, H4);
        elem_method!(h5, H5);
        elem_method!(h6, H6);
        elem_method!(header, Header);
        elem_method!(hgroup, HGroup);
        elem_method!(hr, Hr);
        elem_method!(i, I);
        elem_method!(iframe, IFrame);
        elem_method!(img, Img);
        elem_method!(input, Input);
        elem_method!(ins, Ins);
        elem_method!(kbd, Kbd);
        elem_method!(label, Label);
        elem_method!(main, Main);
        elem_method!(map, Map);
        elem_method!(mark, Mark);
        // elem_method!(math, Math);
        elem_method!(menu, Menu);
        elem_method!(meter, Meter);
        elem_method!(nav, Nav);
        elem_method!(noscript, NoScript);
        elem_method!(object, Object);
        elem_method!(ol, Ol);
        elem_method!(output, Output);
        elem_method!(p, P);
        elem_method!(picture, Picture);
        elem_method!(pre, Pre);
        elem_method!(progress, Progress);
        elem_method!(q, Q);
        elem_method!(ruby, Ruby);
        elem_method!(s, S);
        elem_method!(samp, Samp);
        elem_method!(script, Script);
        elem_method!(search, Search);
        elem_method!(section, Section);
        elem_method!(select, Select);
        elem_method!(slot_elem, Slot, "slot");
        elem_method!(small, Small);
        elem_method!(span, Span);
        elem_method!(strong, Strong);
        elem_method!(sub, Sub);
        elem_method!(sup, Sup);
        // elem_method!(svg, Svg);
        elem_method!(table, Table);
        elem_method!(template, Template);
        elem_method!(textarea, TextArea);
        elem_method!(time, Time);
        elem_method!(u, U);
        elem_method!(ul, Ul);
        elem_method!(var, Var);
        elem_method!(video, Video);
        elem_method!(wbr, Wbr);
        comment_method!();
    };
}
