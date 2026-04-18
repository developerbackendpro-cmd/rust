// #================================================================================================================================================#
// #                                                          CARGO FEATURES                                                                        #
// #                        FEATURES — IXTIYORIY XUSUSIYATLAR. SHARTLI KOMPILYATSIYA. DEPENDENCY XUSUSIYATLARI.                                     #
// #                        FEATURES — ОПЦИОНАЛЬНЫЕ ФУНКЦИИ. УСЛОВНАЯ КОМПИЛЯЦИЯ. ФУНКЦИИ ЗАВИСИМОСТЕЙ.                                             #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

// Cargo Features nima:
// Что такое Cargo Features:
//
//   - Kod qismlarini ixtiyoriy yoki majburiy qilish
//   - Сделать части кода опциональными или обязательными
//   - Dependency xususiyatlarini yoqish/o'chirish
//   - Включение/отключение функций зависимостей
//   - Kompilyatsiya hajmini kamaytirish
//   - Уменьшение размера компиляции
//   - Turli maqsadlar uchun turli konfiguratsiya
//   - Разные конфигурации для разных целей

// [features]
// # Default features — har doim yoqilgan
// # Default features — всегда включены
// default = ["std", "derive"]
//
// # Mustaqil features
// # Независимые features
// std = []
// derive = []
// async = ["dep:tokio"]
// json = ["dep:serde_json"]
// full = ["async", "json", "logging"]
// logging = ["dep:log", "dep:env_logger"]
//
// # Feature — boshqa featurega bog'liq
// # Feature — зависит от другого feature
// advanced = ["std", "derive"]
//
// [dependencies]
// serde = { version = "1", optional = true }
// tokio = { version = "1", optional = true, features = ["full"] }
// log = { version = "0.4", optional = true }
//
// # dep: prefix — feature nomi bilan chalkashmasin
// # dep: prefix — чтобы не путать с именем feature
// serde_json = { version = "1", optional = true }

// #[cfg(feature = "...")] — feature mavjud bo'lsa kompile qil
// #[cfg(feature = "...")] — компилировать если feature есть

// Simulyatsiya uchun — feature atributlari bilan
// Для симуляции — с атрибутами feature

// "std" feature simulyatsiyasi
// Симуляция feature "std"
mod std_feature {
    pub fn std_funksiya() -> String {
        String::from("std feature yoqilgan")
    }
}

// "json" feature simulyatsiyasi
// Симуляция feature "json"
mod json_feature {
    pub fn json_serialize(qiymat: &str) -> String {
        format!("{{\"data\": \"{}\"}}", qiymat)
    }

    pub fn json_parse(json: &str) -> Option<String> {
        if json.contains("data") {
            Some(json.replace("{\"data\": \"", "").replace("\"}", ""))
        } else {
            None
        }
    }
}

// "async" feature simulyatsiyasi
// Симуляция feature "async"
mod async_feature {
    pub fn async_funksiya(nomi: &str) -> String {
        format!("async {} bajardi", nomi)
    }
}

// "logging" feature simulyatsiyasi
// Симуляция feature "logging"
mod logging_feature {
    pub fn log(daraja: &str, xabar: &str) {
        println!("[{}] {}", daraja, xabar);
    }
}

fn cfg_feature_misollari() {

    // Haqiqiy loyihada:
    // В реальном проекте:
    //
    // #[cfg(feature = "std")]
    // fn std_bilan() { ... }
    //
    // #[cfg(not(feature = "std"))]
    // fn stdsiz() { ... }
    //
    // #[cfg(any(feature = "json", feature = "yaml"))]
    // fn serializatsiya() { ... }
    //
    // #[cfg(all(feature = "async", feature = "json"))]
    // fn async_json() { ... }

    // Simulyatsiya:
    // Симуляция:
    println!("{}", std_feature::std_funksiya());
    println!("{}", json_feature::json_serialize("salom dunyo"));
    println!("{:?}", json_feature::json_parse("{\"data\": \"rust\"}"));
    println!("{}", async_feature::async_funksiya("vazifa"));
    logging_feature::log("INFO", "Dastur boshlandi");
    // std feature yoqilgan
    // {"data": "salom dunyo"}
    // Some("rust")
    // async vazifa bajardi
    // [INFO] Dastur boshlandi
}

// Feature kombinatsiyasi — cfg(all, any, not)
// Комбинация features — cfg(all, any, not)
//
// #[cfg(all(feature = "async", feature = "json"))]
// fn async_json_handler() { ... }
//
// #[cfg(any(feature = "json", feature = "yaml"))]
// fn serialize() { ... }
//
// #[cfg(not(feature = "no_std"))]
// fn std_only() { ... }

// Feature — struct/enum fieldlari
// Feature — поля struct/enum
//
// pub struct Config {
//     pub host: String,
//     pub port: u16,
//
//     #[cfg(feature = "tls")]
//     pub tls_cert: Option<String>,
//
//     #[cfg(feature = "logging")]
//     pub log_level: LogLevel,
// }

// Haqiqiy kutubxona misoli:
// Пример реальной библиотеки:
//
// // lib.rs
// pub mod core;
//
// #[cfg(feature = "json")]
// pub mod json;
//
// #[cfg(feature = "async")]
// pub mod async_client;
//
// #[cfg(feature = "derive")]
// pub use my_derive::MyDerive;
//
// pub mod prelude {
//     pub use crate::core::*;
//
//     #[cfg(feature = "json")]
//     pub use crate::json::*;
// }

// Feature ni foydalanuvchi uchun qulay qilish
// Удобное использование feature для пользователя
struct Konfiguratsiya {
    host: String,
    port: u16,
    debug: bool,
    // Real loyihada:
    // В реальном проекте:
    // #[cfg(feature = "tls")]
    // tls_sozlamalari: Option<TlsSozlamalari>,
    // #[cfg(feature = "compression")]
    // siqish: bool,
}

impl Konfiguratsiya {
    fn new(host: &str, port: u16) -> Self {
        Konfiguratsiya {
            host: host.to_string(),
            port,
            debug: false,
        }
    }

    fn debug_yoq(mut self) -> Self {
        self.debug = true;
        self
    }
}

// cargo build --features "json,async"
//   — json va async feature yoqilgan
//   — включены features json и async
//
// cargo build --no-default-features
//   — default features o'chirilgan
//   — отключены default features
//
// cargo build --no-default-features --features "std"
//   — faqat std feature
//   — только feature std
//
// cargo build --all-features
//   — barcha featurelar yoqilgan (test uchun yaxshi)
//   — все features включены (хорошо для тестов)
//
// Dependency featurelarini yoqish:
// Включение features зависимостей:
// serde = { version = "1", features = ["derive", "rc"] }
// tokio = { version = "1", features = ["full"] }
// sqlx = { version = "0.7", features = ["postgres", "runtime-tokio", "tls-rustls"] }

// Kutubxona — turli feature kombinatsiyalari
// Библиотека — разные комбинации features
struct HttpClient {
    base_url: String,
    timeout_ms: u64,
    // Feature asosida qo'shimcha maydonlar
    // Дополнительные поля в зависимости от feature
}

impl HttpClient {
    fn new(base_url: &str) -> Self {
        HttpClient {
            base_url: base_url.to_string(),
            timeout_ms: 30000,
        }
    }

    fn get(&self, path: &str) -> String {
        format!("GET {}{}", self.base_url, path)
    }

    fn post(&self, path: &str, body: &str) -> String {
        format!("POST {}{} body={}", self.base_url, path, body)
    }

    // "json" feature bilan bu metod bo'lar edi:
    // С feature "json" был бы этот метод:
    // #[cfg(feature = "json")]
    // fn get_json<T: serde::DeserializeOwned>(&self, path: &str) -> Result<T, Error> { ... }

    // "async" feature bilan:
    // С feature "async":
    // #[cfg(feature = "async")]
    // async fn get_async(&self, path: &str) -> Result<String, Error> { ... }
}

fn real_hayot_misollari() {

    // Konfiguratsiya
    let cfg = Konfiguratsiya::new("localhost", 8080).debug_yoq();
    println!("Config: {}:{} debug={}", cfg.host, cfg.port, cfg.debug);
    // Config: localhost:8080 debug=true

    // HTTP client
    let client = HttpClient::new("https://api.example.com");
    println!("{}", client.get("/users"));
    println!("{}", client.post("/users", "{\"ism\":\"Dilshod\"}"));
    // GET https://api.example.com/users
    // POST https://api.example.com/users body={"ism":"Dilshod"}

    // Feature kombinatsiyalari — cfg misoli
    // Комбинации features — пример cfg
    let platform: &str = if cfg!(debug_assertions) {
        "development"
    } else {
        "production"
    };
    println!("Muhit: {}", platform);
    // Muhit: development (cargo run da)
    // Muhit: production (cargo run --release da)

    // cfg! macro — runtime da tekshirish
    // cfg! macro — проверка в runtime
    let os_nomi: &str = if cfg!(target_os = "linux") {
        "Linux"
    } else if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else {
        "Boshqa"
    };
    println!("OS: {}", os_nomi);
    // OS: Linux

    // Features qachon ishlatiladi:
    // Когда использовать features:
    println!("\nFeature ishlatish holatlari:");
    println!("1. no_std — embedded uchun std olmadan");
    println!("2. async   — tokio/async-std bilan");
    println!("3. derive  — proc macro derive uchun");
    println!("4. tls     — HTTPS ulanish uchun");
    println!("5. json    — JSON serialization uchun");
    println!("6. full    — barcha xususiyatlar");
}

fn main() {

    println!("=== CFG FEATURE ===");
    cfg_feature_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |   1 | [features] default = [...]      | Har doim yoqilgan featurelar               | Всегда включённые features                              |
// # |   2 | feature_nomi = []               | Bo'sh feature (flag)                       | Пустой feature (флаг)                                   |
// # |   3 | feature = ["dep:crate"]         | Dependency yoquvchi feature                | Feature включающий зависимость                          |
// # |   4 | optional = true                 | Ixtiyoriy dependency                       | Опциональная зависимость                                |
// # |   5 | dep:crate_nomi                  | Dependency reference (chalkashmaslik)      | Ссылка на зависимость (без путаницы)                    |
// # |   6 | #[cfg(feature = "...")]         | Feature mavjudligini tekshirish            | Проверка наличия feature                                |
// # |   7 | #[cfg(not(feature = "..."))]    | Feature yo'qligini tekshirish              | Проверка отсутствия feature                             |
// # |   8 | #[cfg(all(feat_a, feat_b))]     | Ikkala feature mavjud                      | Оба feature присутствуют                                |
// # |   9 | #[cfg(any(feat_a, feat_b))]     | Kamida bitta feature                       | Хотя бы один feature                                    |
// # |  10 | cfg!(feature = "...")           | Runtime tekshirish                         | Проверка в runtime                                      |
// # |  11 | --features "a,b"                | Featurelarni yoqish                        | Включение features                                      |
// # |  12 | --no-default-features           | Default featureslarni o'chirish            | Отключение default features                             |
// # |  13 | --all-features                  | Barcha featurelarni yoqish                 | Включение всех features                                 |
// # |  14 | version.workspace = true        | Workspace versiyasini meros olish          | Наследование версии workspace                           |
// #================================================================================================================================================#