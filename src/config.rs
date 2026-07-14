use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ColorScheme {
    pub user: String,
    pub host: String,
    pub label: String,
    pub value: String,
    pub separator_color: String,
    pub title_color: String,
    pub logo: String,
}

#[derive(Clone)]
pub struct InfoFlags {
    pub os: bool,
    pub host: bool,
    pub kernel: bool,
    pub uptime: bool,
    pub packages: bool,
    pub shell: bool,
    pub terminal: bool,
    pub de: bool,
    pub wm: bool,
    pub cpu: bool,
    pub gpu: bool,
    pub memory: bool,
    pub disk: bool,
    pub locale: bool,
    pub battery: bool,
    pub resolution: bool,
}

#[derive(Clone)]
pub struct LogoConfig {
    pub enabled: bool,
    pub distro: String,
    pub color_override: String,
}

#[derive(Clone)]
pub struct TitleConfig {
    pub enabled: bool,
    pub format: String,
}

#[derive(Clone)]
pub struct Config {
    pub colors: ColorScheme,
    pub info: InfoFlags,
    pub logo: LogoConfig,
    pub title: TitleConfig,
    pub separator: String,
    pub padding: usize,
    pub bold: bool,
    pub dim: bool,
    pub labels_uppercase: bool,
    pub labels_capitalize: bool,
    pub custom_lines: Vec<String>,
    pub color_mode: String,
    pub order: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            colors: ColorScheme {
                user: "cyan".into(),
                host: "cyan".into(),
                label: "cyan".into(),
                value: "white".into(),
                separator_color: "gray".into(),
                title_color: "cyan".into(),
                logo: "auto".into(),
            },
            info: InfoFlags {
                os: true,
                host: true,
                kernel: true,
                uptime: true,
                packages: true,
                shell: true,
                terminal: true,
                de: true,
                wm: true,
                cpu: true,
                gpu: true,
                memory: true,
                disk: true,
                locale: true,
                battery: true,
                resolution: true,
            },
            logo: LogoConfig {
                enabled: true,
                distro: "auto".into(),
                color_override: "auto".into(),
            },
            title: TitleConfig {
                enabled: true,
                format: "{user}@{host}".into(),
            },
            separator: "─".into(),
            padding: 2,
            bold: true,
            dim: false,
            labels_uppercase: false,
            labels_capitalize: true,
            custom_lines: Vec::new(),
            color_mode: "full".into(),
            order: vec![
                "os".into(), "host".into(), "kernel".into(), "uptime".into(),
                "packages".into(), "shell".into(), "terminal".into(), "de".into(),
                "wm".into(), "cpu".into(), "gpu".into(), "memory".into(),
                "disk".into(), "locale".into(), "battery".into(), "resolution".into(),
            ],
        }
    }
}

pub fn load() -> Config {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut cfg = Config::default();

    let mut config_path: Option<PathBuf> = None;
    let mut i = 0;
    while i < args.len() {
        if (args[i] == "--config" || args[i] == "-c") && i + 1 < args.len() {
            config_path = Some(PathBuf::from(&args[i + 1]));
            i += 2;
        } else if args[i] == "--dump-config" {
            print_default_config();
            std::process::exit(0);
        } else {
            i += 1;
        }
    }

    let path = config_path
        .or_else(|| env::var("VOIDFETCH_CONFIG").ok().map(PathBuf::from))
        .or_else(|| dirs_config().map(|p| p.join("config.css")))
        .or_else(|| Some(PathBuf::from("/etc/voidfetch/config.css")));

    if let Some(p) = path {
        if p.exists() {
            if let Ok(content) = fs::read_to_string(&p) {
                apply_css_config(&mut cfg, &content);
            }
        }
    }

    cfg
}

fn dirs_config() -> Option<PathBuf> {
    if let Ok(home) = env::var("HOME") {
        Some(PathBuf::from(home).join(".config").join("voidfetch"))
    } else {
        None
    }
}

fn parse_value(val: &str) -> String {
    val.trim().trim_matches('"').trim_matches('\'').trim_matches(';').trim().to_string()
}

fn parse_bool(val: &str) -> bool {
    matches!(val.trim().to_lowercase().as_str(), "true" | "1" | "yes" | "on" | "enabled")
}

fn parse_usize(val: &str) -> usize {
    val.trim().trim_matches(';').trim().parse().unwrap_or(0)
}

fn apply_css_config(cfg: &mut Config, content: &str) {
    let mut current_block = String::new();
    let mut properties: HashMap<String, String> = HashMap::new();
    let mut all_props: Vec<(String, String)> = Vec::new();

    let mut brace_depth = 0;
    let mut in_comment = false;
    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("/*") {
            in_comment = true;
        }
        if in_comment {
            if trimmed.contains("*/") {
                in_comment = false;
            }
            continue;
        }

        let clean_line = if let Some(pos) = trimmed.find("//") {
            &trimmed[..pos]
        } else {
            trimmed
        };

        if clean_line.starts_with('@') || (brace_depth == 0 && clean_line.contains('{')) {
            if let Some(name) = clean_line.split('{').next() {
                current_block = name.trim().trim_start_matches('@').to_lowercase();
            }
        }

        for ch in clean_line.chars() {
            match ch {
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth -= 1;
                    if brace_depth <= 0 {
                        for (k, v) in &properties {
                            all_props.push((format!("{}.{}", current_block, k), v.clone()));
                        }
                        properties.clear();
                        current_block.clear();
                        brace_depth = 0;
                    }
                }
                _ => {}
            }
        }

        if brace_depth > 0 {
            if let Some((key, val)) = clean_line.split_once(':') {
                let k = key.trim().to_lowercase();
                let v = parse_value(val);
                if !k.is_empty() && !v.is_empty() {
                    properties.insert(k, v);
                }
            }
        }

        if brace_depth == 0 && current_block.is_empty() {
            if let Some((key, val)) = clean_line.split_once(':') {
                let k = key.trim().to_lowercase();
                let v = parse_value(val);
                if !k.is_empty() && !v.is_empty() {
                    all_props.push((k, v));
                }
            }
        }
    }

    for (prop, val) in &all_props {
        match prop.as_str() {
            "colors.user" | "color.user" => cfg.colors.user = val.clone(),
            "colors.host" | "color.host" => cfg.colors.host = val.clone(),
            "colors.label" | "color.label" => cfg.colors.label = val.clone(),
            "colors.value" | "color.value" => cfg.colors.value = val.clone(),
            "colors.separator" | "color.separator" | "colors.separator-color" => cfg.colors.separator_color = val.clone(),
            "colors.title" | "color.title" => cfg.colors.title_color = val.clone(),
            "colors.logo" | "color.logo" => cfg.colors.logo = val.clone(),

            "info.os" => cfg.info.os = parse_bool(val),
            "info.host" => cfg.info.host = parse_bool(val),
            "info.kernel" => cfg.info.kernel = parse_bool(val),
            "info.uptime" => cfg.info.uptime = parse_bool(val),
            "info.packages" => cfg.info.packages = parse_bool(val),
            "info.shell" => cfg.info.shell = parse_bool(val),
            "info.terminal" => cfg.info.terminal = parse_bool(val),
            "info.de" | "info.desktop" => cfg.info.de = parse_bool(val),
            "info.wm" | "info.window-manager" => cfg.info.wm = parse_bool(val),
            "info.cpu" => cfg.info.cpu = parse_bool(val),
            "info.gpu" => cfg.info.gpu = parse_bool(val),
            "info.memory" | "info.mem" => cfg.info.memory = parse_bool(val),
            "info.disk" => cfg.info.disk = parse_bool(val),
            "info.locale" => cfg.info.locale = parse_bool(val),
            "info.battery" => cfg.info.battery = parse_bool(val),
            "info.resolution" | "info.res" => cfg.info.resolution = parse_bool(val),

            "logo.enabled" | "logo.show" => cfg.logo.enabled = parse_bool(val),
            "logo.distro" | "logo.name" => cfg.logo.distro = val.clone(),
            "logo.color" => cfg.logo.color_override = val.clone(),

            "title.enabled" | "title.show" => cfg.title.enabled = parse_bool(val),
            "title.format" => cfg.title.format = val.clone(),

            "separator" | "separator.char" | "separator.character" => cfg.separator = val.clone(),
            "padding" | "padding.left" | "margin" => cfg.padding = parse_usize(val),
            "bold" => cfg.bold = parse_bool(val),
            "dim" => cfg.dim = parse_bool(val),
            "labels.uppercase" | "labels.case" if val == "uppercase" => cfg.labels_uppercase = true,
            "labels.capitalize" | "labels.title-case" => {
                cfg.labels_uppercase = false;
                cfg.labels_capitalize = parse_bool(val);
            }
            "color-mode" | "color_mode" | "colors.mode" => cfg.color_mode = val.clone(),

            "custom.line" | "custom" => cfg.custom_lines.push(val.clone()),

            _ => {}
        }
    }
}

pub fn print_default_config() {
    println!(r#"/* voidfetch config - YES THIS IS CSS. DEAL WITH IT. */
/* put this at ~/.config/voidfetch/config.css */

/* ============================================ */
/*  voidfetch v{} - config                    */
/*  https://github.com/AstralZX/voidfetch     */
/* ============================================ */

/* --- global --- */
:root {{
    separator: "─";
    padding: 2;
    bold: true;
    dim: false;
    color-mode: full;
}}

/* --- colors --- */
@colors {{
    user: cyan;
    host: cyan;
    label: cyan;
    value: white;
    separator: gray;
    title: cyan;
    logo: auto;
}}

/* --- what to show --- */
@info {{
    os: true;
    host: true;
    kernel: true;
    uptime: true;
    packages: true;
    shell: true;
    terminal: true;
    de: true;
    wm: true;
    cpu: true;
    gpu: true;
    memory: true;
    disk: true;
    locale: true;
    battery: true;
    resolution: true;
}}

/* --- logo --- */
@logo {{
    enabled: true;
    distro: auto;
    color: auto;
}}

/* --- title (user@host) --- */
@title {{
    enabled: true;
    format: "{{user}}@{{host}}";
}}

/* --- label style --- */
@labels {{
    capitalize: true;
    uppercase: false;
}}

/* --- order of info fields --- */
@order {{
    os, host, kernel, uptime, packages,
    shell, terminal, de, wm,
    cpu, gpu, memory, disk,
    locale, battery, resolution;
}}

/* --- custom lines (add your own flair) --- */
@custom {{
    line: "════════════════════════════════";
    line: "  voidfetch - embrace the void";
    line: "════════════════════════════════";
}}

/* --- COLOR EXAMPLES ---
 *
 * Named colors:
 *   red, green, yellow, blue, magenta, cyan, white, gray,
 *   orange, pink, lime, violet, indigo, coral, salmon,
 *   gold, crimson, turquoise, aqua, purple, teal
 *
 * Hex colors:
 *   color: #ff6600;
 *
 * RGB:
 *   color: rgb(255, 102, 0);
 *
 * 256-color:
 *   color: 256(208);
 *
 * ANSI:
 *   color: ansi(3);
 *
 * Disable:
 *   color: none;
"#, env!("CARGO_PKG_VERSION"));
}
