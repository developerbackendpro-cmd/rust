// #================================================================================================================================================#
// #                                                            CARGO PLUGINS + SEMVER                                                              #
// #                            CARGO PLUGINS — KENGAYTMALAR. SEMVER — VERSIYALASH. WORKSPACE. PUBLISH. CHANGELOG.                                  #
// #                            CARGO PLUGINS — РАСШИРЕНИЯ. SEMVER — ВЕРСИОНИРОВАНИЕ. WORKSPACE. PUBLISH. CHANGELOG.                                #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::cmp::Ordering;

// Cargo Plugins nima:
// Что такое Cargo Plugins:
//
//   cargo-XXX nomli binary → cargo XXX buyrug'i
//   binary с именем cargo-XXX → команда cargo XXX
//
//   Mashhur pluginlar:
//   Популярные плагины:
//   cargo watch     — o'zgarish kuzatish, qayta kompilyatsiya
//   cargo expand    — macro kengaytilishini ko'rish
//   cargo audit     — xavfsizlik zaifliklari tekshiruv
//   cargo deny      — litsenziya va dep tekshiruv
//   cargo edit      — Cargo.toml boshqaruv
//   cargo bloat     — binary hajm tahlili
//   cargo flamegraph— flamegraph yaratish
//   cargo nextest   — parallel test runner
//   cargo udeps     — ishlatilmagan dep topish
//   cargo release   — versiya chiqarish avtomatlashtirish
//
// Semver nima:
// Что такое Semver:
//
//   Semantic Versioning — MAJOR.MINOR.PATCH
//   Semantic Versioning — MAJOR.MINOR.PATCH
//   MAJOR — API o'zgarish (backward-incompatible)
//   MINOR — Yangi funksiya (backward-compatible)
//   PATCH — Xato tuzatish

#[derive(Debug, Clone, Eq, PartialEq)]
struct Versiya {
    major: u64,
    minor: u64,
    patch: u64,
    pre_release: Option<String>,  // alpha.1, beta.2, rc.1
    build: Option<String>,        // build metadata
}

#[derive(Debug, Clone, PartialEq)]
enum VersiyaXato {
    NotoGriFormat(String),
    BoshMinus,
    SonEmas(String),
}

impl fmt::Display for VersiyaXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersiyaXato::NotoGriFormat(s) => write!(f, "Noto'g'ri format: {}", s),
            VersiyaXato::BoshMinus        => write!(f, "Versiya minus bilan boshlanmaydi"),
            VersiyaXato::SonEmas(s)       => write!(f, "Son emas: {}", s),
        }
    }
}

impl Versiya {
    fn new(major: u64, minor: u64, patch: u64) -> Self {
        Versiya { major, minor, patch, pre_release: None, build: None }
    }

    fn parse(s: &str) -> Result<Self, VersiyaXato> {
        let s = s.trim_start_matches('v'); // "v1.2.3" → "1.2.3"

        // Build metadata ajratish (+)
        let (asosiy, build) = if let Some(i) = s.find('+') {
            (&s[..i], Some(s[i+1..].to_string()))
        } else {
            (s, None)
        };

        // Pre-release ajratish (-)
        let (version_str, pre_release) = if let Some(i) = asosiy.find('-') {
            (&asosiy[..i], Some(asosiy[i+1..].to_string()))
        } else {
            (asosiy, None)
        };

        // Major.Minor.Patch
        let qismlar: Vec<&str> = version_str.split('.').collect();
        if qismlar.len() != 3 {
            return Err(VersiyaXato::NotoGriFormat(
                format!("3 qism kerak, {} ta berildi", qismlar.len())
            ));
        }

        let parse_u64 = |s: &str| -> Result<u64, VersiyaXato> {
            s.parse::<u64>().map_err(|_| VersiyaXato::SonEmas(s.to_string()))
        };

        Ok(Versiya {
            major: parse_u64(qismlar[0])?,
            minor: parse_u64(qismlar[1])?,
            patch: parse_u64(qismlar[2])?,
            pre_release,
            build,
        })
    }

    fn major_yangilash(&self) -> Self {
        Versiya::new(self.major + 1, 0, 0)
    }

    fn minor_yangilash(&self) -> Self {
        Versiya::new(self.major, self.minor + 1, 0)
    }

    fn patch_yangilash(&self) -> Self {
        Versiya::new(self.major, self.minor, self.patch + 1)
    }

    fn pre_release(mut self, pre: &str) -> Self {
        self.pre_release = Some(pre.to_string());
        self
    }

    fn build_meta(mut self, meta: &str) -> Self {
        self.build = Some(meta.to_string());
        self
    }

    fn barqarormi(&self) -> bool {
        self.pre_release.is_none()
    }

    fn ishlab_chiqishmi(&self) -> bool {
        self.major == 0
    }
}

impl fmt::Display for Versiya {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(pre) = &self.pre_release {
            write!(f, "-{}", pre)?;
        }
        if let Some(build) = &self.build {
            write!(f, "+{}", build)?;
        }
        Ok(())
    }
}

impl PartialOrd for Versiya {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}

impl Ord for Versiya {
    fn cmp(&self, b: &Self) -> Ordering {
        // Major → Minor → Patch
        self.major.cmp(&b.major)
            .then(self.minor.cmp(&b.minor))
            .then(self.patch.cmp(&b.patch))
            .then_with(|| {
                // pre-release yo'q > pre-release bor
                match (&self.pre_release, &b.pre_release) {
                    (None, None)     => Ordering::Equal,
                    (None, Some(_))  => Ordering::Greater,
                    (Some(_), None)  => Ordering::Less,
                    (Some(a), Some(b)) => a.cmp(b),
                }
            })
    }
}

fn semver_misoli() {

    println!("=== SEMVER IMPLEMENTATSIYA ===\n");

    // Parse
    let v1 = Versiya::parse("1.2.3").unwrap();
    let v2 = Versiya::parse("v2.0.0-alpha.1").unwrap();
    let v3 = Versiya::parse("1.2.3-beta.2+build.456").unwrap();
    let v4 = Versiya::parse("0.1.0").unwrap();

    println!("{}", v1);  // 1.2.3
    println!("{}", v2);  // 2.0.0-alpha.1
    println!("{}", v3);  // 1.2.3-beta.2+build.456
    println!("{}", v4);  // 0.1.0

    // Xatolar
    println!("{:?}", Versiya::parse("1.2"));       // Err
    println!("{:?}", Versiya::parse("1.x.3"));     // Err
    println!();

    // Versiya yangilash
    let v = Versiya::parse("1.5.3").unwrap();
    println!("Joriy:    {}", v);
    println!("Patch++:  {}", v.patch_yangilash());
    println!("Minor++:  {}", v.minor_yangilash());
    println!("Major++:  {}", v.major_yangilash());
    println!("Pre:      {}", v.clone().pre_release("rc.1"));
    println!();

    // Taqqoslash
    let versiyalar = vec![
        Versiya::parse("1.0.0-alpha").unwrap(),
        Versiya::parse("1.0.0-alpha.1").unwrap(),
        Versiya::parse("1.0.0-beta").unwrap(),
        Versiya::parse("1.0.0-rc.1").unwrap(),
        Versiya::parse("1.0.0").unwrap(),
        Versiya::parse("1.1.0").unwrap(),
        Versiya::parse("2.0.0").unwrap(),
    ];

    let mut saralangan = versiyalar.clone();
    saralangan.sort();
    println!("Saralangan:");
    for v in &saralangan { println!("  {}", v); }
    // 1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-beta < 1.0.0-rc.1 < 1.0.0 < 1.1.0 < 2.0.0

    println!();
    println!("Barqaror: {}", Versiya::parse("1.0.0").unwrap().barqarormi()); // true
    println!("Ishlab chiqish: {}", Versiya::parse("0.5.0").unwrap().ishlab_chiqishmi()); // true
}

#[derive(Debug, Clone)]
enum VersionReq {
    Teng(Versiya),             // =1.2.3
    KattaYokiTeng(Versiya),    // >=1.2.3
    KichikYokiTeng(Versiya),   // <=1.2.3
    Katta(Versiya),            // >1.2.3
    Kichik(Versiya),           // <1.2.3
    Mos(Versiya),              // ^1.2.3 (caret — compatible)
    Taqribiy(Versiya),         // ~1.2.3 (tilde — patch updates only)
    WildcardMinor(u64),        // 1.*
    WildcardPatch(u64, u64),   // 1.2.*
}

impl VersionReq {
    fn parse(s: &str) -> Result<Self, String> {
        let s = s.trim();
        if s.starts_with(">=") {
            Ok(VersionReq::KattaYokiTeng(Versiya::parse(&s[2..]).map_err(|e| e.to_string())?))
        } else if s.starts_with("<=") {
            Ok(VersionReq::KichikYokiTeng(Versiya::parse(&s[2..]).map_err(|e| e.to_string())?))
        } else if s.starts_with('>') {
            Ok(VersionReq::Katta(Versiya::parse(&s[1..]).map_err(|e| e.to_string())?))
        } else if s.starts_with('<') {
            Ok(VersionReq::Kichik(Versiya::parse(&s[1..]).map_err(|e| e.to_string())?))
        } else if s.starts_with('=') {
            Ok(VersionReq::Teng(Versiya::parse(&s[1..]).map_err(|e| e.to_string())?))
        } else if s.starts_with('^') {
            Ok(VersionReq::Mos(Versiya::parse(&s[1..]).map_err(|e| e.to_string())?))
        } else if s.starts_with('~') {
            Ok(VersionReq::Taqribiy(Versiya::parse(&s[1..]).map_err(|e| e.to_string())?))
        } else if s.ends_with(".*") {
            let qism: Vec<&str> = s.trim_end_matches(".*").split('.').collect();
            if qism.len() == 1 {
                let major: u64 = qism[0].parse().map_err(|_| "son emas".to_string())?;
                Ok(VersionReq::WildcardMinor(major))
            } else if qism.len() == 2 {
                let major: u64 = qism[0].parse().map_err(|_| "son emas".to_string())?;
                let minor: u64 = qism[1].parse().map_err(|_| "son emas".to_string())?;
                Ok(VersionReq::WildcardPatch(major, minor))
            } else {
                Err("Noto'g'ri wildcard".to_string())
            }
        } else {
            // Oddiy versiya → ^versiya (Cargo default)
            Ok(VersionReq::Mos(Versiya::parse(s).map_err(|e| e.to_string())?))
        }
    }

    fn mos_keladi(&self, v: &Versiya) -> bool {
        match self {
            VersionReq::Teng(req)           => v == req,
            VersionReq::KattaYokiTeng(req)  => v >= req,
            VersionReq::KichikYokiTeng(req) => v <= req,
            VersionReq::Katta(req)          => v > req,
            VersionReq::Kichik(req)         => v < req,
            VersionReq::Mos(req) => {
                // ^1.2.3 → >=1.2.3, <2.0.0
                // ^0.2.3 → >=0.2.3, <0.3.0
                // ^0.0.3 → >=0.0.3, <0.0.4
                if req.major > 0 {
                    v.major == req.major && v >= req
                } else if req.minor > 0 {
                    v.major == 0 && v.minor == req.minor && v >= req
                } else {
                    v.major == 0 && v.minor == 0 && v.patch == req.patch
                }
            }
            VersionReq::Taqribiy(req) => {
                // ~1.2.3 → >=1.2.3, <1.3.0
                v.major == req.major && v.minor == req.minor && v >= req
            }
            VersionReq::WildcardMinor(major) => v.major == *major,
            VersionReq::WildcardPatch(major, minor) => {
                v.major == *major && v.minor == *minor
            }
        }
    }
}

impl fmt::Display for VersionReq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VersionReq::Teng(v)            => write!(f, "={}", v),
            VersionReq::KattaYokiTeng(v)   => write!(f, ">={}", v),
            VersionReq::KichikYokiTeng(v)  => write!(f, "<={}", v),
            VersionReq::Katta(v)           => write!(f, ">{}", v),
            VersionReq::Kichik(v)          => write!(f, "<{}", v),
            VersionReq::Mos(v)             => write!(f, "^{}", v),
            VersionReq::Taqribiy(v)        => write!(f, "~{}", v),
            VersionReq::WildcardMinor(m)   => write!(f, "{}.*", m),
            VersionReq::WildcardPatch(m,n) => write!(f, "{}.{}.*", m, n),
        }
    }
}

fn version_req_misoli() {

    println!("\n=== VERSIYA TALABI ===\n");

    let sinovlar = vec![
        ("^1.2.3", "1.2.3", true),
        ("^1.2.3", "1.9.0", true),
        ("^1.2.3", "2.0.0", false),
        ("^1.2.3", "1.2.2", false),
        ("~1.2.3", "1.2.5", true),
        ("~1.2.3", "1.3.0", false),
        (">=1.0.0", "1.5.0", true),
        (">=1.0.0", "0.9.9", false),
        (">1.0.0",  "1.0.0", false),
        (">1.0.0",  "1.0.1", true),
        ("1.*",     "1.5.2", true),
        ("1.*",     "2.0.0", false),
        ("1.2.*",   "1.2.9", true),
        ("1.2.*",   "1.3.0", false),
        ("=1.2.3",  "1.2.3", true),
        ("=1.2.3",  "1.2.4", false),
        ("^0.2.3",  "0.2.5", true),
        ("^0.2.3",  "0.3.0", false),
    ];

    for (talabi, versiya_str, kutilgan) in &sinovlar {
        let req = VersionReq::parse(talabi).unwrap();
        let v   = Versiya::parse(versiya_str).unwrap();
        let natija = req.mos_keladi(&v);
        let belgi = if natija == *kutilgan { "✅" } else { "❌" };
        println!("  {} {}  →  {}  [{}]", talabi, versiya_str, natija, belgi);
    }
}

fn cargo_plugins_tushuntirish() {

    println!("\n=== CARGO PLUGINS ===\n");

    let pluginlar = vec![
        ("cargo-watch",    "cargo watch -x run",
         "O'zgarishda avtomatik qayta ishga tushirish"),
        ("cargo-expand",   "cargo expand",
         "Macro kengaytilishini ko'rish"),
        ("cargo-audit",    "cargo audit",
         "CVE xavfsizlik zaifliklarini tekshirish"),
        ("cargo-deny",     "cargo deny check",
         "Litsenziya, dep, xavfsizlik tekshiruv"),
        ("cargo-bloat",    "cargo bloat --release",
         "Binary hajm tahlili — nima katta"),
        ("cargo-flamegraph","cargo flamegraph",
         "CPU profiling flamegraph yaratish"),
        ("cargo-nextest",  "cargo nextest run",
         "Parallel tez test runner"),
        ("cargo-udeps",    "cargo +nightly udeps",
         "Ishlatilmagan dependency topish"),
        ("cargo-release",  "cargo release patch",
         "Versiya chiqarish avtomatlashtirish"),
        ("cargo-edit",     "cargo add serde --features derive",
         "Cargo.toml boshqaruv (add/rm/upgrade)"),
        ("cargo-outdated", "cargo outdated",
         "Eskirgan dependency ro'yxati"),
        ("cargo-tarpaulin","cargo tarpaulin --out html",
         "Kod qamrovi (code coverage) hisobot"),
        ("cargo-fuzz",     "cargo fuzz run target",
         "Fuzz testing — random kirish"),
        ("cargo-criterion","cargo criterion",
         "Criterion benchmark runner"),
        ("cargo-asm",      "cargo asm mylib::my_fn",
         "Funksiya assembly ko'rish"),
    ];

    println!("{:<20} {:<35} {}", "Plugin", "Buyruq", "Tavsif");
    println!("{}", "-".repeat(90));
    for (plugin, buyruq, tavsif) in &pluginlar {
        println!("{:<20} {:<35} {}", plugin, buyruq, tavsif);
    }

    println!("\nO'rnatish:");
    println!("  cargo install cargo-watch");
    println!("  cargo install cargo-expand");
    println!("  cargo install cargo-audit");
    println!("  cargo install cargo-edit");
    println!("  cargo install cargo-nextest --locked");

    println!("\nPlugin yaratish:");
    println!("  1. cargo new --bin cargo-myplugin");
    println!("  2. binary nomi: cargo-myplugin");
    println!("  3. cargo install --path .");
    println!("  4. cargo myplugin — ishlaydi!");
}

fn cargo_plugin_yaratish() {

    println!("\n=== O'Z CARGO PLUGIN YARATISH ===\n");

    println!(r#"// Cargo.toml:
// [package]
// name = "cargo-hello"
// version = "0.1.0"
// edition = "2024"
//
// [[bin]]
// name = "cargo-hello"   ← cargo-XXX nom majburiy!
// path = "src/main.rs"

// src/main.rs:
use std::env;

fn main() {{
    // cargo subcommand bo'lganda args[1] = "hello"
    let args: Vec<String> = env::args().collect();

    // cargo hello world → args = ["cargo-hello", "hello", "world"]
    let buyruq_args: Vec<&str> = if args.len() > 1 && args[1] == "hello" {{
        args[2..].iter().map(|s| s.as_str()).collect()
    }} else {{
        args[1..].iter().map(|s| s.as_str()).collect()
    }};

    if buyruq_args.is_empty() {{
        println!("Foydalanish: cargo hello <nom>");
        return;
    }}

    println!("Salom, {{}}! 🦀", buyruq_args.join(" "));
}}

// O'rnatish va ishlatish:
// cargo install --path .
// cargo hello Dilshod        → Salom, Dilshod! 🦀
// cargo hello Rust dasturchisi → Salom, Rust dasturchisi! 🦀"#);
}

fn workspace_versiyalash() {

    println!("\n=== WORKSPACE VA VERSIYALASH ===\n");

    println!(r#"// Workspace Cargo.toml:
// [workspace]
// members = ["core", "server", "client", "cli"]
// resolver = "2"
//
// [workspace.package]
// version = "1.2.0"     ← Barcha a'zolar uchun umumiy versiya
// authors = ["Dilshod"]
// edition = "2024"
// license = "MIT"
//
// [workspace.dependencies]
// serde = {{ version = "1.0", features = ["derive"] }}
// tokio = {{ version = "1.0", features = ["full"] }}
// anyhow = "1.0"
//
// a'zo Cargo.toml:
// [package]
// name = "myapp-server"
// version.workspace = true     ← workspace versiyasidan olish
// edition.workspace = true
//
// [dependencies]
// myapp-core = {{ path = "../core" }}
// serde.workspace = true        ← workspace dep dan olish
// tokio.workspace = true

// cargo-release bilan versiya chiqarish:
// # patch versiya: 1.2.0 → 1.2.1
// cargo release patch
//
// # minor versiya: 1.2.0 → 1.3.0
// cargo release minor
//
// # major versiya: 1.2.0 → 2.0.0
// cargo release major
//
// # pre-release: 1.2.0 → 1.3.0-alpha.1
// cargo release alpha

// Changelog (CHANGELOG.md):
// ## [Unreleased]
// ### Added
// - Yangi xususiyat
// ### Fixed
// - Xato tuzatildi
//
// ## [1.2.0] - 2024-01-15
// ### Added
// - async/await qo'llovi
// ### Breaking Changes
// - API o'zgardi"#);
}

fn cargo_toml_konfiguratsiya() {

    println!("\n=== CARGO.TOML KONFIGURATSIYA ===\n");

    println!(r#"// To'liq Cargo.toml misoli:

[package]
name = "myapp"
version = "1.0.0"
edition = "2024"
authors = ["Dilshod <dilshod@example.com>"]
description = "Mening Rust ilovam"
license = "MIT OR Apache-2.0"
repository = "https://github.com/user/myapp"
keywords = ["rust", "web", "async"]
categories = ["web-programming"]
readme = "README.md"
include = ["src/**/*", "Cargo.toml", "README.md"]
exclude = ["tests/**/*"]
rust-version = "1.75"      # MSRV

[lib]
name = "myapp"
crate-type = ["cdylib", "rlib"]  # WASM + Rust

[[bin]]
name = "myapp-server"
path = "src/bin/server.rs"

[[example]]
name = "basic"
path = "examples/basic.rs"

[[bench]]
name = "perf"
harness = false

[features]
default = ["std"]
std = []
async = ["tokio"]
full = ["std", "async", "serde"]

[dependencies]
serde = {{ version = "1.0", features = ["derive"], optional = true }}
tokio = {{ version = "1.0", features = ["full"], optional = true }}
anyhow = "1.0"

[dev-dependencies]
criterion = {{ version = "0.5", features = ["html_reports"] }}
proptest = "1.0"

[build-dependencies]
cc = "1.0"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true           # debug info olib tashlash
panic = "abort"

[profile.dev]
opt-level = 0
debug = true

[profile.bench]
inherits = "release"
debug = true           # profiling uchun

[[package.metadata.docs.rs]]
features = ["full"]
targets = ["x86_64-unknown-linux-gnu"]"#);
}

#[derive(Debug, Clone)]
struct Qaram {
    nomi: String,
    talab: VersionReq,
}

impl Qaram {
    fn new(nomi: &str, talab: &str) -> Self {
        Qaram {
            nomi: nomi.to_string(),
            talab: VersionReq::parse(talab).unwrap(),
        }
    }

    fn mos_keladi(&self, v: &Versiya) -> bool {
        self.talab.mos_keladi(v)
    }
}

#[derive(Debug, Clone)]
struct Paket {
    nomi: String,
    versiya: Versiya,
    qaramliklar: Vec<Qaram>,
}

impl Paket {
    fn new(nomi: &str, versiya: &str) -> Self {
        Paket {
            nomi: nomi.to_string(),
            versiya: Versiya::parse(versiya).unwrap(),
            qaramliklar: Vec::new(),
        }
    }

    fn qaram_qosh(mut self, nomi: &str, talab: &str) -> Self {
        self.qaramliklar.push(Qaram::new(nomi, talab));
        self
    }
}

struct PaketRoyxati {
    paketlar: Vec<Paket>,
}

impl PaketRoyxati {
    fn new() -> Self { PaketRoyxati { paketlar: Vec::new() } }

    fn qosh(&mut self, paket: Paket) { self.paketlar.push(paket); }

    fn topish(&self, nomi: &str) -> Vec<&Paket> {
        self.paketlar.iter().filter(|p| p.nomi == nomi).collect()
    }

    fn eng_yangi(&self, nomi: &str) -> Option<&Paket> {
        self.topish(nomi).into_iter().max_by(|a, b| a.versiya.cmp(&b.versiya))
    }

    fn mos_versiya(&self, nomi: &str, talab: &VersionReq) -> Option<&Paket> {
        self.topish(nomi).into_iter()
            .filter(|p| talab.mos_keladi(&p.versiya))
            .max_by(|a, b| a.versiya.cmp(&b.versiya))
    }

    fn qaramliklarni_hal_qil(&self, paket: &Paket) -> Vec<String> {
        let mut natija = Vec::new();
        for qaram in &paket.qaramliklar {
            match self.mos_versiya(&qaram.nomi, &qaram.talab) {
                Some(p) => natija.push(format!("  ✅ {} = {}", p.nomi, p.versiya)),
                None    => natija.push(format!("  ❌ {} {} — topilmadi!", qaram.nomi, qaram.talab)),
            }
        }
        natija
    }
}

fn version_manager_misoli() {

    println!("\n=== VERSION MANAGER SIMULYATSIYA ===\n");

    // Registry
    let mut registry = PaketRoyxati::new();

    // Paketlar ro'yxati
    for (nom, ver) in [
        ("serde", "1.0.193"), ("serde", "1.0.150"), ("serde", "2.0.0"),
        ("tokio", "1.35.0"),  ("tokio", "1.28.0"),  ("tokio", "0.3.0"),
        ("anyhow", "1.0.75"), ("anyhow", "1.0.60"),
        ("reqwest", "0.11.24"),("reqwest", "0.12.0"),
    ] {
        registry.qosh(Paket::new(nom, ver));
    }

    // Loyiha qaramliklari
    let loyiha = Paket::new("myapp", "0.1.0")
        .qaram_qosh("serde",    "^1.0")
        .qaram_qosh("tokio",    ">=1.28")
        .qaram_qosh("anyhow",   "1.*")
        .qaram_qosh("reqwest",  "^0.11");

    println!("Loyiha: {} v{}", loyiha.nomi, loyiha.versiya);
    println!("Qaramliklar hal etish:");

    for qaram in &loyiha.qaramliklar {
        match registry.mos_versiya(&qaram.nomi, &qaram.talab) {
            Some(p) => println!("  ✅ {} {} → {}", qaram.nomi, qaram.talab, p.versiya),
            None    => println!("  ❌ {} {} — mos versiya topilmadi!", qaram.nomi, qaram.talab),
        }
    }

    println!("\nEng yangi versiyalar:");
    for nom in ["serde", "tokio", "anyhow", "reqwest"] {
        if let Some(p) = registry.eng_yangi(nom) {
            println!("  {} → {}", nom, p.versiya);
        }
    }
    // ✅ serde ^1.0 → 1.0.193
    // ✅ tokio >=1.28 → ...
    // ✅ anyhow 1.* → 1.0.75
    // ✅ reqwest ^0.11 → 0.11.24
}

fn main() {

    semver_misoli();
    version_req_misoli();
    cargo_plugins_tushuntirish();
    cargo_plugin_yaratish();
    workspace_versiyalash();
    cargo_toml_konfiguratsiya();
    version_manager_misoli();

    println!("\n=== XULOSA ===");
    println!("Semver: MAJOR.MINOR.PATCH[-pre][+build]");
    println!("  MAJOR: backward-incompatible o'zgarish");
    println!("  MINOR: yangi xususiyat, mos keladi");
    println!("  PATCH: xato tuzatish");
    println!();
    println!("Cargo.toml versiya talablari:");
    println!("  ^1.2.3 — mos (>=1.2.3, <2.0.0)");
    println!("  ~1.2.3 — taqribiy (>=1.2.3, <1.3.0)");
    println!("  >=1.0  — aniq diapazon");
    println!("  1.*    — wildcard");
    println!("  =1.2.3 — aniq versiya");
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                 | Описание (RU)                                          |
// #================================================================================================================================================#
// # |                                        SEMVER                                                                                                |
// #================================================================================================================================================#
// # |   1 | MAJOR.MINOR.PATCH               | Asosiy versiya formati                      | Основной формат версии                                 |
// # |   2 | -alpha.1, -beta.2, -rc.1        | Pre-release belgilari                       | Метки предрелиза                                       |
// # |   3 | +build.456                      | Build metadata (taqqoslashda hisoblanmaydi) | Build метаданные (не учитываются в сравнении)          |
// # |   4 | ^1.2.3 (caret)                  | >=1.2.3, <2.0.0 mos versiya                 | >=1.2.3, <2.0.0 совместимая версия                     |
// # |   5 | ~1.2.3 (tilde)                  | >=1.2.3, <1.3.0 faqat patch                 | >=1.2.3, <1.3.0 только patch                           |
// # |   6 | 1.* wildcard                    | major mos                                   | Совпадение major                                       |
// #================================================================================================================================================#
// # |                                        CARGO PLUGINS                                                                                         |
// #================================================================================================================================================#
// # |   7 | cargo-watch                     | Avtomatik qayta kompilyatsiya              | Автоматическая перекомпиляция                           |
// # |   8 | cargo-audit                     | CVE tekshiruvi                             | Проверка CVE                                            |
// # |   9 | cargo-deny                      | Litsenziya va dep tekshiruvi               | Проверка лицензий и зависимостей                        |
// # |  10 | cargo-bloat                     | Binary hajm tahlili                        | Анализ размера бинарника                                |
// # |  11 | cargo-nextest                   | Parallel test runner                       | Параллельный запуск тестов                              |
// # |  12 | cargo-udeps                     | Ishlatilmagan dep topish                   | Поиск неиспользуемых зависимостей                       |
// # |  13 | cargo-XXX yaratish              | binary nom cargo-XXX → cargo XXX           | binary cargo-XXX → команда cargo XXX                    |
// #================================================================================================================================================#