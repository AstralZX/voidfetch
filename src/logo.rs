use std::env;
use std::fs;
use std::path::PathBuf;

use crate::ansi;
use crate::config::Config;

fn logos_dir() -> PathBuf {
    let exe = env::current_exe().unwrap_or_default();
    let exe_dir = exe.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();

    let candidates = vec![
        exe_dir.join("logos"),
        exe_dir.join("..").join("logos"),
        exe_dir.join("..").join("..").join("logos"),
        exe_dir.join("../share/voidfetch/logos"),
        PathBuf::from("/usr/share/voidfetch/logos"),
        PathBuf::from("/usr/local/share/voidfetch/logos"),
    ];

    for p in &candidates {
        if p.is_dir() {
            let has_files = fs::read_dir(p)
                .map(|mut r| r.next().is_some())
                .unwrap_or(false);
            if has_files {
                return p.clone();
            }
        }
    }

    if let Ok(home) = env::var("HOME") {
        let local = PathBuf::from(&home).join(".local").join("bin").join("logos");
        if local.is_dir() {
            return local;
        }
    }

    exe_dir.join("logos")
}

fn detect_distro() -> String {
    if cfg!(target_os = "windows") {
        return "windows".into();
    }
    if cfg!(target_os = "macos") {
        return "macos".into();
    }

    let data = fs::read_to_string("/etc/os-release").unwrap_or_default();
    let mut id = String::new();
    let mut id_like = String::new();

    for line in data.lines() {
        if let Some(v) = line.strip_prefix("ID=") {
            id = v.trim_matches('"').to_lowercase();
        }
        if let Some(v) = line.strip_prefix("ID_LIKE=") {
            id_like = v.trim_matches('"').to_lowercase();
        }
    }

    let aliases = [
        ("arch", "arch"), ("manjaro", "manjaro"), ("endeavouros", "endeavouros"),
        ("garuda", "garuda"), ("artix", "arch"), ("cachyos", "arch"),
        ("ubuntu", "ubuntu"), ("xubuntu", "xubuntu"), ("lubuntu", "lubuntu"),
        ("kubuntu", "ubuntu"), ("pop", "popos"), ("zorin", "zorin"),
        ("debian", "debian"), ("raspbian", "raspbian"), ("kali", "kali"),
        ("parrot", "parrot"), ("linuxmint", "mint"), ("lmde", "debian"),
        ("fedora", "fedora"), ("nobara", "nobara"), ("bazzite", "bazzite"),
        ("ultramarine", "ultramarine"), ("rhel", "redhat"), ("centos", "centos"),
        ("almalinux", "alma"), ("rocky", "rocky"), ("ol", "oracle"),
        ("amzn", "amazon_linux"), ("opensuse-leap", "opensuse"),
        ("opensuse-tumbleweed", "suse"), ("sles", "suse"),
        ("alpine", "alpine"), ("void", "void"), ("gentoo", "gentoo"),
        ("funtoo", "gentoo"), ("solus", "solus"), ("pardus", "pardus"),
        ("nixos", "nixos"), ("slackware", "slackware"), ("mx", "mx"),
        ("lynx", "lynx"), ("feren", "feren"), ("asahi", "asahi"),
        ("android", "android"), ("haiku", "haiku"),
    ];

    for (distro_id, logo_name) in &aliases {
        if id == *distro_id {
            return logo_name.to_string();
        }
    }

    for part in id_like.split_whitespace() {
        for (distro_id, logo_name) in &aliases {
            if part == *distro_id {
                return logo_name.to_string();
            }
        }
    }

    let uname = std::process::Command::new("uname")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_lowercase())
        .unwrap_or_default();

    match uname.as_str() {
        "freebsd" => "freebsd".into(),
        "openbsd" => "openbsd".into(),
        "netbsd" => "netbsd".into(),
        "dragonfly" => "dragonfly".into(),
        _ => "unknown".into(),
    }
}

pub fn get(cfg: &Config) -> Option<Vec<String>> {
    if !cfg.logo.enabled {
        return None;
    }

    let distro = if cfg.logo.distro == "auto" {
        detect_distro()
    } else {
        cfg.logo.distro.clone()
    };

    let logos = logos_dir();

    let candidates = vec![
        format!("{}.txt", distro),
        format!("{}_small.txt", distro),
    ];

    for candidate in &candidates {
        let path = logos.join(candidate);
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                let lines: Vec<String> = content
                    .lines()
                    .map(|l| apply_logo_colors(l, &cfg.colors.logo))
                    .collect();
                return Some(lines);
            }
        }
    }

    None
}

fn apply_logo_colors(line: &str, logo_color: &str) -> String {
    let base_color = if logo_color == "auto" || logo_color == "none" {
        ansi::color("cyan")
    } else {
        ansi::color(logo_color)
    };

    let mut result = String::new();
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            if let Some(&next) = chars.peek() {
                if next.is_ascii_digit() {
                    chars.next();
                    let color_char = next.to_string();
                    match color_char.as_str() {
                        "1" => result.push_str("\x1b[31m"),
                        "2" => result.push_str("\x1b[32m"),
                        "3" => result.push_str("\x1b[33m"),
                        "4" => result.push_str("\x1b[34m"),
                        "5" => result.push_str("\x1b[35m"),
                        "6" => result.push_str("\x1b[36m"),
                        "7" => result.push_str("\x1b[37m"),
                        "8" => result.push_str("\x1b[90m"),
                        "9" => result.push_str("\x1b[91m"),
                        _ => result.push_str(&base_color),
                    }
                    continue;
                }
            }
            result.push(c);
        } else {
            result.push(c);
        }
    }

    result
}
