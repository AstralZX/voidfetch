use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

static LOGO_OVERRIDE: Mutex<Option<String>> = Mutex::new(None);

mod ansi;
mod config;
mod info;
mod logo;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        return run_display(None);
    }

    match args[0].as_str() {
        "-h" | "--help" => return print_help(),
        "-v" | "--version" => return println!("voidfetch {}", env!("CARGO_PKG_VERSION")),
        "--dump-config" => return config::print_default_config(),
        "--cred" | "--credits" => return open_cred(),
        "--list-themes" => return list_themes(),
        "--list-examples" => return list_examples(),
        "--list-logos" => return list_logos(),
        "--logo" => {
            if args.len() < 2 {
                eprintln!("\x1b[31m[-]\x1b[0m --logo requires a distro name");
                return;
            }
            *LOGO_OVERRIDE.lock().unwrap() = Some(args[1].clone());
            return run_display(None);
        }
        "--sync" => return sync(),
        "--explode" => return explode(),
        "-e" | "--example" => {
            if args.len() < 2 {
                eprintln!("\x1b[31m[-]\x1b[0m --example requires a number");
                return;
            }
            return run_example(&args[1]);
        }
        "-c" | "--config" => {
            if args.len() < 2 {
                eprintln!("\x1b[31m[-]\x1b[0m --config requires a path");
                return;
            }
            return run_display(Some(&args[1]));
        }
        _ => {}
    }

    run_display(None);
}

fn run_display(config_path: Option<&str>) {
    let mut cfg = match config_path {
        Some(p) => config::load_with_path(p),
        None => config::load(),
    };

    if let Some(distro) = LOGO_OVERRIDE.lock().unwrap().take() {
        cfg.logo.distro = distro;
    }

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
    -h, --help            print this help
    -v, --version         print version
    -c, --config <PATH>   use custom config file
    -e, --example N       use example config by number (1-61)
    --logo <NAME>         override distro logo by name
    --dump-config         print default config to stdout
    --cred                open voidfetch github in browser
    --sync                fetch latest from github, build and install
    --explode             uninstall voidfetch from your system
    --list-themes         list available built-in themes
    --list-examples       list example configs with numbers
    --list-logos          list available distro logos

EXAMPLES:
    voidfetch
    voidfetch --example 7
    voidfetch --logo arch
    voidfetch --logo cachyos
    voidfetch --config ~/myconfig.css
    voidfetch --example 4 --dump-config"#, env!("CARGO_PKG_VERSION")
    );
}

fn get_install_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        PathBuf::from(env::var("LOCALAPPDATA").unwrap_or_default()).join("voidfetch")
    } else {
        PathBuf::from(env::var("HOME").unwrap_or_default())
            .join(".local").join("bin")
    }
}

fn get_config_dir() -> PathBuf {
    PathBuf::from(env::var("HOME").unwrap_or_default())
        .join(".config").join("voidfetch")
}

fn list_themes() {
    let themes = [
        ("arctic", "Arctic Frost"), ("sunset", "Sunset Fire"),
        ("neon", "Neon Cyberpunk"), ("dracula", "Dracula"),
        ("tokyo", "Tokyo Night"), ("gruvbox", "Gruvbox Dark"),
        ("catppuccin", "Catppuccin Mocha"), ("monokai", "Monokai Pro"),
        ("nord", "Nord"), ("onedark", "One Dark"),
        ("rosepine", "Rose Pine"), ("solarized", "Solarized Dark"),
        ("github", "GitHub Dark"), ("palenight", "Palenight"),
        ("matrix", "Matrix Green"), ("vaporwave", "Vaporwave"),
        ("retro", "Retro Terminal"), ("void", "Void Purple"),
        ("sakura", "Sakura Pink"), ("blood", "Blood Moon"),
        ("ocean", "Midnight Ocean"), ("forest", "Forest Green"),
        ("lavender", "Lavender Dreams"), ("amber", "Amber Glow"),
        ("emerald", "Emerald Sea"), ("ice", "Ice Blue"),
        ("pastel", "Pastel Dream"), ("crimson", "Crimson Tide"),
        ("golden", "Golden Hour"), ("space", "Space Gray"),
        ("royal", "Royal Purple"), ("abyss", "Abyssal Deep"),
        ("solar", "Solar Flare"),
    ];

    println!("\x1b[36mvoidfetch\x1b[0m - {} themes:\n", themes.len());
    for (name, display) in &themes {
        println!("  \x1b[33m{:<14}\x1b[0m {}", name, display);
    }
    println!("\nuse \x1b[33m@theme {}; \x1b[0m in your config", themes[0].0);
}

fn list_examples() {
    let files = config::get_sorted_examples();
    println!("\x1b[36mvoidfetch\x1b[0m - {} example configs:\n", files.len());

    if files.is_empty() {
        println!("  no example configs found");
        println!("  run \x1b[33mvoidfetch --sync\x1b[0m to install examples");
        return;
    }

    for (i, name) in files.iter().enumerate() {
        let display_name = name.trim_end_matches(".css");
        println!("  \x1b[33m{:>2}\x1b[0m  {}", i + 1, display_name);
    }
    println!("\n  use \x1b[33mvoidfetch --example <number>\x1b[0m to preview");
}

fn list_logos() {
    let logos_dir = logo::logos_dir();
    println!("\x1b[36mvoidfetch\x1b[0m - available logos:\n");

    if !logos_dir.is_dir() {
        println!("  logos directory not found");
        println!("  run \x1b[33mvoidfetch --sync\x1b[0m to install logos");
        return;
    }

    let mut files: Vec<String> = fs::read_dir(&logos_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if name.ends_with(".txt") && !name.contains("404") {
                Some(name.trim_end_matches(".txt").to_string())
            } else {
                None
            }
        })
        .collect();
    files.sort();

    if files.is_empty() {
        println!("  no logos found");
    } else {
        for name in &files {
            println!("  \x1b[33m{}\x1b[0m", name);
        }
        println!("\n  use \x1b[33m@logo {{ distro: <name>; }}\x1b[0m in your config");
        println!("  or \x1b[33mvoidfetch --logo <name>\x1b[0m on the CLI");
    }
}

fn run_example(num_str: &str) {
    let num: usize = match num_str.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("\x1b[31m[-]\x1b[0m invalid number: {}", num_str);
            return;
        }
    };

    let (filename, path) = match config::get_example_by_number(num) {
        Some(v) => v,
        None => {
            let max = config::example_count();
            eprintln!("\x1b[31m[-]\x1b[0m example {} not found (max: {})", num, max);
            return;
        }
    };

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("\x1b[31m[-]\x1b[0m failed to read {}: {}", path.display(), e);
            return;
        }
    };

    let mut cfg = config::Config::default();
    config::apply_css_config_pub(&mut cfg, &content, &path);

    let info = info::gather();
    let logo = logo::get(&cfg);

    println!("\x1b[36m[*]\x1b[0m using example {}: {}\n", num, filename.trim_end_matches(".css"));
    render(&cfg, &info, &logo);
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
        Ok(s) if s.success() => println!("\x1b[32m[+]\x1b[0m opened in browser!"),
        _ => eprintln!("\x1b[33m[!]\x1b[0m could not open browser. visit: \x1b[36m{}\x1b[0m", url),
    }
}

fn sync() {
    let install_dir = get_install_dir();
    let logos_dir = install_dir.join("logos");
    let examples_dir = install_dir.join("examples");
    let tmp_dir = dirs_home().join(".voidfetch_sync_tmp");

    println!("\x1b[36m[*]\x1b[0m syncing voidfetch...");
    let _ = fs::remove_dir_all(&tmp_dir);

    println!("\x1b[36m[*]\x1b[0m cloning latest from github...");
    if !Command::new("git")
        .args(["clone", "--depth=1", "https://github.com/AstralZX/voidfetch.git", &tmp_dir.to_string_lossy()])
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!("\x1b[31m[-]\x1b[0m failed to clone repo. is git installed?");
        let _ = fs::remove_dir_all(&tmp_dir);
        return;
    }

    println!("\x1b[36m[*]\x1b[0m building release binary...");
    if !Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&tmp_dir)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!("\x1b[31m[-]\x1b[0m build failed. is rust installed?");
        let _ = fs::remove_dir_all(&tmp_dir);
        return;
    }

    let bin_name = if cfg!(target_os = "windows") { "voidfetch.exe" } else { "voidfetch" };
    let bin_src = tmp_dir.join("target").join("release").join(bin_name);

    if !bin_src.exists() {
        eprintln!("\x1b[31m[-]\x1b[0m binary not found after build");
        let _ = fs::remove_dir_all(&tmp_dir);
        return;
    }

    println!("\x1b[36m[*]\x1b[0m installing to {}...", install_dir.display());
    let _ = fs::create_dir_all(&install_dir);
    let _ = fs::create_dir_all(&logos_dir);
    let _ = fs::create_dir_all(&examples_dir);

    println!("\x1b[36m[*]\x1b[0m cleaning old installation...");
    let _ = fs::remove_dir_all(&logos_dir);
    let _ = fs::remove_dir_all(&examples_dir);
    let _ = fs::create_dir_all(&logos_dir);
    let _ = fs::create_dir_all(&examples_dir);

    let bin_dst = install_dir.join(bin_name);
    let _ = fs::remove_file(&bin_dst);
    if let Err(e) = fs::copy(&bin_src, &bin_dst) {
        eprintln!("\x1b[31m[-]\x1b[0m failed to copy binary: {}", e);
        let _ = fs::remove_dir_all(&tmp_dir);
        return;
    }

    copy_dir_contents(&tmp_dir.join("logos"), &logos_dir);
    copy_dir_contents(&tmp_dir.join("examples"), &examples_dir);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(&bin_dst, fs::Permissions::from_mode(0o755));
    }

    let _ = fs::remove_dir_all(&tmp_dir);

    let logo_count = count_files(&logos_dir, ".txt");
    let example_count = count_files(&examples_dir, ".css");

    println!("\x1b[32m[+]\x1b[0m voidfetch updated successfully!");
    println!("\x1b[32m[+]\x1b[0m binary:   {}", bin_dst.display());
    println!("\x1b[32m[+]\x1b[0m logos:    {} distro logos", logo_count);
    println!("\x1b[32m[+]\x1b[0m examples: {} config presets", example_count);
}

fn copy_dir_contents(src: &PathBuf, dst: &PathBuf) {
    if !src.is_dir() { return; }
    if let Ok(entries) = fs::read_dir(src) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                let _ = fs::copy(entry.path(), dst.join(entry.file_name()));
            }
        }
    }
}

fn count_files(dir: &PathBuf, ext: &str) -> usize {
    fs::read_dir(dir)
        .map(|r| r.filter(|e| e.as_ref().map(|e| e.file_name().to_string_lossy().ends_with(ext)).unwrap_or(false)).count())
        .unwrap_or(0)
}

fn dirs_home() -> PathBuf {
    PathBuf::from(env::var("HOME").or_else(|_| env::var("USERPROFILE")).unwrap_or_default())
}

fn explode() {
    let install_dir = get_install_dir();
    let config_dir = get_config_dir();
    let bin_name = if cfg!(target_os = "windows") { "voidfetch.exe" } else { "voidfetch" };
    let bin = install_dir.join(bin_name);

    println!("\x1b[31m[!]\x1b[0m voidfetch will be deleted from your system.\n");
    println!("  binary:   {}", bin.display());
    println!("  logos:    {}", install_dir.join("logos").display());
    println!("  examples: {}", install_dir.join("examples").display());
    println!("  config:   {}\n", config_dir.display());

    print!("\x1b[33m[?]\x1b[0m proceed? [y/N] ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim().to_lowercase() != "y" {
        println!("\x1b[36m[*]\x1b[0m cancelled.");
        return;
    }

    println!("\x1b[36m[*]\x1b[0m exploding...");

    remove_path(&bin);
    remove_path(&install_dir.join("logos"));
    remove_path(&install_dir.join("examples"));

    if config_dir.is_dir() {
        print!("\x1b[33m[?]\x1b[0m also delete config? ({}) [y/N] ", config_dir.display());
        io::stdout().flush().unwrap();
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).unwrap();
        if input2.trim().to_lowercase() == "y" {
            remove_path(&config_dir);
        }
    }

    println!("\n\x1b[32m[+]\x1b[0m voidfetch has been obliterated from your system.");
    println!("\x1b[90m    RIP.\x1b[0m");
}

fn remove_path(path: &PathBuf) {
    if !path.exists() { return; }
    let result = if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    };
    match result {
        Ok(_) => println!("\x1b[32m[+]\x1b[0m removed {}", path.display()),
        Err(e) => eprintln!("\x1b[31m[-]\x1b[0m failed to remove {}: {}", path.display(), e),
    }
}

// pretty straightforward - just line up the logo and info side by side
fn render(cfg: &config::Config, info: &info::Info, logo: &Option<Vec<String>>) {
    let c = &cfg.colors;
    let s = &cfg.separator;
    let pad = cfg.padding;

    let user_col = ansi::color(&c.user);
    let host_col = ansi::color(&c.host);
    let label_col = ansi::color(&c.label);
    let value_col = ansi::color(&c.value);
    let sep_col = ansi::color(&c.separator_color);
    let title_col = ansi::color(&c.title_color);
    let reset = ansi::RESET;
    let bold = if cfg.bold { ansi::BOLD } else { "" };
    let dim = if cfg.dim { ansi::DIM } else { "" };
    let italic = if cfg.italic { ansi::ITALIC } else { "" };
    let underline = if cfg.underline { ansi::UNDERLINE } else { "" };

    let mut lines: Vec<String> = Vec::with_capacity(20);

    if cfg.title.enabled {
        let title = format!(
            "{}{}{}{}@{}{}{}{}{}",
            title_col, bold, italic, user_col, info.username,
            host_col, info.hostname, underline, reset,
        );
        lines.push(title);
        let sep_len = info.username.len() + info.hostname.len() + 1;
        let sep_str: String = s.chars().take(1).collect();
        let sep_line: String = sep_str.repeat(sep_len);
        lines.push(format!("{}{}{}", sep_col, sep_line, reset));
    }

    let fields: &[(&str, bool, &str)] = &[
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

    for (name, enabled, value) in fields {
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
            "{}{}{}{}:{} {}{}{}",
            label_col, bold, italic, label_text, reset, dim, value_col, value
        ));
    }

    for custom in &cfg.custom_lines {
        lines.push(format!("{}{}{}{}", label_col, bold, custom, reset));
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

        println!(
            "{}{}{}{}",
            padding_str,
            logo_part,
            if info_part.is_empty() { "" } else { " " },
            info_part,
        );
    }

    if total == 0 {
        println!("{}voidfetch v{}", bold, env!("CARGO_PKG_VERSION"));
    }
}
