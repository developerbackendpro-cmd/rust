// #================================================================================================================================================#
// #                                                                   TOKIO CHUQUR                                                                 #
// #                        TOKIO — ASYNC RUST RUNTIME. TASK, SPAWN, JOIN, SELECT!, CHANNEL, TIMEOUT, SEMAPHORE, BROADCAST.                         #
// #                        TOKIO — ASYNC RUST РАНТАЙМ. TASK, SPAWN, JOIN, SELECT!, CHANNEL, TIMEOUT, SEMAPHORE, BROADCAST.                         #
// #================================================================================================================================================#

// Cargo.toml da qo'shish kerak:
// Нужно добавить в Cargo.toml:
//
// [dependencies]
// tokio = { version = "1", features = ["full"] }

// Bu faylda tokio konseptsiyalari tushuntiriladi va
// std::thread + mpsc bilan simulyatsiya qilinadi.
// В этом файле объясняются концепции tokio и
// симулируются через std::thread + mpsc.
//
// Haqiqiy tokio kodi comment sifatida ko'rsatiladi.
// Настоящий код tokio показан в виде комментариев.

#![allow(dead_code, unused)]

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;

// Tokio nima:
// Что такое Tokio:
//
//   - Async Rust uchun eng mashhur runtime
//   - Самый популярный runtime для Async Rust
//   - Multi-threaded work-stealing executor
//   - Многопоточный executor с work-stealing
//   - Async I/O, Timer, Channel, Sync primitives
//   - Async I/O, Timer, Channel, примитивы синхронизации
//
// Tokio arxitekturasi:
// Архитектура Tokio:
//   tokio::spawn    — yangi async task yaratish (green thread)
//   tokio::join!    — ko'p task parallel kutish
//   tokio::select!  — birinchi tayyor bo'lganini olish
//   tokio::time::*  — async timer, interval, timeout
//   tokio::sync::*  — Mutex, RwLock, Semaphore, Barrier
//   tokio::fs::*    — Async fayl I/O
//   tokio::net::*   — Async tarmoq I/O (TCP, UDP)
//   tokio::io::*    — Async I/O trait lari

// HAQIQIY TOKIO KODI:
// НАСТОЯЩИЙ КОД TOKIO:
//
// #[tokio::main]
// async fn main() {
//     let h1 = tokio::spawn(async {
//         println!("Task 1 boshlandi");
//         tokio::time::sleep(Duration::from_millis(100)).await;
//         println!("Task 1 tugadi");
//         42
//     });
//
//     let h2 = tokio::spawn(async {
//         println!("Task 2 boshlandi");
//         tokio::time::sleep(Duration::from_millis(50)).await;
//         println!("Task 2 tugadi");
//         "salom"
//     });
//
//     let (r1, r2) = tokio::join!(h1, h2);
//     println!("{:?} {:?}", r1, r2);
// }

// Simulyatsiya: thread pool bilan
// Симуляция: с пулом потоков
struct TokioSim {
    handlar: Vec<thread::JoinHandle<()>>,
}

impl TokioSim {
    fn new() -> Self { TokioSim { handlar: vec![] } }

    fn spawn<F>(&mut self, f: F) where F: FnOnce() + Send + 'static {
        self.handlar.push(thread::spawn(f));
    }

    fn join_all(self) {
        for h in self.handlar { h.join().unwrap(); }
    }
}

fn tokio_spawn_join_simulyatsiya() {

    println!("--- tokio::spawn simulyatsiyasi ---");
    let mut rt = TokioSim::new();
    let hisob = Arc::new(Mutex::new(0i32));

    for i in 0..4 {
        let h = Arc::clone(&hisob);
        rt.spawn(move || {
            thread::sleep(Duration::from_millis(10 * (4 - i)));
            let mut v = h.lock().unwrap();
            *v += i as i32;
            println!("[Task {}] qo'shildi: {}, jami: {}", i, i, *v);
        });
    }

    rt.join_all();
    println!("Jami: {}", hisob.lock().unwrap());
    // Task lar parallel ishlaydi
    // Jami: 6  (0+1+2+3)
}

// HAQIQIY TOKIO KODI:
// НАСТОЯЩИЙ КОД TOKIO:
//
// async fn select_misoli() {
//     let t1 = tokio::time::sleep(Duration::from_millis(100));
//     let t2 = tokio::time::sleep(Duration::from_millis(50));
//
//     tokio::select! {
//         _ = t1 => println!("t1 birinchi tugadi"),
//         _ = t2 => println!("t2 birinchi tugadi"),
//     }
//
//     // Channel bilan select
//     let (tx1, mut rx1) = tokio::sync::mpsc::channel::<i32>(1);
//     let (tx2, mut rx2) = tokio::sync::mpsc::channel::<String>(1);
//
//     tokio::spawn(async move { tx1.send(42).await.unwrap(); });
//     tokio::spawn(async move { tx2.send("salom".into()).await.unwrap(); });
//
//     tokio::select! {
//         v = rx1.recv() => println!("rx1: {:?}", v),
//         v = rx2.recv() => println!("rx2: {:?}", v),
//     }
// }

// Select simulyatsiyasi — birinchi kanaldan xabar olish
// Симуляция select — получение первого сообщения из канала
fn select_simulyatsiya() {

    println!("--- tokio::select! simulyatsiyasi ---");

    let (tx1, rx1) = std::sync::mpsc::channel::<i32>();
    let (tx2, rx2) = std::sync::mpsc::channel::<String>();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(30));
        tx1.send(42).unwrap();
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(10)); // tezroq
        tx2.send("salom".to_string()).unwrap();
    });

    // Birinchi xabar qabul qilish
    loop {
        if let Ok(v) = rx1.try_recv() {
            println!("rx1 birinchi: {}", v);
            break;
        }
        if let Ok(v) = rx2.try_recv() {
            println!("rx2 birinchi: {}", v);
            break;
        }
        thread::sleep(Duration::from_millis(1));
    }
    // rx2 birinchi: salom
}

// HAQIQIY TOKIO KODI:
// НАСТОЯЩИЙ КОД TOKIO:
//
// use tokio::time::{sleep, timeout, interval, Instant};
//
// async fn timeout_misoli() {
//     // Timeout — vaqt tugasa xato
//     let natija = timeout(Duration::from_millis(100), async {
//         sleep(Duration::from_millis(50)).await;
//         "vaqtida tugadi"
//     }).await;
//     println!("{:?}", natija); // Ok("vaqtida tugadi")
//
//     let timeout_natija = timeout(Duration::from_millis(10), async {
//         sleep(Duration::from_millis(100)).await;
//         "kech"
//     }).await;
//     println!("{:?}", timeout_natija); // Err(Elapsed)
//
//     // Interval — takroriy timer
//     let mut interval = interval(Duration::from_millis(100));
//     for _ in 0..3 {
//         interval.tick().await;
//         println!("Tick!");
//     }
// }

fn timeout_simulyatsiya() {

    println!("--- tokio::time::timeout simulyatsiyasi ---");

    // Timeout simulyatsiyasi
    fn timeout_fn<F, R>(f: F, limit: Duration) -> Option<R>
    where F: FnOnce() -> R + Send + 'static, R: Send + 'static {
        let (tx, rx) = std::sync::mpsc::channel();
        thread::spawn(move || { tx.send(f()).unwrap(); });
        rx.recv_timeout(limit).ok()
    }

    let natija = timeout_fn(|| {
        thread::sleep(Duration::from_millis(10));
        "vaqtida tugadi"
    }, Duration::from_millis(100));
    println!("Timeout natija: {:?}", natija);
    // Timeout natija: Some("vaqtida tugadi")

    let timeout_natija = timeout_fn(|| {
        thread::sleep(Duration::from_millis(100));
        "kech"
    }, Duration::from_millis(20));
    println!("Timeout natija: {:?}", timeout_natija);
    // Timeout natija: None

    // Interval simulyatsiyasi
    println!("Interval (3 ta tick):");
    for i in 0..3 {
        thread::sleep(Duration::from_millis(20));
        println!("  Tick {}", i + 1);
    }
}

// HAQIQIY TOKIO KODI:
// НАСТОЯЩИЙ КОД TOKIO:
//
// async fn sync_misollari() {
//
//     // Mutex — async, deadlock bo'lmaydi
//     let mutex = Arc::new(tokio::sync::Mutex::new(0));
//     let m = Arc::clone(&mutex);
//     tokio::spawn(async move {
//         let mut guard = m.lock().await;
//         *guard += 1;
//     }).await.unwrap();
//
//     // Semaphore — bir vaqtda N ta task
//     let sem = Arc::new(tokio::sync::Semaphore::new(3));
//     let permit = sem.acquire().await.unwrap();
//     // ... ish
//     drop(permit); // qaytarish
//
//     // Barrier — N task sinxronizatsiya
//     let barrier = Arc::new(tokio::sync::Barrier::new(4));
//     tokio::spawn(async move { barrier.wait().await; });
//
//     // RwLock — async ko'p o'quvchi
//     let rw = Arc::new(tokio::sync::RwLock::new(vec![1, 2, 3]));
//     let r = rw.read().await;
//     let mut w = rw.write().await;
// }

// Semaphore simulyatsiyasi
// Симуляция Semaphore
struct Semaphore {
    permit_soni: Mutex<usize>,
    cvar: std::sync::Condvar,
    limit: usize,
}

impl Semaphore {
    fn new(limit: usize) -> Self {
        Semaphore {
            permit_soni: Mutex::new(0),
            cvar: std::sync::Condvar::new(),
            limit,
        }
    }

    fn olish(&self) {
        let mut soni = self.permit_soni.lock().unwrap();
        while *soni >= self.limit {
            soni = self.cvar.wait(soni).unwrap();
        }
        *soni += 1;
    }

    fn qaytarish(&self) {
        *self.permit_soni.lock().unwrap() -= 1;
        self.cvar.notify_one();
    }
}

fn semaphore_simulyatsiya() {

    println!("--- Semaphore simulyatsiyasi (3 ta bir vaqtda) ---");
    let sem = Arc::new(Semaphore::new(3));
    let hisob = Arc::new(Mutex::new(0usize));
    let max_bir_vaqtda = Arc::new(Mutex::new(0usize));
    let mut handlar = vec![];

    for i in 0..8 {
        let s = Arc::clone(&sem);
        let h = Arc::clone(&hisob);
        let m = Arc::clone(&max_bir_vaqtda);
        handlar.push(thread::spawn(move || {
            s.olish();
            {
                let mut hozir = h.lock().unwrap();
                *hozir += 1;
                let mut maks = m.lock().unwrap();
                if *hozir > *maks { *maks = *hozir; }
                println!("[Task {}] Ishlayapti (bir vaqtda: {})", i, *hozir);
            }
            thread::sleep(Duration::from_millis(20));
            {
                *h.lock().unwrap() -= 1;
            }
            s.qaytarish();
        }));
    }
    for h in handlar { h.join().unwrap(); }
    println!("Maksimal bir vaqtda: {}", max_bir_vaqtda.lock().unwrap());
    // Har doim <= 3
}

// HAQIQIY TOKIO KODI:
// НАСТОЯЩИЙ КОД TOKIO:
//
// async fn channel_misollari() {
//
//     // mpsc — Multi-Producer Single-Consumer
//     let (tx, mut rx) = tokio::sync::mpsc::channel::<i32>(32);
//     let tx2 = tx.clone();
//     tokio::spawn(async move { tx.send(1).await.unwrap(); });
//     tokio::spawn(async move { tx2.send(2).await.unwrap(); });
//     while let Some(v) = rx.recv().await { println!("{}", v); }
//
//     // oneshot — bir marta javob
//     let (tx, rx) = tokio::sync::oneshot::channel::<String>();
//     tokio::spawn(async move { tx.send("natija".into()).unwrap(); });
//     println!("{}", rx.await.unwrap());
//
//     // broadcast — ko'p qabul qiluvchi
//     let (tx, mut rx1) = tokio::sync::broadcast::channel::<i32>(16);
//     let mut rx2 = tx.subscribe();
//     tx.send(42).unwrap();
//     println!("{}", rx1.recv().await.unwrap());
//     println!("{}", rx2.recv().await.unwrap());
//
//     // watch — eng so'nggi qiymat
//     let (tx, rx) = tokio::sync::watch::channel(0);
//     tx.send(42).unwrap();
//     println!("{}", *rx.borrow());
// }

// Broadcast simulyatsiyasi
// Симуляция Broadcast
fn broadcast_simulyatsiya() {

    println!("--- tokio::sync::broadcast simulyatsiyasi ---");

    let subscriber_soni = 3;
    let mut txlar: Vec<std::sync::mpsc::Sender<i32>> = vec![];
    let mut rxlar: Vec<std::sync::mpsc::Receiver<i32>> = vec![];

    for _ in 0..subscriber_soni {
        let (tx, rx) = std::sync::mpsc::channel();
        txlar.push(tx);
        rxlar.push(rx);
    }

    // Broadcast yuborish
    let xabarlar = vec![1, 2, 3];
    for &xabar in &xabarlar {
        for tx in &txlar { tx.send(xabar).unwrap(); }
    }
    drop(txlar);

    let mut handlar = vec![];
    for (i, rx) in rxlar.into_iter().enumerate() {
        handlar.push(thread::spawn(move || {
            let qabul: Vec<i32> = rx.iter().collect();
            println!("[Subscriber {}]: {:?}", i, qabul);
        }));
    }
    for h in handlar { h.join().unwrap(); }
    // [Subscriber 0]: [1, 2, 3]
    // [Subscriber 1]: [1, 2, 3]
    // [Subscriber 2]: [1, 2, 3]
}

// HAQIQIY TOKIO KODI:
// НАСТОЯЩИЙ КОД TOKIO:
//
// use tokio::net::{TcpListener, TcpStream};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
//
// async fn server() {
//     let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
//     loop {
//         let (mut socket, addr) = listener.accept().await.unwrap();
//         tokio::spawn(async move {
//             let mut buf = [0u8; 1024];
//             loop {
//                 let n = socket.read(&mut buf).await.unwrap();
//                 if n == 0 { break; }
//                 socket.write_all(&buf[..n]).await.unwrap(); // echo
//             }
//         });
//     }
// }
//
// async fn client() {
//     let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
//     stream.write_all(b"Salom server!").await.unwrap();
//     let mut buf = vec![0u8; 1024];
//     let n = stream.read(&mut buf).await.unwrap();
//     println!("{}", String::from_utf8_lossy(&buf[..n]));
// }

// TCP echo server simulyatsiyasi (std::net bilan)
// Симуляция TCP echo server (с std::net)
fn tcp_echo_simulyatsiya() {

    use std::net::{TcpListener, TcpStream};
    use std::io::{Read, Write};

    println!("--- TCP Echo Server simulyatsiyasi ---");

    let listener = TcpListener::bind("127.0.0.1:0").unwrap(); // 0 = tasodifiy port
    let addr = listener.local_addr().unwrap();
    println!("Server: {}", addr);

    // Server thread
    let server = thread::spawn(move || {
        if let Ok((mut stream, client_addr)) = listener.accept() {
            println!("Ulanish: {}", client_addr);
            let mut buf = [0u8; 1024];
            loop {
                match stream.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        println!("Server qabul qildi: {}", String::from_utf8_lossy(&buf[..n]));
                        stream.write_all(&buf[..n]).unwrap(); // echo
                    }
                }
            }
        }
    });

    thread::sleep(Duration::from_millis(10));

    // Client
    let mut client = TcpStream::connect(addr).unwrap();
    let xabarlar = ["Salom!", "Dunyo!", "Rust!"];
    for xabar in &xabarlar {
        client.write_all(xabar.as_bytes()).unwrap();
        let mut buf = [0u8; 1024];
        let n = client.read(&mut buf).unwrap();
        println!("Client echo: {}", String::from_utf8_lossy(&buf[..n]));
    }
    drop(client);
    server.join().unwrap();
}

// HAQIQIY TOKIO KODI:
// НАСТОЯЩИЙ КОД TOKIO:
//
// // 1. Graceful shutdown
// async fn graceful_shutdown() {
//     let (shutdown_tx, mut shutdown_rx) = tokio::sync::broadcast::channel::<()>(1);
//
//     let server_task = tokio::spawn(async move {
//         tokio::select! {
//             _ = tokio::signal::ctrl_c() => println!("Ctrl+C"),
//             _ = shutdown_rx.recv() => println!("Shutdown signal"),
//         }
//     });
//
//     tokio::time::sleep(Duration::from_secs(1)).await;
//     shutdown_tx.send(()).unwrap();
//     server_task.await.unwrap();
// }
//
// // 2. Rate limiting — Semaphore bilan
// async fn rate_limited_requests(urls: Vec<String>) {
//     let sem = Arc::new(tokio::sync::Semaphore::new(10)); // 10 ta bir vaqtda
//     let mut tasks = vec![];
//
//     for url in urls {
//         let sem = Arc::clone(&sem);
//         tasks.push(tokio::spawn(async move {
//             let _permit = sem.acquire().await.unwrap();
//             // HTTP so'rov
//             // permit drop bo'lganda sem qaytariladi
//         }));
//     }
//     tokio::join!(futures::future::join_all(tasks));
// }
//
// // 3. Retry bilan timeout
// async fn retry_with_timeout<F, R, E>(
//     f: impl Fn() -> F,
//     urinishlar: u32,
//     timeout_ms: u64,
// ) -> Result<R, E>
// where
//     F: Future<Output = Result<R, E>>,
// {
//     for i in 0..urinishlar {
//         let natija = tokio::time::timeout(
//             Duration::from_millis(timeout_ms),
//             f()
//         ).await;
//         match natija {
//             Ok(Ok(v)) => return Ok(v),
//             Ok(Err(e)) if i + 1 == urinishlar => return Err(e),
//             _ => tokio::time::sleep(Duration::from_millis(100 * 2u64.pow(i))).await,
//         }
//     }
//     unreachable!()
// }

// Retry bilan timeout simulyatsiyasi
// Симуляция retry с timeout
fn retry_timeout_simulyatsiya() {

    println!("--- Retry + Timeout simulyatsiyasi ---");

    let mut urinish_soni = 0;
    let max_urinish = 4;
    let boshlanish = Instant::now();

    let natija: Result<String, &str> = loop {
        urinish_soni += 1;

        // Timeout bilan "so'rov"
        let (tx, rx) = std::sync::mpsc::channel();
        thread::spawn(move || {
            // 3-urinishda muvaffaqiyat
            thread::sleep(Duration::from_millis(10));
            if urinish_soni >= 3 {
                tx.send(Ok("Muvaffaqiyat!".to_string())).unwrap();
            } else {
                tx.send(Err("Vaqtinchalik xato")).unwrap();
            }
        });

        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(Ok(v)) => break Ok(v),
            Ok(Err(e)) => {
                println!("[Urinish {}] Xato: {}", urinish_soni, e);
                if urinish_soni >= max_urinish { break Err(e); }
                // Exponential backoff
                thread::sleep(Duration::from_millis(5 * 2u64.pow(urinish_soni as u32)));
            }
            Err(_) => {
                println!("[Urinish {}] Timeout!", urinish_soni);
                if urinish_soni >= max_urinish { break Err("timeout"); }
            }
        }
    };

    println!("Natija: {:?} ({:.1?} da)", natija, boshlanish.elapsed());
    // [Urinish 1] Xato: Vaqtinchalik xato
    // [Urinish 2] Xato: Vaqtinchalik xato
    // Natija: Ok("Muvaffaqiyat!") (~XX ms da)
}

fn tokio_vs_thread_taqqoslash() {

    println!("--- Tokio vs std::thread taqqoslash ---");
    println!();
    println!("┌─────────────────┬────────────────────┬──────────────────────┐");
    println!("│ Xususiyat       │ std::thread        │ tokio::spawn         │");
    println!("├─────────────────┼────────────────────┼──────────────────────┤");
    println!("│ Model           │ OS thread (1:1)    │ Green thread (M:N)   │");
    println!("│ Xotira          │ ~8MB stack/thread  │ ~KB/task             │");
    println!("│ Yaratish vaqti  │ ~µs                │ ~ns                  │");
    println!("│ Max soni        │ ~1000-10000        │ Millionlar           │");
    println!("│ I/O kutish      │ Thread bloklanadi  │ Thread bloklanmaydi  │");
    println!("│ CPU intensive   │ Yaxshi             │ Thread pool kerak    │");
    println!("│ I/O intensive   │ Sekin (ko'p thread)│ Juda tez (kam thread)│");
    println!("│ Murakkablik     │ Oddiy              │ Async/await kerak    │");
    println!("└─────────────────┴────────────────────┴──────────────────────┘");
    println!();

    // I/O intensive vazifalar uchun tokio afzalligi
    let vazifa_soni = 100;
    let io_kutish_ms = 5;

    // Thread yondashuvi
    let thread_boshlanish = Instant::now();
    let mut thread_lar = vec![];
    for i in 0..vazifa_soni {
        thread_lar.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(io_kutish_ms));
            i
        }));
    }
    let _thread_natijalar: Vec<i32> = thread_lar.into_iter().map(|h| h.join().unwrap()).collect();
    let thread_vaqt = thread_boshlanish.elapsed();

    println!("{} I/O vazifa ({} ms har biri):", vazifa_soni, io_kutish_ms);
    println!("  std::thread: {:.1?}", thread_vaqt);
    println!("  tokio (nazariy): ~{}ms (bir nechta thread, parallel)", io_kutish_ms * 2);
    println!("  (Real tokio sozlangan thread pool bilan yanada tezroq)");
}

// Tokio bilan yozilgan server arxitekturasi
// Архитектура сервера написанного на Tokio
fn server_arxitektura_tushuntirish() {

    println!("--- Tokio Server Arxitekturasi ---");
    println!();
    println!("┌──────────────────────────────────────────────────────────┐");
    println!("│                  Tokio Runtime                           │");
    println!("│  ┌──────────┐  ┌──────────┐  ┌──────────┐                │");
    println!("│  │ Thread 1 │  │ Thread 2 │  │ Thread 3 │  ...           │");
    println!("│  │ (Worker) │  │ (Worker) │  │ (Worker) │                │");
    println!("│  └─────┬────┘  └─────┬────┘  └─────┬────┘                │");
    println!("│        │             │             │                     │");
    println!("│  ┌─────▼─────────────▼─────────────▼──────────────┐      │");
    println!("│  │          Task Queue (Steal-able)               │      │");
    println!("│  │  [Task1] [Task2] [Task3] [Task4] [Task5] ...   │      │");
    println!("│  └──────────────────────────────────────────────-─┘      │");
    println!("└──────────────────────────────────────────────────────────┘");
    println!();
    println!("Har Thread:");
    println!("  1. Task ni poll() qiladi");
    println!("  2. Poll::Pending bo'lsa — boshqa task ga o'tadi");
    println!("  3. I/O tayyor bo'lganda — Waker orqali xabar keladi");
    println!("  4. Task qayta navbatga qo'yiladi");
}

fn real_hayot_misollari() {

    println!("=== SPAWN VA JOIN ===");
    tokio_spawn_join_simulyatsiya();

    println!("\n=== SELECT! ===");
    select_simulyatsiya();

    println!("\n=== TIMEOUT ===");
    timeout_simulyatsiya();

    println!("\n=== SEMAPHORE ===");
    semaphore_simulyatsiya();

    println!("\n=== BROADCAST ===");
    broadcast_simulyatsiya();

    println!("\n=== TCP ECHO ===");
    tcp_echo_simulyatsiya();

    println!("\n=== RETRY + TIMEOUT ===");
    retry_timeout_simulyatsiya();

    println!("\n=== TOKIO vs THREAD ===");
    tokio_vs_thread_taqqoslash();

    println!("\n=== SERVER ARXITEKTURA ===");
    server_arxitektura_tushuntirish();
}
fn main() {
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                               | Описание (RU)                                            |
// #================================================================================================================================================#
// # |                                        TOKIO CORE                                                                                            |
// #================================================================================================================================================#
// # |   1 | #[tokio::main]                  | async main yaratish                        | Создание async main                                     |
// # |   2 | tokio::spawn(async { })         | Yangi async task yaratish                  | Создание новой async задачи                             |
// # |   3 | task.await                      | Task tugashini kutish                      | Ожидание завершения задачи                              |
// # |   4 | tokio::join!(t1, t2, t3)        | Ko'p task parallel kutish                  | Параллельное ожидание задач                             |
// # |   5 | tokio::select!                  | Birinchi tayyor bo'lganini olish           | Взять первого готового                                  |
// #================================================================================================================================================#
// # |                                        TOKIO::TIME                                                                                           |
// #================================================================================================================================================#
// # |   6 | tokio::time::sleep(dur)         | Async sleep                                | Async сон                                               |
// # |   7 | tokio::time::timeout(dur, f)    | Timeout bilan bajarish                     | Выполнение с таймаутом                                  |
// # |   8 | tokio::time::interval(dur)      | Takroriy timer                             | Повторяющийся таймер                                    |
// # |   9 | tokio::time::Instant::now()     | Async vaqt o'lchash                        | Измерение времени async                                 |
// #================================================================================================================================================#
// # |                                        TOKIO::SYNC                                                                                           |
// #================================================================================================================================================#
// # |  10 | tokio::sync::Mutex              | Async mutex (.lock().await)                | Async мьютекс (.lock().await)                           |
// # |  11 | tokio::sync::RwLock             | Async rwlock                               | Async rwlock                                            |
// # |  12 | tokio::sync::Semaphore          | N ta bir vaqtda cheklash                   | Ограничение N одновременно                              |
// # |  13 | tokio::sync::Barrier            | N task sinxronizatsiya                     | Синхронизация N задач                                   |
// #================================================================================================================================================#
// # |                                        TOKIO::CHANNEL                                                                                        |
// #================================================================================================================================================#
// # |  14 | tokio::sync::mpsc::channel(n)   | Async MPSC, n bufer                        | Async MPSC, буфер n                                     |
// # |  15 | tokio::sync::oneshot::channel() | Bir marta javob                            | Одноразовый ответ                                       |
// # |  16 | tokio::sync::broadcast::channel | Ko'p subscriber                            | Много подписчиков                                       |
// # |  17 | tokio::sync::watch::channel     | Eng so'nggi qiymat                         | Последнее значение                                      |
// #================================================================================================================================================#
// # |                                        TOKIO::NET                                                                                            |
// #================================================================================================================================================#
// # |  18 | tokio::net::TcpListener         | Async TCP server                           | Async TCP сервер                                        |
// # |  19 | tokio::net::TcpStream           | Async TCP ulanish                          | Async TCP соединение                                    |
// # |  20 | AsyncReadExt / AsyncWriteExt    | Async o'qish/yozish                        | Async чтение/запись                                     |
// #================================================================================================================================================#