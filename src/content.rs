// content.rs
//
// Copyright (C) 2025  Douglas P Lau
//

/// Text method
macro_rules! text_methods {
    () => {
        /// Add text content
        ///
        /// These characters will be replaced with entities:
        ///
        /// | Char | Entity  |
        /// |------|---------|
        /// | `&`  | `&amp;` |
        /// | `<`  | `&lt;`  |
        /// | `>`  | `&gt;`  |
        pub fn text<'a, V>(&mut self, text: V) -> &mut Self
        where
            V: Into<Value<'a>>,
        {
            self.page.text(text);
            self
        }

        /// Add text content with a maximum character limit
        ///
        /// | Char | Entity  |
        /// |------|---------|
        /// | `&`  | `&amp;` |
        /// | `<`  | `&lt;`  |
        /// | `>`  | `&gt;`  |
        pub fn text_len<'a, V>(&mut self, text: V, len: usize) -> &mut Self
        where
            V: Into<Value<'a>>,
        {
            self.page.text_len(text, len);
            self
        }
    };
}

/// Comment and raw methods
macro_rules! comment_raw_methods {
    () => {
        /// Add a comment
        ///
        /// These characters will be replaced with entities:
        ///
        /// | Char | Entity     |
        /// |------|------------|
        /// | `-`  | `&hyphen;` |
        /// | `<`  | `&gt;`     |
        /// | `>`  | `&lt;`     |
        pub fn comment<'v, V>(&mut self, com: V) -> &mut Self
        where
            V: Into<Value<'v>>,
        {
            self.page.comment(com);
            self
        }

        /// Add raw content
        ///
        /// **WARNING**: `trusted` is used verbatim, with no escaping;
        ///              do not call with untrusted content.
        pub fn raw(&mut self, trusted: impl AsRef<str>) -> &mut Self {
            self.page.raw(trusted);
            self
        }
    };
}

/// Create an element method
macro_rules! elem_method {
    ( $meth:ident, $elem:ident ) => {
        #[doc = concat!("Add `<", stringify!($meth), ">` child element")]
        #[allow(clippy::self_named_constructors)]
        pub fn $meth(self: &mut Self) -> $elem<'_> {
            self.page.elem(stringify!($meth), false);
            $elem { page: self.page }
        }
    };

    ( $meth:ident, $elem:ident, $el:literal ) => {
        #[doc = concat!("Add `<", $el, ">` child element")]
        pub fn $meth(self: &mut Self) -> $elem<'_> {
            self.page.elem($el, false);
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
        comment_raw_methods!();
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
        elem_method!(cite_elem, Cite, "cite"); // FIXME: blockquote cite
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
        elem_method!(form_elem, Form, "form"); // FIXME: fieldset form
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
        comment_raw_methods!();
    };
}

/// Phrasing content
macro_rules! phrasing_content {
    () => {
        text_methods!();
        elem_method!(a, A); // FIXME: containing only phrasing content
        elem_method!(abbr, Abbr);
        elem_method!(area, Area); // FIXME: only descendants of <map>
        elem_method!(audio, Audio);
        elem_method!(b, B);
        elem_method!(bdi, Bdi);
        elem_method!(bdo, Bdo);
        elem_method!(br, Br);
        elem_method!(button, Button);
        elem_method!(canvas, Canvas);
        elem_method!(cite, Cite);
        elem_method!(code, Code);
        elem_method!(data, Data);
        elem_method!(datalist, DataList);
        elem_method!(del, Del); // FIXME: containing only phrasing content
        elem_method!(dfn, Dfn);
        elem_method!(em, Em);
        elem_method!(embed, Embed);
        elem_method!(i, I);
        elem_method!(iframe, IFrame);
        elem_method!(img, Img);
        elem_method!(input, Input);
        elem_method!(ins, Ins); // FIXME: containing only phrasing content
        elem_method!(kbd, Kbd);
        elem_method!(label, Label);
        elem_method!(link, Link); // FIXME: must have itemprop attribute
        elem_method!(map, Map); // FIXME: containing only phrasing content
        elem_method!(mark, Mark);
        // elem_method!(math, Math);
        elem_method!(meta, Meta); // FIXME: must have itemprop attribute
        elem_method!(meter, Meter);
        elem_method!(noscript, NoScript);
        elem_method!(object, Object);
        elem_method!(output, Output);
        elem_method!(picture, Picture);
        elem_method!(progress, Progress);
        elem_method!(q, Q);
        elem_method!(ruby, Ruby);
        elem_method!(s, S);
        elem_method!(samp, Samp);
        elem_method!(script, Script);
        elem_method!(select, Select);
        elem_method!(slot_elem, Slot, "slot");
        elem_method!(small, Small);
        elem_method!(span, Span);
        elem_method!(strong, Strong);
        elem_method!(sub, Sub);
        elem_method!(sup, Sup);
        // elem_method!(svg, Svg);
        elem_method!(template, Template);
        elem_method!(textarea, TextArea);
        elem_method!(time, Time);
        elem_method!(u, U);
        elem_method!(var, Var);
        elem_method!(video, Video);
        elem_method!(wbr, Wbr);
        comment_raw_methods!();
    };
}

/// Non-interactive phrasing content
macro_rules! non_interactive_phrasing_content {
    () => {
        text_methods!();
        // a with href attribute is interactive
        elem_method!(abbr, Abbr);
        elem_method!(area, Area); // FIXME: only descendants of <map>
        // audio with controls attribute is interactive
        elem_method!(b, B);
        elem_method!(bdi, Bdi);
        elem_method!(bdo, Bdo);
        elem_method!(br, Br);
        // button is interactive
        elem_method!(canvas, Canvas);
        elem_method!(cite, Cite);
        elem_method!(code, Code);
        elem_method!(data, Data);
        elem_method!(datalist, DataList);
        elem_method!(del, Del); // FIXME: containing only phrasing content
        elem_method!(dfn, Dfn);
        elem_method!(em, Em);
        // embed is interactive
        elem_method!(i, I);
        // iframe is interactive
        elem_method!(img, Img); // with usemap attribute is interactive
        // input is interactive (if not hidden)
        elem_method!(ins, Ins); // FIXME: containing only phrasing content
        elem_method!(kbd, Kbd);
        // label is interactive
        elem_method!(link, Link); // FIXME: must have itemprop attribute
        elem_method!(map, Map); // FIXME: containing only phrasing content
        elem_method!(mark, Mark);
        // elem_method!(math, Math);
        elem_method!(meta, Meta); // FIXME: must have itemprop attribute
        elem_method!(meter, Meter);
        elem_method!(noscript, NoScript);
        elem_method!(object, Object); // with usemap attribute is interactive
        elem_method!(output, Output);
        elem_method!(picture, Picture);
        elem_method!(progress, Progress);
        elem_method!(q, Q);
        elem_method!(ruby, Ruby);
        elem_method!(s, S);
        elem_method!(samp, Samp);
        elem_method!(script, Script);
        // select is interactive
        elem_method!(slot_elem, Slot, "slot");
        elem_method!(small, Small);
        elem_method!(span, Span);
        elem_method!(strong, Strong);
        elem_method!(sub, Sub);
        elem_method!(sup, Sup);
        // elem_method!(svg, Svg);
        elem_method!(template, Template);
        // textarea is interactive
        elem_method!(time, Time);
        elem_method!(u, U);
        elem_method!(var, Var);
        // video with controls attribute is interactive
        elem_method!(wbr, Wbr);
        comment_raw_methods!();
    };
}

/// Text content
macro_rules! text_content {
    () => {
        text_methods!();
        comment_raw_methods!();
    };
}

/// Address content (flow, with some restrictions)
macro_rules! address_content {
    () => {
        text_methods!();
        elem_method!(a, A);
        elem_method!(abbr, Abbr);
        // address not allowed
        // article not allowed
        // aside not allowed
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
        // footer not allowed
        elem_method!(form, Form);
        // h1 - h6 not allowed
        // header not allowed
        // hgroup not allowed
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
        // nav not allowed
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
        // section not allowed
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
        comment_raw_methods!();
    };
}
