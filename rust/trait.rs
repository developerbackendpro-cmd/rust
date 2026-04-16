// #================================================================================================================================================#
// #                                                                TRAIT                                                                           #
// #                                    TRAIT — UMUMIY XULQ-ATVOR SHARTNOMASI. RUST DA INTERFEYSNING ASOSI.                                         #
// #                                    TRAIT — КОНТРАКТ ОБЩЕГО ПОВЕДЕНИЯ. ОСНОВА ИНТЕРФЕЙСОВ В RUST.                                               #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

trait Salomlash {
    fn salom(&self) -> String;
}

struct Uzbek {
    ism: String,
}

struct Ingliz {
    name: String,
}

impl Salomlash for Uzbek {
    fn salom(&self) -> String {
        format!("Assalomu alaykum, {}!", self.ism)
    }
}

impl Salomlash for Ingliz {
    fn salom(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

trait Hayvon {
    fn ism(&self) -> &str;

    // default metod — override qilish mumkin
    // метод по умолчанию — можно переопределить
    fn tavsif(&self) -> String {
        format!("Bu {} degan hayvon", self.ism())
    }

    // default metod — boshqa metodlardan foydalanadi
    // метод по умолчанию — использует другие методы
    fn salom_bering(&self) -> String {
        format!("Salom! Men {}", self.ism())
    }
}

struct It {
    nomi: String,
}

struct Mushuk {
    nomi: String,
}

impl Hayvon for It {
    fn ism(&self) -> &str {
        &self.nomi
    }
    // tavsif() — default ishlatiladi
    // tavsif() — используется по умолчанию
}

impl Hayvon for Mushuk {
    fn ism(&self) -> &str {
        &self.nomi
    }

    // tavsif() — override qilinadi
    // tavsif() — переопределяется
    fn tavsif(&self) -> String {
        format!("Men {} — mustaqil mushukman!", self.nomi)
    }
}

// MUHIM: dyn Trait uchun const ruxsat yo'q — fn ishlatiladi
// ВАЖНО: для dyn Trait const запрещён — используется fn
trait Shakl {
    fn nomi(&self) -> &'static str;
    fn yuza(&self) -> f64;
    fn perimetr(&self) -> f64;

    // default metod
    // метод по умолчанию
    fn tavsif(&self) -> String {
        format!(
            "{}: yuza={:.2}, perimetr={:.2}",
            self.nomi(),
            self.yuza(),
            self.perimetr()
        )
    }
}

struct Turtburchak {
    eni: f64,
    boyi: f64,
}

struct Doira {
    radius: f64,
}

impl Shakl for Turtburchak {
    fn nomi(&self) -> &'static str { "Turtburchak" }
    fn yuza(&self) -> f64 { self.eni * self.boyi }
    fn perimetr(&self) -> f64 { 2.0 * (self.eni + self.boyi) }
}

impl Shakl for Doira {
    fn nomi(&self) -> &'static str { "Doira" }
    fn yuza(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimetr(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
}

// associated constant — alohida trait da (dyn bilan ishlatilmaydi)
// ассоциированная константа — в отдельном трейте (не используется с dyn)
trait ShaklNomi {
    const NOMI: &'static str;
}

impl ShaklNomi for Turtburchak {
    const NOMI: &'static str = "Turtburchak";
}

impl ShaklNomi for Doira {
    const NOMI: &'static str = "Doira";
}

// trait bound — generic bilan
// ограничение трейта — с generic
fn eng_katta<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// bir nechta trait bound — + bilan
// несколько ограничений — через +
fn chiqar<T: fmt::Display + fmt::Debug>(qiymat: T) {
    println!("Display: {}", qiymat);
    println!("Debug: {:?}", qiymat);
}

// where clause — murakkab boundlar uchun
// where clause — для сложных ограничений
fn solishtir_va_chiqar<T, U>(t: T, u: U)
where
    T: fmt::Display + PartialOrd,
    U: fmt::Display + PartialOrd,
{
    println!("t = {}, u = {}", t, u);
}

// impl Trait — argument sifatida
// impl Trait — как аргумент
fn salom_ayt(narsa: &impl Salomlash) {
    println!("{}", narsa.salom());
}

// impl Trait — qaytarish qiymati sifatida
// impl Trait — как возвращаемое значение
fn uzbek_yasash() -> impl Salomlash {
    Uzbek { ism: String::from("Dilshod") }
}

// trait meros — supertrait
// наследование трейтов — супертрейт
trait Chizish: fmt::Display {
    fn chiz(&self);
}

struct Kvadrat {
    tomon: f64,
}

impl fmt::Display for Kvadrat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kvadrat({})", self.tomon)
    }
}

impl Chizish for Kvadrat {
    fn chiz(&self) {
        println!("Chizilmoqda: {}", self);
    }
}

// blanket impl — barcha T: Display uchun
// обобщённая реализация — для всех T: Display
trait Xabar {
    fn xabar(&self) -> String;
}

impl<T: fmt::Display> Xabar for T {
    fn xabar(&self) -> String {
        format!("Xabar: {}", self)
    }
}

use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vektor2D {
    x: f64,
    y: f64,
}

impl Add for Vektor2D {
    type Output = Vektor2D;

    fn add(self, other: Vektor2D) -> Vektor2D {
        Vektor2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Vektor2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// marker trait — metod yo'q, faqat belgi
// маркерный трейт — без методов, только метка
trait Serializable {}
trait Cacheable {}

struct Foydalanuvchi {
    ism: String,
    yosh: u32,
}

impl Serializable for Foydalanuvchi {}
impl Cacheable for Foydalanuvchi {}

fn saqlash<T: Serializable>(_qiymat: &T) {
    println!("Saqlanmoqda...");
}

fn keshla<T: Cacheable>(_qiymat: &T) {
    println!("Keshlanmoqda...");
}

// generic — compile time, zero cost
// generic — время компиляции, без затрат
fn generic_salom<T: Salomlash>(narsa: &T) {
    println!("{}", narsa.salom());
}

// trait object — runtime, dinamik dispatch
// объект трейта — время выполнения, динамическая диспетчеризация
fn dynamic_salom(narsa: &dyn Salomlash) {
    println!("{}", narsa.salom());
}

// Vec<Box<dyn Trait>> — turli turlarni birga saqlash
// Vec<Box<dyn Trait>> — хранение разных типов вместе
fn turli_shakllar() {
    let shakllar: Vec<Box<dyn Shakl>> = vec![
        Box::new(Turtburchak { eni: 4.0, boyi: 3.0 }),
        Box::new(Doira { radius: 5.0 }),
        Box::new(Turtburchak { eni: 2.0, boyi: 2.0 }),
    ];

    for shakl in &shakllar {
        println!("{}", shakl.tavsif());
    }
}

struct Juft<T> {
    birinchi: T,
    ikkinchi: T,
}

// faqat T: fmt::Display bo'lganda Display implement qilinadi
// Display реализуется только когда T: fmt::Display
impl<T: fmt::Display> fmt::Display for Juft<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.birinchi, self.ikkinchi)
    }
}

// faqat T: PartialOrd + fmt::Display bo'lganda metod qo'shiladi
// метод добавляется только когда T: PartialOrd + fmt::Display
impl<T: PartialOrd + fmt::Display> Juft<T> {
    fn eng_kattasini_chiqar(&self) {
        if self.birinchi >= self.ikkinchi {
            println!("Kattasi: {}", self.birinchi);
        } else {
            println!("Kattasi: {}", self.ikkinchi);
        }
    }
}

fn main() {

    // trait implement qilish va ishlatish
    // реализация и использование трейта
    let uzbek = Uzbek { ism: String::from("Dilshod") };
    let ingliz = Ingliz { name: String::from("John") };
    println!("{}", uzbek.salom());
    println!("{}", ingliz.salom());
    // Assalomu alaykum, Dilshod!
    // Hello, John!

    // It — default tavsif ishlatadi
    // It — использует tavsif по умолчанию
    let it = It { nomi: String::from("Rex") };
    let mushuk = Mushuk { nomi: String::from("Mur") };
    println!("{}", it.tavsif());
    println!("{}", mushuk.tavsif());
    println!("{}", it.salom_bering());
    // Bu Rex degan hayvon
    // Men Mur — mustaqil mushukman!
    // Salom! Men Rex

    // fn orqali nom olish
    // получение имени через fn
    let turtburchak = Turtburchak { eni: 4.0, boyi: 3.0 };
    let doira = Doira { radius: 5.0 };
    println!("{}", turtburchak.tavsif());
    println!("{}", doira.tavsif());
    // Turtburchak: yuza=12.00, perimetr=14.00
    // Doira: yuza=78.54, perimetr=31.42

    // associated constant — alohida trait orqali
    // ассоциированная константа — через отдельный трейт
    println!("Shakl nomi: {}", Turtburchak::NOMI);
    println!("Shakl nomi: {}", Doira::NOMI);
    // Shakl nomi: Turtburchak
    // Shakl nomi: Doira

    // generic trait bound
    // ограничение generic трейта
    let katta_son: i32 = eng_katta(10, 20);
    let katta_float: f64 = eng_katta(3.14, 2.72);
    println!("{}", katta_son);
    println!("{}", katta_float);
    // 20
    // 3.14

    // bir nechta trait bound
    // несколько ограничений трейта
    chiqar(42i32);
    // Display: 42
    // Debug: 42

    // where clause
    solishtir_va_chiqar(10i32, 20i32);
    // t = 10, u = 20

    // impl Trait — argument
    // impl Trait — аргумент
    salom_ayt(&uzbek);
    salom_ayt(&ingliz);
    // Assalomu alaykum, Dilshod!
    // Hello, John!

    // impl Trait — qaytarish
    // impl Trait — возвращение
    let yangi_uzbek = uzbek_yasash();
    println!("{}", yangi_uzbek.salom());
    // Assalomu alaykum, Dilshod!

    // supertrait ishlatish
    // использование супертрейта
    let kvadrat = Kvadrat { tomon: 5.0 };
    kvadrat.chiz();
    println!("{}", kvadrat);
    // Chizilmoqda: Kvadrat(5)
    // Kvadrat(5)

    // blanket impl — barcha Display turlar
    // обобщённая реализация — все Display типы
    let son_xabar: String = 42i32.xabar();
    let matn_xabar: String = "salom".xabar();
    let float_xabar: String = 3.14f64.xabar();
    println!("{}", son_xabar);
    println!("{}", matn_xabar);
    println!("{}", float_xabar);
    // Xabar: 42
    // Xabar: salom
    // Xabar: 3.14

    // Vektor2D — qo'shish
    // Vektor2D — сложение
    let v1 = Vektor2D { x: 1.0, y: 2.0 };
    let v2 = Vektor2D { x: 3.0, y: 4.0 };
    let v3: Vektor2D = v1 + v2;
    println!("{}", v3);
    println!("{:?}", v3);
    // (4, 6)
    // Vektor2D { x: 4.0, y: 6.0 }

    // marker trait bilan ishlash
    // работа с маркерным трейтом
    let foydalanuvchi = Foydalanuvchi {
        ism: String::from("Dilshod"),
        yosh: 22,
    };
    saqlash(&foydalanuvchi);
    keshla(&foydalanuvchi);
    // Saqlanmoqda...
    // Keshlanmoqda...

    // generic — compile time dispatch
    // generic — диспетчеризация во время компиляции
    let uzbek2 = Uzbek { ism: String::from("Ali") };
    generic_salom(&uzbek2);
    // Assalomu alaykum, Ali!

    // trait object — runtime dispatch
    // объект трейта — диспетчеризация во время выполнения
    let salomlashuvchilar: Vec<Box<dyn Salomlash>> = vec![
        Box::new(Uzbek { ism: String::from("Vali") }),
        Box::new(Ingliz { name: String::from("Bob") }),
        Box::new(Uzbek { ism: String::from("Soli") }),
    ];
    for narsa in &salomlashuvchilar {
        dynamic_salom(narsa.as_ref());
    }
    // Assalomu alaykum, Vali!
    // Hello, Bob!
    // Assalomu alaykum, Soli!

    // turli shakllar — dyn Shakl
    // разные фигуры — dyn Shakl
    turli_shakllar();
    // Turtburchak: yuza=12.00, perimetr=14.00
    // Doira: yuza=78.54, perimetr=31.42
    // Turtburchak: yuza=4.00, perimetr=8.00

    // conditional impl — faqat T: Display bo'lganda
    // условная реализация — только когда T: Display
    let juft_son: Juft<i32> = Juft { birinchi: 10, ikkinchi: 20 };
    let juft_matn: Juft<&str> = Juft { birinchi: "salom", ikkinchi: "dunyo" };
    println!("{}", juft_son);
    println!("{}", juft_matn);
    juft_son.eng_kattasini_chiqar();
    // (10, 20)
    // (salom, dunyo)
    // Kattasi: 20
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                          TRAIT ASOSLARI                                                                                      |
// #================================================================================================================================================#
// # |   1 | trait Nomi { }           | Trait e'lon qilish                                   | Объявление трейта                                    |
// # |   2 | impl Trait for Tur       | Trait implement qilish                               | Реализация трейта                                    |
// # |   3 | fn metod(&self)          | Majburiy metod (implement kerak)                     | Обязательный метод (нужна реализация)                |
// # |   4 | fn metod(&self) { ... }  | Default metod (override mumkin)                      | Метод по умолчанию (можно переопределить)            |
// # |   5 | const NOMI: T            | Associated constant (dyn bilan emas!)                | Ассоциированная константа (не с dyn!)                |
// #================================================================================================================================================#
// # |                                          TRAIT BOUND                                                                                         |
// #================================================================================================================================================#
// # |   6 | fn f<T: Trait>(x: T)     | Generic trait bound                                  | Ограничение generic трейта                           |
// # |   7 | T: Trait1 + Trait2       | Bir nechta trait bound                               | Несколько ограничений трейта                         |
// # |   8 | where T: Trait           | Where clause — murakkab boundlar                     | Where clause — для сложных ограничений               |
// # |   9 | fn f(x: &impl Trait)     | impl Trait — argument sifatida                       | impl Trait — как аргумент                            |
// # |  10 | fn f() -> impl Trait     | impl Trait — qaytarish qiymati                       | impl Trait — как возвращаемое значение               |
// #================================================================================================================================================#
// # |                                       TRAIT INHERITANCE                                                                                      |
// #================================================================================================================================================#
// # |  11 | trait A: B               | A — B ning subtraiti (supertrait)                    | A — субтрейт B (супертрейт)                          |
// # |  12 | trait A: B + C           | Bir nechta supertrait                                | Несколько супертрейтов                               |
// #================================================================================================================================================#
// # |                                     BLANKET IMPLEMENTATION                                                                                   |
// #================================================================================================================================================#
// # |  13 | impl<T: Trait> X for T   | Barcha T: Trait uchun implement                      | Реализация для всех T: Trait                         |
// #================================================================================================================================================#
// # |                                       TRAIT OBJECT                                                                                           |
// #================================================================================================================================================#
// # |  14 | &dyn Trait               | Runtime dispatch — trait object                      | Диспетчеризация во время выполнения                  |
// # |  15 | Box<dyn Trait>           | Heap da trait object                                 | Объект трейта в куче                                 |
// # |  16 | Vec<Box<dyn Trait>>      | Turli turlarni birga saqlash                         | Хранение разных типов вместе                         |
// # |  17 | const — dyn bilan emas   | dyn Trait uchun const ruxsat yo'q, fn ishlat         | const запрещён для dyn Trait, используй fn           |
// #================================================================================================================================================#
// # |                                         QOLGANLAR                                                                                            |
// #================================================================================================================================================#
// # |  18 | marker trait             | Metod yo'q, faqat belgi                              | Без методов, только метка                            |
// # |  19 | impl<T: A> X for Juft<T> | Conditional implementation                           | Условная реализация                                  |
// # |  20 | impl ops::Add for T      | Operator overloading traitlar orqali                 | Перегрузка операторов через трейты                   |
// # |  21 | generic vs dyn Trait     | Generic=compile time, dyn=runtime                    | Generic=компиляция, dyn=выполнение                   |
// #================================================================================================================================================#