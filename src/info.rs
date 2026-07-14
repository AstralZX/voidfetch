use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

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
    Info {
        username: get_user(),
        hostname: get_hostname(),
        os: get_os(),
        host: get_host(),
        kernel: get_kernel(),
        uptime: get_uptime(),
        packages: get_packages(),
        shell: get_shell(),
        terminal: get_terminal(),
        de: get_de(),
        wm: get_wm(),
        cpu: get_cpu(),
        gpu: get_gpu(),
        memory: get_memory(),
        disk: get_disk(),
        locale: get_locale(),
        battery: get_battery(),
        resolution: get_resolution(),
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
        let v = run("sw_vers -productVersion");
        return format!("macOS {}", v);
    }

    let data = read_file("/etc/os-release");
    for line in data.lines() {
        if let Some(rest) = line.strip_prefix("PRETTY_NAME=") {
            return rest.trim_matches('"').to_string();
        }
    }

    let sysname = run("uname -s");
    let release = run("uname -r");
    format!("{} {}", sysname, release)
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
    run("cat /sys/class/dmi/id/product_name 2>/dev/null || cat /sys/class/dmi/id/board_name 2>/dev/null || echo N/A")
}

fn get_kernel() -> String {
    run("uname -r")
}

fn get_uptime() -> String {
    let data = read_file("/proc/uptime");
    if let Some(first) = data.split_whitespace().next() {
        if let Ok(secs) = first.parse::<u64>() {
            let d = secs / 86400;
            let h = (secs % 86400) / 3600;
            let m = (secs % 3600) / 60;
            let mut result = String::new();
            if d > 0 {
                result.push_str(&format!("{}d ", d));
            }
            if h > 0 {
                result.push_str(&format!("{}h ", h));
            }
            result.push_str(&format!("{}m", m));
            return result;
        }
    }
    run("uptime -p").trim_start_matches("up ").to_string()
}

fn get_packages() -> String {
    let mut counts: Vec<String> = Vec::new();

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
    if let Some(p) = check("apk info 2>/dev/null | wc -l", "apk") { counts.push(p); }
    if let Some(p) = check("xbps-query -l 2>/dev/null | wc -l", "xbps") { counts.push(p); }
    if let Some(p) = check("snap list 2>/dev/null | tail -n +2 | wc -l", "snap") { counts.push(p); }
    if let Some(p) = check("flatpak list 2>/dev/null | wc -l", "flatpak") { counts.push(p); }
    if let Some(p) = check("emerge -e N 2>/dev/null | wc -l", "portage") { counts.push(p); }

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
    if let Ok(v) = env::var("WT_SESSION") {
        if !v.is_empty() { return "Windows Terminal".into(); }
    }
    if let Ok(v) = env::var("TERM_PROGRAM") {
        if !v.is_empty() { return v; }
    }
    if let Ok(v) = env::var("TERM") {
        if !v.is_empty() { return v; }
    }
    env::var("COLORTERM").unwrap_or_else(|_| "N/A".into())
}

fn get_de() -> String {
    if let Ok(v) = env::var("XDG_CURRENT_DESKTOP") {
        if !v.is_empty() { return v; }
    }
    if let Ok(v) = env::var("DESKTOP_SESSION") {
        if !v.is_empty() { return v; }
    }
    if let Ok(v) = env::var("GDMSESSION") {
        if !v.is_empty() { return v; }
    }
    run("basename \"$(ps -o comm= -p $(ps -o ppid= -p $(ps -o ppid= -p $$)))\" 2>/dev/null || echo N/A")
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
    run("xprop -root _NET_WM_NAME 2>/dev/null | cut -d'\"' -f2 || echo N/A")
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
    run("sysctl -n machdep.cpu.brand_string 2>/dev/null || echo N/A")
}

fn get_gpu() -> String {
    let lspci = run("lspci 2>/dev/null | grep -i 'vga\\|3d\\|display'");
    if let Some(line) = lspci.lines().next() {
        if let Some(gpu) = line.split(": ").nth(1) {
            return gpu.to_string();
        }
    }
    let sysgpu = run("lspci -nn 2>/dev/null | grep -i 'vga\\|3d' | head -1 | sed 's/.*: //'");
    if !sysgpu.is_empty() && sysgpu != "N/A" {
        return sysgpu;
    }
    run("system_profiler SPDisplaysDataType 2>/dev/null | grep 'Chipset Model' | head -1 | awk -F': ' '{print $2}'")
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
        let used = total - avail;
        return format!("{}MB / {}MB", used / 1024, total / 1024);
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
    let bat_path = "/sys/class/power_supply/BAT0";
    if Path::new(bat_path).exists() {
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

    let bat_path = "/sys/class/power_supply/BAT1";
    if Path::new(bat_path).exists() {
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

    let out = run("pmset -g batt 2>/dev/null | grep -o '\\d\\+%' | head -1");
    if !out.is_empty() && out != "N/A" {
        return out;
    }

    "N/A".into()
}

fn get_resolution() -> String {
    let out = run("xrandr 2>/dev/null | grep ' connected' | grep -oP '\\d+x\\d+' | head -1");
    if !out.is_empty() && out != "N/A" {
        return out;
    }
    run("system_profiler SPDisplaysDataType 2>/dev/null | grep Resolution | head -1 | awk -F': ' '{print $2}'")
}
