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
    pub font: String,
    pub variables: HashMap<String, String>,
    pub layout: String,
    pub italic: bool,
    pub underline: bool,
    pub glow: bool,
    #[allow(dead_code)]
    pub style: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            colors: ColorScheme {
                user: "cyan".into(), host: "cyan".into(), label: "cyan".into(),
                value: "white".into(), separator_color: "gray".into(),
                title_color: "cyan".into(), logo: "auto".into(),
            },
            info: InfoFlags {
                os: true, host: true, kernel: true, uptime: true,
                packages: true, shell: true, terminal: true, de: true,
                wm: true, cpu: true, gpu: true, memory: true,
                disk: true, locale: true, battery: true, resolution: true,
            },
            logo: LogoConfig { enabled: true, distro: "auto".into(), color_override: "auto".into() },
            title: TitleConfig { enabled: true, format: "{user}@{host}".into() },
            separator: "─".into(),
            padding: 2,
            bold: true,
            dim: false,
            labels_uppercase: false,
            labels_capitalize: true,
            custom_lines: Vec::new(),
            color_mode: "full".into(),
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
    let mut cfg = Config::default();

    // look for --config / -c in args
    let args: Vec<String> = env::args().skip(1).collect();
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
            env::var("HOME").ok()
                .map(|h| format!("{}/.config/voidfetch/config.css", h))
        })
        .or_else(|| Some("/etc/voidfetch/config.css".into()));

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

pub fn find_examples_dir() -> PathBuf {
    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));

    // look in a few places
    let candidates = [
        exe_dir.join("examples"),
        exe_dir.join("../examples"),
        exe_dir.join("../share/voidfetch/examples"),
    ];

    for c in &candidates {
        if c.is_dir() && has_css_files(c) {
            return c.clone();
        }
    }

    if let Ok(home) = env::var("HOME") {
        let local = PathBuf::from(&home).join(".local/bin/examples");
        if local.is_dir() && has_css_files(&local) {
            return local;
        }
    }

    if let Ok(cwd) = env::current_dir() {
        let cwd_ex = cwd.join("examples");
        if cwd_ex.is_dir() && has_css_files(&cwd_ex) {
            return cwd_ex;
        }
    }

    exe_dir.join("examples")
}

fn has_css_files(dir: &std::path::Path) -> bool {
    fs::read_dir(dir)
        .map(|mut r| {
            r.any(|e| {
                e.ok()
                    .map(|e| e.file_name().to_string_lossy().ends_with(".css"))
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

pub fn get_sorted_examples() -> Vec<String> {
    let dir = find_examples_dir();
    if !dir.is_dir() {
        return Vec::new();
    }
    let mut files: Vec<String> = fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if name.ends_with(".css") { Some(name) } else { None }
        })
        .collect();
    files.sort();
    files
}

pub fn get_example_by_number(num: usize) -> Option<(String, PathBuf)> {
    let files = get_sorted_examples();
    if num == 0 || num > files.len() {
        return None;
    }
    let dir = find_examples_dir();
    let filename = &files[num - 1];
    Some((filename.clone(), dir.join(filename)))
}

pub fn example_count() -> usize {
    get_sorted_examples().len()
}

// replace $var references with their values
fn resolve_variables(content: &str, variables: &HashMap<String, String>) -> String {
    if variables.is_empty() {
        return content.to_string();
    }
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
        if !changed { break; }
    }
    result
}

fn apply_theme(cfg: &mut Config, theme_name: &str) {
    let themes: &[(&[&str], &str, &str, &str, &str, &str, &str)] = &[
        (&["arctic", "arctic-frost"], "#88c0d0", "#81a1c1", "#88c0d0", "#eceff4", "#4c566a", "#88c0d0"),
        (&["sunset", "sunset-fire"], "#ff6b6b", "#ffa500", "#ff6b6b", "#fff5e6", "#cc5500", "#ff6b6b"),
        (&["neon", "neon-cyberpunk"], "#ff00ff", "#00ffff", "#ff00ff", "#ffffff", "#ffff00", "#ff00ff"),
        (&["dracula"], "#bd93f9", "#ff79c6", "#bd93f9", "#f8f8f2", "#6272a4", "#bd93f9"),
        (&["tokyo", "tokyo-night"], "#7aa2f7", "#bb9af7", "#7aa2f7", "#c0caf5", "#414868", "#7aa2f7"),
        (&["gruvbox", "gruvbox-dark"], "#fabd2f", "#fe8019", "#fabd2f", "#ebdbb2", "#928374", "#fabd2f"),
        (&["catppuccin", "catppuccin-mocha"], "#cba6f7", "#f5c2e7", "#cba6f7", "#cdd6f4", "#585b70", "#cba6f7"),
        (&["monokai", "monokai-pro"], "#fc9867", "#a9dc76", "#fc9867", "#fcfcfa", "#727072", "#fc9867"),
        (&["nord"], "#88c0d0", "#8fbcbb", "#88c0d0", "#eceff4", "#4c566a", "#88c0d0"),
        (&["onedark", "one-dark"], "#61afef", "#c678dd", "#61afef", "#abb2bf", "#5c6370", "#61afef"),
        (&["rosepine", "rose-pine"], "#ebbcba", "#c4a7e7", "#ebbcba", "#e0def4", "#908caa", "#ebbcba"),
        (&["solarized", "solarized-dark"], "#268bd2", "#2aa198", "#268bd2", "#839496", "#586e75", "#268bd2"),
        (&["github", "github-dark"], "#58a6ff", "#d2a8ff", "#58a6ff", "#c9d1d9", "#484f58", "#58a6ff"),
        (&["palenight"], "#c792ea", "#82aaff", "#c792ea", "#a6accd", "#39adb5", "#c792ea"),
        (&["matrix", "matrix-green"], "#00ff00", "#00cc00", "#00ff00", "#00ff00", "#009900", "#00ff00"),
        (&["vaporwave"], "#ff71ce", "#01cdfe", "#ff71ce", "#05ffa1", "#b967ff", "#ff71ce"),
        (&["retro", "retro-terminal"], "#33ff00", "#33ff00", "#33ff00", "#33ff00", "#009900", "#33ff00"),
        (&["void", "void-purple"], "#478061", "#96a7c9", "#478061", "#ffffff", "#96a7c9", "#478061"),
        (&["sakura", "sakura-pink"], "#ffb7c5", "#ff69b4", "#ffb7c5", "#fff0f5", "#db7093", "#ffb7c5"),
        (&["blood", "blood-moon"], "#dc143c", "#8b0000", "#dc143c", "#ffe4e1", "#8b0000", "#dc143c"),
        (&["ocean", "midnight-ocean"], "#00ced1", "#20b2aa", "#00ced1", "#e0ffff", "#2f4f4f", "#00ced1"),
        (&["forest"], "#228b22", "#32cd32", "#228b22", "#f0fff0", "#006400", "#228b22"),
        (&["lavender", "lavender-dreams"], "#e6e6fa", "#d8bfd8", "#e6e6fa", "#fffff0", "#9370db", "#e6e6fa"),
        (&["amber", "amber-glow"], "#ffbf00", "#ff8c00", "#ffbf00", "#fffacd", "#ff6347", "#ffbf00"),
        (&["emerald", "emerald-sea"], "#50c878", "#00fa9a", "#50c878", "#f0fff0", "#2e8b57", "#50c878"),
        (&["ice", "ice-blue"], "#add8e6", "#b0e0e6", "#add8e6", "#f0f8ff", "#4682b4", "#add8e6"),
        (&["pastel", "pastel-dream"], "#ffb3ba", "#bae1ff", "#ffb3ba", "#ffffba", "#baffc9", "#ffb3ba"),
        (&["crimson", "crimson-tide"], "#b22222", "#cd5c5c", "#b22222", "#fff5ee", "#8b0000", "#b22222"),
        (&["golden", "golden-hour"], "#daa520", "#b8860b", "#daa520", "#fff8dc", "#8b6914", "#daa520"),
        (&["space", "space-gray"], "#708090", "#778899", "#708090", "#c0c0c0", "#2f4f4f", "#708090"),
        (&["royal", "royal-purple"], "#7851a9", "#9370db", "#7851a9", "#e6e6fa", "#4b0082", "#7851a9"),
        (&["abyss", "abyssal-deep"], "#191970", "#000080", "#4169e1", "#e6e6fa", "#00008b", "#4169e1"),
        (&["solar", "solar-flare"], "#ff4500", "#ff6347", "#ff4500", "#ffdead", "#ff8c00", "#ff4500"),
    ];

    for (aliases, user, host, label, value, sep, title) in themes {
        if aliases.iter().any(|a| *a == theme_name.to_lowercase().as_str()) {
            cfg.colors.user = user.to_string();
            cfg.colors.host = host.to_string();
            cfg.colors.label = label.to_string();
            cfg.colors.value = value.to_string();
            cfg.colors.separator_color = sep.to_string();
            cfg.colors.title_color = title.to_string();
            return;
        }
    }
    eprintln!("\x1b[33m[!]\x1b[0m unknown theme: {}", theme_name);
}

fn apply_style(cfg: &mut Config, style_name: &str) {
    match style_name.to_lowercase().as_str() {
        "minimal" | "min" => {
            cfg.bold = false;
            cfg.dim = true;
            cfg.labels_capitalize = false;
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
            cfg.info = InfoFlags {
                os: true, host: true, kernel: true, uptime: true,
                packages: true, shell: true, terminal: true, de: true,
                wm: true, cpu: true, gpu: true, memory: true,
                disk: true, locale: true, battery: true, resolution: true,
            };
            cfg.separator = "─".into();
        }
        "compact" | "tiny" => {
            cfg.bold = true;
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
            cfg.labels_capitalize = true;
            cfg.glow = true;
            cfg.separator = "═".into();
            cfg.custom_lines = vec![
                "╔══════════════════════════════╗".into(),
                "║     voidfetch - fancy mode   ║".into(),
                "╚══════════════════════════════╝".into(),
            ];
        }
        "hacker" | "matrix" => {
            cfg.bold = true;
            cfg.labels_uppercase = true;
            cfg.glow = true;
            cfg.colors = ColorScheme {
                user: "#00ff00".into(), host: "#00ff00".into(), label: "#00ff00".into(),
                value: "#00ff00".into(), separator_color: "#009900".into(),
                title_color: "#00ff00".into(), logo: "auto".into(),
            };
            cfg.separator = ">".into();
            cfg.custom_lines = vec!["[ SYSTEM INITIALIZED ]".into()];
        }
        "retro" | "old-school" => {
            cfg.bold = true;
            cfg.labels_uppercase = true;
            cfg.colors = ColorScheme {
                user: "#33ff00".into(), host: "#33ff00".into(), label: "#33ff00".into(),
                value: "#33ff00".into(), separator_color: "#009900".into(),
                title_color: "#33ff00".into(), logo: "auto".into(),
            };
            cfg.separator = "-".into();
            cfg.custom_lines = vec!["C:\\> SYSTEM READY_".into()];
        }
        "clean" | "plain" => {
            cfg.bold = false;
            cfg.labels_capitalize = true;
            cfg.logo.enabled = false;
            cfg.custom_lines.clear();
            cfg.separator = ":".into();
            cfg.padding = 1;
        }
        "rainbow" => {
            cfg.bold = true;
            cfg.glow = true;
            cfg.colors = ColorScheme {
                user: "#ff0000".into(), host: "#ff7700".into(), label: "#ffff00".into(),
                value: "#00ff00".into(), separator_color: "#0000ff".into(),
                title_color: "#8b00ff".into(), logo: "auto".into(),
            };
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

fn extract_directive_value(line: &str, prefix: &str) -> String {
    line.trim_start_matches(prefix)
        .trim()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .trim_matches(';')
        .trim()
        .to_string()
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

        if trimmed.starts_with("/*") { in_comment = true; }
        if in_comment {
            if trimmed.contains("*/") { in_comment = false; }
            continue;
        }

        let clean_line = if let Some(pos) = trimmed.find("//") {
            &trimmed[..pos]
        } else {
            trimmed
        };

        if clean_line.is_empty() { continue; }

        // imports
        if clean_line.starts_with("@import") {
            let import_path = extract_directive_value(clean_line, "@import");
            if !import_path.is_empty() {
                let resolved_path = if import_path.starts_with('/') {
                    PathBuf::from(&import_path)
                } else {
                    find_examples_dir().join(&import_path)
                };
                if resolved_path.exists() {
                    if let Ok(import_content) = fs::read_to_string(&resolved_path) {
                        let import_resolved = resolve_variables(&import_content, &cfg.variables);
                        apply_css_config(cfg, &import_resolved, &resolved_path);
                    }
                } else {
                    eprintln!("\x1b[33m[!]\x1b[0m import not found: {}", resolved_path.display());
                }
            }
            continue;
        }

        // directives
        if clean_line.starts_with("@theme") {
            let v = extract_directive_value(clean_line, "@theme");
            if !v.is_empty() { apply_theme(cfg, &v); }
            continue;
        }
        if clean_line.starts_with("@style") {
            let v = extract_directive_value(clean_line, "@style");
            if !v.is_empty() { apply_style(cfg, &v); }
            continue;
        }
        if clean_line.starts_with("@font") {
            let v = extract_directive_value(clean_line, "@font");
            if !v.is_empty() { cfg.font = v; }
            continue;
        }
        if clean_line.starts_with("@separator") {
            let v = extract_directive_value(clean_line, "@separator");
            if !v.is_empty() { cfg.separator = v; }
            continue;
        }
        if clean_line.starts_with("@layout") {
            let v = extract_directive_value(clean_line, "@layout");
            if !v.is_empty() { cfg.layout = v; }
            continue;
        }
        if clean_line.starts_with("@margin") {
            let v = extract_directive_value(clean_line, "@margin");
            if let Ok(val) = v.parse::<usize>() { cfg.padding = val; }
            continue;
        }
        if clean_line.starts_with("@opacity") {
            match extract_directive_value(clean_line, "@opacity").to_lowercase().as_str() {
                "low" | "dim" => { cfg.dim = true; cfg.bold = false; }
                "medium" | "normal" => { cfg.dim = false; cfg.bold = false; }
                "high" | "bright" | "max" | "full" => { cfg.dim = false; cfg.bold = true; }
                _ => {}
            }
            continue;
        }
        if clean_line.starts_with("@italic") {
            cfg.italic = parse_bool(&extract_directive_value(clean_line, "@italic"));
            continue;
        }
        if clean_line.starts_with("@underline") {
            cfg.underline = parse_bool(&extract_directive_value(clean_line, "@underline"));
            continue;
        }
        if clean_line.starts_with("@glow") {
            cfg.glow = parse_bool(&extract_directive_value(clean_line, "@glow"));
            continue;
        }
        if clean_line.starts_with("@palette") {
            let v = extract_directive_value(clean_line, "@palette");
            if !v.is_empty() { apply_palette(cfg, &v); }
            continue;
        }
        if clean_line.starts_with("@reset") {
            *cfg = Config::default();
            continue;
        }

        // @color { user: red; }
        if clean_line.starts_with("@color") {
            let body = clean_line.trim_start_matches("@color").trim()
                .trim_start_matches('{').trim_end_matches('}').trim().to_string();
            if let Some((key, val)) = body.split_once(':') {
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

        // variables: $accent: #88c0d0;
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

        // track which block we're in
        if clean_line.starts_with('@') || (brace_depth == 0 && clean_line.contains('{')) {
            if let Some(name) = clean_line.split('{').next() {
                current_block = name.trim().trim_start_matches('@').to_lowercase();
            }
        }

        // count braces
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

pub fn apply_css_config_pub(cfg: &mut Config, content: &str, config_path: &PathBuf) {
    apply_css_config(cfg, content, config_path);
}

fn apply_palette(cfg: &mut Config, palette_name: &str) {
    let palettes: &[(&[&str], &str, &str, &str, &str, &str, &str)] = &[
        (&["nord"], "#88c0d0", "#81a1c1", "#a3be8c", "#eceff4", "#4c566a", "#88c0d0"),
        (&["dracula"], "#bd93f9", "#ff79c6", "#50fa7b", "#f8f8f2", "#6272a4", "#bd93f9"),
        (&["catppuccin"], "#cba6f7", "#f5c2e7", "#a6e3a1", "#cdd6f4", "#585b70", "#cba6f7"),
        (&["gruvbox"], "#fabd2f", "#fe8019", "#b8bb26", "#ebdbb2", "#928374", "#fabd2f"),
        (&["solarized"], "#268bd2", "#2aa198", "#859900", "#839496", "#586e75", "#268bd2"),
        (&["tokyo"], "#7aa2f7", "#bb9af7", "#9ece6a", "#c0caf5", "#414868", "#7aa2f7"),
        (&["rainbow"], "#ff0000", "#ff7700", "#ffff00", "#00ff00", "#0000ff", "#8b00ff"),
        (&["monochrome", "mono"], "#ffffff", "#cccccc", "#999999", "#ffffff", "#666666", "#ffffff"),
        (&["pastel"], "#ffb3ba", "#bae1ff", "#baffc9", "#ffffba", "#e8baff", "#ffb3ba"),
    ];

    for (aliases, user, host, label, value, sep, title) in palettes {
        if aliases.iter().any(|a| *a == palette_name.to_lowercase().as_str()) {
            cfg.colors.user = user.to_string();
            cfg.colors.host = host.to_string();
            cfg.colors.label = label.to_string();
            cfg.colors.value = value.to_string();
            cfg.colors.separator_color = sep.to_string();
            cfg.colors.title_color = title.to_string();
            return;
        }
    }
    eprintln!("\x1b[33m[!]\x1b[0m unknown palette: {}", palette_name);
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
    println!(r#"/* voidfetch config */
/* put this at ~/.config/voidfetch/config.css */

/* @import "04-dracula.css"; */
/* @theme catppuccin; */

$accent: #88c0d0;

@colors {{
    user: $accent;
    host: $accent;
    label: $accent;
    value: white;
    separator: gray;
    title: $accent;
}}

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

@logo {{
    enabled: true;
    distro: auto;
    color: auto;
}}

@title {{
    enabled: true;
    format: "{{user}}@{{host}}";
}}

/* @color {{ user: red; }} */
/* @separator "═"; */
/* @margin 2; */
/* @italic true; */
/* @glow true; */
/* @reset; */"#);
}