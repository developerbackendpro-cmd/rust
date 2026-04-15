// #================================================================================================================================================#
// #                                                                    GENERICS                                                                    #
// #                            GENERICS — BIR KOD, KO'P TUR. KOMPILYATSIYA VAQTIDA ANIQLANADI. ZERO-COST ABSTRACTION.                              #
// #                            GENERICS — ОДИН КОД, МНОГО ТИПОВ. ОПРЕДЕЛЯЕТСЯ ВО ВРЕМЯ КОМПИЛЯЦИИ. ZERO-COST АБСТРАКЦИЯ.                           #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
// Generics — kompilyatsiya vaqtida tur aniqlanadi (monomorphization)
// Generics — тип определяется во время компиляции (мономорфизация)
//
// Genericsiz:               Generic bilan:
// fn max_i32(a: i32) -> i32  fn max<T: PartialOrd>(a: T) -> T
// fn max_f64(a: f64) -> f64  ← bitta funksiya!
//
// Rust generics = zero cost:
//   fn max<T>(...) → kompilyatorda:
//     fn max_i32(...)
//     fn max_f64(...)
//   Real kodda har tur uchun alohida kod yaratiladi!

// eng oddiy generic funksiya
// простейшая generic функция
fn birinchi<T>(list: &[T]) -> &T {
    &list[0]
}

// T: PartialOrd — bound bilan
// T: PartialOrd — с ограничением
fn eng_katta<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// bir nechta generic parametr
// несколько generic параметров
fn juft<T, U>(birinchi: T, ikkinchi: U) -> (T, U) {
    (birinchi, ikkinchi)
}

// generic + bir nechta bound
// generic + несколько ограничений
fn chiqar_va_qaytarish<T: fmt::Display + Clone>(qiymat: T) -> T {
    println!("{}", qiymat);
    qiymat.clone()
}

// where clause — murakkab boundlar
// where clause — сложные ограничения
fn solishtir_va_chiqar<T, U>(t: &T, u: &U) -> bool
where
    T: fmt::Display + PartialEq,
    U: fmt::Display + PartialEq,
{
    println!("t={}, u={}", t, u);
    false
}

// bitta generic parametr
// один generic параметр
#[derive(Debug)]
struct Quti<T> {
    ichki: T,
}

impl<T> Quti<T> {
    fn new(qiymat: T) -> Self {
        Quti { ichki: qiymat }
    }

    fn qiymat(&self) -> &T {
        &self.ichki
    }

    fn ichki_ga_aylantirish(self) -> T {
        self.ichki
    }
}

// faqat T: Display bo'lganda qo'shimcha metod
// дополнительный метод только когда T: Display
impl<T: fmt::Display> Quti<T> {
    fn chiqar(&self) {
        println!("Quti({})", self.ichki);
    }
}

// ikki generic parametr
// два generic параметра
#[derive(Debug)]
struct Juft<T, U> {
    birinchi: T,
    ikkinchi: U,
}

impl<T: fmt::Display, U: fmt::Display> Juft<T, U> {
    fn new(birinchi: T, ikkinchi: U) -> Self {
        Juft { birinchi, ikkinchi }
    }

    fn chiqar(&self) {
        println!("({}, {})", self.birinchi, self.ikkinchi);
    }
}

// Option<T> — built-in generic enum
// Option<T> — встроенный generic enum
// enum Option<T> {
//     Some(T),
//     None,
// }

// Result<T, E> — built-in generic enum
// Result<T, E> — встроенный generic enum
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// custom generic enum
// пользовательский generic enum
#[derive(Debug)]
enum Natija<T, E> {
    Muvaffaqiyat(T),
    Xato(E),
}

impl<T: fmt::Display, E: fmt::Display> Natija<T, E> {
    fn chiqar(&self) {
        match self {
            Natija::Muvaffaqiyat(v) => println!("Ok: {}", v),
            Natija::Xato(e)         => println!("Err: {}", e),
        }
    }

    fn muvaffaqiyatlimi(&self) -> bool {
        matches!(self, Natija::Muvaffaqiyat(_))
    }
}

// Stack — generic stack
// Stack — generic стек
#[derive(Debug)]
struct Stack<T> {
    elementlar: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { elementlar: Vec::new() }
    }

    fn push(&mut self, element: T) {
        self.elementlar.push(element);
    }

    fn pop(&mut self) -> Option<T> {
        self.elementlar.pop()
    }

    fn tepa(&self) -> Option<&T> {
        self.elementlar.last()
    }

    fn bo_shmi(&self) -> bool {
        self.elementlar.is_empty()
    }

    fn uzunlik(&self) -> usize {
        self.elementlar.len()
    }
}

// generic trait
// generic трейт
trait Aylantirish<T> {
    fn aylantir(&self) -> T;
}

#[derive(Debug, Clone)]
struct Daraja(f64);

impl Aylantirish<f64> for Daraja {
    fn aylantir(&self) -> f64 {
        self.0
    }
}

impl Aylantirish<String> for Daraja {
    fn aylantir(&self) -> String {
        format!("{:.1}°", self.0)
    }
}

// generic trait — impl sifatida qaytarish
// generic трейт — возврат как impl
trait Ikkilash {
    fn ikkilash(&self) -> Self;
}

impl Ikkilash for i32 {
    fn ikkilash(&self) -> i32 {
        self * 2
    }
}

impl Ikkilash for String {
    fn ikkilash(&self) -> String {
        format!("{}{}", self, self)
    }
}

fn ikki_baravar<T: Ikkilash>(qiymat: T) -> T {
    qiymat.ikkilash()
}

// Bu generic funksiya:
// Эта generic функция:
fn qo_shish<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
// Kompilyator shunday kod yaratadi:
// Компилятор создаёт такой код:
// fn qo_shish_i32(a: i32, b: i32) -> i32 { a + b }
// fn qo_shish_f64(a: f64, b: f64) -> f64 { a + b }
// fn qo_shish_String(...) → String { a + b }

// N: usize — kompilyatsiya vaqtida aniq bo'lishi shart
// N: usize — должен быть известен на этапе компиляции
#[derive(Debug)]
struct AniqArray<T, const N: usize> {
    ichki: [T; N],
}

impl<T: Default + Copy + fmt::Debug, const N: usize> AniqArray<T, N> {
    fn new() -> Self {
        AniqArray { ichki: [T::default(); N] }
    }

    fn uzunlik(&self) -> usize {
        N
    }
}

impl<T: Copy, const N: usize> AniqArray<T, N> {
    fn to_ldirish(mut self, qiymat: T) -> Self {
        self.ichki = [qiymat; N];
        self
    }
}

// const generic funksiya
// const generic функция
fn array_yig_indisi<const N: usize>(arr: &[i32; N]) -> i32 {
    arr.iter().sum()
}

// generic + lifetime birga
// generic + lifetime вместе
#[derive(Debug)]
struct Referens<'a, T> {
    qiymat: &'a T,
}

impl<'a, T: fmt::Display> Referens<'a, T> {
    fn new(qiymat: &'a T) -> Self {
        Referens { qiymat }
    }

    fn chiqar(&self) {
        println!("{}", self.qiymat);
    }
}

// funksiyada generic + lifetime
// generic + lifetime в функции
fn eng_uzun<'a, T>(x: &'a [T], y: &'a [T]) -> &'a [T] {
    if x.len() > y.len() { x } else { y }
}

// generic cache
// generic кэш
use std::collections::HashMap;

struct Cache<K, V> {
    ichki: HashMap<K, V>,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> Cache<K, V> {
    fn new() -> Self {
        Cache { ichki: HashMap::new() }
    }

    fn qo_sh(&mut self, kalit: K, qiymat: V) {
        self.ichki.insert(kalit, qiymat);
    }

    fn ol(&self, kalit: &K) -> Option<&V> {
        self.ichki.get(kalit)
    }

    fn o_chir(&mut self, kalit: &K) -> Option<V> {
        self.ichki.remove(kalit)
    }
}

// generic pipeline
// generic конвейер
struct Pipeline<T> {
    qiymat: T,
}

impl<T> Pipeline<T> {
    fn new(qiymat: T) -> Self {
        Pipeline { qiymat }
    }

    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Pipeline<U> {
        Pipeline { qiymat: f(self.qiymat) }
    }

    fn natija(self) -> T {
        self.qiymat
    }
}

// generic validator
// generic валидатор
struct Validator<T> {
    qiymat: T,
    xatolar: Vec<String>,
}

impl<T: Clone + fmt::Debug> Validator<T> {
    fn new(qiymat: T) -> Self {
        Validator { qiymat, xatolar: Vec::new() }
    }

    fn tekshir<F: Fn(&T) -> bool>(mut self, shart: F, xabar: &str) -> Self {
        if !shart(&self.qiymat) {
            self.xatolar.push(xabar.to_string());
        }
        self
    }

    fn tasdiqlash(self) -> Result<T, Vec<String>> {
        if self.xatolar.is_empty() {
            Ok(self.qiymat)
        } else {
            Err(self.xatolar)
        }
    }
}

fn main() {

    // birinchi — turli turlar
    // birinchi — разные типы
    let sonlar: Vec<i32> = vec![10, 20, 30];
    let sozlar: Vec<&str> = vec!["salom", "dunyo", "rust"];
    println!("{}", birinchi(&sonlar));
    println!("{}", birinchi(&sozlar));
    // 10
    // salom

    // eng_katta — turli turlar
    // eng_katta — разные типы
    let katta_son: i32 = eng_katta(10, 20);
    let katta_float: f64 = eng_katta(3.14, 2.72);
    let katta_char: char = eng_katta('z', 'a');
    println!("{}", katta_son);
    println!("{}", katta_float);
    println!("{}", katta_char);
    // 20
    // 3.14
    // z

    // juft — turli kombinatsiyalar
    // juft — разные комбинации
    let j1: (i32, &str) = juft(42, "salom");
    let j2: (f64, bool) = juft(3.14, true);
    println!("{:?}", j1);
    println!("{:?}", j2);
    // (42, "salom")
    // (3.14, true)

    // chiqar_va_qaytarish
    let natija_son: i32 = chiqar_va_qaytarish(42i32);
    let natija_str: String = chiqar_va_qaytarish(String::from("salom"));
    println!("{}", natija_son);
    println!("{}", natija_str);
    // 42
    // salom
    // 42
    // salom

    // Quti — turli turlar
    // Quti — разные типы
    let q_son: Quti<i32> = Quti::new(42);
    let q_str: Quti<String> = Quti::new(String::from("salom"));
    let q_vec: Quti<Vec<i32>> = Quti::new(vec![1, 2, 3]);
    println!("{:?}", q_son);
    println!("{:?}", q_str);
    println!("{:?}", q_vec);
    // Quti { ichki: 42 }
    // Quti { ichki: "salom" }
    // Quti { ichki: [1, 2, 3] }

    // Quti metodlari
    // методы Quti
    println!("{}", q_son.qiymat());
    q_son.chiqar();
    let qiymat: i32 = q_son.ichki_ga_aylantirish();
    println!("{}", qiymat);
    // 42
    // Quti(42)
    // 42

    // Juft — ikki tur
    // Juft — два типа
    let j: Juft<i32, &str> = Juft::new(42, "rust");
    j.chiqar();
    println!("{:?}", j);
    // (42, rust)
    // Juft { birinchi: 42, ikkinchi: "rust" }

    // Natija — custom Result
    // Natija — пользовательский Result
    let m: Natija<i32, String> = Natija::Muvaffaqiyat(42);
    let x: Natija<i32, String> = Natija::Xato(String::from("xato yuz berdi"));
    m.chiqar();
    x.chiqar();
    println!("{}", m.muvaffaqiyatlimi());
    println!("{}", x.muvaffaqiyatlimi());
    // Ok: 42
    // Err: xato yuz berdi
    // true
    // false

    // Stack — generic stack
    // Stack — generic стек
    let mut stack: Stack<i32> = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);
    println!("Tepa: {:?}", stack.tepa());
    println!("Uzunlik: {}", stack.uzunlik());
    println!("Pop: {:?}", stack.pop());
    println!("Pop: {:?}", stack.pop());
    println!("Uzunlik: {}", stack.uzunlik());
    // Tepa: Some(30)
    // Uzunlik: 3
    // Pop: Some(30)
    // Pop: Some(20)
    // Uzunlik: 1

    // Aylantirish — bir struct, ko'p tur
    // Aylantirish — одна структура, много типов
    let daraja = Daraja(36.6);
    let f64_qiymati: f64 = daraja.aylantir();
    let string_qiymati: String = daraja.aylantir();
    println!("{}", f64_qiymati);
    println!("{}", string_qiymati);
    // 36.6
    // 36.6°

    // ikki_baravar — turli turlar
    // ikki_baravar — разные типы
    let son_ikkisi: i32 = ikki_baravar(21i32);
    let matn_ikkisi: String = ikki_baravar(String::from("salom"));
    println!("{}", son_ikkisi);
    println!("{}", matn_ikkisi);
    // 42
    // salomsalom

    // qo'shish — turli turlar uchun alohida kod yaratiladi
    // для каждого типа создаётся отдельный код
    let i32_sum: i32 = qo_shish(10, 20);
    let f64_sum: f64 = qo_shish(1.5, 2.5);
    let str_sum: String = format!("{}{}", "salom ", "dunyo");
    println!("{}", i32_sum);
    println!("{}", f64_sum);
    println!("{}", str_sum);
    // 30
    // 4
    // salom dunyo

    // AniqArray — kompilyatsiya vaqtida o'lcham
    // AniqArray — размер во время компиляции
    let arr3: AniqArray<i32, 3> = AniqArray::new().to_ldirish(5);
    let arr5: AniqArray<f64, 5> = AniqArray::new().to_ldirish(3.14);
    println!("{:?}", arr3);
    println!("{:?}", arr5);
    println!("Uzunlik: {}", arr3.uzunlik());
    println!("Uzunlik: {}", arr5.uzunlik());
    // AniqArray { ichki: [5, 5, 5] }
    // AniqArray { ichki: [3.14, 3.14, 3.14, 3.14, 3.14] }
    // Uzunlik: 3
    // Uzunlik: 5

    // array_yig_indisi — const generic funksiya
    // array_yig_indisi — const generic функция
    let arr_3: [i32; 3] = [1, 2, 3];
    let arr_5: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", array_yig_indisi(&arr_3));
    println!("{}", array_yig_indisi(&arr_5));
    // 6
    // 15

    // Referens — generic + lifetime
    // Referens — generic + lifetime
    let son: i32 = 42;
    let r: Referens<i32> = Referens::new(&son);
    r.chiqar();
    println!("{:?}", r);
    // 42
    // Referens { qiymat: 42 }

    // eng_uzun — uzunroq slice
    // eng_uzun — более длинный slice
    let v1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let v2: Vec<i32> = vec![10, 20, 30];
    let uzun: &[i32] = eng_uzun(&v1, &v2);
    println!("{:?}", uzun);
    // [1, 2, 3, 4, 5]

    // Cache — generic kalit va qiymat
    // Cache — generic ключ и значение
    let mut cache: Cache<String, i32> = Cache::new();
    cache.qo_sh(String::from("bir"), 1);
    cache.qo_sh(String::from("ikki"), 2);
    cache.qo_sh(String::from("uch"), 3);
    println!("{:?}", cache.ol(&String::from("ikki")));
    println!("{:?}", cache.ol(&String::from("tort")));
    // Some(2)
    // None

    // Pipeline — generic transformatsiya zanjiri
    // Pipeline — generic цепочка трансформаций
    let natija: String = Pipeline::new(5i32)
        .map(|x| x * 2)
        .map(|x| x + 1)
        .map(|x| x.to_string())
        .map(|s| format!("Natija: {}", s))
        .natija();
    println!("{}", natija);
    // Natija: 11

    // Validator — generic validatsiya
    // Validator — generic валидация
    let v_ok = Validator::new(25i32)
        .tekshir(|&x| x > 0, "Musbat bo'lishi kerak")
        .tekshir(|&x| x < 100, "100 dan kichik bo'lishi kerak")
        .tasdiqlash();
    println!("{:?}", v_ok);
    // Ok(25)

    let v_err = Validator::new(-5i32)
        .tekshir(|&x| x > 0, "Musbat bo'lishi kerak")
        .tekshir(|&x| x < 100, "100 dan kichik bo'lishi kerak")
        .tasdiqlash();
    println!("{:?}", v_err);
    // Err(["Musbat bo'lishi kerak"])
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                       GENERIC ASOSLARI                                                                                       |
// #================================================================================================================================================#
// # |   1 | fn f<T>(x: T)            | Generic funksiya — T har qanday tur                  | Generic функция — T любого типа                      |
// # |   2 | struct S<T>              | Generic struct — T bilan                             | Generic структура — с T                              |
// # |   3 | enum E<T>                | Generic enum — T bilan                               | Generic перечисление — с T                           |
// # |   4 | impl<T> S<T>             | Generic struct uchun impl                            | impl для generic структуры                           |
// # |   5 | <T, U>                   | Bir nechta generic parametr                          | Несколько generic параметров                         |
// #================================================================================================================================================#
// # |                                          BOUNDLAR                                                                                            |
// #================================================================================================================================================#
// # |   6 | T: Trait                 | T faqat Trait implement qilgan bo'lishi kerak        | T должен реализовать Trait                           |
// # |   7 | T: Trait1 + Trait2       | Bir nechta bound                                     | Несколько ограничений                                |
// # |   8 | where T: Trait           | Where clause — murakkab boundlar uchun               | Where clause — для сложных ограничений               |
// # |   9 | impl<T: Trait> S<T>      | Faqat T: Trait bo'lganda metod qo'shish              | Добавление метода только когда T: Trait              |
// #================================================================================================================================================#
// # |                                       CONST GENERICS                                                                                         |
// #================================================================================================================================================#
// # |  10 | struct S<const N: usize> | Kompilyatsiya vaqtida aniq qiymat                    | Конкретное значение на этапе компиляции              |
// # |  11 | fn f<const N: usize>     | Const generic funksiya                               | Const generic функция                                |
// # |  12 | [T; N]                   | Array o'lchami — const generic bilan                 | Размер массива — через const generic                 |
// #================================================================================================================================================#
// # |                                       MONOMORPHIZATION                                                                                       |
// #================================================================================================================================================#
// # |  13 | Zero-cost abstraction    | Runtime xarajat yo'q — kompilyatsiya vaqtida hal     | Нет затрат в runtime — решается при компиляции       |
// # |  14 | fn f<T> → fn f_i32, f_f64| Har tur uchun alohida kod yaratiladi                 | Для каждого типа создаётся отдельный код             |
// #================================================================================================================================================#
// # |                                     GENERIC + LIFETIME                                                                                       |
// #================================================================================================================================================#
// # |  15 | struct S<'a, T>          | Generic + lifetime birga                             | Generic + lifetime вместе                            |
// # |  16 | fn f<'a, T>(x: &'a T)    | Funksiyada generic + lifetime                        | Generic + lifetime в функции                         |
// #================================================================================================================================================#
// # |                                       REAL HAYOT                                                                                             |
// #================================================================================================================================================#
// # |  17 | Cache<K, V>              | Generic kalit-qiymat xotira                          | Generic хранилище ключ-значение                      |
// # |  18 | Pipeline<T>              | Generic transformatsiya zanjiri                      | Generic цепочка трансформаций                        |
// # |  19 | Validator<T>             | Generic validatsiya                                  | Generic валидация                                    |
// # |  20 | Stack<T>                 | Generic stek ma'lumot strukturasi                    | Generic структура данных стек                        |
// #================================================================================================================================================#