// #================================================================================================================================================#
// #                                                        ASYNC / AWAIT + FUTURE                                                                  #
// #                        ASYNC RUST CHUQUR — FUTURE TRAIT, POLL, WAKER, PIN, EXECUTOR, RUNTIME. TOKIO OLMAY TUSHUNISH.                           #
// #                        ASYNC RUST ГЛУБОКО — FUTURE TRAIT, POLL, WAKER, PIN, EXECUTOR, RUNTIME. БЕЗ TOKIO ПОНЯТЬ.                               #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Async/Await nima:
// Что такое Async/Await:
//
//   async fn — Future qaytaruvchi funksiya
//   async fn — функция возвращающая Future
//   await    — Future tayyor bo'lguncha to'xtatish (suspend)
//   await    — приостановить пока Future не готов (suspend)
//
//   Future trait:
//   trait Future {
//       type Output;
//       fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
//   }
//
//   Poll enum:
//   enum Poll<T> {
//       Ready(T),    — tayyor, qiymat bor
//       Pending,     — hali tayyor emas, keyinroq tekshir
//   }
//
//   Async Rust qanday ishlaydi:
//   Как работает Async Rust:
//   1. async fn → Future struct ga aylantiradi (desugaring)
//      async fn → преобразуется в Future struct
//   2. Executor Future ni poll() qiladi
//      Executor вызывает poll() у Future
//   3. Pending → Waker orqali qayta xabar beradi
//      Pending → уведомляет снова через Waker
//   4. Ready → natija tayyor
//      Ready → результат готов

// Bu async fn:
// Эта async fn:
// async fn kvadrat(n: i32) -> i32 { n * n }
//
// Bu Future ga aylanadi:
// Преобразуется в такой Future:
struct KvadratFuture {
    n: i32,
}

impl Future for KvadratFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<i32> {
        Poll::Ready(self.n * self.n) // darhol tayyor
    }
}

fn kvadrat(n: i32) -> KvadratFuture {
    KvadratFuture { n }
}

// Murakkab Future — State machine
// Сложный Future — машина состояний
//
// Bu async fn:
// Эта async fn:
// async fn ikki_qadam() -> String {
//     let a = birinchi_qadam().await;  // ← to'xtatish nuqtasi 1
//     let b = ikkinchi_qadam(a).await; // ← to'xtatish nuqtasi 2
//     format!("{}", b)
// }
//
// Bu state machine ga aylanadi:
// Преобразуется в такую машину состояний:
#[allow(dead_code)]
enum IkkiQadamHolat {
    Holat0,                           // boshlang'ich
    Holat1 { birinchi_future: i32 },  // birinchi await dan keyin
    Holat2 { natija: String },        // ikkinchi await dan keyin
    Tugadi,
}

// Waker — Future ni qayta poll qilishni so'rash uchun
// Waker — для запроса повторного poll у Future

fn noop_waker() -> Waker {
    // Hech narsa qilmaydigan waker — oddiy executor uchun
    // Ничего не делающий waker — для простого executor
    unsafe fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

// block_on — Future ni sinxron bajarish
// block_on — синхронное выполнение Future
fn block_on<F: Future>(mut future: F) -> F::Output {
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);

    loop {
        let pinned = unsafe { Pin::new_unchecked(&mut future) };
        match pinned.poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending  => {
                // Real executor: waker orqali bildirishni kutadi
                // Real executor: ждёт уведомления через waker
                thread::yield_now();
            }
        }
    }
}

fn sodda_executor_misoli() {

    // KvadratFuture ni bajarish
    let natija = block_on(kvadrat(7));
    println!("{}", natija);
    // 49

    // Future zanjiri — pollable
    let n1 = block_on(kvadrat(3));
    let n2 = block_on(kvadrat(n1));
    println!("{}", n2); // 3^4 = 81
    // 81
}

// Timer Future — ma'lum vaqtdan keyin tayyor bo'ladi
// Timer Future — становится готовым через определённое время
struct TimerFuture {
    boshlanish: Instant,
    kutish_ms: u64,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl TimerFuture {
    fn new(kutish_ms: u64) -> Self {
        let waker_arc: Arc<Mutex<Option<Waker>>> = Arc::new(Mutex::new(None));
        let waker_clone = Arc::clone(&waker_arc);

        // Fon thread — vaqt tugaganda waker ni chaqiradi
        // Фоновый поток — вызывает waker когда время истекло
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(kutish_ms));
            if let Some(waker) = waker_clone.lock().unwrap().take() {
                waker.wake();
            }
        });

        TimerFuture {
            boshlanish: Instant::now(),
            kutish_ms,
            waker: waker_arc,
        }
    }
}

impl Future for TimerFuture {
    type Output = Duration;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Duration> {
        let o_tgan = self.boshlanish.elapsed();
        if o_tgan >= Duration::from_millis(self.kutish_ms) {
            Poll::Ready(o_tgan)
        } else {
            // Waker ni saqlash — vaqt tugaganda chaqiriladi
            // Сохранить waker — будет вызван когда время истечёт
            *self.waker.lock().unwrap() = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// async fn fetch_data(url: &str) -> Result<String, Error> {
//     let connection = connect(url).await;   // ← I/O kutish
//     let data = connection.read().await;    // ← I/O kutish
//     Ok(data)
// }
//
// Bu bir vaqtda BOSHQA VAZIFALAR bilan ishlash imkonini beradi:
// Это позволяет работать с ДРУГИМИ ЗАДАЧАМИ одновременно:
// Thread bloklanmaydi → ko'p vazifa bitta thread da!
// Поток не блокируется → много задач на одном потоке!

// Sodda async runtime simulyatsiyasi — macro_rules! yordamida
// Симуляция простого async runtime — с помощью macro_rules!

// async fn ni simulyatsiya qiluvchi Future
// Future симулирующий async fn
struct AsyncHisoblash {
    n: i32,
    bosqich: u8,
    oraliq: i32,
}

impl AsyncHisoblash {
    fn new(n: i32) -> Self {
        AsyncHisoblash { n, bosqich: 0, oraliq: 0 }
    }
}

impl Future for AsyncHisoblash {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        match self.bosqich {
            0 => {
                // 1-bosqich: qo'shish
                self.oraliq = self.n + 10;
                println!("[Bosqich 1] n + 10 = {}", self.oraliq);
                self.bosqich = 1;
                cx.waker().wake_by_ref(); // darhol qayta poll
                Poll::Pending
            }
            1 => {
                // 2-bosqich: ko'paytirish
                self.oraliq *= 3;
                println!("[Bosqich 2] * 3 = {}", self.oraliq);
                self.bosqich = 2;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            _ => {
                // 3-bosqich: natija
                let natija = self.oraliq - 5;
                println!("[Bosqich 3] - 5 = {}", natija);
                Poll::Ready(natija)
            }
        }
    }
}

fn async_state_machine_misoli() {

    let natija = block_on(AsyncHisoblash::new(5));
    println!("Yakuniy natija: {}", natija);
    // [Bosqich 1] n + 10 = 15
    // [Bosqich 2] * 3 = 45
    // [Bosqich 3] - 5 = 40
    // Yakuniy natija: 40
}

// Pin nima kerak:
// Зачем нужен Pin:
//
//   async fn ichida await bo'lsa — state machine o'ziga referens saqlaydi
//   В async fn с await — машина состояний хранит ссылки на себя
//   Bu struct xotirada ko'chirilishi mumkin emas (self-referential)
//   Такую структуру нельзя перемещать в памяти (self-referential)
//   Pin<&mut T> — T ni xotirada qulflaydi
//   Pin<&mut T> — запирает T в памяти
//
//   Unpin — ko'chirish xavfsiz (aksariyat turlar)
//   Unpin — перемещение безопасно (большинство типов)
//   !Unpin — ko'chirish xavfsiz emas (self-referential)
//   !Unpin — перемещение небезопасно (self-referential)

use std::marker::PhantomPinned;

// Self-referential struct — Pin kerak
// Self-referential структура — нужен Pin
struct SelfReferential {
    qiymat: String,
    ptr: *const String, // o'zining qiymat ga ko'rsatadi
    _pin: PhantomPinned,
}

impl SelfReferential {
    fn new(qiymat: String) -> Pin<Box<Self>> {
        let mut pinned = Box::pin(SelfReferential {
            qiymat,
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });

        let ptr = &pinned.qiymat as *const String;
        unsafe {
            pinned.as_mut().get_unchecked_mut().ptr = ptr;
        }
        pinned
    }

    fn qiymat(&self) -> &str {
        &self.qiymat
    }

    fn ptr_qiymati(&self) -> &str {
        unsafe { &*self.ptr }
    }
}

fn pin_misoli() {

    let sr = SelfReferential::new(String::from("salom"));

    println!("qiymat: {}", sr.qiymat());
    println!("ptr: {}", sr.ptr_qiymati());
    println!("Bir xilmi: {}", sr.qiymat() == sr.ptr_qiymati());
    // qiymat: salom
    // ptr: salom
    // Bir xilmi: true

    // Box::pin — Unpin bo'lmagan Future uchun
    // Box::pin — для Future не реализующих Unpin
    let pinned_future: Pin<Box<dyn Future<Output = i32>>> = Box::pin(kvadrat(6));
    let natija = block_on(pinned_future);
    println!("Pin<Box<dyn Future>>: {}", natija);
    // Pin<Box<dyn Future>>: 36

    // Pin<&mut T> — stack da pin
    // Pin<&mut T> — pin на стеке
    let mut hisob = AsyncHisoblash::new(2);
    let pinned = unsafe { Pin::new_unchecked(&mut hisob) };
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    match pinned.poll(&mut cx) {
        Poll::Ready(v) => println!("Ready: {}", v),
        Poll::Pending  => println!("Pending"),
    }
    // [Bosqich 1] n + 10 = 12
    // Pending
}

// Sodda executor bilan async fn ishlatish
// Использование async fn с простым executor

async fn kvadrat_async(n: i32) -> i32 {
    n * n
}

async fn kubik_async(n: i32) -> i32 {
    let q = kvadrat_async(n).await;
    q * n
}

async fn zanjirli_hisoblash(n: i32) -> String {
    let ikkilangan = n * 2;
    let kvadrat = kvadrat_async(ikkilangan).await;
    let kubik = kubik_async(n).await;
    format!("({}*2)^2={}, {}^3={}", n, kvadrat, n, kubik)
}

// Qo'lda future ulash (join pattern)
// Ручное объединение future (паттерн join)
struct JoinFuture<A: Future + Unpin, B: Future + Unpin> {
    a: Option<A>,
    b: Option<B>,
    a_natija: Option<A::Output>,
    b_natija: Option<B::Output>,
}

impl<A: Future + Unpin, B: Future + Unpin> Future for JoinFuture<A, B>
where
    A::Output: Unpin,
    B::Output: Unpin,
{
    type Output = (A::Output, B::Output);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<(A::Output, B::Output)> {
        if self.a_natija.is_none() {
            if let Some(a) = self.a.as_mut() {
                if let Poll::Ready(v) = Pin::new(a).poll(cx) {
                    self.a_natija = Some(v);
                    self.a = None;
                }
            }
        }

        if self.b_natija.is_none() {
            if let Some(b) = self.b.as_mut() {
                if let Poll::Ready(v) = Pin::new(b).poll(cx) {
                    self.b_natija = Some(v);
                    self.b = None;
                }
            }
        }

        match (self.a_natija.take(), self.b_natija.take()) {
            (Some(a), Some(b)) => Poll::Ready((a, b)),
            (a, b) => {
                self.a_natija = a;
                self.b_natija = b;
                Poll::Pending
            }
        }
    }
}

fn join_future<A: Future + Unpin, B: Future + Unpin>(a: A, b: B) -> JoinFuture<A, B> {
    JoinFuture { a: Some(a), b: Some(b), a_natija: None, b_natija: None }
}

fn async_fn_misollari() {

    // async fn block_on bilan
    // async fn с block_on
    let natija = block_on(kvadrat_async(8));
    println!("{}", natija);
    // 64

    let natija2 = block_on(kubik_async(3));
    println!("{}", natija2);
    // 27

    let natija3 = block_on(zanjirli_hisoblash(4));
    println!("{}", natija3);
    // (4*2)^2=64, 4^3=64

    // Join — ikki future parallel (Box::pin bilan Unpin qilamiz)
    // Join — два future параллельно (делаем Unpin через Box::pin)
    let j = join_future(
        Box::pin(kvadrat_async(5)),
        Box::pin(kvadrat_async(6)),
    );
    let (a, b) = block_on(j);
    println!("({}, {})", a, b);
    // (25, 36)
}

// Sodda task scheduler — Future lar navbati
// Простой планировщик задач — очередь Future
use std::collections::VecDeque;

struct Vazifalar {
    navbat: VecDeque<Pin<Box<dyn Future<Output = ()> + Send>>>,
}

impl Vazifalar {
    fn new() -> Self { Vazifalar { navbat: VecDeque::new() } }

    fn qo_sh<F: Future<Output = ()> + Send + 'static>(&mut self, f: F) {
        self.navbat.push_back(Box::pin(f));
    }

    fn ishga_tushir(&mut self) {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        while !self.navbat.is_empty() {
            let mut vazifa = self.navbat.pop_front().unwrap();
            match vazifa.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {}
                Poll::Pending   => self.navbat.push_back(vazifa), // qayta navbatga
            }
        }
    }
}

async fn xabar_chiqar(id: usize, xabar: &str) {
    println!("[Task {}] {}", id, xabar);
}

fn real_hayot_misollari() {

    // Async state machine
    println!("--- Async State Machine ---");
    async_state_machine_misoli();

    // Pin misoli
    println!("\n--- Pin ---");
    pin_misoli();

    // Async fn
    println!("\n--- Async Fn ---");
    async_fn_misollari();

    // Sodda scheduler
    println!("\n--- Sodda Scheduler ---");
    let mut sched = Vazifalar::new();
    sched.qo_sh(xabar_chiqar(1, "Birinchi vazifa"));
    sched.qo_sh(xabar_chiqar(2, "Ikkinchi vazifa"));
    sched.qo_sh(xabar_chiqar(3, "Uchinchi vazifa"));
    sched.ishga_tushir();
    // [Task 1] Birinchi vazifa
    // [Task 2] Ikkinchi vazifa
    // [Task 3] Uchinchi vazifa

    // Async rust tamoyillar xulosa
    println!("\n--- Xulosa ---");
    println!("1. async fn → Future struct (state machine)");
    println!("2. .await → Future ni poll qilish, Pending bo'lsa to'xtatish");
    println!("3. Executor → Future larni poll qiluvchi runtime");
    println!("4. Waker → Pending Future ni qayta poll qilish uchun signal");
    println!("5. Pin → self-referential Future ni xotirada qulflash");
    println!("6. Tokio/async-std → haqiqiy production executor");
}

fn main() {

    println!("=== FUTURE DESUGARING ===");
    sodda_executor_misoli();

    println!("\n=== ASYNC STATE MACHINE ===");
    async_state_machine_misoli();

    println!("\n=== PIN ===");
    pin_misoli();

    println!("\n=== ASYNC FN ===");
    async_fn_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                     | Tavsif (UZ)                               | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        FUTURE TRAIT                                                                                          |
// #================================================================================================================================================#
// # |   1 | trait Future { poll(...) }       | Asosiy trait — Poll::Ready/Pending        | Основной трейт — Poll::Ready/Pending                    |
// # |   2 | Poll::Ready(T)                   | Natija tayyor                             | Результат готов                                         |
// # |   3 | Poll::Pending                    | Hali tayyor emas — keyinroq tekshir       | Не готов — проверить позже                              |
// # |   4 | async fn → Future struct         | Compiler desugaring — state machine       | Компилятор: state machine                               |
// # |   5 | .await → poll() + suspend        | To'xtatish nuqtasi                        | Точка приостановки                                      |
// #================================================================================================================================================#
// # |                                        WAKER VA EXECUTOR                                                                                     |
// #================================================================================================================================================#
// # |   6 | Waker                            | Future ni qayta poll qilish signali       | Сигнал для повторного poll                              |
// # |   7 | Executor / Runtime               | Future larni boshqaruvchi                 | Управляющий Future                                      |
// # |   8 | block_on(future)                 | Future ni sinxron bajarish                | Синхронное выполнение Future                            |
// # |   9 | Context<'_>                      | poll() ga Waker berish                    | Передача Waker в poll()                                 |
// #================================================================================================================================================#
// # |                                        PIN                                                                                                   |
// #================================================================================================================================================#
// # |  10 | Pin<&mut T>                      | T ni xotirada qulflash                    | Запирает T в памяти                                     |
// # |  11 | Pin<Box<T>>                      | Heap da qullangan T                       | T запертый в куче                                       |
// # |  12 | PhantomPinned                    | !Unpin qilish                             | Сделать !Unpin                                          |
// # |  13 | Unpin                            | Ko'chirish xavfsiz (aksariyat turlar)      | Перемещение безопасно (большинство типов)              |
// # |  14 | Box::pin(future)                 | Future ni pin qilish                      | Pin Future                                              |
// #================================================================================================================================================#
// # |                                        REAL ISHLASH                                                                                          |
// #================================================================================================================================================#
// # |  15 | tokio / async-std                | Production executor/runtime                | Production executor/runtime                            |
// # |  16 | tokio::spawn                     | Async task yaratish                        | Создание async задачи                                  |
// # |  17 | tokio::join!                     | Ko'p Future parallel                       | Несколько Future параллельно                           |
// # |  18 | tokio::select!                   | Birinchi tayyor bo'lganini olish           | Взять первого готового                                 |
// # |  19 | Async I/O                        | Thread bloklashmasdan I/O kutish           | Ожидание I/O без блокировки потока                     |
// # |  20 | Green thread vs OS thread        | Async — green thread modeli                | Async — модель зелёных потоков                         |
// #================================================================================================================================================#