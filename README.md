Simple HTML builder

Elements can be created with methods of the same name:

* [a](struct.Html.html#method.a)
* [abbr](struct.Html.html#method.abbr)
* [address](struct.Html.html#method.address)

```rust
use hatmil::Html;

let mut html = Html::new();
html.div().id("a_div").text("Hello").end();
html.button().class("rounded").text("Press Me!");
assert_eq!(
    html.build(),
    "<div id=\"a_div\">Hello</div><button class=\"rounded\">Press Me!</button>"
);
```
