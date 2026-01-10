_Hatmil_ is an HTML builder for Rust.  It can be used to create or modify web
pages dynamically, including [inline SVG].

With a [Page], there are two "root" methods:
- [html] for a full document
- [frag] for a fragment, starting from an arbitrary [element] (HTML or [SVG])

In either case, an element struct is returned which borrows from the `Page`.

Each element has methods for setting valid attributes, such as `id`.  There
are also methods for adding permitted child elements.

```rust
use hatmil::Page;

let mut page = Page::default();
page.html().body().p().id("my_div").cdata("Content");
assert_eq!(
    page.to_string(),
    "<html><body><p id=\"my_div\">Content</p></body></html>"
);
```

Text content (_character data_) can be added using the `cdata` or `cdata_len`
methods on an [element].  Special HTML characters will automatically be
replaced by [character reference]s, as needed (for content which has already
been escaped, use the [raw] method).  The [close] method can be used to close
the final open element.

After creating the page, use [Display] ([format], `to_string()`, etc) to
get the resulting HTML.  Any open tags will be closed automatically.

```rust
use hatmil::{Page, html::Div};

let mut page = Page::default();
let mut div = page.frag::<Div>();
div.button().class("rounded").cdata("Press Me!");
assert_eq!(
    page.to_string(),
    "<div><button class=\"rounded\">Press Me!</button></div>"
);
```

## Method Names

In most cases, element methods match the HTML tag exactly.  But due to clashes
with attribute names, some methods for creating child elements have an `_el`
suffix:

- `abbr_el` on [Th], clash with `abbr` attribute
- `cite_el` on [BlockQuote] and [Q], clash with `cite` attribute
- `form_el` on [FieldSet], clash with `form` attribute
- `slot_el` on many [element]s, clash with `slot` global attribute
- `style_el` on [Head], [NoScript] and [SVG] elements, clash with `style`
  global attribute
- `title_el` on [Head] and [SVG Style], clash with `title` global attribute

Some HTML names clash with Rust keywords.  In these cases, [raw identifiers]
must be used to call those methods:

- `r#as` on [Link]
- `r#async` on [Script]
- `r#for` on [Label] and [Output]
- `r#in` on multiple SVG filter elements
- `r#loop` on [Audio] and [Video]
- `r#type` on multiple HTML and [SVG] elements
- `r#use` (SVG element)


[Audio]: https://docs.rs/hatmil/latest/hatmil/html/struct.Audio.html
[BlockQuote]: https://docs.rs/hatmil/latest/hatmil/html/struct.BlockQuote.html
[cdata]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.cdata
[cdata_len]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.cdata_len
[character reference]: https://developer.mozilla.org/en-US/docs/Glossary/Character_reference
[close]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.close
[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[element]: https://docs.rs/hatmil/latest/hatmil/html/
[FieldSet]: https://docs.rs/hatmil/latest/hatmil/html/struct.FieldSet.html
[format]: https://doc.rust-lang.org/std/macro.format.html
[frag]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.frag
[Head]: https://docs.rs/hatmil/latest/hatmil/html/struct.Head.html
[html]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.html
[inline SVG]: https://developer.mozilla.org/en-US/docs/Web/SVG/Guides/SVG_in_HTML
[Label]: https://docs.rs/hatmil/latest/hatmil/html/struct.Label.html
[Link]: https://docs.rs/hatmil/latest/hatmil/html/struct.Link.html
[NoScript]: https://docs.rs/hatmil/latest/hatmil/html/struct.NoScript.html
[Output]: https://docs.rs/hatmil/latest/hatmil/html/struct.Output.html
[page]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html
[Q]: https://docs.rs/hatmil/latest/hatmil/html/struct.Q.html
[raw]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.raw
[raw identifiers]: https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html
[Script]: https://docs.rs/hatmil/latest/hatmil/html/struct.Script.html
[SVG]: https://docs.rs/hatmil/latest/hatmil/svg/
[SVG Style]: https://docs.rs/hatmil/latest/hatmil/svg/struct.Style.html
[Th]: https://docs.rs/hatmil/latest/hatmil/html/struct.Th.html
[Video]: https://docs.rs/hatmil/latest/hatmil/html/struct.Video.html
