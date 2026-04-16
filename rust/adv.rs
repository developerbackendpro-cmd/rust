// #================================================================================================================================================#
// #                                                            ADV GENERICS                                                                        #
// #                        YUQORI DARAJALI GENERICS — WHERE CLAUSE, ASSOCIATED TYPES, GAT, PHANTOM DATA, TURLI PATTERNLAR.                         #
// #                        GENERICS ПРОДВИНУТЫЙ — WHERE CLAUSE, ASSOCIATED TYPES, GAT, PHANTOM DATA, РАЗНЫЕ ПАТТЕРНЫ.                              #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::marker::PhantomData;
use std::ops::Add;

// Oddiy bound
// Обычное ограничение
fn oddiy<T: fmt::Display + Clone>(x: T) -> T {
    println!("{}", x);
    x.clone()
}

// Where clause — murakkab boundlar uchun
// Where clause — для сложных ограничений
fn murakkab<T, U, V>(t: T, u: U) -> V
where
    T: fmt::Display + Clone + Into<V>,
    U: fmt::Debug + PartialEq<T>,
    V: fmt::Display + Default,
{
    if u == t {
        t.into()
    } else {
        V::default()
    }
}

// Where clause — trait methodida
// Where clause — в методе трейта
trait Ishlov {
    fn qayta_ishla<T>(&self, qiymat: T) -> String
    where
        T: fmt::Display + fmt::Debug + Clone,
    {
        format!("Display:{} Debug:{:?}", qiymat, qiymat.clone())
    }
}

// Where clause — impl blokda
// Where clause — в блоке impl
struct Juftlik<A, B>(A, B);

impl<A, B> fmt::Display for Juftlik<A, B>
where
    A: fmt::Display,
    B: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl<A, B> Juftlik<A, B>
where
    A: Clone + PartialOrd,
    B: Clone,
{
    fn new(a: A, b: B) -> Self { Juftlik(a, b) }
    fn birinchi(&self) -> A { self.0.clone() }
    fn ikkinchi(&self) -> B { self.1.clone() }
}

fn where_clause_misollari() {

    let j: Juftlik<i32, &str> = Juftlik::new(42, "salom");
    println!("{}", j);
    println!("{} {}", j.birinchi(), j.ikkinchi());
    // (42, salom)
    // 42 salom

    let natija: i32 = oddiy(100);
    println!("{}", natija);
    // 100
    // 100
}

// Generic vs Associated Type farqi:
// Разница Generic vs Associated Type:
//
//   trait Converter<T> { fn convert(&self) -> T; }
//   → bitta tur bir nechta T uchun implement qilishi mumkin
//   → один тип может реализовать для нескольких T
//
//   trait Converter { type Output; fn convert(&self) -> Self::Output; }
//   → bitta tur faqat bitta Output bilan implement qilishi mumkin
//   → один тип может реализовать только с одним Output

trait Aylantir {
    type Chiqish;
    fn aylantir(self) -> Self::Chiqish;
}

struct Gradus(f64);
struct Radian(f64);

impl Aylantir for Gradus {
    type Chiqish = Radian;
    fn aylantir(self) -> Radian {
        Radian(self.0 * std::f64::consts::PI / 180.0)
    }
}

impl Aylantir for Radian {
    type Chiqish = Gradus;
    fn aylantir(self) -> Gradus {
        Gradus(self.0 * 180.0 / std::f64::consts::PI)
    }
}

// Associated type bilan generic bound
// Generic ограничение с associated type
fn aylantir_va_chiqar<T>(qiymat: T) -> T::Chiqish
where
    T: Aylantir,
    T::Chiqish: fmt::Debug,
{
    let natija = qiymat.aylantir();
    println!("{:?}", natija);
    natija
}

impl fmt::Debug for Gradus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}°", self.0)
    }
}

impl fmt::Debug for Radian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.4} rad", self.0)
    }
}

// Iterator — associated type misoli
// Iterator — пример associated type
struct Qadamlar {
    joriy: i32,
    qadam: i32,
    max: i32,
}

impl Qadamlar {
    fn new(boshlanish: i32, qadam: i32, max: i32) -> Self {
        Qadamlar { joriy: boshlanish, qadam, max }
    }
}

impl Iterator for Qadamlar {
    type Item = i32;  // Associated type

    fn next(&mut self) -> Option<i32> {
        if self.joriy <= self.max {
            let qiymat = self.joriy;
            self.joriy += self.qadam;
            Some(qiymat)
        } else {
            None
        }
    }
}

fn associated_types_misollari() {

    let g = Gradus(180.0);
    let r: Radian = aylantir_va_chiqar(g);
    // 3.1416 rad

    let r2 = Radian(std::f64::consts::PI / 2.0);
    let g2: Gradus = aylantir_va_chiqar(r2);
    // 90.00°

    // Iterator associated type
    let qadamlar = Qadamlar::new(0, 5, 20);
    let v: Vec<i32> = qadamlar.collect();
    println!("{:?}", v);
    // [0, 5, 10, 15, 20]
}

// PhantomData — runtime qiymati yo'q, faqat tur ma'lumoti
// PhantomData — нет значения в runtime, только информация о типе

// Type state pattern — PhantomData bilan
// Паттерн type state — с PhantomData
struct Qulflanmagan;
struct Qullangan;

struct Resurs<Holat> {
    qiymat: String,
    _holat: PhantomData<Holat>,
}

impl Resurs<Qulflanmagan> {
    fn new(qiymat: &str) -> Self {
        Resurs { qiymat: qiymat.to_string(), _holat: PhantomData }
    }

    fn qulflash(self) -> Resurs<Qullangan> {
        println!("Resurs qullandi");
        Resurs { qiymat: self.qiymat, _holat: PhantomData }
    }
}

impl Resurs<Qullangan> {
    fn ishlatish(&self) -> &str {
        &self.qiymat
    }

    fn ochish(self) -> Resurs<Qulflanmagan> {
        println!("Resurs ochildi");
        Resurs { qiymat: self.qiymat, _holat: PhantomData }
    }
}

// Birliklari bo'lgan son — PhantomData bilan
// Число с единицами — с PhantomData
struct Metr;
struct Kilogramm;
struct Sekund;

struct Olchov<Birlik> {
    qiymat: f64,
    _birlik: PhantomData<Birlik>,
}

impl<Birlik> Olchov<Birlik> {
    fn new(qiymat: f64) -> Self {
        Olchov { qiymat, _birlik: PhantomData }
    }
    fn qiymat(&self) -> f64 { self.qiymat }
}

impl Add for Olchov<Metr> {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Olchov::new(self.qiymat + b.qiymat)
    }
}

// Olchov<Metr> + Olchov<Kilogramm> — kompile bo'lmaydi! (tur xavfsizligi)
// Olchov<Metr> + Olchov<Kilogramm> — не скомпилируется! (безопасность типов)

impl fmt::Display for Olchov<Metr> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} m", self.qiymat)
    }
}

fn phantom_data_misollari() {

    // Type state
    let r: Resurs<Qulflanmagan> = Resurs::new("muhim ma'lumot");
    let qullangan: Resurs<Qullangan> = r.qulflash();
    println!("{}", qullangan.ishlatish());
    let ochilgan: Resurs<Qulflanmagan> = qullangan.ochish();
    // Resurs qullandi
    // muhim ma'lumot
    // Resurs ochildi

    // Birliklar
    let a: Olchov<Metr> = Olchov::new(5.0);
    let b: Olchov<Metr> = Olchov::new(3.0);
    let c: Olchov<Metr> = a + b;
    println!("{}", c);
    // 8 m

    // O'lcham — PhantomData zero-cost
    println!("Olchov<Metr>: {} bayt", std::mem::size_of::<Olchov<Metr>>());
    println!("f64:           {} bayt", std::mem::size_of::<f64>());
    // Olchov<Metr>: 8 bayt  ← bir xil! PhantomData zero-cost
    // f64:           8 bayt
}

// Newtype — tashqi turdagi traitni implement qilish
// Newtype — реализация трейта для внешнего типа
struct Wrapper<T>(Vec<T>);

impl<T: fmt::Display> fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", "))
    }
}

// Turli xil generic constraintlar
// Различные generic ограничения
fn eng_katta_eng_kichik<T>(v: &[T]) -> Option<(&T, &T)>
where
    T: PartialOrd,
{
    if v.is_empty() { return None; }
    let mut katta = &v[0];
    let mut kichik = &v[0];
    for x in v.iter() {
        if x > katta { katta = x; }
        if x < kichik { kichik = x; }
    }
    Some((katta, kichik))
}

fn real_hayot_misollari() {

    // Where clause
    where_clause_misollari();

    // Associated types
    associated_types_misollari();

    // PhantomData
    phantom_data_misollari();

    // Wrapper newtype
    let w: Wrapper<i32> = Wrapper(vec![1, 2, 3, 4, 5]);
    let w2: Wrapper<&str> = Wrapper(vec!["salom", "dunyo", "rust"]);
    println!("{}", w);
    println!("{}", w2);
    // [1, 2, 3, 4, 5]
    // [salom, dunyo, rust]

    // eng_katta_eng_kichik
    let v: Vec<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6];
    if let Some((katta, kichik)) = eng_katta_eng_kichik(&v) {
        println!("Max:{} Min:{}", katta, kichik);
    }
    // Max:9 Min:1

    let sozlar: Vec<&str> = vec!["banan", "olma", "anor", "nok"];
    if let Some((katta, kichik)) = eng_katta_eng_kichik(&sozlar) {
        println!("Max:{} Min:{}", katta, kichik);
    }
    // Max:olma Min:anor
}

fn main() {
    println!("=== ADV GENERICS ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |   1 | where T: A + B, U: C            | Murakkab generic boundlar                  | Сложные generic ограничения                             |
// # |   2 | type Output = T                 | Associated type — bir xil impl             | Associated type — одна реализация                       |
// # |   3 | T::Output (assoc type bound)    | Associated type bilan constraint           | Ограничение через associated type                       |
// # |   4 | PhantomData<T>                  | Runtime qiymati yo'q, tur ma'lumoti        | Нет значения в runtime, информация о типе               |
// # |   5 | Type state pattern              | PhantomData bilan holat mashinasi          | Машина состояний с PhantomData                          |
// # |   6 | Birlik turi                     | Olchov<Metr> vs O_lchov<Kg> — xavfsiz      | Olchov<Metr> vs O_lchov<Kg> — безопасно                 |
// # |   7 | Newtype<T>                      | Tashqi tur uchun trait impl                | Реализация трейта для внешнего типа                     |
// # |   8 | where Self: Sized               | Object-unsafe metodlarni chiqarish         | Исключение object-unsafe методов                        |
// # |   9 | Generic vs Associated           | Generic — ko'p impl, Assoc — bitta impl    | Generic — много impl, Assoc — одна impl                 |
// # |  10 | Blanket + Where clause          | Kuchli kombinatsiya                        | Мощная комбинация                                       |
// #================================================================================================================================================#