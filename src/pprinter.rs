use axum::http::{HeaderMap, HeaderValue};
use colored::*;
use serde_json::Value;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

pub struct PPrinter;

impl PPrinter {
    pub fn pretty_print(path: String, headers: &HeaderMap, body: String) {
        println!("{}", path.bold().on_bright_red());
        Self::pretty_print_headers(&headers);
        Self::pretty_print_body(
            headers
                .get("Content-Type")
                .unwrap_or(&HeaderValue::from_static(""))
                .to_str()
                .unwrap_or(""),
            &body,
        );
        Self::print_line_separator();
    }

    fn pretty_print_headers(headers: &HeaderMap) {
        // Calculate the maximum header name length for padding
        let max_header_length = headers
            .keys()
            .map(|name| name.as_str().len())
            .max()
            .unwrap_or(0);

        println!("{}", "HEADERS".blue().bold());
        for (name, value) in headers.iter() {
            let name_str = name.as_str();
            let padding = " ".repeat(max_header_length - name_str.len());
            println!(
                "{}{}: {}",
                name.as_str().green(),
                padding,
                value.to_str().unwrap_or("")
            );
        }
    }

    fn pretty_print_body(content_type: &str, body: &str) {
        println!("{}", "BODY".blue().bold());
        if content_type.contains("json") {
            match serde_json::from_str::<Value>(body) {
                Ok(json) => {
                    let formatted_json = serde_json::to_string_pretty(&json).unwrap();
                    let syntax_set = SyntaxSet::load_defaults_newlines();
                    let theme_set = ThemeSet::load_defaults();

                    let theme = &theme_set.themes["base16-ocean.dark"];
                    let syntax = syntax_set.find_syntax_by_extension("json").unwrap();

                    // Print the JSON content with syntax highlighting
                    let mut h = HighlightLines::new(syntax, theme);
                    for line in LinesWithEndings::from(&formatted_json) {
                        let ranges: Vec<(Style, &str)> =
                            h.highlight_line(line, &syntax_set).unwrap_or_default();
                        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                        println!("{}", escaped);
                    }
                }
                Err(_) => {
                    println!("(Invalid JSON)");
                }
            }
        } else {
            println!("{}", body);
        }
    }

    fn print_line_separator() {
        println!(
            "{}",
            "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
                .bold()
                .green()
        ); // Unicode line separator
    }
}
