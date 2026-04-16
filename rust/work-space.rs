// #================================================================================================================================================#
// #                                                            WORKSPACES                                                                          #
// #                     WORKSPACE — BIR NECHTA CRATE BIRGA. UMUMIY CARGO.LOCK VA TARGET. KATTA LOYIHALAR UCHUN.                                    #
// #                     WORKSPACE — НЕСКОЛЬКО КРЕЙТОВ ВМЕСТЕ. ОБЩИЙ CARGO.LOCK И TARGET. ДЛЯ БОЛЬШИХ ПРОЕКТОВ.                                     #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

// Workspace nima:
// Что такое Workspace:
//
//   - Bir nechta crate birgalikda boshqariladi
//   - Несколько крейтов управляются вместе
//   - Bitta Cargo.lock — barcha crate uchun bir xil versiyalar
//   - Один Cargo.lock — одинаковые версии для всех крейтов
//   - Bitta target/ — qayta ishlatiladi, qurishni tezlashtiradi
//   - Одна папка target/ — переиспользуется, ускоряет сборку
//   - Cratlar bir-birini import qila oladi
//   - Крейты могут импортировать друг друга
//
// Workspace tuzilishi:
// Структура Workspace:
//
//   my_workspace/
//   ├── Cargo.toml          ← workspace root
//   ├── Cargo.lock          ← yagona lock fayl
//   ├── target/             ← umumiy build papkasi
//   ├── crates/
//   │   ├── core/           ← asosiy kutubxona
//   │   │   ├── Cargo.toml
//   │   │   └── src/lib.rs
//   │   ├── api/            ← API kutubxona
//   │   │   ├── Cargo.toml
//   │   │   └── src/lib.rs
//   │   └── cli/            ← CLI dastur
//   │       ├── Cargo.toml
//   │       └── src/main.rs
//   └── apps/
//       └── server/         ← server dastur
//           ├── Cargo.toml
//           └── src/main.rs

// ════════════════════════════════════════════════════════════════════════════
//  WORKSPACE CARGO.TOML
// ════════════════════════════════════════════════════════════════════════════

// Workspace root Cargo.toml:
// Cargo.toml корня workspace:
//
// [workspace]
// members = [
//     "crates/core",
//     "crates/api",
//     "crates/cli",
//     "apps/server",
// ]
// resolver = "2"
//
// # Workspace uchun umumiy sozlamalar
// # Общие настройки для workspace
// [workspace.dependencies]
// serde = { version = "1", features = ["derive"] }
// tokio = { version = "1", features = ["full"] }
// anyhow = "1"
//
// [workspace.package]
// version = "0.1.0"
// edition = "2021"
// authors = ["Dilshod"]
// license = "MIT"

// Crate Cargo.toml (workspace memberiga):
// Cargo.toml крейта (члена workspace):
//
// [package]
// name = "my-core"
// version.workspace = true      # workspace dan oladi
// edition.workspace = true
// authors.workspace = true
//
// [dependencies]
// serde.workspace = true        # workspace versiyasini ishlatadi
// anyhow.workspace = true
// my-api = { path = "../api" }  # boshqa workspace memberini import

// ════════════════════════════════════════════════════════════════════════════
//  WORKSPACE AFZALLIKLARI
// ════════════════════════════════════════════════════════════════════════════

// 1. Yagona Cargo.lock — versiya ziddiyatlari yo'q
//    Единый Cargo.lock — нет конфликтов версий
// 2. Umumiy target/ — qayta kompilyatsiya kamayadi
//    Общий target/ — меньше перекомпиляций
// 3. Birgalikda versiyalash — hammasi bir versiyada
//    Совместное версионирование — все в одной версии
// 4. Crate izolyatsiyasi — har biri mustaqil
//    Изоляция крейтов — каждый независим
// 5. Katta loyiha boshqaruvi — modullar orasida bog'liqlik
//    Управление большим проектом — зависимости между модулями

// ════════════════════════════════════════════════════════════════════════════
//  WORKSPACE BUYRUQLARI
// ════════════════════════════════════════════════════════════════════════════

// Barcha cratelarni qurish:
// Собрать все крейты:
//   cargo build
//   cargo build --workspace
//
// Faqat bitta crate:
// Только один крейт:
//   cargo build -p my-core
//   cargo build --package my-api
//
// Testlar:
// Тесты:
//   cargo test --workspace          # hammasi
//   cargo test -p my-core           # faqat bitta
//
// Yangi member qo'shish:
// Добавление нового члена:
//   cargo new crates/new-crate
//   # Cargo.toml members ga qo'shish kerak
//
// Workspace members ro'yxati:
// Список членов workspace:
//   cargo metadata --format-version 1 | jq '.workspace_members'

// ════════════════════════════════════════════════════════════════════════════
//  WORKSPACE SIMULYATSIYASI — BITTA FAYL ICHIDA
// ════════════════════════════════════════════════════════════════════════════

// Haqiqiy workspace da har bir mod alohida crate bo'ladi
// В реальном workspace каждый mod был бы отдельным крейтом

// "core" crate simulyatsiyasi
// Симуляция крейта "core"
mod core_crate {
    pub mod models {
        #[derive(Debug, Clone)]
        pub struct Foydalanuvchi {
            pub id: u64,
            pub ism: String,
            pub email: String,
        }

        #[derive(Debug, Clone)]
        pub struct Mahsulot {
            pub id: u64,
            pub nomi: String,
            pub narx: f64,
        }

        #[derive(Debug)]
        pub struct Buyurtma {
            pub id: u64,
            pub foydalanuvchi_id: u64,
            pub mahsulotlar: Vec<Mahsulot>,
        }

        impl Buyurtma {
            pub fn jami(&self) -> f64 {
                self.mahsulotlar.iter().map(|m| m.narx).sum()
            }
        }
    }

    pub mod xatolar {
        use std::fmt;

        #[derive(Debug)]
        pub enum CoreXato {
            TopilmadI(String),
            Validatsiya(String),
            MalumotatBaza(String),
        }

        impl fmt::Display for CoreXato {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    CoreXato::TopilmadI(s)     => write!(f, "Topilmadi: {}", s),
                    CoreXato::Validatsiya(s)   => write!(f, "Validatsiya xato: {}", s),
                    CoreXato::MalumotatBaza(s) => write!(f, "DB xato: {}", s),
                }
            }
        }

        impl std::error::Error for CoreXato {}
    }
}

// "db" crate simulyatsiyasi — core_crate ga bog'liq
// Симуляция крейта "db" — зависит от core_crate
mod db_crate {
    use super::core_crate::models::{Foydalanuvchi, Mahsulot};
    use super::core_crate::xatolar::CoreXato;
    use std::collections::HashMap;

    pub struct Database {
        foydalanuvchilar: HashMap<u64, Foydalanuvchi>,
        mahsulotlar: HashMap<u64, Mahsulot>,
        keyingi_id: u64,
    }

    impl Database {
        pub fn new() -> Self {
            Database {
                foydalanuvchilar: HashMap::new(),
                mahsulotlar: HashMap::new(),
                keyingi_id: 1,
            }
        }

        pub fn foydalanuvchi_qo_sh(&mut self, ism: &str, email: &str) -> u64 {
            let id = self.keyingi_id;
            self.foydalanuvchilar.insert(id, Foydalanuvchi {
                id,
                ism: ism.to_string(),
                email: email.to_string(),
            });
            self.keyingi_id += 1;
            id
        }

        pub fn mahsulot_qo_sh(&mut self, nomi: &str, narx: f64) -> u64 {
            let id = self.keyingi_id;
            self.mahsulotlar.insert(id, Mahsulot {
                id,
                nomi: nomi.to_string(),
                narx,
            });
            self.keyingi_id += 1;
            id
        }

        pub fn foydalanuvchi_ol(&self, id: u64) -> Result<&Foydalanuvchi, CoreXato> {
            self.foydalanuvchilar.get(&id)
                .ok_or_else(|| CoreXato::TopilmadI(format!("Foydalanuvchi id={}", id)))
        }

        pub fn mahsulot_ol(&self, id: u64) -> Result<&Mahsulot, CoreXato> {
            self.mahsulotlar.get(&id)
                .ok_or_else(|| CoreXato::TopilmadI(format!("Mahsulot id={}", id)))
        }

        pub fn barcha_mahsulotlar(&self) -> Vec<&Mahsulot> {
            self.mahsulotlar.values().collect()
        }
    }
}

// "api" crate simulyatsiyasi — core_crate va db_crate ga bog'liq
// Симуляция крейта "api" — зависит от core_crate и db_crate
mod api_crate {
    use super::core_crate::models::{Foydalanuvchi, Mahsulot, Buyurtma};
    use super::core_crate::xatolar::CoreXato;
    use super::db_crate::Database;

    pub struct ApiServer {
        db: Database,
    }

    #[derive(Debug)]
    pub struct ApiJavob<T> {
        pub muvaffaqiyat: bool,
        pub ma_lumot: Option<T>,
        pub xato: Option<String>,
    }

    impl<T> ApiJavob<T> {
        pub fn ok(ma_lumot: T) -> Self {
            ApiJavob { muvaffaqiyat: true, ma_lumot: Some(ma_lumot), xato: None }
        }

        pub fn xato(xabar: &str) -> ApiJavob<T> {
            ApiJavob { muvaffaqiyat: false, ma_lumot: None, xato: Some(xabar.to_string()) }
        }
    }

    impl ApiServer {
        pub fn new() -> Self {
            ApiServer { db: Database::new() }
        }

        pub fn foydalanuvchi_yaratish(&mut self, ism: &str, email: &str) -> ApiJavob<u64> {
            if ism.is_empty() {
                return ApiJavob::xato("Ism bo'sh bo'lishi mumkin emas");
            }
            let id = self.db.foydalanuvchi_qo_sh(ism, email);
            ApiJavob::ok(id)
        }

        pub fn mahsulot_yaratish(&mut self, nomi: &str, narx: f64) -> ApiJavob<u64> {
            if narx <= 0.0 {
                return ApiJavob::xato("Narx 0 dan katta bo'lishi kerak");
            }
            let id = self.db.mahsulot_qo_sh(nomi, narx);
            ApiJavob::ok(id)
        }

        pub fn foydalanuvchi_ol(&self, id: u64) -> ApiJavob<&Foydalanuvchi> {
            match self.db.foydalanuvchi_ol(id) {
                Ok(f)  => ApiJavob::ok(f),
                Err(e) => ApiJavob::xato(&e.to_string()),
            }
        }

        pub fn katalog(&self) -> ApiJavob<Vec<&Mahsulot>> {
            ApiJavob::ok(self.db.barcha_mahsulotlar())
        }
    }
}

fn real_hayot_misollari() {

    use api_crate::ApiServer;

    let mut server = ApiServer::new();

    // Foydalanuvchi yaratish
    let f_javob = server.foydalanuvchi_yaratish("Dilshod", "d@mail.com");
    println!("Foydalanuvchi: muvaffaqiyat={}, id={:?}", f_javob.muvaffaqiyat, f_javob.ma_lumot);
    // Foydalanuvchi: muvaffaqiyat=true, id=Some(1)

    // Noto'g'ri foydalanuvchi
    let xato_javob = server.foydalanuvchi_yaratish("", "");
    println!("Xato: {:?}", xato_javob.xato);
    // Xato: Some("Ism bo'sh bo'lishi mumkin emas")

    // Mahsulotlar qo'shish
    server.mahsulot_yaratish("Olma", 1500.0);
    server.mahsulot_yaratish("Banan", 3000.0);
    server.mahsulot_yaratish("Anor", 2500.0);

    // Katalog
    let katalog = server.katalog();
    let mut mahsulotlar: Vec<_> = katalog.ma_lumot.unwrap();
    mahsulotlar.sort_by(|a, b| a.nomi.cmp(&b.nomi));
    println!("Katalog:");
    for m in &mahsulotlar {
        println!("  {} — {} so'm", m.nomi, m.narx);
    }
    // Katalog:
    //   Anor — 2500 so'm
    //   Banan — 3000 so'm
    //   Olma — 1500 so'm

    // Foydalanuvchi topish
    let f = server.foydalanuvchi_ol(1);
    if let Some(foydalanuvchi) = f.ma_lumot {
        println!("Topildi: {} <{}>", foydalanuvchi.ism, foydalanuvchi.email);
    }
    // Topildi: Dilshod <d@mail.com>

    // Topilmagan foydalanuvchi
    let yoq = server.foydalanuvchi_ol(999);
    println!("Xato: {:?}", yoq.xato);
    // Xato: Some("Topilmadi: Foydalanuvchi id=999")
}

fn main() {

    println!("=== WORKSPACE SIMULYATSIYASI ===");
    println!("(Haqiqiy workspace da har bir mod alohida crate bo'ladi)");
    println!("(В реальном workspace каждый mod был бы отдельным крейтом)");
    println!();
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                  | Описание (RU)                                               |
// #================================================================================================================================================#
// # |   1 | [workspace] members       | Workspace member cratelar                    | Крейты-члены workspace                                      |
// # |   2 | resolver = "2"            | Zamonaviy dependency resolver                | Современный resolver зависимостей                           |
// # |   3 | [workspace.dependencies]  | Umumiy dependency versiyalari                | Общие версии зависимостей                                   |
// # |   4 | serde.workspace = true    | Workspace versiyasini ishlatish              | Использовать версию из workspace                            |
// # |   5 | version.workspace = true  | Workspace versiyasini meros olish            | Наследовать версию из workspace                             |
// # |   6 | path = "../other-crate"   | Boshqa workspace memberini import            | Импорт другого члена workspace                              |
// # |   7 | cargo build -p name       | Faqat bitta crate                            | Только один крейт                                           |
// # |   8 | cargo test --workspace    | Barcha cratelar testi                        | Тест всех крейтов                                           |
// # |   9 | Yagona Cargo.lock         | Versiya ziddiyatlari yo'q                    | Нет конфликтов версий                                       |
// # |  10 | Umumiy target/            | Qayta kompilyatsiya kamayadi                 | Меньше перекомпиляций                                       |
// # |  11 | Crate izolyatsiyasi       | Har biri mustaqil test/build                 | Каждый независимо тестируется/собирается                    |
// # |  12 | Katta loyiha              | Mono-repo — bir repoda ko'p crate            | Mono-repo — много крейтов в одном репо                      |
// #================================================================================================================================================#