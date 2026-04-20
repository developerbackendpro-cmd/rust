// #================================================================================================================================================#
// #                                                    ZERO-COST ABSTRACTIONS (CHUQUR)                                                             #
// #                        ZERO-COST — ABSTRAKTSIYA = XARAJATSIZ. ASSEMBLY TAHLIL. BENCHMARK. RUST KAFOLATLARI.                                    #
// #                        ZERO-COST — АБСТРАКЦИЯ = БЕСПЛАТНО. АНАЛИЗ ASSEMBLY. БЕНЧМАРК. ГАРАНТИИ RUST.                                           #
// #================================================================================================================================================#

// Bu kurs yakunlovchi mavzu — barcha Zero-Cost mexanizmlarini chuqur ko'rib chiqamiz
// Это завершающая тема курса — глубокое рассмотрение всех Zero-Cost механизмов

#![allow(dead_code, unused)]

use std::time::Instant;
use std::hint::black_box;
use std::marker::PhantomData;
use std::ops::{Add, Mul, Deref};
use std::fmt;

// Zero-Cost Abstraction ta'rifi:
// Определение Zero-Cost Abstraction:
//
//   "What you don't use, you don't pay for.
//    What you do use, you couldn't hand-code any better."
//    — Bjarne Stroustrup
//
//   Rust da Zero-Cost kafolatlari:
//   Гарантии Zero-Cost в Rust:
//   1. Generics → Monomorphization
//   2. Traits → Static dispatch (impl Trait)
//   3. Iterators → Fusion (bitta loop)
//   4. Closures → Inlining
//   5. Newtype → Zero overhead
//   6. Option<T> → NPO (null pointer optimization)
//   7. PhantomData → 0 bayt
//   8. const fn → Compile-time evaluation
//   9. Lifetimes → Runtime xarajat yo'q
//   10. Move semantics → Copy optimization

// Generic → har tur uchun alohida, optimallashtirilgan kod
// Generic → отдельный, оптимизированный код для каждого типа
#[inline(always)]
fn qoshish<T: Add<Output = T>>(a: T, b: T) -> T { a + b }

#[inline(always)]
fn maksimal<T: PartialOrd>(a: T, b: T) -> T { if a > b { a } else { b } }

fn monomorphization_chuqur() {

    println!("=== 1. MONOMORPHIZATION ===\n");

    // Har chaqiruv → alohida type-specific funksiya
    // Каждый вызов → отдельная type-specific функция
    let r1 = qoshish(10i32, 32i32);    // → qoshish_i32(10, 32)
    let r2 = qoshish(3.14f64, 2.71f64); // → qoshish_f64(...)
    let r3 = qoshish(100u64, 200u64);   // → qoshish_u64(...)
    println!("i32: {}, f64: {:.2}, u64: {}", r1, r2, r3);

    // Benchmark: generic vs konkret — bir xil
    let n = 10_000_000i64;
    let v: Vec<i64> = (0..n).collect();

    let t1 = Instant::now();
    let s1: i64 = v.iter().copied().fold(0, qoshish);
    let g1 = t1.elapsed();

    let t2 = Instant::now();
    let mut s2 = 0i64;
    for &x in &v { s2 += x; }
    let g2 = t2.elapsed();

    println!("Generic fold:   {:?} → {}", g1, s1);
    println!("Manual for:     {:?} → {}", g2, s2);
    println!("Bir xilmi: {} (monomorphization!)", s1 == s2);

    // O'lcham — zero overhead
    fn o_lcham_tekshir<T>() -> (usize, usize) {
        (std::mem::size_of::<T>(), std::mem::align_of::<T>())
    }

    println!("\nO'lcham tekshiruvi (generic == concrete):");
    let (s, a) = o_lcham_tekshir::<Vec<i32>>();
    println!("  Vec<i32>:   size={}, align={}", s, a);
    let (s, a) = o_lcham_tekshir::<Vec<f64>>();
    println!("  Vec<f64>:   size={}, align={}", s, a);
    // Bir xil! Monomorphization zero-cost.
}

fn iterator_fusion_chuqur() {

    println!("\n=== 2. ITERATOR FUSION ===\n");

    let n = 2_000_000usize;
    let v: Vec<i32> = (0..n as i32).collect();

    // 5 ta lazy operatsiya → BITTA loop
    // 5 lazy операций → ОДИН цикл
    let t1 = Instant::now();
    let r1: i64 = v.iter()
        .filter(|&&x| x % 3 == 0)
        .map(|&x| x as i64 * x as i64)
        .filter(|&x| x % 2 == 0)
        .take(10_000)
        .sum();
    let g1 = t1.elapsed();

    // Qo'lda yozilgan ekvivalent
    let t2 = Instant::now();
    let mut r2 = 0i64;
    let mut hisob = 0;
    for &x in &v {
        if x % 3 == 0 {
            let kv = x as i64 * x as i64;
            if kv % 2 == 0 {
                r2 += kv;
                hisob += 1;
                if hisob >= 10_000 { break; }
            }
        }
    }
    let g2 = t2.elapsed();

    println!("Iterator zanjiri:  {:?} → {}", g1, r1);
    println!("Qo'lda for loop:   {:?} → {}", g2, r2);
    println!("Bir xilmi: {}", r1 == r2);
    println!("Oraliq Vec: YO'Q (lazy evaluation)");

    // Zip fusion
    let a: Vec<f64> = (0..1_000_000).map(|i| i as f64).collect();
    let b: Vec<f64> = (0..1_000_000).map(|i| (i * 2) as f64).collect();

    let t3 = Instant::now();
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let g3 = t3.elapsed();
    println!("\nDot product (zip+map+sum): {:?} → {:.0}", g3, dot);
}

fn closure_inlining_chuqur() {

    println!("\n=== 3. CLOSURE INLINING ===\n");

    // Closure → inline qilinadi, funksiya chaqiruvi yo'q
    // Closure → inlined, нет вызова функции

    // Higher-order funksiya — zero-cost
    fn apply_twice<T, F: Fn(T) -> T>(f: F, x: T) -> T { f(f(x)) }
    fn apply_n<T: Copy, F: Fn(T) -> T>(f: F, mut x: T, n: u32) -> T {
        for _ in 0..n { x = f(x); }
        x
    }

    let ikki_baravar = |x: i64| x * 2;
    let uch_qoshish  = |x: i64| x + 3;

    let r1 = apply_twice(ikki_baravar, 5);
    let r2 = apply_n(uch_qoshish, 0, 10);
    println!("apply_twice(x*2, 5) = {} (5→10→20)", r1);
    println!("apply_n(x+3, 0, 10) = {} (0+3*10=30)", r2);

    let n = 10_000_000i64;

    // Closure bilan
    let f = |x: i64| x * x + x;
    let t1 = Instant::now();
    let s1: i64 = (0..n).map(f).sum();
    let g1 = t1.elapsed();

    // To'g'ridan
    let t2 = Instant::now();
    let s2: i64 = (0..n).map(|x| x * x + x).sum();
    let g2 = t2.elapsed();

    // Oddiy funksiya
    fn kvadrat_qosh(x: i64) -> i64 { x * x + x }
    let t3 = Instant::now();
    let s3: i64 = (0..n).map(kvadrat_qosh).sum();
    let g3 = t3.elapsed();

    println!("\nClosure vs to'g'ri vs fn ({} el):", n);
    println!("  Closure var: {:?}", g1);
    println!("  To'g'ridan:  {:?}", g2);
    println!("  fn pointer:  {:?}", g3);
    println!("  Hammasi bir xil assembly → Zero-cost!");
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Metr(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Kilogramm(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Soniya(f64);

impl Metr {
    fn new(v: f64) -> Self { Metr(v) }
    fn qiymat(self) -> f64 { self.0 }
}

impl Add for Metr {
    type Output = Self;
    fn add(self, b: Self) -> Self { Metr(self.0 + b.0) }
}

impl Mul<f64> for Metr {
    type Output = Self;
    fn mul(self, k: f64) -> Self { Metr(self.0 * k) }
}

impl fmt::Display for Metr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{:.3}m", self.0) }
}

fn newtype_zero_overhead_chuqur() {

    println!("\n=== 4. NEWTYPE ZERO-OVERHEAD ===\n");

    // O'lchamlar — bir xil!
    println!("Metr:     size={} align={}", std::mem::size_of::<Metr>(), std::mem::align_of::<Metr>());
    println!("f64:      size={} align={}", std::mem::size_of::<f64>(), std::mem::align_of::<f64>());
    println!("Kilogramm:size={} align={}", std::mem::size_of::<Kilogramm>(), std::mem::align_of::<Kilogramm>());
    // Barchasi 8 bayt — zero overhead!

    // Benchmark
    let n = 5_000_000usize;
    let v_metr: Vec<Metr> = (0..n).map(|i| Metr(i as f64 * 0.001)).collect();
    let v_f64:  Vec<f64>  = (0..n).map(|i| i as f64 * 0.001).collect();

    let t1 = Instant::now();
    let s1: f64 = v_metr.iter().map(|m| m.qiymat()).sum();
    let g1 = t1.elapsed();

    let t2 = Instant::now();
    let s2: f64 = v_f64.iter().sum();
    let g2 = t2.elapsed();

    println!("\nVec<Metr> sum: {:?} → {:.1}", g1, s1);
    println!("Vec<f64>  sum: {:?} → {:.1}", g2, s2);
    println!("Farq: ~0 ns (newtype zero-cost!)");

    // Tur xavfsizligi kafolati
    let masofa = Metr::new(100.0);
    let vaqt = Soniya(9.58);
    // let xato = masofa + vaqt; // ← KOMPILE XATO! Tur xavfsizligi!
    let tezlik = masofa.qiymat() / vaqt.0; // Qo'lda konversiya
    println!("\n100m / 9.58s = {:.3} m/s", tezlik);
    println!("Tur xavfsizligi + zero-cost = Rust afzalligi ✅");
}

// Lifetime — faqat kompilyator tekshiruvi, runtime yo'q
// Lifetime — только проверка компилятором, нет runtime затрат

fn eng_uzun<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

struct SatrParser<'a> {
    kirish: &'a str,
    pozitsiya: usize,
}

impl<'a> SatrParser<'a> {
    fn new(kirish: &'a str) -> Self { SatrParser { kirish, pozitsiya: 0 } }

    fn keyingi_token(&mut self) -> Option<&'a str> {
        // Bo'shliqlarni o'tkazib yuborish
        while self.pozitsiya < self.kirish.len()
            && self.kirish.as_bytes()[self.pozitsiya] == b' ' {
            self.pozitsiya += 1;
        }
        if self.pozitsiya >= self.kirish.len() { return None; }

        let boshi = self.pozitsiya;
        while self.pozitsiya < self.kirish.len()
            && self.kirish.as_bytes()[self.pozitsiya] != b' ' {
            self.pozitsiya += 1;
        }
        Some(&self.kirish[boshi..self.pozitsiya])
    }
}

fn lifetime_zero_cost_chuqur() {

    println!("\n=== 5. LIFETIME ZERO-COST ===\n");

    // Lifetime tekshiruvi faqat kompilyator da — runtime overhead 0
    let a = String::from("salom dunyo rust");
    let b = String::from("qisqa");
    let natija = eng_uzun(&a, &b);
    println!("Eng uzun: '{}'", natija);

    // Parser — zero-copy (matn nusxalanmaydi)
    let matn = "Rust    tili  juda    tez";
    let mut parser = SatrParser::new(matn);
    let mut tokenlar = Vec::new();
    while let Some(token) = parser.keyingi_token() {
        tokenlar.push(token); // &str — zero-copy!
    }
    println!("Tokenlar: {:?}", tokenlar);
    println!("Nusxa yo'q — &str borrow (lifetime kafolati)");

    // Benchmark: &str vs String
    let n = 1_000_000;
    let katta = "salom ".repeat(n);

    let t1 = Instant::now();
    let _: Vec<&str> = katta.split_whitespace().collect(); // zero-copy
    let g1 = t1.elapsed();

    let t2 = Instant::now();
    let _: Vec<String> = katta.split_whitespace().map(|s| s.to_string()).collect(); // copy
    let g2 = t2.elapsed();

    println!("\nsplit &str (zero-copy):  {:?}", g1);
    println!("split String (nusxa):    {:?}", g2);
    println!("Lifetime — nusxa olmay xavfsizlik!");
}

fn npo_chuqur() {

    println!("\n=== 6. NULL POINTER OPTIMIZATION ===\n");

    // Option<Box<T>> == Box<T> — bir xil o'lcham!
    println!("Box<i32>:           {} bayt", std::mem::size_of::<Box<i32>>());
    println!("Option<Box<i32>>:   {} bayt", std::mem::size_of::<Option<Box<i32>>>());
    println!("Box<i32> == Option<Box<i32>> ← NPO!");

    println!();
    println!("&T:                 {} bayt", std::mem::size_of::<&i32>());
    println!("Option<&i32>:       {} bayt", std::mem::size_of::<Option<&i32>>());
    println!("NonZeroU32:         {} bayt", std::mem::size_of::<std::num::NonZeroU32>());
    println!("Option<NonZeroU32>: {} bayt", std::mem::size_of::<Option<std::num::NonZeroU32>>());

    // Result ham NPO dan foydalanadi
    println!();
    println!("Result<(), ()>:     {} bayt", std::mem::size_of::<Result<(), ()>>());

    // Benchmark: Option overhead yo'q
    let n = 10_000_000u32;
    use std::num::NonZeroU32;

    let t1 = Instant::now();
    let s1: u64 = (1..=n)
        .map(|i| NonZeroU32::new(i))
        .filter_map(|x| x)
        .map(|x| x.get() as u64)
        .sum();
    let g1 = t1.elapsed();

    let t2 = Instant::now();
    let s2: u64 = (1u64..=n as u64).sum();
    let g2 = t2.elapsed();

    println!("\nNonZeroU32 filter_map: {:?} → {}", g1, s1);
    println!("To'g'ri sum:           {:?} → {}", g2, s2);
    println!("NPO orqali Option overhead yo'q!");
}

const fn faktorial(n: u64) -> u64 {
    match n { 0 | 1 => 1, _ => n * faktorial(n - 1) }
}

// Lookup table — barcha compile time
const FAKTORIALLAR: [u64; 21] = {
    let mut arr = [0u64; 21];
    let mut i = 0;
    while i <= 20 { arr[i] = faktorial(i as u64); i += 1; }
    arr
};

// CRC8 jadval — compile time
const fn crc8_tbl() -> [u8; 256] {
    let mut tbl = [0u8; 256];
    let mut i = 0;
    while i < 256 {
        let mut crc = i as u8;
        let mut j = 0;
        while j < 8 {
            crc = if crc & 0x80 != 0 { (crc << 1) ^ 0x07 } else { crc << 1 };
            j += 1;
        }
        tbl[i] = crc;
        i += 1;
    }
    tbl
}

const CRC8_TABLE: [u8; 256] = crc8_tbl();

// Const generics — compile time o'lcham
struct MatVec<const N: usize>(pub [f64; N]);

impl<const N: usize> MatVec<N> {
    fn new(arr: [f64; N]) -> Self { MatVec(arr) }
    fn dot(&self, b: &MatVec<N>) -> f64 {
        self.0.iter().zip(b.0.iter()).map(|(x, y)| x * y).sum()
    }
    fn norm(&self) -> f64 {
        self.0.iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}

fn const_compile_time_chuqur() {

    println!("\n=== 7. CONST FN + CONST GENERICS ===\n");

    // Lookup table — runtime hisoblash yo'q
    println!("Faktorial jadval (compile time):");
    for i in [0, 5, 10, 15, 20] {
        println!("  {}! = {}", i, FAKTORIALLAR[i]);
    }
    // 0! = 1, 5! = 120, 10! = 3628800

    // CRC8
    let ma_lumot = b"Salom Rust!";
    let crc: u8 = ma_lumot.iter().fold(0u8, |acc, &b| CRC8_TABLE[(acc ^ b) as usize]);
    println!("\nCRC8('Salom Rust!'): 0x{:02X}", crc);

    // Const generics — runtime xatosi imkonsiz
    let v3 = MatVec::new([1.0, 2.0, 3.0]);
    let u3 = MatVec::new([4.0, 5.0, 6.0]);
    println!("\nMatVec<3> dot: {}", v3.dot(&u3)); // 32.0
    println!("MatVec<3> norm: {:.4}", v3.norm()); // 3.7417

    // Bu KOMPILE BO'LMAYDI — o'lcham mos emas:
    // let v4 = MatVec::new([1.0, 2.0, 3.0, 4.0]);
    // v3.dot(&v4); // ← MatVec<3> ≠ MatVec<4>!

    // Benchmark: compile-time vs runtime
    let t1 = Instant::now();
    let mut s = 0u64;
    for _ in 0..1_000_000 { s += FAKTORIALLAR[10]; } // jadval lookup
    let g1 = t1.elapsed();

    let t2 = Instant::now();
    let mut s2 = 0u64;
    for _ in 0..1_000_000 { s2 += faktorial(10); } // runtime (lekin inline)
    let g2 = t2.elapsed();

    println!("\nLookup jadval:  {:?}", g1);
    println!("const fn:       {:?}", g2);
    println!("Ikkisi ham binary da konstanta!");
}

fn move_semantics_chuqur() {

    println!("\n=== 8. MOVE SEMANTICS ===\n");

    // Move — nusxa olmaydi, ownership o'tadi
    // Move — нет копии, передаётся владение

    fn qayta_ishlash(v: Vec<i32>) -> Vec<i32> {
        v.into_iter().map(|x| x * 2).collect()
    }

    let v = vec![1, 2, 3, 4, 5];
    let v2 = qayta_ishlash(v); // Move — heap nusxalanmaydi!
    println!("Moved Vec: {:?}", v2);

    // String move — zero-copy
    fn satr_uzunlik(s: String) -> (String, usize) {
        let n = s.len();
        (s, n) // Move back — nusxa olmaydi
    }

    let s = String::from("Salom Rust!");
    let (s, n) = satr_uzunlik(s);
    println!("Move back: '{}' ({})", s, n);

    // RVO (Return Value Optimization) — kompilyator
    fn katta_vektor() -> Vec<i64> {
        (0..1_000_000i64).collect() // RVO — to'g'ridan joy ajratiladi
    }

    let t = Instant::now();
    let v3 = katta_vektor(); // Stack → Heap to'g'ri, nusxa yo'q
    println!("\nRVO (1M element): {:?}", t.elapsed());
    println!("Yig'indi: {}", v3.iter().sum::<i64>());

    // Benchmark: move vs clone
    let katta: Vec<i32> = (0..100_000).collect();

    let t1 = Instant::now();
    for _ in 0..100 {
        let v = katta.clone(); // Nusxa olish
        black_box(v.iter().sum::<i32>());
    }
    let g1 = t1.elapsed();

    let t2 = Instant::now();
    let sum: i32 = katta.iter().sum(); // Reference — nusxa yo'q
    let g2 = t2.elapsed();

    println!("\nclone 100x:   {:?}", g1);
    println!("iter (borrow):{:?}", g2);
    println!("Move/borrow = zero-copy!");
}

trait Shakl: fmt::Debug {
    fn yuzi(&self) -> f64;
    fn perimetri(&self) -> f64;
}

#[derive(Debug)] struct Aylana(f64);
#[derive(Debug)] struct Turtburchak(f64, f64);

impl Shakl for Aylana {
    fn yuzi(&self) -> f64 { std::f64::consts::PI * self.0 * self.0 }
    fn perimetri(&self) -> f64 { 2.0 * std::f64::consts::PI * self.0 }
}

impl Shakl for Turtburchak {
    fn yuzi(&self) -> f64 { self.0 * self.1 }
    fn perimetri(&self) -> f64 { 2.0 * (self.0 + self.1) }
}

// Static dispatch — zero-cost
fn static_yuzi<T: Shakl>(shakl: &T) -> f64 { shakl.yuzi() }

// Dynamic dispatch — overhead bor
fn dynamic_yuzi(shakl: &dyn Shakl) -> f64 { shakl.yuzi() }

fn static_vs_dynamic_chuqur() {

    println!("\n=== 9. STATIC vs DYNAMIC DISPATCH ===\n");

    let n = 5_000_000usize;
    let aylanalar: Vec<Aylana> = (0..n).map(|i| Aylana(i as f64 * 0.001)).collect();
    let turtburchaklar: Vec<Turtburchak> = (0..n).map(|i| Turtburchak(i as f64, 2.0)).collect();

    // Static dispatch
    let t1 = Instant::now();
    let s1: f64 = aylanalar.iter().map(|a| static_yuzi(a)).sum();
    let g1 = t1.elapsed();

    // Dynamic dispatch (vtable)
    let shakllar: Vec<&dyn Shakl> = aylanalar.iter().map(|a| a as &dyn Shakl).collect();
    let t2 = Instant::now();
    let s2: f64 = shakllar.iter().map(|s| dynamic_yuzi(*s)).sum();
    let g2 = t2.elapsed();

    println!("Static dispatch:  {:?} → {:.1}", g1, s1);
    println!("Dynamic dispatch: {:?} → {:.1}", g2, s2);

    // O'lchamlar
    println!("\n&Aylana:       {} bayt", std::mem::size_of::<&Aylana>());
    println!("&dyn Shakl:    {} bayt (fat pointer!)", std::mem::size_of::<&dyn Shakl>());
    println!("Box<dyn Shakl>:{} bayt", std::mem::size_of::<Box<dyn Shakl>>());
    // static: 8 bayt, dynamic: 16 bayt (data + vtable)
}

fn toliq_benchmark() {

    println!("\n=== 10. TO'LIQ BENCHMARK ===\n");

    let n = 1_000_000usize;

    // 1. Abstraktsiyasiz — eng tez bo'lishi mumkin
    let v: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let t = Instant::now();
    let mut s = 0.0f64;
    for &x in &v { s += x * x; }
    println!("Raw loop:          {:?} → {:.0}", t.elapsed(), s);

    // 2. Iterator abstraktsiyasi — bir xil!
    let t = Instant::now();
    let s2: f64 = v.iter().map(|&x| x * x).sum();
    println!("Iterator chain:    {:?} → {:.0}", t.elapsed(), s2);

    // 3. Generic funksiya + closure
    fn apply_sum<T: Copy, F: Fn(T) -> f64>(v: &[T], f: F) -> f64 {
        v.iter().copied().map(f).sum()
    }
    let t = Instant::now();
    let s3 = apply_sum(&v, |x| x * x);
    println!("Generic + closure: {:?} → {:.0}", t.elapsed(), s3);

    // 4. Newtype wrapper
    struct F64Vec(Vec<f64>);
    impl F64Vec {
        fn kvadrat_yig(&self) -> f64 { self.0.iter().map(|&x| x * x).sum() }
    }
    let fv = F64Vec(v.clone());
    let t = Instant::now();
    let s4 = fv.kvadrat_yig();
    println!("Newtype method:    {:?} → {:.0}", t.elapsed(), s4);

    println!("\nHammasi bir xil natija: {}", s == s2 && s2 == s3 && s3 == s4);
    println!("Abstraktsiya = xarajatsiz! ✅");
}

fn assembly_tahlil() {

    println!("\n=== ASSEMBLY TAHLIL ===\n");

    println!("cargo asm orqali assembly ko'rish:");
    println!("  cargo install cargo-asm");
    println!("  cargo asm --release 'myapp::my_fn'");
    println!();
    println!("LLVM IR ko'rish:");
    println!("  cargo rustc --release -- --emit=llvm-ir");
    println!("  # target/release/deps/*.ll faylida");
    println!();
    println!("MIR ko'rish:");
    println!("  cargo rustc -- -Zunpretty=mir");
    println!();

    println!(r#"// Misol: qoshish<i32>(1, 2) assembly (release):
// add_i32:
//   lea eax, [rdi + rsi]    ; bitta ko'rsatma!
//   ret
//
// Generic bo'lsa ham — bir xil!
// Abstraktsiya = ZERO-COST!

// Misol: Iterator chain assembly (release):
// .L loop:
//   movsd xmm0, [rdi]       ; yuklash
//   mulsd xmm0, xmm0        ; kvadrat
//   addsd xmm1, xmm0        ; yig'ish
//   add rdi, 8
//   cmp rdi, rsi
//   jne .L                  ; qaytish
// Bitta loop — 5 ta Iterator operatsiya!"#);
}

fn main() {

    println!("╔══════════════════════════════════════════════╗");
    println!("║   ZERO-COST ABSTRACTIONS — YAKUNIY MAVZU     ║");
    println!("║   148/148 — RUST KURS TUGADI!  🦀            ║");
    println!("╚══════════════════════════════════════════════╝\n");

    monomorphization_chuqur();
    iterator_fusion_chuqur();
    closure_inlining_chuqur();
    newtype_zero_overhead_chuqur();
    lifetime_zero_cost_chuqur();
    npo_chuqur();
    const_compile_time_chuqur();
    move_semantics_chuqur();
    static_vs_dynamic_chuqur();
    toliq_benchmark();
    assembly_tahlil();

    println!("╔════════════════════════════════════════════════╗");
    println!("║          ZERO-COST XULOSA                      ║");
    println!("╠════════════════════════════════════════════════╣");
    println!("║  Mexanizm         Natija                       ║");
    println!("║  Generics     →   Monomorphization             ║");
    println!("║  Iterators    →   Bitta loop (fusion)          ║");
    println!("║  Closures     →   Inline (chaqiruv yo'q)       ║");
    println!("║  Newtype      →   0 overhead (size = same)     ║");
    println!("║  Lifetimes    →   0 runtime xarajat            ║");
    println!("║  Option<&T>   →   NPO (size = &T)              ║");
    println!("║  const fn     →   Compile-time bajarish        ║");
    println!("║  Move         →   RVO, zero-copy               ║");
    println!("║  impl Trait   →   Static dispatch              ║");
    println!("╠════════════════════════════════════════════════╣");
    println!("║  140/140 mavzu — KURS MUVAFFAQIYATLI TUGADI    ║");
    println!("║                          🦀🎉🏆                ║");
    println!("╚════════════════════════════════════════════════╝");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |   1 | Monomorphization                | Generic → tur-specific kod                 | Generic → тип-специфичный код                           |
// # |   2 | Iterator fusion                 | Zanjir → bitta loop                        | Цепочка → один цикл                                     |
// # |   3 | Closure inlining                | Closure → funksiya chaqiruvi yo'q          | Closure → нет вызова функции                            |
// # |   4 | Newtype                         | size_of<Newtype> == size_of<Inner>         | size_of<Newtype> == size_of<Inner>                      |
// # |   5 | Lifetimes                       | Compile-time only, runtime = 0             | Только compile-time, runtime = 0                        |
// # |   6 | NPO                             | Option<Box<T>> == Box<T> size              | Option<Box<T>> == Box<T> размер                         |
// # |   7 | const fn                        | Compile-time bajarish                      | Выполнение во время компиляции                          |
// # |   8 | Move semantics                  | RVO, zero-copy ownership                   | RVO, zero-copy владение                                 |
// # |   9 | impl Trait (static)             | vtable yo'q, inlined                       | Нет vtable, инлайнинг                                   |
// # |  10 | dyn Trait (dynamic)             | vtable, fat pointer (16 bayt)              | vtable, fat pointer (16 байт)                           |
// #================================================================================================================================================#