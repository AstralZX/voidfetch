# voidfetch

minimal system information tool. config is CSS.

```
                    -`                  @perfectcachy
                   .o+`                 #############
                  `ooo/                 Os: CachyOS
                 `+oooo:                Host: Gaming PC
                `+oooooo:               Kernel: 7.1.3-2-cachyos-bore
               -+oooooo+:              Uptime: 23 minutes
             `/:-:++oooo+:             Packages: 1445 (pacman)
            `/++++/+++++++:            Shell: zsh
           `/++++++++++++++:           Terminal: xterm-kitty
          `/+++oooooooooooo/`         De: Hyprland
         ./ooosssso++osssssso+`        Wm: Hyprland
        .oossssso-````/ossssss+`       Cpu: 12th Gen Intel i5-12400F (12c)
       -osssssso.      :ssssssso.      Memory: 5061MB / 15792MB
      :osssssss/        osssso+++.     Disk: 35G/928G (4%)
```

## features

- **single rust binary** - no dependencies, no runtime, no bloat
- **CSS config** - configure everything with a `config.css` file
- **61 example themes** - built-in presets ready to use
- **33 built-in themes** - dracula, nord, catppuccin, gruvbox, and more
- **8 style presets** - minimal, compact, fancy, hacker, retro, clean, rainbow
- **9 palettes** - nord, dracula, catppuccin, gruvbox, solarized, tokyo, rainbow, mono, pastel
- **542 distro logos** - with color support
- **16 info fields** - OS, kernel, CPU, GPU, memory, disk, uptime, packages, battery, resolution, and more
- **full color control** - named colors, hex, RGB, 256-color, ANSI
- **variables** - define and reuse values across your config
- **custom lines** - add ASCII art, separators, or any text
- **cross-platform** - linux, macOS, freebsd, openbsd, windows

## installation

### quick install

```sh
git clone https://github.com/AstralZX/voidfetch.git
cd voidfetch
./install.sh
```

this compiles and installs to `~/.local/bin/voidfetch`.

### windows

```powershell
git clone https://github.com/AstralZX/voidfetch.git
cd voidfetch
.\install.ps1
```

### prebuilt binaries

check the [releases](https://github.com/AstralZX/voidfetch/releases) page.

## usage

```
voidfetch [OPTIONS]

OPTIONS:
    -h, --help            print help
    -v, --version         print version
    -c, --config <PATH>   use custom config file
    -e, --example N       use example config by number (1-61)
    --dump-config         print default config to stdout
    --cred                open voidfetch github in browser
    --sync                fetch latest from github, build and install
    --explode             uninstall voidfetch from your system
    --list-themes         list available built-in themes
    --list-examples       list example configs with numbers
    --list-logos          list available distro logos
```

### examples

```sh
# default display
voidfetch

# use example config #7 (catppuccin mocha)
voidfetch --example 7

# use a custom config file
voidfetch --config ~/myconfig.css

# see all available themes
voidfetch --list-themes

# see all example configs
voidfetch --list-examples

# generate a default config
voidfetch --dump-config > ~/.config/voidfetch/config.css
```

## configuration

the config file is searched in order:

1. `--config <path>` or `-c <path>`
2. `$VOIDFETCH_CONFIG` environment variable
3. `~/.config/voidfetch/config.css`
4. `/etc/voidfetch/config.css`

### config example

```css
/* yes, the config is CSS. deal with it. */

@theme catppuccin;

:root {
    separator: "─";
    padding: 2;
    bold: true;
}

@colors {
    user: cyan;
    host: cyan;
    label: cyan;
    value: white;
    separator: gray;
    title: cyan;
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

custom: "════════════════════════════════";
custom: "  voidfetch - embrace the void";
custom: "════════════════════════════════";
```

### import example configs

```css
@import "04-dracula.css";
@import "07-catppuccin-mocha.css";
@import "09-nord.css";
```

### quick shortcuts

```css
/* theme preset */
@theme dracula;
@theme nord;
@theme catppuccin;

/* style preset */
@style minimal;
@style compact;
@style fancy;

/* palette */
@palette nord;
@palette dracula;

/* single color override */
@color { user: red; }

/* separator */
@separator "═";

/* padding */
@margin 4;

/* text effects */
@italic true;
@underline true;
@glow true;

/* reset everything */
@reset;
```

### variables

```css
$accent: #88c0d0;
$username: void;

@colors {
    user: $accent;
    host: $accent;
}
```

variables resolve recursively up to 5 levels deep.

### color options

| format | example |
|--------|---------|
| named | `red`, `cyan`, `orange`, `turquoise`, `pink`, `gold` |
| hex | `#ff6600` |
| rgb | `rgb(255, 102, 0)` |
| 256-color | `256(208)` |
| ANSI | `ansi(3)` |
| none | `none` |

available named colors: black, red, green, yellow, blue, magenta, cyan, white, gray, orange, pink, lime, violet, indigo, coral, salmon, gold, crimson, turquoise, aqua, purple, teal.

### info fields

| field | description | default |
|-------|-------------|---------|
| `os` | operating system | on |
| `host` | hardware model | on |
| `kernel` | kernel version | on |
| `uptime` | system uptime | on |
| `packages` | package count | on |
| `shell` | default shell | on |
| `terminal` | terminal emulator | on |
| `de` | desktop environment | on |
| `wm` | window manager | on |
| `cpu` | processor | on |
| `gpu` | graphics card | on |
| `memory` | ram usage | on |
| `disk` | disk usage | on |
| `locale` | system locale | on |
| `battery` | battery status | on |
| `resolution` | display resolution | on |

toggle any field in `@info { field: true/false; }`.

### built-in themes (33)

| theme | description |
|-------|-------------|
| arctic | Arctic Frost - icy blues and whites |
| sunset | Sunset Fire - warm oranges and reds |
| neon | Neon Cyberpunk - vibrant neon colors |
| dracula | Dracula - purple and pink |
| tokyo | Tokyo Night - blue and purple |
| gruvbox | Gruvbox Dark - yellow and orange |
| catppuccin | Catppuccin Mocha - purple, pink, teal |
| monokai | Monokai Pro - classic monokai |
| nord | Nord - frost blue palette |
| onedark | One Dark - Atom one dark |
| rosepine | Rose Pine - soft pink and purple |
| solarized | Solarized Dark |
| github | GitHub Dark |
| palenight | Palenight - material palenight |
| matrix | Matrix Green - green on black |
| vaporwave | Vaporwave - pink, cyan, green |
| retro | Retro Terminal - classic green |
| void | Void Purple |
| sakura | Sakura Pink - cherry blossom |
| blood | Blood Moon - deep crimson |
| ocean | Midnight Ocean - deep sea blues |
| forest | Forest Green - natural greens |
| lavender | Lavender Dreams - soft pastels |
| amber | Amber Glow - warm golden |
| emerald | Emerald Sea - jewel greens |
| ice | Ice Blue - minimal icy |
| pastel | Pastel Dream - soft pastels |
| crimson | Crimson Tide - deep red |
| golden | Golden Hour - warm golden |
| space | Space Gray - dark minimal |
| royal | Royal Purple |
| abyss | Abyssal Deep - deep ocean |
| solar | Solar Flare - bright solar |

### style presets (8)

| style | description |
|-------|-------------|
| minimal | few fields, dim text, no logo extras |
| compact | uppercase labels, tight spacing |
| full | default, all fields enabled |
| fancy | box-drawing borders, glow effect |
| hacker | green-on-black, system initialized |
| retro | old-school green terminal |
| clean | no logo, no effects |
| rainbow | multicolor everything |

### palettes (9)

nord, dracula, catppuccin, gruvbox, solarized, tokyo, rainbow, mono, pastel.

## supported distros

542 distro logos with automatic detection. includes: arch, manjaro, endeavouros, garuda, cachyos, artix, ubuntu, kubuntu, xubuntu, lubuntu, pop!\_os, zorin, debian, kali, parrot, linux mint, lmde, fedora, nobara, bazzite, ultramarine, rhel, centos, almalinux, rocky, oracle, amazon linux, opensuse (leap, tumbleweed, microos, slowroll), alpine, void, gentoo, nixos, slackware, mx linux, elementary, deepin, sparky, tails, manjaro, artix, freebsd, openbsd, netbsd, dragonflybsd, haiku, android, macos, windows, steamdeck, steamos, and 470+ more. run `voidfetch --list-logos` to see all.

## building

```sh
cargo build --release
```

the binary is at `target/release/voidfetch`.

## project structure

```
voidfetch/
├── Cargo.toml
├── src/
│   ├── main.rs        # CLI, rendering, sync/explode
│   ├── config.rs      # CSS parser, themes, palettes, styles
│   ├── info.rs        # parallel system info gathering
│   ├── logo.rs        # logo loader & colorizer
│   └── ansi.rs        # ANSI color utilities
├── logos/             # 542 distro ASCII art files
├── examples/          # 61 example CSS config presets
├── install.sh         # linux/macOS installer
└── install.ps1        # windows installer
```

## license

[AGPL-3.0](LICENSE)
