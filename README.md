Hatmil is a user-friendly HTML builder.

With a [Page] builder, elements can be created using methods with a matching
name, such as [a], [body], [div], or [table].  These methods return an [Elem],
which borrows from the `Page`, and can be closed with the [end] method.
[VoidElem] elements, like [img] and [input], do not need to be closed.

Text content can be added using the [text] or [text_len] methods, which will
automatically escape characters as needed.  For content which has already been
escaped, use the [raw] method.

After creating all elements, use [Display] ([format], `to_string()`, etc) to
get the HTML.  All open tags will be closed automatically.

```rust
use hatmil::Page;

let mut frag = Page::frag();
frag.div().id("a_div").text("Hello").end();
frag.button().class("rounded").text("Press Me!");
assert_eq!(
    frag.to_string(),
    "<div id=\"a_div\">Hello</div><button class=\"rounded\">Press Me!</button>"
);
```

[a]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.a
[body]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.body
[Display]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[div]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.div
[Elem]: https://docs.rs/hatmil/latest/hatmil/struct.Elem.html
[end]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.end
[format]: https://doc.rust-lang.org/std/macro.format.html
[page]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html
[img]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.img
[input]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.input
[raw]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.raw
[table]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.table
[text]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.text
[text_len]: https://docs.rs/hatmil/latest/hatmil/struct.Page.html#method.text_len
[VoidElem]: https://docs.rs/hatmil/latest/hatmil/struct.VoidElem.html
