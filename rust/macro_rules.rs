// #================================================================================================================================================#
// #                                                              MACRO_RULES!                                                                      #
// #                DEKLARATIV MAKROLAR — PATTERN MATCHING ASOSIDA KOD GENERATSIYA. GIGIENIK, VARIADIC, RECURSIVE MAKROLAR.                         #
// #                ДЕКЛАРАТИВНЫЕ МАКРОСЫ — ГЕНЕРАЦИЯ КОДА НА ОСНОВЕ СОПОСТАВЛЕНИЯ ПАТТЕРНОВ. ГИГИЕНА, VARIADIC, РЕКУРСИЯ.                          #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::collections::HashMap;

// macro_rules! nima:
// Что такое macro_rules!:
//
//   - Kompilyatsiya vaqtida kodni generatsiya qiluvchi mexanizm
//   - Механизм генерации кода во время компиляции
//   - Pattern matching asosida ishlaydi
//   - Работает на основе сопоставления с образцом
//   - Gigienik — o'zgaruvchi nomlari chalkashmaydi
//   - Гигиеничный — имена переменных не конфликтуют
//   - Proc macro dan farqi: token stream emas, pattern asosida
//   - Отличие от proc macro: на основе паттерна, не token stream
//
// Metavariable turlari:
// Виды метапеременных:
//   $x:expr    — ifoda (expression)
//   $x:ident   — identifikator
//   $x:ty      — tur (type)
//   $x:pat     — pattern
//   $x:stmt    — operator (statement)
//   $x:block   — blok { }
//   $x:item    — element (fn, struct, ...)
//   $x:meta    — attribute ichidagi
//   $x:tt      — token tree (har qanday token)
//   $x:literal — literal qiymat
//   $x:lifetime— lifetime ('a)
//   $x:vis     — ko'rinish (pub, pub(crate))
//
// Takrorlovchilar:
// Повторители:
//   $(...)*    — 0 yoki ko'p
//   $(...)+    — 1 yoki ko'p
//   $(...)?    — 0 yoki 1

// Oddiy makro — bitta pattern
// Простой макро — один паттерн
macro_rules! salom {
    () => {
        println!("Salom, Dunyo!");
    };
    ($ism:expr) => {
        println!("Salom, {}!", $ism);
    };
    ($ism:expr, $til:expr) => {
        println!("Salom {}! Til: {}", $ism, $til);
    };
}

// Matematik makro
// Математический макрос
macro_rules! kvadrat {
    ($x:expr) => {
        ($x) * ($x)
    };
}

macro_rules! kub {
    ($x:expr) => {
        ($x) * ($x) * ($x)
    };
}

// assert bilan o'xshash
// Похоже на assert
macro_rules! tekshir {
    ($shart:expr) => {
        if !$shart {
            panic!("Tekshiruv muvaffaqiyatsiz: {}", stringify!($shart));
        }
    };
    ($shart:expr, $xabar:literal) => {
        if !$shart {
            panic!("Tekshiruv muvaffaqiyatsiz: {}", $xabar);
        }
    };
    ($shart:expr, $($arg:tt)*) => {
        if !$shart {
            panic!("Tekshiruv muvaffaqiyatsiz: {}", format!($($arg)*));
        }
    };
}

fn asosiy_makro_misollari() {

    salom!();
    salom!("Dilshod");
    salom!("Dilshod", "Rust");
    // Salom, Dunyo!
    // Salom, Dilshod!
    // Salom Dilshod! Til: Rust

    println!("{}", kvadrat!(5));
    println!("{}", kvadrat!(3 + 2));
    println!("{}", kub!(3));
    // 25
    // 25  ← (3+2)*(3+2) = 25
    // 27

    tekshir!(2 + 2 == 4);
    tekshir!(5 > 3, "5 3 dan katta bo'lishi kerak");
    tekshir!(10 % 2 == 0, "{} juft son", 10);
    println!("Barcha tekshiruvlar o'tdi");
    // Barcha tekshiruvlar o'tdi
}

// vec! ning o'z versiyasi
// Наша версия vec!
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($element:expr),+ $(,)?) => {
        {
            let mut v = Vec::new();
            $(v.push($element);)+
            v
        }
    };
}

// HashMap yaratish makrosi
// Макрос создания HashMap
macro_rules! xarita {
    () => {
        HashMap::new()
    };
    ($($kalit:expr => $qiymat:expr),+ $(,)?) => {
        {
            let mut m = HashMap::new();
            $(m.insert($kalit, $qiymat);)+
            m
        }
    };
}

// println-ga o'xshash maxsus log makrosi
// Кастомный макрос лога похожий на println
macro_rules! log {
    (info: $($arg:tt)*) => {
        println!("[INFO ] {}", format!($($arg)*));
    };
    (warn: $($arg:tt)*) => {
        println!("[WARN ] {}", format!($($arg)*));
    };
    (error: $($arg:tt)*) => {
        eprintln!("[ERROR] {}", format!($($arg)*));
    };
    (debug: $($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!("[DEBUG] {}", format!($($arg)*));
    };
}

// Operator overloading uchun makro
// Макрос для перегрузки операторов
macro_rules! impl_operator {
    ($trait_nomi:ident, $metod:ident, $op:tt, $tur:ty) => {
        impl std::ops::$trait_nomi for $tur {
            type Output = $tur;
            fn $metod(self, b: $tur) -> $tur {
                <$tur>::new(self.qiymat $op b.qiymat)
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vekt(f64);
impl Vekt {
    fn new(q: f64) -> Self { Vekt(q) }
}
// Lekin bu misol uchun oddiy qilamiz
// Для этого примера упростим

// Display impl uchun makro
// Макрос для реализации Display
macro_rules! impl_display {
    ($tur:ty, $format:literal, $($field:ident),+) => {
        impl std::fmt::Display for $tur {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, $format, $(self.$field),+)
            }
        }
    };
}

fn variadic_makro_misollari() {

    // my_vec!
    let v1: Vec<i32> = my_vec![];
    let v2: Vec<i32> = my_vec![1, 2, 3, 4, 5];
    let v3: Vec<&str> = my_vec!["salom", "dunyo", "rust",]; // oxirida vergul ham
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
    // []
    // [1, 2, 3, 4, 5]
    // ["salom", "dunyo", "rust"]

    // xarita!
    let m1: HashMap<&str, i32> = xarita!();
    let m2: HashMap<&str, i32> = xarita! {
        "bir" => 1,
        "ikki" => 2,
        "uch" => 3,
    };
    println!("{:?}", m1);
    println!("{:?}", m2.get("ikki"));
    // {}
    // Some(2)

    // log!
    log!(info: "Dastur boshlandi");
    log!(warn: "Xotira {} % to'la", 80);
    log!(error: "Fayl topilmadi: {}", "config.toml");
    log!(debug: "Debug: x = {}", 42);
    // [INFO ] Dastur boshlandi
    // [WARN ] Xotira 80 % to'la
    // [ERROR] Fayl topilmadi: config.toml
    // [DEBUG] Debug: x = 42
}

// Yig'indi — rekursiv makro
// Сумма — рекурсивный макрос
macro_rules! yig_indi {
    ($x:expr) => { $x };
    ($x:expr, $($qolgan:expr),+) => {
        $x + yig_indi!($($qolgan),+)
    };
}

// Ko'paytma — rekursiv
// Произведение — рекурсивный
macro_rules! ko_paytma {
    ($x:expr) => { $x };
    ($x:expr, $($qolgan:expr),+) => {
        $x * ko_paytma!($($qolgan),+)
    };
}

// min! makrosi — rekursiv
// Макрос min! — рекурсивный
macro_rules! kichik {
    ($x:expr) => { $x };
    ($x:expr, $($qolgan:expr),+) => {
        {
            let qolgan_min = kichik!($($qolgan),+);
            if $x < qolgan_min { $x } else { qolgan_min }
        }
    };
}

// stringify! bilan debug
// Отладка с stringify!
macro_rules! dbg_kengaytirilgan {
    ($($x:expr),+) => {
        {
            $(
                println!("{} = {:?}", stringify!($x), $x);
            )+
        }
    };
}

// count! — miqdor hisoblash
// count! — подсчёт количества
macro_rules! hisob {
    () => { 0usize };
    ($_x:expr $(, $qolgan:expr)*) => {
        1usize + hisob!($($qolgan),*)
    };
}

fn rekursiv_makro_misollari() {

    println!("{}", yig_indi!(1, 2, 3, 4, 5));
    println!("{}", ko_paytma!(1, 2, 3, 4, 5));
    println!("{}", kichik!(5, 2, 8, 1, 9, 3));
    // 15
    // 120
    // 1

    let x = 42;
    let y = vec![1, 2, 3];
    let z = "salom";
    dbg_kengaytirilgan!(x, x + 1, y.len(), z);
    // x = 42
    // x + 1 = 43
    // y.len() = 3
    // z = "salom"

    println!("{}", hisob!());
    println!("{}", hisob!(a, b, c));
    println!("{}", hisob!(1, 2, 3, 4, 5));
    // 0
    // 3
    // 5
}

// Bir nechta tur uchun bir xil trait implement qilish
// Реализация одного трейта для нескольких типов
macro_rules! impl_from_str {
    ($($tur:ty),+) => {
        $(
            impl std::str::FromStr for $tur {
                type Err = std::num::ParseIntError;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let n: i64 = s.trim().parse()?;
                    Ok(Self(n as _))
                }
            }
        )+
    };
}

// Newtype pattern uchun makro
// Макрос для паттерна Newtype
macro_rules! newtype {
    ($(#[$meta:meta])* $nomi:ident($ichki:ty)) => {
        $(#[$meta])*
        pub struct $nomi(pub $ichki);

        impl $nomi {
            pub fn new(val: $ichki) -> Self { $nomi(val) }
            pub fn ichki(&self) -> &$ichki { &self.0 }
            pub fn olish(self) -> $ichki { self.0 }
        }

        impl std::fmt::Display for $nomi {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl std::fmt::Debug for $nomi {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($nomi), self.0)
            }
        }

        impl std::ops::Deref for $nomi {
            type Target = $ichki;
            fn deref(&self) -> &$ichki { &self.0 }
        }
    };
}

newtype!(
    #[derive(Clone, PartialEq)]
    UserId(u64)
);

newtype!(
    #[derive(Clone, PartialEq)]
    Email(String)
);

fn trait_impl_makro_misollari() {

    // newtype makrosi
    let uid = UserId::new(42);
    println!("{}", uid);
    println!("{:?}", uid);
    println!("{}", *uid); // Deref
    // 42
    // UserId(42)
    // 42

    let email = Email::new("dilshod@rust.uz".to_string());
    println!("{}", email);
    println!("{}", email.len()); // Deref → String metodlari
    // dilshod@rust.uz
    // 15

    // UserId taqqoslash
    let uid2 = UserId::new(42);
    let uid3 = UserId::new(99);
    println!("{}", uid == uid2);
    println!("{}", uid == uid3);
    // true
    // false
}

// Gigienik — makro ichidagi o'zgaruvchilar tashqi bilan chalkashmaydi
// Гигиеничный — переменные внутри макроса не конфликтуют с внешними
macro_rules! ikkiga_oshir {
    ($x:expr) => {
        {
            let temp = $x; // 'temp' — makro ichida, tashqari bilan chalkashmaydi
            temp * 2       // tashqarida 'temp' bo'lsa ham, bu boshqa
        }
    };
}

// Gigienik emas bo'lar edi — eski C makrolar kabi
// Не был бы гигиеничным — как старые макросы C
// #define DOUBLE(x) ({ int temp = x; temp * 2; })  ← C da xatoga olib kelishi mumkin

macro_rules! swap {
    ($a:expr, $b:expr) => {
        {
            let temp = $a;
            $a = $b;
            $b = temp;
        }
    };
}

fn hygiene_misollari() {

    let temp = 100; // makro ichidagi 'temp' bilan CHALKASHMAYDI
    let natija = ikkiga_oshir!(temp + 5);
    println!("{} {}", temp, natija); // temp o'zgarmadi
    // 100 210

    let mut a = 10;
    let mut b = 20;
    println!("Avval: a={} b={}", a, b);
    swap!(a, b);
    println!("Keyin: a={} b={}", a, b);
    // Avval: a=10 b=20
    // Keyin: a=20 b=10

    // stringify! — ifodani satrga aylantirish
    // stringify! — преобразование выражения в строку
    let ifoda = stringify!(1 + 2 + 3);
    println!("{}", ifoda);
    // 1 + 2 + 3

    // concat! — kompilyatsiya vaqtida satrlarni birlashtirish
    // concat! — конкатенация строк во время компиляции
    let birlashgan = concat!("Salom", " ", "Dunyo", "!");
    println!("{}", birlashgan);
    // Salom Dunyo!

    // file!, line!, column! — meta ma'lumot
    // file!, line!, column! — метаинформация
    println!("Fayl: {}", file!());
    println!("Qator: {}", line!());
    // Fayl: src/main.rs
    // Qator: XX
}

// DSL — Domain Specific Language yaratish
// DSL — создание предметно-ориентированного языка
macro_rules! sql_like {
    (SELECT $($field:ident),+ FROM $jadval:ident WHERE $shart:expr) => {
        {
            println!("SELECT {} FROM {} WHERE {}",
                vec![$(stringify!($field)),+].join(", "),
                stringify!($jadval),
                stringify!($shart)
            );
        }
    };
    (SELECT * FROM $jadval:ident) => {
        println!("SELECT * FROM {}", stringify!($jadval));
    };
    (INSERT INTO $jadval:ident VALUES ($($val:expr),+)) => {
        println!("INSERT INTO {} VALUES ({})",
            stringify!($jadval),
            vec![$(format!("{:?}", $val)),+].join(", ")
        );
    };
}

// State machine makrosi
// Макрос машины состояний
macro_rules! state_machine {
    (
        holat: $HolatTuri:ident,
        boshlanish: $boshlanish:ident,
        otishlar: {
            $($dan:ident => $ga:ident via $amal:ident),+ $(,)?
        }
    ) => {
        #[derive(Debug, PartialEq, Clone)]
        enum $HolatTuri {
            $($dan,)+
            // takrorlanishlar bo'lsa ham OK
        }

        // Otishlar dokumentatsiyasi
        $(
            // $dan -> $ga via $amal
        )+

        impl $HolatTuri {
            fn boshlanish() -> Self {
                $HolatTuri::$boshlanish
            }
        }
    };
}

// Test makrosi
// Макрос тестирования
macro_rules! test_holat {
    ($ism:ident: $ifoda:expr => $kutilgan:expr) => {
        #[test]
        fn $ism() {
            assert_eq!($ifoda, $kutilgan,
                "{} muvaffaqiyatsiz: {} != {}",
                stringify!($ism),
                stringify!($ifoda),
                stringify!($kutilgan)
            );
        }
    };
}

// Test holatlari — test vaqtida ishlaydi
// Тест-кейсы — работают во время тестирования
test_holat!(qo_shish_testi: 2 + 2 => 4);
test_holat!(ko_paytirish_testi: 3 * 4 => 12);
test_holat!(faktorial_testi: (1..=5).product::<i32>() => 120);

fn advanced_makro_misollari() {

    // SQL-like DSL
    sql_like!(SELECT * FROM foydalanuvchilar);
    sql_like!(SELECT ism, yosh, email FROM foydalanuvchilar WHERE yosh > 18);
    sql_like!(INSERT INTO mahsulotlar VALUES ("olma", 1500, 100));
    // SELECT * FROM foydalanuvchilar
    // SELECT ism, yosh, email FROM foydalanuvchilar WHERE yosh > 18
    // INSERT INTO mahsulotlar VALUES ("olma", 1500, 100)

    // env! — muhit o'zgaruvchisi (kompilyatsiya vaqtida)
    // env! — переменная среды (во время компиляции)
    // let api_key = env!("API_KEY"); // agar o'rnatilmagan bo'lsa compile xato

    // option_env! — ixtiyoriy
    // option_env! — опциональный
    let api_key: Option<&str> = option_env!("API_KEY");
    println!("API_KEY: {:?}", api_key);
    // API_KEY: None

    // include_str! — faylni satr sifatida
    // include_str! — файл как строка
    // let sql = include_str!("query.sql");

    // include_bytes! — faylni baytlar sifatida
    // include_bytes! — файл как байты
    // let img = include_bytes!("logo.png");
}

// Error yaratish makrosi
// Макрос создания ошибки
macro_rules! define_error {
    (
        $(#[$meta:meta])*
        $nomi:ident {
            $($variant:ident($xabar:literal)),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug)]
        pub enum $nomi {
            $($variant,)+
        }

        impl std::fmt::Display for $nomi {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match self {
                    $(Self::$variant => write!(f, $xabar),)+
                }
            }
        }

        impl std::error::Error for $nomi {}
    };
}

define_error! {
    #[allow(dead_code)]
    IlovaXato {
        TopilmadI("Ma'lumot topilmadi"),
        Ruxsat("Ruxsat rad etildi"),
        Tarmoq("Tarmoq xatosi"),
        Validatsiya("Ma'lumot noto'g'ri"),
    }
}

// Bench makrosi — vaqt o'lchash
// Макрос Bench — измерение времени
macro_rules! vaqt_olch {
    ($ism:literal, $blok:block) => {
        {
            let boshlanish = std::time::Instant::now();
            let natija = $blok;
            let o_tgan = boshlanish.elapsed();
            println!("[{}] {:.3?}", $ism, o_tgan);
            natija
        }
    };
}

fn real_hayot_misollari() {

    // IlovaXato — define_error! bilan
    let x1 = IlovaXato::TopilmadI;
    let x2 = IlovaXato::Ruxsat;
    println!("{}", x1);
    println!("{}", x2);
    println!("{:?}", x1);
    // Ma'lumot topilmadi
    // Ruxsat rad etildi
    // TopilmadI

    // vaqt_olch! makrosi
    let yig = vaqt_olch!("Faktorial", {
        (1u64..=20).product::<u64>()
    });
    println!("20! = {}", yig);
    // [Faktorial] 0.000µs (taxminan)
    // 20! = 2432902008176640000

    let sorted = vaqt_olch!("Sort", {
        let mut v: Vec<i32> = (0..1000).rev().collect();
        v.sort();
        v.len()
    });
    println!("Sort uzunlik: {}", sorted);
    // [Sort] 0.050µs (taxminan)
    // Sort uzunlik: 1000

    // my_vec + xarita + log kombinatsiyasi
    let sozlar = my_vec!["salom", "dunyo", "rust", "tili"];
    let hisob_m: HashMap<&str, usize> = xarita! {
        "salom" => 5,
        "dunyo" => 5,
        "rust" => 4,
    };

    log!(info: "So'zlar: {:?}", sozlar);
    log!(info: "Hisob: {:?}", hisob_m.get("rust"));
    // [INFO ] So'zlar: ["salom", "dunyo", "rust", "tili"]
    // [INFO ] Hisob: Some(4)
}

fn main() {

    println!("=== ASOSIY MAKROLAR ===");
    asosiy_makro_misollari();

    println!("\n=== VARIADIC MAKROLAR ===");
    variadic_makro_misollari();

    println!("\n=== REKURSIV MAKROLAR ===");
    rekursiv_makro_misollari();

    println!("\n=== TRAIT IMPL MAKROLAR ===");
    trait_impl_makro_misollari();

    println!("\n=== HYGIENE ===");
    hygiene_misollari();

    println!("\n=== ADVANCED PATTERNLAR ===");
    advanced_makro_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                               | Описание (RU)                                            |
// #================================================================================================================================================#
// # |                                        METAVARIABLE TURLARI                                                                                  |
// #================================================================================================================================================#
// # |   1 | $x:expr                         | Ifoda                                      | Выражение                                               |
// # |   2 | $x:ident                        | Identifikator                              | Идентификатор                                           |
// # |   3 | $x:ty                           | Tur                                        | Тип                                                     |
// # |   4 | $x:pat                          | Pattern                                    | Образец                                                 |
// # |   5 | $x:tt                           | Token tree                                 | Дерево токенов                                          |
// # |   6 | $x:literal                      | Literal qiymat                             | Литеральное значение                                    |
// # |   7 | $x:vis                          | Ko'rinish (pub, ...)                       | Видимость (pub, ...)                                    |
// # |   8 | $x:lifetime                     | Lifetime ('a)                              | Lifetime ('a)                                           |
// #================================================================================================================================================#
// # |                                        TAKRORLOVCHILAR                                                                                       |
// #================================================================================================================================================#
// # |   9 | $(...)*                          | 0 yoki ko'p                                | 0 или больше                                           |
// # |  10 | $(...)+                          | 1 yoki ko'p                                | 1 или больше                                           |
// # |  11 | $(...)?                          | 0 yoki 1                                   | 0 или 1                                                |
// # |  12 | $(,)?                            | Ixtiyoriy oxirgi vergul                    | Опциональная запятая в конце                           |
// #================================================================================================================================================#
// # |                                        BUILT-IN MAKROLAR                                                                                     |
// #================================================================================================================================================#
// # |  13 | stringify!($x)                  | Ifodani satrga                             | Выражение в строку                                      |
// # |  14 | concat!(a, b)                   | Satrlarni birlashtirish (compile time)     | Конкатенация строк (compile time)                       |
// # |  15 | file!(), line!(), column!()     | Manbaa joyi                                | Местоположение в исходнике                              |
// # |  16 | env!("VAR")                     | Muhit o'zgaruvchisi (compile time)         | Переменная среды (compile time)                         |
// # |  17 | include_str!("file")            | Fayl mazmuni satr sifatida                 | Содержимое файла как строка                             |
// # |  18 | include_bytes!("file")          | Fayl mazmuni baytlar sifatida              | Содержимое файла как байты                              |
// # |  19 | Gigienik (hygienic)             | O'zgaruvchi nomi chalkashmasligi           | Переменные не конфликтуют                               |
// # |  20 | DSL pattern                     | Macrolar bilan til yaratish                | Создание языка с помощью макросов                       |
// #================================================================================================================================================#