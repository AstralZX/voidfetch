use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;

pub struct Info {
    pub username: String,
    pub hostname: String,
    pub os: String,
    pub host: String,
    pub kernel: String,
    pub uptime: String,
    pub packages: String,
    pub shell: String,
    pub terminal: String,
    pub de: String,
    pub wm: String,
    pub cpu: String,
    pub gpu: String,
    pub memory: String,
    pub disk: String,
    pub locale: String,
    pub battery: String,
    pub resolution: String,
}

fn run(cmd: &str) -> String {
    Command::new("sh")
        .args(["-c", cmd])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub fn gather() -> Info {
    let (os, host, kernel, uptime, packages, shell, terminal, de, wm, cpu, gpu, memory, disk, locale, battery, resolution) = thread::scope(|s| {
        let t_os = s.spawn(|| get_os());
        let t_host = s.spawn(|| get_host());
        let t_kernel = s.spawn(|| get_kernel());
        let t_uptime = s.spawn(|| get_uptime());
        let t_packages = s.spawn(|| get_packages());
        let t_shell = s.spawn(|| get_shell());
        let t_terminal = s.spawn(|| get_terminal());
        let t_de = s.spawn(|| get_de());
        let t_wm = s.spawn(|| get_wm());
        let t_cpu = s.spawn(|| get_cpu());
        let t_gpu = s.spawn(|| get_gpu());
        let t_memory = s.spawn(|| get_memory());
        let t_disk = s.spawn(|| get_disk());
        let t_locale = s.spawn(|| get_locale());
        let t_battery = s.spawn(|| get_battery());
        let t_resolution = s.spawn(|| get_resolution());

        (
            t_os.join().unwrap_or_default(),
            t_host.join().unwrap_or_default(),
            t_kernel.join().unwrap_or_default(),
            t_uptime.join().unwrap_or_default(),
            t_packages.join().unwrap_or_default(),
            t_shell.join().unwrap_or_default(),
            t_terminal.join().unwrap_or_default(),
            t_de.join().unwrap_or_default(),
            t_wm.join().unwrap_or_default(),
            t_cpu.join().unwrap_or_default(),
            t_gpu.join().unwrap_or_default(),
            t_memory.join().unwrap_or_default(),
            t_disk.join().unwrap_or_default(),
            t_locale.join().unwrap_or_default(),
            t_battery.join().unwrap_or_default(),
            t_resolution.join().unwrap_or_default(),
        )
    });

    Info {
        username: get_user(),
        hostname: get_hostname(),
        os, host, kernel, uptime, packages, shell, terminal,
        de, wm, cpu, gpu, memory, disk, locale, battery, resolution,
    }
}

fn get_user() -> String {
    env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| run("whoami"))
}

fn get_hostname() -> String {
    run("hostname")
}

fn get_os() -> String {
    if cfg!(target_os = "windows") {
        return format!("Windows {}", run("ver"));
    }
    if cfg!(target_os = "macos") {
        return format!("macOS {}", run("sw_vers -productVersion"));
    }

    let data = read_file("/etc/os-release");
    for line in data.lines() {
        if let Some(rest) = line.strip_prefix("PRETTY_NAME=") {
            return rest.trim_matches('"').to_string();
        }
    }

    format!("{} {}", run("uname -s"), run("uname -r"))
}

fn get_host() -> String {
    if cfg!(target_os = "macos") {
        let model = run("sysctl -n hw.model");
        if !model.is_empty() {
            return model;
        }
    }
    if let Ok(data) = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name") {
        let name = data.trim();
        if !name.is_empty() && name != "None" {
            return name.to_string();
        }
    }
    if let Ok(data) = fs::read_to_string("/sys/devices/virtual/dmi/id/board_name") {
        let name = data.trim();
        if !name.is_empty() && name != "None" {
            return name.to_string();
        }
    }
    "N/A".into()
}

fn get_kernel() -> String {
    run("uname -r")
}

fn get_uptime() -> String {
    let data = read_file("/proc/uptime");
    if let Some(first) = data.split_whitespace().next() {
        if let Ok(secs) = first.parse::<u64>() {
            let (d, rem) = (secs / 86400, secs % 86400);
            let (h, m) = (rem / 3600, (rem % 3600) / 60);
            let mut parts = Vec::with_capacity(3);
            if d > 0 { parts.push(format!("{}d", d)); }
            if h > 0 { parts.push(format!("{}h", h)); }
            parts.push(format!("{}m", m));
            return parts.join(" ");
        }
    }
    run("uptime -p").trim_start_matches("up ").to_string()
}

fn get_packages() -> String {
    let mut counts: Vec<String> = Vec::with_capacity(4);

    let check = |cmd: &str, name: &str| -> Option<String> {
        let out = run(cmd);
        let trimmed = out.trim();
        if !trimmed.is_empty() && trimmed != "0" {
            Some(format!("{} ({})", trimmed, name))
        } else {
            None
        }
    };

    if let Some(p) = check("pacman -Q 2>/dev/null | wc -l", "pacman") { counts.push(p); }
    if let Some(p) = check("dpkg -l 2>/dev/null | grep -c '^ii'", "dpkg") { counts.push(p); }
    if let Some(p) = check("rpm -qa 2>/dev/null | wc -l", "rpm") { counts.push(p); }
    if let Some(p) = check("xbps-query -l 2>/dev/null | wc -l", "xbps") { counts.push(p); }
    if let Some(p) = check("flatpak list 2>/dev/null | wc -l", "flatpak") { counts.push(p); }
    if let Some(p) = check("snap list 2>/dev/null | tail -n +2 | wc -l", "snap") { counts.push(p); }
    if let Some(p) = check("apk info 2>/dev/null | wc -l", "apk") { counts.push(p); }

    if counts.is_empty() { "N/A".into() } else { counts.join(", ") }
}

fn get_shell() -> String {
    let shell = env::var("SHELL")
        .or_else(|_| env::var("COMSPEC"))
        .unwrap_or_default();
    if !shell.is_empty() {
        return shell.rsplit('/').next().unwrap_or(&shell).to_string();
    }
    "N/A".into()
}

fn get_terminal() -> String {
    for var in &["WT_SESSION", "TERM_PROGRAM", "TERM", "COLORTERM"] {
        if let Ok(v) = env::var(var) {
            if !v.is_empty() { return v; }
        }
    }
    "N/A".into()
}

fn get_de() -> String {
    for var in &["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION", "GDMSESSION"] {
        if let Ok(v) = env::var(var) {
            if !v.is_empty() { return v; }
        }
    }
    let result = run("basename \"$(ps -o comm= -p $(ps -o ppid= -p $(ps -o ppid= -p $$)))\" 2>/dev/null");
    if result.is_empty() { "N/A".into() } else { result }
}

fn get_wm() -> String {
    if let Ok(v) = env::var("XDG_SESSION_TYPE") {
        if v == "wayland" {
            let wm = run("echo $XDG_CURRENT_DESKTOP");
            if !wm.is_empty() && wm != "N/A" {
                return wm;
            }
        }
    }
    let wmctrl = run("wmctrl -m 2>/dev/null | head -1 | awk '{print $2}'");
    if !wmctrl.is_empty() && wmctrl != "N/A" {
        return wmctrl;
    }
    run("xprop -root _NET_WM_NAME 2>/dev/null | cut -d'\"' -f2")
}

fn get_cpu() -> String {
    let data = read_file("/proc/cpuinfo");
    for line in data.lines() {
        if let Some(val) = line.strip_prefix("model name") {
            let cpu = val.trim().trim_start_matches(':').trim();
            if !cpu.is_empty() {
                let cores = std::thread::available_parallelism()
                    .map(|n| n.get())
                    .unwrap_or(1);
                return format!("{} ({}c)", cpu, cores);
            }
        }
    }
    run("sysctl -n machdep.cpu.brand_string 2>/dev/null")
}

fn get_gpu() -> String {
    let lspci = run("lspci 2>/dev/null | grep -i 'vga\\|3d\\|display'");
    if let Some(line) = lspci.lines().next() {
        if let Some(gpu) = line.split(": ").nth(1) {
            return gpu.to_string();
        }
    }
    "N/A".into()
}

fn get_memory() -> String {
    let data = read_file("/proc/meminfo");
    let mut total: u64 = 0;
    let mut avail: u64 = 0;

    for line in data.lines() {
        if let Some(v) = line.strip_prefix("MemTotal:") {
            total = v.trim().split_whitespace().next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        }
        if let Some(v) = line.strip_prefix("MemAvailable:") {
            avail = v.trim().split_whitespace().next()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
        }
    }

    if total > 0 {
        return format!("{}MB / {}MB", (total - avail) / 1024, total / 1024);
    }

    let memsize = run("sysctl -n hw.memsize 2>/dev/null");
    if let Ok(bytes) = memsize.trim().parse::<u64>() {
        return format!("{}MB", bytes / 1024 / 1024);
    }

    "N/A".into()
}

fn get_disk() -> String {
    let out = run("df -h / 2>/dev/null | tail -1");
    let parts: Vec<&str> = out.split_whitespace().collect();
    if parts.len() >= 5 {
        return format!("{}/{} ({})", parts[2], parts[1], parts[4]);
    }
    "N/A".into()
}

fn get_locale() -> String {
    env::var("LANG")
        .or_else(|_| env::var("LC_ALL"))
        .unwrap_or_else(|_| "N/A".into())
}

fn get_battery() -> String {
    for bat in &["BAT0", "BAT1", "BATT"] {
        let bat_path = format!("/sys/class/power_supply/{}", bat);
        if Path::new(&bat_path).exists() {
            let capacity = fs::read_to_string(format!("{}/capacity", bat_path))
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            let status = fs::read_to_string(format!("{}/status", bat_path))
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            if !capacity.is_empty() {
                return format!("{}% [{}]", capacity, status);
            }
        }
    }

    let out = run("pmset -g batt 2>/dev/null | grep -o '\\d\\+%' | head -1");
    if !out.is_empty() { return out; }

    "N/A".into()
}

fn get_resolution() -> String {
    let out = run("xrandr 2>/dev/null | grep ' connected' | grep -oP '\\d+x\\d+' | head -1");
    if !out.is_empty() { return out; }
    run("system_profiler SPDisplaysDataType 2>/dev/null | grep Resolution | head -1 | awk -F': ' '{print $2}'")
}
