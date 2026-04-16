// #================================================================================================================================================#
// #                                                            STD::ERROR::ERROR                                                                   #
// #                        STD::ERROR::ERROR — RUST DA XATO TURLARI UCHUN ASOSIY TRAIT. BARCHA XATOLAR SHU TRAITNI IMPLEMENT QILADI.               #
// #                        STD::ERROR::ERROR — ОСНОВНОЙ ТРЕЙТ ДЛЯ ТИПОВ ОШИБОК В RUST. ВСЕ ОШИБКИ РЕАЛИЗУЮТ ЭТОТ ТРЕЙТ.                            #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

// std::error::Error trait:
//
//   pub trait Error: Debug + Display {
//       fn source(&self) -> Option<&(dyn Error + 'static)> { None }
//       fn description(&self) -> &str { ... }  // deprecated
//       fn cause(&self) -> Option<&dyn Error>   // deprecated
//   }
//
// Error implement qilish uchun shart:
// Требования для реализации Error:
//   1. Debug implement qilingan bo'lishi kerak
//   2. Display implement qilingan bo'lishi kerak
//   3. source() — optional (sabab xato)

// eng minimal error implement
// минимальная реализация ошибки
#[derive(Debug)]
struct OddiyXato {
    xabar: String,
}

impl OddiyXato {
    fn new(xabar: &str) -> Self {
        OddiyXato { xabar: xabar.to_string() }
    }
}

// Display — foydalanuvchi uchun xabar
// Display — сообщение для пользователя
impl fmt::Display for OddiyXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Xato: {}", self.xabar)
    }
}

// Error — source() default None
// Error — source() по умолчанию None
impl Error for OddiyXato {}

// Ilovaning barcha xatolari — enum
// Все ошибки приложения — enum
#[derive(Debug)]
enum IlovaXato {
    TarmoqXato { kod: u16, xabar: String },
    FaylXato { yo_l: String, sabab: String },
    ParseXato(ParseIntError),
    RuxsatXato { foydalanuvchi: String },
    MalumotatBazaXato(String),
}

impl fmt::Display for IlovaXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IlovaXato::TarmoqXato { kod, xabar } =>
                write!(f, "Tarmoq xatosi {}: {}", kod, xabar),
            IlovaXato::FaylXato { yo_l, sabab } =>
                write!(f, "Fayl xatosi '{}': {}", yo_l, sabab),
            IlovaXato::ParseXato(e) =>
                write!(f, "Parse xatosi: {}", e),
            IlovaXato::RuxsatXato { foydalanuvchi } =>
                write!(f, "'{}' foydalanuvchisi ruxsatsiz", foydalanuvchi),
            IlovaXato::MalumotatBazaXato(xabar) =>
                write!(f, "Ma'lumotlar bazasi xatosi: {}", xabar),
        }
    }
}

impl Error for IlovaXato {
    // source() — sabab bo'lgan xatoni qaytarish
    // source() — возврат причины ошибки
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            IlovaXato::ParseXato(e) => Some(e),
            _                        => None,
        }
    }
}

// From implement — ? operatori uchun
// реализация From — для оператора ?
impl From<ParseIntError> for IlovaXato {
    fn from(e: ParseIntError) -> Self {
        IlovaXato::ParseXato(e)
    }
}

// Pastki daraja xato
// Низкоуровневая ошибка
#[derive(Debug)]
struct UlanishXato {
    manzil: String,
    port: u16,
}

impl fmt::Display for UlanishXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{} ga ulanib bo'lmadi", self.manzil, self.port)
    }
}

impl Error for UlanishXato {}

// Yuqori daraja xato — pastki xatoni o'z ichiga oladi
// Высокоуровневая ошибка — содержит нижнеуровневую
#[derive(Debug)]
struct ServerXato {
    xabar: String,
    sabab: UlanishXato,
}

impl fmt::Display for ServerXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Server xatosi: {}", self.xabar)
    }
}

impl Error for ServerXato {
    // source() — sabab xatoni qaytaradi
    // source() — возвращает причину ошибки
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.sabab)
    }
}

// Box<dyn Error> — turli xato turlarini qaytarish
// Box<dyn Error> — возврат разных типов ошибок
type NatijaTuri<T> = Result<T, Box<dyn Error>>;

fn faylni_o_qi_va_parse(matn: &str) -> NatijaTuri<i32> {
    // ParseIntError → Box<dyn Error> (avtomatik)
    // ParseIntError → Box<dyn Error> (автоматически)
    let son: i32 = matn.trim().parse()?;
    Ok(son * 2)
}

fn murakkab_amaliyot(kiritish: &str) -> NatijaTuri<String> {
    let son: i32 = faylni_o_qi_va_parse(kiritish)?;
    if son < 0 {
        return Err(Box::new(OddiyXato::new("Manfiy son ruxsat etilmaydi")));
    }
    Ok(format!("Natija: {}", son))
}

// Kontekst bilan xato — qayerda yuz bergani
// Ошибка с контекстом — где произошла
#[derive(Debug)]
struct KontekstliXato {
    xabar: String,
    fayl: &'static str,
    qator: u32,
    sabab: Option<Box<dyn Error>>,
}

impl KontekstliXato {
    fn new(xabar: &str) -> Self {
        KontekstliXato {
            xabar: xabar.to_string(),
            fayl: file!(),
            qator: line!(),
            sabab: None,
        }
    }

    fn sabab_bilan(mut self, sabab: impl Error + 'static) -> Self {
        self.sabab = Some(Box::new(sabab));
        self
    }
}

impl fmt::Display for KontekstliXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}:{}] {}", self.fayl, self.qator, self.xabar)
    }
}

impl Error for KontekstliXato {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.sabab.as_ref().map(|e| e.as_ref())
    }
}

// xato zanjirini chiqarish
// вывод цепочки ошибок
fn xato_zanjirini_chiqar(xato: &dyn Error) {
    println!("Xato: {}", xato);

    let mut sabab = xato.source();
    let mut daraja: u32 = 1;

    while let Some(s) = sabab {
        println!("  {}. Sabab: {}", daraja, s);
        sabab = s.source();
        daraja += 1;
    }
}

#[derive(Debug)]
struct ValidatsiyaXato {
    maydon: String,
    xabar: String,
}

impl fmt::Display for ValidatsiyaXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Validatsiya xatosi '{}' maydonida: {}", self.maydon, self.xabar)
    }
}

impl Error for ValidatsiyaXato {}

#[derive(Debug)]
struct KoʻpXato {
    xatolar: Vec<ValidatsiyaXato>,
}

impl fmt::Display for KoʻpXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ta validatsiya xatosi:", self.xatolar.len())?;
        for xato in &self.xatolar {
            write!(f, "\n  - {}", xato)?;
        }
        Ok(())
    }
}

impl Error for KoʻpXato {}

// Foydalanuvchi ma'lumotlarini tekshirish
// Проверка данных пользователя
fn tekshir(ism: &str, yosh: i32, email: &str) -> Result<(), KoʻpXato> {
    let mut xatolar: Vec<ValidatsiyaXato> = Vec::new();

    if ism.is_empty() {
        xatolar.push(ValidatsiyaXato {
            maydon: String::from("ism"),
            xabar: String::from("Bo'sh bo'lishi mumkin emas"),
        });
    }
    if ism.len() > 50 {
        xatolar.push(ValidatsiyaXato {
            maydon: String::from("ism"),
            xabar: String::from("50 ta belgidan oshmasligi kerak"),
        });
    }
    if yosh < 0 || yosh > 150 {
        xatolar.push(ValidatsiyaXato {
            maydon: String::from("yosh"),
            xabar: format!("{} noto'g'ri yosh", yosh),
        });
    }
    if !email.contains('@') {
        xatolar.push(ValidatsiyaXato {
            maydon: String::from("email"),
            xabar: String::from("@ belgisi yo'q"),
        });
    }

    if xatolar.is_empty() {
        Ok(())
    } else {
        Err(KoʻpXato { xatolar })
    }
}

// ? operatori — IlovaXato bilan
// оператор ? — с IlovaXato
fn sonni_parse_qilish(s: &str) -> Result<i32, IlovaXato> {
    let son: i32 = s.parse()?;  // ParseIntError → IlovaXato::ParseXato
    Ok(son)
}

fn ikkiga_oshirish(s: &str) -> Result<i32, IlovaXato> {
    let son: i32 = sonni_parse_qilish(s)?;
    if son < 0 {
        return Err(IlovaXato::RuxsatXato {
            foydalanuvchi: String::from("noma'lum"),
        });
    }
    Ok(son * 2)
}

// T: Error — generic bound
// T: Error — generic ограничение
fn xato_xabarini_ol<E: Error>(xato: &E) -> String {
    format!("Xato: {} | Debug: {:?}", xato, xato)
}

// Box<dyn Error> qaytarish — flexible
// возврат Box<dyn Error> — гибко
fn istalgan_xato_qaytarish(muvaffaqiyatlimi: bool) -> Result<String, Box<dyn Error>> {
    if muvaffaqiyatlimi {
        Ok(String::from("Muvaffaqiyatli!"))
    } else {
        Err(Box::new(OddiyXato::new("Biror narsa xato ketdi")))
    }
}

fn main() {

    // OddiyXato — yaratish va ishlatish
    // OddiyXato — создание и использование
    let xato = OddiyXato::new("fayl topilmadi");
    println!("{}", xato);
    println!("{:?}", xato);
    println!("source: {:?}", xato.source());
    // Xato: fayl topilmadi
    // OddiyXato { xabar: "fayl topilmadi" }
    // source: None

    // dyn Error sifatida ishlatish
    // использование как dyn Error
    let dyn_xato: &dyn Error = &xato;
    println!("{}", dyn_xato);
    // Xato: fayl topilmadi

    // IlovaXato — turli variantlar
    // IlovaXato — разные варианты
    let tarmoq: IlovaXato = IlovaXato::TarmoqXato {
        kod: 404,
        xabar: String::from("sahifa topilmadi"),
    };
    let fayl: IlovaXato = IlovaXato::FaylXato {
        yo_l: String::from("/home/user/config.toml"),
        sabab: String::from("o'qish huquqi yo'q"),
    };
    let parse: IlovaXato = IlovaXato::ParseXato(
        "abc".parse::<i32>().unwrap_err()
    );

    println!("{}", tarmoq);
    println!("{}", fayl);
    println!("{}", parse);
    println!("source: {:?}", parse.source());
    // Tarmoq xatosi 404: sahifa topilmadi
    // Fayl xatosi '/home/user/config.toml': o'qish huquqi yo'q
    // Parse xatosi: invalid digit found in string
    // source: Some(ParseIntError { kind: InvalidDigit })

    // ServerXato → UlanishXato zanjiri
    // цепочка ServerXato → UlanishXato
    let server_xatosi = ServerXato {
        xabar: String::from("so'rovni qayta ishlab bo'lmadi"),
        sabab: UlanishXato {
            manzil: String::from("db.example.com"),
            port: 5432,
        },
    };

    xato_zanjirini_chiqar(&server_xatosi);
    // Xato: Server xatosi: so'rovni qayta ishlab bo'lmadi
    //   1. Sabab: db.example.com:5432 ga ulanib bo'lmadi

    // Box<dyn Error> — turli xatolar
    // Box<dyn Error> — разные ошибки
    let r1: NatijaTuri<String> = murakkab_amaliyot("21");
    let r2: NatijaTuri<String> = murakkab_amaliyot("abc");
    let r3: NatijaTuri<String> = murakkab_amaliyot("-5");

    println!("{:?}", r1);
    println!("{:?}", r2);
    println!("{:?}", r3);
    // Ok("Natija: 42")
    // Err(ParseIntError { kind: InvalidDigit })
    // Err(OddiyXato { xabar: "Manfiy son ruxsat etilmaydi" })

    // KontekstliXato — fayl va qator bilan
    // KontekstliXato — с файлом и строкой
    let k_xato = KontekstliXato::new("Ma'lumotlar bazasiga ulanib bo'lmadi")
        .sabab_bilan(OddiyXato::new("timeout"));
    xato_zanjirini_chiqar(&k_xato);
    // Xato: [src/main.rs:XXX] Ma'lumotlar bazasiga ulanib bo'lmadi
    //   1. Sabab: Xato: timeout

    // to'g'ri ma'lumotlar
    // правильные данные
    let to_g_ri = tekshir("Dilshod", 25, "dilshod@example.com");
    println!("{:?}", to_g_ri);
    // Ok(())

    // xatoli ma'lumotlar
    // ошибочные данные
    let xatoli = tekshir("", -5, "noto_g_ri_email");
    match xatoli {
        Ok(()) => println!("To'g'ri"),
        Err(e) => println!("{}", e),
    }
    // 3 ta validatsiya xatosi:
    //   - Validatsiya xatosi 'ism' maydonida: Bo'sh bo'lishi mumkin emas
    //   - Validatsiya xatosi 'yosh' maydonida: -5 noto'g'ri yosh
    //   - Validatsiya xatosi 'email' maydonida: @ belgisi yo'q

    // ? operatori — From orqali avtomatik konversiya
    // оператор ? — автоматическая конвертация через From
    let p1 = ikkiga_oshirish("21");
    let p2 = ikkiga_oshirish("abc");
    println!("{:?}", p1);
    println!("{:?}", p2);
    // Ok(42)
    // Err(ParseXato(ParseIntError { kind: InvalidDigit }))

    // xato_xabarini_ol — generic E: Error
    // xato_xabarini_ol — generic E: Error
    let g_xato = OddiyXato::new("test xato");
    let xabar: String = xato_xabarini_ol(&g_xato);
    println!("{}", xabar);
    // Xato: Xato: test xato | Debug: OddiyXato { xabar: "test xato" }

    // Box<dyn Error> — flexible qaytarish
    // Box<dyn Error> — гибкий возврат
    let m1 = istalgan_xato_qaytarish(true);
    let m2 = istalgan_xato_qaytarish(false);
    println!("{:?}", m1);
    println!("{:?}", m2);
    // Ok("Muvaffaqiyatli!")
    // Err(OddiyXato { xabar: "Biror narsa xato ketdi" })

    // downcast_ref — Box<dyn Error> dan aniq tur olish
    // downcast_ref — получение конкретного типа из Box<dyn Error>
    let boxed: Box<dyn Error> = Box::new(OddiyXato::new("test"));
    if let Some(oddiy) = boxed.downcast_ref::<OddiyXato>() {
        println!("OddiyXato topildi: {}", oddiy.xabar);
    }
    // OddiyXato topildi: test

    // is — tur tekshirish
    // is — проверка типа
    println!("{}", boxed.is::<OddiyXato>());
    // true
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                       ERROR TRAIT ASOSLARI                                                                                   |
// #================================================================================================================================================#
// # |   1 | impl Error for T         | T uchun Error trait implement qilish                 | Реализация Error для T                               |
// # |   2 | Debug + Display shart    | Error uchun Debug va Display bo'lishi shart          | Для Error обязателен Debug и Display                 |
// # |   3 | fn source()              | Sabab xatoni qaytarish (optional)                    | Возврат причины ошибки (опционально)                 |
// # |   4 | source() default None    | Agar sabab yo'q bo'lsa — None                        | Если причины нет — None                              |
// #================================================================================================================================================#
// # |                                       XATO TURLARI                                                                                           |
// #================================================================================================================================================#
// # |   5 | struct Xato              | Oddiy struct xato — minimal implement                | Простая struct ошибка — минимальная реализация       |
// # |   6 | enum Xato                | Ko'p holatli xato — variantlar bilan                 | Многовариантная ошибка — с вариантами                |
// # |   7 | enum Xato + source()     | Zanjirli xato — sabab xatoni saqlash                 | Цепочка ошибок — сохранение причины                  |
// #================================================================================================================================================#
// # |                                       BOX<DYN ERROR>                                                                                         |
// #================================================================================================================================================#
// # |   8 | Box<dyn Error>           | Turli xato turlarini birga qaytarish                 | Возврат разных типов ошибок вместе                   |
// # |   9 | Result<T, Box<dyn Error>>| Eng flexible natija turi                             | Наиболее гибкий тип результата                       |
// # |  10 | type NatijaTuri<T>       | Box<dyn Error> uchun type alias                      | Type alias для Box<dyn Error>                        |
// #================================================================================================================================================#
// # |                                       FROM VA ? OPERATOR                                                                                     |
// #================================================================================================================================================#
// # |  11 | impl From<E1> for MyErr  | ? operatori uchun xato konversiyasi                  | Конвертация ошибок для оператора ?                   |
// # |  12 | ? operatori              | From orqali avtomatik xato konversiyasi              | Автоматическая конвертация через From                |
// # |  13 | xato? — From chaqiriladi | E1 → E2 avtomatik (From<E1> for E2 bo'lsa)           | E1 → E2 автоматически (если есть From<E1> for E2)    |
// #================================================================================================================================================#
// # |                                       XATO ZANJIRI                                                                                           |
// #================================================================================================================================================#
// # |  14 | source() zanjiri         | Xato sababini rekursiv o'qish                        | Рекурсивное чтение причины ошибки                    |
// # |  15 | while let Some(s)        | Barcha sabablarni zanjirda o'qish                    | Чтение всех причин в цепочке                         |
// #================================================================================================================================================#
// # |                                       DOWNCAST                                                                                               |
// #================================================================================================================================================#
// # |  16 | downcast_ref::<T>()      | Box<dyn Error> dan aniq tur olish                    | Получение конкретного типа из Box<dyn Error>         |
// # |  17 | .is::<T>()               | Box<dyn Error> tur tekshirish                        | Проверка типа Box<dyn Error>                         |
// #================================================================================================================================================#
// # |                                       GENERIC BILAN                                                                                          |
// #================================================================================================================================================#
// # |  18 | fn f<E: Error>(e: &E)    | Generic Error bound                                  | Generic ограничение Error                            |
// # |  19 | E: Error + 'static       | Box<dyn Error> ga qo'yish uchun 'static kerak        | 'static нужен для помещения в Box<dyn Error>         |
// # |  20 | dyn Error + Send + Sync  | Thread safe xato                                     | Потокобезопасная ошибка                              |
// #================================================================================================================================================#