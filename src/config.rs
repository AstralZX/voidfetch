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
    #[allow(dead_code)]
    pub order: Vec<String>,
    pub font: String,
    pub variables: HashMap<String, String>,
    pub layout: String,
    pub italic: bool,
    pub underline: bool,
    pub glow: bool,
    pub style: String,
    #[allow(dead_code)]
    pub palettes: HashMap<String, Vec<String>>,
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
            font: "default".into(),
            variables: HashMap::new(),
            layout: "side".into(),
            italic: false,
            underline: false,
            glow: false,
            style: "full".into(),
            palettes: HashMap::new(),
        }
    }
}

pub fn load() -> Config {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut cfg = Config::default();

    let mut config_path: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        if (args[i] == "--config" || args[i] == "-c") && i + 1 < args.len() {
            config_path = Some(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }

    let path = config_path
        .or_else(|| env::var("VOIDFETCH_CONFIG").ok())
        .or_else(|| {
            dirs_config().map(|p| p.join("config.css").to_string_lossy().to_string())
        })
        .or_else(|| Some("/etc/voidfetch/config.css".to_string()));

    if let Some(p) = path {
        let pb = PathBuf::from(&p);
        if pb.exists() {
            if let Ok(content) = fs::read_to_string(&pb) {
                apply_css_config(&mut cfg, &content, &pb);
            }
        }
    }

    cfg
}

pub fn load_with_path(path: &str) -> Config {
    let mut cfg = Config::default();
    let pb = PathBuf::from(path);
    if pb.exists() {
        if let Ok(content) = fs::read_to_string(&pb) {
            apply_css_config(&mut cfg, &content, &pb);
        }
    } else {
        eprintln!("\x1b[33m[!]\x1b[0m config not found: {}", path);
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

fn find_examples_dir() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        let local = PathBuf::from(&home).join(".local").join("bin").join("examples");
        if local.is_dir() {
            let has_files = fs::read_dir(&local)
                .map(|mut r| r.next().is_some())
                .unwrap_or(false);
            if has_files {
                return local;
            }
        }
    }

    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    let candidates = [
        exe_dir.join("examples"),
        exe_dir.join("..").join("examples"),
        exe_dir.join("..").join("..").join("examples"),
        exe_dir.join("../share/voidfetch/examples"),
    ];

    for c in &candidates {
        if c.is_dir() {
            let has_files = fs::read_dir(c)
                .map(|mut r| r.next().is_some())
                .unwrap_or(false);
            if has_files {
                return c.clone();
            }
        }
    }

    if let Ok(cwd) = env::current_dir() {
        let cwd_examples = cwd.join("examples");
        if cwd_examples.is_dir() {
            let has_files = fs::read_dir(&cwd_examples)
                .map(|mut r| r.next().is_some())
                .unwrap_or(false);
            if has_files {
                return cwd_examples;
            }
        }
    }

    exe_dir.join("examples")
}

fn resolve_variables(content: &str, variables: &HashMap<String, String>) -> String {
    let mut result = content.to_string();
    for _ in 0..5 {
        let mut changed = false;
        for (key, value) in variables {
            let placeholder = format!("${{{}}}", key);
            if result.contains(&placeholder) {
                result = result.replace(&placeholder, value);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    result
}

fn apply_theme(cfg: &mut Config, theme_name: &str) {
    match theme_name.to_lowercase().as_str() {
        "arctic" | "arctic-frost" => {
            cfg.colors.user = "#88c0d0".into();
            cfg.colors.host = "#81a1c1".into();
            cfg.colors.label = "#88c0d0".into();
            cfg.colors.value = "#eceff4".into();
            cfg.colors.separator_color = "#4c566a".into();
            cfg.colors.title_color = "#88c0d0".into();
        }
        "sunset" | "sunset-fire" => {
            cfg.colors.user = "#ff6b6b".into();
            cfg.colors.host = "#ffa500".into();
            cfg.colors.label = "#ff6b6b".into();
            cfg.colors.value = "#fff5e6".into();
            cfg.colors.separator_color = "#cc5500".into();
            cfg.colors.title_color = "#ff6b6b".into();
        }
        "neon" | "neon-cyberpunk" => {
            cfg.colors.user = "#ff00ff".into();
            cfg.colors.host = "#00ffff".into();
            cfg.colors.label = "#ff00ff".into();
            cfg.colors.value = "#ffffff".into();
            cfg.colors.separator_color = "#ffff00".into();
            cfg.colors.title_color = "#ff00ff".into();
        }
        "dracula" => {
            cfg.colors.user = "#bd93f9".into();
            cfg.colors.host = "#ff79c6".into();
            cfg.colors.label = "#bd93f9".into();
            cfg.colors.value = "#f8f8f2".into();
            cfg.colors.separator_color = "#6272a4".into();
            cfg.colors.title_color = "#bd93f9".into();
        }
        "tokyo" | "tokyo-night" => {
            cfg.colors.user = "#7aa2f7".into();
            cfg.colors.host = "#bb9af7".into();
            cfg.colors.label = "#7aa2f7".into();
            cfg.colors.value = "#c0caf5".into();
            cfg.colors.separator_color = "#414868".into();
            cfg.colors.title_color = "#7aa2f7".into();
        }
        "gruvbox" | "gruvbox-dark" => {
            cfg.colors.user = "#fabd2f".into();
            cfg.colors.host = "#fe8019".into();
            cfg.colors.label = "#fabd2f".into();
            cfg.colors.value = "#ebdbb2".into();
            cfg.colors.separator_color = "#928374".into();
            cfg.colors.title_color = "#fabd2f".into();
        }
        "catppuccin" | "catppuccin-mocha" => {
            cfg.colors.user = "#cba6f7".into();
            cfg.colors.host = "#f5c2e7".into();
            cfg.colors.label = "#cba6f7".into();
            cfg.colors.value = "#cdd6f4".into();
            cfg.colors.separator_color = "#585b70".into();
            cfg.colors.title_color = "#cba6f7".into();
        }
        "monokai" | "monokai-pro" => {
            cfg.colors.user = "#fc9867".into();
            cfg.colors.host = "#a9dc76".into();
            cfg.colors.label = "#fc9867".into();
            cfg.colors.value = "#fcfcfa".into();
            cfg.colors.separator_color = "#727072".into();
            cfg.colors.title_color = "#fc9867".into();
        }
        "nord" => {
            cfg.colors.user = "#88c0d0".into();
            cfg.colors.host = "#8fbcbb".into();
            cfg.colors.label = "#88c0d0".into();
            cfg.colors.value = "#eceff4".into();
            cfg.colors.separator_color = "#4c566a".into();
            cfg.colors.title_color = "#88c0d0".into();
        }
        "onedark" | "one-dark" => {
            cfg.colors.user = "#61afef".into();
            cfg.colors.host = "#c678dd".into();
            cfg.colors.label = "#61afef".into();
            cfg.colors.value = "#abb2bf".into();
            cfg.colors.separator_color = "#5c6370".into();
            cfg.colors.title_color = "#61afef".into();
        }
        "rosepine" | "rose-pine" => {
            cfg.colors.user = "#ebbcba".into();
            cfg.colors.host = "#c4a7e7".into();
            cfg.colors.label = "#ebbcba".into();
            cfg.colors.value = "#e0def4".into();
            cfg.colors.separator_color = "#908caa".into();
            cfg.colors.title_color = "#ebbcba".into();
        }
        "solarized" | "solarized-dark" => {
            cfg.colors.user = "#268bd2".into();
            cfg.colors.host = "#2aa198".into();
            cfg.colors.label = "#268bd2".into();
            cfg.colors.value = "#839496".into();
            cfg.colors.separator_color = "#586e75".into();
            cfg.colors.title_color = "#268bd2".into();
        }
        "github" | "github-dark" => {
            cfg.colors.user = "#58a6ff".into();
            cfg.colors.host = "#d2a8ff".into();
            cfg.colors.label = "#58a6ff".into();
            cfg.colors.value = "#c9d1d9".into();
            cfg.colors.separator_color = "#484f58".into();
            cfg.colors.title_color = "#58a6ff".into();
        }
        "palenight" => {
            cfg.colors.user = "#c792ea".into();
            cfg.colors.host = "#82aaff".into();
            cfg.colors.label = "#c792ea".into();
            cfg.colors.value = "#a6accd".into();
            cfg.colors.separator_color = "#39adb5".into();
            cfg.colors.title_color = "#c792ea".into();
        }
        "matrix" | "matrix-green" => {
            cfg.colors.user = "#00ff00".into();
            cfg.colors.host = "#00cc00".into();
            cfg.colors.label = "#00ff00".into();
            cfg.colors.value = "#00ff00".into();
            cfg.colors.separator_color = "#009900".into();
            cfg.colors.title_color = "#00ff00".into();
        }
        "vaporwave" => {
            cfg.colors.user = "#ff71ce".into();
            cfg.colors.host = "#01cdfe".into();
            cfg.colors.label = "#ff71ce".into();
            cfg.colors.value = "#05ffa1".into();
            cfg.colors.separator_color = "#b967ff".into();
            cfg.colors.title_color = "#ff71ce".into();
        }
        "retro" | "retro-terminal" => {
            cfg.colors.user = "#33ff00".into();
            cfg.colors.host = "#33ff00".into();
            cfg.colors.label = "#33ff00".into();
            cfg.colors.value = "#33ff00".into();
            cfg.colors.separator_color = "#009900".into();
            cfg.colors.title_color = "#33ff00".into();
        }
        "void" | "void-purple" => {
            cfg.colors.user = "#478061".into();
            cfg.colors.host = "#96a7c9".into();
            cfg.colors.label = "#478061".into();
            cfg.colors.value = "#ffffff".into();
            cfg.colors.separator_color = "#96a7c9".into();
            cfg.colors.title_color = "#478061".into();
        }
        "sakura" | "sakura-pink" => {
            cfg.colors.user = "#ffb7c5".into();
            cfg.colors.host = "#ff69b4".into();
            cfg.colors.label = "#ffb7c5".into();
            cfg.colors.value = "#fff0f5".into();
            cfg.colors.separator_color = "#db7093".into();
            cfg.colors.title_color = "#ffb7c5".into();
        }
        "blood" | "blood-moon" => {
            cfg.colors.user = "#dc143c".into();
            cfg.colors.host = "#8b0000".into();
            cfg.colors.label = "#dc143c".into();
            cfg.colors.value = "#ffe4e1".into();
            cfg.colors.separator_color = "#8b0000".into();
            cfg.colors.title_color = "#dc143c".into();
        }
        "ocean" | "midnight-ocean" => {
            cfg.colors.user = "#00ced1".into();
            cfg.colors.host = "#20b2aa".into();
            cfg.colors.label = "#00ced1".into();
            cfg.colors.value = "#e0ffff".into();
            cfg.colors.separator_color = "#2f4f4f".into();
            cfg.colors.title_color = "#00ced1".into();
        }
        "forest" => {
            cfg.colors.user = "#228b22".into();
            cfg.colors.host = "#32cd32".into();
            cfg.colors.label = "#228b22".into();
            cfg.colors.value = "#f0fff0".into();
            cfg.colors.separator_color = "#006400".into();
            cfg.colors.title_color = "#228b22".into();
        }
        "lavender" | "lavender-dreams" => {
            cfg.colors.user = "#e6e6fa".into();
            cfg.colors.host = "#d8bfd8".into();
            cfg.colors.label = "#e6e6fa".into();
            cfg.colors.value = "#fffff0".into();
            cfg.colors.separator_color = "#9370db".into();
            cfg.colors.title_color = "#e6e6fa".into();
        }
        "amber" | "amber-glow" => {
            cfg.colors.user = "#ffbf00".into();
            cfg.colors.host = "#ff8c00".into();
            cfg.colors.label = "#ffbf00".into();
            cfg.colors.value = "#fffacd".into();
            cfg.colors.separator_color = "#ff6347".into();
            cfg.colors.title_color = "#ffbf00".into();
        }
        "emerald" | "emerald-sea" => {
            cfg.colors.user = "#50c878".into();
            cfg.colors.host = "#00fa9a".into();
            cfg.colors.label = "#50c878".into();
            cfg.colors.value = "#f0fff0".into();
            cfg.colors.separator_color = "#2e8b57".into();
            cfg.colors.title_color = "#50c878".into();
        }
        "ice" | "ice-blue" => {
            cfg.colors.user = "#add8e6".into();
            cfg.colors.host = "#b0e0e6".into();
            cfg.colors.label = "#add8e6".into();
            cfg.colors.value = "#f0f8ff".into();
            cfg.colors.separator_color = "#4682b4".into();
            cfg.colors.title_color = "#add8e6".into();
        }
        "pastel" | "pastel-dream" => {
            cfg.colors.user = "#ffb3ba".into();
            cfg.colors.host = "#bae1ff".into();
            cfg.colors.label = "#ffb3ba".into();
            cfg.colors.value = "#ffffba".into();
            cfg.colors.separator_color = "#baffc9".into();
            cfg.colors.title_color = "#ffb3ba".into();
        }
        "crimson" | "crimson-tide" => {
            cfg.colors.user = "#b22222".into();
            cfg.colors.host = "#cd5c5c".into();
            cfg.colors.label = "#b22222".into();
            cfg.colors.value = "#fff5ee".into();
            cfg.colors.separator_color = "#8b0000".into();
            cfg.colors.title_color = "#b22222".into();
        }
        "golden" | "golden-hour" => {
            cfg.colors.user = "#daa520".into();
            cfg.colors.host = "#b8860b".into();
            cfg.colors.label = "#daa520".into();
            cfg.colors.value = "#fff8dc".into();
            cfg.colors.separator_color = "#8b6914".into();
            cfg.colors.title_color = "#daa520".into();
        }
        "space" | "space-gray" => {
            cfg.colors.user = "#708090".into();
            cfg.colors.host = "#778899".into();
            cfg.colors.label = "#708090".into();
            cfg.colors.value = "#c0c0c0".into();
            cfg.colors.separator_color = "#2f4f4f".into();
            cfg.colors.title_color = "#708090".into();
        }
        "royal" | "royal-purple" => {
            cfg.colors.user = "#7851a9".into();
            cfg.colors.host = "#9370db".into();
            cfg.colors.label = "#7851a9".into();
            cfg.colors.value = "#e6e6fa".into();
            cfg.colors.separator_color = "#4b0082".into();
            cfg.colors.title_color = "#7851a9".into();
        }
        "abyss" | "abyssal-deep" => {
            cfg.colors.user = "#191970".into();
            cfg.colors.host = "#000080".into();
            cfg.colors.label = "#4169e1".into();
            cfg.colors.value = "#e6e6fa".into();
            cfg.colors.separator_color = "#00008b".into();
            cfg.colors.title_color = "#4169e1".into();
        }
        "solar" | "solar-flare" => {
            cfg.colors.user = "#ff4500".into();
            cfg.colors.host = "#ff6347".into();
            cfg.colors.label = "#ff4500".into();
            cfg.colors.value = "#ffdead".into();
            cfg.colors.separator_color = "#ff8c00".into();
            cfg.colors.title_color = "#ff4500".into();
        }
        _ => {
            eprintln!("\x1b[33m[!]\x1b[0m unknown theme: {}", theme_name);
        }
    }
}

fn apply_style(cfg: &mut Config, style_name: &str) {
    match style_name.to_lowercase().as_str() {
        "minimal" | "min" => {
            cfg.bold = false;
            cfg.dim = true;
            cfg.labels_capitalize = false;
            cfg.labels_uppercase = false;
            cfg.custom_lines.clear();
            cfg.info.de = false;
            cfg.info.wm = false;
            cfg.info.locale = false;
            cfg.info.battery = false;
            cfg.info.resolution = false;
            cfg.info.host = false;
            cfg.separator = "-".into();
        }
        "full" | "default" => {
            cfg.bold = true;
            cfg.dim = false;
            cfg.labels_capitalize = true;
            cfg.labels_uppercase = false;
            cfg.info.de = true;
            cfg.info.wm = true;
            cfg.info.locale = true;
            cfg.info.battery = true;
            cfg.info.resolution = true;
            cfg.info.host = true;
            cfg.separator = "─".into();
        }
        "compact" | "tiny" => {
            cfg.bold = true;
            cfg.dim = false;
            cfg.labels_uppercase = true;
            cfg.labels_capitalize = false;
            cfg.padding = 1;
            cfg.custom_lines.clear();
            cfg.info.de = false;
            cfg.info.wm = false;
            cfg.info.locale = false;
            cfg.info.battery = false;
            cfg.info.resolution = false;
            cfg.separator = ":".into();
        }
        "fancy" | "decorated" => {
            cfg.bold = true;
            cfg.dim = false;
            cfg.labels_capitalize = true;
            cfg.glow = true;
            cfg.separator = "═".into();
            cfg.custom_lines.clear();
            cfg.custom_lines.push("╔══════════════════════════════╗".into());
            cfg.custom_lines.push("║     voidfetch - fancy mode   ║".into());
            cfg.custom_lines.push("╚══════════════════════════════╝".into());
        }
        "hacker" | "matrix" => {
            cfg.bold = true;
            cfg.dim = false;
            cfg.labels_uppercase = true;
            cfg.glow = true;
            cfg.colors.user = "#00ff00".into();
            cfg.colors.host = "#00ff00".into();
            cfg.colors.label = "#00ff00".into();
            cfg.colors.value = "#00ff00".into();
            cfg.colors.separator_color = "#009900".into();
            cfg.colors.title_color = "#00ff00".into();
            cfg.separator = ">".into();
            cfg.custom_lines.clear();
            cfg.custom_lines.push("[ SYSTEM INITIALIZED ]".into());
        }
        "retro" | "old-school" => {
            cfg.bold = true;
            cfg.dim = false;
            cfg.labels_uppercase = true;
            cfg.colors.user = "#33ff00".into();
            cfg.colors.host = "#33ff00".into();
            cfg.colors.label = "#33ff00".into();
            cfg.colors.value = "#33ff00".into();
            cfg.colors.separator_color = "#009900".into();
            cfg.colors.title_color = "#33ff00".into();
            cfg.separator = "-".into();
            cfg.custom_lines.clear();
            cfg.custom_lines.push("C:\\> SYSTEM READY_".into());
        }
        "clean" | "plain" => {
            cfg.bold = false;
            cfg.dim = false;
            cfg.labels_capitalize = true;
            cfg.labels_uppercase = false;
            cfg.logo.enabled = false;
            cfg.custom_lines.clear();
            cfg.separator = ":".into();
            cfg.padding = 1;
        }
        "rainbow" => {
            cfg.bold = true;
            cfg.dim = false;
            cfg.glow = true;
            cfg.colors.user = "#ff0000".into();
            cfg.colors.host = "#ff7700".into();
            cfg.colors.label = "#ffff00".into();
            cfg.colors.value = "#00ff00".into();
            cfg.colors.separator_color = "#0000ff".into();
            cfg.colors.title_color = "#8b00ff".into();
            cfg.separator = "彩虹".into();
        }
        _ => {
            eprintln!("\x1b[33m[!]\x1b[0m unknown style: {}", style_name);
        }
    }
}

fn parse_value(val: &str) -> String {
    val.trim().trim_matches(';').trim_matches('"').trim_matches('\'').trim().to_string()
}

fn parse_bool(val: &str) -> bool {
    matches!(val.trim().to_lowercase().as_str(), "true" | "1" | "yes" | "on" | "enabled")
}

fn parse_usize(val: &str) -> usize {
    val.trim().trim_matches(';').trim().parse().unwrap_or(0)
}

fn apply_css_config(cfg: &mut Config, content: &str, _config_path: &PathBuf) {
    let mut current_block = String::new();
    let mut properties: HashMap<String, String> = HashMap::new();
    let mut all_props: Vec<(String, String)> = Vec::new();

    let mut brace_depth = 0;
    let mut in_comment = false;

    let resolved = resolve_variables(content, &cfg.variables);

    for line in resolved.lines() {
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

        if clean_line.is_empty() {
            continue;
        }

        if clean_line.starts_with("@import") {
            let import_path = clean_line
                .trim_start_matches("@import")
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .trim_matches(';')
                .trim()
                .to_string();

            if !import_path.is_empty() {
                let resolved_path = if import_path.starts_with('/') {
                    PathBuf::from(&import_path)
                } else {
                    find_examples_dir().join(&import_path)
                };

                if resolved_path.exists() {
                    if let Ok(import_content) = fs::read_to_string(&resolved_path) {
                        let import_resolved = resolve_variables(&import_content, &cfg.variables);
                        apply_css_config_raw(cfg, &import_resolved, &resolved_path);
                    }
                } else {
                    eprintln!("\x1b[33m[!]\x1b[0m import not found: {}", resolved_path.display());
                }
            }
            continue;
        }

        if clean_line.starts_with("@theme") {
            let theme_name = clean_line
                .trim_start_matches("@theme")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .trim_matches(';')
                .to_string();

            if !theme_name.is_empty() {
                apply_theme(cfg, &theme_name);
            }
            continue;
        }

        if clean_line.starts_with("@style") {
            let style_val = clean_line
                .trim_start_matches("@style")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .trim_matches(';')
                .to_string();

            if !style_val.is_empty() {
                apply_style(cfg, &style_val);
            }
            continue;
        }

        if clean_line.starts_with("@font") {
            let font_name = clean_line
                .trim_start_matches("@font")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .trim_matches(';')
                .to_string();

            if !font_name.is_empty() {
                cfg.font = font_name;
            }
            continue;
        }

        if clean_line.starts_with("@separator") {
            let sep_val = clean_line
                .trim_start_matches("@separator")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .trim_matches(';')
                .to_string();

            if !sep_val.is_empty() {
                cfg.separator = sep_val;
            }
            continue;
        }

        if clean_line.starts_with("@layout") {
            let layout_val = clean_line
                .trim_start_matches("@layout")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .trim_matches(';')
                .to_string();

            if !layout_val.is_empty() {
                cfg.layout = layout_val;
            }
            continue;
        }

        if clean_line.starts_with("@margin") {
            let margin_val = clean_line
                .trim_start_matches("@margin")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .trim_matches(';')
                .to_string();

            if let Ok(val) = margin_val.parse::<usize>() {
                cfg.padding = val;
            }
            continue;
        }

        if clean_line.starts_with("@opacity") {
            let opacity_val = clean_line
                .trim_start_matches("@opacity")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .trim_matches(';')
                .to_string();

            match opacity_val.to_lowercase().as_str() {
                "low" | "dim" => { cfg.dim = true; cfg.bold = false; }
                "medium" | "normal" => { cfg.dim = false; cfg.bold = false; }
                "high" | "bright" => { cfg.dim = false; cfg.bold = true; }
                "max" | "full" => { cfg.dim = false; cfg.bold = true; }
                _ => {}
            }
            continue;
        }

        if clean_line.starts_with("@italic") {
            let italic_val = clean_line
                .trim_start_matches("@italic")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .trim_matches(';')
                .to_string();

            cfg.italic = parse_bool(&italic_val);
            continue;
        }

        if clean_line.starts_with("@underline") {
            let underline_val = clean_line
                .trim_start_matches("@underline")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .trim_matches(';')
                .to_string();

            cfg.underline = parse_bool(&underline_val);
            continue;
        }

        if clean_line.starts_with("@glow") {
            let glow_val = clean_line
                .trim_start_matches("@glow")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .trim_matches(';')
                .to_string();

            cfg.glow = parse_bool(&glow_val);
            continue;
        }

        if clean_line.starts_with("@color") {
            let color_body = clean_line
                .trim_start_matches("@color")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .to_string();

            if let Some((key, val)) = color_body.split_once(':') {
                let k = key.trim().to_lowercase();
                let v = parse_value(val);
                match k.as_str() {
                    "user" => cfg.colors.user = v,
                    "host" => cfg.colors.host = v,
                    "label" => cfg.colors.label = v,
                    "value" => cfg.colors.value = v,
                    "separator" | "sep" => cfg.colors.separator_color = v,
                    "title" => cfg.colors.title_color = v,
                    "logo" => cfg.colors.logo = v,
                    _ => {}
                }
            }
            continue;
        }

        if clean_line.starts_with("@palette") {
            let palette_name = clean_line
                .trim_start_matches("@palette")
                .trim()
                .trim_start_matches('{')
                .trim_end_matches('}')
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .trim_matches(';')
                .to_string();

            if !palette_name.is_empty() {
                apply_palette(cfg, &palette_name);
            }
            continue;
        }

        if clean_line.starts_with("@reset") {
            let defaults = Config::default();
            cfg.colors = defaults.colors;
            cfg.info = defaults.info;
            cfg.logo = defaults.logo;
            cfg.title = defaults.title;
            cfg.separator = defaults.separator;
            cfg.padding = defaults.padding;
            cfg.bold = defaults.bold;
            cfg.dim = defaults.dim;
            cfg.labels_uppercase = defaults.labels_uppercase;
            cfg.labels_capitalize = defaults.labels_capitalize;
            cfg.custom_lines.clear();
            cfg.color_mode = defaults.color_mode;
            cfg.font = defaults.font;
            cfg.layout = defaults.layout;
            cfg.italic = false;
            cfg.underline = false;
            cfg.glow = false;
            cfg.style = defaults.style;
            continue;
        }

        if clean_line.starts_with('$') {
            if let Some((key, val)) = clean_line.split_once(':') {
                let var_name = key.trim().trim_start_matches('$').trim().to_string();
                let var_val = parse_value(val);
                if !var_name.is_empty() && !var_val.is_empty() {
                    cfg.variables.insert(var_name, var_val);
                }
            }
            continue;
        }

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

    apply_properties(cfg, &all_props);
}

fn apply_css_config_raw(cfg: &mut Config, content: &str, config_path: &PathBuf) {
    apply_css_config(cfg, content, config_path);
}

pub fn apply_css_config_pub(cfg: &mut Config, content: &str, config_path: &PathBuf) {
    apply_css_config(cfg, content, config_path);
}

fn apply_palette(cfg: &mut Config, palette_name: &str) {
    match palette_name.to_lowercase().as_str() {
        "nord" => {
            cfg.colors.user = "#88c0d0".into();
            cfg.colors.host = "#81a1c1".into();
            cfg.colors.label = "#a3be8c".into();
            cfg.colors.value = "#eceff4".into();
            cfg.colors.separator_color = "#4c566a".into();
            cfg.colors.title_color = "#88c0d0".into();
        }
        "dracula" => {
            cfg.colors.user = "#bd93f9".into();
            cfg.colors.host = "#ff79c6".into();
            cfg.colors.label = "#50fa7b".into();
            cfg.colors.value = "#f8f8f2".into();
            cfg.colors.separator_color = "#6272a4".into();
            cfg.colors.title_color = "#bd93f9".into();
        }
        "catppuccin" => {
            cfg.colors.user = "#cba6f7".into();
            cfg.colors.host = "#f5c2e7".into();
            cfg.colors.label = "#a6e3a1".into();
            cfg.colors.value = "#cdd6f4".into();
            cfg.colors.separator_color = "#585b70".into();
            cfg.colors.title_color = "#cba6f7".into();
        }
        "gruvbox" => {
            cfg.colors.user = "#fabd2f".into();
            cfg.colors.host = "#fe8019".into();
            cfg.colors.label = "#b8bb26".into();
            cfg.colors.value = "#ebdbb2".into();
            cfg.colors.separator_color = "#928374".into();
            cfg.colors.title_color = "#fabd2f".into();
        }
        "solarized" => {
            cfg.colors.user = "#268bd2".into();
            cfg.colors.host = "#2aa198".into();
            cfg.colors.label = "#859900".into();
            cfg.colors.value = "#839496".into();
            cfg.colors.separator_color = "#586e75".into();
            cfg.colors.title_color = "#268bd2".into();
        }
        "tokyo" => {
            cfg.colors.user = "#7aa2f7".into();
            cfg.colors.host = "#bb9af7".into();
            cfg.colors.label = "#9ece6a".into();
            cfg.colors.value = "#c0caf5".into();
            cfg.colors.separator_color = "#414868".into();
            cfg.colors.title_color = "#7aa2f7".into();
        }
        "rainbow" => {
            cfg.colors.user = "#ff0000".into();
            cfg.colors.host = "#ff7700".into();
            cfg.colors.label = "#ffff00".into();
            cfg.colors.value = "#00ff00".into();
            cfg.colors.separator_color = "#0000ff".into();
            cfg.colors.title_color = "#8b00ff".into();
        }
        "monochrome" | "mono" => {
            cfg.colors.user = "#ffffff".into();
            cfg.colors.host = "#cccccc".into();
            cfg.colors.label = "#999999".into();
            cfg.colors.value = "#ffffff".into();
            cfg.colors.separator_color = "#666666".into();
            cfg.colors.title_color = "#ffffff".into();
        }
        "pastel" => {
            cfg.colors.user = "#ffb3ba".into();
            cfg.colors.host = "#bae1ff".into();
            cfg.colors.label = "#baffc9".into();
            cfg.colors.value = "#ffffba".into();
            cfg.colors.separator_color = "#e8baff".into();
            cfg.colors.title_color = "#ffb3ba".into();
        }
        _ => {
            eprintln!("\x1b[33m[!]\x1b[0m unknown palette: {}", palette_name);
        }
    }
}

fn apply_properties(cfg: &mut Config, all_props: &[(String, String)]) {
    for (prop, val) in all_props {
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

            "font" | "font.name" => cfg.font = val.clone(),

            "layout" | "layout.mode" => cfg.layout = val.clone(),
            "italic" => cfg.italic = parse_bool(val),
            "underline" => cfg.underline = parse_bool(val),
            "glow" | "glow.enabled" => cfg.glow = parse_bool(val),
            "style" | "style.name" => apply_style(cfg, val),

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

/* --- import an example theme --- */
/* @import "01-arctic-frost.css"; */
/* @import "04-dracula.css"; */
/* @import "07-catppuccin-mocha.css"; */
/* @import "09-nord.css"; */
/* @import "24-matrix-green.css"; */
/* @import "29-vaporwave.css"; */

/* --- or use a built-in theme (33 themes) --- */
/* @theme arctic;   @theme sunset;    @theme neon; */
/* @theme dracula;  @theme tokyo;     @theme gruvbox; */
/* @theme catppuccin; @theme monokai; @theme nord; */
/* @theme onedark;  @theme rosepine;  @theme solarized; */
/* @theme github;   @theme palenight; @theme matrix; */
/* @theme vaporwave; @theme retro;    @theme void; */
/* @theme sakura;   @theme blood;     @theme ocean; */
/* @theme forest;   @theme lavender;  @theme amber; */
/* @theme emerald;  @theme ice;       @theme pastel; */
/* @theme crimson;  @theme golden;    @theme space; */
/* @theme royal;    @theme abyss;     @theme solar; */

/* --- or use a style preset --- */
/* @style minimal; */
/* @style compact; */
/* @style full; */
/* @style fancy; */
/* @style hacker; */
/* @style retro; */
/* @style clean; */
/* @style rainbow; */

/* --- or use a palette (9 palettes) --- */
/* @palette nord; */
/* @palette dracula; */
/* @palette catppuccin; */
/* @palette gruvbox; */
/* @palette solarized; */
/* @palette tokyo; */
/* @palette rainbow; */
/* @palette mono; */
/* @palette pastel; */

/* --- variables (recursive) --- */
$accent: #88c0d0;
$bg: #2e3440;
$username: void;
$hostname: fetcher;

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
    user: $accent;
    host: $accent;
    label: $accent;
    value: white;
    separator: gray;
    title: $accent;
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

/* --- shortcuts --- */
/* @color {{ user: red; }} */
/* @separator "═"; */
/* @margin 2; */
/* @opacity high; */
/* @italic true; */
/* @underline true; */
/* @glow true; */
/* @layout "side"; */
/* @font "default"; */
/* @reset; */

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

/* --- SYNTAX REFERENCE ---
 *
 * Variables:     $name: value;   use: $name
 * Import:        @import "file.css";
 * Themes:        @theme dracula;
 * Styles:        @style minimal;
 * Palettes:      @palette nord;
 * Colors:        @color {{ user: red; }}
 * Separator:     @separator "═";
 * Margin:        @margin 4;
 * Opacity:       @opacity low;
 * Italic:        @italic true;
 * Underline:     @underline true;
 * Glow:          @glow true;
 * Layout:        @layout "side";
 * Font:          @font "small";
 * Reset:         @reset;
 *
 * Named colors:  red, green, yellow, blue, magenta, cyan,
 *                white, gray, orange, pink, lime, violet,
 *                indigo, coral, salmon, gold, crimson,
 *                turquoise, aqua, purple, teal
 *
 * Hex:           #ff6600
 * RGB:           rgb(255, 102, 0)
 * 256-color:     256(208)
 * ANSI:          ansi(3)
 * None:          none
"#, env!("CARGO_PKG_VERSION"));
}
