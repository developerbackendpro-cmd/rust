// #================================================================================================================================================#
// #                                                                TURBOFISH  ::<>                                                                 #
// #                            TURBOFISH — GENERIC TURNI ANIQ KO'RSATISH. ::<T> SINTAKSISI. KO'P HOLLARDA KOMPILYATOR O'ZI ANIQLAYDI.              #
// #                            TURBOFISH — ЯВНОЕ УКАЗАНИЕ GENERIC ТИПА. СИНТАКСИС ::<T>. В БОЛЬШИНСТВЕ СЛУЧАЕВ КОМПИЛЯТОР САМ ОПРЕДЕЛЯЕТ.          #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::collections::{HashMap, HashSet, BTreeMap};

// Turbofish — ::<T> sintaksisi
// Turbofish — синтаксис ::<T>
//
// Qachon kerak:
// Когда нужен:
//   - Kompilyator tur aniqlay olmasa
//   - Когда компилятор не может определить тип
//   - Bir nechta tur mumkin bo'lganda
//   - Когда возможно несколько типов
//   - Aniqlik kerak bo'lganda
//   - Когда нужна явность
//
// Qachon kerak emas:
// Когда не нужен:
//   - let x: Vec<i32> = iter.collect()  ← tur annotatsiya yetarli
//   - let x: i32 = s.parse().unwrap()   ← tur annotatsiya yetarli

fn turbofish_asosiy_misollari() {

    // parse() — turbofish bilan
    // parse() — с turbofish
    let s: &str = "42";
    let son_turbo: i32 = s.parse::<i32>().unwrap();
    let son_annot: i32 = s.parse().unwrap();
    println!("{} {}", son_turbo, son_annot);
    // 42 42

    // parse() — turli turlar
    // parse() — разные типы
    let s2: &str = "3.14";
    let f: f64 = s2.parse::<f64>().unwrap();
    let s3: &str = "255";
    let b: u8 = s3.parse::<u8>().unwrap();
    println!("{} {}", f, b);
    // 3.14 255

    // collect() — turbofish bilan
    // collect() — с turbofish
    let v1 = (1..=5).collect::<Vec<i32>>();
    let v2 = (1..=5).collect::<Vec<_>>();
    println!("{:?}", v1);
    println!("{:?}", v2);
    // [1, 2, 3, 4, 5]
    // [1, 2, 3, 4, 5]

    // collect() — turli kolleksiyalar
    // collect() — разные коллекции
    let set = vec![1, 2, 2, 3, 3].into_iter().collect::<HashSet<i32>>();
    let mut set_sorted: Vec<i32> = set.into_iter().collect();
    set_sorted.sort();
    println!("{:?}", set_sorted);
    // [1, 2, 3]

    let map = vec![("bir", 1), ("ikki", 2)].into_iter().collect::<HashMap<_, _>>();
    println!("{:?}", map.get("bir"));
    // Some(1)

    let btree = vec![(3, "c"), (1, "a"), (2, "b")].into_iter().collect::<BTreeMap<_, _>>();
    for (k, v) in &btree {
        print!("{}{} ", k, v);
    }
    println!();
    // 1a 2b 3c

    // iter().sum() — turbofish bilan
    // iter().sum() — с turbofish
    let yig: i64 = vec![1i64, 2, 3, 4, 5].iter().sum::<i64>();
    println!("{}", yig);
    // 15

    // iter().product() — turbofish bilan
    // iter().product() — с turbofish
    let ko_p: u64 = (1u64..=5).product::<u64>();
    println!("{}", ko_p);
    // 120

    // min() va max() — turbofish kerak bo'lmaydi (lekin mumkin)
    // min() и max() — turbofish не нужен (но возможен)
    let v: Vec<i32> = vec![3, 1, 4, 1, 5, 9];
    let katta = v.iter().max().unwrap();
    println!("{}", katta);
    // 9
}

fn turbofish_generic_funksiyalar() {

    // Generic funksiya — turbofish bilan chaqirish
    // Generic функция — вызов с turbofish
    fn aylashtir<T: std::str::FromStr>(s: &str) -> Option<T> {
        s.parse().ok()
    }

    let n: Option<i32> = aylashtir::<i32>("42");
    let f: Option<f64> = aylashtir::<f64>("3.14");
    let b: Option<bool> = aylashtir::<bool>("true");
    println!("{:?}", n);
    println!("{:?}", f);
    println!("{:?}", b);
    // Some(42)
    // Some(3.14)
    // Some(true)

    // Generic funksiya — tur annotatsiya ham ishlaydi
    // Generic функция — аннотация типа тоже работает
    let n2: Option<i32> = aylashtir("100");
    println!("{:?}", n2);
    // Some(100)

    // std::mem::size_of — turbofish majburiy
    // std::mem::size_of — turbofish обязателен
    println!("{}", std::mem::size_of::<i32>());
    println!("{}", std::mem::size_of::<f64>());
    println!("{}", std::mem::size_of::<bool>());
    println!("{}", std::mem::size_of::<char>());
    println!("{}", std::mem::size_of::<String>());
    println!("{}", std::mem::size_of::<Vec<i32>>());
    // 4
    // 8
    // 1
    // 4
    // 24
    // 24

    // std::mem::align_of — turbofish majburiy
    // std::mem::align_of — turbofish обязателен
    println!("{}", std::mem::align_of::<i32>());
    println!("{}", std::mem::align_of::<f64>());
    // 4
    // 8

    // std::mem::zeroed — turbofish majburiy (unsafe)
    // std::mem::zeroed — turbofish обязателен (unsafe)
    let zeroed: i32 = unsafe { std::mem::zeroed::<i32>() };
    println!("{}", zeroed);
    // 0

    // std::default::Default::default — turbofish
    // std::default::Default::default — turbofish
    let default_vec = Vec::<i32>::new();
    let default_map = HashMap::<String, i32>::new();
    println!("{:?}", default_vec);
    println!("{:?}", default_map);
    // []
    // {}
}

fn turbofish_murakkab_misollari() {

    // Bir nechta generic parametr
    // Несколько generic параметров
    fn juft_qiymat<A, B>(a: A, b: B) -> (A, B) {
        (a, b)
    }
    let j = juft_qiymat::<i32, &str>(42, "salom");
    println!("{:?}", j);
    // (42, "salom")

    // from() — turbofish bilan
    // from() — с turbofish
    let s = String::from("salom");
    let s2 = <String as From<&str>>::from("dunyo");
    println!("{}", s);
    println!("{}", s2);
    // salom
    // dunyo

    // into() — tur annotatsiya (turbofish yo'q)
    // into() — аннотация типа (без turbofish)
    let s3: String = "rust".into();
    println!("{}", s3);
    // rust

    // Iterator::collect — murakkab tur
    // Iterator::collect — сложный тип
    let result: Result<Vec<i32>, _> = vec!["1", "2", "3"]
        .iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{:?}", result);
    // Ok([1, 2, 3])

    // turbofish zanjirida
    // в цепочке turbofish
    let natija = "1,2,3,4,5"
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .filter(|&x| x % 2 != 0)
        .collect::<Vec<_>>();
    println!("{:?}", natija);
    // [1, 3, 5]

    // Vec::with_capacity — turbofish
    // Vec::with_capacity — turbofish
    let mut v = Vec::<i32>::with_capacity(10);
    v.extend(1..=5);
    println!("{:?}", v);
    println!("Capacity: {}", v.capacity());
    // [1, 2, 3, 4, 5]
    // Capacity: 10

    // Box::new — turbofish
    // Box::new — turbofish
    let boxed = Box::<i32>::new(42);
    println!("{}", boxed);
    // 42

    // Ok va Err — turbofish bilan
    // Ok и Err — с turbofish
    let ok: Result<i32, String> = Ok(42);
    let err: Result<i32, String> = Err("xato".to_string());
    println!("{:?}", ok);
    println!("{:?}", err);
    // Ok(42)
    // Err("xato")
}

fn turbofish_qachon_kerak_emas() {

    // Tur annotatsiya yetarli
    // Аннотация типа достаточна
    let v: Vec<i32> = (1..=5).collect();
    println!("{:?}", v);
    // [1, 2, 3, 4, 5]

    // Tur kontekstdan aniq
    // Тип понятен из контекста
    let s: String = "salom".to_string();
    let n: i32 = "42".parse().unwrap();
    println!("{} {}", s, n);
    // salom 42

    // Funksiya parametri tur beradi
    // Тип задаётся параметром функции
    fn i32_qabul(n: i32) -> i32 { n * 2 }
    let natija = i32_qabul("21".parse().unwrap());
    println!("{}", natija);
    // 42

    // Return turi aniq bo'lsa
    // Если тип возврата ясен
    fn vec_qaytaradi() -> Vec<i32> {
        (1..=3).collect()  // turbofish kerak emas
    }
    println!("{:?}", vec_qaytaradi());
    // [1, 2, 3]
}

fn turbofish_real_hayot() {

    // JSON parsing simulyatsiyasi — turbofish bilan
    // Симуляция парсинга JSON — с turbofish
    fn string_dan_tur<T: std::str::FromStr>(s: &str, standart: T) -> T {
        s.parse::<T>().unwrap_or(standart)
    }

    let port: u16 = string_dan_tur::<u16>("8080", 80);
    let debug: bool = string_dan_tur::<bool>("true", false);
    let timeout: f64 = string_dan_tur::<f64>("30.5", 30.0);
    println!("port={}, debug={}, timeout={}", port, debug, timeout);
    // port=8080, debug=true, timeout=30.5

    // Transformatsiya pipeline — turbofish
    // Конвейер трансформации — turbofish
    let csv_satri: &str = "1,2,3,4,5,6,7,8,9,10";
    let sonlar: Vec<i32> = csv_satri
        .split(',')
        .filter_map(|s| s.parse::<i32>().ok())
        .filter(|&x| x % 2 == 0)
        .map(|x| x * x)
        .collect::<Vec<_>>();
    println!("{:?}", sonlar);
    // [4, 16, 36, 64, 100]

    // HashMap qurish — turbofish
    // Построение HashMap — turbofish
    let konfiguratsiya = [
        ("host", "localhost"),
        ("port", "8080"),
        ("debug", "true"),
    ]
        .iter()
        .copied()
        .collect::<HashMap<&str, &str>>();
    println!("{:?}", konfiguratsiya.get("port"));
    // Some("8080")

    // size_of bilan xotira tahlili
    // Анализ памяти с size_of
    println!("=== Xotira o'lchamlari ===");
    println!("i8:     {} bayt", std::mem::size_of::<i8>());
    println!("i32:    {} bayt", std::mem::size_of::<i32>());
    println!("i64:    {} bayt", std::mem::size_of::<i64>());
    println!("f32:    {} bayt", std::mem::size_of::<f32>());
    println!("f64:    {} bayt", std::mem::size_of::<f64>());
    println!("usize:  {} bayt", std::mem::size_of::<usize>());
    println!("String: {} bayt", std::mem::size_of::<String>());
    println!("Vec:    {} bayt", std::mem::size_of::<Vec<u8>>());
    println!("Option<i32>: {} bayt", std::mem::size_of::<Option<i32>>());
    println!("Option<Box<i32>>: {} bayt", std::mem::size_of::<Option<Box<i32>>>());
    // i8:     1 bayt
    // i32:    4 bayt
    // i64:    8 bayt
    // f32:    4 bayt
    // f64:    8 bayt
    // usize:  8 bayt
    // String: 24 bayt
    // Vec:    24 bayt
    // Option<i32>: 8 bayt
    // Option<Box<i32>>: 8 bayt  ← null pointer optimization!
}

fn main() {

    println!("=== ASOSIY MISOLAR ===");
    turbofish_asosiy_misollari();

    println!("\n=== GENERIC FUNKSIYALAR ===");
    turbofish_generic_funksiyalar();

    println!("\n=== MURAKKAB MISOLLAR ===");
    turbofish_murakkab_misollari();

    println!("\n=== QACHON KERAK EMAS ===");
    turbofish_qachon_kerak_emas();

    println!("\n=== REAL HAYOT ===");
    turbofish_real_hayot();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                   | Описание (RU)                                              |
// #================================================================================================================================================#
// # |   1 | f::<T>()                  | Turbofish — generic turni aniq ko'rsatish     | Явное указание generic типа                                |
// # |   2 | f::<T, U>()               | Bir nechta generic parametr                   | Несколько generic параметров                               |
// # |   3 | iter.collect::<Vec<T>>()  | collect uchun turbofish                       | Turbofish для collect                                      |
// # |   4 | iter.collect::<Vec<_>>()  | _ — kompilyator aniqlaydi                     | _ — компилятор определяет                                  |
// # |   5 | s.parse::<i32>()          | parse uchun turbofish                         | Turbofish для parse                                        |
// # |   6 | iter.sum::<i64>()         | sum uchun turbofish                           | Turbofish для sum                                          |
// # |   7 | iter.product::<u64>()     | product uchun turbofish                       | Turbofish для product                                      |
// # |   8 | size_of::<T>()            | Majburiy turbofish — argument yo'q            | Обязательный turbofish — нет аргументов                    |
// # |   9 | Vec::<T>::new()           | Tur parametrli konstruktor                    | Конструктор с параметром типа                              |
// # |  10 | <T as Trait>::method()    | UFCS — Universal Function Call Syntax         | UFCS — Универсальный синтаксис вызова функций              |
// # |  11 | Kerak bo'lganda           | Kompilyator aniqlay olmasa                    | Когда компилятор не может определить                       |
// # |  12 | Kerak emas                | Tur annotatsiya yetarli bo'lsa                | Когда достаточно аннотации типа                            |
// #================================================================================================================================================#