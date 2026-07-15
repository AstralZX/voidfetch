# voidfetch

minimal system info tool. config is CSS.

```
           .-------------------------:                 @perfectcachy
          .+========================.                 #############
         :++===++===============- :++-        Os: CachyOS
        :*++====+++++===========-        .==:        Host: Gaming PC
       -*+++=====+***++========= Kernel: 7.1.3-2-cachyos-bore
      =*++++=======------------:                      Uptime: 23 minutes
     =*+++++====-                     ...             Packages: 1445 (pacman)
   .+*+++++=-===:                    .=++=:           Shell: zsh
  :++++=====-==:                     -****+           Terminal: xterm-kitty
 :++=======-=.                      .=+**+.          De: Hyprland
.+==========-.                                    Wm: Hyprland
 :++++++++====-                                .--==-.  Cpu: 12th Gen Intel i5-12400F (12c)
  :++==========.                             :++++++:  Gpu: NVIDIA GeForce RTX 3050
   .-===========.                            =*****+*+ Memory: 5061MB / 15792MB
    .-===========:                           .+*****+:  Disk: 35G/928G (4%)
      -=======++++:::::::::::::::::::::::::-:  .---:   Locale: en_SE.UTF-8
       :======++++====+++******************=.          Resolution: 1920x1080
        :=====+++==========++++++++++++++*-
         .====++==============++++++++++*-
          .===+==================+++++++:
           .-======================+++:
             ..........................
```

## what is this

single rust binary, no dependencies. configure everything with a `config.css` file.
supports linux, macOS, freebsd, openbsd, windows.

## install

```sh
git clone https://github.com/AstralZX/voidfetch.git
cd voidfetch
./install.sh
```

installs to `~/.local/bin/voidfetch`. windows users can use `install.ps1`.

or grab a binary from [releases](https://github.com/AstralZX/voidfetch/releases).

## usage

```
voidfetch [OPTIONS]

-h, --help            print help
-v, --version         print version
-c, --config <PATH>   use custom config file
-e, --example N       use example config by number (1-61)
--logo <NAME>         override distro logo
--dump-config         print default config to stdout
--cred                open github in browser
--sync                update from github
--explode             uninstall
--list-themes         list built-in themes
--list-examples       list example configs
--list-logos          list available logos
```

```sh
voidfetch                      # default
voidfetch --example 7          # use example config #7
voidfetch --logo arch          # force a logo
voidfetch --config ~/my.css    # custom config
voidfetch --dump-config > ~/.config/voidfetch/config.css
```

## config

searched in order:
1. `--config <path>`
2. `$VOIDFETCH_CONFIG` env var
3. `~/.config/voidfetch/config.css`
4. `/etc/voidfetch/config.css`

```css
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

custom: "════════════════════════════════";
custom: "  voidfetch - embrace the void";
custom: "════════════════════════════════";
```

### shortcuts

```css
@theme dracula;         /* theme preset */
@style minimal;         /* style preset */
@palette nord;          /* palette */
@color { user: red; }   /* single color */
@separator "═";
@margin 4;
@italic true;
@underline true;
@glow true;
@reset;                 /* reset all */
```

### variables

```css
$accent: #88c0d0;

@colors {
    user: $accent;
    host: $accent;
}
```

resolves recursively up to 5 levels.

### colors

named: `red`, `cyan`, `orange`, `turquoise`, `pink`, `gold`, etc
hex: `#ff6600`
rgb: `rgb(255, 102, 0)`
256-color: `256(208)`
ansi: `ansi(3)`

### import

```css
@import "04-dracula.css";
@import "07-catppuccin-mocha.css";
```

## themes

33 built-in themes: arctic, sunset, neon, dracula, tokyo, gruvbox, catppuccin, monokai, nord, onedark, rosepine, solarized, github, palenight, matrix, vaporwave, retro, void, sakura, blood, ocean, forest, lavender, amber, emerald, ice, pastel, crimson, golden, space, royal, abyss, solar

## styles

minimal, compact, full, fancy, hacker, retro, clean, rainbow

## palettes

nord, dracula, catppuccin, gruvbox, solarized, tokyo, rainbow, mono, pastel

## distros

542 logos with auto-detection. run `voidfetch --list-logos` to see all.

## building

```sh
cargo build --release
```

binary at `target/release/voidfetch`.

## license

[AGPL-3.0](LICENSE)
