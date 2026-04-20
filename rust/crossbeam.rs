// #================================================================================================================================================#
// #                                                                CROSSBEAM                                                                       #
// #                        CROSSBEAM — CONCURRENT DATA STRUCTURES. CHANNEL, DEQUE, QUEUE, EPOCH GC. LOCK-FREE PATTERNS.                            #
// #                        CROSSBEAM — КОНКУРЕНТНЫЕ СТРУКТУРЫ ДАННЫХ. CHANNEL, DEQUE, QUEUE, EPOCH GC. LOCK-FREE ПАТТЕРНЫ.                         #
// #================================================================================================================================================#

#![allow(dead_code, unused)]
use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::{VecDeque, HashMap};

// Crossbeam nima:
// Что такое Crossbeam:
//
//   crossbeam-channel  — mpmc, bounded/unbounded, select!
//   crossbeam-deque    — work-stealing deque (rayon uchun)
//   crossbeam-queue    — lock-free queue (SegQueue, ArrayQueue)
//   crossbeam-epoch    — epoch-based garbage collection
//   crossbeam-utils    — CachePadded, Backoff, Parker
//
//   std::sync::mpsc dan farqlari:
//   Отличия от std::sync::mpsc:
//   ✅ MPMC — ko'p producer VA ko'p consumer
//   ✅ bounded + unbounded channel
//   ✅ select! — bir nechta kanaldan tanlash
//   ✅ try_send/try_recv + timeout
//   ✅ Sender/Receiver klonlanadi (ikkalasi ham)

struct MpmcChannel<T> {
    navbat: Arc<Mutex<VecDeque<T>>>,
    cvar: Arc<Condvar>,
    yopildi: Arc<AtomicBool>,
}

impl<T: Send + 'static> MpmcChannel<T> {
    fn new() -> (MpmcSender<T>, MpmcReceiver<T>) {
        let navbat = Arc::new(Mutex::new(VecDeque::new()));
        let cvar = Arc::new(Condvar::new());
        let yopildi = Arc::new(AtomicBool::new(false));

        let tx = MpmcSender {
            navbat: Arc::clone(&navbat),
            cvar: Arc::clone(&cvar),
            yopildi: Arc::clone(&yopildi),
        };
        let rx = MpmcReceiver {
            navbat,
            cvar,
            yopildi,
        };
        (tx, rx)
    }
}

#[derive(Clone)]
struct MpmcSender<T> {
    pub navbat: Arc<Mutex<VecDeque<T>>>,
    pub cvar: Arc<Condvar>,
    pub yopildi: Arc<AtomicBool>,
}

impl<T> MpmcSender<T> {
    fn yuborish(&self, qiymat: T) -> Result<(), &'static str> {
        if self.yopildi.load(Ordering::SeqCst) {
            return Err("Kanal yopildi");
        }
        self.navbat.lock().unwrap().push_back(qiymat);
        self.cvar.notify_one();
        Ok(())
    }
}

#[derive(Clone)]
struct MpmcReceiver<T> {
    navbat: Arc<Mutex<VecDeque<T>>>,
    cvar: Arc<Condvar>,
    yopildi: Arc<AtomicBool>,
}

impl<T> MpmcReceiver<T> {
    fn qabul_qilish(&self) -> Option<T> {
        let mut navbat = self.navbat.lock().unwrap();
        loop {
            if let Some(v) = navbat.pop_front() {
                return Some(v);
            }
            if self.yopildi.load(Ordering::SeqCst) {
                return None;
            }
            navbat = self.cvar.wait(navbat).unwrap();
        }
    }

    fn try_qabul(&self) -> Option<T> {
        self.navbat.lock().unwrap().pop_front()
    }

    fn qabul_timeout(&self, dur: Duration) -> Option<T> {
        let deadline = Instant::now() + dur;
        let mut navbat = self.navbat.lock().unwrap();
        loop {
            if let Some(v) = navbat.pop_front() { return Some(v); }
            let qolgan = deadline.saturating_duration_since(Instant::now());
            if qolgan.is_zero() { return None; }
            let (yangi, timeout) = self.cvar.wait_timeout(navbat, qolgan).unwrap();
            navbat = yangi;
            if timeout.timed_out() { return None; }
        }
    }

    fn yopish(&self) {
        self.yopildi.store(true, Ordering::SeqCst);
        self.cvar.notify_all();
    }
}

fn mpmc_misoli() {

    println!("--- MPMC Channel ---");
    let (tx, rx) = MpmcChannel::new();
    let tx2 = tx.clone(); // Ikkinchi producer — std::mpsc da ham mumkin
    let rx2 = rx.clone(); // Ikkinchi consumer — std::mpsc da YO'Q!

    // 2 ta producer
    let p1 = thread::spawn(move || {
        for i in 0..5 {
            tx.yuborish(format!("P1-{}", i)).unwrap();
            thread::sleep(Duration::from_millis(5));
        }
    });

    let p2 = thread::spawn(move || {
        for i in 0..5 {
            tx2.yuborish(format!("P2-{}", i)).unwrap();
            thread::sleep(Duration::from_millis(7));
        }
    });

    // 2 ta consumer
    let hisob1 = Arc::new(AtomicUsize::new(0));
    let hisob2 = Arc::clone(&hisob1);
    let rx_klon = rx2.clone();

    let c1 = thread::spawn(move || {
        let mut n = 0;
        while let Some(v) = rx.qabul_timeout(Duration::from_millis(100)) {
            println!("[C1] {}", v);
            n += 1;
        }
        hisob2.fetch_add(n, Ordering::SeqCst);
    });

    let c2 = thread::spawn(move || {
        while let Some(v) = rx_klon.qabul_timeout(Duration::from_millis(100)) {
            println!("[C2] {}", v);
        }
    });

    p1.join().unwrap();
    p2.join().unwrap();
    c1.join().unwrap();
    c2.join().unwrap();
    // [C1/C2] P1-0, P2-0, P1-1, ... (tartib farqli)
}

// crossbeam select! makrosi simulyatsiyasi
// Симуляция макроса select! crossbeam
fn select_misoli() {

    println!("\n--- Select (bir nechta kanal) ---");

    let (tx1, rx1) = MpmcChannel::new();
    let (tx2, rx2) = MpmcChannel::new();
    let (tx3, rx3) = MpmcChannel::new();

    // Producerlar
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(30));
        tx1.yuborish(42i32).ok();
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(10)); // eng tez
        tx2.yuborish("salom".to_string()).ok();
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        tx3.yuborish(3.14f64).ok();
    });

    // Select — birinchi tayyor bo'lganini olish
    let boshlanish = Instant::now();
    for _ in 0..3 {
        loop {
            if let Some(v) = rx1.try_qabul() {
                println!("rx1: {} ({:.1?})", v, boshlanish.elapsed());
                break;
            }
            if let Some(v) = rx2.try_qabul() {
                println!("rx2: {} ({:.1?})", v, boshlanish.elapsed());
                break;
            }
            if let Some(v) = rx3.try_qabul() {
                println!("rx3: {} ({:.1?})", v, boshlanish.elapsed());
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
    // rx2: salom (~10ms) — eng tez
    // rx1: 42 (~30ms)
    // rx3: 3.14 (~50ms)
}

// crossbeam-deque: Worker (LIFO/FIFO) + Stealer
// crossbeam-deque: Worker (LIFO/FIFO) + Stealer

struct WorkStealingDeque<T: Send> {
    lokal: Arc<Mutex<VecDeque<T>>>,
}

impl<T: Send> WorkStealingDeque<T> {
    fn new() -> (Self, WorkStealer<T>) {
        let lokal = Arc::new(Mutex::new(VecDeque::new()));
        let stealer = WorkStealer { lokal: Arc::clone(&lokal) };
        (WorkStealingDeque { lokal }, stealer)
    }

    fn push(&self, val: T) {
        self.lokal.lock().unwrap().push_back(val);
    }

    fn pop(&self) -> Option<T> {
        self.lokal.lock().unwrap().pop_back() // LIFO — oxiridan
    }

    fn uzunlik(&self) -> usize {
        self.lokal.lock().unwrap().len()
    }
}

struct WorkStealer<T: Send> {
    lokal: Arc<Mutex<VecDeque<T>>>,
}

impl<T: Send> WorkStealer<T> {
    fn steal(&self) -> Option<T> {
        self.lokal.lock().unwrap().pop_front() // FIFO — oldidan o'g'irlash
    }
}

fn work_stealing_misoli() {

    println!("\n--- Work-Stealing Deque ---");

    let (worker, stealer) = WorkStealingDeque::new();

    // Worker vazifalar qo'shadi
    for i in 0..10 {
        worker.push(i);
    }
    println!("Deque uzunlik: {}", worker.uzunlik());

    // Worker o'z ishini oladi (LIFO)
    println!("Worker pop: {:?}", worker.pop()); // Some(9)
    println!("Worker pop: {:?}", worker.pop()); // Some(8)

    // Stealer o'g'irlaydi (FIFO)
    println!("Stealer steal: {:?}", stealer.steal()); // Some(0)
    println!("Stealer steal: {:?}", stealer.steal()); // Some(1)
    println!("Qolgan: {}", worker.uzunlik()); // 6

    // Work-stealing thread pool simulyatsiya
    println!("\nWork-stealing thread pool:");
    let vazifalar_soni = Arc::new(AtomicUsize::new(0));

    let (w1, s1) = WorkStealingDeque::<Box<dyn FnOnce() + Send>>::new();
    let (w2, s2) = WorkStealingDeque::<Box<dyn FnOnce() + Send>>::new();

    // Vazifalar qo'shish
    let w1_arc = Arc::new(w1);
    let w2_arc = Arc::new(w2);

    for i in 0..6 {
        let n = Arc::clone(&vazifalar_soni);
        let v: Box<dyn FnOnce() + Send> = Box::new(move || {
            println!("[Vazifa {}] bajarildi", i);
            n.fetch_add(1, Ordering::SeqCst);
        });
        if i % 2 == 0 {
            w1_arc.push(v);
        } else {
            w2_arc.push(v);
        }
    }

    // Thread 1: o'z vazifalarini bajaradi + Thread 2 dan o'g'irlaydi
    let w1_t = Arc::clone(&w1_arc);
    let t1 = thread::spawn(move || {
        loop {
            if let Some(f) = w1_t.pop() {
                f();
            } else if let Some(f) = s2.steal() {
                println!("[T1] T2 dan o'g'irladi!");
                f();
            } else {
                break;
            }
        }
    });

    // Thread 2: o'z vazifalarini bajaradi + Thread 1 dan o'g'irlaydi
    let w2_t = Arc::clone(&w2_arc);
    let t2 = thread::spawn(move || {
        loop {
            if let Some(f) = w2_t.pop() {
                f();
            } else if let Some(f) = s1.steal() {
                println!("[T2] T1 dan o'g'irladi!");
                f();
            } else {
                break;
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("Jami bajarilgan: {}", vazifalar_soni.load(Ordering::SeqCst));
    // Jami bajarilgan: 6
}

// SegQueue — lock-free unbounded queue
// crossbeam da: SegQueue — ko'p thread parallel push/pop
// В crossbeam: SegQueue — параллельные push/pop из нескольких потоков

struct SimpleQueue<T: Send> {
    ichki: Mutex<VecDeque<T>>,
}

impl<T: Send> SimpleQueue<T> {
    fn new() -> Self { SimpleQueue { ichki: Mutex::new(VecDeque::new()) } }
    fn push(&self, val: T) { self.ichki.lock().unwrap().push_back(val); }
    fn pop(&self) -> Option<T> { self.ichki.lock().unwrap().pop_front() }
    fn uzunlik(&self) -> usize { self.ichki.lock().unwrap().len() }
    fn boshmi(&self) -> bool { self.ichki.lock().unwrap().is_empty() }
}

fn seg_queue_misoli() {

    println!("\n--- SegQueue (Lock-free Queue) ---");

    let queue = Arc::new(SimpleQueue::<String>::new());

    // Ko'p producer
    let mut producers = vec![];
    for i in 0..4 {
        let q = Arc::clone(&queue);
        producers.push(thread::spawn(move || {
            for j in 0..5 {
                q.push(format!("P{}-{}", i, j));
            }
        }));
    }

    // Ko'p consumer
    let qabul_qilingan = Arc::new(Mutex::new(Vec::<String>::new()));
    let mut consumers = vec![];
    for _ in 0..2 {
        let q = Arc::clone(&queue);
        let qabul = Arc::clone(&qabul_qilingan);
        consumers.push(thread::spawn(move || {
            let boshlanish = Instant::now();
            while boshlanish.elapsed() < Duration::from_millis(100) {
                if let Some(v) = q.pop() {
                    qabul.lock().unwrap().push(v);
                } else {
                    thread::sleep(Duration::from_millis(1));
                }
            }
        }));
    }

    for p in producers { p.join().unwrap(); }
    for c in consumers { c.join().unwrap(); }

    // Qolganlarini olish
    while let Some(v) = queue.pop() {
        qabul_qilingan.lock().unwrap().push(v);
    }

    let mut qabul = qabul_qilingan.lock().unwrap();
    qabul.sort();
    println!("Jami qabul qilindi: {}", qabul.len());
    println!("Dastlabki 5: {:?}", &qabul[..5.min(qabul.len())]);
    // Jami qabul qilindi: 20
    // Dastlabki 5: ["P0-0", "P0-1", "P0-2", "P0-3", "P0-4"]
}

// CachePadded — cache line to'ldirib false sharing oldini olish
// CachePadded — заполнение cache line для предотвращения false sharing
//
// False sharing: bir xil cache line da bo'lgan turli o'zgaruvchilar
// False sharing: разные переменные в одной cache line
// bir thread o'zgartirsa — boshqa thread cache invalid bo'ladi
// один поток изменяет — у другого потока cache становится invalid

// #[repr(align(64))] — cache line hajmi (ko'pchilik CPU da 64 bayt)
#[repr(align(64))]
struct CachePadded<T> {
    qiymat: T,
}

impl<T> CachePadded<T> {
    fn new(val: T) -> Self { CachePadded { qiymat: val } }
    fn qiymat(&self) -> &T { &self.qiymat }
}

fn cache_padded_misoli() {

    println!("\n--- CachePadded (False Sharing) ---");

    // False sharing bo'lishi mumkin bo'lgan holatda
    // Случай возможного false sharing
    struct CounterFalsSharing {
        a: AtomicUsize,
        b: AtomicUsize, // a bilan bir xil cache line da bo'lishi mumkin
    }

    // Cache padded — har biri o'z cache line da
    // Cache padded — каждый в своей cache line
    struct CounterCachePadded {
        a: CachePadded<AtomicUsize>,
        b: CachePadded<AtomicUsize>,
    }

    let n = 1_000_000;

    // False sharing test
    let fs = Arc::new(CounterFalsSharing {
        a: AtomicUsize::new(0),
        b: AtomicUsize::new(0),
    });
    let fs2 = Arc::clone(&fs);

    let t1 = Instant::now();
    let h1 = thread::spawn(move || {
        for _ in 0..n { fs.a.fetch_add(1, Ordering::Relaxed); }
    });
    let h2 = thread::spawn(move || {
        for _ in 0..n { fs2.b.fetch_add(1, Ordering::Relaxed); }
    });
    h1.join().unwrap(); h2.join().unwrap();
    let vaqt1 = t1.elapsed();

    // Cache padded test
    let cp = Arc::new(CounterCachePadded {
        a: CachePadded::new(AtomicUsize::new(0)),
        b: CachePadded::new(AtomicUsize::new(0)),
    });
    let cp2 = Arc::clone(&cp);

    let t2 = Instant::now();
    let h3 = thread::spawn(move || {
        for _ in 0..n { cp.a.qiymat().fetch_add(1, Ordering::Relaxed); }
    });
    let h4 = thread::spawn(move || {
        for _ in 0..n { cp2.b.qiymat().fetch_add(1, Ordering::Relaxed); }
    });
    h3.join().unwrap(); h4.join().unwrap();
    let vaqt2 = t2.elapsed();

    println!("False sharing:  {:?}", vaqt1);
    println!("Cache padded:   {:?}", vaqt2);
    println!("Align: {} bayt", std::mem::align_of::<CachePadded<AtomicUsize>>());
    // False sharing:  ~Xms
    // Cache padded:   ~Yms (odatda tezroq)
    // Align: 64 bayt
}

// Backoff — spin → yield → sleep strategiyasi
// Backoff — стратегия spin → yield → sleep
struct Backoff {
    qadam: u32,
}

impl Backoff {
    fn new() -> Self { Backoff { qadam: 0 } }

    fn snooze(&mut self) {
        if self.qadam < 6 {
            // Spin loop — CPU ga imkoniyat berish
            for _ in 0..(1 << self.qadam) {
                std::hint::spin_loop();
            }
        } else if self.qadam < 10 {
            thread::yield_now();
        } else {
            thread::sleep(Duration::from_micros(1 << (self.qadam - 10).min(10)));
        }
        self.qadam = self.qadam.saturating_add(1);
    }

    fn reset(&mut self) { self.qadam = 0; }
    fn is_completed(&self) -> bool { self.qadam >= 10 }
}

fn backoff_misoli() {

    println!("\n--- Backoff (Spin Loop Optimallashtirish) ---");

    let tayyor = Arc::new(AtomicBool::new(false));
    let t = Arc::clone(&tayyor);

    let producer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        t.store(true, Ordering::Release);
        println!("[Producer] Signal yuborildi");
    });

    // Backoff bilan kutish
    let mut backoff = Backoff::new();
    let boshlanish = Instant::now();
    while !tayyor.load(Ordering::Acquire) {
        backoff.snooze();
    }
    println!("[Consumer] Signal olindi ({:.1?})", boshlanish.elapsed());

    producer.join().unwrap();
    // [Producer] Signal yuborildi
    // [Consumer] Signal olindi (~20ms)
}

// Pipeline — crossbeam channel bilan
// Pipeline — с каналом crossbeam
fn pipeline_misoli() {

    println!("\n--- Concurrent Pipeline ---");

    // Stage 1: sonlarni yaratish
    // Stage 2: kvadratga ko'tarish
    // Stage 3: filtrlash (100 dan katta)
    // Stage 4: formatlash

    let (tx1, rx1) = MpmcChannel::new();
    let (tx2, rx2) = MpmcChannel::new();
    let (tx3, rx3) = MpmcChannel::new();

    let s1 = thread::spawn(move || {
        for i in 1i32..=20 {
            tx1.yuborish(i).unwrap();
        }
    });

    let s2 = thread::spawn(move || {
        while let Some(v) = rx1.qabul_timeout(Duration::from_millis(100)) {
            tx2.yuborish(v * v).unwrap();
        }
    });

    let s3 = thread::spawn(move || {
        while let Some(v) = rx2.qabul_timeout(Duration::from_millis(100)) {
            if v > 100 { tx3.yuborish(v).unwrap(); }
        }
    });

    let mut natijalar = vec![];
    s1.join().unwrap();
    s2.join().unwrap();
    s3.join().unwrap();

    while let Some(v) = rx3.try_qabul() {
        natijalar.push(v);
    }

    println!("Pipeline natija (x² > 100): {:?}", natijalar);
    // [121, 144, 169, 196, 225, 256, 289, 324, 361, 400]
}

// Actor tizimi — MPMC bilan
// Система акторов — с MPMC
fn actor_tizimi_misoli() {

    println!("\n--- Actor Tizimi ---");

    #[derive(Debug)]
    enum Buyruq {
        QoSh(i64),
        KoPaytir(i64),
        Qiymat(std::sync::mpsc::SyncSender<i64>),
        Tugat,
    }

    let (tx, rx) = MpmcChannel::<Buyruq>::new();
    let tx2 = MpmcSender { navbat: Arc::clone(&tx.navbat), cvar: Arc::clone(&tx.cvar), yopildi: Arc::clone(&tx.yopildi) };
    let tx3 = MpmcSender { navbat: Arc::clone(&tx.navbat), cvar: Arc::clone(&tx.cvar), yopildi: Arc::clone(&tx.yopildi) };

    // Aktor
    let aktor = thread::spawn(move || {
        let mut holat: i64 = 0;
        while let Some(buyruq) = rx.qabul_qilish() {
            match buyruq {
                Buyruq::QoSh(n)     => holat += n,
                Buyruq::KoPaytir(n) => holat *= n,
                Buyruq::Qiymat(ch)   => ch.send(holat).unwrap(),
                Buyruq::Tugat        => break,
            }
        }
    });

    // Ko'p thread dan buyruqlar
    let t1 = thread::spawn(move || {
        tx.yuborish(Buyruq::QoSh(100)).unwrap();
        tx.yuborish(Buyruq::QoSh(50)).unwrap();
    });

    let t2 = thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        tx2.yuborish(Buyruq::KoPaytir(3)).unwrap();
    });

    t1.join().unwrap();
    t2.join().unwrap();

    // Qiymatni so'rash
    let (qtx, qrx) = std::sync::mpsc::sync_channel(1);
    tx3.yuborish(Buyruq::Qiymat(qtx)).unwrap();
    let qiymat = qrx.recv().unwrap();
    println!("Aktor holati: {}", qiymat); // (100+50)*3 = 450

    tx3.yuborish(Buyruq::Tugat).unwrap();
    aktor.join().unwrap();
    // Aktor holati: 450
}

fn main() {

    println!("=== MPMC CHANNEL ===");
    mpmc_misoli();

    println!("=== SELECT ===");
    select_misoli();

    println!("=== WORK-STEALING DEQUE ===");
    work_stealing_misoli();

    println!("=== SEG QUEUE ===");
    seg_queue_misoli();

    println!("=== CACHE PADDED ===");
    cache_padded_misoli();

    println!("=== BACKOFF ===");
    backoff_misoli();

    println!("=== PIPELINE ===");
    pipeline_misoli();

    println!("=== ACTOR TIZIMI ===");
    actor_tizimi_misoli();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                               | Описание (RU)                                            |
// #================================================================================================================================================#
// # |                                        CROSSBEAM-CHANNEL                                                                                     |
// #================================================================================================================================================#
// # |   1 | unbounded()                     | Cheksiz bufer, non-blocking send           | Безграничный буфер, неблокирующий send                  |
// # |   2 | bounded(n)                      | n bufer, backpressure                      | Буфер n, обратное давление                              |
// # |   3 | rx.clone()                      | Consumer klonlash — MPMC                   | Клонирование consumer — MPMC                            |
// # |   4 | select! { recv(rx) -> v => }    | Bir nechta kanaldan tanlash                | Выбор из нескольких каналов                             |
// # |   5 | recv_timeout(dur)               | Timeout bilan qabul                        | Приём с таймаутом                                       |
// #================================================================================================================================================#
// # |                                        CROSSBEAM-DEQUE                                                                                       |
// #================================================================================================================================================#
// # |   6 | Worker::new_lifo/fifo()         | Work-stealing worker                       | Воркер с work-stealing                                  |
// # |   7 | w.push(val)                     | Lokal qo'shish                             | Локальное добавление                                    |
// # |   8 | w.pop()                         | Lokal olish (LIFO)                         | Локальное взятие (LIFO)                                 |
// # |   9 | stealer.steal()                 | Boshqa threaddan o'g'irlash                | Кража из другого потока                                 |
// #================================================================================================================================================#
// # |                                        CROSSBEAM-QUEUE                                                                                       |
// #================================================================================================================================================#
// # |  10 | SegQueue — unbounded lock-free  | Ko'p thread parallel push/pop              | Параллельные push/pop из нескольких потоков             |
// # |  11 | ArrayQueue — bounded lock-free  | Chegarali, backpressure                    | Ограниченный, обратное давление                         |
// #================================================================================================================================================#
// # |                                        CROSSBEAM-UTILS                                                                                       |
// #================================================================================================================================================#
// # |  12 | CachePadded<T>                  | False sharing oldini olish                 | Предотвращение false sharing                            |
// # |  13 | Backoff::snooze()               | Spin → yield → sleep strategiya            | Стратегия spin → yield → sleep                          |
// # |  14 | Parker::park/unpark             | Thread park/unpark boshqaruvi              | Управление park/unpark потока                           |
// #================================================================================================================================================#