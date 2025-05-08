use colored::*;
use std::{env, fmt::Display};

fn main() {
    let envars = env::vars();
    for (key, value) in envars {
        let line = Line::new("==>", &key, "=", &value);
        println!("{line}");
    }
}

struct Line {
    precedence: String,
    key: String,
    sep: String,
    value: String,
}

impl Line {
    fn new(precedence: &str, key: &str, sep: &str, value: &str) -> Self {
        Self {
            precedence: String::from(precedence),
            key: String::from(key),
            sep: String::from(sep),
            value: String::from(value),
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}{}{}",
            self.precedence.custom_color(CustomColor::new(110, 106, 241)).bold(),
            self.key.custom_color(CustomColor::new(42, 195, 222)),
            self.sep.custom_color(CustomColor::new(160, 160, 160)),
            self.value.custom_color(CustomColor::new(241, 163, 111)),
        )
    }
}
