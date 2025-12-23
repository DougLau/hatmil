Hatmil is a user-friendly HTML builder.

## Elements

With a [Page] builder, elements can be created using methods with a matching
name, such as [a], [body], [div], or [table].  These methods return a struct
which implements [Element], borrowing from the `Page`.  It can be closed with
the [end] method.  [Void elements] such as [img] and [input], do not need to
be closed.

## Attributes

Attributes are available as methods on each element type.

These characters in attribute values will be escaped:
 - `&` &xrarr; `&amp;`
 - `"` &xrarr; `&quot;`

## Content

Text content can be added using the [text] or [text_len] methods, which will
automatically escape characters as needed.  For content which has already been
escaped, use the [raw] method.

## Display

After creating the page, use [Display] ([format], `to_string()`, etc) to
get the resulting HTML.  Any open tags will be closed automatically.

```rust
use hatmil::{Page, elem::Div};

let mut page = Page::default();
page.frag::<Div>().id("a_div").text("Hello").end();
page.button().class("rounded").text("Press Me!");
assert_eq!(
    page.to_string(),
    "<div id=\"a_div\">Hello</div><button class=\"rounded\">Press Me!</button>"
);
```


[a]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.a
[body]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.body
[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[div]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.div
[Element]: https://docs.rs/hatmil/latest/hatmil/elem/trait.Element.html
[end]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.end
[format]: https://doc.rust-lang.org/std/macro.format.html
[page]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html
[img]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.img
[input]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.input
[raw]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.raw
[table]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.table
[text]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.text
[text_len]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.text_len
[Void elements]: https://developer.mozilla.org/en-US/docs/Glossary/Void_element
