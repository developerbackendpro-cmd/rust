// #================================================================================================================================================#
// #                                                                ASSOCIATED TYPES                                                                #
// #                            ASSOCIATED TYPES — TRAIT ICHIDA TUR NOMI BERISH. GENERIC DAN FARQI: BIR MARTA ANIQLANADI.                           #
// #                            ASSOCIATED TYPES — ИМЕНОВАНИЕ ТИПА ВНУТРИ ТРЕЙТА. ОТЛИЧИЕ ОТ GENERIC: ЗАДАЁТСЯ ОДИН РАЗ.                            #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// associated type — trait ichida type kalit so'zi bilan
// associated type — с ключевым словом type внутри трейта
trait Konteyner {
    type Element;

    fn oldin_qo_sh(&mut self, element: Self::Element);
    fn oxirini_ol(&mut self) -> Option<Self::Element>;
    fn bo_shmi(&self) -> bool;
    fn uzunlik(&self) -> usize;
}

// Vec<i32> uchun Konteyner implement qilish
// реализация Konteyner для Vec<i32>
struct SonlarKonteyner {
    ichki: Vec<i32>,
}

impl SonlarKonteyner {
    fn new() -> Self {
        SonlarKonteyner { ichki: Vec::new() }
    }
}

impl Konteyner for SonlarKonteyner {
    type Element = i32;

    fn oldin_qo_sh(&mut self, element: i32) {
        self.ichki.push(element);
    }

    fn oxirini_ol(&mut self) -> Option<i32> {
        self.ichki.pop()
    }

    fn bo_shmi(&self) -> bool {
        self.ichki.is_empty()
    }

    fn uzunlik(&self) -> usize {
        self.ichki.len()
    }
}

// String uchun Konteyner implement qilish
// реализация Konteyner для String
struct MatnlarKonteyner {
    ichki: Vec<String>,
}

impl MatnlarKonteyner {
    fn new() -> Self {
        MatnlarKonteyner { ichki: Vec::new() }
    }
}

impl Konteyner for MatnlarKonteyner {
    type Element = String;

    fn oldin_qo_sh(&mut self, element: String) {
        self.ichki.push(element);
    }

    fn oxirini_ol(&mut self) -> Option<String> {
        self.ichki.pop()
    }

    fn bo_shmi(&self) -> bool {
        self.ichki.is_empty()
    }

    fn uzunlik(&self) -> usize {
        self.ichki.len()
    }
}

// Generic bilan — bir struct bir nechta implement qilishi mumkin
// С Generic — одна структура может иметь несколько реализаций
trait GenericQoshish<T> {
    fn qo_sh(&self, a: T, b: T) -> T;
}

struct Hisoblash;

impl GenericQoshish<i32> for Hisoblash {
    fn qo_sh(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

impl GenericQoshish<f64> for Hisoblash {
    fn qo_sh(&self, a: f64, b: f64) -> f64 {
        a + b
    }
}

impl GenericQoshish<String> for Hisoblash {
    fn qo_sh(&self, a: String, b: String) -> String {
        a + &b
    }
}

// Associated type bilan — bir struct bir marta implement qiladi
// С Associated type — одна структура реализует один раз
trait AssocQoshish {
    type Chiqish;
    fn qo_sh(&self, a: Self::Chiqish, b: Self::Chiqish) -> Self::Chiqish;
}

struct SonHisoblash;

impl AssocQoshish for SonHisoblash {
    type Chiqish = i32;

    fn qo_sh(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

// Iterator trait — eng mashhur associated type misoli
// Iterator trait — наиболее известный пример associated type
struct Hisoblagich {
    hozirgi: u32,
    max: u32,
}

impl Hisoblagich {
    fn new(max: u32) -> Self {
        Hisoblagich { hozirgi: 0, max }
    }
}

impl Iterator for Hisoblagich {
    // associated type — Item
    // associated type — Item
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.hozirgi < self.max {
            self.hozirgi += 1;
            Some(self.hozirgi)
        } else {
            None
        }
    }
}

// bir nechta associated type — graph misoli
// несколько associated types — пример графа
trait Graf {
    type Tugun;
    type Qirra;
    type Ogirlik;

    fn qo_shni_toping(&self, tugun: &Self::Tugun) -> Vec<Self::Tugun>;
    fn qirra_og_irligi(&self, qirra: &Self::Qirra) -> Self::Ogirlik;
}

// oddiy graf implement qilish
// реализация простого графа
#[derive(Debug, Clone, PartialEq)]
struct GrafTugun {
    id: u32,
    nomi: String,
}

#[derive(Debug, Clone)]
struct GrafQirra {
    dan: u32,
    ga: u32,
}

struct OddiyGraf {
    tugunlar: Vec<GrafTugun>,
    qirralar: Vec<GrafQirra>,
}

impl Graf for OddiyGraf {
    type Tugun = GrafTugun;
    type Qirra = GrafQirra;
    type Ogirlik = f64;

    fn qo_shni_toping(&self, tugun: &GrafTugun) -> Vec<GrafTugun> {
        self.qirralar.iter()
            .filter(|q| q.dan == tugun.id)
            .filter_map(|q| self.tugunlar.iter().find(|t| t.id == q.ga))
            .cloned()
            .collect()
    }

    fn qirra_og_irligi(&self, _qirra: &GrafQirra) -> f64 {
        1.0
    }
}

// Self::Element ga bound qo'shish
// добавление bound к Self::Element
trait ChiqaruvchiKonteyner {
    type Element: fmt::Display + Clone;

    fn birinchisini_ol(&self) -> Option<&Self::Element>;
    fn hammasini_chiqar(&self);
}

struct Korsatuvchi<T: fmt::Display + Clone> {
    elementlar: Vec<T>,
}

impl<T: fmt::Display + Clone> ChiqaruvchiKonteyner for Korsatuvchi<T> {
    type Element = T;

    fn birinchisini_ol(&self) -> Option<&T> {
        self.elementlar.first()
    }

    fn hammasini_chiqar(&self) {
        for el in &self.elementlar {
            print!("{} ", el);
        }
        println!();
    }
}

// where clause — associated type ga bound
// where clause — bound для associated type
fn birinchi_elementni_chiqar<K>(konteyner: &K)
where
    K: ChiqaruvchiKonteyner,
    K::Element: fmt::Debug,
{
    match konteyner.birinchisini_ol() {
        Some(el) => println!("Birinchi: {:?}", el),
        None      => println!("Bo'sh!"),
    }
}

// Add trait — associated type Output
// трейт Add — associated type Output
use std::ops::{Add, Mul, Neg};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Kompleks {
    real: f64,
    xayoliy: f64,
}

impl Kompleks {
    fn new(real: f64, xayoliy: f64) -> Self {
        Kompleks { real, xayoliy }
    }

    fn modul(&self) -> f64 {
        (self.real * self.real + self.xayoliy * self.xayoliy).sqrt()
    }
}

impl fmt::Display for Kompleks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.xayoliy >= 0.0 {
            write!(f, "{} + {}i", self.real, self.xayoliy)
        } else {
            write!(f, "{} - {}i", self.real, -self.xayoliy)
        }
    }
}

impl Add for Kompleks {
    // Output — associated type
    type Output = Kompleks;

    fn add(self, other: Kompleks) -> Kompleks {
        Kompleks {
            real: self.real + other.real,
            xayoliy: self.xayoliy + other.xayoliy,
        }
    }
}

impl Mul for Kompleks {
    type Output = Kompleks;

    fn mul(self, other: Kompleks) -> Kompleks {
        Kompleks {
            real: self.real * other.real - self.xayoliy * other.xayoliy,
            xayoliy: self.real * other.xayoliy + self.xayoliy * other.real,
        }
    }
}

impl Neg for Kompleks {
    type Output = Kompleks;

    fn neg(self) -> Kompleks {
        Kompleks {
            real: -self.real,
            xayoliy: -self.xayoliy,
        }
    }
}

// associated type — builder pattern
// associated type — паттерн строитель
trait Yasovchi {
    type Mahsulot;
    type Xato;

    fn qur(&self) -> Result<Self::Mahsulot, Self::Xato>;
}

#[derive(Debug)]
struct ServerKonfiguratsiya {
    port: u16,
    host: String,
    max_ulanish: u32,
}

#[derive(Debug)]
enum ServerXato {
    NotogriiPort,
    HostBosh,
}

impl fmt::Display for ServerXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServerXato::NotogriiPort => write!(f, "Port 1024-65535 oralig'ida bo'lishi kerak"),
            ServerXato::HostBosh   => write!(f, "Host bo'sh bo'lishi mumkin emas"),
        }
    }
}

struct ServerYasovchi {
    port: u16,
    host: String,
    max_ulanish: u32,
}

impl ServerYasovchi {
    fn new() -> Self {
        ServerYasovchi {
            port: 8080,
            host: String::from("localhost"),
            max_ulanish: 100,
        }
    }

    fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }
}

impl Yasovchi for ServerYasovchi {
    type Mahsulot = ServerKonfiguratsiya;
    type Xato = ServerXato;

    fn qur(&self) -> Result<ServerKonfiguratsiya, ServerXato> {
        if self.port < 1024 {
            return Err(ServerXato::NotogriiPort);
        }
        if self.host.is_empty() {
            return Err(ServerXato::HostBosh);
        }
        Ok(ServerKonfiguratsiya {
            port: self.port,
            host: self.host.clone(),
            max_ulanish: self.max_ulanish,
        })
    }
}

// associated type ga qo'shimcha bound
// дополнительный bound для associated type
fn iteratsiya_qil<I>(iterator: I)
where
    I: Iterator,
    I::Item: fmt::Display,
{
    for element in iterator {
        print!("{} ", element);
    }
    println!();
}

fn yig_indini_hisoblash<I>(iterator: I) -> i64
where
    I: Iterator<Item = i64>,
{
    iterator.sum()
}

fn main() {

    // SonlarKonteyner — associated type = i32
    // SonlarKonteyner — associated type = i32
    let mut sonlar = SonlarKonteyner::new();
    sonlar.oldin_qo_sh(10);
    sonlar.oldin_qo_sh(20);
    sonlar.oldin_qo_sh(30);
    println!("Uzunlik: {}", sonlar.uzunlik());
    println!("Oxirgi: {:?}", sonlar.oxirini_ol());
    println!("Bo'shmi: {}", sonlar.bo_shmi());
    // Uzunlik: 3
    // Oxirgi: Some(30)
    // Bo'shmi: false

    // MatnlarKonteyner — associated type = String
    // MatnlarKonteyner — associated type = String
    let mut matnlar = MatnlarKonteyner::new();
    matnlar.oldin_qo_sh(String::from("salom"));
    matnlar.oldin_qo_sh(String::from("dunyo"));
    println!("Uzunlik: {}", matnlar.uzunlik());
    println!("Oxirgi: {:?}", matnlar.oxirini_ol());
    // Uzunlik: 2
    // Oxirgi: Some("dunyo")

    // generic — bir nechta implement
    // generic — несколько реализаций
    let hisob = Hisoblash;
    let son_natija: i32 = hisob.qo_sh(10, 20);
    let float_natija: f64 = hisob.qo_sh(1.5, 2.5);
    let matn_natija: String = hisob.qo_sh(
        String::from("salom "),
        String::from("dunyo")
    );
    println!("{}", son_natija);
    println!("{}", float_natija);
    println!("{}", matn_natija);
    // 30
    // 4
    // salom dunyo

    // associated type — bitta implement
    // associated type — одна реализация
    let son_hisob = SonHisoblash;
    let assoc_natija: i32 = son_hisob.qo_sh(5, 7);
    println!("{}", assoc_natija);
    // 12

    // Hisoblagich — custom iterator
    // Hisoblagich — пользовательский итератор
    let hisoblagich = Hisoblagich::new(5);
    let elementlar: Vec<u32> = hisoblagich.collect();
    println!("{:?}", elementlar);
    // [1, 2, 3, 4, 5]

    // iterator metodlari — associated type Item ishlatiladi
    // методы итератора — используется associated type Item
    let hisoblagich2 = Hisoblagich::new(10);
    let juftlar: Vec<u32> = hisoblagich2
        .filter(|&x| x % 2 == 0)
        .collect();
    println!("{:?}", juftlar);
    // [2, 4, 6, 8, 10]

    // yig'indi
    // сумма
    let hisoblagich3 = Hisoblagich::new(5);
    let yig_indi: u32 = hisoblagich3.sum();
    println!("{}", yig_indi);
    // 15

    // zip — ikki iterator
    // zip — два итератора
    let h1 = Hisoblagich::new(3);
    let h2 = Hisoblagich::new(3);
    let juftliklar: Vec<(u32, u32)> = h1.zip(h2).collect();
    println!("{:?}", juftliklar);
    // [(1, 1), (2, 2), (3, 3)]

    // graf yaratish va ishlatish
    // создание и использование графа
    let graf = OddiyGraf {
        tugunlar: vec![
            GrafTugun { id: 1, nomi: String::from("A") },
            GrafTugun { id: 2, nomi: String::from("B") },
            GrafTugun { id: 3, nomi: String::from("C") },
        ],
        qirralar: vec![
            GrafQirra { dan: 1, ga: 2 },
            GrafQirra { dan: 1, ga: 3 },
            GrafQirra { dan: 2, ga: 3 },
        ],
    };

    let a_tuguni = &graf.tugunlar[0];
    let qo_shnilar = graf.qo_shni_toping(a_tuguni);
    println!("A ning qo'shnilari:");
    for q in &qo_shnilar {
        println!("  {:?}", q);
    }
    // A ning qo'shnilari:
    //   GrafTugun { id: 2, nomi: "B" }
    //   GrafTugun { id: 3, nomi: "C" }

    // ChiqaruvchiKonteyner — Element: Display + Clone
    // ChiqaruvchiKonteyner — Element: Display + Clone
    let ko_rsatuvchi = Korsatuvchi {
        elementlar: vec![1, 2, 3, 4, 5],
    };
    ko_rsatuvchi.hammasini_chiqar();
    println!("{:?}", ko_rsatuvchi.birinchisini_ol());
    // 1 2 3 4 5
    // Some(1)

    let matn_ko_rsatuvchi = Korsatuvchi {
        elementlar: vec!["salom", "dunyo", "rust"],
    };
    matn_ko_rsatuvchi.hammasini_chiqar();
    // salom dunyo rust

    // where clause bilan associated type
    // associated type с where clause
    birinchi_elementni_chiqar(&ko_rsatuvchi);
    birinchi_elementni_chiqar(&matn_ko_rsatuvchi);
    // Birinchi: 1
    // Birinchi: "salom"

    // Kompleks son — Add, Mul, Neg
    // Комплексное число — Add, Mul, Neg
    let k1 = Kompleks::new(3.0, 4.0);
    let k2 = Kompleks::new(1.0, -2.0);

    let qo_shma: Kompleks = k1 + k2;
    let ko_paytma: Kompleks = k1 * k2;
    let inkor: Kompleks = -k1;

    println!("k1 = {}", k1);
    println!("k2 = {}", k2);
    println!("k1 + k2 = {}", qo_shma);
    println!("k1 * k2 = {}", ko_paytma);
    println!("-k1 = {}", inkor);
    println!("|k1| = {:.2}", k1.modul());
    // k1 = 3 + 4i
    // k2 = 1 - 2i
    // k1 + k2 = 4 + 2i
    // k1 * k2 = 11 - 2i
    // -k1 = -3 - 4i
    // |k1| = 5.00

    // muvaffaqiyatli server
    // успешный сервер
    let server_yasovchi = ServerYasovchi::new()
        .port(3000)
        .host("example.com");

    let server_natija = server_yasovchi.qur();
    println!("{:#?}", server_natija);
    // Ok(
    //     ServerKonfiguratsiya {
    //         port: 3000,
    //         host: "example.com",
    //         max_ulanish: 100,
    //     },
    // )

    // xatolik — notogri port
    // ошибка — неправильный порт
    let xato_yasovchi = ServerYasovchi::new().port(80);
    let xato_natija = xato_yasovchi.qur();
    println!("{:?}", xato_natija);
    // Err(NotogriiPort)

    // iteratsiya_qil — I::Item: Display
    // iteratsiya_qil — I::Item: Display
    let sonlar_iter = vec![10, 20, 30, 40, 50];
    iteratsiya_qil(sonlar_iter.iter());
    // 10 20 30 40 50

    let matnlar_iter = vec!["salom", "dunyo", "rust"];
    iteratsiya_qil(matnlar_iter.iter());
    // salom dunyo rust

    // yig_indi — Item = i64
    // сумма — Item = i64
    let i64_sonlar: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let yig_indi: i64 = yig_indini_hisoblash(i64_sonlar.into_iter());
    println!("Yig'indi: {}", yig_indi);
    // Yig'indi: 55
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                       ASSOCIATED TYPE ASOSLARI                                                                               |
// #================================================================================================================================================#
// # |   1 | type Nomi;               | Trait ichida associated type e'lon qilish             | Объявление associated type внутри трейта            |
// # |   2 | type Nomi = T;           | Implement qilishda aniqlashtirish                     | Конкретизация при реализации                        |
// # |   3 | Self::Nomi               | Associated type ga murojaat qilish                   | Обращение к associated type                          |
// # |   4 | K::Element               | Trait bound orqali associated type ga murojaat       | Обращение через ограничение трейта                   |
// #================================================================================================================================================#
// # |                                    GENERIC VS ASSOCIATED TYPE                                                                                |
// #================================================================================================================================================#
// # |   5 | trait T<G>               | Generic — bir struct ko'p implement qilishi mumkin   | Generic — одна структура может иметь много реализаций|
// # |   6 | trait T { type A; }      | Associated — bir struct bir marta implement qiladi   | Associated — одна структура реализует один раз       |
// # |   7 | Iterator<Item=T>         | Associated type ga aniq tur berish                   | Указание конкретного типа для associated type        |
// #================================================================================================================================================#
// # |                                     STANDART TRAITLAR                                                                                        |
// #================================================================================================================================================#
// # |   8 | Iterator::Item           | Har bir elementning turi                             | Тип каждого элемента                                 |
// # |   9 | Add::Output              | Qo'shish natijasining turi                           | Тип результата сложения                              |
// # |  10 | Mul::Output              | Ko'paytirish natijasining turi                       | Тип результата умножения                             |
// # |  11 | Neg::Output              | Inkor natijasining turi                              | Тип результата отрицания                             |
// #================================================================================================================================================#
// # |                                        BOUND BILAN                                                                                           |
// #================================================================================================================================================#
// # |  12 | type E: Display + Clone  | Associated type ga bound qo'shish                    | Добавление bound к associated type                   |
// # |  13 | where I::Item: Display   | Where clause bilan associated type bound             | Bound associated type через where clause             |
// # |  14 | Iterator<Item=i64>       | Aniq associated type bilan generic bound             | Generic bound с конкретным associated type           |
// #================================================================================================================================================#
// # |                                       QACHON ISHLATISH                                                                                       |
// #================================================================================================================================================#
// # |  15 | Associated type          | Bir tur bilan ishlaydigan trait                      | Трейт работающий с одним типом                       |
// # |  16 | Generic                  | Bir nechta tur bilan ishlaydigan trait               | Трейт работающий с несколькими типами                |
// # |  17 | Iterator, Add, Mul       | Standart traitlar associated type ishlatadi          | Стандартные трейты используют associated type        |
// #================================================================================================================================================#