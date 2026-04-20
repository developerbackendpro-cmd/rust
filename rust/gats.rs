// #================================================================================================================================================#
// #                                                                GATs                                                                            #
// #                        GATs — GENERIC ASSOCIATED TYPES. TRAIT DA GENERIC TURLAR. LIFETIME BILAN. HRP VA POLONIUS.                              #
// #                        GATs — ОБОБЩЁННЫЕ АССОЦИИРОВАННЫЕ ТИПЫ. GENERIC ТИПЫ В ТРЕЙТАХ. С LIFETIME. HRP И POLONIUS.                             #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::marker::PhantomData;
use std::collections::HashMap;

// GATs nima:
// Что такое GATs:
//
//   Generic Associated Types — Rust 1.65 (stable)
//   Generic Associated Types — стабильно с Rust 1.65
//
//   Oddiy Associated Type:
//   Обычный Associated Type:
//   trait Iter { type Item; }
//
//   GAT — Associated type o'zi generic:
//   GAT — ассоциированный тип сам по себе generic:
//   trait LendingIter { type Item<'a> where Self: 'a; }
//   trait Container { type Ref<'a, T> where Self: 'a; }
//
//   Nima uchun kerak:
//   Зачем нужны:
//   - Streaming iterator (element hayoti cheklangan)
//   - Streaming итератор (ограниченный lifetime элемента)
//   - Self-referential iterator (o'ziga reference qaytaradi)
//   - Self-referential итератор (возвращает ссылку на себя)
//   - Higher-kinded types simulyatsiya
//   - Симуляция типов высшего порядка

fn gat_asosiy_misoli() {

    println!("=== GAT ASOSIY ===\n");

    // Oddiy AssocType — lifetime bog'liq emas
    trait OddiyIter {
        type Item; // Lifetime yo'q
        fn keyingi(&mut self) -> Option<Self::Item>;
    }

    // GAT — Item lifetime bor
    trait LendingIter {
        type Item<'this> where Self: 'this; // 'this lifetimiga bog'liq
        fn keyingi<'this>(&'this mut self) -> Option<Self::Item<'this>>;
    }

    // Farq:
    // OddiyIter: element owned yoki 'static bo'lishi kerak
    // LendingIter: element iterator dan qarz olinishi mumkin

    // ────────── Oddiy Iterator ──────────
    struct SonlarIter { joriy: i32, max: i32 }

    impl OddiyIter for SonlarIter {
        type Item = i32; // owned — OK
        fn keyingi(&mut self) -> Option<i32> {
            if self.joriy >= self.max { return None; }
            let v = self.joriy;
            self.joriy += 1;
            Some(v)
        }
    }

    let mut iter = SonlarIter { joriy: 0, max: 5 };
    let mut natija = vec![];
    while let Some(v) = iter.keyingi() { natija.push(v); }
    println!("OddiyIter: {:?}", natija);
    // OddiyIter: [0, 1, 2, 3, 4]

    // ────────── GAT: Lending Iterator ──────────
    struct StringLenIter { satrlar: Vec<String>, indeks: usize }

    impl LendingIter for StringLenIter {
        type Item<'this> = &'this str where Self: 'this;

        fn keyingi<'this>(&'this mut self) -> Option<&'this str> {
            if self.indeks >= self.satrlar.len() { return None; }
            let s = &self.satrlar[self.indeks];
            self.indeks += 1;
            Some(s.as_str()) // iterator ga reference — 'this lifetime
        }
    }

    let mut sl = StringLenIter {
        satrlar: vec!["salom".into(), "dunyo".into(), "rust".into()],
        indeks: 0,
    };

    while let Some(s) = sl.keyingi() {
        println!("  GAT element: '{}'", s);
    }
    // GAT element: 'salom'
    // GAT element: 'dunyo'
    // GAT element: 'rust'
}

trait StreamingIter {
    type Item<'iter> where Self: 'iter;

    fn next<'iter>(&'iter mut self) -> Option<Self::Item<'iter>>;

    fn count(mut self) -> usize where Self: Sized {
        let mut n = 0;
        while self.next().is_some() { n += 1; }
        n
    }
}

// ── 1. Slice iterator — o'ziga reference qaytaradi
struct SliceChunks<'a, T> {
    ma_lumot: &'a [T],
    chunk_size: usize,
    pozitsiya: usize,
}

impl<'a, T> SliceChunks<'a, T> {
    fn new(ma_lumot: &'a [T], chunk_size: usize) -> Self {
        SliceChunks { ma_lumot, chunk_size, pozitsiya: 0 }
    }
}

impl<'data, T: 'data> StreamingIter for SliceChunks<'data, T> {
    type Item<'iter> = &'iter [T] where Self: 'iter;

    fn next<'iter>(&'iter mut self) -> Option<&'iter [T]> {
        if self.pozitsiya >= self.ma_lumot.len() { return None; }
        let end = (self.pozitsiya + self.chunk_size).min(self.ma_lumot.len());
        let chunk = &self.ma_lumot[self.pozitsiya..end];
        self.pozitsiya = end;
        Some(chunk)
    }
}

// ── 2. File-like reader — bufer qaytaradi
struct BuferOquvchi {
    ma_lumot: Vec<u8>,
    pozitsiya: usize,
    bufer: Vec<u8>,
    chunk_size: usize,
}

impl BuferOquvchi {
    fn new(ma_lumot: Vec<u8>, chunk_size: usize) -> Self {
        BuferOquvchi {
            ma_lumot, pozitsiya: 0,
            bufer: vec![0u8; chunk_size], chunk_size,
        }
    }
}

impl StreamingIter for BuferOquvchi {
    type Item<'iter> = &'iter [u8] where Self: 'iter;

    fn next<'iter>(&'iter mut self) -> Option<&'iter [u8]> {
        if self.pozitsiya >= self.ma_lumot.len() { return None; }
        let end = (self.pozitsiya + self.chunk_size).min(self.ma_lumot.len());
        let soni = end - self.pozitsiya;
        self.bufer[..soni].copy_from_slice(&self.ma_lumot[self.pozitsiya..end]);
        self.pozitsiya = end;
        Some(&self.bufer[..soni])
    }
}

fn streaming_iter_misoli() {

    println!("\n=== STREAMING ITERATOR ===");

    // SliceChunks
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut chunks = SliceChunks::new(&v, 3);

    while let Some(chunk) = chunks.next() {
        println!("  Chunk: {:?}", chunk);
    }
    // Chunk: [1, 2, 3]
    // Chunk: [4, 5, 6]
    // Chunk: [7, 8, 9]
    // Chunk: [10]

    // BuferOquvchi
    println!();
    let ma_lumot: Vec<u8> = b"Salom Rust GATs!".to_vec();
    let mut oquvchi = BuferOquvchi::new(ma_lumot, 4);

    while let Some(buf) = oquvchi.next() {
        println!("  Bufer: {:?} = '{}'",
                 buf, std::str::from_utf8(buf).unwrap_or("?"));
    }
    // Bufer: [83, 97, 108, 111] = 'Salo'
    // Bufer: [109, 32, 82, 117] = 'm Ru'
    // ...
}

// Higher-kinded type simulyatsiya
// Симуляция типов высшего порядка
trait Container {
    type Elem;
    type Ref<'a>: Copy where Self: 'a;  // Ref Copy bo'lishi kerak sum uchun
    type RefMut<'a> where Self: 'a;
    type Iter<'a>: Iterator<Item = Self::Ref<'a>> where Self: 'a;

    fn ol<'a>(&'a self, i: usize) -> Option<Self::Ref<'a>>;
    fn ol_mut<'a>(&'a mut self, i: usize) -> Option<Self::RefMut<'a>>;
    fn iter<'a>(&'a self) -> Self::Iter<'a>;
    fn uzunlik(&self) -> usize;
}

struct VecContainer<T>(Vec<T>);

impl<T: Clone + fmt::Debug + 'static> Container for VecContainer<T> {
    type Elem = T;
    type Ref<'a> = &'a T where Self: 'a;
    type RefMut<'a> = &'a mut T where Self: 'a;
    type Iter<'a> = std::slice::Iter<'a, T> where Self: 'a;

    fn ol<'a>(&'a self, i: usize) -> Option<&'a T> { self.0.get(i) }
    fn ol_mut<'a>(&'a mut self, i: usize) -> Option<&'a mut T> { self.0.get_mut(i) }
    fn iter<'a>(&'a self) -> std::slice::Iter<'a, T> { self.0.iter() }
    fn uzunlik(&self) -> usize { self.0.len() }
}

// Umumiy funksiya — Container trait bilan ishlaydi
fn container_yig_indi<C>(c: &C) -> i32
where
    C: Container<Elem = i32>,
    for<'a> C::Ref<'a>: std::ops::Deref<Target = i32>,
{
    c.iter().map(|r| *r).sum()
}

fn container_misoli() {

    println!("\n=== GAT CONTAINER ===");

    let mut v = VecContainer(vec![10, 20, 30, 40, 50]);

    println!("Uzunlik: {}", v.uzunlik());
    println!("v[2] = {:?}", v.ol(2));
    println!("Yig'indi: {}", container_yig_indi(&v));

    if let Some(elem) = v.ol_mut(1) {
        *elem *= 2;
    }
    println!("v[1]*=2: {:?}", v.ol(1));

    // Iterator
    let collected: Vec<i32> = v.iter().copied().collect();
    println!("All: {:?}", collected);
    // Uzunlik: 5
    // v[2] = Some(30)
    // Yig'indi: 150
    // v[1]*=2: Some(40)
    // All: [10, 40, 30, 40, 50]
}

trait Functor {
    type Item;
    type Mapped<B>;

    fn map<B, F: FnMut(Self::Item) -> B>(self, f: F) -> Self::Mapped<B>;
}

impl<T> Functor for Option<T> {
    type Item = T;
    type Mapped<B> = Option<B>;

    fn map<B, F: FnMut(T) -> B>(self, mut f: F) -> Option<B> {
        self.map(|x| f(x))
    }
}

impl<T, E> Functor for Result<T, E> {
    type Item = T;
    type Mapped<B> = Result<B, E>;

    fn map<B, F: FnMut(T) -> B>(self, f: F) -> Result<B, E> {
        self.map(f)
    }
}

impl<T> Functor for Vec<T> {
    type Item = T;
    type Mapped<B> = Vec<B>;

    fn map<B, F: FnMut(T) -> B>(self, f: F) -> Vec<B> {
        self.into_iter().map(f).collect()
    }
}

fn functor_misoli() {

    println!("\n=== GAT FUNCTOR ===");

    let opt: Option<i32> = Some(21);
    let doubled = opt.map(|x| x * 2);
    println!("Option map: {:?}", doubled); // Some(42)

    let res: Result<i32, &str> = Ok(10);
    let res_mapped = res.map(|x| format!("qiymat: {}", x));
    println!("Result map: {:?}", res_mapped); // Ok("qiymat: 10")

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let squares: Vec<i32> = v.map(|x| x * x);
    println!("Vec map: {:?}", squares); // [1, 4, 9, 16, 25]
    // Some(42)
    // Ok("qiymat: 10")
    // [1, 4, 9, 16, 25]
}

use std::future::Future;
use std::pin::Pin;

// GAT bilan async trait (async-trait crate olmay)
// async trait без async-trait crate
trait AsyncKochiruvchi {
    type KochirFuture<'this>: Future<Output = Result<usize, String>>
    where Self: 'this;

    fn ko_chir<'this>(&'this self, joy: &'this str, ma_lumot: &'this [u8]) -> Self::KochirFuture<'this>;
}

// Sodda implementatsiya — sync future (haqiqiy async executor siz)
struct FaylKochiruvchi { prefiks: String }

impl FaylKochiruvchi {
    fn new(prefiks: &str) -> Self { FaylKochiruvchi { prefiks: prefiks.to_string() } }
}

impl AsyncKochiruvchi for FaylKochiruvchi {
    type KochirFuture<'this> = std::future::Ready<Result<usize, String>>
    where Self: 'this;

    fn ko_chir<'this>(&'this self, joy: &'this str, ma_lumot: &'this [u8]) -> Self::KochirFuture<'this> {
        let to_liq_joy = format!("{}/{}", self.prefiks, joy);
        println!("[Ko'chiruvchi] {} ga {} bayt", to_liq_joy, ma_lumot.len());
        std::future::ready(Ok(ma_lumot.len()))
    }
}

fn noop_waker() -> std::task::Waker {
    use std::task::{RawWaker, RawWakerVTable, Waker};
    unsafe fn clone(_: *const ()) -> RawWaker { RawWaker::new(std::ptr::null(), &VT) }
    unsafe fn noop(_: *const ()) {}
    static VT: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VT)) }
}

fn async_trait_misoli() {

    println!("\n=== GAT ASYNC TRAIT ===");

    let ko_chiruvchi = FaylKochiruvchi::new("/tmp");
    let waker = noop_waker();
    let mut cx = std::task::Context::from_waker(&waker);

    let ma_lumot = b"Hello, WASM and GATs!";
    let mut fut = ko_chiruvchi.ko_chir("output.bin", ma_lumot);

    match Pin::new(&mut fut).poll(&mut cx) {
        std::task::Poll::Ready(Ok(n))  => println!("Ko'chirildi: {} bayt", n),
        std::task::Poll::Ready(Err(e)) => println!("Xato: {}", e),
        std::task::Poll::Pending       => println!("Kutmoqda..."),
    }
    // [Ko'chiruvchi] /tmp/output.bin ga 21 bayt
    // Ko'chirildi: 21 bayt
}

// Database cursor simulyatsiya — GAT bilan
trait DbCursor {
    type Row<'row> where Self: 'row;
    type Error: fmt::Debug;

    fn keyingi<'this>(&'this mut self) -> Option<Result<Self::Row<'this>, Self::Error>>;
    fn yopish(self);
}

#[derive(Debug)]
struct SqlXato(String);

impl fmt::Display for SqlXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "SQL xato: {}", self.0) }
}

struct RowData<'a>(&'a HashMap<String, String>);

impl<'a> RowData<'a> {
    fn ustun(&self, nom: &str) -> Option<&str> {
        self.0.get(nom).map(|s| s.as_str())
    }
}

struct MockCursor {
    qatorlar: Vec<HashMap<String, String>>,
    indeks: usize,
}

impl MockCursor {
    fn new(qatorlar: Vec<HashMap<String, String>>) -> Self {
        MockCursor { qatorlar, indeks: 0 }
    }
}

impl DbCursor for MockCursor {
    type Row<'row> = RowData<'row> where Self: 'row;
    type Error = SqlXato;

    fn keyingi<'this>(&'this mut self) -> Option<Result<RowData<'this>, SqlXato>> {
        if self.indeks >= self.qatorlar.len() { return None; }
        let row = &self.qatorlar[self.indeks];
        self.indeks += 1;
        Some(Ok(RowData(row)))
    }

    fn yopish(self) {
        println!("[Cursor] Yopildi ({} qator o'qildi)", self.indeks);
    }
}

fn db_cursor_misoli() {

    println!("\n=== GAT DB CURSOR ===");

    let qatorlar = vec![
        [("id", "1"), ("ism", "Dilshod"), ("yosh", "22")].iter()
            .map(|&(k,v)| (k.to_string(), v.to_string())).collect::<HashMap<_,_>>(),
        [("id", "2"), ("ism", "Ali"), ("yosh", "25")].iter()
            .map(|&(k,v)| (k.to_string(), v.to_string())).collect::<HashMap<_,_>>(),
        [("id", "3"), ("ism", "Vali"), ("yosh", "20")].iter()
            .map(|&(k,v)| (k.to_string(), v.to_string())).collect::<HashMap<_,_>>(),
    ];

    let mut cursor = MockCursor::new(qatorlar);

    while let Some(result) = cursor.keyingi() {
        match result {
            Ok(row) => {
                println!("  id={}, ism={}, yosh={}",
                         row.ustun("id").unwrap_or("?"),
                         row.ustun("ism").unwrap_or("?"),
                         row.ustun("yosh").unwrap_or("?"),
                );
            }
            Err(e) => println!("  Xato: {}", e),
        }
    }

    cursor.yopish();
    // id=1, ism=Dilshod, yosh=22
    // id=2, ism=Ali, yosh=25
    // id=3, ism=Vali, yosh=20
    // [Cursor] Yopildi (3 qator o'qildi)
}

fn main() {

    gat_asosiy_misoli();
    streaming_iter_misoli();
    container_misoli();
    functor_misoli();
    async_trait_misoli();
    db_cursor_misoli();

    println!("\n=== XULOSA ===");
    println!("GATs imkoniyatlari:");
    println!("  Streaming Iterator    — element iterator dan reference");
    println!("  Lending Iterator      — yashash vaqti bo'yicha bog'liq");
    println!("  Higher-kinded types   — Container, Functor, Monad");
    println!("  Async Trait (no macro)— async-trait crate siz");
    println!("  DB Cursor             — qator reference o'qish");
    println!();
    println!("Rust 1.65+ da stable!");
    println!("where Self: 'this — GAT constraint kalit so'zi");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        GAT ASOSLARI                                                                                          |
// #================================================================================================================================================#
// # |   1 | type Item<'a> where Self: 'a    | Lifetime bilan GAT                         | GAT с lifetime                                          |
// # |   2 | type Mapped<B>                  | Generic tur parametrli GAT                 | GAT с параметром типа                                   |
// # |   3 | type Future<'a>: Future<Output> | Async trait GAT                            | GAT для async trait                                     |
// # |   4 | fn next<'this>(&'this mut self) | GAT qaytaruvchi funksiya                   | Функция возвращающая GAT                                |
// #================================================================================================================================================#
// # |                                        PATTERNLAR                                                                                            |
// #================================================================================================================================================#
// # |   5 | Streaming Iterator              | Item<'iter> = &'iter T                     | Item<'iter> = &'iter T                                  |
// # |   6 | Container                       | Ref<'a>, RefMut<'a>, Iter<'a>              | Ref<'a>, RefMut<'a>, Iter<'a>                           |
// # |   7 | Functor                         | Mapped<B> — tur o'zgartirish               | Mapped<B> — преобразование типа                         |
// # |   8 | Async Trait                     | Ko_chirFuture<'this>: Future               | Ko_chirFuture<'this>: Future                            |
// # |   9 | DB Cursor                       | Row<'row> — qatorga reference              | Row<'row> — ссылка на строку                            |
// #================================================================================================================================================#
// # |                                        CHEKLOVLAR                                                                                            |
// #================================================================================================================================================#
// # |  10 | where Self: 'a                  | GAT constraint — zaruriy                   | GAT constraint — необходим                              |
// # |  11 | Rust 1.65+                      | Stable bosqichdan GATs                     | GATs с стабильной версии                                |
// # |  12 | Polonius                        | Lifetime solver — GAT uchun yangi          | Новый resolver lifetime для GAT                         |
// #================================================================================================================================================#