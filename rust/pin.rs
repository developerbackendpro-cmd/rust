// #================================================================================================================================================#
// #                                                                   PIN / UNPIN                                                                  #
// #                        PIN — XOTIRADA QULFLASH. UNPIN — KO'CHIRISH XAVFSIZ. SELF-REFERENTIAL. ASYNC FUTURE UCHUN MUHIM.                        #
// #                        PIN — ЗАКРЕПЛЕНИЕ В ПАМЯТИ. UNPIN — ПЕРЕМЕЩЕНИЕ БЕЗОПАСНО. SELF-REFERENTIAL. ВАЖНО ДЛЯ ASYNC FUTURE.                    #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::pin::Pin;
use std::marker::{PhantomPinned, PhantomData};
use std::ptr::NonNull;
use std::fmt;

// Pin / Unpin nima:
// Что такое Pin / Unpin:
//
//   Unpin — marker trait: T ni xotiradan ko'chirish XAVFSIZ
//   Unpin — маркерный трейт: перемещение T из памяти БЕЗОПАСНО
//   Aksariyat turlar: Unpin (i32, String, Vec, ...)
//   Большинство типов: Unpin (i32, String, Vec, ...)
//
//   !Unpin — T ni ko'chirish XAVFLI (self-referential)
//   !Unpin — перемещение T ОПАСНО (self-referential)
//   PhantomPinned → T: !Unpin qiladi
//   PhantomPinned → делает T: !Unpin
//
//   Pin<P> — P pointer ga kirish huquqini cheklaydi
//   Pin<P> — ограничивает доступ к указателю P
//   Pin<&mut T> — T xotirada qulflangan (ko'chirilmaydi)
//   Pin<&mut T> — T заблокирован в памяти (не перемещается)
//   Pin<Box<T>> — heap da qulflangan
//   Pin<Box<T>> — заблокирован в куче
//
//   get_mut() — faqat T: Unpin bo'lsa (xavfsiz)
//   get_mut() — только если T: Unpin (безопасно)
//   get_unchecked_mut() — unsafe, T: !Unpin bo'lsa ham
//   get_unchecked_mut() — unsafe, даже если T: !Unpin
//
//   NIMA UCHUN KERAK:
//   ЗАЧЕМ НУЖЕН:
//   async fn → state machine → self-referential struct
//   → ko'chirish xotiradagi referenslarni buzadi
//   → перемещение нарушает ссылки внутри структуры

fn unpin_misollari() {

    // Aksariyat turlar Unpin — Pin<&mut T> dan &mut T olish mumkin
    // Большинство типов Unpin — из Pin<&mut T> можно получить &mut T

    let mut n: i32 = 42;
    let mut pinned: Pin<&mut i32> = Pin::new(&mut n);

    // get_mut() — faqat Unpin bo'lsa ishlaydi
    // get_mut() — работает только если Unpin
    let r: &mut i32 = pinned.as_mut().get_mut();
    *r += 8;
    println!("{}", n); // 50
    // 50

    // Pin::new() — faqat T: Unpin bo'lsa
    // Pin::new() — только если T: Unpin
    let mut s = String::from("salom");
    let mut ps: Pin<&mut String> = Pin::new(&mut s);
    ps.as_mut().get_mut().push_str(" dunyo");
    println!("{}", s); // salom dunyo
    // salom dunyo

    // Vec — Unpin
    let mut v: Vec<i32> = vec![1, 2, 3];
    let pv: Pin<&mut Vec<i32>> = Pin::new(&mut v);
    // pv.get_mut().push(4); // mumkin — Unpin

    // Unpin turlar uchun Pin hech qanday kafolat bermaydi
    // Для Unpin типов Pin не даёт никаких гарантий
    println!("i32: Unpin = {}", is_unpin::<i32>());
    println!("String: Unpin = {}", is_unpin::<String>());
    println!("Vec<i32>: Unpin = {}", is_unpin::<Vec<i32>>());
    // i32: Unpin = true
    // String: Unpin = true
    // Vec<i32>: Unpin = true
}

fn is_unpin<T: Unpin>() -> bool { true }

// PhantomPinned — struct ni !Unpin qilish
// PhantomPinned — сделать структуру !Unpin
#[derive(Debug)]
struct Pinlangan {
    qiymat: i32,
    _pin: PhantomPinned, // bu !Unpin qiladi
}

impl Pinlangan {
    fn new(qiymat: i32) -> Self {
        Pinlangan { qiymat, _pin: PhantomPinned }
    }

    // Pin<&mut Self> kerak — &mut Self emas
    // Нужен Pin<&mut Self> — не &mut Self
    fn qiymat_qo_y(self: Pin<&mut Self>, yangi: i32) {
        // get_unchecked_mut() — unsafe, !Unpin uchun
        // get_unchecked_mut() — unsafe, для !Unpin
        unsafe { self.get_unchecked_mut().qiymat = yangi; }
    }

    fn qiymat_ol(self: Pin<&Self>) -> i32 {
        self.qiymat
    }
}

fn phantompinned_misoli() {

    // Box::pin — heap da Pin yaratish
    // Box::pin — создание Pin в куче
    let mut pinlangan: Pin<Box<Pinlangan>> = Box::pin(Pinlangan::new(42));

    println!("{}", pinlangan.as_ref().qiymat_ol()); // 42
    pinlangan.as_mut().qiymat_qo_y(100);
    println!("{}", pinlangan.as_ref().qiymat_ol()); // 100
    // 42
    // 100

    // Pin::new() — !Unpin uchun XATO
    // Pin::new() — ОШИБКА для !Unpin
    // let p: Pin<&mut Pinlangan> = Pin::new(&mut p); // ← KOMPILE XATO

    // pin! makrosi — stack da pin (Rust 1.68+)
    // Макрос pin! — pin на стеке (Rust 1.68+)
    // use std::pin::pin;
    // let mut p = pin!(Pinlangan::new(10));

    // unsafe — stack da pin
    // unsafe — pin на стеке
    let mut stack_p = Pinlangan::new(77);
    let mut pinned_stack: Pin<&mut Pinlangan> = unsafe {
        Pin::new_unchecked(&mut stack_p)
    };
    println!("{}", pinned_stack.as_ref().qiymat_ol()); // 77
    pinned_stack.as_mut().qiymat_qo_y(88);
    println!("{}", pinned_stack.as_ref().qiymat_ol()); // 88
    // 77
    // 88
}

// Self-referential struct — o'zining boshqa maydoniga pointer saqlaydi
// Self-referential struct — хранит указатель на другое поле

struct SelfRef {
    qiymat: String,
    ptr: *const String, // qiymat ga ko'rsatadi
    _pin: PhantomPinned,
}

impl SelfRef {
    fn yangi(qiymat: &str) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfRef {
            qiymat: qiymat.to_string(),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });

        // qiymat ning manzilini ptr ga saqlash
        // Сохранение адреса qiymat в ptr
        let ptr = &boxed.qiymat as *const String;
        unsafe {
            boxed.as_mut().get_unchecked_mut().ptr = ptr;
        }
        boxed
    }

    fn qiymat(self: Pin<&Self>) -> &str {
        // get_ref() dan reference olamiz
        // Берём ссылку из get_ref()
        let this = self.get_ref();
        &this.qiymat
    }

    fn ptr_qiymati(self: Pin<&Self>) -> &str {
        // ptr hali ham to'g'ri — ko'chirilmagan!
        // ptr всё ещё правильный — не перемещён!
        let this = self.get_ref();
        unsafe { &*this.ptr }
    }

    fn bir_xilmi(self: Pin<&Self>) -> bool {
        let this = self.get_ref();
        &this.qiymat as *const String == this.ptr
    }
}

impl fmt::Debug for SelfRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SelfRef {{ qiymat: {:?}, ptr_valid: {} }}", self.qiymat, !self.ptr.is_null())
    }
}

fn self_referential_misoli() {

    let sr = SelfRef::yangi("salom dunyo");

    println!("qiymat: {}", sr.as_ref().qiymat());
    println!("ptr: {}", sr.as_ref().ptr_qiymati());
    println!("Bir xilmi: {}", sr.as_ref().bir_xilmi());
    // qiymat: salom dunyo
    // ptr: salom dunyo
    // Bir xilmi: true

    // MUHIM: sr ko'chirilmaydi — ptr hali ham to'g'ri
    // ВАЖНО: sr не перемещается — ptr всё ещё правильный
    // let sr2 = *sr; // ← XATO! !Unpin ko'chirilmaydi

    // Box::pin bilan ko'chirish simulyatsiyasi
    // Симуляция — даже при перемещении Box ptr корректен
    let sr2 = SelfRef::yangi("ikkinchi");
    println!("{:?}", *sr2);
    // SelfRef { qiymat: "ikkinchi", ptr_valid: true }

    // Nima uchun Pin kerak — tushuntirish
    println!("\nNima uchun Pin kerak:");
    println!("  async fn → state machine → o'ziga pointer saqlaydi");
    println!("  Ko'chirilsa → pointer noto'g'ri → undefined behavior");
    println!("  Pin → ko'chirishni taqiqlaydi → xavfsiz");
}

fn pin_metodlari_misollari() {

    // 1. Pin::new(r) — faqat T: Unpin
    // 1. Pin::new(r) — только T: Unpin
    let mut n = 42i32;
    let p: Pin<&mut i32> = Pin::new(&mut n);
    println!("Pin::new: {:?}", p); // 42
    // 42

    // 2. Pin::new_unchecked(r) — unsafe, istalgan T
    // 2. Pin::new_unchecked(r) — unsafe, любой T
    let mut m = 100i32;
    let p2: Pin<&mut i32> = unsafe { Pin::new_unchecked(&mut m) };
    println!("Pin::new_unchecked: {:?}", p2); // 100
    // 100

    // 3. as_ref() — Pin<&mut T> → Pin<&T>
    // 3. as_ref() — Pin<&mut T> → Pin<&T>
    let mut v = vec![1, 2, 3];
    let mut pv: Pin<&mut Vec<i32>> = Pin::new(&mut v);
    let r: Pin<&Vec<i32>> = pv.as_ref();
    println!("{:?}", *r); // [1, 2, 3]
    // [1, 2, 3]

    // 4. as_mut() — Pin<&mut T> → Pin<&mut T> (re-borrow)
    // 4. as_mut() — Pin<&mut T> → Pin<&mut T> (ре-заимствование)
    pv.as_mut().get_mut().push(4);
    println!("{:?}", *pv); // [1, 2, 3, 4]
    // [1, 2, 3, 4]

    // 5. get_ref() — Pin<&T> → &T (faqat immutable)
    // 5. get_ref() — Pin<&T> → &T (только иммутабельный)
    let s = String::from("salom");
    let ps: Pin<&String> = Pin::new(&s);
    let sr: &String = ps.get_ref();
    println!("{}", sr); // salom
    // salom

    // 6. get_mut() — Pin<&mut T> → &mut T (faqat T: Unpin)
    // 6. get_mut() — Pin<&mut T> → &mut T (только T: Unpin)
    let mut x = 10i32;
    let mut px: Pin<&mut i32> = Pin::new(&mut x);
    let xr: &mut i32 = px.get_mut(); // Unpin bo'lgani uchun OK
    *xr = 20;
    println!("{}", x); // 20
    // 20

    // 7. get_unchecked_mut() — unsafe, istalgan T uchun
    // 7. get_unchecked_mut() — unsafe, для любого T
    let mut y = 30i32;
    let mut py: Pin<&mut i32> = Pin::new(&mut y);
    unsafe { *py.as_mut().get_unchecked_mut() = 40; }
    println!("{}", y); // 40
    // 40

    // 8. Pin<Box<T>> — map_unchecked bilan
    // 8. Pin<Box<T>> — с map_unchecked
    let mut pb: Pin<Box<i32>> = Box::pin(50i32);
    *pb = 60;
    println!("{}", *pb); // 60
    // 60

    // 9. into_inner() — faqat T: Unpin
    // 9. into_inner() — только T: Unpin
    let pi: Pin<Box<i32>> = Box::pin(70i32);
    let inner_box: Box<i32> = Pin::into_inner(pi); // Box<i32> qaytaradi
    println!("{}", *inner_box); // 70
    // 70
}

use std::future::Future;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

// Noop waker
fn noop_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker { RawWaker::new(std::ptr::null(), &VTABLE) }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

// async fn → !Unpin Future yaratadi
// async fn → создаёт !Unpin Future
async fn oddiy_future(n: i32) -> i32 {
    n * n
}

// Pin<Box<dyn Future>> — heap da async future
// Pin<Box<dyn Future>> — async future в куче
fn async_future_misoli() {

    // async fn → !Unpin — Box::pin kerak
    // async fn → !Unpin — нужен Box::pin
    let mut future: Pin<Box<dyn Future<Output = i32>>> =
        Box::pin(oddiy_future(7));

    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);

    // poll() — Pin<&mut Self> talab qiladi
    // poll() — требует Pin<&mut Self>
    match future.as_mut().poll(&mut cx) {
        Poll::Ready(v)  => println!("Ready: {}", v), // 49
        Poll::Pending   => println!("Pending"),
    }
    // Ready: 49

    // Vec<Pin<Box<dyn Future>>> — turli future lar
    // Vec<Pin<Box<dyn Future>>> — различные future
    let mut futures: Vec<Pin<Box<dyn Future<Output = i32>>>> = vec![
        Box::pin(oddiy_future(3)),
        Box::pin(oddiy_future(4)),
        Box::pin(oddiy_future(5)),
    ];

    for mut f in futures {
        if let Poll::Ready(v) = f.as_mut().poll(&mut cx) {
            print!("{} ", v); // 9 16 25
        }
    }
    println!();
    // 9 16 25
}

// Pattern 1: Structural pinning
// Паттерн 1: Структурное закрепление
// Agar struct pin bo'lsa — ichki maydonlar ham pin
// Если struct закреплена — внутренние поля тоже закреплены
struct StructPinned {
    a: i32,            // Unpin — structural pinning emas
    b: PhantomPinned,  // !Unpin — structural pinning
}

impl StructPinned {
    fn yangi(a: i32) -> Pin<Box<Self>> {
        Box::pin(StructPinned { a, b: PhantomPinned })
    }

    // a ga mutable kirish — Unpin bo'lgani uchun xavfsiz
    // Мутабельный доступ к a — безопасно так как Unpin
    fn a_mut(self: Pin<&mut Self>) -> &mut i32 {
        unsafe { &mut self.get_unchecked_mut().a }
    }
}

// Pattern 2: Pin bilan builder
// Паттерн 2: Builder с Pin
struct AsyncIshchi {
    id: usize,
    holat: i32,
    _pin: PhantomPinned,
}

impl AsyncIshchi {
    fn yangi(id: usize) -> Pin<Box<Self>> {
        Box::pin(AsyncIshchi { id, holat: 0, _pin: PhantomPinned })
    }

    fn qadam(self: Pin<&mut Self>) -> Option<i32> {
        let this = unsafe { self.get_unchecked_mut() };
        this.holat += 1;
        if this.holat <= 3 {
            Some(this.holat * this.id as i32)
        } else {
            None
        }
    }
}

fn pinning_patternlar() {

    // Structural pinning
    let mut sp = StructPinned::yangi(10);
    *sp.as_mut().a_mut() = 20;
    println!("a: {}", sp.a); // 20
    // 20

    // Async ishchi
    let mut ishchi = AsyncIshchi::yangi(5);
    while let Some(natija) = ishchi.as_mut().qadam() {
        println!("Qadam: {}", natija);
    }
    // Qadam: 5
    // Qadam: 10
    // Qadam: 15

    // Pin::map_unchecked — maydon ga Pin
    // Pin::map_unchecked — Pin к полю
    let mut sp2 = StructPinned::yangi(42);
    let a_val: i32 = sp2.as_ref().a; // a — Unpin, to'g'ridan kirish mumkin
    println!("map_unchecked a: {}", a_val); // 42
}

// Intrusive future — Pin bilan
// Интрузивный future — с Pin
struct HisoblashFuture {
    bosqich: u32,
    qiymat: i64,
    _pin: PhantomPinned,
}

impl HisoblashFuture {
    fn yangi(boshlanish: i64) -> Pin<Box<Self>> {
        Box::pin(HisoblashFuture {
            bosqich: 0,
            qiymat: boshlanish,
            _pin: PhantomPinned,
        })
    }
}

impl Future for HisoblashFuture {
    type Output = i64;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i64> {
        let this = unsafe { self.get_unchecked_mut() };
        match this.bosqich {
            0 => { this.qiymat += 10; this.bosqich = 1; cx.waker().wake_by_ref(); Poll::Pending }
            1 => { this.qiymat *= 3;  this.bosqich = 2; cx.waker().wake_by_ref(); Poll::Pending }
            _ => { this.qiymat -= 5;  Poll::Ready(this.qiymat) }
        }
    }
}

fn block_on_pin<F: Future>(mut f: Pin<Box<F>>) -> F::Output {
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    loop {
        match f.as_mut().poll(&mut cx) {
            Poll::Ready(v)  => return v,
            Poll::Pending   => std::hint::spin_loop(),
        }
    }
}

fn real_hayot_misollari() {

    println!("--- HisoblashFuture ---");
    // (5 + 10) * 3 - 5 = 40
    let natija = block_on_pin(HisoblashFuture::yangi(5));
    println!("Natija: {}", natija); // 40
    // Natija: 40

    println!("\n--- Ko'p Future parallel ---");
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);

    let mut futures: Vec<Pin<Box<HisoblashFuture>>> = vec![
        HisoblashFuture::yangi(1),
        HisoblashFuture::yangi(2),
        HisoblashFuture::yangi(3),
    ];

    let mut natijalar: Vec<i64> = Vec::new();

    // Round-robin polling
    while !futures.is_empty() {
        futures.retain_mut(|f| {
            match f.as_mut().poll(&mut cx) {
                Poll::Ready(v) => { natijalar.push(v); false }
                Poll::Pending  => true,
            }
        });
    }

    natijalar.sort();
    println!("Natijalar: {:?}", natijalar);
    // (1+10)*3-5=28, (2+10)*3-5=31, (3+10)*3-5=34
    // Natijalar: [28, 31, 34]

    println!("\n--- Pin Xulosa ---");
    println!("✅ T: Unpin     — Pin::new(), get_mut() xavfsiz");
    println!("⚠️  T: !Unpin   — Box::pin(), get_unchecked_mut() unsafe kerak");
    println!("📌 async fn    — !Unpin Future yaratadi");
    println!("📦 Box::pin    — heap da qulflash");
    println!("🔒 Pin<&mut T> — ko'chirishni taqiqlaydi");
}

fn main() {

    println!("=== UNPIN MISOLLARI ===");
    unpin_misollari();

    println!("\n=== PHANTOMPINNED ===");
    phantompinned_misoli();

    println!("\n=== SELF-REFERENTIAL ===");
    self_referential_misoli();

    println!("\n=== PIN METODLARI ===");
    pin_metodlari_misollari();

    println!("\n=== ASYNC FUTURE BILAN ===");
    async_future_misoli();

    println!("\n=== PINNING PATTERNLAR ===");
    pinning_patternlar();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        UNPIN                                                                                                 |
// #================================================================================================================================================#
// # |   1 | T: Unpin                        | Ko'chirish xavfsiz (aksariyat turlar)      | Перемещение безопасно (большинство типов)               |
// # |   2 | PhantomPinned                   | T: !Unpin qilish                           | Сделать T: !Unpin                                       |
// # |   3 | async fn → !Unpin               | Async state machine                        | Асинхронная машина состояний                            |
// #================================================================================================================================================#
// # |                                        PIN YARATISH                                                                                          |
// #================================================================================================================================================#
// # |   4 | Pin::new(&mut T)                | Faqat T: Unpin bo'lsa                      | Только если T: Unpin                                    |
// # |   5 | Pin::new_unchecked(&mut T)      | Unsafe, istalgan T                         | Unsafe, любой T                                         |
// # |   6 | Box::pin(val)                   | Heap da Pin yaratish                       | Создание Pin в куче                                     |
// # |   7 | pin!(val) makros                | Stack da Pin (Rust 1.68+)                  | Pin на стеке (Rust 1.68+)                               |
// #================================================================================================================================================#
// # |                                        PIN METODLARI                                                                                         |
// #================================================================================================================================================#
// # |   8 | p.as_ref()                      | Pin<&mut T> → Pin<&T>                      | Pin<&mut T> → Pin<&T>                                   |
// # |   9 | p.as_mut()                      | Pin<&mut T> re-borrow                      | Pin<&mut T> ре-заимствование                            |
// # |  10 | p.get_ref()                     | Pin<&T> → &T                               | Pin<&T> → &T                                            |
// # |  11 | p.get_mut()                     | Pin<&mut T> → &mut T (faqat Unpin)         | Pin<&mut T> → &mut T (только Unpin)                     |
// # |  12 | p.get_unchecked_mut()           | Unsafe — istalgan T uchun                  | Unsafe — для любого T                                   |
// # |  13 | Pin::into_inner(p)              | Pin<Box<T>> → T (faqat Unpin)              | Pin<Box<T>> → T (только Unpin)                          |
// # |  14 | p.map_unchecked_mut(\|t\| f)    | Maydon ga Pin (unsafe)                     | Pin к полю (unsafe)                                     |
// #================================================================================================================================================#
// # |                                        PATTERNLAR                                                                                            |
// #================================================================================================================================================#
// # |  15 | Self-referential struct         | Pin zarur — ptr buzilmasligi               | Pin необходим — чтобы ptr не сломался                   |
// # |  16 | Structural pinning              | Struct pin → maydon ham pin                | Struct pin → поле тоже pin                              |
// # |  17 | Box<dyn Future> + Box::pin      | Turli xil futuralar saqlash                | Хранение разнообразных future                           |
// # |  18 | Future::poll(Pin<&mut Self>)    | poll() Pin talab qiladi                    | poll() требует Pin                                      |
// #================================================================================================================================================#