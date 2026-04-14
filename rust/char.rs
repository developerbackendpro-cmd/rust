// #================================================================================================================================================#
// #                                                                        CHAR                                                                    #
// #                                    CHAR — UNICODE BELGI. 4 BAYT. BITTA UNICODE SCALAR VALUE NI IFODALAYDI.                                     #
// #                                    CHAR — СИМВОЛ UNICODE. 4 БАЙТА. ПРЕДСТАВЛЯЕТ ОДНО ЗНАЧЕНИЕ UNICODE SCALAR.                                  #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

fn main() {

    // char — bitta unicode belgi, bitta qo'shtirnoq
    // один символ unicode, одинарные кавычки
    let kichik_harf: char = 'a';
    let katta_harf: char = 'Z';
    println!("{} {}", kichik_harf, katta_harf);
    // a Z

    // char — unicode belgilar
    // символы unicode
    let emoji: char = '😊';
    let arab_harf: char = 'ع';
    let rus_harf: char = 'ж';
    println!("{} {} {}", emoji, arab_harf, rus_harf);
    // 😊 ع ж

    // char — escape sequences
    // управляющие последовательности
    let yangi_qator: char = '\n';
    let tab: char = '\t';
    let teskari_chiziq: char = '\\';
    let bitta_qavs: char = '\'';
    let nol: char = '\0';
    println!("yangi_qator: {:?}", yangi_qator);
    println!("tab: {:?}", tab);
    println!("teskari_chiziq: {:?}", teskari_chiziq);
    // yangi_qator: '\n'
    // tab: '\t'
    // teskari_chiziq: '\\'

    // char — unicode escape
    // экранирование unicode
    let yurak: char = '\u{2764}';
    let qisqichbaqa: char = '\u{1F980}';
    println!("{} {}", yurak, qisqichbaqa);
    // ❤ 🦀

    // size_of::<char>() == 4 bayt
    // занимает 4 байта
    let char_hajmi = std::mem::size_of::<char>();
    println!("{}", char_hajmi);
    // 4

    // .is_alphabetic() — harfmi?
    // является ли буквой?
    let kichik_a: char = 'a';
    let bir_raqam: char = '1';
    let rus_zh: char = 'ж';
    println!("{}", kichik_a.is_alphabetic());
    println!("{}", bir_raqam.is_alphabetic());
    println!("{}", rus_zh.is_alphabetic());
    // true
    // false
    // true

    // .is_alphanumeric() — harf yoki raqammi?
    // буква или цифра?
    let harf_a: char = 'a';
    let raqam_5: char = '5';
    let undov: char = '!';
    println!("{}", harf_a.is_alphanumeric());
    println!("{}", raqam_5.is_alphanumeric());
    println!("{}", undov.is_alphanumeric());
    // true
    // true
    // false

    // .is_numeric() — raqammi? (unicode raqamlar)
    // цифра? (unicode цифры)
    let oddiy_5: char = '5';
    let kvadrat_2: char = '²';
    let harf_b: char = 'a';
    println!("{}", oddiy_5.is_numeric());
    println!("{}", kvadrat_2.is_numeric());
    println!("{}", harf_b.is_numeric());
    // true
    // true
    // false

    // .is_ascii_digit() — faqat 0-9 raqammi?
    // только цифры 0-9?
    let ascii_5: char = '5';
    let unicode_kvadrat: char = '²';
    println!("{}", ascii_5.is_ascii_digit());
    println!("{}", unicode_kvadrat.is_ascii_digit());
    // true
    // false

    // .is_ascii() — ASCII belgisimi? (0-127)
    // является ли ASCII символом?
    let lotin_a: char = 'a';
    let tabassum: char = '😊';
    println!("{}", lotin_a.is_ascii());
    println!("{}", tabassum.is_ascii());
    // true
    // false

    // .is_uppercase() — katta harfmi?
    // заглавная буква?
    let katta_a: char = 'A';
    let kichik_a2: char = 'a';
    println!("{}", katta_a.is_uppercase());
    println!("{}", kichik_a2.is_uppercase());
    // true
    // false

    // .is_lowercase() — kichik harfmi?
    // строчная буква?
    let kichik_a3: char = 'a';
    let katta_a2: char = 'A';
    println!("{}", kichik_a3.is_lowercase());
    println!("{}", katta_a2.is_lowercase());
    // true
    // false

    // .is_whitespace() — bo'sh joy belgisimi?
    // пробельный символ?
    let probel: char = ' ';
    let tab2: char = '\t';
    let yangi_qator2: char = '\n';
    let harf_c: char = 'a';
    println!("{}", probel.is_whitespace());
    println!("{}", tab2.is_whitespace());
    println!("{}", yangi_qator2.is_whitespace());
    println!("{}", harf_c.is_whitespace());
    // true
    // true
    // true
    // false

    // .is_ascii_punctuation() — tinish belgisimi?
    // знак препинания?
    let undov2: char = '!';
    let nuqta: char = '.';
    let harf_d: char = 'a';
    println!("{}", undov2.is_ascii_punctuation());
    println!("{}", nuqta.is_ascii_punctuation());
    println!("{}", harf_d.is_ascii_punctuation());
    // true
    // true
    // false

    // .is_control() — boshqaruv belgisimi?
    // управляющий символ?
    let yangi_qator3: char = '\n';
    let tab3: char = '\t';
    let harf_e: char = 'a';
    println!("{}", yangi_qator3.is_control());
    println!("{}", tab3.is_control());
    println!("{}", harf_e.is_control());
    // true
    // true
    // false

    // .is_digit(radix) — berilgan sanoq sistemasida raqammi?
    // цифра ли в данной системе счисления?
    let onlik_5: char = '5';
    let hex_f: char = 'F';
    let noto_g_ri: char = 'z';
    println!("{}", onlik_5.is_digit(10));
    println!("{}", hex_f.is_digit(16));
    println!("{}", noto_g_ri.is_digit(10));
    // true
    // true
    // false

    // .to_uppercase() — katta harfga o'tkazish (iterator qaytaradi)
    // преобразование в заглавную (возвращает итератор)
    let kichik_c: char = 'a';
    for ch in kichik_c.to_uppercase() {
        println!("{}", ch);
    }
    // A

    // .to_lowercase() — kichik harfga o'tkazish (iterator qaytaradi)
    // преобразование в строчную (возвращает итератор)
    let katta_c: char = 'A';
    for ch in katta_c.to_lowercase() {
        println!("{}", ch);
    }
    // a

    // .to_ascii_uppercase() — ASCII katta harf (bitta char qaytaradi)
    // ASCII заглавная буква (возвращает один char)
    let ascii_kichik: char = 'a';
    let ascii_katta: char = ascii_kichik.to_ascii_uppercase();
    println!("{}", ascii_katta);
    // A

    // .to_ascii_lowercase() — ASCII kichik harf (bitta char qaytaradi)
    // ASCII строчная буква (возвращает один char)
    let ascii_katta2: char = 'A';
    let ascii_kichik2: char = ascii_katta2.to_ascii_lowercase();
    println!("{}", ascii_kichik2);
    // a

    // .to_digit(radix) — raqamga o'tkazish
    // преобразование в цифру
    let char_5: char = '5';
    let char_f: char = 'F';
    let char_1: char = '1';
    let char_z: char = 'z';
    let onlik: Option<u32> = char_5.to_digit(10);
    let hex: Option<u32> = char_f.to_digit(16);
    let ikkilik: Option<u32> = char_1.to_digit(2);
    let noto_g_ri2: Option<u32> = char_z.to_digit(10);
    println!("{:?}", onlik);
    println!("{:?}", hex);
    println!("{:?}", ikkilik);
    println!("{:?}", noto_g_ri2);
    // Some(5)
    // Some(15)
    // Some(1)
    // None

    // .to_string() — String ga o'tkazish
    // преобразование в String
    let rust_r: char = 'R';
    let rust_string: String = rust_r.to_string();
    println!("{}", rust_string);
    // R

    // char → u32 (unicode kod nuqtasi)
    // char → u32 (кодовая точка unicode)
    let katta_a3: char = 'A';
    let kod_nuqtasi: u32 = katta_a3 as u32;
    println!("{}", kod_nuqtasi);
    // 65

    // char → u8 (faqat ASCII uchun)
    // char → u8 (только для ASCII)
    let katta_a4: char = 'A';
    let ascii_kod: u8 = katta_a4 as u8;
    println!("{}", ascii_kod);
    // 65

    // u32 → char (from_u32) — Option qaytaradi
    // u32 → char — возвращает Option
    let char_65: Option<char> = char::from_u32(65);
    println!("{:?}", char_65);
    // Some('A')

    // u32 → char (xavfli, unsafe)
    // u32 → char (небезопасно)
    let char_xavfli: char = unsafe { char::from_u32_unchecked(65) };
    println!("{}", char_xavfli);
    // A

    // u8 → char (from)
    // u8 → char
    let char_from_u8: char = char::from(65u8);
    println!("{}", char_from_u8);
    // A

    // char → String (String::from)
    // char → String через String::from
    let rust_r2: char = 'R';
    let string_from_char: String = String::from(rust_r2);
    println!("{}", string_from_char);
    // R

    // .len_utf8() — UTF-8 da necha bayt egallaydi
    // сколько байт занимает в UTF-8
    let lotin: char = 'a';
    let kirill: char = 'ж';
    let emoji2: char = '😊';
    println!("{}", lotin.len_utf8());
    println!("{}", kirill.len_utf8());
    println!("{}", emoji2.len_utf8());
    // 1
    // 2
    // 4

    // .len_utf16() — UTF-16 da necha birlik egallaydi
    // сколько единиц занимает в UTF-16
    let lotin2: char = 'a';
    let emoji3: char = '😊';
    println!("{}", lotin2.len_utf16());
    println!("{}", emoji3.len_utf16());
    // 1
    // 2

    // .encode_utf8() — UTF-8 baytlariga aylantirish
    // преобразование в байты UTF-8
    let lotin3: char = 'a';
    let mut buf = [0u8; 4];
    let utf8_str: &str = lotin3.encode_utf8(&mut buf);
    println!("{}", utf8_str);
    // a

    // String ni char larga ajratish
    // разбиение String на символы
    let so_z: &str = "salom";
    for c in so_z.chars() {
        print!("{} ", c);
    }
    println!();
    // s a l o m

    // faqat harflarni olish
    // получить только буквы
    let aralash: &str = "salom123!";
    let faqat_harflar: String = aralash.chars().filter(|c| c.is_alphabetic()).collect();
    println!("{}", faqat_harflar);
    // salom

    // faqat raqamlarni olish
    // получить только цифры
    let telefon: &str = "tel: +998-90-123-45-67";
    let faqat_raqamlar: String = telefon.chars().filter(|c| c.is_ascii_digit()).collect();
    println!("{}", faqat_raqamlar);
    // 998901234567

    // katta harfga o'tkazish
    // преобразование в верхний регистр
    let kichik_gap: &str = "salom dunyo";
    let katta_gap: String = kichik_gap.chars().map(|c| c.to_ascii_uppercase()).collect();
    println!("{}", katta_gap);
    // SALOM DUNYO

    // birinchi harfni katta qilish
    // первую букву сделать заглавной
    let gap: &str = "salom";
    let mut gap_chars = gap.chars();
    let katta_bosh: String = match gap_chars.next() {
        None    => String::new(),
        Some(c) => c.to_uppercase().to_string() + gap_chars.as_str(),
    };
    println!("{}", katta_bosh);
    // Salom

    // palindrommi?
    // является ли палиндромом?
    let palindrom_so_z: &str = "aba";
    let palindrom_chars: Vec<char> = palindrom_so_z.chars().collect();
    let teskari: Vec<char> = palindrom_chars.iter().rev().cloned().collect();
    let palindrommi: bool = palindrom_chars == teskari;
    println!("{}", palindrommi);
    // true

    // char soni (bayt emas, unicode soni)
    // количество символов (не байт, а unicode)
    let unicode_gap: &str = "salom 😊";
    let char_soni: usize = unicode_gap.chars().count();
    println!("{}", char_soni);
    // 7

    // char — match bilan range
    // char с match по диапазону
    let tekshirilayotgan: char = 'A';
    let char_turi: &str = match tekshirilayotgan {
        'a'..='z' => "kichik harf",
        'A'..='Z' => "katta harf",
        '0'..='9' => "raqam",
        _         => "boshqa",
    };
    println!("{}", char_turi);
    // katta harf
}

// #================================================================================================================================================#
// # |  №  | Metod                    | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                        TEKSHIRISH METODLARI                                                                                  |
// #================================================================================================================================================#
// # |   1 | .is_alphabetic()         | Harfmi? (barcha unicode harflar)                     | Буква? (все unicode буквы)                           |
// # |   2 | .is_alphanumeric()       | Harf yoki raqammi?                                   | Буква или цифра?                                     |
// # |   3 | .is_numeric()            | Unicode raqammi?                                     | Unicode цифра?                                       |
// # |   4 | .is_ascii_digit()        | Faqat 0-9 raqammi?                                   | Только цифры 0-9?                                    |
// # |   5 | .is_ascii()              | ASCII belgisimi? (0-127)                             | ASCII символ? (0-127)                                |
// # |   6 | .is_uppercase()          | Katta harfmi?                                        | Заглавная буква?                                     |
// # |   7 | .is_lowercase()          | Kichik harfmi?                                       | Строчная буква?                                      |
// # |   8 | .is_whitespace()         | Bo'sh joy belgisimi?                                 | Пробельный символ?                                   |
// # |   9 | .is_ascii_punctuation()  | Tinish belgisimi?                                    | Знак препинания?                                     |
// # |  10 | .is_control()            | Boshqaruv belgisimi?                                 | Управляющий символ?                                  |
// # |  11 | .is_digit(radix)         | Sanoq sistemasida raqammi?                           | Цифра в системе счисления?                           |
// #================================================================================================================================================#
// # |                                       O'ZGARTIRISH METODLARI                                                                                 |
// #================================================================================================================================================#
// # |  12 | .to_uppercase()          | Katta harfga (iterator qaytaradi)                    | В заглавную (возвращает итератор)                    |
// # |  13 | .to_lowercase()          | Kichik harfga (iterator qaytaradi)                   | В строчную (возвращает итератор)                     |
// # |  14 | .to_ascii_uppercase()    | ASCII katta harf (char qaytaradi)                    | ASCII заглавная (возвращает char)                    |
// # |  15 | .to_ascii_lowercase()    | ASCII kichik harf (char qaytaradi)                   | ASCII строчная (возвращает char)                     |
// # |  16 | .to_digit(radix)         | Raqamga o'tkazish (Option<u32>)                      | Преобразование в цифру (Option<u32>)                 |
// # |  17 | .to_string()             | String ga o'tkazish                                  | Преобразование в String                              |
// #================================================================================================================================================#
// # |                                         TUR O'ZGARTIRISH                                                                                     |
// #================================================================================================================================================#
// # |  18 | c as u32                 | Unicode kod nuqtasi                                  | Кодовая точка unicode                                |
// # |  19 | c as u8                  | ASCII qiymati (faqat ASCII uchun)                    | ASCII значение (только для ASCII)                    |
// # |  20 | char::from_u32(n)        | u32 dan char (Option qaytaradi)                      | char из u32 (возвращает Option)                      |
// # |  21 | char::from_u32_unchecked | u32 dan char (unsafe, xavfli)                        | char из u32 (unsafe, опасно)                         |
// # |  22 | char::from(n: u8)        | u8 dan char                                          | char из u8                                           |
// # |  23 | String::from(c)          | char dan String                                      | String из char                                       |
// #================================================================================================================================================#
// # |                                        UTF-8 / UNICODE METODLARI                                                                             |
// #================================================================================================================================================#
// # |  24 | .len_utf8()              | UTF-8 da necha bayt                                  | Сколько байт в UTF-8                                 |
// # |  25 | .len_utf16()             | UTF-16 da necha birlik                               | Сколько единиц в UTF-16                              |
// # |  26 | .encode_utf8(&mut buf)   | UTF-8 baytlariga aylantirish                         | Преобразование в байты UTF-8                         |
// #================================================================================================================================================#
// # |                                         REAL HAYOT                                                                                           |
// #================================================================================================================================================#
// # |  27 | s.chars()                | String ni char larga ajratish                        | Разбиение String на символы                          |
// # |  28 | .filter(|c| c.is_*())    | Shartga mos charlarni olish                          | Получение символов по условию                        |
// # |  29 | .map(|c| c.to_*())       | Har bir charni o'zgartirish                          | Преобразование каждого символа                       |
// # |  30 | 'a'..='z' (range match)  | Char oraliqda pattern matching                       | Pattern matching по диапазону символов               |
// # |  31 | s.chars().count()        | Unicode char soni (bayt emas)                        | Количество unicode символов (не байт)                |
// # |  32 | size_of::<char>() == 4   | Xotirada 4 bayt egallaydi                            | Занимает 4 байта в памяти                            |
// #================================================================================================================================================#