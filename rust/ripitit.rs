// #================================================================================================================================================#
// #                                                                RPITIT                                                                          #
// #                    RPITIT — RETURN POSITION IMPL TRAIT IN TRAIT. RUST 1.75 STABLE. ASYNC TRAIT. ITERATOR IN TRAIT.                             #
// #                    RPITIT — IMPL TRAIT В ПОЗИЦИИ ВОЗВРАТА В ТРЕЙТЕ. RUST 1.75 STABLE. ASYNC TRAIT. ITERATOR В ТРЕЙТЕ.                          #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// RPITIT nima:
// Что такое RPITIT:
//
//   Return Position Impl Trait In Trait
//   Rust 1.75 (2023) dan STABLE!
//
//   Avval (Rust 1.74-):
//   Раньше (до Rust 1.74):
//   trait Iter {
//       fn barcha(&self) -> Box<dyn Iterator<Item=i32>>; // heap!
//   }
//
//   Hozir (Rust 1.75+) RPITIT:
//   Сейчас (Rust 1.75+) RPITIT:
//   trait Iter {
//       fn barcha(&self) -> impl Iterator<Item=i32>; // zero-cost!
//   }
//
//   Cheklov:
//   Ограничение:
//   - dyn Dispatch bilan ishlamaydi (object-safe emas)
//   - Не работает с dyn Dispatch (не object-safe)
//   - Har implementatsiya o'z tur qaytaradi
//   - Каждая реализация возвращает свой тип

fn rpitit_tarixiy_muammo() {

    println!("=== TARIXIY MUAMMO ===\n");

    // Avval: Box<dyn Iterator> kerak edi
    trait OldIter {
        fn barcha(&self) -> Box<dyn Iterator<Item = i32>>;
        fn filter_iter(&self, f: Box<dyn Fn(&i32) -> bool>) -> Box<dyn Iterator<Item = i32>>;
    }

    struct OldVec { ma_lumot: Vec<i32> }

    impl OldIter for OldVec {
        fn barcha(&self) -> Box<dyn Iterator<Item = i32>> {
            // Heap alloc majburiy! Clone kerak
            Box::new(self.ma_lumot.clone().into_iter())
        }

        fn filter_iter(&self, f: Box<dyn Fn(&i32) -> bool>) -> Box<dyn Iterator<Item = i32>> {
            Box::new(self.ma_lumot.clone().into_iter().filter(move |x| f(x)))
        }
    }

    let old = OldVec { ma_lumot: vec![1, 2, 3, 4, 5, 6] };
    let v: Vec<i32> = old.barcha().collect();
    println!("Old (Box<dyn>): {:?}", v);
    let v2: Vec<i32> = old.filter_iter(Box::new(|x| x % 2 == 0)).collect();
    println!("Old filter:     {:?}", v2);
    // Old (Box<dyn>): [1, 2, 3, 4, 5, 6]
    // Old filter:     [2, 4, 6]

    println!("\nMuammo:");
    println!("  ❌ Heap alloc har chaqiruvda");
    println!("  ❌ Dynamic dispatch (sekin)");
    println!("  ❌ Clone majburiy");
    println!("  ❌ Lifetime murakkab");
}

// RPITIT: trait ichida -> impl Trait
trait YangiIter {
    // Rust 1.75+ — RPITIT!
    fn barcha(&self) -> impl Iterator<Item = i32>;
    fn juft_barcha(&self) -> impl Iterator<Item = i32>;
    fn mapped(&self, offset: i32) -> impl Iterator<Item = i32>;

    // Default implementatsiya ham mumkin
    fn yig_indi(&self) -> i32 {
        self.barcha().sum()
    }

    fn maksimal(&self) -> Option<i32> {
        self.barcha().max()
    }

    fn minimal(&self) -> Option<i32> {
        self.barcha().min()
    }
}

struct IntVec { ma_lumot: Vec<i32> }
struct RangeIter { boshlanish: i32, oxir: i32 }
struct MappedIter { asosiy: Vec<i32>, koeffitsient: i32 }

impl YangiIter for IntVec {
    fn barcha(&self) -> impl Iterator<Item = i32> {
        self.ma_lumot.iter().copied()
    }

    fn juft_barcha(&self) -> impl Iterator<Item = i32> {
        self.ma_lumot.iter().copied().filter(|x| x % 2 == 0)
    }

    fn mapped(&self, offset: i32) -> impl Iterator<Item = i32> {
        self.ma_lumot.iter().map(move |&x| x + offset)
    }
}

impl YangiIter for RangeIter {
    fn barcha(&self) -> impl Iterator<Item = i32> {
        self.boshlanish..self.oxir
    }

    fn juft_barcha(&self) -> impl Iterator<Item = i32> {
        (self.boshlanish..self.oxir).filter(|x| x % 2 == 0)
    }

    fn mapped(&self, offset: i32) -> impl Iterator<Item = i32> {
        (self.boshlanish..self.oxir).map(move |x| x + offset)
    }
}

impl YangiIter for MappedIter {
    fn barcha(&self) -> impl Iterator<Item = i32> {
        let k = self.koeffitsient;
        self.asosiy.iter().map(move |&x| x * k)
    }

    fn juft_barcha(&self) -> impl Iterator<Item = i32> {
        let k = self.koeffitsient;
        self.asosiy.iter().map(move |&x| x * k).filter(|x| x % 2 == 0)
    }

    fn mapped(&self, offset: i32) -> impl Iterator<Item = i32> {
        let k = self.koeffitsient;
        self.asosiy.iter().map(move |&x| x * k + offset)
    }
}

fn rpitit_yangi_yechim() {

    println!("\n=== RPITIT YANGI YECHIM (Rust 1.75+) ===");

    let iv = IntVec { ma_lumot: vec![1, 2, 3, 4, 5, 6, 7, 8] };
    let rv = RangeIter { boshlanish: 1, oxir: 10 };
    let mv = MappedIter { asosiy: vec![1, 2, 3, 4, 5], koeffitsient: 3 };

    // IntVec
    println!("\nIntVec:");
    println!("  barcha:  {:?}", iv.barcha().collect::<Vec<_>>());
    println!("  juft:    {:?}", iv.juft_barcha().collect::<Vec<_>>());
    println!("  +10:     {:?}", iv.mapped(10).collect::<Vec<_>>());
    println!("  yig:     {}", iv.yig_indi());
    println!("  maks:    {:?}", iv.maksimal());
    // barcha: [1, 2, 3, 4, 5, 6, 7, 8]
    // juft:   [2, 4, 6, 8]
    // +10:    [11, 12, 13, 14, 15, 16, 17, 18]
    // yig:    36
    // maks:   Some(8)

    // RangeIter
    println!("\nRangeIter(1..10):");
    println!("  barcha:  {:?}", rv.barcha().collect::<Vec<_>>());
    println!("  juft:    {:?}", rv.juft_barcha().collect::<Vec<_>>());
    println!("  yig:     {}", rv.yig_indi());
    // barcha: [1, 2, 3, 4, 5, 6, 7, 8, 9]
    // juft:   [2, 4, 6, 8]
    // yig:    45

    // MappedIter
    println!("\nMappedIter(x3):");
    println!("  barcha:  {:?}", mv.barcha().collect::<Vec<_>>());
    println!("  juft:    {:?}", mv.juft_barcha().collect::<Vec<_>>());
    println!("  +1:      {:?}", mv.mapped(1).collect::<Vec<_>>());
    // barcha: [3, 6, 9, 12, 15]
    // juft:   [6, 12]
    // +1:     [4, 7, 10, 13, 16]

    println!("\nAfzalliklar:");
    println!("  ✅ Heap alloc yo'q (zero-cost)");
    println!("  ✅ Static dispatch (tez)");
    println!("  ✅ Clone shart emas");
    println!("  ✅ Default impl mumkin");
}

// async fn in trait — RPITIT orqali (Rust 1.75+)
// async fn в трейте — через RPITIT (Rust 1.75+)
trait AsyncHisoblash {
    // Rust 1.75+ — async fn in trait STABLE!
    async fn hisoblash(&self, n: u32) -> u64;
    async fn bir_nechta(&self, ns: &[u32]) -> Vec<u64>;

    // Default async
    async fn yig_indi(&self, ns: &[u32]) -> u64 {
        let mut yig = 0u64;
        for &n in ns {
            yig += self.hisoblash(n).await;
        }
        yig
    }
}

struct FaktorialHisob;
struct FibHisob;

impl AsyncHisoblash for FaktorialHisob {
    async fn hisoblash(&self, n: u32) -> u64 {
        (1..=n as u64).product()
    }

    async fn bir_nechta(&self, ns: &[u32]) -> Vec<u64> {
        let mut v = Vec::new();
        for &n in ns {
            v.push(self.hisoblash(n).await);
        }
        v
    }
}

impl AsyncHisoblash for FibHisob {
    async fn hisoblash(&self, n: u32) -> u64 {
        let (mut a, mut b) = (0u64, 1u64);
        for _ in 0..n { (a, b) = (b, a + b); }
        a
    }

    async fn bir_nechta(&self, ns: &[u32]) -> Vec<u64> {
        ns.iter().map(|&n| {
            let (mut a, mut b) = (0u64, 1u64);
            for _ in 0..n { (a, b) = (b, a + b); }
            a
        }).collect()
    }
}

fn noop_waker() -> std::task::Waker {
    use std::task::{RawWaker, RawWakerVTable, Waker};
    unsafe fn clone(_: *const ()) -> RawWaker { RawWaker::new(std::ptr::null(), &VT) }
    unsafe fn noop(_: *const ()) {}
    static VT: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VT)) }
}

fn poll_fut<F: Future>(mut f: Pin<Box<F>>) -> F::Output {
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    loop {
        match f.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending  => std::hint::spin_loop(),
        }
    }
}

fn async_trait_rpitit_misoli() {

    println!("\n=== ASYNC FN IN TRAIT (Rust 1.75+) ===");

    let fakt = FaktorialHisob;
    let fib = FibHisob;

    // Faktorial
    let r1 = poll_fut(Box::pin(fakt.hisoblash(5)));
    let r2 = poll_fut(Box::pin(fakt.hisoblash(10)));
    println!("\nFaktorial:");
    println!("  5! = {}", r1);   // 120
    println!("  10! = {}", r2);  // 3628800

    let ns = [0u32, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let fakt_v = poll_fut(Box::pin(fakt.bir_nechta(&ns)));
    println!("  Bir nechta: {:?}", fakt_v);

    // Fibonacci
    let fib_v = poll_fut(Box::pin(fib.bir_nechta(&ns)));
    println!("\nFibonacci:");
    println!("  Bir nechta: {:?}", fib_v);
    // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55]

    // Default async
    let yig = poll_fut(Box::pin(fib.yig_indi(&[5, 10, 15])));
    println!("  fib(5)+fib(10)+fib(15) = {}", yig);
    // 5 + 55 + 610 = 670
}

// RPITIT cheklov: dyn Trait bilan ishlamaydi
// RPITIT ограничение: не работает с dyn Trait

// Bu KOMPILE BO'LMAYDI:
// Это НЕ СКОМПИЛИРУЕТСЯ:
// let iter: Box<dyn YangiIter> = Box::new(iv); // object-safe emas!

// Yechim 1: Enum bilan dispatch
#[derive(Debug)]
enum IterEnum {
    Vec(Vec<i32>),
    Range(std::ops::Range<i32>),
}

impl IterEnum {
    fn barcha(&self) -> Box<dyn Iterator<Item = i32> + '_> {
        match self {
            IterEnum::Vec(v)   => Box::new(v.iter().copied()),
            IterEnum::Range(r) => Box::new(r.clone()),
        }
    }
}

// Yechim 2: wrapper struct bilan
struct ErasedIter {
    ichki: Box<dyn Fn() -> Box<dyn Iterator<Item = i32>>>,
}

impl ErasedIter {
    fn from_vec(v: Vec<i32>) -> Self {
        ErasedIter {
            ichki: Box::new(move || Box::new(v.clone().into_iter()))
        }
    }

    fn from_range(r: std::ops::Range<i32>) -> Self {
        ErasedIter {
            ichki: Box::new(move || Box::new(r.clone()))
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = i32>> {
        (self.ichki)()
    }
}

fn cheklov_va_yechim_misoli() {

    println!("\n=== RPITIT CHEKLOV VA YECHIM ===");

    // Enum dispatch
    let qatorlar: Vec<IterEnum> = vec![
        IterEnum::Vec(vec![1, 2, 3]),
        IterEnum::Range(10..15),
        IterEnum::Vec(vec![100, 200]),
    ];

    println!("\nEnum dispatch:");
    for q in &qatorlar {
        let v: Vec<i32> = q.barcha().collect();
        println!("  {:?} → {:?}", q, v);
    }
    // Vec([1, 2, 3]) → [1, 2, 3]
    // Range(10..15) → [10, 11, 12, 13, 14]
    // Vec([100, 200]) → [100, 200]

    // Erased iter
    println!("\nErasedIter:");
    let iterlar = vec![
        ErasedIter::from_vec(vec![10, 20, 30]),
        ErasedIter::from_range(1..6),
    ];

    for iter in &iterlar {
        let v: Vec<i32> = iter.iter().collect();
        println!("  {:?}", v);
    }
    // [10, 20, 30]
    // [1, 2, 3, 4, 5]
}

// Data source trait — RPITIT bilan
trait DataSource {
    fn o_qi(&self) -> impl Iterator<Item = String>;
    fn filtrlash(&self, kalit: &str) -> impl Iterator<Item = String> + '_;
    fn o_lcham(&self) -> usize;

    fn barcha_vec(&self) -> Vec<String> {
        self.o_qi().collect()
    }

    fn mavjudmi(&self, kalit: &str) -> bool {
        self.filtrlash(kalit).next().is_some()
    }
}

struct MemorySource { ma_lumot: Vec<String> }
struct FilteredSource { ma_lumot: Vec<String>, prefiks: String }

impl DataSource for MemorySource {
    fn o_qi(&self) -> impl Iterator<Item = String> {
        self.ma_lumot.iter().cloned()
    }

    fn filtrlash(&self, kalit: &str) -> impl Iterator<Item = String> + '_ {
        let k = kalit.to_string();
        self.ma_lumot.iter()
            .filter(move |s| s.contains(&k))
            .cloned()
    }

    fn o_lcham(&self) -> usize { self.ma_lumot.len() }
}

impl DataSource for FilteredSource {
    fn o_qi(&self) -> impl Iterator<Item = String> {
        let p = self.prefiks.clone();
        self.ma_lumot.iter()
            .filter(move |s| s.starts_with(&p))
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn filtrlash(&self, kalit: &str) -> impl Iterator<Item = String> + '_ {
        let p = self.prefiks.clone();
        let k = kalit.to_string();
        self.ma_lumot.iter()
            .filter(move |s| s.starts_with(&p) && s.contains(&k))
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn o_lcham(&self) -> usize {
        self.ma_lumot.iter()
            .filter(|s| s.starts_with(&self.prefiks))
            .count()
    }
}

// Umumiy funksiya — har qanday DataSource bilan
fn source_tahlil<S: DataSource>(source: &S, qidiruv: &str) {
    println!("  Jami: {}", source.o_lcham());
    println!("  '{}' qidiruv: {} ta", qidiruv,
             source.filtrlash(qidiruv).count());
    println!("  Mavjud: {}", source.mavjudmi(qidiruv));
    let barcha: Vec<String> = source.barcha_vec();
    println!("  Birinchi 3: {:?}", &barcha[..3.min(barcha.len())]);
}

fn real_hayot_misollari() {

    println!("\n=== REAL HAYOT: DataSource ===");

    let mem = MemorySource {
        ma_lumot: vec![
            "rust_iter".into(), "python_list".into(), "rust_trait".into(),
            "go_interface".into(), "rust_async".into(), "java_stream".into(),
        ]
    };

    let filt = FilteredSource {
        ma_lumot: mem.ma_lumot.clone(),
        prefiks: "rust".to_string(),
    };

    println!("\nMemorySource:");
    source_tahlil(&mem, "rust");
    // Jami: 6
    // 'rust' qidiruv: 3 ta
    // Mavjud: true
    // Birinchi 3: ["rust_iter", "python_list", "rust_trait"]

    println!("\nFilteredSource (prefiks='rust'):");
    source_tahlil(&filt, "trait");
    // Jami: 3
    // 'trait' qidiruv: 1 ta
    // Mavjud: true
    // Birinchi 3: ["rust_iter", "rust_trait", "rust_async"]

    // Iterator pipeline bilan ishlatish
    println!("\nPipeline:");
    let uzun_satrlar: Vec<String> = mem.o_qi()
        .filter(|s| s.len() > 10)
        .map(|s| s.to_uppercase())
        .collect();
    println!("  10+ harf: {:?}", uzun_satrlar);
    // ["PYTHON_LIST", "RUST_TRAIT", "GO_INTERFACE", "RUST_ASYNC", "JAVA_STREAM"]
}

fn main() {

    rpitit_tarixiy_muammo();
    rpitit_yangi_yechim();
    async_trait_rpitit_misoli();
    cheklov_va_yechim_misoli();
    real_hayot_misollari();

    println!("\n=== XULOSA ===");
    println!("RPITIT (Rust 1.75+ STABLE):");
    println!("  trait T {{ fn f(&self) -> impl Iterator; }}");
    println!("  async fn in trait → impl Future qaytaradi");
    println!();
    println!("Afzalliklari:");
    println!("  ✅ Zero-cost (static dispatch)");
    println!("  ✅ Heap alloc yo'q");
    println!("  ✅ Default implementatsiya");
    println!("  ✅ async fn in trait stable");
    println!();
    println!("Cheklov:");
    println!("  ❌ dyn Trait bilan ishlamaydi");
    println!("  → Yechim: Enum dispatch yoki Box<dyn Fn>");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                               | Описание (RU)                                            |
// #================================================================================================================================================#
// # |                                        RPITIT                                                                                                |
// #================================================================================================================================================#
// # |   1 | fn f(&self) -> impl Trait       | Trait da impl Trait qaytarish (1.75+)     | Возврат impl Trait в трейте (1.75+)                      |
// # |   2 | async fn f(&self) -> T in trait | Async fn in trait (1.75+)                 | Async fn в трейте (1.75+)                                |
// # |   3 | Default fn bilan impl Trait     | Default implementatsiya RPITIT bilan      | Default реализация с RPITIT                              |
// # |   4 | -> impl Iterator + '_           | Lifetime bilan RPITIT                     | RPITIT с lifetime                                        |
// #================================================================================================================================================#
// # |                                        AVVAL VS HOZIR                                                                                        |
// #================================================================================================================================================#
// # |   5 | Box<dyn Iterator> (eski)        | Heap + dynamic dispatch                    | Куча + динамическая диспетчеризация                     |
// # |   6 | impl Iterator (yangi, RPITIT)   | Stack + static dispatch                    | Стек + статическая диспетчеризация                      |
// # |   7 | async-trait crate (eski)        | Makro yordamchi, heap future               | Макрос помощник, future в куче                          |
// # |   8 | async fn in trait (yangi)       | Native, zero-cost                          | Нативный, нулевые затраты                               |
// #================================================================================================================================================#
// # |                                        CHEKLOV VA YECHIM                                                                                     |
// #================================================================================================================================================#
// # |   9 | object-safe emas                | dyn Trait bilan ishlamaydi                 | Не работает с dyn Trait                                 |
// # |  10 | Enum dispatch                   | Enum { A(TypeA), B(TypeB) }                | Enum { A(TypeA), B(TypeB) }                             |
// # |  11 | Box<dyn Fn> wrapper             | Type erasure yechim                        | Решение через type erasure                              |
// # |  12 | Generic fn<S: DataSource>       | Static dispatch bilan polimorfizm          | Полиморфизм со статической диспетчеризацией             |
// #================================================================================================================================================#