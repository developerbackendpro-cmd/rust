// #================================================================================================================================================#
// #                                                          FORMAT! CHUQUR                                                                        #
// #                     FORMAT! — SATR FORMATLASH MAKROSI. DISPLAY, DEBUG, PADDING, ALIGNMENT, PRECISION VA KO'PROQ.                               #
// #                     FORMAT! — МАКРОС ФОРМАТИРОВАНИЯ СТРОК. DISPLAY, DEBUG, PADDING, ALIGNMENT, PRECISION И БОЛЬШЕ.                             #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// format! oilasi:
// Семейство format!:
//   format!()   → String qaytaradi
//   print!()    → stdout ga, yangi satr yo'q
//   println!()  → stdout ga, yangi satr bor
//   eprint!()   → stderr ga, yangi satr yo'q
//   eprintln!() → stderr ga, yangi satr bor
//   write!()    → Writer ga yozish
//   writeln!()  → Writer ga yozish + yangi satr

fn asosiy_format_misollari() {

    // {} — Display trait (oddiy chiqarish)
    // {} — трейт Display (обычный вывод)
    let s: String = format!("{}", 42);
    let s2: String = format!("{}", "salom");
    let s3: String = format!("{}", 3.14);
    let s4: String = format!("{}", true);
    println!("{} {} {} {}", s, s2, s3, s4);
    // 42 salom 3.14 true

    // {:?} — Debug trait
    // {:?} — трейт Debug
    let v: Vec<i32> = vec![1, 2, 3];
    let t: (i32, &str) = (1, "salom");
    println!("{:?}", v);
    println!("{:?}", t);
    // [1, 2, 3]
    // (1, "salom")

    // {:#?} — Pretty Debug
    // {:#?} — красивый Debug
    #[derive(Debug)]
    struct Foydalanuvchi { ism: String, yosh: u32 }
    let f = Foydalanuvchi { ism: "Dilshod".to_string(), yosh: 22 };
    println!("{:#?}", f);
    // Foydalanuvchi {
    //     ism: "Dilshod",
    //     yosh: 22,
    // }

    // Bir nechta argument
    // Несколько аргументов
    let s5: String = format!("{} {} {}", "bir", "ikki", "uch");
    println!("{}", s5);
    // bir ikki uch

    // Pozitsion argument
    // Позиционный аргумент
    let s6: String = format!("{0} {1} {0}", "ping", "pong");
    println!("{}", s6);
    // ping pong ping

    // Nomlangan argument
    // Именованный аргумент
    let s7: String = format!("{ism} {yosh} yoshda", ism="Dilshod", yosh=22);
    println!("{}", s7);
    // Dilshod 22 yoshda

    // Escape qavs
    // Экранирование фигурных скобок
    println!("{{}} — bo'sh qavs");
    println!("{{{}}}", 42);
    // {} — bo'sh qavs
    // {42}
}

fn sonlar_format_misollari() {

    let n: i32 = 42;
    let f: f64 = 3.14159265;

    // Sonlar formatlash
    // Форматирование чисел
    println!("{:b}", n);    // ikkilik    // двоичный
    println!("{:o}", n);    // sakkizlik  // восьмеричный
    println!("{:x}", n);    // o'n oltilik (kichik)  // шестнадцатеричный (маленький)
    println!("{:X}", n);    // o'n oltilik (katta)   // шестнадцатеричный (большой)
    println!("{:e}", f);    // ilmiy     // научный
    println!("{:E}", f);    // ilmiy (E) // научный (E)
    // 101010
    // 52
    // 2a
    // 2A
    // 3.14159265e0
    // 3.14159265E0

    // Prefiks bilan
    // С префиксом
    println!("{:#b}", n);   // 0b101010
    println!("{:#o}", n);   // 0o52
    println!("{:#x}", n);   // 0x2a
    println!("{:#X}", n);   // 0x2A

    // Aniqlik (precision)
    // Точность
    println!("{:.2}", f);   // 3.14
    println!("{:.4}", f);   // 3.1416
    println!("{:.0}", f);   // 3
    println!("{:.10}", f);  // 3.1415926500

    // Kenglik (width)
    // Ширина
    println!("{:10}", n);   // "        42"
    println!("{:10}", "hi");// "hi        "
    println!("{:10.3}", f); // "     3.142"

    // Alignment
    // Выравнивание
    println!("{:<10}", n);  // chapga   // влево
    println!("{:>10}", n);  // o'ngga   // вправо
    println!("{:^10}", n);  // markazga // по центру
    // "42        "
    // "        42"
    // "    42    "

    // To'ldirish belgisi
    // Символ заполнения
    println!("{:0>10}", n); // "0000000042"
    println!("{:0<10}", n); // "4200000000"
    println!("{:*^10}", n); // "****42****"
    println!("{:->10}", n); // "--------42"
    println!("{:-<10}", n); // "42--------"

    // Ishorat (sign)
    // Знак
    println!("{:+}", 42);   // +42
    println!("{:+}", -42);  // -42

    // Kenglik va aniqlik birga
    // Ширина и точность вместе
    println!("{:10.3}", f);  // "     3.142"
    println!("{:<10.3}", f); // "3.142     "
    println!("{:0>10.3}", f);// "00003.142"

    // Dinamik kenglik va aniqlik
    // Динамическая ширина и точность
    let kenglik: usize = 10;
    let aniqlik: usize = 3;
    println!("{:>width$.prec$}", f, width=kenglik, prec=aniqlik);
    // "     3.142"
}

#[derive(Debug)]
struct MatrixRow {
    qiymatlar: Vec<f64>,
}

impl fmt::Display for MatrixRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.qiymatlar.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{:.2}", v)?;
        }
        write!(f, "]")
    }
}

#[derive(Debug)]
struct Rang {
    r: u8, g: u8, b: u8,
}

impl fmt::Display for Rang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}

// LowerHex va UpperHex implement qilish
// Реализация LowerHex и UpperHex
impl fmt::LowerHex for Rang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl fmt::Binary for Rang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:08b}, {:08b}, {:08b})", self.r, self.g, self.b)
    }
}

// Aniqlikni hurmat qiladigan Display
// Display уважающий точность
#[derive(Debug)]
struct Temperatur(f64);

impl fmt::Display for Temperatur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let aniqlik: usize = f.precision().unwrap_or(1);
        write!(f, "{:.prec$}°C", self.0, prec=aniqlik)
    }
}

fn display_debug_misollari() {

    // MatrixRow
    let row = MatrixRow { qiymatlar: vec![1.5, 2.7, 3.14159] };
    println!("{}", row);
    println!("{:?}", row);
    // [1.50, 2.70, 3.14]
    // MatrixRow { qiymatlar: [1.5, 2.7, 3.14159] }

    // Rang
    let qizil = Rang { r: 255, g: 0, b: 0 };
    let yashil = Rang { r: 0, g: 128, b: 0 };
    let ko_k = Rang { r: 30, g: 144, b: 255 };
    println!("{}", qizil);   // #FF0000
    println!("{:x}", qizil); // #ff0000
    println!("{:b}", yashil); // (00000000, 10000000, 00000000)
    println!("{}", ko_k);    // #1E90FF

    // Temperatur — aniqlik bilan
    // Temperatur — с точностью
    let t = Temperatur(36.6789);
    println!("{}", t);      // 36.7°C
    println!("{:.0}", t);   // 37°C
    println!("{:.3}", t);   // 36.679°C
}

fn write_misollari() {
    use std::fmt::Write;

    // String ga write!
    // write! в String
    let mut s: String = String::new();
    write!(s, "Salom").unwrap();
    write!(s, " {}!", "dunyo").unwrap();
    println!("{}", s);
    // Salom dunyo!

    // writeln! — yangi satr bilan
    // writeln! — с новой строкой
    let mut s2: String = String::new();
    writeln!(s2, "Birinchi qator").unwrap();
    writeln!(s2, "Ikkinchi qator").unwrap();
    write!(s2, "Uchinchi qator").unwrap();
    print!("{}", s2);
    println!();
    // Birinchi qator
    // Ikkinchi qator
    // Uchinchi qator

    // Vec<u8> ga write! (io::Write)
    // write! в Vec<u8> (io::Write)
    use std::io::Write as IoWrite;
    let mut buf: Vec<u8> = Vec::new();
    write!(buf, "Salom {}", 42).unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
    // Salom 42
}

fn real_hayot_misollari() {

    // 1. Jadval chiqarish
    // 1. Вывод таблицы
    println!("{:-<50}", "");
    println!("{:<20} {:>10} {:>10} {:>8}", "Mahsulot", "Narx", "Soni", "Jami");
    println!("{:-<50}", "");
    let mahsulotlar: &[(&str, f64, u32)] = &[
        ("Olma", 1500.0, 5),
        ("Non", 800.0, 3),
        ("Sut", 2000.0, 2),
    ];
    let mut jami_jami: f64 = 0.0;
    for (nomi, narx, soni) in mahsulotlar {
        let jami: f64 = narx * *soni as f64;
        jami_jami += jami;
        println!("{:<20} {:>10.0} {:>10} {:>8.0}", nomi, narx, soni, jami);
    }
    println!("{:-<50}", "");
    println!("{:<20} {:>10} {:>10} {:>8.0}", "JAMI", "", "", jami_jami);
    // --------------------------------------------------
    // Mahsulot              Narx       Soni     Jami
    // --------------------------------------------------
    // Olma                  1500          5     7500
    // Non                    800          3     2400
    // Sut                   2000          2     4000
    // --------------------------------------------------
    // JAMI                                     13900

    // 2. Log formatlash
    // 2. Форматирование логов
    fn log(daraja: &str, xabar: &str) {
        println!("[{:>5}] {}", daraja, xabar);
    }
    log("INFO", "Dastur boshlandi");
    log("WARN", "Xotira kam");
    log("ERROR", "Fayl topilmadi");
    // [ INFO] Dastur boshlandi
    // [ WARN] Xotira kam
    // [ERROR] Fayl topilmadi

    // 3. Progress bar
    // 3. Индикатор прогресса
    for i in (0..=100).step_by(25) {
        let to_ldirilgan: usize = i / 5;
        let bosh: usize = 20 - to_ldirilgan;
        println!("\r[{}>{}] {:3}%",
                 "=".repeat(to_ldirilgan),
                 " ".repeat(bosh),
                 i
        );
    }
    // [>                   ]   0%
    // [=====>              ]  25%
    // [==========>         ]  50%
    // [===============>    ]  75%
    // [====================>] 100%

    // 4. Ikkilik dump
    // 4. Двоичный дамп
    let ma_lumot: &[u8] = b"Salom Rust!";
    print!("HEX: ");
    for (i, b) in ma_lumot.iter().enumerate() {
        if i > 0 && i % 4 == 0 { print!(" "); }
        print!("{:02X}", b);
    }
    println!();
    // HEX: 53616C6F 6D205275 7374

    // 5. Rang formatlash
    // 5. Форматирование цвета
    let ranglar: &[(u8, u8, u8, &str)] = &[
        (255, 0, 0, "Qizil"),
        (0, 255, 0, "Yashil"),
        (0, 0, 255, "Ko'k"),
    ];
    for &(r, g, b, nom) in ranglar {
        println!("{:<10} #{:02X}{:02X}{:02X}  rgb({:3},{:3},{:3})",
                 nom, r, g, b, r, g, b);
    }
    // Qizil      #FF0000  rgb(255,  0,  0)
    // Yashil     #00FF00  rgb(  0,255,  0)
    // Ko'k       #0000FF  rgb(  0,  0,255)
}

fn main() {

    println!("=== ASOSIY FORMAT ===");
    asosiy_format_misollari();

    println!("\n=== SONLAR FORMAT ===");
    sonlar_format_misollari();

    println!("\n=== DISPLAY VA DEBUG ===");
    display_debug_misollari();

    println!("\n=== WRITE! ===");
    write_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Specifier              | Tavsif (UZ)                                | Описание (RU)                                                    |
// #================================================================================================================================================#
// # |   1 | {}                     | Display — oddiy chiqarish                  | Display — обычный вывод                                          |
// # |   2 | {:?}                   | Debug — tuzatish uchun                     | Debug — для отладки                                              |
// # |   3 | {:#?}                  | Pretty Debug — chiroyli                    | Pretty Debug — красивый                                          |
// # |   4 | {:b} {:o} {:x} {:X}    | Ikkilik, sakkizlik, o'n oltilik            | Двоичный, восьмеричный, шестнадцатеричный                        |
// # |   5 | {:#b} {:#o} {:#x}      | Prefiks bilan (0b, 0o, 0x)                 | С префиксом (0b, 0o, 0x)                                         |
// # |   6 | {:e} {:E}              | Ilmiy yozuv                                | Научная запись                                                   |
// # |   7 | {:.N}                  | N ta kasr raqami (precision)               | N знаков после запятой                                           |
// # |   8 | {:W}                   | W kenglik (width)                          | Ширина W                                                         |
// # |   9 | {:<} {:>} {:^}         | Chapga, o'ngga, markazga                   | Влево, вправо, по центру                                         |
// # |  10 | {:0>W}                 | Nol bilan to'ldirish                       | Заполнение нулями                                                |
// # |  11 | {:*^W}                 | * bilan to'ldirish                         | Заполнение *                                                     |
// # |  12 | {:+}                   | Musbat ishora ko'rsatish                   | Показывать знак положительного                                   |
// # |  13 | {0} {1}                | Pozitsion argument                         | Позиционный аргумент                                             |
// # |  14 | {ism=val}              | Nomlangan argument                         | Именованный аргумент                                             |
// # |  15 | {{}}                   | Qavs escaping                              | Экранирование скобок                                             |
// # |  16 | {W$.P$}                | Dinamik kenglik va aniqlik                 | Динамическая ширина и точность                                   |
// # |  17 | write!(buf, ...)       | Buffer ga yozish                           | Запись в буфер                                                   |
// # |  18 | f.precision()          | Display da aniqlik olish                   | Получение точности в Display                                     |
// #================================================================================================================================================#