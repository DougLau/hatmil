_Hatmil_ is an HTML builder for Rust.  It can be used to create or modify web
pages dynamically.

With a [Page], there are two "root" methods:
- [html] for a full document
- [frag] for a fragment, starting from an arbitrary [element]

In either case, an [element] struct is returned which borrows from the `Page`.

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

Text content (_character data_) can be added using the [cdata] or [cdata_len]
methods, which will automatically escape characters as needed.  For content
which has already been escaped, use the [raw] method.

After creating the page, use [Display] ([format], `to_string()`, etc) to
get the resulting HTML.  Any open tags will be closed automatically.

```rust
use hatmil::{Page, html::Div};

let mut page = Page::default();
let mut div = page.frag::<Div>();
div.button().class("rounded").cdata("Press Me!").close();
assert_eq!(
    page.to_string(),
    "<div><button class=\"rounded\">Press Me!</button></div>"
);
```

## Method Names

In most cases, element methods match the HTML tag exactly.  But due to clashes
with attribute names, some methods for creating child elements have an `_el`
suffix:

- `slot_el` on [flow elements], clash with `slot` global attribute
- `style_el` on [flow elements], clash with `style` global attribute
- `title_el` on [flow elements], clash with `title` global attribute
- `abbr_el` on [Th], clash with `abbr` attribute
- `cite_el` on [BlockQuote], clash with `cite` attribute
- `form_el` on [FieldSet], clash with `form` attribute

Some HTML names clash with Rust keywords.  In these cases, [raw identifiers]
must be used to call those methods:

- `r#as` on [Link]
- `r#async` on [Script]
- `r#for` on [Label] and [Output]
- `r#in` on multiple SVG filter elements
- `r#loop` on [Audio] and [Video]
- `r#type` on multiple HTML and SVG elements
- `r#use` (SVG element)


[Audio]: https://docs.rs/hatmil/latest/hatmil/html/struct.Audio.html
[BlockQuote]: https://docs.rs/hatmil/latest/hatmil/html/struct.BlockQuote.html
[cdata]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.cdata
[cdata_len]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.cdata_len
[close]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.close
[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[element]: https://docs.rs/hatmil/latest/hatmil/html/
[FieldSet]: https://docs.rs/hatmil/latest/hatmil/html/struct.FieldSet.html
[flow elements]: https://developer.mozilla.org/en-US/docs/Web/HTML/Guides/Content_categories#flow_content
[format]: https://doc.rust-lang.org/std/macro.format.html
[frag]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.frag
[html]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.html
[Label]: https://docs.rs/hatmil/latest/hatmil/html/struct.Label.html
[Link]: https://docs.rs/hatmil/latest/hatmil/html/struct.Link.html
[Output]: https://docs.rs/hatmil/latest/hatmil/html/struct.Output.html
[page]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html
[raw]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.raw
[raw identifiers]: https://doc.rust-lang.org/rust-by-example/compatibility/raw_identifiers.html
[Script]: https://docs.rs/hatmil/latest/hatmil/html/struct.Script.html
[Th]: https://docs.rs/hatmil/latest/hatmil/html/struct.Th.html
[Video]: https://docs.rs/hatmil/latest/hatmil/html/struct.Video.html
