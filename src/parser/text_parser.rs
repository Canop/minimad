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
            }
            _ => {
                lines.push(line);
            }
        }
    }
    Text { lines }
}

