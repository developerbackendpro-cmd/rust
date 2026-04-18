// #================================================================================================================================================#
// #                                                                    ATTRIBUTES                                                                  #
// #                        ATTRIBUTES — RUST ELEMENTLARIGA META-MA'LUMOT QO'SHISH. #[...] SINTAKSISI. KOMPILYATOR KO'RSATMALARI.                   #
// #                        ATTRIBUTES — ДОБАВЛЕНИЕ МЕТАДАННЫХ К ЭЛЕМЕНТАМ RUST. СИНТАКСИС #[...]. ИНСТРУКЦИИ ДЛЯ КОМПИЛЯТОРА.                      #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// Attribute turlari:
// Виды атрибутов:
//
//   #[attr]        — tashqi attribute (element oldida)
//   #[attr]        — внешний атрибут (перед элементом)
//   #![attr]       — ichki attribute (modul/crate ichida)
//   #![attr]       — внутренний атрибут (внутри модуля/крейта)
//
//   Qayerga qo'yiladi:
//   Куда применяется:
//     struct, enum, fn, mod, use, trait, impl, field, variant

// #[derive(...)] — eng ko'p ishlatiladigan attribute
// #[derive(...)] — наиболее часто используемый атрибут
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Talaba {
    ism: String,
    yosh: u8,
    baho: u32,
}

#[derive(Debug, Clone, PartialEq)]
enum Holat {
    Faol,
    Nofaol,
    Kutmoqda(String),
}

fn derive_misollari() {

    let t1: Talaba = Talaba { ism: "Dilshod".to_string(), yosh: 22, baho: 90 };
    let t2: Talaba = t1.clone();

    // Debug — {:?} bilan chiqarish
    println!("{:?}", t1);
    // Talaba { ism: "Dilshod", yosh: 22, baho: 90 }

    // PartialEq — == operatori
    println!("{}", t1 == t2);  // true

    // Hash — HashMap kalit sifatida
    use std::collections::HashMap;
    let mut xarita: HashMap<Talaba, &str> = HashMap::new();
    xarita.insert(t1, "a'lo");
    println!("{:?}", xarita.values().next());
    // Some("a'lo")

    // PartialOrd + Ord — tartiblash
    let mut talabalar: Vec<Talaba> = vec![
        Talaba { ism: "Vali".to_string(), yosh: 20, baho: 85 },
        Talaba { ism: "Ali".to_string(), yosh: 21, baho: 92 },
    ];
    talabalar.sort();
    println!("{}", talabalar[0].ism);
    // Ali (leksikografik tartib)
}

// #[allow(...)] — ogohlantirishni o'chirish
// #[allow(...)] — отключение предупреждений
#[allow(dead_code)]
fn ishlatilmagan_funksiya() {
    println!("Bu funksiya chaqirilmaydi");
}

// #[warn(...)] — ogohlantirishni yoqish
// #[warn(...)] — включение предупреждений

// #[deny(...)] — ogohlantirishni xatoga aylantirish
// #[deny(...)] — превращение предупреждения в ошибку
#[deny(unused_variables)]
fn barcha_o_zgaruvchilar_ishlatilishi_kerak() {
    let x: i32 = 42; // x ishlatilishi kerak
    println!("{}", x);
}

// #[deprecated] — eskirgan deb belgilash
// #[deprecated] — пометить как устаревшее
#[deprecated(since = "2.0.0", note = "yangi_funksiya() ishlatilsin")]
fn eski_funksiya() -> i32 { 42 }

fn yangi_funksiya() -> i32 { 42 }

// #[cfg(...)] — shart bo'yicha kompilyatsiya
// #[cfg(...)] — условная компиляция
#[cfg(debug_assertions)]
fn debug_funksiya() {
    println!("Debug rejim");
}

#[cfg(not(debug_assertions))]
fn debug_funksiya() {
    println!("Release rejim");
}

#[cfg(target_os = "linux")]
fn linux_funksiya() {
    println!("Linux tizim");
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn unix_funksiya() {
    println!("Unix tizim");
}

// #[cfg_attr] — shartli attribute
// #[cfg_attr] — условный атрибут
#[cfg_attr(debug_assertions, derive(Debug))]
struct DebugFaqat {
    qiymat: i32,
}

// #[repr(C)] — C tartibida xotira
// #[repr(C)] — расположение памяти в порядке C
#[repr(C)]
struct CStruct {
    a: u8,
    b: u32,
    c: u8,
}

// #[repr(packed)] — to'ldirish baytlarsiz
// #[repr(packed)] — без байтов заполнения
#[repr(packed)]
struct PackedStruct {
    a: u8,
    b: u32,
    c: u8,
}

// #[repr(align(N))] — N bayt hizalash
// #[repr(align(N))] — выравнивание N байт
#[repr(align(16))]
struct AlignedStruct {
    qiymat: i32,
}

// #[repr(u8)] — enum diskriminanti u8 da
// #[repr(u8)] — дискриминант enum как u8
#[repr(u8)]
enum KichikEnum {
    A = 0,
    B = 1,
    C = 255,
}

// #[repr(transparent)] — wrapper uchun
// #[repr(transparent)] — для обёрток
#[repr(transparent)]
struct Wrapper(i32);

fn repr_misollari() {

    println!("CStruct:      {} bayt", std::mem::size_of::<CStruct>());
    println!("PackedStruct: {} bayt", std::mem::size_of::<PackedStruct>());
    println!("AlignedStruct:{} bayt", std::mem::size_of::<AlignedStruct>());
    println!("i32:          {} bayt", std::mem::size_of::<i32>());
    println!("Wrapper:      {} bayt", std::mem::size_of::<Wrapper>());
    // CStruct:      12 bayt (padding bilan)
    // PackedStruct: 6 bayt (padding yo'q)
    // AlignedStruct: 16 bayt (16 bayt hizalangan)
    // i32:          4 bayt
    // Wrapper:      4 bayt (transparent — i32 bilan bir xil)

    // KichikEnum diskriminanti
    let a: u8 = KichikEnum::A as u8;
    let c: u8 = KichikEnum::C as u8;
    println!("A={}, C={}", a, c);
    // A=0, C=255
}

// #[inline] — inlining maslahat
// #[inline] — рекомендация для инлайнинга
#[inline]
fn tez_qo_shish(a: i32, b: i32) -> i32 { a + b }

// #[inline(always)] — har doim inline
// #[inline(always)] — всегда инлайн
#[inline(always)]
fn har_doim_inline(x: i32) -> i32 { x * 2 }

// #[inline(never)] — hech qachon inline emas
// #[inline(never)] — никогда не инлайн
#[inline(never)]
fn hech_qachon_inline(x: i32) -> i32 { x + 1 }

// #[cold] — kamdan-kam chaqiriladigan
// #[cold] — редко вызываемая функция
#[cold]
fn xato_holati(xabar: &str) -> ! {
    panic!("{}", xabar);
}

// #[must_use] — natija ishlatilishi shart
// #[must_use] — результат должен быть использован
#[must_use]
fn muhim_hisoblash(x: i32) -> i32 { x * x + 1 }

// #[must_use] — struct uchun
// #[must_use] — для struct
#[must_use = "Bu natija e'tiborsiz qoldirilmasligi kerak"]
struct MuhimNatija(i32);

// #[test] — test funksiyasi
// #[test] — тестовая функция
// #[cfg(test)] — faqat test vaqtida kompile bo'ladi
// #[cfg(test)] — компилируется только при тестировании

#[cfg(test)]
mod testlar {
    use super::*;

    // #[test] — oddiy test
    // #[test] — обычный тест
    #[test]
    fn qo_shish_testi() {
        assert_eq!(tez_qo_shish(2, 3), 5);
        assert_eq!(tez_qo_shish(-1, 1), 0);
    }

    // #[should_panic] — panic bo'lishi kerak
    // #[should_panic] — должен паниковать
    #[test]
    #[should_panic(expected = "nolga bo'lish")]
    fn nolga_bolish_testi() {
        let x: i32 = 10;
        let y: i32 = 0;
        if y == 0 { panic!("nolga bo'lish xatosi!") }
        let _ = x / y;
    }

    // #[ignore] — o'chirilgan test
    // #[ignore] — отключённый тест
    #[test]
    #[ignore = "hali tayyor emas"]
    fn kelajakdagi_test() {
        todo!("Keyinroq yoziladi");
    }
}

// #[non_exhaustive] — yangi variant qo'shilishi mumkin
// #[non_exhaustive] — могут быть добавлены новые варианты
#[non_exhaustive]
pub enum ApiXato {
    TarmoqXato,
    AuthXato,
    ServerXato(String),
}

// #[doc = "..."] — dokumentatsiya
// #[doc = "..."] — документация
/// Bu funksiya ikkita sonni qo'shadi
/// Эта функция складывает два числа
///
/// # Misol / Пример
/// ```
/// let natija = qo_shish_doc(2, 3);
/// assert_eq!(natija, 5);
/// ```
pub fn qo_shish_doc(a: i32, b: i32) -> i32 { a + b }

// #[link_name] — tashqi funksiya nomi
// #[link_name] — имя внешней функции
// (FFI da ishlatiladi)
// (используется в FFI)

// #[unsafe(no_mangle)] — funksiya nomini o'zgartirmaslik
// #[unsafe(no_mangle)] — не изменять имя функции
#[unsafe(no_mangle)]
pub extern "C" fn rust_funksiya() -> i32 { 42 }

fn real_hayot_misollari() {

    // derive misollari
    derive_misollari();

    // repr misollari
    repr_misollari();

    // cfg misollari
    debug_funksiya();
    // Debug rejim (yoki Release)

    // deprecated
    #[allow(deprecated)]
    let _ = eski_funksiya();
    println!("Yangi funksiya: {}", yangi_funksiya());
    // Yangi funksiya: 42

    // inline funksiyalar
    println!("{}", tez_qo_shish(10, 20));
    println!("{}", har_doim_inline(5));
    // 30
    // 10

    // must_use
    let natija: i32 = muhim_hisoblash(5);
    println!("{}", natija);
    // 26

    // non_exhaustive — barcha variantlarni ko'rib chiqish
    let xato: ApiXato = ApiXato::TarmoqXato;
    let xabar: &str = match xato {
        ApiXato::TarmoqXato   => "Tarmoq xato",
        ApiXato::AuthXato     => "Auth xato",
        ApiXato::ServerXato(ref s) => "Server xato",
        // _ kerak — #[non_exhaustive] sababli yangi variant bo'lishi mumkin
        _ => "Noma'lum xato",
    };
    println!("{}", xabar);
    // Tarmoq xato

    // doc funksiya
    println!("{}", qo_shish_doc(15, 27));
    // 42
}

fn main() {
    println!("=== ATTRIBUTES ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Attribute                 | Tavsif (UZ)                                  | Описание (RU)                                               |
// #================================================================================================================================================#
// # |   1 | #[derive(...)]            | Trait avtomatik implement                     | Автоматическая реализация трейта                           |
// # |   2 | #[allow(lint)]            | Lint ogohlantirishini o'chirish               | Отключение lint предупреждения                             |
// # |   3 | #[warn(lint)]             | Lint ogohlantirishini yoqish                  | Включение lint предупреждения                              |
// # |   4 | #[deny(lint)]             | Lint ni xatoga aylantirish                    | Превращение lint в ошибку                                  |
// # |   5 | #[deprecated]             | Eskirgan deb belgilash                        | Пометить как устаревшее                                    |
// # |   6 | #[cfg(...)]               | Shartli kompilyatsiya                         | Условная компиляция                                        |
// # |   7 | #[cfg_attr(..., attr)]    | Shartli attribute                             | Условный атрибут                                           |
// # |   8 | #[repr(C)]                | C tartibida xotira                            | Расположение памяти в порядке C                            |
// # |   9 | #[repr(packed)]           | To'ldirish baytlarsiz                         | Без байтов заполнения                                      |
// # |  10 | #[repr(align(N))]         | N bayt hizalash                               | Выравнивание N байт                                        |
// # |  11 | #[repr(transparent)]      | Wrapper — ichki tur bilan bir xil             | Обёртка — идентична внутреннему типу                       |
// # |  12 | #[inline]                 | Inlining maslahat                             | Рекомендация для инлайнинга                                |
// # |  13 | #[inline(always)]         | Har doim inline                               | Всегда инлайн                                              |
// # |  14 | #[must_use]               | Natija ishlatilishi shart                     | Результат должен использоваться                            |
// # |  15 | #[test]                   | Test funksiyasi                               | Тестовая функция                                           |
// # |  16 | #[should_panic]           | Panic bo'lishi kerak                          | Должен паниковать                                          |
// # |  17 | #[ignore]                 | Testni o'chirish                              | Отключение теста                                           |
// # |  18 | #[non_exhaustive]         | Yangi variant qo'shilishi mumkin              | Могут быть добавлены новые варианты                        |
// # |  19 | #[unsafe(no_mangle)]      | Funksiya nomini saqlash (FFI)                 | Сохранить имя функции (FFI)                                |
// # |  20 | /// doc comment           | Dokumentatsiya                                | Документация                                               |
// #================================================================================================================================================#