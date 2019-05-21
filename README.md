

[![MIT][s2]][l2] [![Latest Version][s1]][l1] [![docs][s3]][l3] [![Chat on Miaou][s4]][l4]

[s1]: https://img.shields.io/crates/v/minimad.svg
[l1]: https://crates.io/crates/minimad

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: minimad/LICENSE

[s3]: https://docs.rs/minimad/badge.svg
[l3]: https://docs.rs/minimad/

[s4]: https://miaou.dystroy.org/static/shields/room.svg
[l4]: https://miaou.dystroy.org/3

A *very* simple, non universal purpose, markdown parser.

If you're looking for a Markdown parser, this one is probably *not* the one you want.

Minimad can be used on its own but is first designed for the [termimad](https://github.com/Canop/termimad) lib, which displays static and dynamic markdown snippets on a terminal without mixing the skin with the code.

### Usage


```toml
[dependencies]
minimad = "0.2"
```

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

assert_eq!(
    Line::from("*Italic then **bold and italic `and some *code*`** and italic*"),
    Line {
	style: LineStyle::Normal,
	compounds: vec![
	    Compound::raw_str("Italic then ").italic(),
	    Compound::raw_str("bold and italic ").bold().italic(),
	    Compound::raw_str("and some *code*").bold().italic().code(),
	    Compound::raw_str(" and italic").italic(),
	]
    }
);
```

