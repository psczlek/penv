use std::{env, fmt::Display};

use colored::{Colorize, CustomColor};

fn main() {
    for (key, value) in env::vars() {
        let line = Line::new("==>", &key, "=", &value);
        println!("{line}");
    }
}

struct Line<'a> {
    precedence: &'a str,
    key: &'a str,
    sep: &'a str,
    value: &'a str,
    is_truecolor_term: bool,
}

impl<'a> Line<'a> {
    fn new(precedence: &'a str, key: &'a str, sep: &'a str, value: &'a str) -> Self {
        Self {
            precedence: precedence,
            key: key,
            sep: sep,
            value: value,
            is_truecolor_term: env::var("COLORTERM")
                .map(|colorterm| colorterm == "truecolor" || colorterm == "24bit")
                .unwrap_or(false),
        }
    }
}

impl<'a> Display for Line<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.is_truecolor_term {
            false => write!(
                f,
                "{} {}{}{}",
                self.precedence.blue().bold(),
                self.key.cyan(),
                self.sep,
                self.value.yellow(),
            ),
            true => write!(
                f,
                "{} {}{}{}",
                self.precedence
                    .custom_color(CustomColor::new(110, 106, 241))
                    .bold(),
                self.key.custom_color(CustomColor::new(42, 195, 222)),
                self.sep.custom_color(CustomColor::new(160, 160, 160)),
                self.value.custom_color(CustomColor::new(241, 163, 111)),
            ),
        }
    }
}
