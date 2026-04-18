// #================================================================================================================================================#
// #                                                                CRATES + CARGO.TOML                                                             #
// #                        CRATE — RUST KOMPILYATSIYA BIRLIGI. CARGO.TOML — LOYIHA KONFIGURATSIYASI. DEPENDENCIES, FEATURES.                       #
// #                        CRATE — ЕДИНИЦА КОМПИЛЯЦИИ RUST. CARGO.TOML — КОНФИГУРАЦИЯ ПРОЕКТА. DEPENDENCIES, FEATURES.                             #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

// Crate va Cargo.toml haqida:
// О Crate и Cargo.toml:
//
//   Crate — Rust kompilyatsiya birligi (bir binary yoki library)
//   Crate — единица компиляции Rust (один binary или библиотека)
//
//   Ikkita crate turi:
//   Два вида крейта:
//     binary crate — main() bor, dastur
//     binary crate — есть main(), программа
//     library crate — lib.rs, kutubxona
//     library crate — lib.rs, библиотека
//
//   Cargo.toml tuzilishi:
//   Структура Cargo.toml:
//     [package]      — loyiha ma'lumotlari
//     [dependencies] — tashqi kutubxonalar
//     [dev-dependencies] — test uchun
//     [build-dependencies] — build.rs uchun
//     [features]     — ixtiyoriy xususiyatlar
//     [profile]      — kompilyatsiya sozlamalari
//
//   Versiya belgilash:
//   Указание версий:
//     "1.0"    = "^1.0" (1.0 dan 2.0 gacha, 2.0 kirmas)
//     "=1.0.0" — aniq versiya
//     ">=1.0"  — 1.0 va undan katta
//     "*"      — istalgan versiya


// Oddiy binary loyiha Cargo.toml:
// Cargo.toml простого бинарного проекта:
//
// [package]
// name = "my_app"
// version = "0.1.0"
// edition = "2021"
// authors = ["Dilshod <dilshod@example.com>"]
// description = "Mening ilovam"
// license = "MIT"
// repository = "https://github.com/dilshod/my_app"
//
// [dependencies]
// serde = { version = "1", features = ["derive"] }
// serde_json = "1"
// tokio = { version = "1", features = ["full"] }
// anyhow = "1"
// thiserror = "1"
// clap = { version = "4", features = ["derive"] }
// log = "0.4"
// env_logger = "0.10"
//
// [dev-dependencies]
// mockall = "0.11"
// criterion = "0.5"
//
// [build-dependencies]
// cc = "1"
//
// [[bin]]
// name = "my_app"
// path = "src/main.rs"
//
// [[bin]]
// name = "my_tool"
// path = "src/bin/tool.rs"

// Crate ildizidan murojaat — crate::
// Обращение от корня крейта — crate::
mod utils {
    pub mod string_utils {
        pub fn katta_harf(s: &str) -> String {
            s.to_uppercase()
        }

        pub fn kichik_harf(s: &str) -> String {
            s.to_lowercase()
        }

        pub fn capitalize(s: &str) -> String {
            let mut c = s.chars();
            match c.next() {
                None    => String::new(),
                Some(f) => f.to_uppercase().to_string() + c.as_str(),
            }
        }
    }

    pub mod son_utils {
        pub fn faktorial(n: u64) -> u64 {
            (1..=n).product()
        }

        pub fn fibonachi(n: u32) -> u64 {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 0..n { (a, b) = (b, a + b); }
            a
        }

        pub fn juftmi(n: i32) -> bool { n % 2 == 0 }
    }
}

mod ilova {
    // crate:: — crate ildizidan murojaat
    // crate:: — обращение от корня крейта
    use crate::utils::string_utils;
    use crate::utils::son_utils;

    pub struct Ishlov {
        pub nomi: String,
    }

    impl Ishlov {
        pub fn new(nomi: &str) -> Self {
            Ishlov { nomi: string_utils::capitalize(nomi) }
        }

        pub fn faktorial_hisoblash(&self, n: u64) -> String {
            format!("{}! = {}", n, son_utils::faktorial(n))
        }
    }
}

fn crate_murojaat_misollari() {

    // crate:: yo'l bilan murojaat
    // Обращение через путь crate::
    let katta: String = crate::utils::string_utils::katta_harf("salom dunyo");
    let kichik: String = crate::utils::string_utils::kichik_harf("RUST TILI");
    println!("{}", katta);
    println!("{}", kichik);
    // SALOM DUNYO
    // rust tili

    // use bilan import qilingan
    // Импортированное через use
    use utils::son_utils::{faktorial, fibonachi};
    println!("5! = {}", faktorial(5));
    println!("fib(10) = {}", fibonachi(10));
    // 5! = 120
    // fib(10) = 55

    // Ilova struct — crate murojaat ichida
    // Структура Ilova — обращение к crate внутри
    let ishlov = ilova::Ishlov::new("dilshod");
    println!("Ism: {}", ishlov.nomi);
    println!("{}", ishlov.faktorial_hisoblash(6));
    // Ism: Dilshod
    // 6! = 720
}

// Eski yondashuv (Rust 2015):
// Старый подход (Rust 2015):
//   extern crate serde;
//   extern crate tokio;
//
// Zamonaviy (Rust 2018+):
// Современный (Rust 2018+):
//   Cargo.toml ga yozing — avtomatik import bo'ladi
//   Просто добавьте в Cargo.toml — импортируется автоматически
//   use serde::Serialize;
//   use tokio::runtime::Runtime;

// Cargo.toml da features:
// Features в Cargo.toml:
//
// [features]
// default = ["json"]
// json = ["serde_json"]
// async = ["tokio"]
// full = ["json", "async", "logging"]
// logging = ["log", "env_logger"]
//
// Ishlatish:
// Использование:
//   cargo build --features "async,logging"
//   cargo build --no-default-features --features json
//
// Kod ichida feature tekshirish:
// Проверка feature в коде:
//   #[cfg(feature = "json")]
//   pub fn json_serialize() { ... }
//
//   #[cfg(not(feature = "async"))]
//   pub fn sodda_versiya() { ... }

// cfg attribute bilan feature tekshirish simulyatsiyasi
// Симуляция проверки feature через cfg
#[cfg(debug_assertions)]
fn debug_rejim() {
    println!("DEBUG rejimda ishlayapti");
}

#[cfg(not(debug_assertions))]
fn debug_rejim() {
    println!("RELEASE rejimda ishlayapti");
}

// Target OS tekshirish
// Проверка целевой ОС
fn os_tekshirish() {
    #[cfg(target_os = "linux")]
    println!("Linux");

    #[cfg(target_os = "windows")]
    println!("Windows");

    #[cfg(target_os = "macos")]
    println!("macOS");

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    println!("Boshqa OS");
}

// Cargo.toml profile sozlamalari:
// Настройки profile в Cargo.toml:
//
// [profile.dev]
// opt-level = 0        # Optimizatsiya yo'q
// debug = true         # Debug ma'lumotlari
// overflow-checks = true
//
// [profile.release]
// opt-level = 3        # Maksimal optimizatsiya
// debug = false
// lto = true           # Link Time Optimization
// codegen-units = 1    # Yaxshi optimizatsiya
// panic = "abort"      # Unwind o'rniga abort
//
// [profile.test]
// opt-level = 0
// debug = true
//
// Ishlatish:
// Использование:
//   cargo build           — dev profile
//   cargo build --release — release profile
//   cargo test            — test profile

// 1. Crate tuzilishi maslahatlar:
// 1. Советы по структуре крейта:
//
//   src/
//     main.rs        — binary entry point
//     lib.rs         — library root
//     bin/
//       tool1.rs     — qo'shimcha binary
//     models/
//       mod.rs       — models moduli
//       user.rs
//       product.rs
//     services/
//       mod.rs       — services moduli
//       auth.rs
//       payment.rs
//     utils/
//       mod.rs

// 2. Versiya belgilash amaliyoti:
// 2. Практика указания версий:
//
//   # Eng yaxshi:
//   serde = "1"          # ^1.0.0 — patch va minor yangilanishlar
//
//   # Aniq versiya kerak bo'lsa:
//   serde = "=1.0.195"   # Faqat shu versiya
//
//   # Yaxshi emas:
//   serde = "*"          # Istalgan versiya — xavfli

// Cargo buyruqlari:
// Команды Cargo:
//
//   cargo new my_app          — yangi binary loyiha
//   cargo new --lib my_lib    — yangi library loyiha
//   cargo build               — build (debug)
//   cargo build --release     — build (release)
//   cargo run                 — build + run
//   cargo test                — testlarni ishga tushirish
//   cargo doc                 — dokumentatsiya yaratish
//   cargo doc --open          — brauzerda ochish
//   cargo fmt                 — kodni formatlash
//   cargo clippy              — linting
//   cargo check               — tez tekshirish (build yo'q)
//   cargo add serde           — dependency qo'shish
//   cargo remove serde        — dependency o'chirish
//   cargo update              — dependencylarni yangilash
//   cargo tree                — dependency daraxti
//   cargo publish             — crates.io ga yuborish
//   cargo install ripgrep     — binary o'rnatish

fn real_hayot_misollari() {

    // 1. String utils kutubxona
    // 1. Библиотека string utils
    use utils::string_utils::*;

    let sozlar: Vec<&str> = vec!["rust", "TILI", "ajoyib"];
    let natijalar: Vec<String> = sozlar.iter().map(|&s| capitalize(s)).collect();
    println!("{:?}", natijalar);
    // ["Rust", "Tili", "Ajoyib"]

    // 2. Faktorial va fibonachi
    // 2. Факториал и Фибоначчи
    use utils::son_utils::*;

    for n in 1..=6 {
        print!("{}!={} ", n, faktorial(n));
    }
    println!();
    // 1!=1 2!=2 3!=6 4!=24 5!=120 6!=720

    let fib_seriya: Vec<u64> = (0..10).map(|n| fibonachi(n)).collect();
    println!("{:?}", fib_seriya);
    // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

    // 3. cfg — build rejimi
    // 3. cfg — режим сборки
    debug_rejim();
    // DEBUG rejimda ishlayapti (cargo run da)
    // RELEASE rejimda ishlayapti (cargo run --release da)

    // 4. OS tekshirish
    // 4. Проверка ОС
    os_tekshirish();
    // Linux (yoki boshqa)

    // 5. cfg(test) — test uchun kod
    // 5. cfg(test) — код для тестов
    // Bu kod faqat `cargo test` da kompile bo'ladi
    // Этот код компилируется только при `cargo test`
    #[cfg(test)]
    {
        println!("Bu test rejimida ishlaydi");
    }

    // 6. Conditional compilation
    // 6. Условная компиляция
    let platform: &str = if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "windows") {
        "Windows"
    } else {
        "Boshqa"
    };
    println!("Platform: {}", platform);
    // Platform: Linux
}

fn main() {
    println!("=== CRATE MUROJAAT ===");
    crate_murojaat_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                   | Описание (RU)                                              |
// #================================================================================================================================================#
// # |   1 | [package]                 | Loyiha ma'lumotlari                           | Информация о проекте                                       |
// # |   2 | [dependencies]            | Tashqi kutubxonalar                           | Внешние библиотеки                                         |
// # |   3 | [dev-dependencies]        | Test va benchmark uchun                       | Для тестов и бенчмарков                                    |
// # |   4 | [features]                | Ixtiyoriy xususiyatlar                        | Опциональные функции                                       |
// # |   5 | [profile.release]         | Release sozlamalari                           | Настройки release                                          |
// # |   6 | crate::                   | Crate ildizidan murojaat                      | Обращение от корня крейта                                  |
// # |   7 | super::                   | Ota moduldan murojaat                         | Обращение от родителя                                      |
// # |   8 | #[cfg(feature = "...")]   | Feature tekshirish                            | Проверка feature                                           |
// # |   9 | #[cfg(target_os = "...")] | OS tekshirish                                 | Проверка ОС                                                |
// # |  10 | cfg!(...)                 | Runtime cfg tekshirish                        | Проверка cfg в runtime                                     |
// # |  11 | cargo add                 | Dependency qo'shish                           | Добавление зависимости                                     |
// # |  12 | cargo tree                | Dependency daraxti                            | Дерево зависимостей                                        |
// # |  13 | "1" versiya               | ^1.0 — minor va patch yangilanishlar          | ^1.0 — обновления minor и patch                            |
// # |  14 | "=1.0.0" versiya          | Aniq versiya                                  | Точная версия                                              |
// #================================================================================================================================================#