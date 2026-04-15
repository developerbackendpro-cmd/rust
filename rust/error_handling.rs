// #================================================================================================================================================#
// #                                              ERROR HANDLING — ANYHOW  |  THISERROR                                                             #
// #                     ANYHOW — TEZKOR XATO BOSHQARUVI. THISERROR — PROFESSIONAL XATO TURLARI. IKKALASI ? BILAN ISHLAYDI.                         #
// #                     ANYHOW — БЫСТРАЯ ОБРАБОТКА ОШИБОК. THISERROR — ПРОФЕССИОНАЛЬНЫЕ ТИПЫ ОШИБОК. ОБА РАБОТАЮТ С ?.                             #
// #================================================================================================================================================#

// Cargo.toml ga qo'shish kerak:
// Нужно добавить в Cargo.toml:
//
// [dependencies]
// anyhow = "1"
// thiserror = "1"

#![allow(dead_code, unused)]

use anyhow::{anyhow, bail, ensure, Context, Result as AnyResult};
use thiserror::Error;
use std::fmt;
use std::num::ParseIntError;

// Ikki kutubxona farqi:
// Разница двух библиотек:
//
//   thiserror — Kutubxona ICHIDA xato turlari uchun
//   thiserror — Для типов ошибок ВНУТРИ библиотеки
//     - Aniq xato turlari kerak bo'lsa
//     - Когда нужны конкретные типы ошибок
//     - API foydalanuvchi xatoni catch qilishi kerak bo'lsa
//     - Когда пользователь API должен поймать ошибку
//
//   anyhow — Ilovalar va skriptlar uchun
//   anyhow — Для приложений и скриптов
//     - Tezkor prototiplash
//     - Быстрое прототипирование
//     - Xato turini bilish shart emas
//     - Тип ошибки не важен
//     - Box<dyn Error> o'rniga ishlatiladi
//     - Используется вместо Box<dyn Error>

// ════════════════════════════════════════════════════════════════════════════
//  THISERROR — XATO TURLARI YARATISH
//  THISERROR — СОЗДАНИЕ ТИПОВ ОШИБОК
// ════════════════════════════════════════════════════════════════════════════

// #[derive(Error)] — std::error::Error avtomatik implement qiladi
// #[derive(Error)] — автоматически реализует std::error::Error

#[derive(Debug, Error)]
enum FaylXato {
    // #[error("...")] — Display avtomatik implement qiladi
    // #[error("...")] — автоматически реализует Display
    #[error("Fayl topilmadi: {yo_l}")]
    TopilmadI { yo_l: String },

    #[error("O'qish huquqi yo'q: {yo_l}")]
    RuxsatYoq { yo_l: String },

    #[error("Fayl bo'sh: {yo_l}")]
    BoshFayl { yo_l: String },

    // #[from] — From trait avtomatik implement qiladi
    // #[from] — автоматически реализует From
    #[error("IO xato: {0}")]
    IoXato(#[from] std::io::Error),
}

#[derive(Debug, Error)]
enum TarmoqXato {
    #[error("Ulanib bo'lmadi: {manzil}:{port}")]
    UlanishXato { manzil: String, port: u16 },

    #[error("Timeout: {sekund} soniyadan keyin")]
    Timeout { sekund: u64 },

    #[error("DNS xato: {domen}")]
    DnsXato { domen: String },

    #[error("HTTP {status}: {xabar}")]
    HttpXato { status: u16, xabar: String },
}

#[derive(Debug, Error)]
enum ParseXato {
    #[error("Son parse xatosi: {0}")]
    SonXato(#[from] ParseIntError),

    #[error("Bo'sh qator")]
    BoshQator,

    #[error("Noto'g'ri format: '{qiymat}'")]
    NotogriiFormat { qiymat: String },
}

// source() — sabab xato — #[source] bilan
// source() — причина ошибки — через #[source]
#[derive(Debug, Error)]
enum IlovaXato {
    #[error("Fayl xatosi: {0}")]
    Fayl(#[from] FaylXato),

    #[error("Tarmoq xatosi: {0}")]
    Tarmoq(#[from] TarmoqXato),

    #[error("Parse xatosi: {0}")]
    Parse(#[from] ParseXato),

    #[error("Konfiguratsiya xatosi: {xabar}")]
    Konfiguratsiya { xabar: String },
}

fn thiserror_misollari() {

    // FaylXato — yaratish
    // FaylXato — создание
    let x1 = FaylXato::TopilmadI { yo_l: "/etc/config.toml".to_string() };
    let x2 = FaylXato::RuxsatYoq { yo_l: "/root/secret".to_string() };
    println!("{}", x1);
    println!("{}", x2);
    // Fayl topilmadi: /etc/config.toml
    // O'qish huquqi yo'q: /root/secret

    // TarmoqXato — yaratish
    // TarmoqXato — создание
    let t1 = TarmoqXato::UlanishXato {
        manzil: "db.example.com".to_string(),
        port: 5432,
    };
    let t2 = TarmoqXato::HttpXato { status: 404, xabar: "Not Found".to_string() };
    println!("{}", t1);
    println!("{}", t2);
    // Ulanib bo'lmadi: db.example.com:5432
    // HTTP 404: Not Found

    // ParseXato — #[from] bilan
    // ParseXato — через #[from]
    let parse_err: ParseXato = "abc".parse::<i32>().unwrap_err().into();
    println!("{}", parse_err);
    // Son parse xatosi: invalid digit found in string

    // IlovaXato — zanjir
    // IlovaXato — цепочка
    let ilova_x: IlovaXato = IlovaXato::Fayl(FaylXato::TopilmadI {
        yo_l: "config.toml".to_string(),
    });
    println!("{}", ilova_x);
    // Fayl xatosi: Fayl topilmadi: config.toml

    // ? operatori — From orqali avtomatik konversiya
    // оператор ? — автоматическая конвертация через From
    fn fayl_o_qi(yo_l: &str) -> Result<String, FaylXato> {
        if yo_l.starts_with("/root") {
            return Err(FaylXato::RuxsatYoq { yo_l: yo_l.to_string() });
        }
        Ok(format!("{} mazmuni", yo_l))
    }

    fn ma_lumot_ol(yo_l: &str) -> Result<String, IlovaXato> {
        let mazmun: String = fayl_o_qi(yo_l)?;
        // FaylXato → IlovaXato (#[from] orqali)
        // FaylXato → IlovaXato (через #[from])
        Ok(mazmun)
    }

    println!("{:?}", ma_lumot_ol("config.toml"));
    println!("{:?}", ma_lumot_ol("/root/secret"));
    // Ok("config.toml mazmuni")
    // Err(Fayl(RuxsatYoq { yo_l: "/root/secret" }))
}

// ════════════════════════════════════════════════════════════════════════════
//  ANYHOW — TEZKOR XATO BOSHQARUVI
//  ANYHOW — БЫСТРАЯ ОБРАБОТКА ОШИБОК
// ════════════════════════════════════════════════════════════════════════════

// anyhow::Result<T> = Result<T, anyhow::Error>
// anyhow::Error — istalgan xatoni o'z ichiga oladi
// anyhow::Error — содержит любую ошибку

fn anyhow_asosiy_misollari() -> AnyResult<()> {

    // ? operatori — istalgan xato bilan
    // оператор ? — с любой ошибкой
    let s: &str = "42";
    let n: i32 = s.parse::<i32>()?;
    println!("{}", n);
    // 42

    // anyhow!() — yangi xato yaratish
    // anyhow!() — создание новой ошибки
    let x: i32 = -5;
    if x < 0 {
        // Err(anyhow!("..."))
        // anyhow! — xato xabarini formatlaydi
        // anyhow! — форматирует сообщение ошибки
        return Err(anyhow!("Manfiy son: {}", x));
    }

    // bail!() — Err qaytarish uchun qisqa yozuv
    // bail!() — краткая запись для возврата Err
    let y: i32 = 200;
    if y > 100 {
        bail!("Juda katta: {}", y);
        // ≡ return Err(anyhow!("Juda katta: {}", y));
    }

    Ok(())
}

fn anyhow_ensure_misoli() -> AnyResult<()> {

    // ensure!() — shart tekshirish
    // ensure!() — проверка условия
    let port: u16 = 8080;
    ensure!(port >= 1024, "Port tizim porti: {}", port);
    // ≡ if !(port >= 1024) { bail!("Port tizim porti: {}", port) }

    let ism: &str = "Dilshod";
    ensure!(!ism.is_empty(), "Ism bo'sh bo'lishi mumkin emas");
    ensure!(ism.len() <= 50, "Ism juda uzun: {} ta belgi", ism.len());

    println!("Port: {}, Ism: {}", port, ism);
    // Port: 8080, Ism: Dilshod
    Ok(())
}

fn anyhow_context_misoli() -> AnyResult<()> {

    // .context() — xatoga kontekst qo'shish
    // .context() — добавление контекста к ошибке
    let fayl_nomi: &str = "config.toml";
    let n: i32 = "abc"
        .parse::<i32>()
        .context(format!("{} faylidan son o'qishda xato", fayl_nomi))?;

    // .with_context() — lazy kontekst
    // .with_context() — ленивый контекст
    let m: i32 = "xyz"
        .parse::<i32>()
        .with_context(|| format!("'{}' ni parse qilib bo'lmadi", "xyz"))?;

    Ok(())
}

// anyhow::Error — barcha xatolarni qabul qiladi
// anyhow::Error — принимает все ошибки
fn anyhow_turli_xatolar() -> AnyResult<String> {

    // std::io::Error — ?
    // std::io::Error — ?
    fn io_amaliyot() -> std::io::Result<String> {
        Ok("io natija".to_string())
    }
    let io_natija: String = io_amaliyot()?;

    // ParseIntError — ?
    // ParseIntError — ?
    let n: i32 = "42".parse::<i32>()?;

    // Custom xato — ?
    // Custom ошибка — ?
    fn custom_amaliyot() -> Result<String, FaylXato> {
        Ok("custom natija".to_string())
    }
    let custom_natija: String = custom_amaliyot()?;
    // FaylXato → anyhow::Error avtomatik

    Ok(format!("{} {} {}", io_natija, n, custom_natija))
}

// ════════════════════════════════════════════════════════════════════════════
//  ANYHOW — XATO ZANJIRINI O'QISH
//  ANYHOW — ЧТЕНИЕ ЦЕПОЧКИ ОШИБОК
// ════════════════════════════════════════════════════════════════════════════

fn xato_zanjirini_chiqar(xato: &anyhow::Error) {
    println!("Xato: {}", xato);
    println!("Debug: {:?}", xato);

    // chain() — barcha sabablar
    // chain() — все причины
    for sabab in xato.chain() {
        println!("  Sabab: {}", sabab);
    }

    // root_cause() — eng pastki sabab
    // root_cause() — самая нижняя причина
    println!("Asosiy sabab: {}", xato.root_cause());
}

fn xato_zanjiri_misoli() {
    let natija: AnyResult<i32> = "abc"
        .parse::<i32>()
        .context("Konfiguratsiyani o'qishda xato")
        .context("Serverni ishga tushirishda xato");

    if let Err(ref xato) = natija {
        xato_zanjirini_chiqar(xato);
    }
    // Xato: Serverni ishga tushirishda xato
    // Sabab: Serverni ishga tushirishda xato
    // Sabab: Konfiguratsiyani o'qishda xato
    // Sabab: invalid digit found in string
    // Asosiy sabab: invalid digit found in string
}

// ════════════════════════════════════════════════════════════════════════════
//  THISERROR VA ANYHOW BIRGA
//  THISERROR И ANYHOW ВМЕСТЕ
// ════════════════════════════════════════════════════════════════════════════

// Kutubxona — thiserror (aniq xato turlari)
// Библиотека — thiserror (конкретные типы ошибок)
mod kutubxona {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum ApiXato {
        #[error("Autentifikatsiya xatosi")]
        Auth,

        #[error("Ma'lumot topilmadi: id={id}")]
        TopilmadI { id: u32 },

        #[error("Server xatosi: {0}")]
        Server(String),
    }

    pub fn foydalanuvchi_ol(id: u32) -> Result<String, ApiXato> {
        match id {
            0 => Err(ApiXato::Auth),
            1 => Ok("Dilshod".to_string()),
            _ => Err(ApiXato::TopilmadI { id }),
        }
    }
}

// Ilova — anyhow (tezkor, xato turi muhim emas)
// Приложение — anyhow (быстро, тип ошибки не важен)
fn ilova_kodi() -> AnyResult<()> {
    use kutubxona::foydalanuvchi_ol;

    // thiserror xatosi → anyhow xatosi (?)
    // thiserror ошибка → anyhow ошибка (?)
    let f1: String = foydalanuvchi_ol(1)
        .context("Foydalanuvchi 1 ni olishda xato")?;
    println!("Foydalanuvchi: {}", f1);
    // Foydalanuvchi: Dilshod

    let f2: Result<String, _> = foydalanuvchi_ol(99)
        .context("Foydalanuvchi 99 ni olishda xato");
    match f2 {
        Ok(ism) => println!("{}", ism),
        Err(e)  => println!("Xato: {}", e),
    }
    // Xato: Foydalanuvchi 99 ni olishda xato

    Ok(())
}

// ════════════════════════════════════════════════════════════════════════════
//  REAL HAYOT — TO'LIQ XATO BOSHQARUVI
//  РЕАЛЬНАЯ ЖИЗНЬ — ПОЛНАЯ ОБРАБОТКА ОШИБОК
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Error)]
enum KonfiguratsiyaXato {
    #[error("Majburiy kalit yo'q: '{kalit}'")]
    MajburiyKalitYoq { kalit: String },

    #[error("Noto'g'ri qiymat '{kalit}': {sabab}")]
    NotogriiQiymat { kalit: String, sabab: String },

    #[error("Port diapazoni: {port} (1024-65535 bo'lishi kerak)")]
    PortDiapazoni { port: u16 },
}

fn konfiguratsiya_yuklash(sozlamalar: &[(&str, &str)])
                          -> Result<(String, u16), KonfiguratsiyaXato>
{
    let xarita: std::collections::HashMap<_, _> = sozlamalar.iter().copied().collect();

    let host: &str = xarita.get("host")
        .copied()
        .ok_or_else(|| KonfiguratsiyaXato::MajburiyKalitYoq {
            kalit: "host".to_string(),
        })?;

    let port_str: &str = xarita.get("port")
        .copied()
        .ok_or_else(|| KonfiguratsiyaXato::MajburiyKalitYoq {
            kalit: "port".to_string(),
        })?;

    let port: u16 = port_str.parse::<u16>().map_err(|e| {
        KonfiguratsiyaXato::NotogriiQiymat {
            kalit: "port".to_string(),
            sabab: e.to_string(),
        }
    })?;

    if port < 1024 {
        return Err(KonfiguratsiyaXato::PortDiapazoni { port });
    }

    Ok((host.to_string(), port))
}

fn main() {

    println!("=== THISERROR ===");
    thiserror_misollari();

    println!("\n=== ANYHOW ASOSIY ===");
    match anyhow_asosiy_misollari() {
        Ok(()) => println!("Muvaffaqiyat"),
        Err(e) => println!("Xato: {}", e),
    }
    // Xato: Manfiy son: -5

    println!("\n=== ENSURE ===");
    match anyhow_ensure_misoli() {
        Ok(()) => println!("Muvaffaqiyat"),
        Err(e) => println!("Xato: {}", e),
    }
    // Muvaffaqiyat

    println!("\n=== CONTEXT ===");
    match anyhow_context_misoli() {
        Ok(()) => println!("Muvaffaqiyat"),
        Err(e) => println!("Xato: {}", e),
    }
    // Xato: config.toml faylidan son o'qishda xato

    println!("\n=== TURLI XATOLAR ===");
    match anyhow_turli_xatolar() {
        Ok(s)  => println!("{}", s),
        Err(e) => println!("Xato: {}", e),
    }
    // io natija 42 custom natija

    println!("\n=== XATO ZANJIRI ===");
    xato_zanjiri_misoli();

    println!("\n=== BIRGA ISHLASH ===");
    match ilova_kodi() {
        Ok(()) => println!("Muvaffaqiyat"),
        Err(e) => println!("Xato: {}", e),
    }

    println!("\n=== REAL HAYOT ===");
    let to_g_ri = konfiguratsiya_yuklash(&[("host", "localhost"), ("port", "8080")]);
    println!("{:?}", to_g_ri);
    // Ok(("localhost", 8080))

    let host_yoq = konfiguratsiya_yuklash(&[("port", "8080")]);
    println!("{}", host_yoq.unwrap_err());
    // Majburiy kalit yo'q: 'host'

    let port_xato = konfiguratsiya_yuklash(&[("host", "localhost"), ("port", "abc")]);
    println!("{}", port_xato.unwrap_err());
    // Noto'g'ri qiymat 'port': invalid digit found in string

    let port_kichik = konfiguratsiya_yuklash(&[("host", "localhost"), ("port", "80")]);
    println!("{}", port_kichik.unwrap_err());
    // Port diapazoni: 80 (1024-65535 bo'lishi kerak)
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                  | Описание (RU)                                               |
// #================================================================================================================================================#
// # |                                       THISERROR                                                                                              |
// #================================================================================================================================================#
// # |   1 | #[derive(Error)]          | std::error::Error avtomatik                  | Автоматически реализует Error                               |
// # |   2 | #[error("...")]           | Display avtomatik                            | Автоматически реализует Display                             |
// # |   3 | #[error("... {field}")]   | Field nomini xabarga kiritish                | Вставка имени поля в сообщение                              |
// # |   4 | #[from]                   | From trait avtomatik, ? bilan ishlaydi       | Автоматический From, работает с ?                           |
// # |   5 | #[source]                 | source() metodi uchun                        | Для метода source()                                         |
// # |   6 | Kutubxonalar uchun        | Aniq xato turlari kerak bo'lganda            | Когда нужны конкретные типы ошибок                          |
// #================================================================================================================================================#
// # |                                       ANYHOW                                                                                                 |
// #================================================================================================================================================#
// # |   7 | anyhow::Result<T>         | Result<T, anyhow::Error>                     | Result<T, anyhow::Error>                                    |
// # |   8 | anyhow!("...")            | Yangi xato yaratish                          | Создание новой ошибки                                       |
// # |   9 | bail!("...")              | Err qaytarish uchun qisqa yozuv              | Краткая запись для возврата Err                             |
// # |  10 | ensure!(shart, "...")     | Shart tekshirish, aks holda bail!            | Проверка условия, иначе bail!                               |
// # |  11 | .context("...")           | Xatoga kontekst qo'shish                     | Добавление контекста к ошибке                               |
// # |  12 | .with_context(|| "...")   | Lazy kontekst (faqat xato bo'lsa)            | Ленивый контекст (только при ошибке)                        |
// # |  13 | .chain()                  | Barcha sabablar zanjiri                      | Цепочка всех причин                                         |
// # |  14 | .root_cause()             | Eng pastki sabab                             | Самая нижняя причина                                        |
// # |  15 | Ilovalar uchun            | Xato turini bilish shart bo'lmaganda         | Когда тип ошибки не важен                                   |
// #================================================================================================================================================#
// # |                                       BIRGA ISHLATISH                                                                                        |
// #================================================================================================================================================#
// # |  16 | Kutubxona → thiserror     | Aniq xato turlari eksport qilish             | Экспорт конкретных типов ошибок                             |
// # |  17 | Ilova → anyhow            | thiserror xatolarini ? bilan yutish          | Поглощение thiserror ошибок через ?                         |
// # |  18 | thiserror + anyhow        | Professional xato boshqaruvi                 | Профессиональная обработка ошибок                           |
// #================================================================================================================================================#