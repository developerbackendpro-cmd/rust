// #================================================================================================================================================#
// #                                                             ASYNC-STREAM                                                                       #
// #                    ASYNC STREAM — ASYNC ITERATOR. STREAM TRAIT, PIN, POLL_NEXT. GENERATOR PATTERN. TOKIO-STREAM.                               #
// #                    ASYNC STREAM — ASYNC ИТЕРАТОР. ТРЕЙТ STREAM, PIN, POLL_NEXT. ПАТТЕРН ГЕНЕРАТОРА. TOKIO-STREAM.                              #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::VecDeque;

// Stream nima:
// Что такое Stream:
//
//   Iterator — sinxron, qiymatlar ketma-ket
//   Iterator — синхронный, значения последовательно
//   Stream   — asinkron, qiymatlar vaqt o'tib keladi
//   Stream   — асинхронный, значения поступают со временем
//
//   trait Stream {
//       type Item;
//       fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>)
//           -> Poll<Option<Self::Item>>;
//   }
//
//   Poll::Ready(Some(T)) — qiymat bor
//   Poll::Ready(None)    — stream tugadi
//   Poll::Pending        — hali tayyor emas
//
// Stream vs Iterator:
//   Iterator::next(&mut self) -> Option<T>       — blocking
//   Stream::poll_next(Pin<&mut Self>, cx) -> Poll<Option<T>> — non-blocking
//
// Tokio da:
// В Tokio:
//   tokio_stream::Stream — asosiy trait
//   tokio_stream::StreamExt — map, filter, next, collect...
//   async_stream::stream! — generator makrosi

// Stream trait — std da yo'q, shuning uchun o'zimiz implement qilamiz
// Stream trait — нет в std, поэтому реализуем сами
trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

// StreamExt — qulay metodlar
// StreamExt — удобные методы
trait StreamExt: Stream + Sized {
    fn next(&mut self) -> NextFuture<'_, Self> {
        NextFuture { stream: self }
    }

    fn collect_vec(mut self) -> Vec<Self::Item>
    where
        Self: Unpin,
    {
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        let mut natijalar = Vec::new();
        loop {
            match Pin::new(&mut self).poll_next(&mut cx) {
                Poll::Ready(Some(v)) => natijalar.push(v),
                Poll::Ready(None)    => break,
                Poll::Pending        => {
                    thread::yield_now();
                }
            }
        }
        natijalar
    }
}

impl<S: Stream + Sized> StreamExt for S {}

// NextFuture — stream dan bitta element olish
// NextFuture — получение одного элемента из stream
struct NextFuture<'a, S: Stream> {
    stream: &'a mut S,
}

impl<'a, S: Stream + Unpin> Future for NextFuture<'a, S> {
    type Output = Option<S::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        Pin::new(&mut *self.stream).poll_next(cx)
    }
}

// Noop waker
fn noop_waker() -> std::task::Waker {
    use std::task::{RawWaker, RawWakerVTable, Waker};
    unsafe fn clone(_: *const ()) -> RawWaker { RawWaker::new(std::ptr::null(), &VTABLE) }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn noop_drop(_: *const ()) {}
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, noop_drop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

fn block_on_stream<S: Stream + Unpin>(mut stream: S) -> Vec<S::Item> {
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    let mut natijalar = Vec::new();
    loop {
        match Pin::new(&mut stream).poll_next(&mut cx) {
            Poll::Ready(Some(v)) => natijalar.push(v),
            Poll::Ready(None)    => break,
            Poll::Pending        => thread::yield_now(),
        }
    }
    natijalar
}

// 1. RangeStream — sonlar oralig'i
// 1. RangeStream — диапазон чисел
struct RangeStream {
    joriy: i32,
    max: i32,
}

impl RangeStream {
    fn new(boshlanish: i32, max: i32) -> Self {
        RangeStream { joriy: boshlanish, max }
    }
}

impl Stream for RangeStream {
    type Item = i32;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<i32>> {
        if self.joriy >= self.max {
            Poll::Ready(None)
        } else {
            let v = self.joriy;
            self.joriy += 1;
            Poll::Ready(Some(v))
        }
    }
}

// 2. FibStream — Fibonacci ketma-ketligi
// 2. FibStream — последовательность Фибоначчи
struct FibStream {
    a: u64,
    b: u64,
    qolgan: usize,
}

impl FibStream {
    fn new(n: usize) -> Self {
        FibStream { a: 0, b: 1, qolgan: n }
    }
}

impl Stream for FibStream {
    type Item = u64;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<u64>> {
        if self.qolgan == 0 {
            return Poll::Ready(None);
        }
        let natija = self.a;
        (self.a, self.b) = (self.b, self.a + self.b);
        self.qolgan -= 1;
        Poll::Ready(Some(natija))
    }
}

// 3. MapStream — transformatsiya
// 3. MapStream — трансформация
struct MapStream<S: Stream, F> {
    ichki: S,
    funksiya: F,
}

impl<S: Stream + Unpin, F, B> Stream for MapStream<S, F>
where
    F: FnMut(S::Item) -> B,
    Self: Unpin,
{
    type Item = B;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<B>> {
        let this = self.get_mut();
        match Pin::new(&mut this.ichki).poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some((this.funksiya)(v))),
            Poll::Ready(None)    => Poll::Ready(None),
            Poll::Pending        => Poll::Pending,
        }
    }
}

// 4. FilterStream — filtrlash
// 4. FilterStream — фильтрация
struct FilterStream<S: Stream, P> {
    ichki: S,
    predikat: P,
}

impl<S: Stream + Unpin, P> Stream for FilterStream<S, P>
where
        for<'a> P: FnMut(&'a S::Item) -> bool,
        Self: Unpin,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        let this = self.get_mut();
        loop {
            match Pin::new(&mut this.ichki).poll_next(cx) {
                Poll::Ready(Some(v)) => {
                    if (this.predikat)(&v) {
                        return Poll::Ready(Some(v));
                    }
                }
                other => return other,
            }
        }
    }
}

// 5. ChainStream — ikki streamni birlashtirish
// 5. ChainStream — объединение двух потоков
struct ChainStream<A: Stream, B: Stream<Item = A::Item>> {
    birinchi: A,
    ikkinchi: B,
    birinchi_tugadi: bool,
}

impl<A: Stream + Unpin, B: Stream<Item = A::Item> + Unpin> Stream for ChainStream<A, B> {
    type Item = A::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<A::Item>> {
        let this = self.get_mut();
        if !this.birinchi_tugadi {
            match Pin::new(&mut this.birinchi).poll_next(cx) {
                Poll::Ready(None)    => this.birinchi_tugadi = true,
                other                => return other,
            }
        }
        Pin::new(&mut this.ikkinchi).poll_next(cx)
    }
}

// 6. TakeStream — N ta element olish
// 6. TakeStream — взять N элементов
struct TakeStream<S: Stream> {
    ichki: S,
    qolgan: usize,
}

impl<S: Stream + Unpin> Stream for TakeStream<S> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        let this = self.get_mut();
        if this.qolgan == 0 {
            return Poll::Ready(None);
        }
        match Pin::new(&mut this.ichki).poll_next(cx) {
            Poll::Ready(Some(v)) => {
                this.qolgan -= 1;
                Poll::Ready(Some(v))
            }
            other => other,
        }
    }
}

// StreamBuilder — qulay yaratish
// StreamBuilder — удобное создание
struct StreamBuilder;

impl StreamBuilder {
    fn diapazon(boshlanish: i32, max: i32) -> RangeStream {
        RangeStream::new(boshlanish, max)
    }

    fn fibonacci(n: usize) -> FibStream {
        FibStream::new(n)
    }

    fn iter<I: Iterator>(iter: I) -> IterStream<I> {
        IterStream { iter }
    }
}

// IterStream — Iterator → Stream
// IterStream — Iterator → Stream
struct IterStream<I: Iterator> {
    iter: I,
}

impl<I: Iterator + Unpin> Stream for IterStream<I> {
    type Item = I::Item;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<I::Item>> {
        Poll::Ready(self.get_mut().iter.next())
    }
}

fn custom_stream_misollari() {

    // RangeStream
    let range = RangeStream::new(0, 5);
    let v = block_on_stream(range);
    println!("{:?}", v);
    // [0, 1, 2, 3, 4]

    // FibStream
    let fib = FibStream::new(8);
    let v2 = block_on_stream(fib);
    println!("{:?}", v2);
    // [0, 1, 1, 2, 3, 5, 8, 13]

    // MapStream
    let mapped = MapStream {
        ichki: RangeStream::new(1, 6),
        funksiya: |x| x * x,
    };
    let v3 = block_on_stream(mapped);
    println!("{:?}", v3);
    // [1, 4, 9, 16, 25]

    // FilterStream
    fn juft_tekshir(x: &i32) -> bool { x % 2 == 0 }
    let filtered = FilterStream {
        ichki: RangeStream::new(0, 10),
        predikat: juft_tekshir as fn(&i32) -> bool,
    };
    let v4 = block_on_stream(filtered);
    println!("{:?}", v4);
    // [0, 2, 4, 6, 8]

    // ChainStream
    let chained = ChainStream {
        birinchi: RangeStream::new(0, 3),
        ikkinchi: RangeStream::new(10, 13),
        birinchi_tugadi: false,
    };
    let v5 = block_on_stream(chained);
    println!("{:?}", v5);
    // [0, 1, 2, 10, 11, 12]

    // TakeStream — cheksiz streamdan N ta
    // TakeStream — N элементов из бесконечного
    let take = TakeStream {
        ichki: FibStream { a: 0, b: 1, qolgan: usize::MAX },
        qolgan: 6,
    };
    let v6 = block_on_stream(take);
    println!("{:?}", v6);
    // [0, 1, 1, 2, 3, 5]

    // IterStream — Iterator → Stream
    let iter_s = StreamBuilder::iter(vec!["salom", "dunyo", "rust"].into_iter());
    let v7 = block_on_stream(iter_s);
    println!("{:?}", v7);
    // ["salom", "dunyo", "rust"]
}

// Kanal orqali stream — amaliy ishlatish
// Stream через канал — практическое использование
struct ChannelStream<T> {
    rx: mpsc::Receiver<T>,
}

impl<T> ChannelStream<T> {
    fn new() -> (mpsc::Sender<T>, Self) {
        let (tx, rx) = mpsc::channel();
        (tx, ChannelStream { rx })
    }
}

impl<T: Send + 'static> Stream for ChannelStream<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>> {
        match self.rx.try_recv() {
            Ok(v)                               => Poll::Ready(Some(v)),
            Err(mpsc::TryRecvError::Empty)      => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            Err(mpsc::TryRecvError::Disconnected) => Poll::Ready(None),
        }
    }
}

fn channel_stream_misoli() {

    println!("--- Channel Stream ---");
    let (tx, mut stream) = ChannelStream::<i32>::new();

    // Producer thread
    let producer = thread::spawn(move || {
        for i in 0..5 {
            tx.send(i * 10).unwrap();
            thread::sleep(Duration::from_millis(5));
        }
        // tx drop — stream tugaydi
    });

    // Stream ni o'qish
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    let mut natijalar = vec![];

    // Kichik kutish — producer ishlasin
    thread::sleep(Duration::from_millis(50));
    producer.join().unwrap();

    loop {
        match Pin::new(&mut stream).poll_next(&mut cx) {
            Poll::Ready(Some(v)) => natijalar.push(v),
            Poll::Ready(None)    => break,
            Poll::Pending        => break,
        }
    }
    println!("{:?}", natijalar);
    // [0, 10, 20, 30, 40]
}

// HAQIQIY async_stream + tokio KODI:
// НАСТОЯЩИЙ КОД async_stream + tokio:
//
// Cargo.toml:
//   [dependencies]
//   tokio = { version = "1", features = ["full"] }
//   tokio-stream = "0.1"
//   async-stream = "0.3"
//
// // stream! makrosi — generator pattern
// // Макрос stream! — паттерн генератора
// use async_stream::stream;
// use tokio_stream::StreamExt;
//
// fn sonlar_stream() -> impl Stream<Item = i32> {
//     stream! {
//         for i in 0..5 {
//             tokio::time::sleep(Duration::from_millis(100)).await;
//             yield i;  // ← yield bilan qiymat berish
//         }
//     }
// }
//
// #[tokio::main]
// async fn main() {
//     let mut s = sonlar_stream();
//     while let Some(v) = s.next().await {
//         println!("{}", v);
//     }
// }
//
// // try_stream! — Result bilan
// fn xatoli_stream() -> impl Stream<Item = Result<i32, String>> {
//     async_stream::try_stream! {
//         for i in 0..5 {
//             if i == 3 { Err("xato!")? }
//             yield i;
//         }
//     }
// }
//
// // StreamExt metodlari:
// // Методы StreamExt:
// s.map(|x| x * 2)
// s.filter(|x| *x > 2)
// s.take(3)
// s.skip(2)
// s.zip(other_stream)
// s.chain(other_stream)
// s.enumerate()
// s.fold(0, |acc, x| async move { acc + x }).await
// s.collect::<Vec<_>>().await
// s.for_each(|x| async move { println!("{}", x) }).await
// s.timeout(Duration::from_secs(1))
// s.chunks(3)
// s.buffer_unordered(10)  // parallel processing

// Generator state machine — yield ni simulyatsiya qilish
// Машина состояний генератора — симуляция yield
enum GeneratorHolat<T> {
    Davom,
    Qiymat(T),
    Tugadi,
}

struct Generator<T, F: FnMut() -> GeneratorHolat<T>> {
    funksiya: F,
}

impl<T, F: FnMut() -> GeneratorHolat<T>> Generator<T, F> {
    fn new(f: F) -> Self { Generator { funksiya: f } }
}

impl<T, F: FnMut() -> GeneratorHolat<T> + Unpin> Stream for Generator<T, F> {
    type Item = T;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<T>> {
        match (self.funksiya)() {
            GeneratorHolat::Qiymat(v) => Poll::Ready(Some(v)),
            GeneratorHolat::Tugadi    => Poll::Ready(None),
            GeneratorHolat::Davom     => Poll::Pending,
        }
    }
}

// macro_rules! bilan stream! simulyatsiyasi
// Симуляция stream! с macro_rules!
macro_rules! stream_gen {
    ($($x:expr),+ $(,)?) => {
        {
            let qiymatlar: Vec<_> = vec![$($x),+];
            IterStream { iter: qiymatlar.into_iter() }
        }
    };
}

fn generator_pattern_misoli() {

    println!("--- Generator Pattern ---");

    // Fibonacci generator
    let mut a = 0u64;
    let mut b = 1u64;
    let mut hisob = 0;

    let fib_gen = Generator::new(move || {
        if hisob >= 8 {
            return GeneratorHolat::Tugadi;
        }
        let natija = a;
        (a, b) = (b, a + b);
        hisob += 1;
        GeneratorHolat::Qiymat(natija)
    });

    let v = block_on_stream(fib_gen);
    println!("Fibonacci: {:?}", v);
    // Fibonacci: [0, 1, 1, 2, 3, 5, 8, 13]

    // stream_gen! makrosi
    let s = stream_gen![1, 4, 9, 16, 25];
    let v2 = block_on_stream(s);
    println!("stream_gen!: {:?}", v2);
    // stream_gen!: [1, 4, 9, 16, 25]
}

// ZipStream — ikki stream ni juftlashtirish
// ZipStream — объединение двух потоков в пары
struct ZipStream<A: Stream, B: Stream> {
    a: A,
    b: B,
    a_buf: Option<A::Item>,
}

impl<A: Stream + Unpin, B: Stream + Unpin> Stream for ZipStream<A, B> where A::Item: Unpin {
    type Item = (A::Item, B::Item);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<(A::Item, B::Item)>> {
        let this = self.get_mut();
        if this.a_buf.is_none() {
            match Pin::new(&mut this.a).poll_next(cx) {
                Poll::Ready(Some(v)) => this.a_buf = Some(v),
                Poll::Ready(None)    => return Poll::Ready(None),
                Poll::Pending        => return Poll::Pending,
            }
        }

        match Pin::new(&mut this.b).poll_next(cx) {
            Poll::Ready(Some(b_val)) => {
                let a_val = this.a_buf.take().unwrap();
                Poll::Ready(Some((a_val, b_val)))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending     => Poll::Pending,
        }
    }
}

// ScanStream — holat bilan transformatsiya
// ScanStream — трансформация с состоянием
struct ScanStream<S: Stream, St, F> {
    ichki: S,
    holat: St,
    funksiya: F,
}

impl<S: Stream + Unpin, St: Unpin, F, B> Stream for ScanStream<S, St, F>
where
    F: FnMut(&mut St, S::Item) -> Option<B> + Unpin,
{
    type Item = B;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<B>> {
        let this = self.get_mut();
        match Pin::new(&mut this.ichki).poll_next(cx) {
            Poll::Ready(Some(v)) => {
                match (this.funksiya)(&mut this.holat, v) {
                    Some(b) => Poll::Ready(Some(b)),
                    None    => Poll::Ready(None),
                }
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending     => Poll::Pending,
        }
    }
}

fn stream_kombinatorlar_misollari() {

    println!("--- Stream Kombinatorlar ---");

    // ZipStream
    let zip = ZipStream {
        a: RangeStream::new(1, 5),
        b: StreamBuilder::iter(["a", "b", "c", "d"].into_iter()),
        a_buf: None,
    };
    let v = block_on_stream(zip);
    println!("Zip: {:?}", v);
    // Zip: [(1, "a"), (2, "b"), (3, "c"), (4, "d")]

    // ScanStream — kumulyativ yig'indi
    // ScanStream — кумулятивная сумма
    let scan = ScanStream {
        ichki: RangeStream::new(1, 6),
        holat: 0i32,
        funksiya: |holat: &mut i32, x: i32| -> Option<i32> { *holat += x; Some(*holat) },
    };
    let v2 = block_on_stream(scan);
    println!("Scan (kumulyativ): {:?}", v2);
    // Scan (kumulyativ): [1, 3, 6, 10, 15]

    // Map + Filter zanjiri
    fn juft_tekshir2(x: &i32) -> bool { x % 2 == 0 }
    let pipeline = FilterStream {
        ichki: MapStream {
            ichki: RangeStream::new(0, 10),
            funksiya: |x: i32| x * x,
        },
        predikat: juft_tekshir2 as fn(&i32) -> bool,
    };
    let v3 = block_on_stream(pipeline);
    println!("Map+Filter: {:?}", v3);
    // Map+Filter: [0, 4, 16, 36, 64]
}

// Event stream — voqealar oqimi
// Event stream — поток событий
#[derive(Debug, Clone)]
enum Voqea {
    FoydalanuvchiKirdi { id: u32 },
    SorovYuborildi { url: String },
    XatoYuzBerdi { xabar: String },
    Tugadi,
}

struct VoqeaStream {
    voqealar: VecDeque<Voqea>,
}

impl VoqeaStream {
    fn new(voqealar: Vec<Voqea>) -> Self {
        VoqeaStream { voqealar: voqealar.into_iter().collect() }
    }
}

impl Stream for VoqeaStream {
    type Item = Voqea;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Voqea>> {
        Poll::Ready(self.voqealar.pop_front())
    }
}

// Metrics stream — monitoring
// Metrics stream — мониторинг
struct MetricsStream {
    hisob: u32,
    max: u32,
}

impl MetricsStream {
    fn new(max: u32) -> Self { MetricsStream { hisob: 0, max } }
}

#[derive(Debug)]
struct Metric {
    vaqt_ms: u64,
    cpu_prosent: f32,
    xotira_mb: f32,
    so_rovlar: u32,
}

impl Stream for MetricsStream {
    type Item = Metric;

    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Metric>> {
        if self.hisob >= self.max {
            return Poll::Ready(None);
        }
        let i = self.hisob;
        self.hisob += 1;

        // Simulyatsiya qilingan metrik
        let metric = Metric {
            vaqt_ms: i as u64 * 1000,
            cpu_prosent: 20.0 + (i as f32 * 7.3) % 60.0,
            xotira_mb: 256.0 + (i as f32 * 12.5) % 512.0,
            so_rovlar: i * 15 + 100,
        };
        Poll::Ready(Some(metric))
    }
}

fn real_hayot_misollari() {

    println!("--- Event Stream ---");
    let voqealar = vec![
        Voqea::FoydalanuvchiKirdi { id: 1 },
        Voqea::SorovYuborildi { url: "/api/users".to_string() },
        Voqea::FoydalanuvchiKirdi { id: 2 },
        Voqea::XatoYuzBerdi { xabar: "DB ulanish xatosi".to_string() },
        Voqea::SorovYuborildi { url: "/api/data".to_string() },
    ];

    let stream = VoqeaStream::new(voqealar);
    let events = block_on_stream(stream);

    let mut xatolar = 0;
    let mut sorovlar = 0;
    let mut kirganlar = 0;

    for v in &events {
        match v {
            Voqea::FoydalanuvchiKirdi { id } => {
                kirganlar += 1;
                println!("  👤 Foydalanuvchi {} kirdi", id);
            }
            Voqea::SorovYuborildi { url } => {
                sorovlar += 1;
                println!("  📡 So'rov: {}", url);
            }
            Voqea::XatoYuzBerdi { xabar } => {
                xatolar += 1;
                println!("  ❌ Xato: {}", xabar);
            }
            Voqea::Tugadi => break,
        }
    }
    println!("Statistika: kirgan={}, sorov={}, xato={}", kirganlar, sorovlar, xatolar);

    println!("\n--- Metrics Stream ---");
    let metrics = MetricsStream::new(5);
    let all_metrics = block_on_stream(metrics);

    println!("{:<10} {:<12} {:<12} {:<10}", "Vaqt(ms)", "CPU(%)", "Xotira(MB)", "So'rovlar");
    println!("{}", "-".repeat(48));
    for m in &all_metrics {
        println!("{:<10} {:<12.1} {:<12.1} {:<10}", m.vaqt_ms, m.cpu_prosent, m.xotira_mb, m.so_rovlar);
    }

    // O'rtacha CPU
    let ortacha_cpu: f32 = all_metrics.iter().map(|m| m.cpu_prosent).sum::<f32>() / all_metrics.len() as f32;
    println!("O'rtacha CPU: {:.1}%", ortacha_cpu);

    println!("\n--- Channel Stream ---");
    channel_stream_misoli();

    println!("\n--- Generator Pattern ---");
    generator_pattern_misoli();

    println!("\n--- Stream Kombinatorlar ---");
    stream_kombinatorlar_misollari();
}

fn main() {
    println!("=== CUSTOM STREAM ===");
    custom_stream_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        STREAM TRAIT                                                                                          |
// #================================================================================================================================================#
// # |   1 | trait Stream { poll_next() }    | Async iterator                             | Асинхронный итератор                                    |
// # |   2 | Poll::Ready(Some(T))            | Qiymat bor                                 | Значение есть                                           |
// # |   3 | Poll::Ready(None)               | Stream tugadi                              | Поток завершён                                          |
// # |   4 | Poll::Pending                   | Hali tayyor emas                           | Ещё не готов                                            |
// # |   5 | Pin<&mut Self>                  | Self-referential struct uchun              | Для self-referential структуры                          |
// #================================================================================================================================================#
// # |                                        CUSTOM STREAMLAR                                                                                      |
// #================================================================================================================================================#
// # |   6 | RangeStream                     | Sonlar oralig'i                            | Диапазон чисел                                          |
// # |   7 | FibStream                       | Fibonacci ketma-ketligi                    | Последовательность Фибоначчи                            |
// # |   8 | MapStream                       | Transformatsiya                            | Трансформация                                           |
// # |   9 | FilterStream                    | Filtrlash                                  | Фильтрация                                              |
// # |  10 | ChainStream                     | Ikki streamni birlashtirish                | Объединение двух потоков                                |
// # |  11 | TakeStream                      | N ta element olish                         | Взять N элементов                                       |
// # |  12 | ZipStream                       | Juftlashtirish                             | Объединение в пары                                      |
// # |  13 | ScanStream                      | Holat bilan transformatsiya                | Трансформация с состоянием                              |
// # |  14 | IterStream                      | Iterator → Stream                          | Iterator → Stream                                       |
// # |  15 | ChannelStream                   | mpsc kanal → Stream                        | mpsc канал → Stream                                     |
// #================================================================================================================================================#
// # |                                        ASYNC-STREAM KUTUBXONA                                                                                |
// #================================================================================================================================================#
// # |  16 | async_stream::stream! { yield } | Generator makrosi                          | Макрос генератора                                       |
// # |  17 | async_stream::try_stream!       | Result bilan generator                     | Генератор с Result                                      |
// # |  18 | tokio_stream::StreamExt         | map, filter, take, zip, collect, ...       | map, filter, take, zip, collect, ...                    |
// # |  19 | .buffer_unordered(n)            | N ta parallel hisoblash                    | N параллельных вычислений                               |
// # |  20 | .timeout(dur)                   | Timeout bilan stream                       | Поток с таймаутом                                       |
// #================================================================================================================================================#