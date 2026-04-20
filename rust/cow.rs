// #================================================================================================================================================#
// #                                                                 COW<'A, T>                                                                     #
// #                                COW — CLONE ON WRITE. BORROWED YOKI OWNED. FAQAT O'ZGARTIRISHDA CLONE QILADI.                                   #
// #                                COW — CLONE ON WRITE. BORROWED ИЛИ OWNED. КЛОНИРУЕТ ТОЛЬКО ПРИ ИЗМЕНЕНИИ.                                       #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::borrow::Cow;
use std::borrow::ToOwned;
use std::fmt;

// Cow<'a, B> — ikki holatdan biri:
// Cow<'a, B> — одно из двух состояний:
//
//   Cow::Borrowed(&'a B)  — reference (clone yo'q, tez)
//   Cow::Owned(B::Owned)  — owned (clone bo'lgan, to'liq nazorat)
//
// B: ToOwned + ?Sized bo'lishi kerak
// B должен реализовать ToOwned + ?Sized
//
// Eng ko'p ishlatish:
// Наиболее частое использование:
//   Cow<'a, str>    — &str yoki String
//   Cow<'a, [T]>    — &[T] yoki Vec<T>
//   Cow<'a, Path>   — &Path yoki PathBuf

fn cow_yaratish_misollari() {

    // Cow::Borrowed — reference (clone yo'q)
    // Cow::Borrowed — ссылка (без clone)
    let borrowed: Cow<str> = Cow::Borrowed("salom dunyo");
    println!("{}", borrowed);
    println!("Borrowed: {}", matches!(borrowed, Cow::Borrowed(_)));
    // salom dunyo
    // Borrowed: true

    // Cow::Owned — owned String
    // Cow::Owned — owned String
    let owned: Cow<str> = Cow::Owned(String::from("salom dunyo"));
    println!("{}", owned);
    println!("Owned: {}", matches!(owned, Cow::Owned(_)));
    // salom dunyo
    // Owned: true

    // From<&str> — Borrowed yaratish
    // From<&str> — создание Borrowed
    let from_str: Cow<str> = Cow::from("salom");
    println!("{}", from_str);
    // salom

    // From<String> — Owned yaratish
    // From<String> — создание Owned
    let from_string: Cow<str> = Cow::from(String::from("salom"));
    println!("{}", from_string);
    // salom

    // &str.into() — Borrowed
    // &str.into() — Borrowed
    let into_borrowed: Cow<str> = "salom".into();
    println!("{}", into_borrowed);
    // salom

    // String.into() — Owned
    // String.into() — Owned
    let into_owned: Cow<str> = String::from("salom").into();
    println!("{}", into_owned);
    // salom

    // Cow<[T]> — slice uchun
    // Cow<[T]> — для slice
    let slice_borrowed: Cow<[i32]> = Cow::Borrowed(&[1, 2, 3]);
    let slice_owned: Cow<[i32]> = Cow::Owned(vec![1, 2, 3]);
    println!("{:?}", slice_borrowed);
    println!("{:?}", slice_owned);
    // [1, 2, 3]
    // [1, 2, 3]
}

fn cow_metodlar_misollari() {

    // matches!(cow, Cow::Borrowed(_)) — Borrowed ekanligini tekshirish
    // проверка является ли Borrowed
    let borrowed: Cow<str> = Cow::Borrowed("salom");
    let owned: Cow<str> = Cow::Owned(String::from("dunyo"));
    println!("{}", matches!(borrowed, Cow::Borrowed(_)));
    println!("{}", matches!(owned, Cow::Borrowed(_)));
    // true
    // false

    // matches!(cow, Cow::Owned(_)) — Owned ekanligini tekshirish
    // проверка является ли Owned
    println!("{}", matches!(borrowed, Cow::Owned(_)));
    println!("{}", matches!(owned, Cow::Owned(_)));
    // false
    // true

    // .into_owned() — har doim owned qaytaradi
    // всегда возвращает owned
    let borrowed2: Cow<str> = Cow::Borrowed("salom");
    let owned_str: String = borrowed2.into_owned();
    println!("{}", owned_str);
    // salom

    // .to_mut() — o'zgartirish uchun (kerak bo'lsa clone qiladi)
    // для изменения (клонирует если нужно)
    let mut cow: Cow<str> = Cow::Borrowed("salom");
    println!("Oldin: {}", matches!(cow, Cow::Borrowed(_)));
    let s: &mut String = cow.to_mut();
    s.push_str(" dunyo");
    println!("Keyin: {}", matches!(cow, Cow::Owned(_)));
    println!("{}", cow);
    // Oldin: true
    // Keyin: true   ← endi owned!
    // salom dunyo

    // .as_ref() — &B reference olish
    // получение &B ссылки
    let cow2: Cow<str> = Cow::Borrowed("rust");
    let str_ref: &str = cow2.as_ref();
    println!("{}", str_ref);
    // rust

    // len() — DST metodlarini chaqirish (Deref orqali)
    // вызов методов DST (через Deref)
    let cow3: Cow<str> = Cow::Borrowed("salom dunyo");
    let uzunlik: usize = cow3.len();
    let katta: String = cow3.to_uppercase();
    println!("{}", uzunlik);
    println!("{}", katta);
    // 11
    // SALOM DUNYO
}

fn to_mut_misollari() {

    // to_mut — Borrowed bo'lsa clone qilib Owned ga o'tadi
    // to_mut — клонирует Borrowed и переходит в Owned
    let mut cow1: Cow<str> = Cow::Borrowed("salom");
    {
        let s: &mut String = cow1.to_mut();
        s.push_str(" dunyo!");
    }
    println!("{}", cow1);
    println!("Owned: {}", matches!(cow1, Cow::Owned(_)));
    // salom dunyo!
    // Owned: true

    // to_mut — Owned bo'lsa clone QILMAYDI
    // to_mut — если Owned, НЕ клонирует
    let mut cow2: Cow<str> = Cow::Owned(String::from("rust"));
    {
        let s: &mut String = cow2.to_mut();
        s.push_str(" tili");
    }
    println!("{}", cow2);
    // rust tili

    // Cow<[i32]> — to_mut bilan
    // Cow<[i32]> — с to_mut
    let mut cow3: Cow<[i32]> = Cow::Borrowed(&[1, 2, 3]);
    cow3.to_mut().push(4);
    cow3.to_mut().push(5);
    println!("{:?}", cow3);
    // [1, 2, 3, 4, 5]
}

// Cow qaytarish — kerak bo'lganda clone
// возврат Cow — clone только когда нужно
fn tozala<'a>(matn: &'a str) -> Cow<'a, str> {
    if matn.contains("  ") {
        // ikki bo'sh joy bor — o'zgartirish kerak
        // есть двойные пробелы — нужно изменить
        Cow::Owned(matn.replace("  ", " "))
    } else {
        // o'zgartirish kerak emas — clone yo'q!
        // изменение не нужно — без clone!
        Cow::Borrowed(matn)
    }
}

// katta harfga aylantirish — kerak bo'lganda
// преобразование в верхний регистр — когда нужно
fn shart_bilan_katta_qil<'a>(matn: &'a str, kerakmi: bool) -> Cow<'a, str> {
    if kerakmi {
        Cow::Owned(matn.to_uppercase())
    } else {
        Cow::Borrowed(matn)
    }
}

// HTML escape — faqat kerak bo'lganda
// HTML escape — только когда нужно
fn html_escape<'a>(matn: &'a str) -> Cow<'a, str> {
    if matn.contains('<') || matn.contains('>') || matn.contains('&') {
        let mut natija = String::with_capacity(matn.len());
        for ch in matn.chars() {
            match ch {
                '<' => natija.push_str("&lt;"),
                '>' => natija.push_str("&gt;"),
                '&' => natija.push_str("&amp;"),
                _   => natija.push(ch),
            }
        }
        Cow::Owned(natija)
    } else {
        Cow::Borrowed(matn)
    }
}

// ToOwned implement qilingan custom struct
// пользовательская структура с ToOwned

#[derive(Debug, Clone, PartialEq)]
struct Config {
    port: u16,
    host: String,
}

impl Config {
    fn new(port: u16, host: &str) -> Self {
        Config { port, host: host.to_string() }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

// Clone bo'lgani uchun Cow<Config> mumkin
// Cow<Config> возможен так как есть Clone
fn config_bilan_ishlash() {
    let asl_config = Config::new(8080, "localhost");

    // Borrowed — clone yo'q
    // Borrowed — без clone
    let cow_config: Cow<Config> = Cow::Borrowed(&asl_config);
    println!("{}", cow_config);
    println!("Borrowed: {}", matches!(cow_config, Cow::Borrowed(_)));
    // localhost:8080
    // Borrowed: true

    // to_mut — kerak bo'lganda clone
    // to_mut — clone когда нужно
    let mut cow_config2: Cow<Config> = Cow::Borrowed(&asl_config);
    cow_config2.to_mut().port = 3000;
    println!("{}", cow_config2);
    println!("Owned: {}", matches!(cow_config2, Cow::Owned(_)));
    // localhost:3000
    // Owned: true

    // asl o'zgarmadi
    // оригинал не изменился
    println!("{}", asl_config);
    // localhost:8080
}

// 1. String normalizatsiya — ko'p holatlarda clone yo'q
// 1. Нормализация строк — в большинстве случаев без clone
fn normalizatsiya<'a>(matn: &'a str) -> Cow<'a, str> {
    let trimmed: &str = matn.trim();
    if trimmed == matn && !matn.contains("  ") {
        // O'zgartirish kerak emas
        // Изменение не нужно
        Cow::Borrowed(matn)
    } else {
        // O'zgartirish kerak
        // Нужно изменить
        Cow::Owned(trimmed.replace("  ", " "))
    }
}

// 2. Log xabarlari — prefiks qo'shish
// 2. Сообщения лога — добавление префикса
fn log_formatlash<'a>(daraja: &str, xabar: &'a str) -> Cow<'a, str> {
    if daraja == "ODDIY" {
        // Oddiy daraja — prefiks qo'shmaydi
        // Обычный уровень — без префикса
        Cow::Borrowed(xabar)
    } else {
        // Boshqa daraja — prefiks qo'shadi
        // Другой уровень — добавляет префикс
        Cow::Owned(format!("[{}] {}", daraja, xabar))
    }
}

// 3. Fayl yo'li normalizatsiya
// 3. Нормализация пути к файлу
fn yol_normalizatsiya<'a>(yol: &'a str) -> Cow<'a, str> {
    if yol.starts_with('/') {
        Cow::Borrowed(yol)
    } else {
        Cow::Owned(format!("/{}", yol))
    }
}

// 4. Iterator bilan Cow
// 4. Cow с итератором
fn cow_iterator_misoli() {
    let sozlar: Vec<&str> = vec!["salom", "  ikki  ", "uch", "  tort  "];

    let natijalar: Vec<Cow<str>> = sozlar.iter()
        .map(|s| normalizatsiya(s))
        .collect();

    let mut borrowed_soni: usize = 0;
    let mut owned_soni: usize = 0;

    for cow in &natijalar {
        if matches!(cow, Cow::Borrowed(_)) {
            borrowed_soni += 1;
        } else {
            owned_soni += 1;
        }
        println!("{}", cow);
    }

    println!("Borrowed (clone yo'q): {}", borrowed_soni);
    println!("Owned (clone bo'lgan): {}", owned_soni);
    // salom
    // ikki
    // uch
    // tort
    // Borrowed (clone yo'q): 2
    // Owned (clone bo'lgan): 2
}

// 5. API response — conditional clone
// 5. Ответ API — условное клонирование
struct ApiResponse<'a> {
    status: u16,
    body: Cow<'a, str>,
}

impl<'a> ApiResponse<'a> {
    fn ok(body: &'a str) -> Self {
        ApiResponse {
            status: 200,
            // Borrowed — clone yo'q
            // Borrowed — без clone
            body: Cow::Borrowed(body),
        }
    }

    fn xato(kod: u16, xabar: &str) -> Self {
        ApiResponse {
            status: kod,
            // Owned — format! yangi String yaratadi
            // Owned — format! создаёт новую String
            body: Cow::Owned(format!("Xato {}: {}", kod, xabar)),
        }
    }

    fn chiqar(&self) {
        println!("Status: {} | Body: {}", self.status, self.body);
        println!("Borrowed: {}", matches!(self.body, Cow::Borrowed(_)));
    }
}

fn main() {
    cow_yaratish_misollari();

    cow_metodlar_misollari();

    to_mut_misollari();

    // tozala — ikki bo'sh joy bor → Owned
    // tozala — есть двойные пробелы → Owned
    let t1: Cow<str> = tozala("salom  dunyo");
    let t2: Cow<str> = tozala("salom dunyo");
    println!("{} (owned: {})", t1, matches!(t1, Cow::Owned(_)));
    println!("{} (borrowed: {})", t2, matches!(t2, Cow::Borrowed(_)));
    // salom dunyo (owned: true)
    // salom dunyo (borrowed: true)

    // shart_bilan_katta_qil
    // shart_bilan_katta_qil
    let k1: Cow<str> = shart_bilan_katta_qil("salom", true);
    let k2: Cow<str> = shart_bilan_katta_qil("salom", false);
    println!("{} (owned: {})", k1, matches!(k1, Cow::Owned(_)));
    println!("{} (borrowed: {})", k2, matches!(k2, Cow::Borrowed(_)));
    // SALOM (owned: true)
    // salom (borrowed: true)

    // html_escape — < > & bor → Owned
    // html_escape — есть < > & → Owned
    let h1: Cow<str> = html_escape("<script>alert('xss')</script>");
    let h2: Cow<str> = html_escape("oddiy matn");
    println!("{}", h1);
    println!("{} (borrowed: {})", h2, matches!(h2, Cow::Borrowed(_)));
    // &lt;script&gt;alert('xss')&lt;/script&gt;
    // oddiy matn (borrowed: true)

    config_bilan_ishlash();

    // normalizatsiya
    // нормализация
    let n1: Cow<str> = normalizatsiya("  salom  dunyo  ");
    let n2: Cow<str> = normalizatsiya("salom dunyo");
    println!("{} (owned: {})", n1, matches!(n1, Cow::Owned(_)));
    println!("{} (borrowed: {})", n2, matches!(n2, Cow::Borrowed(_)));
    // salom dunyo (owned: true)
    // salom dunyo (borrowed: true)

    // log_formatlash
    // log_formatlash
    let l1: Cow<str> = log_formatlash("ODDIY", "ma'lumot");
    let l2: Cow<str> = log_formatlash("XATO", "fayl topilmadi");
    println!("{} (borrowed: {})", l1, matches!(l1, Cow::Borrowed(_)));
    println!("{} (owned: {})", l2, matches!(l2, Cow::Owned(_)));
    // ma'lumot (borrowed: true)
    // [XATO] fayl topilmadi (owned: true)

    // yol_normalizatsiya
    // нормализация пути
    let y1: Cow<str> = yol_normalizatsiya("/home/user");
    let y2: Cow<str> = yol_normalizatsiya("home/user");
    println!("{} (borrowed: {})", y1, matches!(y1, Cow::Borrowed(_)));
    println!("{} (owned: {})", y2, matches!(y2, Cow::Owned(_)));
    // /home/user (borrowed: true)
    // /home/user (owned: true)

    // iterator bilan
    // с итератором
    cow_iterator_misoli();

    // ApiResponse
    // ApiResponse
    let muvaffaqiyatli = ApiResponse::ok("Muvaffaqiyatli!");
    let xatoli = ApiResponse::xato(404, "sahifa topilmadi");
    muvaffaqiyatli.chiqar();
    xatoli.chiqar();
    // Status: 200 | Body: Muvaffaqiyatli!
    // Borrowed: true
    // Status: 404 | Body: Xato 404: sahifa topilmadi
    // Borrowed: false

    // Pattern: o'zgartirish kerak bo'lmasa — Borrowed (clone yo'q)
    //          o'zgartirish kerak bo'lsa   — Owned (clone bo'ladi)
    // Паттерн: не нужно изменять — Borrowed (без clone)
    //          нужно изменять   — Owned (с clone)

    let matnlar: Vec<&str> = vec![
        "oddiy matn",
        "<html>",
        "yana oddiy",
        "& belgi",
        "xavfsiz",
    ];

    let mut clone_soni: usize = 0;
    for matn in &matnlar {
        let escaped: Cow<str> = html_escape(matn);
        if matches!(escaped, Cow::Owned(_)) {
            clone_soni += 1;
        }
        println!("{:<30} → {}", matn, escaped);
    }
    println!("\nJami {} ta clone bo'ldi ({} ta borrowed qoldi)",
             clone_soni,
             matnlar.len() - clone_soni
    );
    // oddiy matn                     → oddiy matn
    // <html>                         → &lt;html&gt;
    // yana oddiy                     → yana oddiy
    // & belgi                        → &amp; belgi
    // xavfsiz                        → xavfsiz
    //
    // Jami 2 ta clone bo'ldi (3 ta borrowed qoldi)
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                          COW YARATISH                                                                                        |
// #================================================================================================================================================#
// # |   1 | Cow::Borrowed(&'a B)     | Reference — clone yo'q, tez                          | Ссылка — без clone, быстро                           |
// # |   2 | Cow::Owned(B::Owned)     | Owned — to'liq nazorat                               | Owned — полный контроль                              |
// # |   3 | Cow::from("str")         | &str → Borrowed                                      | &str → Borrowed                                      |
// # |   4 | Cow::from(String)        | String → Owned                                       | String → Owned                                       |
// # |   5 | "str".into()             | &str → Cow Borrowed                                  | &str → Cow Borrowed                                  |
// # |   6 | String.into()            | String → Cow Owned                                   | String → Cow Owned                                   |
// #================================================================================================================================================#
// # |                                          COW METODLAR                                                                                        |
// #================================================================================================================================================#
// # |   7 | matches!(cow, Borrowed(_))  | Borrowed ekanligini tekshirish                    | Проверка является ли Borrowed                        |
// # |   8 | matches!(cow, Owned(_))     | Owned ekanligini tekshirish                       | Проверка является ли Owned                           |
// # |   9 | .into_owned()               | Har doim owned qaytaradi (kerak bo'lsa clone)     | Всегда возвращает owned (clone если нужно)           |
// # |  10 | .to_mut()                   | &mut T olish (Borrowed bo'lsa clone qiladi)       | Получение &mut T (клонирует если Borrowed)           |
// # |  11 | .as_ref()                   | &B reference olish                                | Получение ссылки &B                                  |
// # |  12 | DST metodlar (Deref)        | .len(), .contains() va boshqalar to'g'ridan       | .len(), .contains() и др. напрямую                   |
// #================================================================================================================================================#
// # |                                          TO_MUT QOIDASI                                                                                      |
// #================================================================================================================================================#
// # |  13 | Borrowed + to_mut()      | Clone qilib Owned ga o'tadi                          | Клонирует и переходит в Owned                        |
// # |  14 | Owned + to_mut()         | Clone QILMAYDI — allaqachon owned                    | НЕ клонирует — уже owned                             |
// #================================================================================================================================================#
// # |                                          COW TURLARI                                                                                         |
// #================================================================================================================================================#
// # |  15 | Cow<'a, str>             | &str yoki String                                     | &str или String                                      |
// # |  16 | Cow<'a, [T]>             | &[T] yoki Vec<T>                                     | &[T] или Vec<T>                                      |
// # |  17 | Cow<'a, Path>            | &Path yoki PathBuf                                   | &Path или PathBuf                                    |
// # |  18 | Cow<'a, T: Clone>        | &T yoki T (custom struct)                            | &T или T (пользовательская структура)                |
// #================================================================================================================================================#
// # |                                       QACHON ISHLATISH                                                                                       |
// #================================================================================================================================================#
// # |  19 | Ko'pincha o'zgarmas      | Borrowed — clone yo'q, performance yaxshi            | Borrowed — без clone, хорошая производительность     |
// # |  20 | Ba'zan o'zgaruvchan      | Owned — o'zgartirish kerak bo'lganda                 | Owned — когда нужно изменить                         |
// # |  21 | HTML escape              | Maxsus belgi yo'q → Borrowed, bor → Owned            | Нет спец. символов → Borrowed, есть → Owned          |
// # |  22 | String normalizatsiya    | O'zgartirish kerak emas → Borrowed                   | Изменение не нужно → Borrowed                        |
// # |  23 | API response body        | Statik matn → Borrowed, dinamik → Owned              | Статичный текст → Borrowed, динамика → Owned         |
// #================================================================================================================================================#
// # |                                       CLONE ON WRITE                                                                                         |
// #================================================================================================================================================#
// # |  24 | Read only → Borrowed     | O'qishda clone yo'q                                  | При чтении clone отсутствует                         |
// # |  25 | Write → Owned            | Yozishda (to_mut) kerak bo'lganda clone              | При записи (to_mut) clone когда нужно                |
// # |  26 | Performance              | Ko'pchilik holatlarda clone yo'q — tez               | В большинстве случаев без clone — быстро             |
// #================================================================================================================================================#