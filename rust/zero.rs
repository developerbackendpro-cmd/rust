// #================================================================================================================================================#
// #                                                            ZERO-COST ABSTRACTIONS                                                              #
// #                        ZERO-COST — ABSTRAKTSIYA = XARAJATSIZ. MONOMORPHIZATION, INLINE, ITERATOR FUSION. BENCHMARK.                            #
// #                        ZERO-COST — АБСТРАКЦИЯ = БЕСПЛАТНО. МОНОРМОРФИЗАЦИЯ, INLINE, СЛИЯНИЕ ИТЕРАТОРОВ. БЕНЧМАРК.                              #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::time::Instant;

// Zero-Cost Abstractions nima:
// Что такое Zero-Cost Abstractions:
//
//   "What you don't use, you don't pay for.
//    What you do use, you couldn't hand-code any better."
//    — Bjarne Stroustrup
//
//   Rust da:
//   В Rust:
//   1. Generics → monomorphization (har tur uchun alohida kod)
//      Generics → мономорфизация (отдельный код для каждого типа)
//   2. Iterators → fusion (bitta loop ga birlashadi)
//      Iterators → слияние (объединяются в один цикл)
//   3. Closures → inline (funksiya chaqiruvi yo'q)
//      Closures → инлайнинг (нет вызова функции)
//   4. Traits → static dispatch (runtime yo'q)
//      Traits → статическая диспетчеризация (нет runtime)
//   5. Enums → tagged union (virtual table yo'q)
//      Enums → помеченное объединение (нет virtual table)

// Generic funksiya — har tur uchun alohida kod generatsiya qilinadi
// Generic функция — для каждого типа генерируется отдельный код
fn maksimal<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// Kompilyator quyidagilarni yaratadi (pseudo-kod):
// Компилятор создаёт следующее (псевдокод):
// fn maksimal_i32(a: i32, b: i32) -> i32 { ... }
// fn maksimal_f64(a: f64, b: f64) -> f64 { ... }
// fn maksimal_char(a: char, b: char) -> char { ... }
// Runtime overhead YO'Q!

fn monomorphization_misoli() {

    println!("--- Monomorphization ---");

    // Har chaqiruv — alohida type-specific kod
    // Каждый вызов — отдельный type-specific код
    let a = maksimal(10i32, 20i32);   // → maksimal_i32
    let b = maksimal(3.14f64, 2.71f64); // → maksimal_f64
    let c = maksimal('z', 'a');         // → maksimal_char
    let d = maksimal("salom", "dunyo"); // → maksimal_str

    println!("i32:  {}", a);  // 20
    println!("f64:  {}", b);  // 3.14
    println!("char: {}", c);  // z
    println!("str:  {}", d);  // salom
    // 20
    // 3.14
    // z
    // salom

    // Generics o'lchami — Zero overhead
    println!("\nO'lchamlar (zero overhead):");
    println!("Vec<i32>:  {} bayt", std::mem::size_of::<Vec<i32>>());   // 24
    println!("Vec<u8>:   {} bayt", std::mem::size_of::<Vec<u8>>());    // 24
    println!("Vec<f64>:  {} bayt", std::mem::size_of::<Vec<f64>>());   // 24
    // Barcha Vec bir xil struct o'lcham (24 bayt)

    // Benchmark: generic vs konkret
    let n = 10_000_000;
    let v: Vec<i32> = (0..n as i32).collect();

    let t1 = Instant::now();
    let _: i32 = v.iter().max().copied().unwrap_or(0);
    let vaqt_iter = t1.elapsed();

    let t2 = Instant::now();
    let mut maks = i32::MIN;
    for &x in &v { if x > maks { maks = x; } }
    let vaqt_for = t2.elapsed();

    println!("\n.max() vs for loop ({} element):", n);
    println!(".max():  {:?}", vaqt_iter);
    println!("for:     {:?}", vaqt_for);
    println!("Farq: ~0 (ikkalasi bir xil assembly!)");
}

// Iterator zanjiri bitta loop ga birlashadi
// Цепочка итераторов объединяется в один цикл
fn iterator_fusion_misoli() {

    println!("\n--- Iterator Fusion ---");

    let v: Vec<i32> = (1..=1_000_000).collect();

    // Bu KO'P LOOP emas — BITTA loop!
    // Это НЕ МНОГО ЦИКЛОВ — ОДИН цикл!
    let t1 = Instant::now();
    let natija: i32 = v.iter()
        .filter(|&&x| x % 2 == 0)      // lazy
        .map(|&x| x * x)                // lazy
        .filter(|&x| x % 3 == 0)        // lazy
        .take(100)                       // lazy
        .sum();                          // terminal → BITTA loop
    let vaqt_iter = t1.elapsed();

    // Ekvivalent — qo'lda yozilgan
    // Эквивалент — написанный вручную
    let t2 = Instant::now();
    let mut natija2: i32 = 0;
    let mut hisob = 0;
    for &x in &v {
        if x % 2 == 0 {
            let kv = x * x;
            if kv % 3 == 0 {
                natija2 += kv;
                hisob += 1;
                if hisob >= 100 { break; }
            }
        }
    }
    let vaqt_for = t2.elapsed();

    println!("Iterator zanjiri: {:?} (natija: {})", vaqt_iter, natija);
    println!("Qo'lda for loop:  {:?} (natija: {})", vaqt_for, natija2);
    println!("Bir xilmi: {}", natija == natija2);
    // Iterator zanjiri: ~Xms
    // Qo'lda for loop:  ~Xms
    // Bir xilmi: true

    // Oraliq Vec YARATILMAYDI
    // Промежуточные Vec НЕ СОЗДАЮТСЯ
    println!("\nXotira:");
    println!("filter().map().take().sum() — oraliq Vec yo'q!");
    println!("Vec::filter + Vec::map = 2x xotira (eager evaluation)");
}

// Closure — runtime overhead yo'q
// Closure — нет накладных расходов в runtime
#[inline(always)]
fn qo_sh(a: i32, b: i32) -> i32 { a + b }

fn inline_misoli() {

    println!("\n--- Inline & Closure Zero-Cost ---");

    // Closure va funksiya — bir xil assembly
    // Closure и функция — одинаковый assembly
    let n = 1_000_000;

    let t1 = Instant::now();
    let mut yig1 = 0i64;
    for i in 0..n { yig1 += qo_sh(i, i * 2) as i64; }
    let vaqt_fn = t1.elapsed();

    let t2 = Instant::now();
    let f = |a: i32, b: i32| a + b;
    let mut yig2 = 0i64;
    for i in 0..n { yig2 += f(i, i * 2) as i64; }
    let vaqt_closure = t2.elapsed();

    let t3 = Instant::now();
    let mut yig3 = 0i64;
    for i in 0..n { yig3 += (i + i * 2) as i64; }
    let vaqt_inline = t3.elapsed();

    println!("Funksiya:  {:?} ({})", vaqt_fn, yig1);
    println!("Closure:   {:?} ({})", vaqt_closure, yig2);
    println!("To'g'ridan:{:?} ({})", vaqt_inline, yig3);
    println!("Hammasi bir xil bytecode!");
    // Hammasi deyarli bir xil

    // Higher-order funksiya — zero-cost
    fn hisoblash<F: Fn(i32) -> i32>(v: &[i32], f: F) -> Vec<i32> {
        v.iter().map(|&x| f(x)).collect()
    }

    let v = vec![1, 2, 3, 4, 5];
    let kvadratlar = hisoblash(&v, |x| x * x);
    let ikkilangan = hisoblash(&v, |x| x * 2);
    println!("\nHOF natija: {:?}", kvadratlar); // [1, 4, 9, 16, 25]
    println!("HOF natija: {:?}", ikkilangan);   // [2, 4, 6, 8, 10]
    // [1, 4, 9, 16, 25]
    // [2, 4, 6, 8, 10]
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Metr(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Kilogramm(f64);

impl Metr {
    fn new(v: f64) -> Self { Metr(v) }
    fn qiymat(self) -> f64 { self.0 }
}

impl std::ops::Add for Metr {
    type Output = Self;
    fn add(self, b: Self) -> Self { Metr(self.0 + b.0) }
}

fn newtype_zero_cost_misoli() {

    println!("\n--- Newtype Zero-Cost ---");

    // Newtype — zero runtime overhead
    println!("Metr o'lcham:     {} bayt", std::mem::size_of::<Metr>());   // 8
    println!("f64 o'lcham:      {} bayt", std::mem::size_of::<f64>());    // 8
    // Bir xil! Newtype qo'shimcha xotira olmaydi

    let n = 1_000_000;
    let v_metr: Vec<Metr> = (0..n).map(|i| Metr(i as f64)).collect();
    let v_f64: Vec<f64> = (0..n).map(|i| i as f64).collect();

    let t1 = Instant::now();
    let _: f64 = v_metr.iter().map(|m| m.qiymat()).sum();
    let vaqt_metr = t1.elapsed();

    let t2 = Instant::now();
    let _: f64 = v_f64.iter().sum();
    let vaqt_f64 = t2.elapsed();

    println!("Vec<Metr> sum: {:?}", vaqt_metr);
    println!("Vec<f64> sum:  {:?}", vaqt_f64);
    println!("Farq: ~0 bayt, ~0 ms");
    // Bir xil natija!
}

// Enum — virtual table yo'q, tag + data
// Enum — нет virtual table, tag + data
#[derive(Debug)]
enum Shakl {
    Aylana(f64),
    Turtburchak(f64, f64),
    Uchburchak(f64, f64, f64),
}

impl Shakl {
    fn yuzi(&self) -> f64 {
        match self {
            Self::Aylana(r) => std::f64::consts::PI * r * r,
            Self::Turtburchak(e, b) => e * b,
            Self::Uchburchak(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

fn enum_zero_cost_misoli() {

    println!("\n--- Enum Zero-Cost (Tagged Union) ---");

    println!("Shakl o'lcham: {} bayt", std::mem::size_of::<Shakl>());
    println!("f64 o'lcham:   {} bayt", std::mem::size_of::<f64>());
    // Shakl: 32 bayt (tag + 3 * f64), virtual table yo'q!

    let shakllar = vec![
        Shakl::Aylana(5.0),
        Shakl::Turtburchak(4.0, 6.0),
        Shakl::Uchburchak(3.0, 4.0, 5.0),
    ];

    let n = 1_000_000;

    // Enum dispatch — static, inlined
    let t1 = Instant::now();
    let mut jami_enum = 0.0f64;
    for _ in 0..n { for sh in &shakllar { jami_enum += sh.yuzi(); } }
    let vaqt_enum = t1.elapsed();

    // dyn dispatch — runtime vtable
    let dyn_shakllar: Vec<Box<dyn Fn() -> f64>> = vec![
        Box::new(|| std::f64::consts::PI * 25.0),
        Box::new(|| 24.0),
        Box::new(|| 6.0),
    ];

    let t2 = Instant::now();
    let mut jami_dyn = 0.0f64;
    for _ in 0..n { for f in &dyn_shakllar { jami_dyn += f(); } }
    let vaqt_dyn = t2.elapsed();

    println!("Enum (static):  {:?}", vaqt_enum);
    println!("dyn Fn (vtable):{:?}", vaqt_dyn);
    println!("Enum odatda tezroq (virtual table yo'q)");
    // Enum odatda 2-5x tezroq!
}

// Const funksiyalar — compile time da bajariladi
// Const функции — выполняются во время компиляции
const fn faktorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n * faktorial(n - 1),
    }
}

const fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

const FAKT_10: u64 = faktorial(10);  // compile time!
const FIB_20: u64 = fibonacci(20);   // compile time!
const PRIMES: [u32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

fn const_misoli() {

    println!("\n--- Compile-Time Hisoblash ---");

    // Bu qiymatlar binary da HARDCODED
    // Эти значения HARDCODED в бинарнике
    println!("10! = {} (compile time)", FAKT_10);   // 3628800
    println!("fib(20) = {} (compile time)", FIB_20); // 6765
    println!("Primes: {:?}", PRIMES);
    // 10! = 3628800
    // fib(20) = 6765
    // Primes: [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]

    // Benchmark: const vs runtime
    let t1 = Instant::now();
    let mut yig1 = 0u64;
    for _ in 0..1_000_000 { yig1 += FAKT_10; } // konstanta — super tez
    let vaqt_const = t1.elapsed();

    let t2 = Instant::now();
    let mut yig2 = 0u64;
    for _ in 0..1_000_000 { yig2 += faktorial(10); } // hali ham tez (inline)
    let vaqt_fn = t2.elapsed();

    println!("\nconst:    {:?}", vaqt_const);
    println!("fn:       {:?}", vaqt_fn);
    // Ikkalasi ham juda tez!

    // Const generics — compile time o'lcham
    fn massiv_yig_indi<const N: usize>(arr: &[i32; N]) -> i32 {
        arr.iter().sum()
    }

    let a5: [i32; 5] = [1, 2, 3, 4, 5];
    let a3: [i32; 3] = [10, 20, 30];
    println!("\n[5]yig'indi: {}", massiv_yig_indi(&a5)); // 15
    println!("[3]yig'indi: {}", massiv_yig_indi(&a3));   // 60
    // [5]yig'indi: 15
    // [3]yig'indi: 60
}

// WordCount — iteratorlar bilan, zero-cost
fn wordcount_iterator(matn: &str) -> std::collections::HashMap<&str, usize> {
    matn.split_whitespace()
        .fold(std::collections::HashMap::new(), |mut m, s| {
            *m.entry(s).or_insert(0) += 1;
            m
        })
}

// WordCount — imperatif, qo'lda yozilgan
fn wordcount_imperatif(matn: &str) -> std::collections::HashMap<&str, usize> {
    let mut m = std::collections::HashMap::new();
    for s in matn.split_whitespace() {
        *m.entry(s).or_insert(0) += 1;
    }
    m
}

fn real_hayot_misoli() {

    println!("\n--- Real Hayot: WordCount ---");

    let katta_matn: String = "rust tili tez va xavfsiz ".repeat(100_000);

    let t1 = Instant::now();
    let hisob1 = wordcount_iterator(&katta_matn);
    let vaqt1 = t1.elapsed();

    let t2 = Instant::now();
    let hisob2 = wordcount_imperatif(&katta_matn);
    let vaqt2 = t2.elapsed();

    println!("Iterator: {:?} (so'z: {})", vaqt1, hisob1.len());
    println!("For loop: {:?} (so'z: {})", vaqt2, hisob2.len());
    println!("Bir xilmi: {}", hisob1 == hisob2);
    // Deyarli bir xil tezlik!

    // Sort karşılaştırma
    println!("\n--- Vec sort ---");
    let mut v1: Vec<i32> = (0..100_000).rev().collect();
    let mut v2 = v1.clone();

    let t1 = Instant::now();
    v1.sort();
    let vaqt_sort = t1.elapsed();

    let t2 = Instant::now();
    v2.sort_unstable();
    let vaqt_unstable = t2.elapsed();

    println!("sort:          {:?}", vaqt_sort);
    println!("sort_unstable: {:?}", vaqt_unstable);
    // sort_unstable odatda 20-30% tezroq
}

fn main() {

    println!("=== MONOMORPHIZATION ===");
    monomorphization_misoli();

    println!("\n=== ITERATOR FUSION ===");
    iterator_fusion_misoli();

    println!("\n=== INLINE & CLOSURE ===");
    inline_misoli();

    println!("\n=== NEWTYPE ZERO-COST ===");
    newtype_zero_cost_misoli();

    println!("\n=== ENUM ZERO-COST ===");
    enum_zero_cost_misoli();

    println!("\n=== COMPILE-TIME ===");
    const_misoli();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misoli();

    println!("\n=== XULOSA ===");
    println!("Rust Zero-Cost Abstractions:");
    println!("  Generics    → monomorphization (type-specific kod)");
    println!("  Iterators   → fusion (bitta loop)");
    println!("  Closures    → inline (chaqiruv yo'q)");
    println!("  Newtype     → zero overhead (bir xil o'lcham)");
    println!("  Enum        → tagged union (virtual table yo'q)");
    println!("  const fn    → compile-time evaluation");
    println!("  Traits      → static dispatch (impl Trait)");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        ZERO-COST MEXANIZMLARI                                                                                |
// #================================================================================================================================================#
// # |   1 | Generic<T>                      | Monomorphization — har tur uchun alohida   | Мономорфизация — отдельно для каждого типа              |
// # |   2 | Iterator zanjiri                | Fusion — bitta loop                        | Слияние — один цикл                                     |
// # |   3 | Closure                         | Inline — runtime chaqiruv yo'q             | Инлайнинг — нет вызова в runtime                        |
// # |   4 | Newtype                         | Zero size overhead                         | Нулевые накладные расходы по размеру                    |
// # |   5 | Enum                            | Tagged union — virtual table yo'q          | Помеченное объединение — нет virtual table              |
// # |   6 | const fn                        | Compile-time evaluation                    | Вычисление во время компиляции                          |
// # |   7 | impl Trait                      | Static dispatch                            | Статическая диспетчеризация                             |
// #================================================================================================================================================#
// # |                                        IMPL VS DYN                                                                                           |
// #================================================================================================================================================#
// # |   8 | impl Trait                      | Zero-cost, binary katta                    | Zero-cost, большой бинарник                             |
// # |   9 | dyn Trait                       | Runtime overhead (vtable), binary kichik   | Накладные расходы runtime, малый бинарник               |
// # |  10 | Qachon dyn                      | Ko'p tur, runtime tanlash zarur            | Много типов, нужен выбор в runtime                      |
// # |  11 | Qachon impl                     | Ma'lum tur, performance muhim              | Известный тип, производительность важна                 |
// #================================================================================================================================================#
// # |                                        BENCHMARKING                                                                                          |
// #================================================================================================================================================#
// # |  12 | Instant::now() + elapsed()      | Vaqt o'lchash                              | Измерение времени                                       |
// # |  13 | release mode                    | Optimizatsiya uchun: cargo run --release   | Для оптимизации: cargo run --release                    |
// # |  14 | criterion crate                 | Haqiqiy benchmark kutubxonasi              | Настоящая библиотека бенчмаркинга                       |
// #================================================================================================================================================#