// #================================================================================================================================================#
// #                                                            DEREF  |  DEREFMUT                                                                  #
// #                        DEREF — * OPERATORI ORQALI ICHIDAGI QIYMATGA KIRISH. DEREF COERCION — AVTOMATIK AYLANTIRISH.                            #
// #                        DEREF — ДОСТУП К ВНУТРЕННЕМУ ЗНАЧЕНИЮ ЧЕРЕЗ ОПЕРАТОР *. DEREF COERCION — АВТОМАТИЧЕСКОЕ ПРИВЕДЕНИЕ.                     #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::ops::{Deref, DerefMut};
use std::fmt;

// Deref trait:
//   trait Deref {
//       type Target: ?Sized;
//       fn deref(&self) -> &Self::Target;
//   }
//
// DerefMut trait:
//   trait DerefMut: Deref {
//       fn deref_mut(&mut self) -> &mut Self::Target;
//   }
//
// Deref coercion — kompilyator avtomatik aylantiradi:
// Deref coercion — компилятор автоматически приводит:
//   &T → &U  (agar T: Deref<Target=U>)
//   &mut T → &U  (agar T: Deref<Target=U>)
//   &mut T → &mut U  (agar T: DerefMut<Target=U>)
//
// Built-in Deref:
//   String: Deref<Target=str>    → &String → &str
//   Vec<T>: Deref<Target=[T]>    → &Vec<T> → &[T]
//   Box<T>: Deref<Target=T>      → &Box<T> → &T
//   Rc<T>:  Deref<Target=T>      → &Rc<T>  → &T
//   Arc<T>: Deref<Target=T>      → &Arc<T> → &T

fn star_operator_misollari() {

    // Reference — * orqali qiymatga kirish
    // Reference — доступ к значению через *
    let x: i32 = 42;
    let r: &i32 = &x;
    println!("{}", *r);
    println!("{}", r);   // Display orqali ham ishlaydi
    // 42
    // 42

    // * — o'zgartirish uchun
    // * — для изменения
    let mut y: i32 = 10;
    let r_mut: &mut i32 = &mut y;
    *r_mut += 5;
    println!("{}", y);
    // 15

    // Box — * orqali ichki qiymat
    // Box — внутреннее значение через *
    let b: Box<i32> = Box::new(99);
    println!("{}", *b);
    println!("{}", b);   // Deref coercion
    // 99
    // 99

    // Box<String> — ikki marta deref
    // Box<String> — двойной deref
    let bs: Box<String> = Box::new(String::from("salom"));
    let s_ref: &str = &**bs;
    // &**bs = & * (*bs) = &String → &str
    // &**bs = & * (*bs) = &String → &str
    println!("{}", s_ref);
    // salom

    // String — * orqali str
    // String — str через *
    let s: String = String::from("dunyo");
    let str_ref: &str = &*s;
    // &*s = & * s = &str
    println!("{}", str_ref);
    // dunyo

    // Vec — * orqali slice
    // Vec — slice через *
    let v: Vec<i32> = vec![1, 2, 3];
    let slice: &[i32] = &*v;
    println!("{:?}", slice);
    // [1, 2, 3]
}

fn deref_coercion_misollari() {

    // String → &str — deref coercion
    // String → &str — deref coercion
    fn str_qabul(s: &str) {
        println!("str: {}", s);
    }

    let owned: String = String::from("salom");
    str_qabul(&owned);         // &String → &str (deref coercion)
    str_qabul(&owned[..]);     // ochiq yozuv
    str_qabul(owned.as_str()); // aniq yozuv
    // str: salom
    // str: salom
    // str: salom

    // Vec<T> → &[T] — deref coercion
    // Vec<T> → &[T] — deref coercion
    fn slice_qabul(s: &[i32]) {
        println!("slice: {:?}", s);
    }

    let v: Vec<i32> = vec![1, 2, 3];
    slice_qabul(&v);            // &Vec<i32> → &[i32]
    slice_qabul(&v[..]);        // ochiq yozuv
    slice_qabul(v.as_slice());  // aniq yozuv
    // slice: [1, 2, 3]
    // slice: [1, 2, 3]
    // slice: [1, 2, 3]

    // Box<T> → &T — deref coercion
    // Box<T> → &T — deref coercion
    fn i32_qabul(n: &i32) {
        println!("i32: {}", n);
    }

    let b: Box<i32> = Box::new(42);
    i32_qabul(&b);  // &Box<i32> → &i32
    // i32: 42

    // Zanjirli deref coercion
    // Цепочечный deref coercion
    let bs: Box<String> = Box::new(String::from("zanjir"));
    str_qabul(&bs);
    // &Box<String> → &String → &str  (ikki marta coercion)
    // &Box<String> → &String → &str  (двойное приведение)
    // str: zanjir

    // Rc<String> → &str
    // Rc<String> → &str
    use std::rc::Rc;
    let rc: Rc<String> = Rc::new(String::from("rc salom"));
    str_qabul(&rc);
    // &Rc<String> → &String → &str
    // str: rc salom

    // Metod chaqirish — deref coercion avtomatik
    // Вызов методов — deref coercion автоматически
    let s: String = String::from("SALOM DUNYO");
    // String metodlari + str metodlari — ikkalasi ishlaydi
    // методы String + методы str — работают оба
    println!("{}", s.to_lowercase());  // String metodi
    println!("{}", s.len());           // str metodi (deref orqali)
    println!("{}", s.contains("SAL")); // str metodi (deref orqali)
    // salom dunyo
    // 11
    // true
}

// Oddiy newtype — Deref bilan
// Простой newtype — с Deref
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: fmt::Display> fmt::Display for MyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyBox({})", self.0)
    }
}

// Stack — Deref bilan Vec metodlarini olish
// Stack — получение методов Vec через Deref
#[derive(Debug)]
struct Stek<T> {
    ichki: Vec<T>,
}

impl<T> Stek<T> {
    fn new() -> Self {
        Stek { ichki: Vec::new() }
    }

    fn push(&mut self, val: T) {
        self.ichki.push(val);
    }

    fn pop(&mut self) -> Option<T> {
        self.ichki.pop()
    }
}

impl<T> Deref for Stek<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.ichki
    }
}

impl<T> DerefMut for Stek<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.ichki
    }
}

// Konfiguratsiya wrapper — Deref bilan HashMap metodlarini olish
// Обёртка конфигурации — получение методов HashMap через Deref
use std::collections::HashMap;

#[derive(Debug)]
struct Konfiguratsiya {
    ichki: HashMap<String, String>,
}

impl Konfiguratsiya {
    fn new() -> Self {
        Konfiguratsiya { ichki: HashMap::new() }
    }

    fn qo_sh(&mut self, kalit: &str, qiymat: &str) {
        self.ichki.insert(kalit.to_string(), qiymat.to_string());
    }
}

impl Deref for Konfiguratsiya {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &HashMap<String, String> {
        &self.ichki
    }
}

impl DerefMut for Konfiguratsiya {
    fn deref_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.ichki
    }
}

fn custom_deref_misollari() {

    // MyBox — * orqali ichki qiymat
    // MyBox — внутреннее значение через *
    let mb: MyBox<i32> = MyBox::new(42);
    println!("{}", *mb);
    println!("{}", mb);
    // 42
    // MyBox(42)

    // MyBox<String> — deref coercion
    // MyBox<String> — deref coercion
    let mb_s: MyBox<String> = MyBox::new(String::from("salom"));

    fn str_qabul(s: &str) { println!("{}", s); }
    str_qabul(&mb_s);  // &MyBox<String> → &String → &str
    // salom

    // MyBox — DerefMut
    // MyBox — DerefMut
    let mut mb_mut: MyBox<i32> = MyBox::new(10);
    *mb_mut += 5;
    println!("{}", *mb_mut);
    // 15

    // Stek — Vec metodlarini Deref orqali olish
    // Stek — получение методов Vec через Deref
    let mut stek: Stek<i32> = Stek::new();
    stek.push(1);
    stek.push(2);
    stek.push(3);

    // Vec metodlari — Deref orqali
    // методы Vec — через Deref
    println!("{}", stek.len());       // Vec::len()
    println!("{:?}", stek.first());   // Vec'dan
    println!("{}", stek.is_empty());  // Vec'dan
    let yig: i32 = stek.iter().sum();
    println!("{}", yig);
    // 3
    // Some(1)
    // false
    // 6

    // Konfiguratsiya — HashMap metodlarini Deref orqali
    // Konfiguratsiya — методы HashMap через Deref
    let mut konfig: Konfiguratsiya = Konfiguratsiya::new();
    konfig.qo_sh("host", "localhost");
    konfig.qo_sh("port", "8080");

    // HashMap metodlari — Deref orqali
    // методы HashMap — через Deref
    println!("{:?}", konfig.get("host"));     // HashMap::get()
    println!("{}", konfig.contains_key("port")); // HashMap::contains_key()
    println!("{}", konfig.len());             // HashMap::len()
    // Some("localhost")
    // true
    // 2
}

fn deref_vs_borrow() {

    // Deref — * operatori, tur konversiyasi
    // Deref — оператор *, преобразование типа
    // Borrow — borrow qilish, bir xil tur

    let s: String = String::from("salom");

    // Deref — String → str (tur o'zgaradi)
    // Deref — String → str (тип меняется)
    let str_ref: &str = &*s;
    let str_ref2: &str = s.deref();
    println!("{} {}", str_ref, str_ref2);
    // salom salom

    // Borrow — String → String (tur bir xil)
    // Borrow — String → String (тип не меняется)
    use std::borrow::Borrow;
    let str_borrow: &str = s.borrow();  // String: Borrow<str>
    println!("{}", str_borrow);
    // salom

    // AsRef — Deref ga o'xshash lekin Hash/Eq kafolat yo'q
    // AsRef — похоже на Deref, но без гарантии Hash/Eq
    let as_ref_str: &str = s.as_ref();
    println!("{}", as_ref_str);
    // salom
}

fn real_hayot_misollari() {

    // 1. Arc<Mutex<T>> — Deref zanjiri
    // 1. Arc<Mutex<T>> — цепочка Deref
    use std::sync::{Arc, Mutex};
    let shared: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(vec![1, 2, 3]));
    {
        let mut guard = shared.lock().unwrap();
        guard.push(4);  // MutexGuard → &mut Vec → .push()
    }
    println!("{:?}", shared.lock().unwrap().as_slice());
    // [1, 2, 3, 4]

    // 2. Smart pointer chain
    // 2. Цепочка умных указателей
    use std::rc::Rc;
    let rc_string: Rc<String> = Rc::new(String::from("rc string"));
    println!("{}", rc_string.to_uppercase());
    // &Rc<String> → &String → .to_uppercase()
    // RC STRING

    // 3. Funksiyaga turli xil qiymatlar berish
    // 3. Передача разных значений в функцию
    fn uzunlik(s: &str) -> usize { s.len() }

    let literal: &str = "salom";
    let owned: String = String::from("dunyo rust");
    let boxed: Box<String> = Box::new(String::from("ajoyib"));
    let rc: Rc<String> = Rc::new(String::from("rc qiymat"));

    println!("{}", uzunlik(literal));
    println!("{}", uzunlik(&owned));    // &String → &str
    println!("{}", uzunlik(&boxed));    // &Box<String> → &String → &str
    println!("{}", uzunlik(&rc));       // &Rc<String> → &String → &str
    // 5
    // 10
    // 7
    // 8

    // 4. Mut Deref — o'zgartirish
    // 4. Mut Deref — изменение
    let mut v: Box<Vec<i32>> = Box::new(vec![1, 2, 3]);
    v.push(4);  // &mut Box<Vec<i32>> → &mut Vec<i32> → .push()
    println!("{:?}", *v);
    // [1, 2, 3, 4]
}

fn main() {

    println!("=== * OPERATORI ===");
    star_operator_misollari();

    println!("\n=== DEREF COERCION ===");
    deref_coercion_misollari();

    println!("\n=== CUSTOM DEREF ===");
    custom_deref_misollari();

    println!("\n=== DEREF VS BORROW ===");
    deref_vs_borrow();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                   | Описание (RU)                                              |
// #================================================================================================================================================#
// # |                                       DEREF TRAIT                                                                                            |
// #================================================================================================================================================#
// # |   1 | impl Deref for T          | * operatori uchun                             | Для оператора *                                            |
// # |   2 | type Target = U;          | Deref chiqish turi                            | Тип результата Deref                                       |
// # |   3 | fn deref(&self) -> &U     | Ichki qiymatga reference                      | Ссылка на внутреннее значение                              |
// # |   4 | *val                      | val.deref() ga ekvivalent                     | Эквивалентно val.deref()                                   |
// #================================================================================================================================================#
// # |                                       DEREFMUT TRAIT                                                                                         |
// #================================================================================================================================================#
// # |   5 | impl DerefMut for T       | *mut operatori uchun                          | Для оператора *mut                                         |
// # |   6 | fn deref_mut(&mut self)   | Ichki qiymatga mut reference                  | Мутабельная ссылка на внутреннее значение                  |
// # |   7 | *val = new_val            | deref_mut() + o'zlashtirish                   | deref_mut() + присвоение                                   |
// #================================================================================================================================================#
// # |                                       DEREF COERCION                                                                                         |
// #================================================================================================================================================#
// # |   8 | &String → &str            | Avtomatik — String: Deref<Target=str>         | Автоматически — String: Deref<Target=str>                  |
// # |   9 | &Vec<T> → &[T]            | Avtomatik — Vec: Deref<Target=[T]>            | Автоматически — Vec: Deref<Target=[T]>                     |
// # |  10 | &Box<T> → &T              | Avtomatik — Box: Deref<Target=T>              | Автоматически — Box: Deref<Target=T>                       |
// # |  11 | Zanjirli coercion         | &Box<String> → &String → &str                 | &Box<String> → &String → &str                              |
// # |  12 | Metod chaqirish           | Avtomatik deref — metod qidiradi              | Автоматический deref — поиск метода                        |
// #================================================================================================================================================#
// # |                                       BUILT-IN DEREF                                                                                         |
// #================================================================================================================================================#
// # |  13 | Box<T>: Deref<Target=T>   | Box ni transparent qiladi                     | Делает Box прозрачным                                      |
// # |  14 | Rc<T>: Deref<Target=T>    | Rc ni transparent qiladi                      | Делает Rc прозрачным                                       |
// # |  15 | Arc<T>: Deref<Target=T>   | Arc ni transparent qiladi                     | Делает Arc прозрачным                                      |
// # |  16 | MutexGuard: DerefMut      | lock() dan keyin → &mut T                     | После lock() → &mut T                                      |
// #================================================================================================================================================#