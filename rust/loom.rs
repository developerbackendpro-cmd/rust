// #================================================================================================================================================#
// #                                                                    LOOM + MIRI                                                                 #
// #                                    LOOM — CONCURRENT KOD TEKSHIRUVI. MIRI — UNDEFINED BEHAVIOR TOPISH. SANITIZER.                              #
// #                                    LOOM — ПРОВЕРКА КОНКУРЕНТНОГО КОДА. MIRI — ПОИСК UNDEFINED BEHAVIOR. SANITIZER.                             #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::sync::{Arc, Mutex, atomic::{AtomicUsize, AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use std::cell::UnsafeCell;

// LOOM nima:
// Что такое LOOM:
//
//   Concurrent Rust kodini sistematik tekshirish
//   Систематическая проверка конкурентного кода Rust
//   Barcha mumkin bo'lgan thread interleaving larni sinovdan o'tkazadi
//   Проверяет все возможные чередования потоков
//
//   cargo.toml:
//   [dev-dependencies]
//   loom = "0.7"
//
//   Ishlatish:
//   Использование:
//   LOOM_MAX_BRANCHES=100 cargo test --test concurrent_test
//
// MIRI nima:
// Что такое MIRI:
//
//   Rust MIR interpritatori — Undefined Behavior topadi
//   Интерпретатор Rust MIR — находит Undefined Behavior
//
//   cargo +nightly miri run
//   cargo +nightly miri test
//
//   Nima topadi:
//   Что находит:
//   - Use after free
//   - Dangling pointer
//   - Memory leak (ba'zan)
//   - Uninitialized memory
//   - Race condition (ba'zi holatlarda)
//   - Invalid enum discriminant
//   - Stack overflow

fn loom_haqiqiy_kod() {

    println!("=== LOOM HAQIQIY KOD ===");
    println!();

    println!(r#"// tests/concurrent_test.rs
use loom::sync::{{Arc, Mutex}};
use loom::thread;

// LOOM test: Mutex bilan counter
#[test]
fn test_mutex_counter() {{
    loom::model(|| {{
        let hisob = Arc::new(Mutex::new(0));
        let mut handlar = vec![];

        for _ in 0..2 {{
            let h = Arc::clone(&hisob);
            handlar.push(thread::spawn(move || {{
                let mut guard = h.lock().unwrap();
                *guard += 1;
            }}));
        }}

        for h in handlar {{ h.join().unwrap(); }}
        assert_eq!(*hisob.lock().unwrap(), 2);
    }});
}}

// LOOM test: AtomicUsize
use loom::sync::atomic::{{AtomicUsize, Ordering}};

#[test]
fn test_atomic_counter() {{
    loom::model(|| {{
        let hisob = Arc::new(AtomicUsize::new(0));
        let h1 = Arc::clone(&hisob);
        let h2 = Arc::clone(&hisob);

        let t1 = thread::spawn(move || {{
            h1.fetch_add(1, Ordering::SeqCst);
        }});

        let t2 = thread::spawn(move || {{
            h2.fetch_add(1, Ordering::SeqCst);
        }});

        t1.join().unwrap();
        t2.join().unwrap();

        // Barcha interleaving larda 2 bo'lishi kerak
        assert_eq!(hisob.load(Ordering::SeqCst), 2);
    }});
}}

// LOOM test: Lock-free queue
use loom::sync::{{Arc, atomic::*}};
use std::ptr;

struct LockFreeQueue<T> {{
    bosh: AtomicPtr<Node<T>>,
    oxir: AtomicPtr<Node<T>>,
}}

// ... implementatsiya ...

#[test]
fn test_lock_free_queue() {{
    loom::model(|| {{
        let q = Arc::new(LockFreeQueue::new());
        let q2 = Arc::clone(&q);

        let producer = thread::spawn(move || {{
            q.push(42);
        }});

        let consumer = thread::spawn(move || {{
            q2.pop()
        }});

        producer.join().unwrap();
        let natija = consumer.join().unwrap();
        // Natija 42 yoki None bo'lishi mumkin (ordering ga qarab)
    }});
}}"#);

    println!();
    println!("loom afzalliklari:");
    println!("  ✅ Barcha interleaving lar sinovdan o'tadi");
    println!("  ✅ Data race ni topadi");
    println!("  ✅ Deadlock ni topadi");
    println!("  ✅ Weak memory ordering xatolarini topadi");
    println!("  ⚠️  Sekin — production kodda emas, faqat testda");
}

fn miri_haqiqiy_kod() {

    println!("\n=== MIRI HAQIQIY KOD ===");
    println!();

    println!(r#"// Miri bilan tekshirish:
// cargo +nightly miri run
// cargo +nightly miri test

// 1. USE AFTER FREE — Miri topadi
fn use_after_free_misol() {{
    let ptr: *mut i32;
    {{
        let mut n = 42;
        ptr = &mut n as *mut i32;
    }} // n bu yerda drop!

    // MIRI: error: use-after-free!
    unsafe {{ *ptr = 100; }}
}}

// 2. UNINITIALIZED MEMORY — Miri topadi
fn uninit_misol() {{
    let n: i32;
    // MIRI: error: using uninitialized data
    println!("{{}}", n); // UB!
}}

// 3. OUT OF BOUNDS — Miri topadi
fn oob_misol() {{
    let v = vec![1, 2, 3];
    let ptr = v.as_ptr();
    // MIRI: error: out-of-bounds pointer arithmetic
    unsafe {{ *ptr.add(10) }}; // UB!
}}

// 4. INVALID TRANSMUTE — Miri topadi
fn invalid_transmute() {{
    let b: bool = unsafe {{
        // MIRI: error: invalid value: 42 is not a valid bool
        std::mem::transmute::<u8, bool>(42)
    }};
}}

// 5. DATA RACE — Miri ba'zan topadi
fn data_race() {{
    let mut n = 0;
    let ptr: *mut i32 = &mut n;

    std::thread::spawn(move || {{
        // MIRI: error: data race detected
        unsafe {{ *ptr = 1; }}
    }});
    n = 2; // Race!
}}

// Miri bilan muvaffaqiyatli o'tadigan xavfsiz kod:
fn xavfsiz_unsafe() {{
    let mut v: Vec<i32> = Vec::with_capacity(5);
    unsafe {{
        // MIRI: OK — ptr valid, bounds OK
        v.set_len(3);
        *v.as_mut_ptr() = 1;
        *v.as_mut_ptr().add(1) = 2;
        *v.as_mut_ptr().add(2) = 3;
    }}
    assert_eq!(v, vec![1, 2, 3]);
}}"#);

    println!();
    println!("Miri afzalliklari:");
    println!("  ✅ Use after free topadi");
    println!("  ✅ Uninitialized memory topadi");
    println!("  ✅ Out of bounds topadi");
    println!("  ✅ Invalid type transmute topadi");
    println!("  ✅ Alignment xatolar topadi");
    println!("  ⚠️  Juda sekin (~100x slower)");
    println!("  ⚠️  Nightly Rust kerak");
}

// Umumiy concurrent xatolar — std bilan namoyon qilish
// Общие конкурентные ошибки — демонстрация с std

// 1. DATA RACE SIMULYATSIYA — noto'g'ri kod
fn data_race_simulyatsiya() {

    println!("\n--- Data Race Simulyatsiya ---");
    println!("(Bu xato Mutex bilan to'g'rilanadi)");

    // XATO: atomic olmay raw pointer bilan
    // ОШИБКА: с сырым указателем без atomic

    // To'G'RI versiya — Mutex bilan:
    let hisob = Arc::new(Mutex::new(0u64));
    let mut handlar = vec![];

    for _ in 0..8 {
        let h = Arc::clone(&hisob);
        handlar.push(thread::spawn(move || {
            for _ in 0..10000 {
                *h.lock().unwrap() += 1;
            }
        }));
    }

    for h in handlar { h.join().unwrap(); }
    let natija = *hisob.lock().unwrap();
    println!("Mutex bilan to'g'ri natija: {}", natija); // 80000
    // 80000

    // AtomicUsize bilan:
    let atomic_hisob = Arc::new(AtomicUsize::new(0));
    let mut handlar2 = vec![];

    for _ in 0..8 {
        let h = Arc::clone(&atomic_hisob);
        handlar2.push(thread::spawn(move || {
            for _ in 0..10000 {
                h.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for h in handlar2 { h.join().unwrap(); }
    println!("Atomic bilan to'g'ri natija: {}",
             atomic_hisob.load(Ordering::SeqCst)); // 80000
    // 80000
}

// 2. DEADLOCK SIMULYATSIYA VA OLDINI OLISH
fn deadlock_oldini_olish() {

    println!("\n--- Deadlock Oldini Olish ---");

    // DEADLOCK: tartibsiz lock
    // ДЕДЛОК: блокировка в произвольном порядке
    /*
    let m1 = Arc::new(Mutex::new(1));
    let m2 = Arc::new(Mutex::new(2));

    // Thread 1: m1 → m2
    // Thread 2: m2 → m1
    // DEADLOCK!
    */

    // TO'G'RI: bir xil tartibda lock
    // ПРАВИЛЬНО: блокировка в одном порядке
    let m1 = Arc::new(Mutex::new(1i32));
    let m2 = Arc::new(Mutex::new(2i32));

    let m1_a = Arc::clone(&m1);
    let m2_a = Arc::clone(&m2);
    let t1 = thread::spawn(move || {
        let g1 = m1_a.lock().unwrap(); // Birinchi m1
        thread::sleep(Duration::from_millis(1));
        let g2 = m2_a.lock().unwrap(); // Keyin m2
        println!("T1: {} + {}", *g1, *g2);
    });

    let m1_b = Arc::clone(&m1);
    let m2_b = Arc::clone(&m2);
    let t2 = thread::spawn(move || {
        let g1 = m1_b.lock().unwrap(); // Birinchi m1 (bir xil tartib!)
        thread::sleep(Duration::from_millis(1));
        let g2 = m2_b.lock().unwrap(); // Keyin m2
        println!("T2: {} + {}", *g1, *g2);
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("Deadlock bo'lmadi ✅");
    // T1: 1 + 2 (yoki T2 birinchi)
    // Deadlock bo'lmadi ✅

    // try_lock — deadlock oldini olish
    // try_lock — предотвращение дедлока
    let m3 = Mutex::new(42i32);
    match m3.try_lock() {
        Ok(guard)  => println!("try_lock OK: {}", *guard),
        Err(_)     => println!("try_lock: Qulflangan!"),
    }
    // try_lock OK: 42
}

// 3. MEMORY ORDERING TEKSHIRUVI
fn memory_ordering_misoli() {

    println!("\n--- Memory Ordering ---");

    // Relaxed — hech qanday kafolat yo'q
    // Relaxed — нет никаких гарантий
    let relaxed = Arc::new(AtomicUsize::new(0));
    let r2 = Arc::clone(&relaxed);
    let t = thread::spawn(move || {
        for i in 0..100 { r2.fetch_add(1, Ordering::Relaxed); }
    });
    for i in 0..100 { relaxed.fetch_add(1, Ordering::Relaxed); }
    t.join().unwrap();
    // Hisob 200 bo'ladi, lekin ordering kafolat yo'q
    println!("Relaxed hisob: {}", relaxed.load(Ordering::SeqCst));

    // SeqCst — eng qattiq kafolat
    // SeqCst — самая строгая гарантия
    let flag = Arc::new(AtomicBool::new(false));
    let data = Arc::new(Mutex::new(0i32));

    let f2 = Arc::clone(&flag);
    let d2 = Arc::clone(&data);
    let producer = thread::spawn(move || {
        *d2.lock().unwrap() = 42;
        f2.store(true, Ordering::SeqCst); // Publish
    });

    let f3 = Arc::clone(&flag);
    let d3 = Arc::clone(&data);
    let consumer = thread::spawn(move || {
        while !f3.load(Ordering::SeqCst) {
            thread::yield_now();
        }
        let val = *d3.lock().unwrap(); // Guaranteed to see 42
        println!("SeqCst: ma'lumot = {}", val);
    });

    producer.join().unwrap();
    consumer.join().unwrap();
    // SeqCst: ma'lumot = 42
}

fn sanitizerlar_tushuntirish() {

    println!("\n=== SANITIZERLAR ===");
    println!();

    println!("Rust sanitizerlari (nightly):
");

    println!("1. ADDRESS SANITIZER (ASan):");
    println!("   RUSTFLAGS=\"-Z sanitizer=address\" cargo +nightly run");
    println!("   - Use after free");
    println!("   - Buffer overflow");
    println!("   - Stack overflow");
    println!("   - Heap overflow");
    println!();

    println!("2. THREAD SANITIZER (TSan):");
    println!("   RUSTFLAGS=\"-Z sanitizer=thread\" cargo +nightly run");
    println!("   - Data race topadi");
    println!("   - Lock ordering xatolar");
    println!("   - Loom dan tezroq, lekin to'liq emas");
    println!();

    println!("3. MEMORY SANITIZER (MSan):");
    println!("   RUSTFLAGS=\"-Z sanitizer=memory\" cargo +nightly run");
    println!("   - Uninitialized memory o'qish");
    println!("   - Linux x86_64 da ishlaydi");
    println!();

    println!("4. LEAK SANITIZER (LSan):");
    println!("   RUSTFLAGS=\"-Z sanitizer=leak\" cargo +nightly run");
    println!("   - Xotira sizishi (memory leak)");
    println!("   - Box, Vec, String sizmasa ham");
    println!();

    println!("5. HARDWARE ADDRESS SANITIZER (HWASan):");
    println!("   - AArch64 da hardware-accelerated ASan");
    println!("   - Android/ARM server da");
}

// Xavfsiz concurrent stack implementatsiyasi
// Безопасная реализация конкурентного стека
struct ConcurrentStack<T> {
    ichki: Mutex<Vec<T>>,
    o_lcham: AtomicUsize,
}

impl<T: Send> ConcurrentStack<T> {
    fn new() -> Self {
        ConcurrentStack {
            ichki: Mutex::new(Vec::new()),
            o_lcham: AtomicUsize::new(0),
        }
    }

    fn push(&self, val: T) {
        self.ichki.lock().unwrap().push(val);
        self.o_lcham.fetch_add(1, Ordering::Release);
    }

    fn pop(&self) -> Option<T> {
        let mut guard = self.ichki.lock().unwrap();
        let val = guard.pop();
        if val.is_some() {
            self.o_lcham.fetch_sub(1, Ordering::Release);
        }
        val
    }

    fn uzunlik(&self) -> usize {
        self.o_lcham.load(Ordering::Acquire)
    }

    fn boshmi(&self) -> bool { self.uzunlik() == 0 }
}

fn concurrent_stack_test() {

    println!("\n--- ConcurrentStack Test ---");
    let stack = Arc::new(ConcurrentStack::new());

    // Ko'p producer
    let mut producer_lar = vec![];
    for i in 0..4 {
        let s = Arc::clone(&stack);
        producer_lar.push(thread::spawn(move || {
            for j in 0..25 {
                s.push(i * 25 + j);
            }
        }));
    }
    for p in producer_lar { p.join().unwrap(); }
    println!("Jami push: {}", stack.uzunlik()); // 100

    // Ko'p consumer
    let pop_soni = Arc::new(AtomicUsize::new(0));
    let mut consumer_lar = vec![];
    for _ in 0..4 {
        let s = Arc::clone(&stack);
        let cnt = Arc::clone(&pop_soni);
        consumer_lar.push(thread::spawn(move || {
            let mut lokal = 0;
            while let Some(_) = s.pop() { lokal += 1; }
            cnt.fetch_add(lokal, Ordering::Relaxed);
        }));
    }
    for c in consumer_lar { c.join().unwrap(); }

    println!("Jami pop: {}", pop_soni.load(Ordering::SeqCst)); // 100
    println!("Stack bo'sh: {}", stack.boshmi()); // true
    // Jami push: 100
    // Jami pop: 100
    // Stack bo'sh: true
}

// Loom bilan test yozish namunasi
fn loom_test_namunasi() {

    println!("\n=== LOOM TEST NAMUNASI ===");
    println!(r#"
// tests/loom_test.rs

#[cfg(loom)]
mod tests {{
    use loom::sync::{{Arc, Mutex}};
    use loom::sync::atomic::{{AtomicUsize, Ordering}};
    use loom::thread;

    // ConcurrentStack loom testi
    #[test]
    fn stack_concurrent_test() {{
        loom::model(|| {{
            let stack = Arc::new(ConcurrentStack::new());
            let s1 = Arc::clone(&stack);
            let s2 = Arc::clone(&stack);

            // 2 ta thread push qiladi
            let t1 = thread::spawn(move || {{ s1.push(1); }});
            let t2 = thread::spawn(move || {{ s2.push(2); }});

            t1.join().unwrap();
            t2.join().unwrap();

            // Barcha interleaving larda 2 ta element bo'lishi kerak
            assert_eq!(stack.uzunlik(), 2);
        }});
    }}

    // SeqLock loom testi
    #[test]
    fn seqlock_test() {{
        loom::model(|| {{
            let lock = Arc::new(SeqLock::new(0));
            let l1 = Arc::clone(&lock);
            let l2 = Arc::clone(&lock);

            let writer = thread::spawn(move || {{
                l1.write(42);
            }});

            let reader = thread::spawn(move || {{
                let val = l2.read();
                // 0 yoki 42 — lekin hech qachon o'rta holat emas
                assert!(val == 0 || val == 42);
            }});

            writer.join().unwrap();
            reader.join().unwrap();
        }});
    }}
}}

// Ishlatish:
// RUSTFLAGS="--cfg loom" cargo test --test loom_test"#);
}

fn main() {

    loom_haqiqiy_kod();
    miri_haqiqiy_kod();
    data_race_simulyatsiya();
    deadlock_oldini_olish();
    memory_ordering_misoli();
    sanitizerlar_tushuntirish();
    concurrent_stack_test();
    loom_test_namunasi();

    println!("\n=== XULOSA ===");
    println!("Concurrent kod tekshirish vositalari:");
    println!("  loom    — barcha interleaving, deterministic");
    println!("  Miri    — UB, uninitialized, use-after-free");
    println!("  TSan    — data race, runtime tekshiruv");
    println!("  ASan    — buffer overflow, use-after-free");
    println!("  LSan    — memory leak");
    println!();
    println!("Ishlatish tartibi:");
    println!("  1. cargo test          — oddiy testlar");
    println!("  2. cargo miri test     — UB tekshiruvi");
    println!("  3. LOOM testlar        — concurrent kafolat");
    println!("  4. TSan bilan run      — data race");
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        LOOM                                                                                                  |
// #================================================================================================================================================#
// # |   1 | loom::model(|| { ... })         | Barcha interleaving larni sinash           | Проверка всех чередований                               |
// # |   2 | loom::sync::Mutex               | Loom Mutex (std o'rniga)                   | Loom Mutex (вместо std)                                 |
// # |   3 | loom::sync::atomic::*           | Loom atomics                               | Loom атомики                                            |
// # |   4 | loom::thread::spawn             | Loom thread (std o'rniga)                  | Loom поток (вместо std)                                 |
// # |   5 | #[cfg(loom)]                    | Faqat loom testda                          | Только в loom тесте                                     |
// #================================================================================================================================================#
// # |                                        MIRI                                                                                                  |
// #================================================================================================================================================#
// # |   6 | cargo +nightly miri run         | Miri bilan ishga tushirish                 | Запуск с Miri                                           |
// # |   7 | cargo +nightly miri test        | Miri bilan test                            | Тест с Miri                                             |
// # |   8 | Use after free                  | Miri topadi                                | Miri обнаруживает                                       |
// # |   9 | Uninitialized memory            | Miri topadi                                | Miri обнаруживает                                       |
// # |  10 | Invalid transmute               | Miri topadi                                | Miri обнаруживает                                       |
// #================================================================================================================================================#
// # |                                        SANITIZERLAR                                                                                          |
// #================================================================================================================================================#
// # |  11 | -Z sanitizer=address            | Buffer overflow, use-after-free            | Переполнение буфера, use-after-free                     |
// # |  12 | -Z sanitizer=thread             | Data race                                  | Гонка данных                                            |
// # |  13 | -Z sanitizer=memory             | Uninitialized memory                       | Неинициализированная память                             |
// # |  14 | -Z sanitizer=leak               | Memory leak                                | Утечка памяти                                           |
// #================================================================================================================================================#
// # |                                        MEMORY ORDERING                                                                                       |
// #================================================================================================================================================#
// # |  15 | Ordering::Relaxed               | Faqat atomik, ordering kafolat yo'q        | Только атомарность, нет гарантий порядка                |
// # |  16 | Ordering::Release/Acquire       | Producer/Consumer sinxronizatsiya          | Синхронизация Producer/Consumer                         |
// # |  17 | Ordering::SeqCst                | Eng qattiq — global tartib kafolat         | Строжайшая — гарантия глобального порядка               |
// #================================================================================================================================================#