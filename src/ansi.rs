pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINE: &str = "\x1b[4m";
#[allow(dead_code)]
pub const BLINK: &str = "\x1b[5m";
#[allow(dead_code)]
pub const REVERSE: &str = "\x1b[7m";
#[allow(dead_code)]
pub const HIDDEN: &str = "\x1b[8m";
#[allow(dead_code)]
pub const STRIKETHROUGH: &str = "\x1b[9m";

pub fn color(name: &str) -> String {
    match name.to_lowercase().as_str() {
        "black" => "\x1b[30m".into(),
        "red" => "\x1b[31m".into(),
        "green" => "\x1b[32m".into(),
        "yellow" => "\x1b[33m".into(),
        "blue" => "\x1b[34m".into(),
        "magenta" | "purple" => "\x1b[35m".into(),
        "cyan" | "teal" => "\x1b[36m".into(),
        "white" | "lightgray" | "light-grey" => "\x1b[37m".into(),
        "gray" | "grey" | "darkgray" | "darkgrey" => "\x1b[90m".into(),
        "lightred" | "brightred" => "\x1b[91m".into(),
        "lightgreen" | "brightgreen" => "\x1b[92m".into(),
        "lightyellow" | "brightyellow" => "\x1b[93m".into(),
        "lightblue" | "brightblue" => "\x1b[94m".into(),
        "lightmagenta" | "brightmagenta" | "brightpurple" | "lightpurple" => "\x1b[95m".into(),
        "lightcyan" | "brightcyan" | "brightteal" | "lightteal" => "\x1b[96m".into(),
        "brightwhite" | "lightwhite" => "\x1b[97m".into(),

        "orange" => "\x1b[38;5;208m".into(),
        "pink" => "\x1b[38;5;213m".into(),
        "lime" => "\x1b[38;5;112m".into(),
        "violet" => "\x1b[38;5;135m".into(),
        "indigo" => "\x1b[38;5;63m".into(),
        "coral" => "\x1b[38;5;209m".into(),
        "salmon" => "\x1b[38;5;210m".into(),
        "gold" => "\x1b[38;5;220m".into(),
        "crimson" => "\x1b[38;5;196m".into(),
        "turquoise" => "\x1b[38;5;45m".into(),
        "aqua" => "\x1b[38;5;45m".into(),

        s if s.starts_with('#') && s.len() == 7 => {
            let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(0);
            format!("\x1b[38;2;{};{};{}m", r, g, b)
        }

        s if s.starts_with("rgb(") => {
            let inner = s.trim_start_matches("rgb(").trim_end_matches(')');
            let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();
            if parts.len() == 3 {
                let r = parts[0].parse::<u8>().unwrap_or(0);
                let g = parts[1].parse::<u8>().unwrap_or(0);
                let b = parts[2].parse::<u8>().unwrap_or(0);
                return format!("\x1b[38;2;{};{};{}m", r, g, b);
            }
            "\x1b[36m".into()
        }

        s if s.starts_with("ansi(") => {
            let inner = s.trim_start_matches("ansi(").trim_end_matches(')');
            let code: u8 = inner.parse().unwrap_or(0);
            format!("\x1b[3{}m", code)
        }

        s if s.starts_with("256(") => {
            let inner = s.trim_start_matches("256(").trim_end_matches(')');
            let code: u8 = inner.parse().unwrap_or(0);
            format!("\x1b[38;5;{}m", code)
        }

        "none" | "transparent" | "reset" => RESET.into(),
        _ => "\x1b[36m".into(),
    }
}

pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            while let Some(next) = chars.next() {
                if next == 'm' {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn visible_len(s: &str) -> usize {
    strip_ansi(s).chars().count()
}
