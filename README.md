Simple HTML builder

```rust
use hatmil::Html;

let mut html = Html::new();
html.div().id("a_div").text("Hello").end();
html.button().class("rounded").text("Press Me!");
assert_eq!(
    String::from(html),
    "<div id=\"a_div\">Hello</div><button class=\"rounded\">Press Me!</button>"
);
```
