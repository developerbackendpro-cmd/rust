// #================================================================================================================================================#
// #                                             DEFAULT  |  PARTIALEQ  |  EQ  |  PARTIALORD  |  ORD                                                #
// #                                BU TRAITLAR — RUST DA TAQQOSLASH VA STANDART QIYMAT BERISH UCHUN ASOSIY VOSITALAR.                              #
// #                                ЭТИ ТРЕЙТЫ — ОСНОВНЫЕ ИНСТРУМЕНТЫ ДЛЯ СРАВНЕНИЯ И ЗНАЧЕНИЙ ПО УМОЛЧАНИЮ В RUST.                                 #
// #================================================================================================================================================#

#![allow(dead_code, unused)]
use std::cmp::Ordering;

// #[derive(Default)] — avtomatik Default
// автоматический Default
#[derive(Debug, Default)]
struct Konfiguratsiya {
    port: u16,
    host: String,
    debug: bool,
    ulanishlar: u32,
}

// Default ni qo'lda implement qilish
// ручная реализация Default
#[derive(Debug)]
struct Server {
    port: u16,
    host: String,
    timeout: u64,
    max_ulanish: u32,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            port: 8080,
            host: String::from("localhost"),
            timeout: 30,
            max_ulanish: 100,
        }
    }
}

// Default — enum uchun (qo'lda)
// Default для enum (вручную)
#[derive(Debug, PartialEq)]
enum Rejim {
    Oddiy,
    Tez,
    Sekin,
}

impl Default for Rejim {
    fn default() -> Self {
        Rejim::Oddiy
    }
}

// #[derive(PartialEq)] — avtomatik == va !=
// автоматические == и !=
#[derive(Debug, PartialEq)]
struct Nuqta {
    x: f64,
    y: f64,
}

// #[derive(PartialEq, Eq)] — to'liq tenglik
// полное равенство
#[derive(Debug, PartialEq, Eq, Hash)]
struct UserId(u64);

// PartialEq ni qo'lda implement qilish
// ручная реализация PartialEq
#[derive(Debug)]
struct Talaba {
    ism: String,
    id: u32,
    baho: f64,
}

impl PartialEq for Talaba {
    fn eq(&self, other: &Self) -> bool {
        // faqat id bo'yicha tenglikni tekshirish
        // проверка равенства только по id
        self.id == other.id
    }
}

// Eq — PartialEq ni to'ldiradi (reflexive)
// Eq дополняет PartialEq (рефлексивность)
impl Eq for Talaba {}

// #[derive(PartialOrd, Ord)] — avtomatik tartiblash
// автоматическая сортировка
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Versiya {
    major: u32,
    minor: u32,
    patch: u32,
}

// PartialOrd ni qo'lda implement qilish
// ручная реализация PartialOrd
#[derive(Debug, PartialEq)]
struct Mahsulot {
    nomi: String,
    narx: f64,
    reyting: f32,
}

impl PartialOrd for Mahsulot {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // narx bo'yicha taqqoslash
        // сравнение по цене
        self.narx.partial_cmp(&other.narx)
    }
}

// Ord ni qo'lda implement qilish
// ручная реализация Ord
#[derive(Debug, PartialEq, Eq)]
struct Xodim {
    ism: String,
    yosh: u32,
    maosh: u64,
}

impl PartialOrd for Xodim {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Xodim {
    fn cmp(&self, other: &Self) -> Ordering {
        // maosh bo'yicha, teng bo'lsa yosh bo'yicha
        // по зарплате, при равенстве по возрасту
        self.maosh.cmp(&other.maosh)
            .then(self.yosh.cmp(&other.yosh))
    }
}

fn main() {

    // derive Default — barcha fieldlar 0/""/false
    // все поля 0/""/false
    let konfiguratsiya: Konfiguratsiya = Konfiguratsiya::default();
    println!("{:#?}", konfiguratsiya);
    // Konfiguratsiya {
    //     port: 0,
    //     host: "",
    //     debug: false,
    //     ulanishlar: 0,
    // }

    // Default — struct update syntax bilan
    // Default с синтаксисом обновления структуры
    let maxsus_konfiguratsiya = Konfiguratsiya {
        port: 3000,
        debug: true,
        ..Default::default()
    };
    println!("{:#?}", maxsus_konfiguratsiya);
    // Konfiguratsiya {
    //     port: 3000,
    //     host: "",
    //     debug: true,
    //     ulanishlar: 0,
    // }

    // qo'lda Default — ma'noli qiymatlar
    // ручной Default — осмысленные значения
    let server: Server = Server::default();
    println!("{:#?}", server);
    // Server {
    //     port: 8080,
    //     host: "localhost",
    //     timeout: 30,
    //     max_ulanish: 100,
    // }

    // Server — ba'zi fieldlarni o'zgartirish
    // изменение некоторых полей Server
    let maxsus_server = Server {
        port: 443,
        host: String::from("example.com"),
        ..Server::default()
    };
    println!("{:#?}", maxsus_server);
    // Server {
    //     port: 443,
    //     host: "example.com",
    //     timeout: 30,
    //     max_ulanish: 100,
    // }

    // enum Default
    let rejim: Rejim = Rejim::default();
    println!("{:?}", rejim);
    // Oddiy

    // built-in turlar Default
    // встроенные типы Default
    let son: i32 = Default::default();
    let matn: String = Default::default();
    let bool_qiymati: bool = Default::default();
    let vektor: Vec<i32> = Default::default();
    let opsiya: Option<i32> = Default::default();
    println!("{}", son);
    println!("\"{}\"", matn);
    println!("{}", bool_qiymati);
    println!("{:?}", vektor);
    println!("{:?}", opsiya);
    // 0
    // ""
    // false
    // []
    // None

    // == va != operatorlari
    // операторы == и !=
    let nuqta1 = Nuqta { x: 1.0, y: 2.0 };
    let nuqta2 = Nuqta { x: 1.0, y: 2.0 };
    let nuqta3 = Nuqta { x: 3.0, y: 4.0 };
    println!("{}", nuqta1 == nuqta2);
    println!("{}", nuqta1 == nuqta3);
    println!("{}", nuqta1 != nuqta3);
    // true
    // false
    // true

    // f64 — PartialEq bor, Eq yo'q (NaN sabab)
    // f64 — есть PartialEq, нет Eq (из-за NaN)
    let nan: f64 = f64::NAN;
    let oddiy: f64 = 1.0;
    println!("{}", nan == nan);
    println!("{}", oddiy == oddiy);
    // false  ← NaN hech narsaga teng emas!
    // true

    // custom PartialEq — faqat id bo'yicha
    // пользовательский PartialEq — только по id
    let talaba1 = Talaba { ism: String::from("Ali"), id: 1, baho: 9.5 };
    let talaba2 = Talaba { ism: String::from("Vali"), id: 1, baho: 7.0 };
    let talaba3 = Talaba { ism: String::from("Ali"), id: 2, baho: 9.5 };
    println!("{}", talaba1 == talaba2);
    println!("{}", talaba1 == talaba3);
    // true  ← id bir xil (ism va baho farqli bo'lsa ham)
    // false ← id farqli

    // Eq — HashMap va HashSet uchun shart
    // Eq — обязателен для HashMap и HashSet
    use std::collections::HashSet;
    let mut set: HashSet<UserId> = HashSet::new();
    set.insert(UserId(1));
    set.insert(UserId(2));
    set.insert(UserId(1));
    println!("{}", set.len());
    // 2  ← takror qo'shilmadi

    // Versiya — derive Ord
    // Версия — derive Ord
    let v1 = Versiya { major: 1, minor: 0, patch: 0 };
    let v2 = Versiya { major: 1, minor: 2, patch: 0 };
    let v3 = Versiya { major: 2, minor: 0, patch: 0 };
    println!("{}", v1 < v2);
    println!("{}", v2 < v3);
    println!("{}", v1 == v1.clone());
    // true
    // true
    // true  ← Versiya Copy emas, shuning uchun .clone() yo'q — xato

    // Versiya — tartiblash
    // Версия — сортировка
    let mut versiyalar = vec![
        Versiya { major: 1, minor: 5, patch: 0 },
        Versiya { major: 0, minor: 9, patch: 1 },
        Versiya { major: 2, minor: 0, patch: 0 },
        Versiya { major: 1, minor: 0, patch: 3 },
    ];
    versiyalar.sort();
    println!("{:#?}", versiyalar);
    // [
    //   Versiya { major: 0, minor: 9, patch: 1 },
    //   Versiya { major: 1, minor: 0, patch: 3 },
    //   Versiya { major: 1, minor: 5, patch: 0 },
    //   Versiya { major: 2, minor: 0, patch: 0 },
    // ]

    // Versiya — max va min
    // Версия — max и min
    let eng_katta = versiyalar.iter().max().unwrap();
    let eng_kichik = versiyalar.iter().min().unwrap();
    println!("{:?}", eng_katta);
    println!("{:?}", eng_kichik);
    // Versiya { major: 2, minor: 0, patch: 0 }
    // Versiya { major: 0, minor: 9, patch: 1 }

    // custom PartialOrd — narx bo'yicha
    // пользовательский PartialOrd — по цене
    let mahsulot1 = Mahsulot {
        nomi: String::from("Telefon"),
        narx: 500.0,
        reyting: 4.5,
    };
    let mahsulot2 = Mahsulot {
        nomi: String::from("Noutbuk"),
        narx: 1200.0,
        reyting: 4.8,
    };
    println!("{}", mahsulot1 < mahsulot2);
    println!("{}", mahsulot1 > mahsulot2);
    // true
    // false

    // custom Ord — maosh bo'yicha tartiblash
    // пользовательский Ord — сортировка по зарплате
    let mut xodimlar = vec![
        Xodim { ism: String::from("Ali"), yosh: 30, maosh: 5000 },
        Xodim { ism: String::from("Vali"), yosh: 25, maosh: 8000 },
        Xodim { ism: String::from("Soli"), yosh: 35, maosh: 5000 },
        Xodim { ism: String::from("Holi"), yosh: 28, maosh: 3000 },
    ];
    xodimlar.sort();
    for x in &xodimlar {
        println!("{}: {} so'm, {} yosh", x.ism, x.maosh, x.yosh);
    }
    // Holi: 3000 so'm, 28 yosh
    // Ali: 5000 so'm, 30 yosh
    // Soli: 5000 so'm, 35 yosh
    // Vali: 8000 so'm, 25 yosh

    // Ordering — to'g'ridan-to'g'ri ishlatish
    // прямое использование Ordering
    let taqqoslash: Ordering = 5u32.cmp(&10u32);
    println!("{:?}", taqqoslash);
    // Less

    let taqqoslash2: Ordering = 10u32.cmp(&10u32);
    println!("{:?}", taqqoslash2);
    // Equal

    let taqqoslash3: Ordering = 15u32.cmp(&10u32);
    println!("{:?}", taqqoslash3);
    // Greater

    // Ordering — zanjirli taqqoslash (.then, .then_with)
    // цепочка сравнений (.then, .then_with)
    let birinchi_nom: &str = "Ali";
    let ikkinchi_nom: &str = "Ali";
    let birinchi_yosh: u32 = 25;
    let ikkinchi_yosh: u32 = 30;
    let zanjir: Ordering = birinchi_nom.cmp(ikkinchi_nom)
        .then(birinchi_yosh.cmp(&ikkinchi_yosh));
    println!("{:?}", zanjir);
    // Less  ← ismlar teng, yosh bo'yicha kichik

    // .then_with — closure bilan
    // с замыканием
    let zanjir2: Ordering = birinchi_nom.cmp(ikkinchi_nom)
        .then_with(|| birinchi_yosh.cmp(&ikkinchi_yosh));
    println!("{:?}", zanjir2);
    // Less

    // sort_by — custom tartiblash
    // пользовательская сортировка
    let mut sonlar: Vec<i32> = vec![5, 2, 8, 1, 9, 3];
    sonlar.sort_by(|a, b| b.cmp(a));
    println!("{:?}", sonlar);
    // [9, 8, 5, 3, 2, 1]  ← kamayish tartibida

    // sort_by_key — kalit bo'yicha tartiblash
    // сортировка по ключу
    let mut so_zlar: Vec<&str> = vec!["salom", "hi", "privet", "yo"];
    so_zlar.sort_by_key(|s| s.len());
    println!("{:?}", so_zlar);
    // ["hi", "yo", "salom", "privet"]

    // clamp — oraliqda qoldirish (Ord talab qiladi)
    // удержание в диапазоне (требует Ord)
    let son: i32 = 150;
    let cheklangan: i32 = son.clamp(0, 100);
    println!("{}", cheklangan);
    // 100

    let son2: i32 = -50;
    let cheklangan2: i32 = son2.clamp(0, 100);
    println!("{}", cheklangan2);
    // 0

    // std::cmp::min va max
    // std::cmp::min и max
    let kichik: i32 = std::cmp::min(10, 20);
    let katta: i32 = std::cmp::max(10, 20);
    println!("{}", kichik);
    println!("{}", katta);
    // 10
    // 20

    // min_by_key va max_by_key
    // min_by_key и max_by_key
    let so_zlar2: Vec<&str> = vec!["salom", "hi", "privet"];
    let eng_qisqa = so_zlar2.iter().min_by_key(|s| s.len()).unwrap();
    let eng_uzun = so_zlar2.iter().max_by_key(|s| s.len()).unwrap();
    println!("{}", eng_qisqa);
    println!("{}", eng_uzun);
    // hi
    // privet
}
// #================================================================================================================================================#
// # |  №  | Trait / Metod            | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                           DEFAULT TRAIT                                                                                      |
// #================================================================================================================================================#
// # |   1 | #[derive(Default)]       | Avtomatik default qiymatlar                          | Автоматические значения по умолчанию                 |
// # |   2 | impl Default for T       | Qo'lda default qiymatlar                             | Ручные значения по умолчанию                         |
// # |   3 | T::default()             | Default qiymatni olish                               | Получение значения по умолчанию                      |
// # |   4 | ..Default::default()     | Struct update syntax bilan                           | С синтаксисом обновления структуры                   |
// #================================================================================================================================================#
// # |                                        PARTIALEQ VA EQ TRAITLARI                                                                             |
// #================================================================================================================================================#
// # |   5 | #[derive(PartialEq)]     | Avtomatik == va != operatorlari                      | Автоматические операторы == и !=                     |
// # |   6 | #[derive(Eq)]            | To'liq tenglik (HashSet/HashMap uchun shart)         | Полное равенство (обязательно для HashSet/HashMap)   |
// # |   7 | impl PartialEq for T     | Qo'lda == va != implement qilish                     | Ручная реализация == и !=                            |
// # |   8 | f64 — Eq yo'q            | NaN == NaN = false, shuning uchun Eq yo'q            | NaN == NaN = false, поэтому Eq отсутствует           |
// #================================================================================================================================================#
// # |                                       PARTIALORD VA ORD TRAITLARI                                                                            |
// #================================================================================================================================================#
// # |   9 | #[derive(PartialOrd)]    | Avtomatik < > <= >= operatorlari                     | Автоматические операторы < > <= >=                   |
// # |  10 | #[derive(Ord)]           | To'liq tartib (sort() uchun shart)                   | Полный порядок (обязателен для sort())               |
// # |  11 | impl PartialOrd for T    | Qo'lda partial_cmp implement qilish                  | Ручная реализация partial_cmp                        |
// # |  12 | impl Ord for T           | Qo'lda cmp implement qilish                          | Ручная реализация cmp                                |
// #================================================================================================================================================#
// # |                                           ORDERING                                                                                           |
// #================================================================================================================================================#
// # |  13 | Ordering::Less           | Chap taraf kichik                                    | Левая часть меньше                                   |
// # |  14 | Ordering::Equal          | Teng                                                 | Равны                                                |
// # |  15 | Ordering::Greater        | Chap taraf katta                                     | Левая часть больше                                   |
// # |  16 | .then(ordering)          | Teng bo'lsa keyingi taqqoslash                       | При равенстве следующее сравнение                    |
// # |  17 | .then_with(|| ...)       | Teng bo'lsa closure bilan                            | При равенстве с замыканием                           |
// #================================================================================================================================================#
// # |                                          FOYDALI METODLAR                                                                                    |
// #================================================================================================================================================#
// # |  18 | .sort()                  | Ord bo'yicha tartiblash                              | Сортировка по Ord                                    |
// # |  19 | .sort_by(|a,b| ...)      | Custom taqqoslash bilan tartiblash                   | Сортировка с пользовательским сравнением             |
// # |  20 | .sort_by_key(|x| ...)    | Kalit bo'yicha tartiblash                            | Сортировка по ключу                                  |
// # |  21 | .clamp(min, max)         | Oraliqda qoldirish                                   | Удержание в диапазоне                                |
// # |  22 | std::cmp::min/max        | Ikki qiymatdan kichik/kattasini olish                | Получение меньшего/большего из двух значений         |
// # |  23 | .min_by_key / max_by_key | Kalit bo'yicha min/max                               | min/max по ключу                                     |
// #================================================================================================================================================#