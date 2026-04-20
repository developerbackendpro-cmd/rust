// #================================================================================================================================================#
// #                                                        DASHMAP + PARKING_LOT                                                                   #
// #                        DASHMAP — CONCURRENT HASHMAP. PARKING_LOT — TEZKOR MUTEX/RWLOCK. STD ALTERNATIVLARI.                                    #
// #                        DASHMAP — КОНКУРЕНТНЫЙ HASHMAP. PARKING_LOT — БЫСТРЫЙ MUTEX/RWLOCK. АЛЬТЕРНАТИВЫ STD.                                   #
// #================================================================================================================================================#

// Cargo.toml (haqiqiy loyihada):
// [dependencies]
// dashmap = "6"
// parking_lot = "0.12"

// Bu fayl dashmap va parking_lot konseptsiyalarini
// std bilan simulyatsiya qiladi.
// Haqiqiy kutubxona kodi comment sifatida ko'rsatiladi.

#![allow(dead_code, unused)]

use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;

// Dashmap nima:
// Что такое DashMap:
//
//   HashMap<K, V> ga o'xshash lekin thread-safe
//   Похож на HashMap<K, V> но потокобезопасный
//   Lock segmentatsiyasi bilan — ichida ko'p RwLock
//   С сегментацией блокировок — внутри много RwLock
//   Arc<DashMap> + clone — xuddi Arc<RwLock<HashMap>> kabi
//   Arc<DashMap> + clone — как Arc<RwLock<HashMap>>
//   Lekin granular locking — faqat kerakli segment lock
//   Но с гранулярной блокировкой — только нужный сегмент
//
// Parking_lot nima:
// Что такое parking_lot:
//
//   std::sync::Mutex dan 2-4x tezroq
//   В 2-4 раза быстрее std::sync::Mutex
//   Poisoning yo'q — always Ok()
//   Нет poisoning — всегда Ok()
//   ReentrantMutex — bir thread ikki marta lock olishi mumkin
//   ReentrantMutex — один поток может взять lock дважды
//   FairMutex — FIFO tartibda
//   FairMutex — в порядке FIFO
//   Condvar — parking_lot ning o'zida

// DashMap — ichida sharding bilan implementatsiya
// DashMap — внутри реализован с шардированием
const SHARD_SONI: usize = 16;

struct DashMapSim<K, V> {
    shardlar: Vec<RwLock<HashMap<K, V>>>,
}

impl<K, V> DashMapSim<K, V>
where
    K: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn new() -> Self {
        let mut shardlar = Vec::with_capacity(SHARD_SONI);
        for _ in 0..SHARD_SONI {
            shardlar.push(RwLock::new(HashMap::new()));
        }
        DashMapSim { shardlar }
    }

    fn shard_indeks(&self, kalit: &K) -> usize {
        use std::hash::{Hash, Hasher};
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        kalit.hash(&mut hasher);
        (hasher.finish() as usize) % SHARD_SONI
    }

    fn qo_sh(&self, kalit: K, qiymat: V) {
        let idx = self.shard_indeks(&kalit);
        self.shardlar[idx].write().unwrap().insert(kalit, qiymat);
    }

    fn ol(&self, kalit: &K) -> Option<V> {
        let idx = self.shard_indeks(kalit);
        self.shardlar[idx].read().unwrap().get(kalit).cloned()
    }

    fn mavjud(&self, kalit: &K) -> bool {
        let idx = self.shard_indeks(kalit);
        self.shardlar[idx].read().unwrap().contains_key(kalit)
    }

    fn o_chir(&self, kalit: &K) -> Option<V> {
        let idx = self.shard_indeks(kalit);
        self.shardlar[idx].write().unwrap().remove(kalit)
    }

    fn uzunlik(&self) -> usize {
        self.shardlar.iter().map(|s| s.read().unwrap().len()).sum()
    }

    fn yangilash<F: Fn(&mut V)>(&self, kalit: &K, f: F) -> bool {
        let idx = self.shard_indeks(kalit);
        let mut shard = self.shardlar[idx].write().unwrap();
        if let Some(v) = shard.get_mut(kalit) {
            f(v);
            true
        } else {
            false
        }
    }

    fn entry_or_insert(&self, kalit: K, qiymat: V) -> V {
        let idx = self.shard_indeks(&kalit);
        let mut shard = self.shardlar[idx].write().unwrap();
        shard.entry(kalit).or_insert(qiymat).clone()
    }

    fn barcha_kalitlar(&self) -> Vec<K> {
        self.shardlar.iter()
            .flat_map(|s| s.read().unwrap().keys().cloned().collect::<Vec<_>>())
            .collect()
    }

    fn retain<F: Fn(&K, &V) -> bool>(&self, f: F) {
        for shard in &self.shardlar {
            shard.write().unwrap().retain(|k, v| f(k, v));
        }
    }
}

fn dashmap_sim_misoli() {

    println!("--- DashMap Simulyatsiya ---");

    let xarita = Arc::new(DashMapSim::new());

    // Parallel insert
    let mut handlar = vec![];
    for i in 0..8 {
        let x = Arc::clone(&xarita);
        handlar.push(thread::spawn(move || {
            for j in 0..100 {
                let kalit = format!("thread_{}_key_{}", i, j);
                x.qo_sh(kalit, i * 100 + j);
            }
        }));
    }
    for h in handlar { h.join().unwrap(); }

    println!("Jami elementlar: {}", xarita.uzunlik()); // 800

    // Parallel read
    let mut o_qish_handlar = vec![];
    for i in 0..4 {
        let x = Arc::clone(&xarita);
        o_qish_handlar.push(thread::spawn(move || {
            let kalit = format!("thread_{}_key_50", i);
            x.ol(&kalit)
        }));
    }
    for h in o_qish_handlar {
        println!("O'qildi: {:?}", h.join().unwrap());
    }

    // Yangilash
    let kalit = "thread_0_key_0".to_string();
    xarita.yangilash(&kalit, |v| *v *= 10);
    println!("Yangilangan: {:?}", xarita.ol(&kalit));

    // Retain — filtrlash
    xarita.retain(|_, v| *v > 500);
    println!("Filtrlangandan keyin: {}", xarita.uzunlik());
    // Jami elementlar: 800
    // O'qildi: Some(50) / Some(150) / ...
    // Yangilangan: Some(0)
    // Filtrlangandan keyin: XX
}

// parking_lot::Mutex foydalari:
// Преимущества parking_lot::Mutex:
// 1. Poisoning yo'q — unwrap() shart emas
// 2. 2-4x tezroq (kernel space ga tushmaydi)
// 3. try_lock() — Option qaytaradi (Result emas)
// 4. Kichikroq o'lcham

// parking_lot simulyatsiya — wrapper
struct ParkingMutex<T> {
    ichki: Mutex<T>,
}

impl<T> ParkingMutex<T> {
    fn new(val: T) -> Self { ParkingMutex { ichki: Mutex::new(val) } }

    // Poisoning yo'q — har doim guard qaytaradi
    // Без poisoning — всегда возвращает guard
    fn lock(&self) -> std::sync::MutexGuard<'_, T> {
        // Poisoning holatda ham qiymatni qaytaramiz
        // При poisoning тоже возвращаем значение
        self.ichki.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn try_lock(&self) -> Option<std::sync::MutexGuard<'_, T>> {
        self.ichki.try_lock().ok()
    }

    fn into_inner(self) -> T {
        self.ichki.into_inner().unwrap_or_else(|e| e.into_inner())
    }
}

struct ParkingRwLock<T> {
    ichki: RwLock<T>,
}

impl<T> ParkingRwLock<T> {
    fn new(val: T) -> Self { ParkingRwLock { ichki: RwLock::new(val) } }

    fn read(&self) -> std::sync::RwLockReadGuard<'_, T> {
        self.ichki.read().unwrap_or_else(|e| e.into_inner())
    }

    fn write(&self) -> std::sync::RwLockWriteGuard<'_, T> {
        self.ichki.write().unwrap_or_else(|e| e.into_inner())
    }

    fn try_read(&self) -> Option<std::sync::RwLockReadGuard<'_, T>> {
        self.ichki.try_read().ok()
    }

    fn try_write(&self) -> Option<std::sync::RwLockWriteGuard<'_, T>> {
        self.ichki.try_write().ok()
    }
}

fn parking_mutex_misoli() {

    println!("\n--- Parking_lot Mutex Simulyatsiya ---");

    // ParkingMutex — poisoning yo'q
    let mutex = Arc::new(ParkingMutex::new(0i64));
    let mut handlar = vec![];

    for _ in 0..8 {
        let m = Arc::clone(&mutex);
        handlar.push(thread::spawn(move || {
            for _ in 0..10000 {
                let mut g = m.lock(); // unwrap() kerak emas!
                *g += 1;
            }
        }));
    }
    for h in handlar { h.join().unwrap(); }
    println!("Hisob: {}", *mutex.lock()); // 80000

    // try_lock misoli
    let m2 = ParkingMutex::new(42i32);
    let _guard = m2.lock(); // qulflangan
    match m2.try_lock() {
        Some(v) => println!("Olindi: {}", *v),
        None    => println!("Qulflangan — try_lock None"), // bu chiqadi
    }
    // Hisob: 80000
    // Qulflangan — try_lock None

    // Performance taqqoslash
    println!("\n--- Performance: std::Mutex vs ParkingMutex ---");
    let n = 100_000;

    // std::Mutex
    let std_m = Arc::new(Mutex::new(0i64));
    let t1 = Instant::now();
    let mut hs: Vec<_> = (0..4).map(|_| {
        let m = Arc::clone(&std_m);
        thread::spawn(move || {
            for _ in 0..n { *m.lock().unwrap() += 1; }
        })
    }).collect();
    for h in hs { h.join().unwrap(); }
    let vaqt1 = t1.elapsed();

    // ParkingMutex
    let park_m = Arc::new(ParkingMutex::new(0i64));
    let t2 = Instant::now();
    let hs2: Vec<_> = (0..4).map(|_| {
        let m = Arc::clone(&park_m);
        thread::spawn(move || {
            for _ in 0..n { *m.lock() += 1; }
        })
    }).collect();
    for h in hs2 { h.join().unwrap(); }
    let vaqt2 = t2.elapsed();

    println!("std::Mutex:      {:?} ({})", vaqt1, *std_m.lock().unwrap());
    println!("ParkingMutex:    {:?} ({})", vaqt2, *park_m.lock());
}

// ReentrantMutex — bir thread bir nechta marta lock olishi mumkin
// ReentrantMutex — один поток может взять lock несколько раз
// std::sync::Mutex da bu deadlock qiladi!

use std::cell::Cell;

struct ReentrantMutex<T> {
    ichki: Mutex<T>,
    egasi: Mutex<Option<thread::ThreadId>>,
    chuqurlik: Cell<usize>,  // bu xavfli — faqat single-thread test uchun
}

// Sodda ReentrantMutex — haqiqiy parking_lot::ReentrantMutex dan soddaroq
// Упрощённый ReentrantMutex — проще чем настоящий parking_lot::ReentrantMutex
unsafe impl<T: Send> Send for ReentrantMutex<T> {}
unsafe impl<T: Send> Sync for ReentrantMutex<T> {}

impl<T> ReentrantMutex<T> {
    fn new(val: T) -> Self {
        ReentrantMutex {
            ichki: Mutex::new(val),
            egasi: Mutex::new(None),
            chuqurlik: Cell::new(0),
        }
    }
}

fn reentrant_tushuntirish() {

    println!("\n--- ReentrantMutex ---");

    // std::Mutex da deadlock:
    // Дедлок в std::Mutex:
    // let m = Mutex::new(0);
    // let g1 = m.lock().unwrap();
    // let g2 = m.lock().unwrap(); // ← DEADLOCK! g1 hali bor

    // parking_lot::ReentrantMutex bilan:
    // С parking_lot::ReentrantMutex:
    // let m = ReentrantMutex::new(0);
    // let g1 = m.lock();
    // let g2 = m.lock(); // OK! bir xil thread ikki marta olishi mumkin

    // Simulyatsiya — Mutex bilan reentrant pattern
    // Симуляция — паттерн reentrant с Mutex
    let hisob = Arc::new(Mutex::new(0i32));

    fn rekursiv_oshir(hisob: &Mutex<i32>, n: u32) {
        if n == 0 { return; }
        // Har chaqiruvda lock olamiz va tashlaymiz
        {
            let mut h = hisob.lock().unwrap();
            *h += 1;
        } // lock bu yerda tashlanadi
        rekursiv_oshir(hisob, n - 1);
    }

    rekursiv_oshir(&hisob, 5);
    println!("Rekursiv hisob: {}", hisob.lock().unwrap()); // 5
    // Rekursiv hisob: 5

    println!("parking_lot::ReentrantMutex qoidalari:");
    println!("  - Bir thread: bir nechta lock olishi mumkin");
    println!("  - Chuqurlik hisoblanadi — unlock ham shuncha marta");
    println!("  - Turli thread larga hali ham bloklanadi");
}

// Thread-safe kesh — DashMap bilan
// Потокобезопасный кэш — с DashMap
struct Cache<K, V> {
    ichki: Arc<DashMapSim<K, V>>,
    hits: Arc<AtomicUsize>,
    misses: Arc<AtomicUsize>,
}

impl<K, V> Cache<K, V>
where
    K: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn new() -> Self {
        Cache {
            ichki: Arc::new(DashMapSim::new()),
            hits: Arc::new(AtomicUsize::new(0)),
            misses: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn ol_yoki_hisoblash<F: FnOnce() -> V>(&self, kalit: K, hisoblash: F) -> V {
        if let Some(v) = self.ichki.ol(&kalit) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            return v;
        }
        self.misses.fetch_add(1, Ordering::Relaxed);
        let qiymat = hisoblash();
        self.ichki.qo_sh(kalit, qiymat.clone());
        qiymat
    }

    fn statistika(&self) -> (usize, usize, usize) {
        (
            self.ichki.uzunlik(),
            self.hits.load(Ordering::Relaxed),
            self.misses.load(Ordering::Relaxed),
        )
    }
}

impl<K: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static> Clone for Cache<K, V> {
    fn clone(&self) -> Self {
        Cache {
            ichki: Arc::clone(&self.ichki),
            hits: Arc::clone(&self.hits),
            misses: Arc::clone(&self.misses),
        }
    }
}

// So'z chastotasi hisoblagich
// Счётчик частоты слов
struct SozChastotasi {
    ichki: Arc<DashMapSim<String, Arc<AtomicUsize>>>,
}

impl SozChastotasi {
    fn new() -> Self {
        SozChastotasi { ichki: Arc::new(DashMapSim::new()) }
    }

    fn qo_sh(&self, soz: &str) {
        let kalit = soz.to_lowercase();
        if !self.ichki.mavjud(&kalit) {
            self.ichki.qo_sh(kalit.clone(), Arc::new(AtomicUsize::new(0)));
        }
        if let Some(n) = self.ichki.ol(&kalit) {
            n.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn hisob(&self, soz: &str) -> usize {
        self.ichki.ol(&soz.to_lowercase())
            .map(|n| n.load(Ordering::Relaxed))
            .unwrap_or(0)
    }

    fn top_n(&self, n: usize) -> Vec<(String, usize)> {
        let mut barcha: Vec<(String, usize)> = self.ichki.barcha_kalitlar()
            .into_iter()
            .filter_map(|k| {
                self.ichki.ol(&k).map(|v: Arc<AtomicUsize>| (k, v.load(Ordering::Relaxed)))
            })
            .collect();
        barcha.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        barcha.into_iter().take(n).collect()
    }
}

impl Clone for SozChastotasi {
    fn clone(&self) -> Self {
        SozChastotasi { ichki: Arc::clone(&self.ichki) }
    }
}

fn real_hayot_misollari() {

    println!("\n--- Thread-safe Kesh ---");
    let kesh: Cache<String, u64> = Cache::new();

    let mut handlar = vec![];
    for i in 0..4 {
        let k = kesh.clone();
        handlar.push(thread::spawn(move || {
            for j in 0..20 {
                let kalit = format!("key_{}", j % 10); // 10 ta noyob kalit
                k.ol_yoki_hisoblash(kalit, || {
                    // Qimmat hisoblash simulyatsiyasi
                    (i * 100 + j) as u64
                });
            }
        }));
    }
    for h in handlar { h.join().unwrap(); }

    let (o_lcham, hits, misses) = kesh.statistika();
    println!("Kesh o'lcham: {}", o_lcham);
    println!("Hit: {}, Miss: {}", hits, misses);
    println!("Hit ratio: {:.1}%", hits as f64 / (hits + misses) as f64 * 100.0);

    println!("\n--- So'z Chastotasi (Parallel) ---");
    let matnlar = vec![
        "rust tili tez va xavfsiz dasturlash uchun",
        "rust ownership va borrowing modeli ajoyib",
        "concurrent dasturlash rust bilan oson va xavfsiz",
        "dashmap va parking_lot rust ekotizimida mashhur",
        "tez va xavfsiz concurrent rust kutubxonalari",
    ];

    let chastota = SozChastotasi::new();
    let mut handlar = vec![];

    for matn in &matnlar {
        let c = chastota.clone();
        let s = matn.to_string();
        handlar.push(thread::spawn(move || {
            for soz in s.split_whitespace() {
                c.qo_sh(soz);
            }
        }));
    }
    for h in handlar { h.join().unwrap(); }

    println!("Top 5 so'z:");
    for (soz, n) in chastota.top_n(5) {
        println!("  {}: {}", soz, n);
    }
    println!("'rust' chastotasi: {}", chastota.hisob("rust"));
    println!("'va' chastotasi: {}", chastota.hisob("va"));
    // Top 5 so'z:
    //   rust: 4
    //   va: 4
    //   xavfsiz: 3
    //   ...

    println!("\n--- ParkingRwLock Misoli ---");
    let config = Arc::new(ParkingRwLock::new(HashMap::<String, String>::new()));

    // Yozuvchi
    {
        let mut c = config.write();
        c.insert("host".into(), "localhost".into());
        c.insert("port".into(), "8080".into());
    }

    // Ko'p o'quvchi parallel
    let mut readers = vec![];
    for i in 0..4 {
        let cfg = Arc::clone(&config);
        readers.push(thread::spawn(move || {
            let c = cfg.read();
            println!("O'quvchi {}: host={:?}", i, c.get("host"));
        }));
    }
    for r in readers { r.join().unwrap(); }
    // O'quvchi X: host=Some("localhost")
}

fn main() {

    println!("=== DASHMAP SIMULYATSIYA ===");
    dashmap_sim_misoli();

    println!("\n=== PARKING_LOT MUTEX ===");
    parking_mutex_misoli();

    println!("\n=== REENTRANT MUTEX ===");
    reentrant_tushuntirish();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        DASHMAP                                                                                               |
// #================================================================================================================================================#
// # |   1 | DashMap::new()                  | Thread-safe HashMap, o'zi Sync             | Потокобезопасный HashMap, сам Sync                      |
// # |   2 | map.insert(k, v)                | O'chirish — lock kerak emas                | Вставка — без явного lock                               |
// # |   3 | map.get(&k)                     | Guard qaytaradi (shard lock)               | Возвращает Guard (lock шарда)                           |
// # |   4 | map.get_mut(&k)                 | Mutable Guard                              | Мутабельный Guard                                       |
// # |   5 | map.entry(k).or_insert(v)       | entry API — insert yoki olish              | entry API — вставка или получение                       |
// # |   6 | map.retain(\|k, v\| ...)        | Shartli o'chirish                          | Условное удаление                                       |
// # |   7 | Sharding                        | Ko'p shard — parallel kirish               | Много шардов — параллельный доступ                      |
// #================================================================================================================================================#
// # |                                        PARKING_LOT                                                                                           |
// #================================================================================================================================================#
// # |   8 | Mutex::lock()                   | Poisoning yo'q — unwrap() kerak emas       | Нет poisoning — unwrap() не нужен                       |
// # |   9 | Mutex::try_lock()               | Option qaytaradi (std: Result)             | Возвращает Option (std: Result)                         |
// # |  10 | 2-4x tezroq                     | Kernel space ga tushmaslik                 | Без спуска в пространство ядра                          |
// # |  11 | ReentrantMutex                  | Bir thread bir nechta lock                 | Один поток несколько lock                               |
// # |  12 | FairMutex                       | FIFO tartibda beriladi                     | Выдаётся в порядке FIFO                                 |
// # |  13 | Kichik o'lcham                  | std::Mutex dan kichikroq                   | Меньше чем std::Mutex                                   |
// # |  14 | Condvar                         | parking_lot ning o'z Condvar               | Собственный Condvar parking_lot                         |
// #================================================================================================================================================#
// # |                                        QACHON NIMA                                                                                           |
// #================================================================================================================================================#
// # |  15 | DashMap                         | Ko'p thread parallel HashMap kirishi       | Параллельный доступ к HashMap из потоков                |
// # |  16 | parking_lot::Mutex              | Yuqori unumdorlik + poisoning yo'q         | Высокая производительность + нет poisoning              |
// # |  17 | parking_lot::RwLock             | Ko'p o'quvchi + yuqori unumdorlik          | Много читателей + высокая производительность            |
// # |  18 | Arc<DashMap>                    | Global concurrent registry                 | Глобальный конкурентный реестр                          |
// #================================================================================================================================================#