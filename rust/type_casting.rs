// #================================================================================================================================================#
// #                                                              TYPE CASTING  (as)                                                                #
// #                                    TYPE CASTING — BIR TURNI BOSHQASIGA AYLANTIRISH. as KALIT SO'ZI BILAN.                                      #
// #                                    TYPE CASTING — ПРЕОБРАЗОВАНИЕ ОДНОГО ТИПА В ДРУГОЙ. С КЛЮЧЕВЫМ СЛОВОМ as.                                   #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

fn main() {

    // i32 → u8 (kichraytirishda qirqiladi)
    // i32 → u8 (усечение при уменьшении)
    let katta: i32 = 300;
    let kichik: u8 = katta as u8;
    println!("{}", kichik);
    // 44  (300 % 256 = 44)

    // u8 → i32 (kattalashtirish, xavfsiz)
    // u8 → i32 (расширение, безопасно)
    let kichik2: u8 = 200;
    let katta2: i32 = kichik2 as i32;
    println!("{}", katta2);
    // 200

    // i32 → u32 (manfiy → katta musbat)
    // i32 → u32 (отрицательное → большое положительное)
    let manfiy: i32 = -1;
    let musbat: u32 = manfiy as u32;
    println!("{}", musbat);
    // 4294967295

    // i64 → i32 (qirqiladi)
    // i64 → i32 (усечение)
    let katta_son: i64 = 1_000_000_000_000;
    let kichik_son: i32 = katta_son as i32;
    println!("{}", kichik_son);
    // 727379968

    // u32 → i32 (katta qiymat manfiyga o'tishi mumkin)
    // u32 → i32 (большое значение может стать отрицательным)
    let katta_u32: u32 = 3_000_000_000;
    let i32_qiymat: i32 = katta_u32 as i32;
    println!("{}", i32_qiymat);
    // -1294967296

    // usize → u32
    // usize → u32
    let indeks: usize = 42;
    let indeks_u32: u32 = indeks as u32;
    println!("{}", indeks_u32);
    // 42

    // i32 → usize (indekslash uchun)
    // i32 → usize (для индексирования)
    let son: i32 = 10;
    let usize_son: usize = son as usize;
    println!("{}", usize_son);
    // 10

    // f64 → i32 (kasr qismi tashlanadi, round emas!)
    // f64 → i32 (дробная часть отбрасывается, не округляется!)
    let kasr: f64 = 3.99;
    let butun: i32 = kasr as i32;
    println!("{}", butun);
    // 3  (round emas, truncate!)

    // f64 → i32 (manfiy)
    // f64 → i32 (отрицательное)
    let manfiy_kasr: f64 = -3.99;
    let manfiy_butun: i32 = manfiy_kasr as i32;
    println!("{}", manfiy_butun);
    // -3

    // i32 → f64
    // i32 → f64
    let butun2: i32 = 42;
    let kasr2: f64 = butun2 as f64;
    println!("{}", kasr2);
    // 42.0

    // f32 → f64 (aniqlik oshadi)
    // f32 → f64 (точность увеличивается)
    let f32_son: f32 = 3.14;
    let f64_son: f64 = f32_son as f64;
    println!("{}", f64_son);
    // 3.140000104904175

    // f64 → f32 (aniqlik kamayadi)
    // f64 → f32 (точность уменьшается)
    let f64_son2: f64 = 3.141592653589793;
    let f32_son2: f32 = f64_son2 as f32;
    println!("{}", f32_son2);
    // 3.1415927

    // f64 → u8 (saturating — 0..=255 oralig'ida qoladi)
    // f64 → u8 (насыщение — остаётся в диапазоне 0..=255)
    let katta_f: f64 = 999.0;
    let u8_son: u8 = katta_f as u8;
    println!("{}", u8_son);
    // 255

    let manfiy_f: f64 = -50.0;
    let u8_son2: u8 = manfiy_f as u8;
    println!("{}", u8_son2);
    // 0

    // char → u32 (unicode kod nuqtasi)
    // char → u32 (кодовая точка unicode)
    let harf: char = 'A';
    let kod: u32 = harf as u32;
    println!("{}", kod);
    // 65

    // char → u8 (faqat ASCII)
    // char → u8 (только ASCII)
    let ascii_harf: char = 'A';
    let ascii_kod: u8 = ascii_harf as u8;
    println!("{}", ascii_kod);
    // 65

    // u8 → char (ASCII range)
    // u8 → char (диапазон ASCII)
    let ascii_son: u8 = 65;
    let ascii_char: char = ascii_son as char;
    println!("{}", ascii_char);
    // A

    // bool → i32
    // bool → i32
    let rost: bool = true;
    let yolg_on: bool = false;
    let rost_son: i32 = rost as i32;
    let yolg_on_son: i32 = yolg_on as i32;
    println!("{}", rost_son);
    println!("{}", yolg_on_son);
    // 1
    // 0

    // bool → u8
    // bool → u8
    let bayroq: bool = true;
    let bayroq_u8: u8 = bayroq as u8;
    println!("{}", bayroq_u8);
    // 1

    // raw pointer → usize (xotira manzili)
    // raw pointer → usize (адрес памяти)
    let x: i32 = 42;
    let ptr: *const i32 = &x;
    let manzil: usize = ptr as usize;
    println!("{:#x}", manzil);
    // 0x7ffd... (xotira manzili)

    // usize → raw pointer
    // usize → raw pointer
    let manzil2: usize = 0x12345678;
    let ptr2: *const i32 = manzil2 as *const i32;
    println!("{:?}", ptr2);
    // 0x12345678

    // From::from() — xavfsiz, yo'qolishsiz aylantirish
    // From::from() — безопасное преобразование без потерь
    let kichik_u8: u8 = 42;
    let katta_i32: i32 = i32::from(kichik_u8);
    println!("{}", katta_i32);
    // 42

    // TryFrom::try_from() — xavfsiz, xato qaytarishi mumkin
    // TryFrom::try_from() — безопасное, может вернуть ошибку
    use std::convert::TryFrom;
    let katta_i32_2: i32 = 1000;
    let u8_natija: Result<u8, _> = u8::try_from(katta_i32_2);
    println!("{:?}", u8_natija);
    // Err(TryFromIntError(()))

    let kichik_i32: i32 = 42;
    let u8_natija2: Result<u8, _> = u8::try_from(kichik_i32);
    println!("{:?}", u8_natija2);
    // Ok(42)

    // checked_add — overflow tekshirish
    // проверка переполнения
    let a: u8 = 200;
    let b: u8 = 100;
    let natija: Option<u8> = a.checked_add(b);
    println!("{:?}", natija);
    // None (overflow!)

    // saturating_add — to'yinuvchi qo'shish
    // насыщающее сложение
    let a2: u8 = 200;
    let b2: u8 = 100;
    let natija2: u8 = a2.saturating_add(b2);
    println!("{}", natija2);
    // 255 (maksimumda qoladi)

    // wrapping_add — aylanuvchi qo'shish
    // оборачивающее сложение
    let a3: u8 = 200;
    let b3: u8 = 100;
    let natija3: u8 = a3.wrapping_add(b3);
    println!("{}", natija3);
    // 44 (200 + 100 = 300, 300 % 256 = 44)
}

// #================================================================================================================================================#
// # |  №  | Casting                  | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                       INTEGER → INTEGER                                                                                      |
// #================================================================================================================================================#
// # |   1 | i32 as u8                | Kichraytirishda qirqiladi (% 256)                    | Усечение при уменьшении (% 256)                      |
// # |   2 | u8 as i32                | Kattalashtirish — xavfsiz                            | Расширение — безопасно                               |
// # |   3 | i32 as u32               | Manfiy → katta musbat (bit pattern saqlanadi)        | Отрицательное → большое положительное                |
// # |   4 | usize as u32             | Platforma o'lchamidan o'zgartirish                   | Преобразование из платформенного размера             |
// #================================================================================================================================================#
// # |                                    FLOAT ↔ INTEGER                                                                                           |
// #================================================================================================================================================#
// # |   5 | f64 as i32               | Kasr tashlanadi (truncate, round emas!)              | Дробь отбрасывается (truncate, не округление!)       |
// # |   6 | i32 as f64               | Butun → float, xavfsiz                               | Целое → float, безопасно                             |
// # |   7 | f32 as f64               | Aniqlik oshadi                                       | Точность увеличивается                               |
// # |   8 | f64 as f32               | Aniqlik kamayadi                                     | Точность уменьшается                                 |
// # |   9 | f64 as u8                | Saturating — 0..=255 da qoladi                       | Насыщение — остаётся в 0..=255                       |
// #================================================================================================================================================#
// # |                                    CHAR ↔ INTEGER                                                                                            |
// #================================================================================================================================================#
// # |  10 | char as u32              | Unicode kod nuqtasi                                  | Кодовая точка unicode                                |
// # |  11 | char as u8               | ASCII qiymati (faqat ASCII uchun)                    | ASCII значение (только для ASCII)                    |
// # |  12 | u8 as char               | ASCII koddan char                                    | char из ASCII кода                                   |
// #================================================================================================================================================#
// # |                                       BOOL → INTEGER                                                                                         |
// #================================================================================================================================================#
// # |  13 | true as i32 → 1          | true = 1, false = 0                                  | true = 1, false = 0                                  |
// # |  14 | true as u8 → 1           | u8 ga aylantirish                                    | Преобразование в u8                                  |
// #================================================================================================================================================#
// # |                                    POINTER ↔ INTEGER                                                                                         |
// #================================================================================================================================================#
// # |  15 | ptr as usize             | Xotira manzilini olish                               | Получение адреса памяти                              |
// # |  16 | usize as *const T        | Manzildan pointer yaratish                           | Создание указателя из адреса                         |
// #================================================================================================================================================#
// # |                                    XAVFSIZ ALTERNATIVALAR                                                                                    |
// #================================================================================================================================================#
// # |  17 | i32::from(u8)            | Yo'qolishsiz xavfsiz aylantirish                     | Безопасное преобразование без потерь                 |
// # |  18 | u8::try_from(i32)        | Xato qaytarishi mumkin (Result)                      | Может вернуть ошибку (Result)                        |
// # |  19 | checked_add()            | Overflow bo'lsa None                                 | None при переполнении                                |
// # |  20 | saturating_add()         | Maksimumda to'xtaydi                                 | Останавливается на максимуме                         |
// # |  21 | wrapping_add()           | Aylanib ketadi (overflow)                            | Оборачивается при переполнении                       |
// #================================================================================================================================================#