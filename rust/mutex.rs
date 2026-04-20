// #================================================================================================================================================#
// #                                                           MUTEX  |  RWLOCK                                                                     #
// #                    MUTEX VA RWLOCK CHUQUR — POISONING, DEADLOCK, LOCK-FREE ALTERNATIV, TRYLOCK, GUARD PATTERN.                                 #
// #                    MUTEX И RWLOCK ГЛУБОКО — POISONING, DEADLOCK, LOCK-FREE АЛЬТЕРНАТИВЫ, TRYLOCK, ПАТТЕРН GUARD.                               #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;

// Mutex va RwLock:
// Mutex и RwLock:
//
//   Mutex<T>  — bir vaqtda faqat bitta thread kiradi (yozish yoki o'qish)
//   Mutex<T>  — в один момент только один поток (чтение или запись)
//   RwLock<T> — bir vaqtda ko'p o'quvchi YOKI bitta yozuvchi
//   RwLock<T> — много читателей ИЛИ один писатель одновременно
//
//   Mutex qachon:           RwLock qachon:
//   Mutex когда:            RwLock когда:
//   - Ko'p yozish           - Ko'p o'qish, kam yozish
//   - Много записей         - Много чтения, мало записи
//   - Sodda holatlar        - Cache, konfiguratsiya
//   - Простые случаи        - Кэш, конфигурация
//
//   Poisoning — panic bo'lsa Mutex/RwLock zaharlangan deb belgilanadi
//   Poisoning — при панике Mutex/RwLock помечается как отравленный

fn mutex_asosiy_misollari() {

    // Mutex::new — yangi mutex
    // Mutex::new — новый мьютекс
    let mutex: Mutex<i32> = Mutex::new(0);

    // lock() — qulflashtirish, MutexGuard qaytaradi
    // lock() — блокировка, возвращает MutexGuard
    {
        let mut qiymat: MutexGuard<i32> = mutex.lock().unwrap();
        *qiymat += 10;
        println!("{}", *qiymat);
        // Scope tugaganda — guard drop → qulf ochiladi (avtomatik!)
        // При выходе из scope — guard drop → разблокировка (автоматически!)
    }
    // 10

    // Yana kirib olish mumkin
    // Можно войти снова
    println!("{}", mutex.lock().unwrap());
    // 10

    // Arc<Mutex<T>> — thread lar orasida ulashish
    // Arc<Mutex<T>> — совместное использование между потоками
    let shared = Arc::new(Mutex::new(0i64));
    let mut handlar = vec![];

    for _ in 0..10 {
        let s = Arc::clone(&shared);
        handlar.push(thread::spawn(move || {
            for _ in 0..1000 {
                *s.lock().unwrap() += 1;
            }
        }));
    }
    for h in handlar { h.join().unwrap(); }
    println!("Aniq hisob: {}", shared.lock().unwrap()); // har doim 10000
    // Aniq hisob: 10000

    // try_lock() — bloklanmasdan urinib ko'rish
    // try_lock() — попытка без блокирования
    let m = Mutex::new(42i32);
    let _guard = m.lock().unwrap(); // qulflangan

    match m.try_lock() {
        Ok(v)  => println!("Kirish mumkin: {}", *v),
        Err(e) => println!("Qulflangan: {}", e),
    }
    // Qulflangan: try lock failed

    // into_inner() — Mutex ni iste'mol qilib qiymat olish
    // into_inner() — взять значение, поглотив Mutex
    let m2 = Mutex::new(String::from("salom"));
    let s: String = m2.into_inner().unwrap();
    println!("{}", s);
    // salom

    // get_mut() — faqat bitta owner bo'lsa unsafe olmay
    // get_mut() — без unsafe если один владелец
    let mut m3 = Mutex::new(vec![1, 2, 3]);
    m3.get_mut().unwrap().push(4);
    println!("{:?}", m3.into_inner().unwrap());
    // [1, 2, 3, 4]
}

// Poisoning — mutex qulflangan holda panic bo'lsa
// Poisoning — если паника пока mutex заблокирован
fn poisoning_misollari() {

    let mutex = Arc::new(Mutex::new(0i32));
    let m = Arc::clone(&mutex);

    // Panic bo'lgan thread — mutex zaharlanadi
    // Поток с паникой — мьютекс становится отравленным
    let xatoli_thread = thread::spawn(move || {
        let _guard = m.lock().unwrap();
        panic!("Guard qulflangan holda panic!");
    });

    let _ = xatoli_thread.join(); // Err qaytaradi

    // lock() — PoisonError qaytaradi
    // lock() — возвращает PoisonError
    match mutex.lock() {
        Ok(v)  => println!("Sog'lom: {}", v),
        Err(e) => {
            println!("Zaharlanган mutex: {}", e);
            // into_inner() — zaharlangan qiymatni olish
            let v = e.into_inner();
            println!("Qiymat hali bor: {}", v);
        }
    }
    // Zaharlanган mutex: poisoned lock: another task failed while holding the lock
    // Qiymat hali bor: 0

    // is_poisoned() — tekshirish
    // is_poisoned() — проверка
    println!("Zaharlandimi? {}", mutex.is_poisoned());
    // Zaharlandimi? true

    // Poisoning ni e'tiborsiz qoldirish
    // Игнорирование poisoning
    let qiymat = mutex.lock().unwrap_or_else(|e| e.into_inner());
    println!("Yutib yuborildi: {}", qiymat);
    // Yutib yuborildi: 0

    // clear_poison() — zaharlashni tozalash (Rust 1.77+)
    // clear_poison() — очистка отравления (Rust 1.77+)
    mutex.clear_poison();
    println!("Zaharlandimi (tozalangandan keyin)? {}", mutex.is_poisoned());
    // Zaharlandimi (tozalangandan keyin)? false
}

fn rwlock_misollari() {

    let rw: RwLock<Vec<String>> = RwLock::new(vec![]);

    // write() — yozish uchun qulflash
    // write() — блокировка для записи
    {
        let mut w: RwLockWriteGuard<Vec<String>> = rw.write().unwrap();
        w.push("birinchi".to_string());
        w.push("ikkinchi".to_string());
    }

    // read() — o'qish uchun (bir nechta bir vaqtda)
    // read() — для чтения (несколько одновременно)
    {
        let r1: RwLockReadGuard<Vec<String>> = rw.read().unwrap();
        let r2: RwLockReadGuard<Vec<String>> = rw.read().unwrap(); // bir vaqtda!
        println!("{:?}", *r1);
        println!("{:?}", *r2);
    }
    // ["birinchi", "ikkinchi"]
    // ["birinchi", "ikkinchi"]

    // Arc<RwLock<T>> — thread lar orasida
    // Arc<RwLock<T>> — между потоками
    let kesh = Arc::new(RwLock::new(HashMap::<String, i32>::new()));
    let mut handlar = vec![];

    // 3 ta yozuvchi
    for i in 0..3 {
        let k = Arc::clone(&kesh);
        handlar.push(thread::spawn(move || {
            let mut w = k.write().unwrap();
            w.insert(format!("kalit_{}", i), i * 10);
        }));
    }

    // 5 ta o'quvchi (yozuvchilar tugagandan keyin)
    for h in handlar { h.join().unwrap(); }

    let mut o_quvchilar = vec![];
    for i in 0..5 {
        let k = Arc::clone(&kesh);
        o_quvchilar.push(thread::spawn(move || {
            let r = k.read().unwrap();
            println!("O'quvchi {}: {} ta yozuv", i, r.len());
        }));
    }
    for h in o_quvchilar { h.join().unwrap(); }
    // O'quvchi X: 3 ta yozuv

    // try_read() va try_write()
    // try_read() и try_write()
    let rw2 = RwLock::new(0i32);
    let _r = rw2.read().unwrap();

    match rw2.try_write() {
        Ok(_)  => println!("Yozish mumkin"),
        Err(e) => println!("Yozish mumkin emas: {}", e),
    }
    // Yozish mumkin emas: try lock failed

    // into_inner()
    let rw3 = RwLock::new(vec![1, 2, 3]);
    let v: Vec<i32> = rw3.into_inner().unwrap();
    println!("{:?}", v);
    // [1, 2, 3]
}

fn deadlock_tushuntirish() {

    // DEADLOCK — ikki thread bir-birini kutadi
    // DEADLOCK — два потока ожидают друг друга
    //
    // Thread A:          Thread B:
    //   lock(M1)           lock(M2)
    //   lock(M2) ← kutish  lock(M1) ← kutish
    //
    // Yechim 1: Har doim bir xil tartibda qulflash
    // Решение 1: Всегда блокировать в одном порядке

    let m1 = Arc::new(Mutex::new(1i32));
    let m2 = Arc::new(Mutex::new(2i32));

    // XAVFSIZ: ikkalasi ham m1→m2 tartibida
    // БЕЗОПАСНО: оба блокируют в порядке m1→m2
    let m1a = Arc::clone(&m1);
    let m2a = Arc::clone(&m2);

    let h1 = thread::spawn(move || {
        let _g1 = m1a.lock().unwrap(); // avval m1
        thread::sleep(Duration::from_millis(5));
        let _g2 = m2a.lock().unwrap(); // keyin m2
        println!("Thread A: har ikkisini oldi");
    });

    let m1b = Arc::clone(&m1);
    let m2b = Arc::clone(&m2);

    let h2 = thread::spawn(move || {
        let _g1 = m1b.lock().unwrap(); // avval m1 (bir xil tartib!)
        let _g2 = m2b.lock().unwrap(); // keyin m2
        println!("Thread B: har ikkisini oldi");
    });

    h1.join().unwrap();
    h2.join().unwrap();
    // Thread A: har ikkisini oldi
    // Thread B: har ikkisini oldi

    // Yechim 2: try_lock + timeout
    // Решение 2: try_lock + timeout
    let m3 = Arc::new(Mutex::new(0i32));
    let m3a = Arc::clone(&m3);

    let h3 = thread::spawn(move || {
        let boshlanish = Instant::now();
        loop {
            if let Ok(mut g) = m3a.try_lock() {
                *g += 1;
                return;
            }
            if boshlanish.elapsed() > Duration::from_millis(100) {
                println!("Timeout — qulflash muvaffaqiyatsiz");
                return;
            }
            thread::yield_now();
        }
    });
    h3.join().unwrap();
    println!("Deadlock yechimi: {}", m3.lock().unwrap());
    // Deadlock yechimi: 1

    // Yechim 3: qulfni imkon boricha tezroq qo'yib yuborish
    // Решение 3: освобождать блокировку как можно скорее
    let m4 = Mutex::new(vec![1, 2, 3]);

    // YOMON: uzoq vaqt qulflangan
    // ПЛОХО: заблокировано долго
    // {
    //     let v = m4.lock().unwrap();
    //     uzun_operatsiya(); // ← qulf ushlab turadi!
    //     println!("{:?}", *v);
    // }

    // YAXSHI: qiymatni ko'chirib olib, qulfni tezda qo'yib yuborish
    // ХОРОШО: скопировать значение и сразу освободить блокировку
    let snapshot: Vec<i32> = m4.lock().unwrap().clone();
    // ← qulf shu yerda qo'yib yuborildi
    println!("{:?}", snapshot); // qulfsiz ishlash
    // [1, 2, 3]
}

// Mutex Guard ni struct da saqlash
// Хранение Mutex Guard в структуре
struct QulflashBoshqaruvchi<'a, T> {
    guard: MutexGuard<'a, T>,
    operatsiya_nomi: &'static str,
}

impl<'a, T: std::fmt::Debug> QulflashBoshqaruvchi<'a, T> {
    fn new(mutex: &'a Mutex<T>, nomi: &'static str) -> Self {
        println!("[{}] Qulflandi", nomi);
        QulflashBoshqaruvchi {
            guard: mutex.lock().unwrap(),
            operatsiya_nomi: nomi,
        }
    }

    fn qiymat(&self) -> &T {
        &self.guard
    }

    fn qiymat_mut(&mut self) -> &mut T {
        &mut self.guard
    }
}

impl<'a, T> Drop for QulflashBoshqaruvchi<'a, T> {
    fn drop(&mut self) {
        println!("[{}] Qulf ochildi", self.operatsiya_nomi);
    }
}

// Kesh — RwLock bilan
// Кэш — с RwLock
struct Kesh<K, V> {
    ichki: RwLock<HashMap<K, V>>,
    sig_im: usize,
}

impl<K: std::hash::Hash + Eq + Clone, V: Clone> Kesh<K, V> {
    fn new(sig_im: usize) -> Self {
        Kesh { ichki: RwLock::new(HashMap::new()), sig_im }
    }

    fn ol(&self, kalit: &K) -> Option<V> {
        self.ichki.read().unwrap().get(kalit).cloned()
    }

    fn qo_sh(&self, kalit: K, qiymat: V) -> bool {
        let mut w = self.ichki.write().unwrap();
        if w.len() >= self.sig_im { return false; }
        w.insert(kalit, qiymat);
        true
    }

    fn uzunlik(&self) -> usize {
        self.ichki.read().unwrap().len()
    }

    fn barcha(&self) -> Vec<(K, V)> {
        self.ichki.read().unwrap()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

fn guard_pattern_misollari() {

    // QulflashBoshqaruvchi
    let mutex = Mutex::new(vec![1, 2, 3]);
    {
        let mut bq = QulflashBoshqaruvchi::new(&mutex, "ma'lumot_qayta_ishlash");
        bq.qiymat_mut().push(4);
        println!("{:?}", bq.qiymat());
    } // ← avtomatik qulf ochildi
    // [ma'lumot_qayta_ishlash] Qulflandi
    // [1, 2, 3, 4]
    // [ma'lumot_qayta_ishlash] Qulf ochildi

    // Kesh — thread-safe
    let kesh = Arc::new(Kesh::new(100));
    let mut handlar = vec![];

    for i in 0..5 {
        let k = Arc::clone(&kesh);
        handlar.push(thread::spawn(move || {
            k.qo_sh(format!("kalit_{}", i), i * 100);
        }));
    }
    for h in handlar { h.join().unwrap(); }

    println!("Kesh uzunlik: {}", kesh.uzunlik());
    println!("{:?}", kesh.ol(&"kalit_2".to_string()));
    // Kesh uzunlik: 5
    // Some(200)
}

// Rate limiter — Mutex bilan
// Rate limiter — с Mutex
struct RateLimiter {
    so_rovlar: Mutex<Vec<Instant>>,
    oyna_ms: u64,
    limit: usize,
}

impl RateLimiter {
    fn new(limit: usize, oyna_ms: u64) -> Self {
        RateLimiter {
            so_rovlar: Mutex::new(vec![]),
            oyna_ms,
            limit,
        }
    }

    fn ruxsat_berish(&self) -> bool {
        let mut so_rovlar = self.so_rovlar.lock().unwrap();
        let hozir = Instant::now();
        let chegara = hozir - Duration::from_millis(self.oyna_ms);

        // Eski so'rovlarni o'chirish
        so_rovlar.retain(|&t| t > chegara);

        if so_rovlar.len() < self.limit {
            so_rovlar.push(hozir);
            true
        } else {
            false
        }
    }
}

// Config — RwLock bilan
// Config — с RwLock
struct Config {
    ichki: RwLock<HashMap<String, String>>,
}

impl Config {
    fn new() -> Self {
        Config { ichki: RwLock::new(HashMap::new()) }
    }

    fn o_rnat(&self, kalit: &str, qiymat: &str) {
        self.ichki.write().unwrap()
            .insert(kalit.to_string(), qiymat.to_string());
    }

    fn ol(&self, kalit: &str) -> Option<String> {
        self.ichki.read().unwrap().get(kalit).cloned()
    }

    fn yangilash(&self, o_zgarishlar: HashMap<String, String>) {
        let mut w = self.ichki.write().unwrap();
        for (k, v) in o_zgarishlar {
            w.insert(k, v);
        }
    }
}

fn real_hayot_misollari() {

    // Rate limiter
    let limiter = Arc::new(RateLimiter::new(3, 100));
    let mut natijalar = vec![];

    for i in 0..6 {
        let l = Arc::clone(&limiter);
        let ruxsat = l.ruxsat_berish();
        natijalar.push(ruxsat);
        println!("So'rov {}: {}", i + 1, if ruxsat { "✅ ruxsat" } else { "❌ rad" });
    }
    // So'rov 1: ✅ ruxsat
    // So'rov 2: ✅ ruxsat
    // So'rov 3: ✅ ruxsat
    // So'rov 4: ❌ rad
    // So'rov 5: ❌ rad
    // So'rov 6: ❌ rad

    // Config — thread-safe
    let cfg = Arc::new(Config::new());
    cfg.o_rnat("host", "localhost");
    cfg.o_rnat("port", "8080");

    let cfg2 = Arc::clone(&cfg);
    let h = thread::spawn(move || {
        let mut o_zgarishlar = HashMap::new();
        o_zgarishlar.insert("debug".to_string(), "true".to_string());
        o_zgarishlar.insert("workers".to_string(), "4".to_string());
        cfg2.yangilash(o_zgarishlar);
    });
    h.join().unwrap();

    println!("host: {:?}", cfg.ol("host"));
    println!("debug: {:?}", cfg.ol("debug"));
    // host: Some("localhost")
    // debug: Some("true")

    // Mutex Poisoning
    println!("\n--- Poisoning ---");
    poisoning_misollari();

    // Deadlock
    println!("\n--- Deadlock yechimi ---");
    deadlock_tushuntirish();
}

fn main() {

    println!("=== MUTEX ASOSIY ===");
    mutex_asosiy_misollari();

    println!("\n=== RWLOCK ===");
    rwlock_misollari();

    println!("\n=== GUARD PATTERN ===");
    guard_pattern_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        MUTEX                                                                                                 |
// #================================================================================================================================================#
// # |   1 | Mutex::new(val)                 | Yangi mutex                                | Новый мьютекс                                           |
// # |   2 | .lock()                         | Qulflash, MutexGuard qaytaradi             | Блокировка, возвращает MutexGuard                       |
// # |   3 | .try_lock()                     | Bloklanmasdan urinish                      | Попытка без блокирования                                |
// # |   4 | MutexGuard drop                 | Scope tugaganda avtomatik ochiladi         | Автоматически открывается в конце scope                 |
// # |   5 | .into_inner()                   | Mutex ni iste'mol qilib qiymat olish       | Взять значение, поглотив Mutex                          |
// # |   6 | .get_mut()                      | Bitta owner bo'lsa xavfsiz mut ref         | Безопасный mut ref при одном владельце                  |
// # |   7 | Poisoning                       | Panic bo'lsa mutex zaharlanadi             | При панике мьютекс отравляется                          |
// # |   8 | .is_poisoned()                  | Zaharlanganligini tekshirish               | Проверка отравления                                     |
// # |   9 | .clear_poison()                 | Zaharlashni tozalash (1.77+)               | Очистка отравления (1.77+)                              |
// #================================================================================================================================================#
// # |                                        RWLOCK                                                                                                |
// #================================================================================================================================================#
// # |  10 | RwLock::new(val)                | Yangi rwlock                               | Новый rwlock                                            |
// # |  11 | .read()                         | O'qish uchun, bir nechta bir vaqtda        | Для чтения, несколько одновременно                      |
// # |  12 | .write()                        | Yozish uchun, faqat bitta                  | Для записи, только один                                 |
// # |  13 | .try_read() / .try_write()      | Bloklanmasdan urinish                      | Попытка без блокирования                                |
// # |  14 | Qachon: ko'p o'qish             | Mutex dan samarali: ko'p reader            | Эффективнее Mutex: много читателей                      |
// #================================================================================================================================================#
// # |                                        DEADLOCK VA PATTERNLAR                                                                                |
// #================================================================================================================================================#
// # |  15 | Bir xil tartibda qulflash       | Deadlock oldini olish                      | Предотвращение deadlock                                 |
// # |  16 | try_lock + timeout              | Deadlock yechimi                           | Решение deadlock                                        |
// # |  17 | Tez qo'yib yuborish             | Qulfni imkon boricha qisqa saqlash         | Держать блокировку как можно меньше                     |
// # |  18 | Guard pattern                   | Mutex guard ni struct da saqlash           | Хранение guard в структуре                              |
// # |  19 | Snapshot pattern                | Qiymatni ko'chirib qulfsiz ishlash         | Работа без блокировки через копию                       |
// # |  20 | Rate limiter                    | Mutex bilan tezlikni cheklash              | Ограничение скорости через Mutex                        |
// #================================================================================================================================================#