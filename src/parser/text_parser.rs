use crate::*;

/// Parse a markdown string into a text
pub fn parse(
    md: &str,
    options: Options,
) -> Text<'_> {
    if options.clean_indentations {
        parse_lines(clean::lines(md).into_iter(), options)
    } else {
        parse_lines(md.lines(), options)
    }
}

/// Parse lines
pub(crate) fn parse_lines<'s, I>(
    md_lines: I,
    options: Options,
) -> Text<'s>
where
    I: Iterator<Item = &'s str>,
{
    let mut lines = Vec::new();
    let mut between_fences = false;
    let mut continue_code = false;
    let mut continue_italic = false;
    let mut continue_bold = false;
    let mut continue_strikeout = false;
    for md_line in md_lines {
        let mut line_parser = parser::LineParser::from(md_line);
        let line = if between_fences {
            continue_code = false;
            continue_italic = false;
            continue_bold = false;
            continue_strikeout = false;
            line_parser.as_code()
        } else {
            if continue_code {
                line_parser.code = true;
            }
            if continue_italic {
                line_parser.italic = true;
            }
            if continue_bold {
                line_parser.bold = true;
            }
            if continue_strikeout {
                line_parser.strikeout = true;
            }
            let line = line_parser.parse_line();
            continue_code = options.continue_inline_code && line_parser.code;
            continue_italic = options.continue_italic && line_parser.italic;
            continue_bold = options.continue_bold && line_parser.bold;
            continue_strikeout = options.continue_strikeout && line_parser.strikeout;
            line
        };
        match line {
            Line::CodeFence(..) => {
                between_fences = !between_fences;
                if options.keep_code_fences {
                    lines.push(line);
                }
            }
            _ => {
                lines.push(line);
            }
        }
    }
    fix_ordered_indexes(&mut lines);
    Text { lines }
}

/// Renumber the indexes of consecutive OrderedListItem lines so that ordered
/// lists read as a text have consistent numbering, starting from the index of
/// the first item in each run.
fn fix_ordered_indexes(lines: &mut Vec<Line<'_>>) {
    #[derive(Default)]
    struct Seq {
        start: u32,
        count: u32,
        active: bool,
    }
    let mut seqs: Vec<Seq> = Vec::new();
    for line in lines.iter_mut() {
        if let Line::Normal(Composite {
            style: CompositeStyle::OrderedListItem { level, index },
            ..
        }) = line
        {
            let level = *level as usize;
            while seqs.len() <= level {
                seqs.push(Seq::default());
            }
            if seqs[level].active {
                seqs[level].count += 1;
            } else {
                seqs[level].start = *index;
                seqs[level].count = 1;
                seqs[level].active = true;
                // deeper sequences are no longer part of the current run
                for s in seqs.iter_mut().skip(level + 1) {
                    s.active = false;
                }
            }
            *index = seqs[level].start + seqs[level].count - 1;
        } else {
            for s in seqs.iter_mut() {
                s.active = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn assert_ordered(
        line: &Line,
        level: u8,
        index: u32,
    ) {
        match line {
            Line::Normal(Composite {
                style: CompositeStyle::OrderedListItem { level: l, index: i },
                ..
            }) => {
                assert_eq!(*l, level);
                assert_eq!(*i, index);
            }
            _ => panic!("expected ordered list item, got {:?}", line),
        }
    }

    #[test]
    fn renumbers_repeated_one_markers() {
        let text = parse_text("1. a\n1. b\n1. c", Options::default());
        assert_eq!(text.lines.len(), 3);
        assert_ordered(&text.lines[0], 0, 1);
        assert_ordered(&text.lines[1], 0, 2);
        assert_ordered(&text.lines[2], 0, 3);
    }

    #[test]
    fn keeps_ascending_start_indexes() {
        let text = parse_text("2. a\n3. b\n4. c", Options::default());
        assert_eq!(text.lines.len(), 3);
        assert_ordered(&text.lines[0], 0, 2);
        assert_ordered(&text.lines[1], 0, 3);
        assert_ordered(&text.lines[2], 0, 4);
    }

    #[test]
    fn resets_ordered_index_after_break() {
        let text = parse_text("1. a\n1. b\n\n1. c", Options::default());
        assert_eq!(text.lines.len(), 4);
        assert_ordered(&text.lines[0], 0, 1);
        assert_ordered(&text.lines[1], 0, 2);
        assert_ordered(&text.lines[3], 0, 1);
    }

    #[test]
    fn handles_nested_ordered_lists() {
        let text = parse_text("1. a\n 1. inner\n 2. inner\n2. b", Options::default());
        assert_eq!(text.lines.len(), 4);
        assert_ordered(&text.lines[0], 0, 1);
        assert_ordered(&text.lines[1], 1, 1);
        assert_ordered(&text.lines[2], 1, 2);
        assert_ordered(&text.lines[3], 0, 2);
    }
}
