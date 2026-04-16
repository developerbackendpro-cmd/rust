// #================================================================================================================================================#
// #                                                                FROM  |  INTO                                                                   #
// #                                FROM — BOSHQA TURDAN YARATISH. INTO — BOSHQA TURGA AYLANTIRISH. IKKALASI BOG'LIQ.                               #
// #                                FROM — СОЗДАНИЕ ИЗ ДРУГОГО ТИПА. INTO — ПРЕОБРАЗОВАНИЕ В ДРУГОЙ ТИП. ОБА СВЯЗАНЫ.                               #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// From implement qilsak — Into bepul keladi
// реализуя From — Into достаётся бесплатно
// T: From<U>  →  U: Into<T>  (avtomatik)

// oddiy struct uchun From
// From для простой структуры
#[derive(Debug)]
struct Wrapper(i32);

impl From<i32> for Wrapper {
    fn from(qiymat: i32) -> Self {
        Wrapper(qiymat)
    }
}

// bir nechta From implement qilish
// реализация нескольких From
#[derive(Debug)]
struct Temperatura {
    celsius: f64,
}

impl From<f64> for Temperatura {
    fn from(celsius: f64) -> Self {
        Temperatura { celsius }
    }
}

impl From<i32> for Temperatura {
    fn from(celsius: i32) -> Self {
        Temperatura { celsius: celsius as f64 }
    }
}

// Kelvin dan Celsius ga
// из Кельвина в Цельсий
#[derive(Debug)]
struct Kelvin(f64);

impl From<Kelvin> for Temperatura {
    fn from(k: Kelvin) -> Self {
        Temperatura { celsius: k.0 - 273.15 }
    }
}

// String::from — eng ko'p ishlatiladigan From
// String::from — наиболее часто используемый From
fn string_from_misollari() {
    let s1: String = String::from("salom");
    let s2: String = String::from('R');
    let s3: String = String::from(42.to_string());

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    // salom
    // R
    // 42
}

// kichik → katta: xavfsiz
// маленький → большой: безопасно
fn integer_from_misollari() {
    let u8_son: u8 = 42;
    let u16_son: u16 = u16::from(u8_son);
    let u32_son: u32 = u32::from(u8_son);
    let u64_son: u64 = u64::from(u8_son);
    let i32_son: i32 = i32::from(u8_son);
    let i64_son: i64 = i64::from(u8_son);

    println!("u8  → u16: {}", u16_son);
    println!("u8  → u32: {}", u32_son);
    println!("u8  → u64: {}", u64_son);
    println!("u8  → i32: {}", i32_son);
    println!("u8  → i64: {}", i64_son);
    // u8  → u16: 42
    // u8  → u32: 42
    // u8  → u64: 42
    // u8  → i32: 42
    // u8  → i64: 42

    // bool → integer
    // bool → целое
    let rost: bool = true;
    let yolg_on: bool = false;
    let rost_i32: i32 = i32::from(rost);
    let yolg_on_u8: u8 = u8::from(yolg_on);
    println!("true → i32: {}", rost_i32);
    println!("false → u8: {}", yolg_on_u8);
    // true → i32: 1
    // false → u8: 0

    // char → u32
    // char → u32
    let harf: char = 'A';
    let kod: u32 = u32::from(harf);
    println!("'A' → u32: {}", kod);
    // 'A' → u32: 65
}

// From implement qilsak Into bepul
// реализуя From, Into достаётся бесплатно
fn into_misollari() {
    // Wrapper From<i32> implement qilgan
    // Wrapper реализует From<i32>
    let w1: Wrapper = Wrapper::from(42);
    let w2: Wrapper = 99.into();
    println!("{:?}", w1);
    println!("{:?}", w2);
    // Wrapper(42)
    // Wrapper(99)

    // Temperatura uchun Into
    // Into для Temperatura
    let t1: Temperatura = Temperatura::from(36.6f64);
    let t2: Temperatura = 37.0f64.into();
    let t3: Temperatura = 36i32.into();
    let t4: Temperatura = Kelvin(310.15).into();
    println!("{:?}", t1);
    println!("{:?}", t2);
    println!("{:?}", t3);
    println!("{:?}", t4);
    // Temperatura { celsius: 36.6 }
    // Temperatura { celsius: 37.0 }
    // Temperatura { celsius: 36.0 }
    // Temperatura { celsius: 37.0 }

    // String into
    // String into
    let s: String = "salom".into();
    let s2: String = 'R'.into();
    println!("{}", s);
    println!("{}", s2);
    // salom
    // R
}

#[derive(Debug)]
struct Foydalanuvchi {
    ism: String,
    yosh: u32,
    email: String,
}

// &str dan Foydalanuvchi yaratish
// создание Foydalanuvchi из &str
impl From<&str> for Foydalanuvchi {
    fn from(ism: &str) -> Self {
        Foydalanuvchi {
            ism: ism.to_string(),
            yosh: 0,
            email: format!("{}@example.com", ism.to_lowercase()),
        }
    }
}

// tuple dan Foydalanuvchi yaratish
// создание Foydalanuvchi из кортежа
impl From<(&str, u32)> for Foydalanuvchi {
    fn from((ism, yosh): (&str, u32)) -> Self {
        Foydalanuvchi {
            ism: ism.to_string(),
            yosh,
            email: format!("{}@example.com", ism.to_lowercase()),
        }
    }
}

#[derive(Debug)]
enum Xato {
    Tarmoq(String),
    Fayl(String),
    Parse(String),
}

impl From<std::num::ParseIntError> for Xato {
    fn from(e: std::num::ParseIntError) -> Self {
        Xato::Parse(e.to_string())
    }
}

impl From<std::io::Error> for Xato {
    fn from(e: std::io::Error) -> Self {
        Xato::Fayl(e.to_string())
    }
}

impl fmt::Display for Xato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Xato::Tarmoq(m) => write!(f, "Tarmoq xatosi: {}", m),
            Xato::Fayl(m)   => write!(f, "Fayl xatosi: {}", m),
            Xato::Parse(m)  => write!(f, "Parse xatosi: {}", m),
        }
    }
}

// ? operatori From bilan ishlaydi
// оператор ? работает с From
fn son_parse_qil(s: &str) -> Result<i32, Xato> {
    let son: i32 = s.parse()?;  // ParseIntError → Xato::Parse (From orqali)
    Ok(son * 2)
}

// impl Into<T> o'rniga impl From<X> for T ishlatish yaxshi
// лучше использовать impl From<X> for T вместо impl Into<T>
//
// Lekin funksiya parametrida Into juda qulay:
// Но в параметрах функции Into очень удобен:

fn salom_ayt(ism: impl Into<String>) {
    let ism_string: String = ism.into();
    println!("Salom, {}!", ism_string);
}

fn yosh_belgilashtir(yosh: impl Into<u64>) -> u64 {
    yosh.into()
}

fn collection_from_misollari() {
    // Vec dan boshqa kolleksiyalar
    // коллекции из Vec

    // Vec → String (join bilan)
    // Vec → String (через join)
    let harflar: Vec<char> = vec!['s', 'a', 'l', 'o', 'm'];
    let matn: String = harflar.into_iter().collect();
    println!("{}", matn);
    // salom

    // &str → Vec<u8>
    // &str → Vec<u8>
    let baytlar: Vec<u8> = Vec::from("salom");
    println!("{:?}", baytlar);
    // [115, 97, 108, 111, 109]

    // array → Vec
    // массив → Vec
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let vektor: Vec<i32> = Vec::from(array);
    println!("{:?}", vektor);
    // [1, 2, 3, 4, 5]

    // Vec → VecDeque
    // Vec → VecDeque
    use std::collections::VecDeque;
    let v: Vec<i32> = vec![1, 2, 3];
    let deque: VecDeque<i32> = VecDeque::from(v);
    println!("{:?}", deque);
    // [1, 2, 3]

    // array → String
    // массив символов → String
    let chars: [char; 5] = ['R', 'u', 's', 't', '!'];
    let s: String = chars.iter().collect();
    println!("{}", s);
    // Rust!

    // Box — From
    // Box — From
    let boxed: Box<i32> = Box::from(42);
    println!("{}", boxed);
    // 42

    // Box<str> — From<&str>
    // Box<str> — From<&str>
    let boxed_str: Box<str> = Box::from("salom");
    println!("{}", boxed_str);
    // salom

    // Rc — From
    // Rc — From
    use std::rc::Rc;
    let rc: Rc<i32> = Rc::from(42);
    println!("{}", rc);
    // 42

    // Arc — From
    // Arc — From
    use std::sync::Arc;
    let arc: Arc<str> = Arc::from("salom");
    println!("{}", arc);
    // salom
}

fn option_result_from_misollari() {
    // Option<T> — From
    // Option<T> — From
    let x: Option<i32> = Option::from(42);
    let y: Option<i32> = None;
    println!("{:?}", x);
    println!("{:?}", y);
    // Some(42)
    // None

    // Result dan Option ga
    // из Result в Option
    let ok_result: Result<i32, &str> = Ok(42);
    let err_result: Result<i32, &str> = Err("xato");
    let ok_option: Option<i32> = ok_result.ok();
    let err_option: Option<&str> = err_result.err();
    println!("{:?}", ok_option);
    println!("{:?}", err_option);
    // Some(42)
    // Some("xato")
}

// T: From<U> — generic bound
// T: From<U> — generic ограничение
fn aylantir<T, U>(qiymat: U) -> T
where
    T: From<U>,
{
    T::from(qiymat)
}

// Into<T> — argument sifatida
// Into<T> — как аргумент
fn birlash<T: Into<String>>(a: T, b: T) -> String {
    let mut natija: String = a.into();
    natija.push_str(&b.into());
    natija
}

// Newtype pattern — From bilan
// Паттерн Newtype — с From
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Metr(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Santimetr(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Millimetr(f64);

impl From<Santimetr> for Metr {
    fn from(sm: Santimetr) -> Self {
        Metr(sm.0 / 100.0)
    }
}

impl From<Millimetr> for Metr {
    fn from(mm: Millimetr) -> Self {
        Metr(mm.0 / 1000.0)
    }
}

impl From<Metr> for Santimetr {
    fn from(m: Metr) -> Self {
        Santimetr(m.0 * 100.0)
    }
}

impl From<Millimetr> for Santimetr {
    fn from(mm: Millimetr) -> Self {
        Santimetr(mm.0 / 10.0)
    }
}

impl fmt::Display for Metr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.3} m", self.0)
    }
}

impl fmt::Display for Santimetr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.1} sm", self.0)
    }
}

impl fmt::Display for Millimetr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.0} mm", self.0)
    }
}

fn main() {

    // Wrapper — From<i32>
    // Wrapper — From<i32>
    let w1: Wrapper = Wrapper::from(42);
    let w2: Wrapper = Wrapper::from(-10);
    println!("{:?}", w1);
    println!("{:?}", w2);
    // Wrapper(42)
    // Wrapper(-10)

    // Temperatura — bir nechta From
    // Temperatura — несколько From
    let t1: Temperatura = Temperatura::from(36.6f64);
    let t2: Temperatura = Temperatura::from(37i32);
    let t3: Temperatura = Temperatura::from(Kelvin(310.15));
    println!("{:?}", t1);
    println!("{:?}", t2);
    println!("{:?}", t3);
    // Temperatura { celsius: 36.6 }
    // Temperatura { celsius: 37.0 }
    // Temperatura { celsius: 37.0 }

    // String::from
    // String::from
    string_from_misollari();

    // integer From
    // From для целых
    integer_from_misollari();

    // Into — From dan bepul
    // Into — бесплатно от From
    into_misollari();

    // &str dan Foydalanuvchi
    // Foydalanuvchi из &str
    let f1: Foydalanuvchi = Foydalanuvchi::from("Dilshod");
    let f2: Foydalanuvchi = Foydalanuvchi::from(("Ali", 25u32));
    let f3: Foydalanuvchi = "Vali".into();
    println!("{:#?}", f1);
    println!("{:#?}", f2);
    println!("{:#?}", f3);
    // Foydalanuvchi { ism: "Dilshod", yosh: 0, email: "dilshod@example.com" }
    // Foydalanuvchi { ism: "Ali", yosh: 25, email: "ali@example.com" }
    // Foydalanuvchi { ism: "Vali", yosh: 0, email: "vali@example.com" }

    // ? operatori From orqali
    // оператор ? через From
    let muvaffaqiyatli = son_parse_qil("21");
    let xatoli = son_parse_qil("abc");
    println!("{:?}", muvaffaqiyatli);
    println!("{:?}", xatoli);
    // Ok(42)
    // Err(Parse("invalid digit found in string"))

    // impl Into<String> — &str ham, String ham qabul qiladi
    // impl Into<String> — принимает и &str и String
    salom_ayt("Dilshod");
    salom_ayt(String::from("Ali"));
    // Salom, Dilshod!
    // Salom, Ali!

    // impl Into<u64>
    let y1: u64 = yosh_belgilashtir(25u8);
    let y2: u64 = yosh_belgilashtir(30u32);
    println!("{}", y1);
    println!("{}", y2);
    // 25
    // 30

    collection_from_misollari();

    option_result_from_misollari();

    // T: From<U>
    let wrapper: Wrapper = aylantir(42i32);
    let temperatura: Temperatura = aylantir(36.6f64);
    println!("{:?}", wrapper);
    println!("{:?}", temperatura);
    // Wrapper(42)
    // Temperatura { celsius: 36.6 }

    // Into<String> — birlash
    let birlashtirgan1: String = birlash("salom ", "dunyo");
    let birlashtirgan2: String = birlash(
        String::from("Rust "),
        String::from("tili"),
    );
    println!("{}", birlashtirgan1);
    println!("{}", birlashtirgan2);
    // salom dunyo
    // Rust tili

    // Metr ↔ Santimetr ↔ Millimetr
    let bir_metr = Metr(1.0);
    let ikki_yuz_sm = Santimetr(200.0);
    let besh_yuz_mm = Millimetr(500.0);

    let sm_dan_metr: Metr = Metr::from(ikki_yuz_sm);
    let mm_dan_metr: Metr = Metr::from(besh_yuz_mm);
    let metr_dan_sm: Santimetr = Santimetr::from(bir_metr);
    let mm_dan_sm: Santimetr = Santimetr::from(besh_yuz_mm);

    println!("{} = {}", ikki_yuz_sm, sm_dan_metr);
    println!("{} = {}", besh_yuz_mm, mm_dan_metr);
    println!("{} = {}", bir_metr, metr_dan_sm);
    println!("{} = {}", besh_yuz_mm, mm_dan_sm);
    // 200.0 sm = 2.000 m
    // 500 mm = 0.500 m
    // 1.000 m = 100.0 sm
    // 500 mm = 50.0 sm

    // into sintaksisi — yanada qulay
    // синтаксис into — ещё удобнее
    let uzunlik_metr: Metr = Santimetr(150.0).into();
    println!("{}", uzunlik_metr);
    // 1.500 m
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                          FROM TRAIT                                                                                          |
// #================================================================================================================================================#
// # |   1 | impl From<U> for T       | U dan T yaratish                                     | Создание T из U                                      |
// # |   2 | T::from(u)               | From ni chaqirish                                    | Вызов From                                           |
// # |   3 | From<i32>, From<f64>...  | Bir nechta From implement qilish                     | Реализация нескольких From                           |
// # |   4 | From<Struct>             | Custom struct dan aylantirish                        | Преобразование из пользовательской структуры         |
// # |   5 | String::from("...")      | Eng ko'p ishlatiladigan From                         | Наиболее часто используемый From                     |
// #================================================================================================================================================#
// # |                                          INTO TRAIT                                                                                          |
// #================================================================================================================================================#
// # |   6 | T: From<U> → U: Into<T> | From implement qilsak Into bepul keladi              | Реализуя From, Into достаётся бесплатно               |
// # |   7 | let x: T = u.into()     | Into ni chaqirish                                    | Вызов Into                                            |
// # |   8 | fn f(x: impl Into<T>)   | Funksiya parametrida Into — qulay                    | Into в параметре функции — удобно                     |
// #================================================================================================================================================#
// # |                                       INTEGER KONVERSIYA                                                                                     |
// #================================================================================================================================================#
// # |   9 | u8 → u16, u32, u64      | Kichikdan kattaga — xavfsiz                          | Из маленького в большой — безопасно                   |
// # |  10 | bool → i32, u8          | true=1, false=0                                      | true=1, false=0                                       |
// # |  11 | char → u32              | Unicode kod nuqtasi                                  | Кодовая точка unicode                                 |
// # |  12 | i32 → f64               | Integer → float                                      | Целое → с плавающей точкой                            |
// #================================================================================================================================================#
// # |                                       COLLECTION KONVERSIYA                                                                                  |
// #================================================================================================================================================#
// # |  13 | Vec::from(array)        | Array → Vec                                          | Массив → Vec                                          |
// # |  14 | Vec::from("str")        | &str → Vec<u8>                                       | &str → Vec<u8>                                        |
// # |  15 | Box::from(val)          | Qiymat → Box                                         | Значение → Box                                        |
// # |  16 | Rc::from(val)           | Qiymat → Rc                                          | Значение → Rc                                         |
// # |  17 | Arc::from(val)          | Qiymat → Arc                                         | Значение → Arc                                        |
// #================================================================================================================================================#
// # |                                       ENUM VA XATO BILAN                                                                                     |
// #================================================================================================================================================#
// # |  18 | impl From<E1> for MyErr | ? operatori uchun xato konversiyasi                  | Конвертация ошибок для оператора ?                    |
// # |  19 | ? operatori             | From orqali avtomatik xato konversiyasi              | Автоматическая конвертация ошибок через From          |
// #================================================================================================================================================#
// # |                                       GENERIC BILAN                                                                                          |
// #================================================================================================================================================#
// # |  20 | fn f<T: From<U>>(u: U)  | Generic From bound                                   | Generic ограничение From                              |
// # |  21 | fn f(x: impl Into<T>)   | impl Into — qulay API dizayn                         | impl Into — удобный дизайн API                        |
// #================================================================================================================================================#
// # |                                       MUHIM QOIDALAR                                                                                         |
// #================================================================================================================================================#
// # |  22 | From implement qil      | Into bepul keladi (teskari emas!)                    | Into достаётся бесплатно (не наоборот!)               |
// # |  23 | From — yo'qolishsiz     | Agar yo'qolish bo'lsa TryFrom ishlatiladi            | При потере данных используется TryFrom                |
// # |  24 | impl Into<T> yozma      | Doim impl From<U> for T yoz                          | Всегда пиши impl From<U> for T                        |
// # |  25 | Newtype + From          | O'lchov birliklari va type-safe API uchun            | Для единиц измерения и type-safe API                  |
// #================================================================================================================================================#