use std::env;
use std::fs;
use std::path::PathBuf;

use crate::ansi;
use crate::config::Config;

pub fn logos_dir() -> PathBuf {
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

    let aliases: &[(&str, &str)] = &[
        // ── Arch-based ──
        ("arch", "arch"),
        ("archlinux", "arch"),
        ("manjaro", "manjaro"),
        ("endeavouros", "endeavouros"),
        ("endeavour", "endeavouros"),
        ("garuda", "garuda"),
        ("artix", "artix"),
        ("cachyos", "cachyos"),
        ("blackarch", "blackarch"),
        ("archcraft", "archcraft"),
        ("archlabs", "archlabs"),
        ("archstrike", "archstrike"),
        ("anarchy", "anarchy"),
        ("arcolinux", "arco"),
        ("arco", "arco"),
        ("cleanjaro", "cleanjaro"),
        ("swagarch", "swagarch"),
        ("xeroarch", "xeroarch"),
        ("berserkarch", "berserkarch"),
        ("bedrock", "bedrock"),
        ("blendos", "arch"),
        ("vanillalinux", "arch"),

        // ── Debian-based ──
        ("debian", "debian"),
        ("raspbian", "debian"),
        ("devuan", "devuan"),
        ("antix", "antix"),
        ("antixos", "antix"),
        ("bunsenlabs", "bunsenlabs"),
        ("parsix", "parsix"),
        ("kaisen", "kaisen"),
        ("siduction", "siduction"),
        ("deepin", "deepin"),
        ("droidian", "droidian"),
        ("postmarketos", "postmarketos"),
        ("dietpi", "dietpi"),
        ("vzlinux", "vzlinux"),
        ("cumulus", "debian"),

        // ── Ubuntu-based ──
        ("ubuntu", "ubuntu"),
        ("xubuntu", "xubuntu"),
        ("kubuntu", "kubuntu"),
        ("lubuntu", "lubuntu"),
        ("ubuntu-budgie", "ubuntu_budgie"),
        ("ubuntu-cinnamon", "ubuntu_cinnamon"),
        ("ubuntu-gnome", "ubuntu_gnome"),
        ("ubuntu-mate", "ubuntu_mate"),
        ("ubuntu-studio", "ubuntu_studio"),
        ("ubuntu-unity", "ubuntu_unity"),
        ("ubuntu-kylin", "ubuntu_kylin"),
        ("ubuntu-sway", "ubuntu_sway"),
        ("ubuntu-touch", "ubuntu_touch"),
        ("pop", "popos"),
        ("popos", "popos"),
        ("zorin", "zorin"),
        ("elementary", "elementary"),
        ("elementaryos", "elementary"),
        ("kde_neon", "kdeneon"),
        ("neon", "kdeneon"),
        ("peppermint", "peppermint"),
        ("tails", "tails"),
        ("tailsos", "tails"),
        ("regolith", "regolith"),
        ("cosmic", "cosmic"),
        ("bodhi", "bodhi"),
        ("sparky", "sparky"),
        ("linuxlite", "linuxlite"),
        ("linspire", "linspire"),
        ("mx", "mx"),
        ("mxlinux", "mx"),
        ("linuxmint", "linuxmint"),
        ("lmde", "lmde"),
        ("kali", "kali"),
        ("parrot", "parrot"),
        ("cutefishos", "cutefishos"),
        ("cuteos", "cuteos"),
        ("nexalinux", "nexalinux"),
        ("ublinux", "ublinux"),
        ("unifi", "unifi"),
        ("biglinux", "biglinux"),
        ("chrom", "chrom"),
        ("tuxedo_os", "tuxedo_os"),
        ("openkylin", "openkylin"),
        ("uos", "uos"),
        ("linpus", "uos"),
        ("kylin", "kylin"),
        ("harmonyos", "harmonyos"),
        ("gxde", "gxde"),

        // ── Fedora-based ──
        ("fedora", "fedora"),
        ("nobara", "nobara"),
        ("bazzite", "bazzite"),
        ("ultramarine", "ultramarine"),
        ("korora", "korora"),
        ("fedora-kinoite", "fedora_kinoite"),
        ("fedora-silverblue", "fedora_silverblue"),
        ("fedora-sericea", "fedora_sericea"),
        ("fedora-coreos", "fedora_coreos"),
        ("rebl", "fedora"),

        // ── RHEL-based ──
        ("rhel", "rhel"),
        ("rhel_old", "rhel"),
        ("centos", "centos"),
        ("almalinux", "alma"),
        ("alma", "alma"),
        ("rocky", "rocky"),
        ("ol", "oracle"),
        ("oracle", "oracle"),
        ("amzn", "amazon_linux"),
        ("amazon_linux", "amazon_linux"),
        ("eurolinux", "eurolinux"),
        ("scientific", "scientific"),
        ("redhat", "rhel"),
        ("nethydra", "centos"),
        ("truenas", "truenas"),
        ("proxmox", "proxmox"),
        ("cbl_mariner", "cbl_mariner"),
        ("azurelinux", "azurelinux"),
        ("torizoncore", "torizoncore"),

        // ── SUSE-based ──
        ("opensuse-leap", "opensuse_leap"),
        ("opensuse-tumbleweed", "opensuse_tumbleweed"),
        ("opensuse-microos", "opensuse_microos"),
        ("opensuse-slowroll", "opensuse_slowroll"),
        ("opensuse", "opensuse"),
        ("sles", "suse"),
        ("suse", "suse"),
        ("openmamba", "openmamba"),

        // ── Gentoo-based ──
        ("gentoo", "gentoo"),
        ("funtoo", "funtoo"),
        ("exherbo", "exherbo"),
        ("calculate", "calculate"),
        ("source_mage", "source_mage"),
        ("sourcemage", "source_mage"),

        // ── NixOS ──
        ("nixos", "nixos"),

        // ── Slackware-based ──
        ("slackware", "slackware"),
        ("slackel", "slackel"),
        ("salix", "salix"),
        ("pclinuxos", "pclinuxos"),

        // ── Alpine / Void ──
        ("alpine", "alpine"),
        ("void", "void"),

        // ── BSDs (uname fallback handled below, but also in os-release) ──
        ("freebsd", "freebsd"),
        ("openbsd", "openbsd"),
        ("netbsd", "netbsd"),
        ("dragonfly", "dragonfly"),
        ("ghostbsd", "ghostbsd"),
        ("midnightbsd", "midnightbsd"),
        ("nomadbsd", "nomadbsd"),
        ("bitrig", "bitrig"),
        ("bsd", "bsd"),
        ("pacbsd", "pacbsd"),

        // ── Haiku ──
        ("haiku", "haiku"),
        ("haikuos", "haiku"),

        // ── Android / ChromeOS ──
        ("android", "android"),
        ("chromeos", "chromeos"),
        ("chromiumos", "chromeos"),

        // ── macOS / Windows (handled by cfg above, but also in os-release) ──
        ("macos", "macos"),
        ("windows", "windows"),
        ("windows11", "windows_11"),
        ("windows_11", "windows_11"),
        ("windows_10", "windows"),
        ("windows_2025", "windows_2025"),
        ("windows_8", "windows_8"),
        ("windows_95", "windows_95"),

        // ── SteamOS / Steam Deck ──
        ("steamos", "steamos"),
        ("steamdeck", "steamdeck"),
        ("holos", "steamdeck"),

        // ── clear linux ──
        ("clear-linux-os", "clear_linux"),
        ("clear_linux", "clear_linux"),
        ("clear", "clear_linux"),

        // ── Mageia / Mandriva / Mandrake ──
        ("mageia", "mageia"),
        ("mandriva", "mandriva"),
        ("mandrake", "mandriva"),
        ("openmandriva", "openmandriva"),
        ("openruyi", "openruyi"),
        ("rosa", "rosa"),

        // ── Solus ──
        ("solus", "solus"),

        // ── Pardus ──
        ("pardus", "pardus"),

        // ── ALT Linux ──
        ("alt", "altlinux"),
        ("altlinux", "altlinux"),
        ("alt_linux", "altlinux"),

        // ── openEuler ──
        ("openeuler", "openeuler"),
        ("open-euler", "openeuler"),

        // ── AOSC OS ──
        ("aosc", "aoscos"),
        ("aosc_os", "aoscos"),

        // ── Asahi Linux ──
        ("asahi", "asahi"),

        // ── Various independent distros ──
        ("aster", "aster"),
        ("asteroidos", "asteroidos"),
        ("astos", "astos"),
        ("astra", "astra_linux"),
        ("athenaos", "athenaos"),
        ("aurora", "aurora"),
        ("axos", "axos"),
        ("azos", "azos"),
        ("blackmesa", "blackmesa"),
        ("blackpanther", "blackpanther"),
        ("blag", "blag"),
        ("blankon", "blankon"),
        ("bluelight", "bluelight"),
        ("bonsai", "bonsai"),
        ("bredos", "bredos"),
        ("cbl_mariner", "cbl_mariner"),
        ("celos", "celos"),
        ("cereus", "cereus"),
        ("chakra", "chakra"),
        ("chaletos", "chaletos"),
        ("chapeau", "chapeau"),
        ("chimera", "chimera_linux"),
        ("chimera_linux", "chimera_linux"),
        ("chonkysealos", "chonkysealos"),
        ("clover", "clover"),
        ("cobalt", "cobalt"),
        ("codex", "codex"),
        ("condres", "condres"),
        ("crystal", "crystal"),
        ("crux", "crux"),
        ("cucumber", "cucumber"),
        ("cuerdos", "cuerdos"),
        ("cyberos", "cyberos"),
        ("cycledream", "cycledream"),
        ("dahlia", "dahlia"),
        ("darkos", "darkos"),
        ("desaos", "desaos"),
        ("dracos", "dracos"),
        ("drauger", "drauger"),
        ("elbrus", "elbrus"),
        ("elive", "elive"),
        ("emmabuntus", "emmabuntus"),
        ("emperoros", "emperoros"),
        ("encryptos", "encryptos"),
        ("endless", "endless"),
        ("enos", "enos"),
        ("enso", "enso"),
        ("eshanizedos", "eshanizedos"),
        ("evolutionos", "evolutionos"),
        ("eweos", "eweos"),
        ("exodia_predator", "exodia_predator"),
        ("fastfetch", "fastfetch"),
        ("femboyos", "femboyos"),
        ("feren", "feren"),
        ("filotimo", "filotimo"),
        ("finnix", "finnix"),
        ("flatcar", "flatcar"),
        ("floflis", "floflis"),
        ("freemint", "freemint"),
        ("frugalware", "frugalware"),
        ("furreto", "furreto"),
        ("galliumos", "galliumos"),
        ("ghostfreak", "ghostfreak"),
        ("glaucus", "glaucus"),
        ("gnewsense", "gnewsense"),
        ("gnu", "gnu"),
        ("gobolinux", "gobolinux"),
        ("goldendoglinux", "goldendoglinux"),
        ("grapheneos", "grapheneos"),
        ("grombyang", "grombyang"),
        ("guix", "guix"),
        ("hamonikr", "hamonikr"),
        ("hardclanz", "hardclanz"),
        ("hce", "hce"),
        ("heliumos", "heliumos"),
        ("huayra", "huayra"),
        ("hybrid", "hybrid"),
        ("hydroos", "hydroos"),
        ("hyperbola", "hyperbola"),
        ("hypros", "hypros"),
        ("iglunix", "iglunix"),
        ("instantos", "instantos"),
        ("interix", "interix"),
        ("irix", "irix"),
        ("ironclad", "ironclad"),
        ("itc", "itc"),
        ("januslinux", "januslinux"),
        ("kalpa", "kalpa"),
        ("kaos", "kaos"),
        ("kdelinux", "kdelinux"),
        ("kernelos", "kernelos"),
        ("kibaos", "kibaos"),
        ("kibojoe", "kibojoe"),
        ("kiss", "kiss"),
        ("kogaion", "kogaion"),
        ("krassos", "krassos"),
        ("kslinux", "kslinux"),
        ("lainos", "lainos"),
        ("langitketujuh", "langitketujuh"),
        ("laxeros", "laxeros"),
        ("lede", "lede"),
        ("lfs", "lfs"),
        ("libreelec", "libreelec"),
        ("lilidog", "lilidog"),
        ("limeos", "limeos"),
        ("lingmo", "lingmo"),
        ("live_raizo", "live_raizo"),
        ("lliurex", "lliurex"),
        ("locos", "locos"),
        ("lunar", "lunar"),
        ("macaronios", "macaronios"),
        ("magix", "magix"),
        ("magpieos", "magpieos"),
        ("mainsailos", "mainsailos"),
        ("massos", "massos"),
        ("matuusos", "matuusos"),
        ("maui", "maui"),
        ("mauna", "mauna"),
        ("meowix", "meowix"),
        ("mer", "mer"),
        ("midos", "midos"),
        ("minix", "minix"),
        ("miracle_linux", "miracle_linux"),
        ("mos", "mos"),
        ("msys2", "msys2"),
        ("namib", "namib"),
        ("nebios", "nebios"),
        ("nekos", "nekos"),
        ("neptune", "neptune"),
        ("netrunner", "netrunner"),
        ("nitrux", "nitrux"),
        ("nomadbsd", "nomadbsd"),
        ("nuros", "nuros"),
        ("nurunner", "nurunner"),
        ("nutyx", "nutyx"),
        ("obarun", "obarun"),
        ("obrevenge", "obrevenge"),
        ("obsidianos", "obsidianos"),
        ("omnios", "omnios"),
        ("openindiana", "openindiana"),
        ("openstage", "openstage"),
        ("openwrt", "openwrt"),
        ("opnsense", "opnsense"),
        ("orchid", "orchid"),
        ("oreon", "oreon"),
        ("origami", "origami"),
        ("os2warp", "os2warp"),
        ("osmc", "osmc"),
        ("panwah", "panwah"),
        ("parabola", "parabola"),
        ("parch", "parch"),
        ("pcbsd", "pcbsd"),
        ("pearos", "pearos"),
        ("pengwin", "pengwin"),
        ("pentoo", "pentoo"),
        ("peropesis", "peropesis"),
        ("phyos", "phyos"),
        ("pikaos", "pikaos"),
        ("pisi", "pisi"),
        ("pnm_linux", "pnm_linux"),
        ("porteus", "porteus"),
        ("prismlinux", "prismlinux"),
        ("puffos", "puffos"),
        ("puppy", "puppy"),
        ("pureos", "pureos"),
        ("q4os", "q4os"),
        ("qts", "qts"),
        ("quasar", "quasar"),
        ("qubes", "qubes"),
        ("qubyt", "qubyt"),
        ("quibian", "quibian"),
        ("quirinux", "quirinux"),
        ("radix", "radix"),
        ("ravynos", "ravynos"),
        ("rebornos", "rebornos"),
        ("redcore", "redcore"),
        ("redos", "redos"),
        ("redrose", "redrose"),
        ("redstar", "redstar"),
        ("refracta", "refracta"),
        ("regata", "regata"),
        ("rengeos", "rengeos"),
        ("rhaymos", "rhaymos"),
        ("rhino", "rhino"),
        ("sabayon", "sabayon"),
        ("sabotage", "sabotage"),
        ("sailfish", "sailfish"),
        ("salentos", "salentos"),
        ("salientos", "salientos"),
        ("sambabox", "sambabox"),
        ("sasanqua", "sasanqua"),
        ("secureblue", "secureblue"),
        ("semc", "semc"),
        ("septor", "septor"),
        ("serene", "serene"),
        ("serpent_os", "serpent_os"),
        ("sharklinux", "sharklinux"),
        ("shastraos", "shastraos"),
        ("shebang", "shebang"),
        ("skiffos", "skiffos"),
        ("sleeperos", "sleeperos"),
        ("slitaz", "slitaz"),
        ("smartos", "smartos"),
        ("snigdhaos", "snigdhaos"),
        ("soda", "soda"),
        ("solaris", "solaris"),
        ("spoinkos", "spoinkos"),
        ("star", "star"),
        ("stock_linux", "stock_linux"),
        ("sulin", "sulin"),
        ("summitos", "summitos"),
        ("t2", "t2"),
        ("tatra", "tatra"),
        ("tearch", "tearch"),
        ("templeos", "templeos"),
        ("tileos", "tileos"),
        ("trisquel", "trisquel"),
        ("turkish", "turkish"),
        ("twister", "twister"),
        ("univalent", "univalent"),
        ("univention", "univention"),
        ("urukos", "urukos"),
        ("uwuntu", "uwuntu"),
        ("uzbek", "uzbek"),
        ("valhalla", "valhalla"),
        ("vanilla", "vanilla"),
        ("venom", "venom"),
        ("vincentos", "vincentos"),
        ("vnux", "vnux"),
        ("wii_linux", "wii_linux"),
        ("wolfos", "wolfos"),
        ("xcp_ng", "xcp_ng"),
        ("xenia", "xenia"),
        ("xferience", "xferience"),
        ("ximper", "ximper"),
        ("xinux", "xinux"),
        ("xj380", "xj380"),
        ("xray_os", "xray_os"),
        ("yiffos", "yiffos"),
        ("zerene", "zerene"),
        ("zos", "zos"),
        ("zraxyl", "zraxyl"),
        ("lynx", "lynx"),
        ("lynis", "lynx"),

        // ── Generic "linux" fallback ──
        ("linux", "linux"),
    ];

    for (distro_id, logo_name) in aliases {
        if id == *distro_id {
            return logo_name.to_string();
        }
    }

    for part in id_like.split_whitespace() {
        for (distro_id, logo_name) in aliases {
            if part == *distro_id {
                return logo_name.to_string();
            }
        }
    }

    if id.starts_with("opensuse") {
        return "opensuse".into();
    }
    if id.starts_with("ubuntu") {
        return "ubuntu".into();
    }
    if id.starts_with("fedora") {
        return "fedora".into();
    }
    if id.starts_with("centos") {
        return "centos".into();
    }
    if id.starts_with("windows") {
        return "windows".into();
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
        "haiku" => "haiku".into(),
        "sunos" => "solaris".into(),
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
