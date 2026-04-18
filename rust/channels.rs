// #================================================================================================================================================#
// #                                                                CHANNELS                                                                        #
// #                        KANALLAR — MPSC, BROADCAST, ONESHOT, RENDEZVOUS. MESSAGE PASSING. PIPELINE. ACTOR PATTERN.                              #
// #                        КАНАЛЫ — MPSC, BROADCAST, ONESHOT, RENDEZVOUS. ПЕРЕДАЧА СООБЩЕНИЙ. PIPELINE. ПАТТЕРН ACTOR.                             #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::sync::mpsc::{self, Sender, Receiver, SyncSender, TrySendError, TryRecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;

// Kanallar nima:
// Что такое каналы:
//
//   std::sync::mpsc — Multi-Producer, Single-Consumer
//   std::sync::mpsc — Несколько производителей, один потребитель
//
//   channel()      — cheksiz bufer (async)
//   channel()      — безграничный буфер (async)
//   sync_channel(n)— n o'lchamli bufer (sync, blocking)
//   sync_channel(n)— буфер размером n (sync, блокирующий)
//
//   Xabar uzatish falsafasi:
//   Философия передачи сообщений:
//   "Do not communicate by sharing memory;
//    instead, share memory by communicating"
//    — Go proverb (Rust ham shu tamoyilni qo'llaydi)

fn mpsc_asosiy_misollari() {

    // channel() — cheksiz, non-blocking send
    // channel() — безграничный, неблокирующий send
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let tx2 = tx.clone(); // Sender clone qilish mumkin (Multi-Producer)

    let h1 = thread::spawn(move || {
        for i in 0..5 {
            tx.send(i).unwrap();
            println!("[P1] yuborildi: {}", i);
        }
    });

    let h2 = thread::spawn(move || {
        for i in 10..15 {
            tx2.send(i).unwrap();
            println!("[P2] yuborildi: {}", i);
        }
    });

    h1.join().unwrap();
    h2.join().unwrap();

    // recv() — blocking, xabar kelguncha kutadi
    // recv() — блокирующий, ждёт сообщение
    let mut barcha = vec![];
    while let Ok(v) = rx.try_recv() {
        barcha.push(v);
    }
    barcha.sort();
    println!("Qabul qilindi: {:?}", barcha);
    // Qabul qilindi: [0, 1, 2, 3, 4, 10, 11, 12, 13, 14]
}

fn sync_channel_misollari() {

    // sync_channel(2) — 2 ta bufer, send bloklanadi
    // sync_channel(2) — буфер 2, send блокируется
    let (tx, rx): (SyncSender<String>, Receiver<String>) = mpsc::sync_channel(2);

    let producer = thread::spawn(move || {
        for i in 0..5 {
            let xabar = format!("xabar-{}", i);
            println!("[P] yuborilmoqda: {}", xabar);
            tx.send(xabar).unwrap(); // 3-dan keyin bloklanadi
            println!("[P] yuborildi: xabar-{}", i);
        }
    });

    thread::sleep(Duration::from_millis(30));

    let consumer = thread::spawn(move || {
        for _ in 0..5 {
            thread::sleep(Duration::from_millis(10));
            if let Ok(v) = rx.recv() {
                println!("[C] qabul qilindi: {}", v);
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    // try_send() — bloklanmasdan yuborish
    // try_send() — отправка без блокировки
    let (tx2, rx2) = mpsc::sync_channel(1);
    tx2.send("birinchi").unwrap();
    match tx2.try_send("ikkinchi") {
        Ok(())  => println!("Yuborildi"),
        Err(TrySendError::Full(v))  => println!("To'la: {}", v),
        Err(TrySendError::Disconnected(v)) => println!("Uzildi: {}", v),
    }
    println!("rx2: {:?}", rx2.recv());
    // To'la: ikkinchi
    // rx2: Ok("birinchi")
}

fn try_recv_misollari() {

    let (tx, rx) = mpsc::channel::<i32>();

    // try_recv() — bloklanmasdan urinish
    // try_recv() — попытка без блокировки
    match rx.try_recv() {
        Ok(v)                        => println!("Qabul: {}", v),
        Err(TryRecvError::Empty)     => println!("Bo'sh — xabar yo'q"),
        Err(TryRecvError::Disconnected) => println!("Uzildi"),
    }
    // Bo'sh — xabar yo'q

    tx.send(42).unwrap();
    println!("{:?}", rx.try_recv()); // Ok(42)

    // recv_timeout() — timeout bilan kutish
    // recv_timeout() — ожидание с таймаутом
    match rx.recv_timeout(Duration::from_millis(10)) {
        Ok(v)  => println!("Qabul: {}", v),
        Err(e) => println!("Timeout yoki uzildi: {}", e),
    }
    // Timeout yoki uzildi: channel is empty and sending half is closed

    // iter() — kanal yopilguncha iteratsiya
    // iter() — итерация пока канал не закрыт
    let (tx2, rx2) = mpsc::channel::<i32>();
    for i in 0..5 { tx2.send(i).unwrap(); }
    drop(tx2); // kanal yopiladi

    let yig: i32 = rx2.iter().sum(); // yopilguncha o'qiydi
    println!("Yig'indi: {}", yig);
    // Yig'indi: 10
}

// Pipeline: ma'lumot bir kanaldan ikkinchisiga o'tadi
// Pipeline: данные проходят через цепочку каналов
fn pipeline_misoli() {

    // 1-bosqich → 2-bosqich → 3-bosqich
    // Stage1 → Stage2 → Stage3

    let (tx1, rx1) = mpsc::channel::<i32>();
    let (tx2, rx2) = mpsc::channel::<i32>();
    let (tx3, rx3) = mpsc::channel::<String>();

    // Stage 1: har sonni ikkilantirish
    // Stage 1: удвоить каждое число
    let bosqich1 = thread::spawn(move || {
        for v in rx1.iter() {
            tx2.send(v * 2).unwrap();
        }
    });

    // Stage 2: manfiy raqamlarni filtrash
    // Stage 2: фильтрация отрицательных чисел
    let bosqich2 = thread::spawn(move || {
        for v in rx2.iter() {
            if v > 0 {
                tx3.send(format!("[{}]", v)).unwrap();
            }
        }
    });

    // Kirish ma'lumotlar
    for i in [-3, -1, 0, 2, 4, 6, -5, 8] {
        tx1.send(i).unwrap();
    }
    drop(tx1);

    bosqich1.join().unwrap();
    bosqich2.join().unwrap();

    // Natijalarni yig'ish
    let natijalar: Vec<String> = rx3.iter().collect();
    println!("{:?}", natijalar);
    // ["[4]", "[8]", "[12]", "[16]"]
    // (-3*2=-6 < 0 ✗, -1*2=-2 < 0 ✗, 0*2=0 ✗, 2*2=4 ✓, ...)
}

// Oneshot simulyatsiyasi — channel(0) yoki sync_channel(0)
// Симуляция Oneshot — channel(0) или sync_channel(0)
struct Oneshot<T> {
    tx: Option<Sender<T>>,
    rx: Receiver<T>,
}

impl<T: Send + 'static> Oneshot<T> {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Oneshot { tx: Some(tx), rx }
    }

    fn yuboruvchi(&mut self) -> OneshotSender<T> {
        OneshotSender { tx: self.tx.take().unwrap() }
    }

    fn kutish(self) -> T {
        self.rx.recv().unwrap()
    }
}

struct OneshotSender<T> {
    tx: Sender<T>,
}

impl<T> OneshotSender<T> {
    fn yuborish(self, qiymat: T) {
        self.tx.send(qiymat).unwrap();
        // self drop — kanal yopiladi
    }
}

fn oneshot_misoli() {

    let mut oneshot: Oneshot<String> = Oneshot::new();
    let yuboruvchi = oneshot.yuboruvchi();

    let ish = thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        println!("[Worker] Ish tugadi, javob yuborilmoqda");
        yuboruvchi.yuborish(String::from("Hisoblash natijasi: 42"));
    });

    println!("[Main] Javob kutilmoqda...");
    let javob = oneshot.kutish();
    println!("[Main] Javob: {}", javob);
    ish.join().unwrap();
    // [Main] Javob kutilmoqda...
    // [Worker] Ish tugadi, javob yuborilmoqda
    // [Main] Javob: Hisoblash natijasi: 42
}

// std::sync::mpsc — faqat bitta consumer
// Broadcast uchun: Arc<Mutex<Vec<Sender>>> pattern
// std::sync::mpsc — только один consumer
// Для broadcast: паттерн Arc<Mutex<Vec<Sender>>>

struct BroadcastKanal<T: Clone> {
    qabul_qiluvchilar: Mutex<Vec<Sender<T>>>,
}

impl<T: Clone + Send + 'static> BroadcastKanal<T> {
    fn new() -> Self {
        BroadcastKanal { qabul_qiluvchilar: Mutex::new(vec![]) }
    }

    fn obuna_bo_l(&self) -> Receiver<T> {
        let (tx, rx) = mpsc::channel();
        self.qabul_qiluvchilar.lock().unwrap().push(tx);
        rx
    }

    fn yuborish(&self, qiymat: T) {
        let mut qqlar = self.qabul_qiluvchilar.lock().unwrap();
        // Uzilgan qabul qiluvchilarni olib tashlash
        qqlar.retain(|tx| tx.send(qiymat.clone()).is_ok());
    }
}

fn broadcast_misoli() {

    let kanal = Arc::new(BroadcastKanal::<String>::new());

    let rx1 = kanal.obuna_bo_l();
    let rx2 = kanal.obuna_bo_l();
    let rx3 = kanal.obuna_bo_l();

    // Har receiver o'z threadida
    let consumer1 = thread::spawn(move || {
        for v in rx1.iter() { println!("[Subscriber-1] {}", v); }
    });
    let consumer2 = thread::spawn(move || {
        for v in rx2.iter() { println!("[Subscriber-2] {}", v); }
    });
    let consumer3 = thread::spawn(move || {
        for v in rx3.iter() { println!("[Subscriber-3] {}", v); }
    });

    // Broadcast yuborish
    kanal.yuborish("Voqea 1".to_string());
    kanal.yuborish("Voqea 2".to_string());
    kanal.yuborish("Voqea 3".to_string());

    drop(kanal); // Kanallarni yopish

    consumer1.join().unwrap();
    consumer2.join().unwrap();
    consumer3.join().unwrap();
    // [Subscriber-X] Voqea 1
    // [Subscriber-X] Voqea 2
    // [Subscriber-X] Voqea 3 (barcha 3 subscriber uchun)
}

// Aktor — o'z holati va xabar navbatiga ega mustaqil birlik
// Актор — независимая единица со своим состоянием и очередью сообщений
#[derive(Debug)]
enum HisobXabari {
    Qosh(i64),
    Ayir(i64),
    Kopaytir(i64),
    Qiymat(Sender<i64>),
    Nolga,
    Tugat,
}

struct HisobAktor {
    rx: Receiver<HisobXabari>,
    qiymat: i64,
}

impl HisobAktor {
    fn new(rx: Receiver<HisobXabari>) -> Self {
        HisobAktor { rx, qiymat: 0 }
    }

    fn ishga_tushir(mut self) {
        while let Ok(xabar) = self.rx.recv() {
            match xabar {
                HisobXabari::Qosh(n)     => self.qiymat += n,
                HisobXabari::Ayir(n)      => self.qiymat -= n,
                HisobXabari::Kopaytir(n) => self.qiymat *= n,
                HisobXabari::Nolga        => self.qiymat = 0,
                HisobXabari::Qiymat(tx)   => tx.send(self.qiymat).unwrap(),
                HisobXabari::Tugat        => break,
            }
        }
    }
}

struct HisobHandler {
    tx: Sender<HisobXabari>,
}

impl HisobHandler {
    fn yangi() -> Self {
        let (tx, rx) = mpsc::channel();
        let aktor = HisobAktor::new(rx);
        thread::spawn(move || aktor.ishga_tushir());
        HisobHandler { tx }
    }

    fn qo_sh(&self, n: i64)     { self.tx.send(HisobXabari::Qosh(n)).unwrap(); }
    fn ayir(&self, n: i64)       { self.tx.send(HisobXabari::Ayir(n)).unwrap(); }
    fn ko_paytir(&self, n: i64)  { self.tx.send(HisobXabari::Kopaytir(n)).unwrap(); }
    fn nolga(&self)               { self.tx.send(HisobXabari::Nolga).unwrap(); }

    fn qiymat(&self) -> i64 {
        let (tx, rx) = mpsc::channel();
        self.tx.send(HisobXabari::Qiymat(tx)).unwrap();
        rx.recv().unwrap()
    }

    fn tugat(self) {
        self.tx.send(HisobXabari::Tugat).unwrap();
    }
}

fn actor_misoli() {

    let hisob = HisobHandler::yangi();

    hisob.qo_sh(100);
    hisob.qo_sh(50);
    hisob.ayir(30);
    hisob.ko_paytir(3);
    println!("Qiymat: {}", hisob.qiymat()); // (100+50-30)*3 = 360
    // Qiymat: 360

    hisob.nolga();
    println!("Noldan keyin: {}", hisob.qiymat());
    // Noldan keyin: 0

    // Ko'p thread dan aktorga xabar yuborish
    let hisob2 = Arc::new(Mutex::new(HisobHandler::yangi()));
    let mut handlar = vec![];

    for i in 0..5 {
        let h = Arc::clone(&hisob2);
        handlar.push(thread::spawn(move || {
            h.lock().unwrap().qo_sh(i * 10);
        }));
    }
    for h in handlar { h.join().unwrap(); }

    let son = hisob2.lock().unwrap().qiymat();
    println!("Ko'p thread: {}", son); // 0+10+20+30+40 = 100
    // Ko'p thread: 100
}

// Task dispatcher — ishlarni threadlarga tarqatish
// Task dispatcher — распределение задач по потокам
fn task_dispatcher_misoli() {

    #[derive(Debug)]
    struct Vazifa {
        id: usize,
        ma_lumot: Vec<i32>,
    }

    #[derive(Debug)]
    struct Natija {
        id: usize,
        yig_indi: i32,
        worker: usize,
    }

    let worker_soni = 3;
    let (task_tx, task_rx) = mpsc::channel::<Vazifa>();
    let task_rx = Arc::new(Mutex::new(task_rx));
    let (natija_tx, natija_rx) = mpsc::channel::<Natija>();

    // Workerlar
    let mut handlar = vec![];
    for worker_id in 0..worker_soni {
        let rx = Arc::clone(&task_rx);
        let ntx = natija_tx.clone();
        handlar.push(thread::spawn(move || {
            loop {
                let vazifa = {
                    match rx.lock().unwrap().try_recv() {
                        Ok(v) => v,
                        Err(TryRecvError::Empty) => {
                            thread::sleep(Duration::from_millis(1));
                            continue;
                        }
                        Err(TryRecvError::Disconnected) => break,
                    }
                };

                let yig: i32 = vazifa.ma_lumot.iter().sum();
                ntx.send(Natija { id: vazifa.id, yig_indi: yig, worker: worker_id }).unwrap();
            }
        }));
    }

    // Vazifalar yuborish
    for i in 0..9 {
        task_tx.send(Vazifa {
            id: i,
            ma_lumot: (1..=(i as i32 + 1)).collect(),
        }).unwrap();
    }
    drop(task_tx);

    for h in handlar { h.join().unwrap(); }
    drop(natija_tx);

    let mut natijalar: Vec<Natija> = natija_rx.iter().collect();
    natijalar.sort_by_key(|n| n.id);

    for n in &natijalar {
        println!("Vazifa {}: yig'indi={}, worker={}", n.id, n.yig_indi, n.worker);
    }
    // Vazifa 0: yig'indi=1, worker=X
    // Vazifa 1: yig'indi=3, worker=X
    // ...
}

fn real_hayot_misollari() {

    println!("--- Pipeline ---");
    pipeline_misoli();

    println!("\n--- Oneshot ---");
    oneshot_misoli();

    println!("\n--- Broadcast ---");
    broadcast_misoli();

    println!("\n--- Actor Pattern ---");
    actor_misoli();

    println!("\n--- Task Dispatcher ---");
    task_dispatcher_misoli();
}

fn main() {

    println!("=== MPSC ASOSIY ===");
    mpsc_asosiy_misollari();

    println!("\n=== SYNC_CHANNEL ===");
    sync_channel_misollari();

    println!("\n=== TRY_RECV ===");
    try_recv_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        MPSC                                                                                                  |
// #================================================================================================================================================#
// # |   1 | mpsc::channel()                 | Cheksiz bufer, async send                  | Безграничный буфер, async send                          |
// # |   2 | mpsc::sync_channel(n)           | n bufer, blocking send                     | Буфер n, блокирующий send                               |
// # |   3 | tx.clone()                      | Ko'p yuboruvchi yaratish                   | Создание нескольких отправителей                        |
// # |   4 | rx.recv()                       | Blocking qabul qilish                      | Блокирующий приём                                       |
// # |   5 | rx.try_recv()                   | Non-blocking, Empty/Disconnected xato      | Неблокирующий, ошибка Empty/Disconnected                |
// # |   6 | rx.recv_timeout(dur)            | Timeout bilan kutish                       | Ожидание с таймаутом                                    |
// # |   7 | rx.iter()                       | Kanal yopilguncha iteratsiya               | Итерация пока канал не закрыт                           |
// # |   8 | tx.try_send()                   | Non-blocking, Full/Disconnected xato       | Неблокирующий, ошибка Full/Disconnected                 |
// # |   9 | drop(tx)                        | Kanal yopiladi — rx.recv() Err qaytaradi   | Канал закрывается — rx.recv() вернёт Err                |
// #================================================================================================================================================#
// # |                                        PATTERNLAR                                                                                            |
// #================================================================================================================================================#
// # |  10 | Pipeline                        | Kanallar zanjiri — bosqichma-bosqich       | Цепочка каналов — поэтапно                              |
// # |  11 | Oneshot                         | Bir marta javob qaytaruvchi kanal          | Канал для одноразового ответа                           |
// # |  12 | Broadcast                       | Bir xabar ko'p qabul qiluvchiga            | Одно сообщение нескольким получателям                   |
// # |  13 | Actor pattern                   | Xabar asosida holat mashina                | Машина состояний на основе сообщений                    |
// # |  14 | Task dispatcher                 | Ishlarni workerlar orasida taqsimlash      | Распределение задач между воркерами                     |
// # |  15 | Request-Response                | tx kanal natija uchun xabarda              | tx канал для ответа передаётся в сообщении              |
// #================================================================================================================================================#