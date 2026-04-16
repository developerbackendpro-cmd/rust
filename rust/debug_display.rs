// #================================================================================================================================================#
// #                                                             DEBUG  |  DISPLAY                                                                  #
// #                                    DEBUG — DASTURCHI UCHUN. DISPLAY — FOYDALANUVCHI UCHUN. IKKALASI TRAIT.                                     #
// #                                    DEBUG — ДЛЯ РАЗРАБОТЧИКА. DISPLAY — ДЛЯ ПОЛЬЗОВАТЕЛЯ. ОБА ТРЕЙТА.                                           #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// #[derive(Debug)] — avtomatik Debug implementatsiya
// автоматическая реализация Debug
#[derive(Debug)]
struct Nuqta {
    x: f64,
    y: f64,
}

// #[derive(Debug)] — enum uchun
// Debug для enum
#[derive(Debug)]
enum Rang {
    Qizil,
    Yashil,
    Moviy,
    Maxsus(u8, u8, u8),
}

// #[derive(Debug)] — ichma-ich structlar
// вложенные структуры с Debug
#[derive(Debug)]
struct Doira {
    markaz: Nuqta,
    radius: f64,
}

// Display — qo'lda implement qilinadi
// Display реализуется вручную
struct Talaba {
    ism: String,
    yosh: u32,
    baho: f64,
}

impl fmt::Display for Talaba {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (yosh: {}, baho: {:.1})", self.ism, self.yosh, self.baho)
    }
}

// Display va Debug birga
// Display и Debug вместе
#[derive(Debug)]
struct Temperatura {
    daraja: f64,
}

impl fmt::Display for Temperatura {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1}°C", self.daraja)
    }
}

// Display — enum uchun
// Display для enum
#[derive(Debug)]
enum Holat {
    Faol,
    Nofaol,
    Kutmoqda,
}

impl fmt::Display for Holat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Holat::Faol     => write!(f, "Faol"),
            Holat::Nofaol   => write!(f, "Nofaol"),
            Holat::Kutmoqda => write!(f, "Kutmoqda..."),
        }
    }
}

// Debug ni qo'lda implement qilish
// ручная реализация Debug
struct Parol {
    qiymat: String,
}

impl fmt::Debug for Parol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parol(***)")
    }
}

// debug_struct builder pattern
// построитель debug_struct
struct Foydalanuvchi {
    ism: String,
    email: String,
    yosh: u32,
}

impl fmt::Debug for Foydalanuvchi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Foydalanuvchi")
            .field("ism", &self.ism)
            .field("email", &self.email)
            .field("yosh", &self.yosh)
            .finish()
    }
}

// Binary — {:b} formati
// Binary — формат {:b}
struct Bayroq(u8);

impl fmt::Binary for Bayroq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

// LowerHex — {:x} formati
// LowerHex — формат {:x}
struct RangKodi(u32);

impl fmt::LowerHex for RangKodi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:06x}", self.0)
    }
}

struct Matn(String);

impl fmt::Display for Matn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // kengligi berilganmi?
        // указана ли ширина?
        if let Some(kenglik) = f.width() {
            write!(f, "{:>kenglik$}", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

fn main() {

    // {:?} — oddiy debug chiqish
    // обычный вывод debug
    let nuqta = Nuqta { x: 1.0, y: 2.0 };
    println!("{:?}", nuqta);
    // Nuqta { x: 1.0, y: 2.0 }

    // {:#?} — chiroyli debug chiqish (pretty print)
    // красивый вывод debug (pretty print)
    let doira = Doira {
        markaz: Nuqta { x: 0.0, y: 0.0 },
        radius: 5.0,
    };
    println!("{:#?}", doira);
    // Doira {
    //     markaz: Nuqta {
    //         x: 0.0,
    //         y: 0.0,
    //     },
    //     radius: 5.0,
    // }

    // enum debug
    // отладка enum
    let rang = Rang::Maxsus(255, 128, 0);
    println!("{:?}", rang);
    // Maxsus(255, 128, 0)

    // built-in turlar debug
    // отладка встроенных типов
    let vektor: Vec<i32> = vec![1, 2, 3];
    let tuple: (i32, &str, bool) = (42, "salom", true);
    let opsiya: Option<i32> = Some(99);
    println!("{:?}", vektor);
    println!("{:?}", tuple);
    println!("{:?}", opsiya);
    // [1, 2, 3]
    // (42, "salom", true)
    // Some(99)

    // {} — display chiqish
    // вывод display
    let talaba = Talaba {
        ism: String::from("Dilshod"),
        yosh: 22,
        baho: 9.5,
    };
    println!("{}", talaba);
    // Dilshod (yosh: 22, baho: 9.5)

    // temperatura display va debug farqi
    // разница display и debug для температуры
    let temp = Temperatura { daraja: 36.6 };
    println!("{}", temp);
    println!("{:?}", temp);
    // 36.6°C
    // Temperatura { daraja: 36.6 }

    // holat display
    // вывод состояния
    let holat = Holat::Kutmoqda;
    println!("{}", holat);
    println!("{:?}", holat);
    // Kutmoqda...
    // Kutmoqda

    // parol debug — maxfiy
    // отладка пароля — скрытый
    let parol = Parol { qiymat: String::from("secret123") };
    println!("{:?}", parol);
    // Parol(***)

    // foydalanuvchi debug_struct bilan
    // пользователь с debug_struct
    let user = Foydalanuvchi {
        ism: String::from("Dilshod"),
        email: String::from("dilshod@example.com"),
        yosh: 22,
    };
    println!("{:#?}", user);
    // Foydalanuvchi {
    //     ism: "Dilshod",
    //     email: "dilshod@example.com",
    //     yosh: 22,
    // }

    // Binary — {:b}
    let bayroq = Bayroq(0b10110101);
    println!("{:b}", bayroq);
    // 10110101

    // LowerHex — {:x}
    let rang_kodi = RangKodi(0xFF8800);
    println!("{:x}", rang_kodi);
    // #ff8800

    // {:?} — debug
    let son: i32 = 42;
    println!("{:?}", son);
    // 42

    // {:#?} — pretty debug
    let vektor2: Vec<i32> = vec![1, 2, 3];
    println!("{:#?}", vektor2);
    // [
    //     1,
    //     2,
    //     3,
    // ]

    // {:p} — pointer manzili
    // адрес указателя
    let x: i32 = 42;
    println!("{:p}", &x);
    // 0x7ffd...

    // {:b} — ikkilik
    // двоичное
    let ikkilik: u8 = 0b10101010;
    println!("{:b}", ikkilik);
    // 10101010

    // {:o} — sakkizlik
    // восьмеричное
    let sakkizlik: u8 = 255;
    println!("{:o}", sakkizlik);
    // 377

    // {:x} — kichik hex
    // строчный hex
    let hex_kichik: u8 = 255;
    println!("{:x}", hex_kichik);
    // ff

    // {:X} — katta hex
    // заглавный hex
    let hex_katta: u8 = 255;
    println!("{:X}", hex_katta);
    // FF

    // {:e} — ilmiy notation (kichik)
    // научная нотация (строчная)
    let ilmiy: f64 = 1_000_000.0;
    println!("{:e}", ilmiy);
    // 1e6

    // {:E} — ilmiy notation (katta)
    // научная нотация (заглавная)
    println!("{:E}", ilmiy);
    // 1E6

    // kenglik
    // ширина
    let chap_hizalash: String = format!("{:<10}", "chap");
    let ong_hizalash: String = format!("{:>10}", "ong");
    let markaz_hizalash: String = format!("{:^10}", "markaz");
    println!("|{}|", chap_hizalash);
    println!("|{}|", ong_hizalash);
    println!("|{}|", markaz_hizalash);
    // |chap      |
    // |       ong|
    // |  markaz  |

    // belgili to'ldirish
    // заполнение символом
    let nol_bilan: String = format!("{:0>8}", 42);
    let chiziq_bilan: String = format!("{:-<10}", "salom");
    let yulduz_bilan: String = format!("{:*^10}", "hi");
    println!("{}", nol_bilan);
    println!("{}", chiziq_bilan);
    println!("{}", yulduz_bilan);
    // 00000042
    // salom-----
    // ****hi****

    // aniqlik
    // точность
    let aniqlik2: String = format!("{:.2}", 3.14159);
    let aniqlik5: String = format!("{:.5}", 3.14159);
    println!("{}", aniqlik2);
    println!("{}", aniqlik5);
    // 3.14
    // 3.14159

    // kenglik va aniqlik birga
    // ширина и точность вместе
    let birga: String = format!("{:10.3}", 3.14159);
    println!("|{}|", birga);
    // |     3.142|

    // Display implement qilsa to_string() bepul keladi
    // реализация Display даёт to_string() бесплатно
    let temp2 = Temperatura { daraja: 100.0 };
    let matn: String = temp2.to_string();
    println!("{}", matn);
    // 100.0°C

    // format! bilan string yaratish
    // создание строки через format!
    let talaba2 = Talaba {
        ism: String::from("Ali"),
        yosh: 20,
        baho: 8.5,
    };
    let xabar: String = format!("Talaba: {}", talaba2);
    println!("{}", xabar);
    // Talaba: Ali (yosh: 20, baho: 8.5)

    // dbg! — debug uchun eng qulay
    // наиболее удобный для отладки
    let nuqta2 = Nuqta { x: 3.0, y: 4.0 };
    let masofa: f64 = dbg!(nuqta2.x * nuqta2.x + nuqta2.y * nuqta2.y).sqrt();
    println!("{}", masofa);
    // [src/main.rs:XX] nuqta2.x * nuqta2.x + nuqta2.y * nuqta2.y = 25.0
    // 5.0
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                          DEBUG TRAIT                                                                                         |
// #================================================================================================================================================#
// # |   1 | #[derive(Debug)]         | Avtomatik Debug implementatsiya                      | Автоматическая реализация Debug                      |
// # |   2 | {:?}                     | Debug formati                                        | Формат Debug                                         |
// # |   3 | {:#?}                    | Pretty debug (chiroyli, ichma-ich)                   | Красивый debug (с отступами)                         |
// # |   4 | impl fmt::Debug          | Qo'lda Debug implementatsiya                         | Ручная реализация Debug                              |
// # |   5 | f.debug_struct(...)      | Struct uchun debug builder                           | Построитель debug для структуры                      |
// #================================================================================================================================================#
// # |                                         DISPLAY TRAIT                                                                                        |
// #================================================================================================================================================#
// # |   6 | impl fmt::Display        | Foydalanuvchi uchun chiqish                          | Вывод для пользователя                               |
// # |   7 | {}                       | Display formati                                      | Формат Display                                       |
// # |   8 | .to_string()             | Display → String (bepul keladi)                      | Display → String (бесплатно)                         |
// #================================================================================================================================================#
// # |                                    QOSHIMCHA FORMAT TRAITLARI                                                                                |
// #================================================================================================================================================#
// # |   9 | impl fmt::Binary         | {:b} formati uchun                                   | Для формата {:b}                                     |
// # |  10 | impl fmt::LowerHex       | {:x} formati uchun                                   | Для формата {:x}                                     |
// # |  11 | impl fmt::UpperHex       | {:X} formati uchun                                   | Для формата {:X}                                     |
// # |  12 | impl fmt::Octal          | {:o} formati uchun                                   | Для формата {:o}                                     |
// #================================================================================================================================================#
// # |                                       FORMAT BELGILARI                                                                                       |
// #================================================================================================================================================#
// # |  13 | {:b}                     | Ikkilik format                                       | Двоичный формат                                      |
// # |  14 | {:o}                     | Sakkizlik format                                     | Восьмеричный формат                                  |
// # |  15 | {:x} / {:X}              | Hex format (kichik / katta)                          | Hex формат (строчный / заглавный)                    |
// # |  16 | {:e} / {:E}              | Ilmiy notation                                       | Научная нотация                                      |
// # |  17 | {:p}                     | Pointer manzili                                      | Адрес указателя                                      |
// #================================================================================================================================================#
// # |                                      KENGLIK VA TO'LDIRISH                                                                                   |
// #================================================================================================================================================#
// # |  18 | {:<10}                   | Chap hizalash, 10 belgili kenglik                    | Выравнивание влево, ширина 10                        |
// # |  19 | {:>10}                   | Ong hizalash, 10 belgili kenglik                     | Выравнивание вправо, ширина 10                       |
// # |  20 | {:^10}                   | Markaz hizalash                                      | Выравнивание по центру                               |
// # |  21 | {:0>8}                   | Nol bilan to'ldirish                                 | Заполнение нулями                                    |
// # |  22 | {:-<10}                  | Chiziq bilan to'ldirish                              | Заполнение тире                                      |
// # |  23 | {:.2}                    | 2 kasr raqami aniqlik                                | Точность 2 знака после запятой                       |
// # |  24 | {:10.3}                  | Kenglik 10, aniqlik 3                                | Ширина 10, точность 3                                |
// #================================================================================================================================================#