//! A simple CLI tool for testing Minimad parsing.
//!
//! Run with: cargo run --example minimad-cli [markdown content]

use minimad::{CompositeStyle, Line, Options, parse_text};
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let md = if args.len() > 1 {
        fs::read_to_string(&args[1]).expect("Could not read file")
    } else {
        "- Test\n  - Nested \n  - Still nested \n    - One level deeper \n  - Again Nested "
            .to_string()
    };
    let text = parse_text(&md, Options::default());

    for (i, line) in text.lines.iter().enumerate() {
        match line {
            Line::Normal(composite) => {
                let style_name = match &composite.style {
                    CompositeStyle::Paragraph => "Paragraph",
                    CompositeStyle::Header(_n) => "Header",
                    CompositeStyle::ListItem(_d) => "ListItem",
                    CompositeStyle::Code => "Code",
                    CompositeStyle::Quote => "Quote",
                };
                let depth = match &composite.style {
                    CompositeStyle::ListItem(d) => *d,
                    _ => 0,
                };
                let content: String = composite.compounds.iter().map(|c| c.src).collect();
                println!("Line {}: [{}] depth={} '{}'", i, style_name, depth, content);
            }
            Line::TableRow(row) => {
                println!("Line {}: TableRow with {} cells", i, row.cells.len());
            }
            Line::TableRule(rule) => {
                println!("Line {}: TableRule {:?}", i, rule);
            }
            Line::CodeFence(composite) => {
                let content: String = composite.compounds.iter().map(|c| c.src).collect();
                println!("Line {}: CodeFence '{}'", i, content);
            }
            Line::HorizontalRule => {
                println!("Line {}: HorizontalRule", i);
            }
        }
    }
}
