
[![Chat on Miaou](https://miaou.dystroy.org/static/shields/room-fr.svg?v=1)](https://miaou.dystroy.org/3?Code_Croissants)

A *very* simple, non universal purpose, markdown parser.

Minimad can be used on its own but is first designed for the [termimad](https://github.com/Canop/termimad) lib, which displays static and dynamic markdown snippets on a terminal without mixing the skin with the code.

Usage:

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

