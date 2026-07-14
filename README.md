<p align="center">
  <img src="https://raw.githubusercontent.com/AstralZX/voidfetch/master/.github/banner.png" width="100%" alt="voidfetch banner">
</p>

<h3 align="center">voidfetch</h3>

<p align="center">
  <strong>minimal system information</strong><br>
  <sub>blazingly fast, obsessively customizable, zero bloat</sub>
</p>

<p align="center">
  <a href="#installation"><img src="https://img.shields.io/badge/install-guide-blue?style=flat-square" alt="install"></a>
  <a href="https://github.com/AstralZX/voidfetch/blob/master/LICENSE"><img src="https://img.shields.io/github/license/AstralZX/voidfetch?style=flat-square&color=green" alt="license"></a>
  <a href="https://github.com/AstralZX/voidfetch/releases"><img src="https://img.shields.io/github/v/release/AstralZX/voidfetch?style=flat-square&color=orange" alt="version"></a>
  <a href="https://github.com/AstralZX/voidfetch/stargazers"><img src="https://img.shields.io/github/stars/AstralZX/voidfetch?style=flat-square&color=yellow" alt="stars"></a>
</p>

---

## what is this

voidfetch is a system information tool written in **Rust**. it displays your OS, CPU, GPU, memory, and more alongside colored ASCII art for your distro.

**yes, the config format is CSS.** that is a feature, not a bug.

## features

- **rust binary** - fast startup, tiny memory footprint, no runtime dependencies
- **CSS config** - configure everything with a `config.css` file
- **50+ distro logos** - sourced from fastfetch, with color support
- **20+ info fields** - OS, kernel, CPU, GPU, memory, disk, uptime, packages, battery, resolution, and more
- **full color control** - named colors, hex (`#ff6600`), RGB (`rgb(255,102,0)`), 256-color, ANSI
- **custom lines** - add ASCII art, separators, or any text to output
- **cross-platform** - linux, macOS, freebsd, openbsd, windows

## installation

### from source

```sh
git clone https://github.com/AstralZX/voidfetch.git
cd voidfetch
chmod +x install.sh
./install.sh
```

this will:
1. compile the rust binary with `cargo build --release`
2. install to `~/.local/bin/voidfetch`
3. copy logos to `~/.local/bin/logos/`
4. create a default config at `~/.config/voidfetch/config.css`

### prebuilt binary

check the [releases](https://github.com/AstralZX/voidfetch/releases) page for prebuilt binaries.

### windows (powershell)

```powershell
git clone https://github.com/AstralZX/voidfetch.git
cd voidfetch
.\install.ps1
```

requires rust installed from [rustup.rs](https://rustup.rs).

## usage

```
voidfetch [OPTIONS]

OPTIONS:
    -h, --help       print help
    -v, --version    print version
    --dump-config    print default config to stdout
    --config <PATH>  use custom config file
```

## configuration

voidfetch uses **CSS** as its config format. the config is searched in:

1. `--config <path>`
2. `$VOIDFETCH_CONFIG`
3. `~/.config/voidfetch/config.css`
4. `/etc/voidfetch/config.css`

generate the default config:

```sh
voidfetch --dump-config > ~/.config/voidfetch/config.css
```

### config example

```css
/* yes this is css. deal with it. */

:root {
    separator: "─";
    padding: 2;
    bold: true;
    dim: false;
    color-mode: full;
}

@colors {
    user: cyan;
    host: cyan;
    label: cyan;
    value: white;
    separator: gray;
    title: cyan;
    logo: auto;
}

@info {
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
}

@logo {
    enabled: true;
    distro: auto;
    color: auto;
}

@title {
    enabled: true;
    format: "{user}@{host}";
}

@labels {
    capitalize: true;
    uppercase: false;
}

@custom {
    line: "════════════════════════════════";
    line: "  voidfetch - embrace the void";
    line: "════════════════════════════════";
}
```

### color options

```css
/* named */
color: red;
color: cyan;
color: orange;
color: turquoise;

/* hex */
color: #ff6600;

/* rgb */
color: rgb(255, 102, 0);

/* 256-color */
color: 256(208);

/* ansi */
color: ansi(3);

/* disable */
color: none;
```

### available info fields

| field | description |
|-------|-------------|
| `os` | operating system |
| `host` | hardware model |
| `kernel` | kernel version |
| `uptime` | system uptime |
| `packages` | package count |
| `shell` | default shell |
| `terminal` | terminal emulator |
| `de` | desktop environment |
| `wm` | window manager |
| `cpu` | processor |
| `gpu` | graphics card |
| `memory` | ram usage |
| `disk` | disk usage |
| `locale` | system locale |
| `battery` | battery status |
| `resolution` | display resolution |

toggle any field in `@info { ... }`.

## supported distros

arch, manjaro, endeavouros, garuda, cachyos, artix, ubuntu, xubuntu, kubuntu, lubuntu, pop!\_os, zorin, debian, raspbian, kali, parrot, linux mint, fedora, nobara, bazzite, ultramarine, rhel, centos, almalinux, rocky, oracle, amazon linux, opensuse, suse, alpine, void, gentoo, funtoo, solus, pardus, nixos, slackware, mx linux, lynx, feren, asahi, freebsd, openbsd, netbsd, dragonflybsd, macOS, windows, and more.

## building

```sh
# debug
cargo build

# release (optimized)
cargo build --release

# run directly
cargo run --release
```

the release binary is stripped and LTO-optimized for minimum size.

## project structure

```
voidfetch/
├── Cargo.toml
├── src/
│   ├── main.rs        # entry point, rendering
│   ├── config.rs      # CSS config parser
│   ├── info.rs        # system info gatherer
│   ├── logo.rs        # logo loader & colorizer
│   └── ansi.rs        # ANSI color utilities
├── logos/             # 50+ distro ASCII art files
├── install.sh         # linux/macOS installer
└── install.ps1        # windows installer
```

## license

[MIT](LICENSE)

---

<p align="center">
  <sub>built with rust and questionable design decisions</sub>
</p>
