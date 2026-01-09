_Hatmil_ is an HTML builder for Rust.  It can be used to create or modify web
pages dynamically.

With a [Page], there are two building methods:
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

#### Content

Text content (_character data_) can be added using the [cdata] or [cdata_len]
methods, which will automatically escape characters as needed.  For content
which has already been escaped, use the [raw] method.

#### Display

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


[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[close]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.close
[element]: https://docs.rs/hatmil/latest/hatmil/html/
[format]: https://doc.rust-lang.org/std/macro.format.html
[page]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html
[frag]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.frag
[html]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.html
[raw]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.raw
[cdata]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.cdata
[cdata_len]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.cdata_len
