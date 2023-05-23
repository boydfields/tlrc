use std::io::{self, BufWriter, Write};

use yansi::Style;

use crate::config::{OutputConfig, StyleConfig};
use crate::error::Result;

const TITLE: &str = "# ";
const DESC: &str = "> ";
const BULLET: &str = "- ";
const EXAMPLE: char = '`';

/// Highlight a substring between `from` and `to` inside `s`.
fn highlight_between(
    from: &str,
    to: &str,
    s: &str,
    style_normal: &Style,
    style_hl: &Style,
) -> String {
    let mut result = String::new();

    for (i, spl) in s.split(from).enumerate() {
        if from == to {
            // Only odd indexes contain the part to be highlighted
            if i % 2 == 0 {
                result.push_str(&style_normal.paint(spl).to_string());
            } else {
                result.push_str(&style_hl.paint(spl).to_string());
            }
        } else if spl.contains(to) {
            let mut spl2 = spl.split(to);

            result.push_str(&style_hl.paint(spl2.next().unwrap()).to_string());
            result.push_str(&style_normal.paint(spl2.next().unwrap()).to_string());
        } else {
            result.push_str(&style_normal.paint(spl).to_string());
        }
    }

    result
}

/// Print the given page to stdout.
pub fn print_page(
    page_string: &str,
    outputcfg: &OutputConfig,
    stylecfg: StyleConfig,
) -> Result<()> {
    if outputcfg.raw_markdown {
        write!(io::stdout(), "{page_string}")?;
        return Ok(());
    }
    let mut handle = BufWriter::new(io::stdout().lock());

    let title: Style = stylecfg.title.into();
    let desc: Style = stylecfg.description.into();
    let bullet: Style = stylecfg.bullet.into();
    let example: Style = stylecfg.example.into();
    let url: Style = stylecfg.url.into();
    let inline_code: Style = stylecfg.inline_code.into();
    let placeholder: Style = stylecfg.placeholder.into();

    for line in page_string.lines() {
        if outputcfg.show_title && line.starts_with(TITLE) {
            if !outputcfg.compact {
                writeln!(handle)?;
            }
            writeln!(
                handle,
                "  {}",
                title.paint(&line.strip_prefix(TITLE).unwrap())
            )?;
        } else if line.starts_with(DESC) {
            writeln!(
                handle,
                "  {}",
                highlight_between(
                    "`",
                    "`",
                    &highlight_between("<", ">", line.strip_prefix(DESC).unwrap(), &desc, &url),
                    &desc,
                    &inline_code
                )
            )?;
        } else if line.starts_with(BULLET) {
            writeln!(
                handle,
                "  {}",
                highlight_between(
                    "`",
                    "`",
                    line.strip_prefix(BULLET).unwrap(),
                    &bullet,
                    &inline_code,
                )
            )?;
        } else if line.starts_with(EXAMPLE) {
            writeln!(
                handle,
                "    {}",
                highlight_between(
                    "{{",
                    "}}",
                    line.strip_prefix(EXAMPLE)
                        .unwrap()
                        .strip_suffix(EXAMPLE)
                        .unwrap(),
                    &example,
                    &placeholder,
                )
            )?;
        } else if !outputcfg.compact && line.is_empty() {
            writeln!(handle)?;
        }
    }

    if !outputcfg.compact {
        writeln!(handle)?;
    }

    handle.flush()?;

    Ok(())
}
