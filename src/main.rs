use std::env;

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
