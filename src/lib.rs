/*!
This crate provides a *very* simple markdown parser.

Its main motivation was to be the basis of the [termimad](https://github.com/Canop/termimad) lib, which displays static and dynamic markdown snippets on a terminal without mixing the skin with the code.

It can be used on its own:

```rust
assert_eq!(
    Line::from("## a header with some **bold**!"),
    Line {
        style: LineStyle::Header(2),
        compounds: vec![
            Compound::raw_str("a header with some "),
            Compound::raw_str("bold").bold(),
            Compound::raw_str("!"),
        ]
    }
);
```
*/

mod compound;
mod line;
mod line_parser;

pub use compound::Compound;
pub use line::Line;
