// #================================================================================================================================================#
// #                                                            MODULES + USE + PUB                                                                 #
// #                            MODULES — KODNI MANTIQIY GURUHLASH. USE — IMPORT. PUB — OMMAVIY KIRISH. RUST MODUL TIZIMI.                          #
// #                            MODULES — ЛОГИЧЕСКАЯ ГРУППИРОВКА КОДА. USE — ИМПОРТ. PUB — ПУБЛИЧНЫЙ ДОСТУП. СИСТЕМА МОДУЛЕЙ RUST.                  #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

// Rust modul tizimi:
// Система модулей Rust:
//
//   mod   — modul e'lon qilish
//   mod   — объявление модуля
//   pub   — ommaviy qilish (ko'rish huquqi)
//   pub   — сделать публичным (право видимости)
//   use   — import, scope ga olib kirish
//   use   — импорт, введение в область видимости
//   super — ota modulga murojaat
//   super — обращение к родительскому модулю
//   crate — crate ildiziga murojaat
//   crate — обращение к корню крейта
//   self  — joriy modulga murojaat
//   self  — обращение к текущему модулю
//
// Ko'rish qoidalari (visibility):
// Правила видимости:
//   pub             — hamma ko'ra oladi
//   pub(crate)      — faqat shu crate ichida
//   pub(super)      — faqat ota modul
//   pub(in path)    — faqat ko'rsatilgan yo'l
//   (hech narsa)    — faqat shu modul va farzandlari

mod geometriya {

    // Modul ichidagi element — default yopiq
    // Элемент внутри модуля — по умолчанию закрытый
    struct YopiqStruct {
        qiymat: i32,
    }

    // pub — ommaviy
    // pub — публичный
    pub struct Nuqta {
        pub x: f64,
        pub y: f64,
    }

    impl Nuqta {
        pub fn new(x: f64, y: f64) -> Self {
            Nuqta { x, y }
        }

        pub fn masofa(&self, b: &Nuqta) -> f64 {
            let dx = self.x - b.x;
            let dy = self.y - b.y;
            (dx * dx + dy * dy).sqrt()
        }

        // pub emas — faqat modul ichida
        // не pub — только внутри модуля
        fn debug_info(&self) -> String {
            format!("Nuqta({}, {})", self.x, self.y)
        }
    }

    pub struct Doira {
        pub markaz: Nuqta,
        pub radius: f64,
    }

    impl Doira {
        pub fn new(x: f64, y: f64, radius: f64) -> Self {
            Doira { markaz: Nuqta::new(x, y), radius }
        }

        pub fn yuza(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }

        pub fn perimetr(&self) -> f64 {
            2.0 * std::f64::consts::PI * self.radius
        }
    }

    // pub fn — ommaviy funksiya
    // pub fn — публичная функция
    pub fn masofa_hisoblash(a: &Nuqta, b: &Nuqta) -> f64 {
        a.masofa(b)
    }

    // Ichki modul
    // Внутренний модуль
    pub mod uch_o_lchamli {
        pub struct Nuqta3D {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }

        impl Nuqta3D {
            pub fn new(x: f64, y: f64, z: f64) -> Self {
                Nuqta3D { x, y, z }
            }

            pub fn masofa(&self, b: &Nuqta3D) -> f64 {
                let dx = self.x - b.x;
                let dy = self.y - b.y;
                let dz = self.z - b.z;
                (dx*dx + dy*dy + dz*dz).sqrt()
            }
        }
    }
}

mod matematik {

    pub mod arifmetika {

        pub fn qo_shish(a: i32, b: i32) -> i32 { a + b }
        pub fn ayirish(a: i32, b: i32) -> i32 { a - b }
        pub fn ko_paytirish(a: i32, b: i32) -> i32 { a * b }
        pub fn bo_lish(a: i32, b: i32) -> Option<i32> {
            if b == 0 { None } else { Some(a / b) }
        }
    }

    pub mod statistika {
        pub fn o_rtacha(v: &[f64]) -> f64 {
            if v.is_empty() { return 0.0; }
            v.iter().sum::<f64>() / v.len() as f64
        }

        pub fn mediana(v: &mut Vec<f64>) -> f64 {
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let n = v.len();
            if n % 2 == 0 { (v[n/2 - 1] + v[n/2]) / 2.0 }
            else           { v[n/2] }
        }

        pub fn dispersiya(v: &[f64]) -> f64 {
            let ort = super::statistika::o_rtacha(v); // super — ota modul
            v.iter().map(|&x| (x - ort).powi(2)).sum::<f64>() / v.len() as f64
        }
    }

    // pub(crate) — faqat shu crate ichida ko'rinadi
    // pub(crate) — видно только внутри этого крейта
    pub(crate) fn crate_ichida_korinadi() -> &'static str {
        "Bu faqat crate ichida"
    }
}

// use — to'liq yo'l bilan import
// use — импорт с полным путём
use geometriya::Nuqta;
use geometriya::Doira;
use geometriya::uch_o_lchamli::Nuqta3D;

// use — bir nechta bir xil moduldan
// use — несколько из одного модуля
use matematik::arifmetika::{qo_shish, ayirish, ko_paytirish};

// use ... as — alias
// use ... as — псевдоним
use matematik::statistika::o_rtacha as ortacha_hisoblash;
use matematik::statistika::dispersiya as disp;

// use * — wildcard (barcha ommaviy elementlar)
// use * — wildcard (все публичные элементы)
use matematik::statistika::*;

fn use_misollari() {

    // Nuqta — use bilan import qilingan
    // Nuqta — импортирован через use
    let n1: Nuqta = Nuqta::new(0.0, 0.0);
    let n2: Nuqta = Nuqta::new(3.0, 4.0);
    println!("Masofa: {:.1}", n1.masofa(&n2));
    // Masofa: 5.0

    // Doira
    let d: Doira = Doira::new(0.0, 0.0, 5.0);
    println!("Yuza: {:.2}", d.yuza());
    println!("Perimetr: {:.2}", d.perimetr());
    // Yuza: 78.54
    // Perimetr: 31.42

    // 3D nuqta
    let p1: Nuqta3D = Nuqta3D::new(0.0, 0.0, 0.0);
    let p2: Nuqta3D = Nuqta3D::new(1.0, 2.0, 2.0);
    println!("3D masofa: {:.1}", p1.masofa(&p2));
    // 3D masofa: 3.0

    // use bilan import qilingan funksiyalar
    // Импортированные через use функции
    println!("{}", qo_shish(10, 5));
    println!("{}", ayirish(10, 5));
    println!("{}", ko_paytirish(10, 5));
    // 15
    // 5
    // 50

    // alias bilan
    // с псевдонимом
    let v: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    println!("O'rtacha: {}", ortacha_hisoblash(&v));
    println!("Dispersiya: {:.2}", disp(&v));
    // O'rtacha: 3
    // Dispersiya: 2.00

    // wildcard import bilan (mediana)
    let mut v2: Vec<f64> = vec![5.0, 2.0, 8.0, 1.0, 9.0];
    println!("Mediana: {}", mediana(&mut v2));
    // Mediana: 5

    // pub(crate) — shu crate ichida ko'rinadi
    // pub(crate) — видно в этом крейте
    println!("{}", matematik::crate_ichida_korinadi());
    // Bu faqat crate ichida

    // To'liq yo'l bilan — use olmay
    // С полным путём — без use
    let n3: geometriya::Nuqta = geometriya::Nuqta::new(1.0, 1.0);
    println!("({}, {})", n3.x, n3.y);
    // (1, 1)
}

mod tashqi {
    pub mod ichki {

        // pub — hamma ko'ra oladi
        // pub — все могут видеть
        pub fn hamma_korishim() -> &'static str { "pub — hamma" }

        // pub(crate) — faqat shu crate
        // pub(crate) — только этот крейт
        pub(crate) fn crate_korishim() -> &'static str { "pub(crate)" }

        // pub(super) — faqat ota modul (tashqi)
        // pub(super) — только родитель (tashqi)
        pub(super) fn ota_korishim() -> &'static str { "pub(super)" }

        // (hech narsa) — faqat ichki modul
        // (ничего) — только модуль ichki
        fn faqat_ichki() -> &'static str { "private" }

        pub fn hammaga_ko_rsat() {
            // Ichki modul o'zining private ni ko'ra oladi
            // Внутренний модуль может видеть свои private
            println!("{}", faqat_ichki());
        }
    }

    // pub(super) ni ota modul ko'ra oladi
    // pub(super) может видеть родительский модуль
    pub fn ota_foydalanish() {
        println!("{}", ichki::ota_korishim());
    }
}

fn pub_misollari() {

    // pub — hamma yerdan ko'rinadi
    println!("{}", tashqi::ichki::hamma_korishim());
    // pub — hamma

    // pub(crate) — crate ichida ko'rinadi
    println!("{}", tashqi::ichki::crate_korishim());
    // pub(crate)

    // pub(super) — faqat ota modul (bu yerda tashqi)
    // tashqi::ichki::ota_korishim() — bunda ishlamaydi
    tashqi::ota_foydalanish(); // ota modul orqali chaqirish
    // pub(super)

    // private — shu moduldan tashqarida ko'rinmaydi
    // tashqi::ichki::faqat_ichki()  ← kompile bo'lmaydi

    tashqi::ichki::hammaga_ko_rsat();
    // private
}

mod kutubxona {
    mod ichki_impl {
        pub struct Api {
            pub versiya: &'static str,
        }

        impl Api {
            pub fn new() -> Self {
                Api { versiya: "1.0.0" }
            }

            pub fn so_rov(&self, endpoint: &str) -> String {
                format!("GET {}", endpoint)
            }
        }

        pub fn yaratish() -> Api {
            Api::new()
        }
    }

    // pub use — re-export
    // pub use — реэкспорт
    pub use ichki_impl::Api;
    pub use ichki_impl::yaratish;
}

fn reexport_misollari() {

    // kutubxona::ichki_impl::Api — to'liq yo'l (private)
    // kutubxona::Api — re-export orqali (public)
    let api: kutubxona::Api = kutubxona::yaratish();
    println!("Versiya: {}", api.versiya);
    println!("{}", api.so_rov("/users"));
    // Versiya: 1.0.0
    // GET /users
}

mod ilovalar {
    pub mod models {
        #[derive(Debug, Clone)]
        pub struct Foydalanuvchi {
            pub id: u32,
            pub ism: String,
            pub(crate) parol_hash: String,
        }

        impl Foydalanuvchi {
            pub fn new(id: u32, ism: &str, parol: &str) -> Self {
                Foydalanuvchi {
                    id,
                    ism: ism.to_string(),
                    parol_hash: format!("hash:{}", parol), // simulyatsiya
                }
            }

            pub fn parolni_tekshir(&self, parol: &str) -> bool {
                self.parol_hash == format!("hash:{}", parol)
            }
        }
    }

    pub mod xizmatlar {
        use super::models::Foydalanuvchi;
        use std::collections::HashMap;

        pub struct FoydalanuvchiXizmati {
            foydalanuvchilar: HashMap<u32, Foydalanuvchi>,
            keyingi_id: u32,
        }

        impl FoydalanuvchiXizmati {
            pub fn new() -> Self {
                FoydalanuvchiXizmati {
                    foydalanuvchilar: HashMap::new(),
                    keyingi_id: 1,
                }
            }

            pub fn ro_yxatga_olish(&mut self, ism: &str, parol: &str) -> u32 {
                let id = self.keyingi_id;
                self.foydalanuvchilar.insert(id, Foydalanuvchi::new(id, ism, parol));
                self.keyingi_id += 1;
                id
            }

            pub fn kirish(&self, id: u32, parol: &str) -> Option<&Foydalanuvchi> {
                let f = self.foydalanuvchilar.get(&id)?;
                if f.parolni_tekshir(parol) { Some(f) } else { None }
            }

            pub fn topish(&self, id: u32) -> Option<&Foydalanuvchi> {
                self.foydalanuvchilar.get(&id)
            }
        }
    }

    // pub use — tashqi API
    // pub use — внешний API
    pub use models::Foydalanuvchi;
    pub use xizmatlar::FoydalanuvchiXizmati;
}

fn real_hayot_misollari() {

    use ilovalar::{Foydalanuvchi, FoydalanuvchiXizmati};

    let mut xizmat = FoydalanuvchiXizmati::new();

    let id1 = xizmat.ro_yxatga_olish("Dilshod", "sir_parol");
    let id2 = xizmat.ro_yxatga_olish("Ali", "boshqa_parol");

    println!("Ro'yxatdan o'tdi: id={}", id1);

    match xizmat.kirish(id1, "sir_parol") {
        Some(f) => println!("Kirdi: {}", f.ism),
        None    => println!("Kirish muvaffaqiyatsiz"),
    }

    match xizmat.kirish(id1, "noto'g'ri_parol") {
        Some(f) => println!("Kirdi: {}", f.ism),
        None    => println!("Noto'g'ri parol"),
    }

    if let Some(f) = xizmat.topish(id2) {
        println!("Topildi: {} (id={})", f.ism, f.id);
        // parol_hash — pub(crate) — shu crate ichida ko'rinadi
        println!("Hash: {}", f.parol_hash);
    }
    // Ro'yxatdan o'tdi: id=1
    // Kirdi: Dilshod
    // Noto'g'ri parol
    // Topildi: Ali (id=2)
    // Hash: hash:boshqa_parol
}

fn main() {

    println!("=== USE MISOLLARI ===");
    use_misollari();

    println!("\n=== PUB MISOLLARI ===");
    pub_misollari();

    println!("\n=== RE-EXPORT ===");
    reexport_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                   | Описание (RU)                                              |
// #================================================================================================================================================#
// # |   1 | mod nomi { }              | Modul e'lon qilish                            | Объявление модуля                                          |
// # |   2 | pub                       | Hamma ko'ra oladi                             | Все могут видеть                                           |
// # |   3 | pub(crate)                | Faqat shu crate                               | Только этот крейт                                          |
// # |   4 | pub(super)                | Faqat ota modul                               | Только родительский модуль                                 |
// # |   5 | pub(in path)              | Faqat ko'rsatilgan yo'l                       | Только указанный путь                                      |
// # |   6 | use a::b::C               | Import                                        | Импорт                                                     |
// # |   7 | use a::b::{C, D}          | Bir nechta import                             | Несколько импортов                                         |
// # |   8 | use a::b::C as X          | Alias bilan import                            | Импорт с псевдонимом                                       |
// # |   9 | use a::b::*               | Wildcard import                               | Групповой импорт                                           |
// # |  10 | pub use a::b::C           | Re-export                                     | Реэкспорт                                                  |
// # |  11 | super::                   | Ota modul                                     | Родительский модуль                                        |
// # |  12 | crate::                   | Crate ildizi                                  | Корень крейта                                              |
// # |  13 | self::                    | Joriy modul                                   | Текущий модуль                                             |
// #================================================================================================================================================#