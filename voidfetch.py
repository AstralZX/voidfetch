#!/usr/bin/env python3
"""
voidfetch - minimal system info
"""

import os
import sys
import platform
import subprocess
import re
from pathlib import Path

VERSION = "1.0.0"

BOLD = "\033[1m"
RESET = "\033[0m"
DIM = "\033[2m"

COLORS = {
    "$1": "\033[31m",
    "$2": "\033[32m",
    "$3": "\033[33m",
    "$4": "\033[34m",
    "$5": "\033[35m",
    "$6": "\033[36m",
    "$7": "\033[37m",
    "$8": "\033[90m",
    "$9": "\033[91m",
}

COLOR_NAMES = {
    "red": "\033[31m",
    "green": "\033[32m",
    "yellow": "\033[33m",
    "blue": "\033[34m",
    "magenta": "\033[35m",
    "cyan": "\033[36m",
    "white": "\033[37m",
    "gray": "\033[90m",
    "lightred": "\033[91m",
    "lightgreen": "\033[92m",
    "lightyellow": "\033[93m",
    "lightblue": "\033[94m",
    "lightmagenta": "\033[95m",
    "lightcyan": "\033[96m",
}


def colorize(text):
    for code, ansi in COLORS.items():
        text = text.replace(code, ansi)
    return text


def strip_colors(text):
    return re.sub(r"\$\d", "", text)


def run(cmd):
    try:
        r = subprocess.run(cmd, shell=True, capture_output=True, text=True, timeout=5)
        return r.stdout.strip()
    except Exception:
        return ""


def get_logo_path():
    logos_dir = Path(__file__).parent / "logos"

    if sys.platform == "win32":
        ver = platform.version()
        if "10.0.22" in ver or "10.0.2" in ver:
            return logos_dir / "windows11.txt"
        return logos_dir / "windows10.txt"

    if sys.platform == "darwin":
        return logos_dir / "macos.txt"

    os_release = Path("/etc/os-release")
    if os_release.exists():
        data = os_release.read_text().lower()
        id_match = re.search(r'^id="?(\w+)"?', data, re.M)
        id_like_match = re.search(r'^id_like="?([\w\s]+)"?', data, re.M)
        distro_id = id_match.group(1) if id_match else ""
        distro_id_like = id_like_match.group(1) if id_like_match else ""

        mapping = {
            "arch": "arch.txt",
            "manjaro": "manjaro.txt",
            "endeavouros": "endeavouros.txt",
            "garuda": "garuda.txt",
            "artix": "arch.txt",
            "ubuntu": "ubuntu.txt",
            "xubuntu": "xubuntu.txt",
            "lubuntu": "lubuntu.txt",
            "pop": "popos.txt",
            "zorin": "zorin.txt",
            "debian": "debian.txt",
            "raspbian": "raspbian.txt",
            "kali": "kali.txt",
            "parrot": "parrot.txt",
            "linuxmint": "mint.txt",
            "fedora": "fedora.txt",
            "nobara": "nobara.txt",
            "bazzite": "bazzite.txt",
            "ultramarine": "ultramarine.txt",
            "rhel": "redhat.txt",
            "centos": "centos.txt",
            "almalinux": "alma.txt",
            "rocky": "rocky.txt",
            "ol": "oracle.txt",
            "amzn": "amazon_linux.txt",
            "opensuse": "opensuse.txt",
            "opensuse-leap": "opensuse.txt",
            "opensuse-tumbleweed": "suse.txt",
            "sles": "suse.txt",
            "alpine": "alpine.txt",
            "void": "void.txt",
            "gentoo": "gentoo.txt",
            "funtoo": "gentoo.txt",
            "solus": "solus.txt",
            "pardus": "pardus.txt",
            "nixos": "nixos.txt",
            "slackware": "slackware.txt",
            "mx": "mx.txt",
            "lynx": "lynx.txt",
            "feren": "feren.txt",
            "asahi": "asahi.txt",
        }

        if distro_id in mapping:
            return logos_dir / mapping[distro_id]
        for alias in distro_id_like.split():
            if alias in mapping:
                return logos_dir / mapping[alias]

    uname = run("uname").lower()
    if "freebsd" in uname:
        return logos_dir / "freebsd.txt"
    if "openbsd" in uname:
        return logos_dir / "openbsd.txt"
    if "netbsd" in uname:
        return logos_dir / "netbsd.txt"
    if "dragonfly" in uname:
        return logos_dir / "dragonfly.txt"

    return None


def load_logo(path):
    if not path or not path.exists():
        return None
    return path.read_text(errors="replace")


def get_user():
    return os.environ.get("USER") or os.environ.get("USERNAME") or run("whoami")


def get_hostname():
    try:
        return platform.node()
    except Exception:
        return run("hostname")


def get_os():
    if sys.platform == "win32":
        return f"Windows {platform.release()}"
    if sys.platform == "darwin":
        ver = platform.mac_ver()[0]
        return f"macOS {ver}"

    os_release = Path("/etc/os-release")
    if os_release.exists():
        data = os_release.read_text()
        name_match = re.search(r'^PRETTY_NAME="?([^"]+)"?', data, re.M)
        if name_match:
            return name_match.group(1)
    return f"{run('uname -s')} {run('uname -r')}"


def get_kernel():
    return platform.release()


def get_uptime():
    if sys.platform == "win32":
        t = int(run("wmic os get lastbootuptime")) if False else 0
        if t:
            return str(t)
        return run("systeminfo | findstr Boot")

    up = run("cat /proc/uptime")
    if up:
        secs = int(float(up.split()[0]))
        d, rem = divmod(secs, 86400)
        h, rem = divmod(rem, 3600)
        m, s = divmod(rem, 60)
        parts = []
        if d:
            parts.append(f"{d}d")
        if h:
            parts.append(f"{h}h")
        if m:
            parts.append(f"{m}m")
        if not parts:
            parts.append(f"{s}s")
        return " ".join(parts)

    up = run("uptime -p")
    if up:
        return up.replace("up ", "")
    return run("uptime")


def get_packages():
    counts = []
    if sys.platform != "win32" and sys.platform != "darwin":
        pkgmanagers = [
            ("pacman", "pacman -Q 2>/dev/null | wc -l"),
            ("dpkg", "dpkg -l 2>/dev/null | grep '^ii' | wc -l"),
            ("rpm", "rpm -qa 2>/dev/null | wc -l"),
            ("apk", "apk info 2>/dev/null | wc -l"),
            ("xbps-query", "xbps-query -l 2>/dev/null | wc -l"),
            ("nix", "nix-store -qR /run/current-system/sw 2>/dev/null | wc -l"),
            ("snap", "snap list 2>/dev/null | tail -n +2 | wc -l"),
            ("flatpak", "flatpak list 2>/dev/null | wc -l"),
            ("emerge", "ls /var/db/pkg/*/* 2>/dev/null | wc -l"),
            ("lsblk", None),
        ]
        for name, cmd in pkgmanagers:
            if cmd:
                out = run(cmd)
                if out and out.strip() and out.strip() != "0":
                    counts.append(f"{out.strip()} ({name})")
    return ", ".join(counts) if counts else "N/A"


def get_shell():
    shell = os.environ.get("SHELL") or os.environ.get("COMSPEC") or "unknown"
    return Path(shell).name


def get_terminal():
    if sys.platform == "win32":
        term = os.environ.get("WT_SESSION")
        if term:
            return "Windows Terminal"
        term = os.environ.get("TERM_PROGRAM")
        if term:
            return term
        return "conhost"

    term = os.environ.get("TERM_PROGRAM")
    if term:
        return term
    term = os.environ.get("TERM")
    if term:
        return term
    return os.environ.get("COLORTERM", "unknown")


def get_cpu():
    if sys.platform == "win32":
        return run("wmic cpu get name /value").replace("Name=", "").strip()

    cpuinfo = Path("/proc/cpuinfo")
    if cpuinfo.exists():
        data = cpuinfo.read_text()
        match = re.search(r"model name\s*:\s*(.+)", data)
        if match:
            return match.group(1).strip()

    return run("sysctl -n machdep.cpu.brand_string 2>/dev/null") or "unknown"


def get_memory():
    if sys.platform == "win32":
        out = run("wmic OS get FreePhysicalMemory,TotalVisibleMemorySize /Value")
        if out:
            import re as _re
            total = _re.search(r"TotalVisibleMemorySize=(\d+)", out)
            free = _re.search(r"FreePhysicalMemory=(\d+)", out)
            if total and free:
                t = int(total.group(1)) // 1024
                f = int(free.group(1)) // 1024
                u = t - f
                return f"{u}MB / {t}MB"

    meminfo = Path("/proc/meminfo")
    if meminfo.exists():
        data = meminfo.read_text()
        total = int(re.search(r"MemTotal:\s+(\d+)", data).group(1)) // 1024
        avail = int(re.search(r"MemAvailable:\s+(\d+)", data).group(1)) // 1024
        used = total - avail
        return f"{used}MB / {total}MB"

    out = run("sysctl hw.memsize hw.physmem 2>/dev/null")
    if out:
        for line in out.splitlines():
            if "memsize" in line:
                total = int(line.split(":")[1].strip()) // 1024 // 1024
                return f"{total}MB"
    return "N/A"


def get_gpu():
    if sys.platform == "win32":
        out = run("wmic path win32_videocontroller get name /value")
        if out:
            for line in out.splitlines():
                if "Name=" in line and line.split("=", 1)[1].strip():
                    return line.split("=", 1)[1].strip()

    out = run("lspci 2>/dev/null | grep -i 'vga\\|3d\\|display'")
    if out:
        return out.splitlines()[0].split(": ", 1)[-1].strip() if ": " in out else out.strip()

    out = run("system_profiler SPDisplaysDataType 2>/dev/null | grep 'Chipset Model\\|Chipset'")
    if out:
        return out.splitlines()[0].split(": ", 1)[-1].strip()

    return "N/A"


def get_disk():
    if sys.platform == "win32":
        out = run("wmic logicaldisk get size,freespace,caption /format:csv")
        if out:
            parts = []
            for line in out.splitlines()[1:]:
                if line.strip():
                    f = line.split(",")
                    if len(f) >= 4 and f[1].strip():
                        try:
                            total = int(f[3].strip()) // 1024 // 1024
                            free = int(f[2].strip()) // 1024 // 1024
                            used = total - free
                            parts.append(f"{f[1].strip()} {used}MB/{total}MB")
                        except ValueError:
                            pass
            return ", ".join(parts) if parts else "N/A"

    out = run("df -h / 2>/dev/null | tail -1")
    if out:
        parts = out.split()
        if len(parts) >= 5:
            return f"{parts[2]}/{parts[1]} ({parts[4]})"
    return "N/A"


def get_locale():
    if sys.platform == "win32":
        return run("powershell -Command (Get-Culture).Name")
    return os.environ.get("LANG", os.environ.get("LC_ALL", "unknown"))


def render_logo(logo_text, lines, color):
    logo_lines = logo_text.rstrip("\n").splitlines()
    max_logo_width = 0
    cleaned = []
    for line in logo_lines:
        clean = strip_colors(line)
        cleaned.append((line, clean))
        if len(clean) > max_logo_width:
            max_logo_width = len(clean)

    result = []
    for i in range(max(len(logo_lines), len(lines))):
        if i < len(cleaned):
            logo_part = colorize(cleaned[i][0])
            padding = max_logo_width - len(cleaned[i][1])
            logo_part += " " * padding
        else:
            logo_part = " " * max_logo_width

        if i < len(lines):
            result.append(f" {logo_part} {color}{lines[i]}{RESET}")
        else:
            result.append(f" {logo_part}")

    return "\n".join(result)


def main():
    logo_path = get_logo_path()
    logo = load_logo(logo_path)

    if not logo:
        print(f" {BOLD}voidfetch{RESET} v{VERSION}")
        print(f" {DIM}no logo for current distro{RESET}")
        print()
    else:
        color = COLORS.get("$6", "\033[36m")

        info_parts = []
        info_parts.append(f"{BOLD}{get_user()}{RESET}@{BOLD}{get_hostname()}{RESET}")
        info_parts.append(f"{'─' * (len(get_user()) + len(get_hostname()) + 1)}")
        info_parts.append(f"OS: {get_os()}")
        info_parts.append(f"Host: {platform.node()}")
        info_parts.append(f"Kernel: {get_kernel()}")
        info_parts.append(f"Uptime: {get_uptime()}")
        info_parts.append(f"Packages: {get_packages()}")
        info_parts.append(f"Shell: {get_shell()}")
        info_parts.append(f"Terminal: {get_terminal()}")
        info_parts.append(f"CPU: {get_cpu()}")
        info_parts.append(f"Memory: {get_memory()}")
        info_parts.append(f"GPU: {get_gpu()}")
        info_parts.append(f"Disk: {get_disk()}")
        info_parts.append(f"Locale: {get_locale()}")

        print(render_logo(logo, info_parts, color))


if __name__ == "__main__":
    main()
