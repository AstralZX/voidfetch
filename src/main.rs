use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

mod config;
mod info;
mod logo;
mod ansi;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|a| a == "--help" || a == "-h") {
        print_help();
        return;
    }
    if args.iter().any(|a| a == "--version" || a == "-v") {
        println!("voidfetch {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if args.iter().any(|a| a == "--dump-config") {
        config::print_default_config();
        return;
    }
    if args.iter().any(|a| a == "--sync") {
        sync();
        return;
    }
    if args.iter().any(|a| a == "--explode") {
        explode();
        return;
    }
    if args.iter().any(|a| a == "--cred" || a == "--credits") {
        open_cred();
        return;
    }
    if args.iter().any(|a| a == "--list-themes") {
        list_themes();
        return;
    }
    if args.iter().any(|a| a == "--list-examples") {
        list_examples();
        return;
    }
    if args.iter().any(|a| a == "--example" || a == "-e") {
        let idx = args.iter().position(|a| a == "--example" || a == "-e");
        if let Some(pos) = idx {
            if pos + 1 < args.len() {
                run_example(&args[pos + 1]);
                return;
            } else {
                eprintln!("\x1b[31m[-]\x1b[0m --example requires a number");
                return;
            }
        }
    }

    let cfg = config::load();
    let info = info::gather();
    let logo = logo::get(&cfg);

    render(&cfg, &info, &logo);
}

fn print_help() {
    println!(
r#"voidfetch {} - minimal system info

USAGE:
    voidfetch [OPTIONS]

OPTIONS:
    -h, --help       print this help
    -v, --version    print version
    --dump-config    print default config to stdout
    --config <PATH>  use custom config file
    -e, --example N  use example config by number (1-41)
    --cred           open voidfetch github in browser
    --sync           fetch latest and rebuild
    --explode        uninstall voidfetch from your system

CSS SYNTAX:
    @import          import example configs from examples/ folder
    @theme           apply a theme preset
    @font            set ascii art font style
    @separator       shorthand for separator config
    @layout          set layout (side, top, bottom)
    @margin          shorthand for padding
    @opacity         set text opacity (dim level)
    @italic          enable italic text
    @underline       enable underline
    @glow            enable glow effect (bright colors)
    @style           apply style preset (minimal, full, compact)
    $var             reference variables in values
    --list-themes    list available themes
    --list-examples  list example configs

CONFIG:
    Config files use CSS syntax and are searched in:
      1. --config <path>
      2. $VOIDFETCH_CONFIG
      3. ~/.config/voidfetch/config.css
      4. /etc/voidfetch/config.css

EXAMPLE:
    voidfetch --config ~/myconfig.css"#, env!("CARGO_PKG_VERSION")
    );
}

fn get_install_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        dirs_data().join("voidfetch")
    } else {
        dirs_home().join(".local").join("bin")
    }
}

fn get_config_dir() -> PathBuf {
    dirs_home().join(".config").join("voidfetch")
}

fn get_logos_dir() -> PathBuf {
    get_install_dir().join("logos")
}

fn dirs_home() -> PathBuf {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

fn dirs_data() -> PathBuf {
    if let Ok(v) = env::var("LOCALAPPDATA") {
        PathBuf::from(v)
    } else {
        dirs_home().join(".local").join("share")
    }
}

fn list_themes() {
    let themes = vec![
        ("arctic", "Arctic Frost", "Icy blues and whites"),
        ("sunset", "Sunset Fire", "Warm oranges and reds"),
        ("neon", "Neon Cyberpunk", "Vibrant neon colors"),
        ("dracula", "Dracula", "Purple and pink"),
        ("tokyo", "Tokyo Night", "Blue and purple"),
        ("gruvbox", "Gruvbox Dark", "Yellow and orange"),
        ("catppuccin", "Catppuccin Mocha", "Purple, pink, and teal"),
        ("monokai", "Monokai Pro", "Classic monokai colors"),
        ("nord", "Nord", "Frost blue palette"),
        ("onedark", "One Dark", "Atom one dark theme"),
        ("rosepine", "Rose Pine", "Soft pink and purple"),
        ("solarized", "Solarized Dark", "Solarized color scheme"),
        ("github", "GitHub Dark", "GitHub dark theme"),
        ("palenight", "Palenight", "Material palenight"),
        ("matrix", "Matrix Green", "Green on black"),
        ("vaporwave", "Vaporwave", "Pink, cyan, and green"),
        ("retro", "Retro Terminal", "Classic green terminal"),
        ("void", "Void Purple", "Deep purple void"),
    ];

    println!("\x1b[36mvoidfetch\x1b[0m - available themes:\n");
    for (name, display, desc) in themes {
        println!("  \x1b[33m{:<14}\x1b[0m {} - {}", name, display, desc);
    }
    println!("\nuse \x1b[33m@theme {{ name: <theme>; }}\x1b[0m in your config");
}

fn list_examples() {
    let examples_dir = get_examples_dir();
    println!("\x1b[36mvoidfetch\x1b[0m - example configs:\n");

    if examples_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&examples_dir) {
            let mut files: Vec<String> = entries
                .flatten()
                .filter_map(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    if name.ends_with(".css") { Some(name) } else { None }
                })
                .collect();
            files.sort();

            if files.is_empty() {
                println!("  no example configs found");
            } else {
                for (i, name) in files.iter().enumerate() {
                    let num = i + 1;
                    let display_name = name.trim_end_matches(".css");
                    println!("  \x1b[33m{:>2}\x1b[0m  {}", num, display_name);
                }
                println!("\n  use \x1b[33mvoidfetch --example <number>\x1b[0m to apply");
                println!("  or \x1b[33m@import \"<name>.css\"\x1b[0m in your config");
            }
        }
    } else {
        println!("  examples directory not found at: {}", examples_dir.display());
    }
}

fn run_example(num_str: &str) {
    let num: usize = match num_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("\x1b[31m[-]\x1b[0m invalid example number: {}", num_str);
            return;
        }
    };

    if num < 1 {
        eprintln!("\x1b[31m[-]\x1b[0m example number must be >= 1");
        return;
    }

    let examples_dir = get_examples_dir();
    if !examples_dir.is_dir() {
        eprintln!("\x1b[31m[-]\x1b[0m examples directory not found");
        return;
    }

    let mut files: Vec<String> = fs::read_dir(&examples_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if name.ends_with(".css") { Some(name) } else { None }
        })
        .collect();
    files.sort();

    if num > files.len() {
        eprintln!("\x1b[31m[-]\x1b[0m example {} not found (max: {})", num, files.len());
        return;
    }

    let filename = &files[num - 1];
    let path = examples_dir.join(filename);

    if let Ok(content) = fs::read_to_string(&path) {
        let mut cfg = config::Config::default();
        config::apply_css_config_pub(&mut cfg, &content, &path);

        let info = info::gather();
        let logo = logo::get(&cfg);

        println!("\x1b[36m[*]\x1b[0m using example {}: {}\n", num, filename.trim_end_matches(".css"));
        render(&cfg, &info, &logo);
    } else {
        eprintln!("\x1b[31m[-]\x1b[0m failed to read example: {}", path.display());
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

fn open_cred() {
    let url = "https://github.com/AstralZX/voidfetch";
    println!("\x1b[36m[*]\x1b[0m opening voidfetch github...");
    println!("\x1b[90m    {}\x1b[0m", url);

    let status = if cfg!(target_os = "macos") {
        Command::new("open").arg(url).status()
    } else if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "start", url]).status()
    } else {
        Command::new("xdg-open").arg(url).status()
    };

    match status {
        Ok(s) if s.success() => {
            println!("\x1b[32m[+]\x1b[0m opened in browser!");
        }
        _ => {
            eprintln!("\x1b[31m[-]\x1b[0m failed to open browser. visit manually:");
            eprintln!("    {}", url);
        }
    }
}

fn sync() {
    let install_dir = get_install_dir();
    let logos_dir = get_logos_dir();
    let tmp_dir = dirs_home().join(".voidfetch_sync_tmp");

    println!("\x1b[36m[*]\x1b[0m syncing voidfetch...");

    // clean old tmp
    let _ = fs::remove_dir_all(&tmp_dir);

    println!("\x1b[36m[*]\x1b[0m cloning latest from github...");
    let status = Command::new("git")
        .args(["clone", "--depth=1", "https://github.com/AstralZX/voidfetch.git", &tmp_dir.to_string_lossy()])
        .status();

    match status {
        Ok(s) if s.success() => {}
        _ => {
            eprintln!("\x1b[31m[-]\x1b[0m failed to clone repo. is git installed?");
            let _ = fs::remove_dir_all(&tmp_dir);
            return;
        }
    }

    println!("\x1b[36m[*]\x1b[0m building release binary...");
    let build = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&tmp_dir)
        .status();

    match build {
        Ok(s) if s.success() => {}
        _ => {
            eprintln!("\x1b[31m[-]\x1b[0m build failed. is rust installed?");
            let _ = fs::remove_dir_all(&tmp_dir);
            return;
        }
    }

    let bin_src = tmp_dir.join("target").join("release").join(if cfg!(target_os = "windows") { "voidfetch.exe" } else { "voidfetch" });

    if !bin_src.exists() {
        eprintln!("\x1b[31m[-]\x1b[0m binary not found after build");
        let _ = fs::remove_dir_all(&tmp_dir);
        return;
    }

    println!("\x1b[36m[*]\x1b[0m installing to {}...", install_dir.display());
    let _ = fs::create_dir_all(&install_dir);
    let _ = fs::create_dir_all(&logos_dir);

    let bin_dst = install_dir.join(if cfg!(target_os = "windows") { "voidfetch.exe" } else { "voidfetch" });
    if let Err(e) = fs::copy(&bin_src, &bin_dst) {
        eprintln!("\x1b[31m[-]\x1b[0m failed to copy binary: {}", e);
        let _ = fs::remove_dir_all(&tmp_dir);
        return;
    }

    // copy logos
    let logos_src = tmp_dir.join("logos");
    if logos_src.is_dir() {
        if let Ok(entries) = fs::read_dir(&logos_src) {
            for entry in entries.flatten() {
                let dst = logos_dir.join(entry.file_name());
                let _ = fs::copy(entry.path(), &dst);
            }
        }
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&bin_dst, fs::Permissions::from_mode(0o755));
    }

    let _ = fs::remove_dir_all(&tmp_dir);

    println!("\x1b[32m[+]\x1b[0m voidfetch updated successfully!");
    println!("\x1b[32m[+]\x1b[0m run 'voidfetch' to see the latest version.");
}

fn explode() {
    let install_dir = get_install_dir();
    let config_dir = get_config_dir();
    let bin = install_dir.join(if cfg!(target_os = "windows") { "voidfetch.exe" } else { "voidfetch" });

    println!("\x1b[31m[!]\x1b[0m voidfetch will be deleted from your system.");
    println!();
    println!("  binary:  {}", bin.display());
    println!("  logos:   {}", install_dir.join("logos").display());
    println!("  config:  {}", config_dir.display());
    println!();

    print!("\x1b[33m[?]\x1b[0m proceed? [y/N] ");

    use std::io::{self, Write};
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    if input != "y" && input != "yes" {
        println!("\x1b[36m[*]\x1b[0m cancelled.");
        return;
    }

    println!("\x1b[36m[*]\x1b[0m exploding...");

    // remove binary
    if bin.exists() {
        match fs::remove_file(&bin) {
            Ok(_) => println!("\x1b[32m[+]\x1b[0m removed {}", bin.display()),
            Err(e) => eprintln!("\x1b[31m[-]\x1b[0m failed to remove {}: {}", bin.display(), e),
        }
    }

    // remove logos dir
    let logos_dir = install_dir.join("logos");
    if logos_dir.is_dir() {
        match fs::remove_dir_all(&logos_dir) {
            Ok(_) => println!("\x1b[32m[+]\x1b[0m removed {}", logos_dir.display()),
            Err(e) => eprintln!("\x1b[31m[-]\x1b[0m failed to remove {}: {}", logos_dir.display(), e),
        }
    }

    // remove config
    if config_dir.is_dir() {
        print!("\x1b[33m[?]\x1b[0m also delete config? ({}) [y/N] ", config_dir.display());
        io::stdout().flush().unwrap();
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();
        if input2.trim().to_lowercase() == "y" || input2.trim().to_lowercase() == "yes" {
            match fs::remove_dir_all(&config_dir) {
                Ok(_) => println!("\x1b[32m[+]\x1b[0m removed {}", config_dir.display()),
                Err(e) => eprintln!("\x1b[31m[-]\x1b[0m failed to remove {}: {}", config_dir.display(), e),
            }
        }
    }

    println!();
    println!("\x1b[32m[+]\x1b[0m voidfetch has been obliterated from your system.");
    println!("\x1b[90m    RIP. \u{1f480}\x1b[0m");
}

fn render(cfg: &config::Config, info: &info::Info, logo: &Option<Vec<String>>) {
    let c = &cfg.colors;
    let s = &cfg.separator;
    let pad = cfg.padding;

    let user_col = ansi::color(&c.user);
    let host_col = ansi::color(&c.host);
    let label_col = ansi::color(&c.label);
    let value_col = ansi::color(&c.value);
    let sep_col = ansi::color(&c.separator_color);
    let reset = ansi::RESET;
    let bold = if cfg.bold { ansi::BOLD } else { "" };
    let _dim = if cfg.dim { ansi::DIM } else { "" };

    let mut lines: Vec<String> = Vec::new();

    if cfg.title.enabled {
        let title = format!(
            "{}{}{}@{}{}{}{}",
            user_col, bold, info.username, host_col, info.hostname, reset, ""
        );
        lines.push(title);
        let sep_len = info.username.len() + info.hostname.len() + 1;
        let sep_str: String = s.chars().take(1).collect();
        let sep_line: String = sep_str.repeat(sep_len);
        lines.push(format!("{}{}{}", sep_col, sep_line, reset));
    }

    let fields: Vec<(&str, bool, &str)> = vec![
        ("os", cfg.info.os, &info.os),
        ("host", cfg.info.host, &info.host),
        ("kernel", cfg.info.kernel, &info.kernel),
        ("uptime", cfg.info.uptime, &info.uptime),
        ("packages", cfg.info.packages, &info.packages),
        ("shell", cfg.info.shell, &info.shell),
        ("terminal", cfg.info.terminal, &info.terminal),
        ("de", cfg.info.de, &info.de),
        ("wm", cfg.info.wm, &info.wm),
        ("cpu", cfg.info.cpu, &info.cpu),
        ("gpu", cfg.info.gpu, &info.gpu),
        ("memory", cfg.info.memory, &info.memory),
        ("disk", cfg.info.disk, &info.disk),
        ("locale", cfg.info.locale, &info.locale),
        ("battery", cfg.info.battery, &info.battery),
        ("resolution", cfg.info.resolution, &info.resolution),
    ];

    for (name, enabled, value) in &fields {
        if !enabled || value.is_empty() || *value == "N/A" {
            continue;
        }
        let label_text = if cfg.labels_uppercase {
            name.to_uppercase()
        } else if cfg.labels_capitalize {
            let mut c = name.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        } else {
            name.to_string()
        };
        lines.push(format!(
            "{}{}{}:{} {}{}",
            label_col, bold, label_text, reset, value_col, value
        ));
    }

    if !cfg.custom_lines.is_empty() {
        for custom in &cfg.custom_lines {
            lines.push(format!("{}{}{}", label_col, custom, reset));
        }
    }

    let logo_lines = logo.as_deref().unwrap_or(&[]);
    let max_logo_w = logo_lines.iter().map(|l| ansi::visible_len(l)).max().unwrap_or(0);
    let empty_logo = " ".repeat(max_logo_w);

    let total = lines.len().max(logo_lines.len());
    let padding_str = " ".repeat(pad);

    for i in 0..total {
        let logo_part = if i < logo_lines.len() {
            let l = &logo_lines[i];
            let vis = ansi::visible_len(l);
            let pad_right = max_logo_w.saturating_sub(vis);
            format!("{}{}", l, " ".repeat(pad_right))
        } else {
            empty_logo.clone()
        };

        let info_part = if i < lines.len() {
            lines[i].clone()
        } else {
            String::new()
        };

        println!("{}{}{}{}", padding_str, logo_part, if info_part.is_empty() { "" } else { " " }, info_part);
    }

    if total == 0 {
        println!("{}voidfetch v{}", bold, env!("CARGO_PKG_VERSION"));
    }
}
