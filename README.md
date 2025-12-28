_Hatmil_ is an HTML builder for Rust.  It can be used to build or modify web
pages dynamically.

A [Page] builder can be used two ways:
1. The [html] method to build a full HTML document
2. The [frag] method can build a HTML fragment, starting from any element

In either case, an element struct is returned which borrows from the `Page`.

Each element struct has methods for setting every valid attribute, such as `id`.
They also have methods for adding all valid child elements.

```rust
use hatmil::Page;

let mut page = Page::default();
page.html().body().div().id("my_div").text("Content");
assert_eq!(
    page.to_string(),
    "<html><body><div id=\"my_div\">Content</div></body></html>"
);
```

#### Content

Text content (_character data_) can be added using the [text] or [text_len]
methods, which will automatically escape characters as needed.  For content
which has already been escaped, use the [raw] method.

#### Display

After creating the page, use [Display] ([format], `to_string()`, etc) to
get the resulting HTML.  Any open tags will be closed automatically.

```rust
use hatmil::{Page, html::Div};

let mut page = Page::default();
let mut div = page.frag::<Div>();
div.button().class("rounded").text("Press Me!").close();
assert_eq!(
    page.to_string(),
    "<div><button class=\"rounded\">Press Me!</button></div>"
);
```


[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[close]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.close
[format]: https://doc.rust-lang.org/std/macro.format.html
[page]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html
[frag]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.frag
[html]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.html
[raw]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.raw
[text]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.text
[text_len]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.text_len
