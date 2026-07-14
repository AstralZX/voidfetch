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
    pub font: String,
    pub variables: HashMap<String, String>,
    pub layout: String,
    pub italic: bool,
    pub underline: bool,
    pub glow: bool,
    pub style: String,
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
                apply_css_config(&mut cfg, &content, &p);
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

fn get_examples_dir() -> PathBuf {
    if let Ok(home) = env::var("HOME") {
        let local = PathBuf::from(&home).join(".local").join("bin").join("examples");
        if local.is_dir() {
            return local;
        }
    }
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    let candidates = [
        exe_dir.join("examples"),
        exe_dir.parent().unwrap_or(&exe_dir).join("examples"),
        exe_dir.join("..").join("examples"),
        exe_dir.join("..").join("..").join("examples"),
        PathBuf::from("examples"),
    ];

    for c in &candidates {
        if c.is_dir() && !c.read_dir().map(|mut r| r.next().is_none()).unwrap_or(true) {
            return c.clone();
        }
    }
    exe_dir.join("examples")
}

fn resolve_variables(content: &str, variables: &HashMap<String, String>) -> String {
    let mut result = content.to_string();
    for (key, value) in variables {
        let placeholder = format!("${{{}}}", key);
        result = result.replace(&placeholder, value);
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
        _ => {
            eprintln!("\x1b[33m[!]\x1b[0m unknown style: {}", style_name);
        }
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
                    get_examples_dir().join(&import_path)
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
                .to_string();

            if !theme_name.is_empty() {
                apply_theme(cfg, &theme_name);
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

        if clean_line.starts_with("$") {
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

/* --- or use a built-in theme --- */
/* @theme arctic; */
/* @theme dracula; */
/* @theme catppuccin; */
/* @theme nord; */
/* @theme matrix; */
/* @theme vaporwave; */
/* @theme retro; */

/* --- or use a style preset --- */
/* @style minimal; */
/* @style compact; */
/* @style fancy; */
/* @style hacker; */
/* @style retro; */

/* --- variables --- */
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

/* --- layout --- */
/* @layout "side"; */
/* @layout "top"; */
/* @layout "bottom"; */

/* --- font style for ascii art --- */
/* @font "default"; */
/* @font "small"; */
/* @font "large"; */

/* --- separator shorthand --- */
/* @separator "─"; */
/* @separator "═"; */
/* @separator "─·─"; */

/* --- margin/padding --- */
/* @margin 2; */
/* @margin 4; */

/* --- opacity/dim --- */
/* @opacity low; */
/* @opacity medium; */
/* @opacity high; */

/* --- text effects --- */
/* @italic true; */
/* @underline true; */
/* @glow true; */

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
 *
 * Variables:
 *   $varname: value;
 *   color: $varname;
 *
 * Import examples:
 *   @import "01-arctic-frost.css";
 *
 * Built-in themes:
 *   @theme arctic, dracula, catppuccin, nord, matrix,
 *   @theme vaporwave, retro, tokyo, gruvbox, rosepine,
 *   @theme solarized, github, palenight, void, neon,
 *   @theme sunset, monokai, onedark
 *
 * Style presets:
 *   @style minimal, compact, full, fancy, hacker, retro
 *
 * Layout:
 *   @layout "side", "top", "bottom"
 *
 * ASCII art font:
 *   @font "default", "small"
 *
 * Separator shorthand:
 *   @separator "═";
 *
 * Margin/Padding:
 *   @margin 2;
 *
 * Opacity:
 *   @opacity low, medium, high, max
 *
 * Text effects:
 *   @italic true;
 *   @underline true;
 *   @glow true;
"#, env!("CARGO_PKG_VERSION"));
}
