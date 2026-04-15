// #================================================================================================================================================#
// #                                                          BUILT-IN MACROS                                                                       #
// #                                    BUILT-IN MACROS — RUST STANDART KUTUBXONASIDAGI TAYYOR MAKROLAR.                                            #
// #                                    BUILT-IN MACROS — ВСТРОЕННЫЕ МАКРОСЫ СТАНДАРТНОЙ БИБЛИОТЕКИ RUST.                                           #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

fn main() {

    // println! — stdout ga chiqarish + yangi qator
    // вывод в stdout + новая строка
    println!("salom dunyo!");
    println!("son: {}", 42);
    println!("ikki qiymat: {} va {}", 10, 20);
    // salom dunyo!
    // son: 42
    // ikki qiymat: 10 va 20

    // print! — stdout ga chiqarish (yangi qator yo'q)
    // вывод в stdout (без новой строки)
    print!("birinchi ");
    print!("ikkinchi ");
    println!();
    // birinchi ikkinchi

    // eprintln! — stderr ga chiqarish + yangi qator
    // вывод в stderr + новая строка
    eprintln!("xato: fayl topilmadi");
    eprintln!("xato kodi: {}", 404);
    // xato: fayl topilmadi  (stderr ga)
    // xato kodi: 404

    // eprint! — stderr ga chiqarish (yangi qator yo'q)
    // вывод в stderr (без новой строки)
    eprint!("ogohlantirish: ");
    eprintln!("xotira kam!");
    // ogohlantirish: xotira kam!

    // dbg! — qiymat va joy ni chiqaradi, qiymatni qaytaradi
    // выводит значение и место, возвращает значение
    let son: i32 = 42;
    let ikki_baravar: i32 = dbg!(son * 2);
    println!("{}", ikki_baravar);
    // [src/main.rs:XX] son * 2 = 84
    // 84

    // dbg! — zanjirli ishlatish
    // цепочка dbg!
    let natija: i32 = dbg!(dbg!(2 + 3) * dbg!(4 + 5));
    println!("{}", natija);
    // [src/main.rs:XX] 2 + 3 = 5
    // [src/main.rs:XX] 4 + 5 = 9
    // [src/main.rs:XX] dbg!(2 + 3) * dbg!(4 + 5) = 45
    // 45

    // format! — String yaratish
    // создание String
    let ism: &str = "Dilshod";
    let yosh: u32 = 25;
    let gap: String = format!("Mening ismim {} va yoshim {}", ism, yosh);
    println!("{}", gap);
    // Mening ismim Dilshod va yoshim 25

    // format! — turli formatlar
    // различные форматы
    let ikkilik: String = format!("{:b}", 42);
    let hex: String = format!("{:x}", 255);
    let sakkizlik: String = format!("{:o}", 8);
    let ilmiy: String = format!("{:e}", 1000000.0f64);
    println!("{}", ikkilik);
    println!("{}", hex);
    println!("{}", sakkizlik);
    println!("{}", ilmiy);
    // 101010
    // ff
    // 10
    // 1e6

    // format! — to'ldirish va hizalash
    // заполнение и выравнивание
    let chap: String = format!("{:<10}", "chap");
    let ong: String = format!("{:>10}", "ong");
    let markaz: String = format!("{:^10}", "markaz");
    let to_ldirilgan: String = format!("{:0>5}", 42);
    println!("|{}|", chap);
    println!("|{}|", ong);
    println!("|{}|", markaz);
    println!("{}", to_ldirilgan);
    // |chap      |
    // |       ong|
    // |  markaz  |
    // 00042

    // assert! — shart bajarilmasa panic
    // паника если условие не выполнено
    let x: i32 = 5;
    assert!(x > 0);
    assert!(x < 10, "x 10 dan kichik bo'lishi kerak, lekin x = {}", x);
    // (panic yo'q, davom etadi)

    // assert_eq! — teng bo'lmasa panic
    // паника если не равны
    let a: i32 = 2 + 2;
    let b: i32 = 4;
    assert_eq!(a, b);
    assert_eq!(a, b, "a va b teng emas: {} != {}", a, b);
    // (panic yo'q)

    // assert_ne! — teng bo'lsa panic
    // паника если равны
    let x2: i32 = 5;
    let y2: i32 = 10;
    assert_ne!(x2, y2);
    assert_ne!(x2, y2, "x va y teng bo'lmasligi kerak");
    // (panic yo'q)

    // debug_assert! — faqat debug modeda ishlaydi
    // работает только в режиме debug
    let son2: i32 = 42;
    debug_assert!(son2 > 0, "son musbat bo'lishi kerak");
    // (release modeda tekshirilmaydi — zero cost!)

    // debug_assert_eq! — debug modeda tenglikni tekshirish
    // проверка равенства только в debug режиме
    let kutilgan: i32 = 42;
    let haqiqiy: i32 = 42;
    debug_assert_eq!(kutilgan, haqiqiy);
    debug_assert_eq!(kutilgan, haqiqiy, "qiymatlar teng emas: {} != {}", kutilgan, haqiqiy);
    // (release modeda tekshirilmaydi — zero cost!)

    // debug_assert_ne! — debug modeda tengsizlikni tekshirish
    // проверка неравенства только в debug режиме
    let birinchi: i32 = 5;
    let ikkinchi: i32 = 10;
    debug_assert_ne!(birinchi, ikkinchi);
    debug_assert_ne!(birinchi, ikkinchi, "qiymatlar teng bo'lmasligi kerak");
    // (release modeda tekshirilmaydi — zero cost!)

    // panic! — dasturni to'xtatish
    // остановка программы
    // panic!("kutilmagan xato!");
    // panic!("xato: {} qiymat noto'g'ri", 42);

    // todo! — hali yozilmagan kod
    // код ещё не написан
    fn hisob() -> i32 {
        todo!("bu funksiya hali yozilmagan")
    }
    // hisob(); // panics: not yet implemented

    // unimplemented! — qo'llab-quvvatlanmaydi
    // не поддерживается
    fn eski_metod() {
        unimplemented!("bu metod olib tashlangan")
    }
    // eski_metod(); // panics

    // unreachable! — bu yerga kelmaslik kerak
    // сюда не должны доходить
    let kod: i32 = 2;
    let natija2: &str = match kod {
        1 => "bir",
        2 => "ikki",
        3 => "uch",
        _ => unreachable!("faqat 1-3 bo'lishi mumkin"),
    };
    println!("{}", natija2);
    // ikki

    // vec! — Vec yaratish
    // создание Vec
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v2: Vec<i32> = vec![0; 5];
    println!("{:?}", v1);
    println!("{:?}", v2);
    // [1, 2, 3, 4, 5]
    // [0, 0, 0, 0, 0]

    // compile_error! — kompilyatsiya vaqtida xato chiqarish
    // ошибка во время компиляции
    // #[cfg(not(target_os = "linux"))]
    // compile_error!("bu dastur faqat Linux da ishlaydi!");
    //
    // feature tekshirishda ishlatiladi:
    // используется для проверки фич:
    // #[cfg(not(feature = "required_feature"))]
    // compile_error!("required_feature yoqilishi kerak!");

    // env! — muhit o'zgaruvchisi (compile time)
    // переменная среды (время компиляции)
    let cargo_pkg: &str = env!("CARGO_PKG_NAME");
    let cargo_ver: &str = env!("CARGO_PKG_VERSION");
    println!("{}", cargo_pkg);
    println!("{}", cargo_ver);
    // hello_rust
    // 0.1.0

    // option_env! — mavjud bo'lmasligi mumkin
    // может отсутствовать
    let home: Option<&str> = option_env!("HOME");
    println!("{:?}", home);
    // Some("/home/user") yoki None

    // file! — joriy fayl nomi
    // имя текущего файла
    let fayl_nomi: &str = file!();
    println!("{}", fayl_nomi);
    // src/main.rs

    // line! — joriy qator raqami
    // номер текущей строки
    let qator: u32 = line!();
    println!("{}", qator);
    // (joriy qator raqami)

    // column! — joriy ustun raqami
    // номер текущего столбца
    let ustun: u32 = column!();
    println!("{}", ustun);
    // (joriy ustun raqami)

    // module_path! — joriy modul yo'li
    // путь к текущему модулю
    let modul: &str = module_path!();
    println!("{}", modul);
    // hello_rust

    // stringify! — kodni string ga aylantirish
    // преобразование кода в строку
    let ifoda: &str = stringify!(2 + 2 * 3);
    println!("{}", ifoda);
    // 2 + 2 * 3

    // concat! — string larni birlashtirish (compile time)
    // объединение строк (время компиляции)
    let birlashgan: &str = concat!("salom", " ", "dunyo", "!");
    println!("{}", birlashgan);
    // salom dunyo!

    // include_str! — faylni string sifatida yuklash (compile time)
    // загрузка файла как строки (время компиляции)
    // let matn: &str = include_str!("../README.md");
    // println!("{}", matn);

    // include_bytes! — faylni bayt sifatida yuklash
    // загрузка файла как байтов
    // let baytlar: &[u8] = include_bytes!("../README.md");
    // println!("{:?}", &baytlar[..10]);

    // include! — boshqa fayldan Rust kodni yuklash
    // загрузка Rust кода из другого файла
    // let qiymat = include!("data.rs");
    //
    // data.rs ichida:
    // содержимое data.rs:
    // vec![1, 2, 3, 4, 5]
    //
    // ishlatilishi:
    // использование:
    // let v: Vec<i32> = include!("data.rs");

    // write! — bufferga yozish (yangi qator yo'q)
    // запись в буфер (без новой строки)
    use std::fmt::Write;
    let mut buffer: String = String::new();
    write!(buffer, "salom {}", "dunyo").unwrap();
    println!("{}", buffer);
    // salom dunyo

    // writeln! — bufferga yozish + yangi qator
    // запись в буфер + новая строка
    let mut buffer2: String = String::new();
    writeln!(buffer2, "birinchi qator").unwrap();
    writeln!(buffer2, "ikkinchi qator").unwrap();
    print!("{}", buffer2);
    // birinchi qator
    // ikkinchi qator

    // matches! — pattern mos kelishini tekshirish
    // проверка совпадения паттерна
    let rang: &str = "qizil";
    let qizilmi: bool = matches!(rang, "qizil" | "to'q qizil");
    println!("{}", qizilmi);
    // true

    // cfg! — kompilyatsiya konfiguratsiyasi
    // конфигурация компиляции
    let debug_mode: bool = cfg!(debug_assertions);
    println!("{}", debug_mode);
    // true (debug modeda)

    // std::mem::size_of — tur hajmi
    // размер типа
    let i32_hajmi: usize = std::mem::size_of::<i32>();
    let i32_hizalanish: usize = std::mem::align_of::<i32>();
    println!("{}", i32_hajmi);
    println!("{}", i32_hizalanish);
    // 4
    // 4
}

// #================================================================================================================================================#
// # |  №  | Makro                    | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                        CHIQARISH MAKROLARI                                                                                   |
// #================================================================================================================================================#
// # |   1 | println!(...)            | stdout + yangi qator                                 | stdout + новая строка                                |
// # |   2 | print!(...)              | stdout, yangi qator yo'q                             | stdout, без новой строки                             |
// # |   3 | eprintln!(...)           | stderr + yangi qator                                 | stderr + новая строка                                |
// # |   4 | eprint!(...)             | stderr, yangi qator yo'q                             | stderr, без новой строки                             |
// # |   5 | dbg!(expr)               | Qiymat + joy ni chiqaradi, qiymatni qaytaradi        | Выводит значение + место, возвращает значение        |
// #================================================================================================================================================#
// # |                                        FORMAT MAKROLARI                                                                                      |
// #================================================================================================================================================#
// # |   6 | format!(...)             | String yaratish                                      | Создание String                                      |
// # |   7 | {:b} {:x} {:o} {:e}      | Ikkilik, hex, sakkizlik, ilmiy format                | Двоичный, hex, восьмеричный, научный формат          |
// # |   8 | {:<10} {:>10} {:^10}     | Chap, ong, markaz hizalash                           | Выравнивание влево, вправо, по центру                |
// # |   9 | {:0>5}                   | Nol bilan to'ldirish                                 | Заполнение нулями                                    |
// #================================================================================================================================================#
// # |                                        ASSERT MAKROLARI                                                                                      |
// #================================================================================================================================================#
// # |  10 | assert!(cond)            | Shart bajarilmasa panic                              | Паника если условие не выполнено                     |
// # |  11 | assert_eq!(a, b)         | Teng bo'lmasa panic                                  | Паника если не равны                                 |
// # |  12 | assert_ne!(a, b)         | Teng bo'lsa panic                                    | Паника если равны                                    |
// # |  13 | debug_assert!(cond)      | Faqat debug modeda (release = zero cost)             | Только в debug (release = zero cost)                 |
// # |  14 | debug_assert_eq!(a, b)   | Debug modeda tenglikni tekshirish                    | Проверка равенства только в debug                    |
// # |  15 | debug_assert_ne!(a, b)   | Debug modeda tengsizlikni tekshirish                 | Проверка неравенства только в debug                  |
// #================================================================================================================================================#
// # |                                        PANIC MAKROLARI                                                                                       |
// #================================================================================================================================================#
// # |  16 | panic!(...)              | Dasturni to'xtatish                                  | Остановка программы                                  |
// # |  17 | todo!(...)               | Hali yozilmagan kod                                  | Код ещё не написан                                   |
// # |  18 | unimplemented!(...)      | Qo'llab-quvvatlanmaydi                               | Не поддерживается                                    |
// # |  19 | unreachable!(...)        | Bu yerga kelmaslik kerak                             | Сюда не должны доходить                              |
// #================================================================================================================================================#
// # |                                     KOLLEKSIYA MAKROLARI                                                                                     |
// #================================================================================================================================================#
// # |  20 | vec![...]                | Vec yaratish                                         | Создание Vec                                         |
// # |  21 | vec![val; n]             | n ta bir xil qiymatli Vec                            | Vec из n одинаковых значений                         |
// #================================================================================================================================================#
// # |                                   KOMPILYATSIYA VAQTI MAKROLARI                                                                              |
// #================================================================================================================================================#
// # |  22 | compile_error!(...)      | Kompilyatsiya vaqtida xato chiqarish                 | Ошибка во время компиляции                           |
// # |  23 | env!(...)                | Muhit o'zgaruvchisi (compile time)                   | Переменная среды (время компиляции)                  |
// # |  24 | option_env!(...)         | Mavjud bo'lmasligi mumkin (Option)                   | Может отсутствовать (Option)                         |
// # |  25 | file!()                  | Joriy fayl nomi                                      | Имя текущего файла                                   |
// # |  26 | line!()                  | Joriy qator raqami                                   | Номер текущей строки                                 |
// # |  27 | column!()                | Joriy ustun raqami                                   | Номер текущего столбца                               |
// # |  28 | module_path!()           | Joriy modul yo'li                                    | Путь к текущему модулю                               |
// # |  29 | stringify!(expr)         | Kodni string ga aylantirish                          | Преобразование кода в строку                         |
// # |  30 | concat!(...)             | String larni compile time birlashtirish              | Объединение строк во время компиляции                |
// # |  31 | include_str!(path)       | Faylni string sifatida yuklash                       | Загрузка файла как строки                            |
// # |  32 | include_bytes!(path)     | Faylni bayt sifatida yuklash                         | Загрузка файла как байтов                            |
// # |  33 | include!(path)           | Boshqa fayldan Rust kodni yuklash                    | Загрузка Rust кода из другого файла                  |
// #================================================================================================================================================#
// # |                                       YOZISH MAKROLARI                                                                                       |
// #================================================================================================================================================#
// # |  34 | write!(buf, ...)         | Bufferga yozish                                      | Запись в буфер                                       |
// # |  35 | writeln!(buf, ...)       | Bufferga yozish + yangi qator                        | Запись в буфер + новая строка                        |
// #================================================================================================================================================#
// # |                                       BOSHQA MUHIM MAKROLAR                                                                                  |
// #================================================================================================================================================#
// # |  36 | matches!(val, pat)       | Pattern mos kelishini tekshirish                     | Проверка совпадения паттерна                         |
// # |  37 | cfg!(feature)            | Kompilyatsiya konfiguratsiyasi                       | Конфигурация компиляции                              |
// #================================================================================================================================================#