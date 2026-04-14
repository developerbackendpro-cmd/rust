// #================================================================================================================================================#
// #                                                       HRTB — FOR<'A> (HIGHER-RANKED TRAIT BOUNDS)                                              #
// #                               HRTB — "BARCHA MUMKIN BO'LGAN LIFETIMELAR UCHUN". ISTALGAN LIFETIMELI REFERENS BILAN ISHLAYDI.                   #
// #                               HRTB — "ДЛЯ ВСЕХ ВОЗМОЖНЫХ LIFETIME". РАБОТАЕТ С ССЫЛКАМИ ЛЮБОГО LIFETIME.                                       #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// Muammo: quyidagi kod ishlamaydi
// Проблема: следующий код не работает
//
//   fn chaqir<'a, F: Fn(&'a str) -> &'a str>(f: F, s: &'a str) -> &'a str {
//       f(s)
//   }
//
// Sabab: 'a bitta aniq lifetime — F faqat shu 'a uchun ishlaydi
// Причина: 'a — один конкретный lifetime — F работает только для этого 'a
//
// Yechim: HRTB — for<'a>
// Решение: HRTB — for<'a>
//
//   fn chaqir<F: for<'a> Fn(&'a str) -> &'a str>(f: F, s: &str) -> &str {
//       f(s)
//   }
//
// for<'a> — "F BARCHA mumkin bo'lgan 'a uchun Fn(&'a str) -> &'a str"
// for<'a> — "F реализует Fn(&'a str) -> &'a str для ВСЕХ возможных 'a"

// HRTB bilan — istalgan lifetime qabul qiladi
// С HRTB — принимает любой lifetime
fn hrtb_chaqir<F>(f: F, s: &str) -> &str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(s)
}

// Bir nechta hrtb
// Несколько hrtb
fn hrtb_ikki_argument<F>(f: F, a: &str, b: &str) -> bool
where
    F: for<'a> Fn(&'a str, &'a str) -> bool,
{
    f(a, b)
}

// Oddiy lifetime bound — MUAMMO
// Обычное ограничение lifetime — ПРОБЛЕМА
fn oddiy_chaqir<'a, F>(f: F, s: &'a str) -> &'a str
where
    F: Fn(&'a str) -> &'a str,
{
    f(s)
}
// Bu funksiya f ni FAQAT 'a lifetime bilan ishlatishi mumkin
// Эта функция может использовать f ТОЛЬКО с lifetime 'a
// Lekin biz f ni har xil lifetime bilan ishlatmoqchimiz!
// Но мы хотим использовать f с разными lifetime!

// HRTB bilan — TO'G'RI YECHIM
// С HRTB — ПРАВИЛЬНОЕ РЕШЕНИЕ
fn hrtb_to_g_ri<F>(f: F) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    // f ni turli lifetime bilan chaqira olamiz
    // можем вызывать f с разными lifetime
    let s1: &str = "birinchi";
    let natija1: &str = f(s1);

    let s2: String = String::from("ikkinchi");
    let natija2: &str = f(&s2);

    format!("{} va {}", natija1, natija2)
}

// Closure — HRTB avtomatik implement qiladi
// Замыкание — автоматически реализует HRTB
fn closure_hrtb_misoli() {

    // Bu closure for<'a> Fn(&'a str) -> &'a str implement qiladi
    // Это замыкание реализует for<'a> Fn(&'a str) -> &'a str
    // HRTB closure — fn pointer sifatida yoziladi
    // HRTB closure — записывается как fn pointer
    fn identity(s: &str) -> &str { s }

    // turli lifetime bilan chaqirish
    // вызов с разными lifetime
    let s1: &'static str = "statik literal";
    let s2: String = String::from("owned string");

    let r1: &str = identity(s1);
    let r2: &str = identity(&s2);
    println!("{}", r1);
    println!("{}", r2);
    // statik literal
    // owned string

    // hrtb_chaqir bilan
    // с hrtb_chaqir
    fn birinchi_soz(s: &str) -> &str {
        s.split_whitespace().next().unwrap_or("")
    }
    let gap: &str = "salom dunyo rust";
    let natija: &str = hrtb_chaqir(birinchi_soz, gap);
    println!("{}", natija);
    // salom

    // hrtb_ikki_argument bilan
    // с hrtb_ikki_argument
    let uzunroqmi = |a: &str, b: &str| a.len() > b.len();
    println!("{}", hrtb_ikki_argument(uzunroqmi, "salom", "hi"));
    // true
}

// HRTB bilan trait
// Трейт с HRTB
trait QaytaIshlovchi {
    fn qayta_ishla<'a>(&self, kiritish: &'a str) -> &'a str;
}

// HRTB bilan trait bound
// Ограничение трейта с HRTB
fn tizim_ishga_tush<T>(ishlovchi: &T, kiritish: &str) -> String
where
    T: for<'a> Fn(&'a str) -> &'a str,
{
    let natija: &str = ishlovchi(kiritish);
    format!("Natija: {}", natija)
}

// Fn, FnMut, FnOnce bilan HRTB
// HRTB с Fn, FnMut, FnOnce

// for<'a> Fn(&'a str)
fn fn_hrtb<F>(f: F, s: &str)
where
    F: for<'a> Fn(&'a str),
{
    f(s);
}

// for<'a> FnMut(&'a str)
fn fnmut_hrtb<F>(mut f: F, s: &str)
where
    F: for<'a> FnMut(&'a str),
{
    f(s);
}

// for<'a> FnOnce(&'a str) — bir marta chaqiriladi
// for<'a> FnOnce(&'a str) — вызывается один раз
fn fnonce_hrtb<F>(f: F, s: &str)
where
    F: for<'a> FnOnce(&'a str),
{
    f(s);
}

// Generic struct + HRTB
// Generic структура + HRTB
struct Ishlovchi<F>
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    funksiya: F,
}

impl<F> Ishlovchi<F>
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    fn new(funksiya: F) -> Self {
        Ishlovchi { funksiya }
    }

    fn ishla<'a>(&self, kiritish: &'a str) -> &'a str {
        (self.funksiya)(kiritish)
    }

    fn ikki_marta<'a>(&self, kiritish: &'a str) -> String {
        let birinchi: &str = (self.funksiya)(kiritish);
        let ikkinchi: &str = (self.funksiya)(kiritish);
        format!("{} {}", birinchi, ikkinchi)
    }
}

// HRTB bilan ishlovchilar zanjiri
// Цепочка обработчиков с HRTB
struct ZanjirIshlovchi<F, G>
where
    F: for<'a> Fn(&'a str) -> &'a str,
    G: for<'a> Fn(&'a str) -> &'a str,
{
    birinchi: F,
    ikkinchi: G,
}

impl<F, G> ZanjirIshlovchi<F, G>
where
    F: for<'a> Fn(&'a str) -> &'a str,
    G: for<'a> Fn(&'a str) -> &'a str,
{
    fn new(birinchi: F, ikkinchi: G) -> Self {
        ZanjirIshlovchi { birinchi, ikkinchi }
    }

    fn ishla<'a>(&self, kiritish: &'a str) -> String {
        let oraliq: &str = (self.birinchi)(kiritish);
        let natija: &str = (self.ikkinchi)(oraliq);
        natija.to_string()
    }
}

// 1. Validator — HRTB bilan
// 1. Валидатор — с HRTB
struct Validator<F>
where
    F: for<'a> Fn(&'a str) -> bool,
{
    tekshirish: F,
    xabar: &'static str,
}

impl<F> Validator<F>
where
    F: for<'a> Fn(&'a str) -> bool,
{
    fn new(tekshirish: F, xabar: &'static str) -> Self {
        Validator { tekshirish, xabar }
    }

    fn tekshir(&self, kiritish: &str) -> Result<(), &'static str> {
        if (self.tekshirish)(kiritish) {
            Ok(())
        } else {
            Err(self.xabar)
        }
    }
}

fn validator_zanjiri(kiritish: &str) -> Result<(), Vec<&'static str>> {
    let validatorlar: Vec<Box<dyn for<'a> Fn(&'a str) -> Result<(), &'static str>>> = vec![
        Box::new(|s: &str| {
            if s.is_empty() { Err("Bo'sh bo'lishi mumkin emas") } else { Ok(()) }
        }),
        Box::new(|s: &str| {
            if s.len() < 3 { Err("Kamida 3 ta belgi") } else { Ok(()) }
        }),
        Box::new(|s: &str| {
            if s.contains('@') { Ok(()) } else { Err("@ belgisi kerak") }
        }),
    ];

    let xatolar: Vec<&'static str> = validatorlar.iter()
        .filter_map(|v| v(kiritish).err())
        .collect();

    if xatolar.is_empty() { Ok(()) } else { Err(xatolar) }
}

// 2. Matn o'zgartiruvchi pipeline
// 2. Конвейер преобразования текста
fn matn_pipeline(
    kiritish: &str,
    bosqichlar: &[Box<dyn for<'a> Fn(&'a str) -> String>],
) -> String {
    let mut joriy: String = kiritish.to_string();
    for bosqich in bosqichlar {
        joriy = bosqich(&joriy);
    }
    joriy
}

// 3. Generic sort — HRTB bilan taqqoslash
// 3. Generic сортировка — сравнение с HRTB
fn hrtb_sort<T, F>(elementlar: &mut Vec<T>, taqqoslash: F)
where
    F: for<'a> Fn(&'a T, &'a T) -> std::cmp::Ordering,
{
    elementlar.sort_by(|a, b| taqqoslash(a, b));
}

fn main() {

    // hrtb_chaqir — istalgan lifetime
    // hrtb_chaqir — любой lifetime
    let gap: &str = "salom dunyo";
    let natija: &str = hrtb_chaqir(|s| s, gap);
    println!("{}", natija);
    // salom dunyo

    // hrtb_to_g_ri — turli lifetime bilan
    // hrtb_to_g_ri — с разными lifetime
    let natija2: String = hrtb_to_g_ri(|s| s);
    println!("{}", natija2);
    // birinchi va ikkinchi

    closure_hrtb_misoli();

    // fn_hrtb — Fn bilan
    // fn_hrtb — с Fn
    fn_hrtb(|s| println!("Fn: {}", s), "salom");
    // Fn: salom

    // fnmut_hrtb — FnMut bilan
    // fnmut_hrtb — с FnMut
    let mut hisob: i32 = 0;
    fnmut_hrtb(|s| {
        hisob += 1;
        println!("FnMut {}: {}", hisob, s);
    }, "dunyo");
    // FnMut 1: dunyo

    // fnonce_hrtb — FnOnce bilan
    // fnonce_hrtb — с FnOnce
    let prefix: String = String::from("[LOG] ");
    fnonce_hrtb(move |s| {
        println!("{}{}", prefix, s);
    }, "rust");
    // [LOG] rust

    // Ishlovchi struct
    // Структура Ishlovchi
    let trim_ishlov = Ishlovchi::new(|s: &str| s.trim());
    let s1: String = String::from("  salom  ");
    let s2: &str = "  dunyo  ";
    println!("{}", trim_ishlov.ishla(&s1));
    println!("{}", trim_ishlov.ishla(s2));
    println!("{}", trim_ishlov.ikki_marta("  rust  "));
    // salom
    // dunyo
    // rust rust

    // ZanjirIshlovchi — ikki funksiya
    // ZanjirIshlovchi — две функции
    let zanjir = ZanjirIshlovchi::new(
        |s: &str| s.trim(),
        |s: &str| s.split_whitespace().next().unwrap_or(""),
    );
    println!("{}", zanjir.ishla("  salom dunyo  "));
    // salom

    // tizim_ishga_tush — HRTB bound
    // tizim_ishga_tush — ограничение HRTB
    fn ishlovchi(s: &str) -> &str { s }
    let natija3: String = tizim_ishga_tush(&ishlovchi, "salom");
    println!("{}", natija3);
    // Natija: salom

    // 1. Validator zanjiri
    // 1. Цепочка валидаторов
    let to_g_ri = validator_zanjiri("dilshod@example.com");
    let xatoli1 = validator_zanjiri("");
    let xatoli2 = validator_zanjiri("di");
    let xatoli3 = validator_zanjiri("dilshod");

    println!("{:?}", to_g_ri);
    println!("{:?}", xatoli1);
    println!("{:?}", xatoli2);
    println!("{:?}", xatoli3);
    // Ok(())
    // Err(["Bo'sh bo'lishi mumkin emas", "Kamida 3 ta belgi", "@ belgisi kerak"])
    // Err(["Kamida 3 ta belgi", "@ belgisi kerak"])
    // Err(["@ belgisi kerak"])

    // 2. Matn pipeline
    // 2. Конвейер текста
    let bosqichlar: Vec<Box<dyn for<'a> Fn(&'a str) -> String>> = vec![
        Box::new(|s: &str| s.trim().to_string()),
        Box::new(|s: &str| s.to_uppercase()),
        Box::new(|s: &str| format!("[{}]", s)),
    ];
    let natija4: String = matn_pipeline("  salom dunyo  ", &bosqichlar);
    println!("{}", natija4);
    // [SALOM DUNYO]

    // 3. Generic sort — HRTB bilan
    // 3. Generic сортировка — с HRTB
    let mut sozlar: Vec<String> = vec![
        String::from("banan"),
        String::from("olma"),
        String::from("anor"),
        String::from("uzum"),
    ];
    hrtb_sort(&mut sozlar, |a, b| a.cmp(b));
    println!("{:?}", sozlar);
    // ["anor", "banan", "olma", "uzum"]

    // uzunlik bo'yicha sort
    // сортировка по длине
    hrtb_sort(&mut sozlar, |a, b| a.len().cmp(&b.len()));
    println!("{:?}", sozlar);
    // ["anor", "olma", "uzum", "banan"]
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                       HRTB ASOSLARI                                                                                          |
// #================================================================================================================================================#
// # |   1 | for<'a> Fn(&'a str)      | BARCHA lifetime uchun Fn trait                       | Трейт Fn для ВСЕХ lifetime                           |
// # |   2 | for<'a> Fn(&'a T) -> &'a T| Kiritish va chiqish bir xil lifetime                | Вход и выход одного lifetime                         |
// # |   3 | for<'a, 'b> Fn(...)      | Bir nechta HRTB lifetime                             | Несколько HRTB lifetime                              |
// #================================================================================================================================================#
// # |                                       ODDIY BOUND VS HRTB                                                                                    |
// #================================================================================================================================================#
// # |   4 | F: Fn(&'a str)           | FAQAT 'a lifetime — cheklangan                       | ТОЛЬКО lifetime 'a — ограничено                      |
// # |   5 | F: for<'a> Fn(&'a str)   | BARCHA lifetime — universal                          | ВСЕ lifetime — универсально                          |
// # |   6 | Closure — avtomatik HRTB | Closure lar for<'a> ni avtomatik implement qiladi    | Замыкания автоматически реализуют for<'a>            |
// #================================================================================================================================================#
// # |                                       FN TRAITLAR BILAN                                                                                      |
// #================================================================================================================================================#
// # |   7 | for<'a> Fn(&'a str)      | Ko'p marta chaqiriluvchi                             | Вызывается многократно                               |
// # |   8 | for<'a> FnMut(&'a str)   | Ko'p marta, ichki holat o'zgaradi                    | Многократно, изменяет внутреннее состояние           |
// # |   9 | for<'a> FnOnce(&'a str)  | Bir marta chaqiriluvchi                              | Вызывается один раз                                  |
// #================================================================================================================================================#
// # |                                       STRUCTDA HRTB                                                                                          |
// #================================================================================================================================================#
// # |  10 | struct S<F: for<'a> Fn>  | Struct field sifatida HRTB closure                   | HRTB замыкание как поле структуры                    |
// # |  11 | impl<F: for<'a> Fn>      | Impl blokda HRTB bound                               | HRTB ограничение в блоке impl                        |
// #================================================================================================================================================#
// # |                                       REAL HAYOT                                                                                             |
// #================================================================================================================================================#
// # |  12 | Validator zanjiri        | Box<dyn for<'a> Fn> bilan validatsiya                | Валидация с Box<dyn for<'a> Fn>                      |
// # |  13 | Matn pipeline            | Ketma-ket matn o'zgartirish                          | Последовательное преобразование текста               |
// # |  14 | Generic sort             | for<'a> Fn bilan tartiblash                          | Сортировка с for<'a> Fn                              |
// # |  15 | Callback systems         | Turli lifetime closurelarni qabul qilish             | Принятие замыканий разных lifetime                   |
// #================================================================================================================================================#