// #================================================================================================================================================#
// #                                                            ATOMIC TYPES                                                                        #
// #                    ATOMIK TURLAR — LOCK-FREE THREAD-SAFE OPERATSIYALAR. ORDERING. CAS. FETCH_*. SPINLOCK. SEQLOCK.                             #
// #                    АТОМАРНЫЕ ТИПЫ — LOCK-FREE ПОТОКОБЕЗОПАСНЫЕ ОПЕРАЦИИ. ORDERING. CAS. FETCH_*. SPINLOCK. SEQLOCK.                            #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize,
    AtomicU8, AtomicU16, AtomicU32, AtomicU64, AtomicUsize,
    AtomicPtr, Ordering,
};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Atomik turlar nima:
// Что такое атомарные типы:
//
//   - Lock-free thread-safe operatsiyalar
//   - Потокобезопасные операции без блокировок
//   - CPU darajasida atomik (bo'linmas) operatsiyalar
//   - Атомарные (неделимые) операции на уровне CPU
//   - Mutex dan tezroq — kernel space ga tushmaydi
//   - Быстрее Mutex — не спускается в пространство ядра
//
// Ordering — xotira tartib kafolatlari:
// Ordering — гарантии порядка памяти:
//
//   Relaxed  — tartib kafolat yo'q, faqat atomiklik
//   Relaxed  — нет гарантий порядка, только атомарность
//   Acquire  — keyingi o'qish/yozishlar shu operatsiyadan keyin
//   Acquire  — последующие read/write после этой операции
//   Release  — oldingi o'qish/yozishlar shu operatsiyadan oldin
//   Release  — предшествующие read/write до этой операции
//   AcqRel   — Acquire + Release (read-modify-write uchun)
//   AcqRel   — Acquire + Release (для read-modify-write)
//   SeqCst   — to'liq tartib kafolat (eng sekin, eng xavfsiz)
//   SeqCst   — полная гарантия порядка (медленнее всего, самый безопасный)
//
// Ordering tanlash qoidasi:
// Правило выбора Ordering:
//   Ishonchsiz bo'lsang — SeqCst ishlatgin (eng xavfsiz)
//   Если не уверен — используй SeqCst (самый безопасный)
//   Yukni kamaytirish uchun — Relaxed faqat hisoblagich uchun
//   Для снижения нагрузки — Relaxed только для счётчиков

fn asosiy_atomik_misollari() {

    // AtomicUsize — eng ko'p ishlatiladigan
    // AtomicUsize — наиболее часто используемый
    let hisob = AtomicUsize::new(0);

    hisob.store(42, Ordering::Relaxed);
    println!("{}", hisob.load(Ordering::Relaxed));
    // 42

    // fetch_add — atomik qo'shish va eski qiymatni qaytarish
    // fetch_add — атомарное сложение и возврат старого значения
    let eski: usize = hisob.fetch_add(10, Ordering::SeqCst);
    println!("Eski: {}, Yangi: {}", eski, hisob.load(Ordering::SeqCst));
    // Eski: 42, Yangi: 52

    // fetch_sub — atomik ayirish
    // fetch_sub — атомарное вычитание
    let eski2 = hisob.fetch_sub(5, Ordering::SeqCst);
    println!("Eski: {}, Yangi: {}", eski2, hisob.load(Ordering::SeqCst));
    // Eski: 52, Yangi: 47

    // swap — atomik almashtirish
    // swap — атомарная замена
    let oldingi = hisob.swap(100, Ordering::SeqCst);
    println!("Oldingi: {}, Hozir: {}", oldingi, hisob.load(Ordering::SeqCst));
    // Oldingi: 47, Hozir: 100

    // AtomicBool
    // AtomicBool
    let flag = AtomicBool::new(false);
    flag.store(true, Ordering::Release);
    println!("{}", flag.load(Ordering::Acquire));
    // true

    // fetch_or, fetch_and, fetch_xor — bit operatsiyalar
    // fetch_or, fetch_and, fetch_xor — битовые операции
    let bits = AtomicU8::new(0b0000_0000);
    bits.fetch_or(0b0000_1111, Ordering::SeqCst);
    println!("{:08b}", bits.load(Ordering::SeqCst));
    // 00001111

    bits.fetch_and(0b1111_0000, Ordering::SeqCst);
    println!("{:08b}", bits.load(Ordering::SeqCst));
    // 00000000

    bits.fetch_xor(0b1010_1010, Ordering::SeqCst);
    println!("{:08b}", bits.load(Ordering::SeqCst));
    // 10101010

    // AtomicI64
    // AtomicI64
    let n = AtomicI64::new(-100);
    n.fetch_add(200, Ordering::Relaxed);
    println!("{}", n.load(Ordering::Relaxed));
    // 100

    // fetch_max va fetch_min
    // fetch_max и fetch_min
    let m = AtomicI32::new(50);
    m.fetch_max(100, Ordering::SeqCst);
    println!("{}", m.load(Ordering::SeqCst)); // 100
    m.fetch_min(30, Ordering::SeqCst);
    println!("{}", m.load(Ordering::SeqCst)); // 30
    // 100
    // 30

    // Barcha atomik turlar
    // Все атомарные типы
    let _au8  = AtomicU8::new(0);
    let _au16 = AtomicU16::new(0);
    let _au32 = AtomicU32::new(0);
    let _au64 = AtomicU64::new(0);
    let _ai8  = AtomicI8::new(0);
    let _ai16 = AtomicI16::new(0);
    let _ai32 = AtomicI32::new(0);
    let _ai64 = AtomicI64::new(0);
    let _ab   = AtomicBool::new(false);
    let _aus  = AtomicUsize::new(0);
    let _ais  = AtomicIsize::new(0);
    println!("Barcha atomik turlar OK");
}

fn cas_misollari() {

    // compare_exchange — CAS operatsiyasi
    // compare_exchange — операция CAS
    //
    // Pseudo-kod:
    // if *self == expected {
    //     *self = new;
    //     Ok(old_value)
    // } else {
    //     Err(current_value)
    // }

    let atom = AtomicI32::new(10);

    // Kutilgan qiymat to'g'ri — muvaffaqiyat
    // Ожидаемое значение верно — успех
    match atom.compare_exchange(10, 20, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(eski)  => println!("CAS muvaffaqiyat! Eski: {}, Yangi: {}", eski, atom.load(Ordering::SeqCst)),
        Err(hozir) => println!("CAS muvaffaqiyatsiz. Hozir: {}", hozir),
    }
    // CAS muvaffaqiyat! Eski: 10, Yangi: 20

    // Kutilgan qiymat noto'g'ri — muvaffaqiyatsiz
    // Ожидаемое значение неверно — неудача
    match atom.compare_exchange(10, 30, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(eski)   => println!("CAS muvaffaqiyat! Eski: {}", eski),
        Err(hozir) => println!("CAS muvaffaqiyatsiz. Hozir: {}", hozir),
    }
    // CAS muvaffaqiyatsiz. Hozir: 20

    // compare_exchange_weak — CPU optimizatsiyasi (spurious failure mumkin)
    // compare_exchange_weak — оптимизация CPU (возможен spurious failure)
    // Loop ichida ishlatiladi
    // Используется в цикле
    let atom2 = AtomicU64::new(0);
    let mut eski = atom2.load(Ordering::Relaxed);
    loop {
        match atom2.compare_exchange_weak(eski, eski + 1, Ordering::SeqCst, Ordering::Relaxed) {
            Ok(_)  => { println!("Yangi qiymat: {}", atom2.load(Ordering::SeqCst)); break; }
            Err(v) => eski = v,
        }
    }
    // Yangi qiymat: 1

    // CAS loop — lock-free increment
    // CAS loop — lock-free инкремент
    let shared = Arc::new(AtomicU64::new(0));
    let mut handlar = vec![];

    for _ in 0..4 {
        let s = Arc::clone(&shared);
        handlar.push(thread::spawn(move || {
            for _ in 0..1000 {
                // fetch_add ham ishlatsa bo'ladi, bu CAS ni ko'rsatish uchun
                let mut hozirgi = s.load(Ordering::Relaxed);
                loop {
                    match s.compare_exchange_weak(
                        hozirgi,
                        hozirgi + 1,
                        Ordering::SeqCst,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => break,
                        Err(v) => hozirgi = v,
                    }
                }
            }
        }));
    }
    for h in handlar { h.join().unwrap(); }
    println!("CAS natija: {}", shared.load(Ordering::SeqCst)); // 4000
    // CAS natija: 4000
}

fn ordering_misollari() {

    // Relaxed — faqat atomiklik, tartib kafolat yo'q
    // Relaxed — только атомарность, нет гарантий порядка
    // Foydalanish: hisoblagich, statistika
    // Использование: счётчики, статистика
    let relaxed_hisob = Arc::new(AtomicU64::new(0));
    let h = {
        let c = Arc::clone(&relaxed_hisob);
        thread::spawn(move || {
            for _ in 0..1000 {
                c.fetch_add(1, Ordering::Relaxed);
            }
        })
    };
    h.join().unwrap();
    println!("Relaxed hisob: {}", relaxed_hisob.load(Ordering::Relaxed));
    // Relaxed hisob: 1000

    // Acquire/Release — Producer-Consumer pattern
    // Acquire/Release — паттерн Producer-Consumer
    let ma_lumot = Arc::new(AtomicU64::new(0));
    let tayyor = Arc::new(AtomicBool::new(false));

    let (ma_lumot2, tayyor2) = (Arc::clone(&ma_lumot), Arc::clone(&tayyor));
    let producer = thread::spawn(move || {
        // Ma'lumot yozish
        ma_lumot2.store(42, Ordering::Relaxed);
        // Release — store dan oldingi barcha yozishlar ko'rinadi
        // Release — все записи до store видны
        tayyor2.store(true, Ordering::Release);
    });

    let (ma_lumot3, tayyor3) = (Arc::clone(&ma_lumot), Arc::clone(&tayyor));
    let consumer = thread::spawn(move || {
        // Acquire — load dan keyingi barcha o'qishlar kafolatlangan
        // Acquire — все чтения после load гарантированы
        while !tayyor3.load(Ordering::Acquire) {
            thread::yield_now();
        }
        // ma_lumot2.store() ning Release bilan sinxronlangan
        // Синхронизировано с Release от ma_lumot2.store()
        println!("Ma'lumot: {}", ma_lumot3.load(Ordering::Relaxed));
    });

    producer.join().unwrap();
    consumer.join().unwrap();
    // Ma'lumot: 42

    // SeqCst — to'liq tartib (eng xavfsiz)
    // SeqCst — полный порядок (самый безопасный)
    let x = Arc::new(AtomicBool::new(false));
    let y = Arc::new(AtomicBool::new(false));
    let z = Arc::new(AtomicUsize::new(0));

    let (x2, y2, z2) = (Arc::clone(&x), Arc::clone(&y), Arc::clone(&z));
    let t1 = thread::spawn(move || {
        x2.store(true, Ordering::SeqCst);
        if !y2.load(Ordering::SeqCst) {
            z2.fetch_add(1, Ordering::SeqCst);
        }
    });

    let (x3, y3, z3) = (Arc::clone(&x), Arc::clone(&y), Arc::clone(&z));
    let t2 = thread::spawn(move || {
        y3.store(true, Ordering::SeqCst);
        if !x3.load(Ordering::SeqCst) {
            z3.fetch_add(1, Ordering::SeqCst);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("SeqCst z: {}", z.load(Ordering::SeqCst));
    // SeqCst z: 0 yoki 1 (SeqCst bilan 2 bo'lmaydi kafolatlangan)
}

// Spinlock — CPU ni kutayotganda ishlatiladigan mutex
// Spinlock — мьютекс который крутится пока ждёт
struct Spinlock {
    qulflangan: AtomicBool,
}

impl Spinlock {
    fn new() -> Self {
        Spinlock { qulflangan: AtomicBool::new(false) }
    }

    fn qulflash(&self) {
        // false → true bo'lguncha spin
        // Спиним пока false → true
        while self.qulflangan
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // CPU ga imkoniyat berish — busy waiting optimallashtirish
            // Уступить CPU — оптимизация busy waiting
            std::hint::spin_loop();
        }
    }

    fn qulfni_ochish(&self) {
        self.qulflangan.store(false, Ordering::Release);
    }

    fn bilan<F: FnOnce() -> R, R>(&self, f: F) -> R {
        self.qulflash();
        let natija = f();
        self.qulfni_ochish();
        natija
    }
}

// SpinlockGuard — RAII guard
// SpinlockGuard — RAII guard
struct SpinlockGuard<'a> {
    lock: &'a Spinlock,
}

impl<'a> Drop for SpinlockGuard<'a> {
    fn drop(&mut self) {
        self.lock.qulfni_ochish();
    }
}

impl Spinlock {
    fn guard(&self) -> SpinlockGuard<'_> {
        self.qulflash();
        SpinlockGuard { lock: self }
    }
}

fn spinlock_misollari() {

    let lock = Arc::new(Spinlock::new());
    let hisob = Arc::new(AtomicI64::new(0));
    let mut handlar = vec![];

    for _ in 0..4 {
        let l = Arc::clone(&lock);
        let h = Arc::clone(&hisob);
        handlar.push(thread::spawn(move || {
            for _ in 0..1000 {
                l.bilan(|| {
                    let v = h.load(Ordering::Relaxed);
                    h.store(v + 1, Ordering::Relaxed);
                });
            }
        }));
    }
    for h in handlar { h.join().unwrap(); }
    println!("Spinlock hisob: {}", hisob.load(Ordering::Relaxed));
    // Spinlock hisob: 4000
}

fn atomic_ptr_misollari() {

    // AtomicPtr — raw pointer ni atomik boshqarish
    // AtomicPtr — атомарное управление сырым указателем
    let mut qiymat = 42i32;
    let ptr: AtomicPtr<i32> = AtomicPtr::new(&mut qiymat);

    let olindi: *mut i32 = ptr.load(Ordering::SeqCst);
    unsafe {
        println!("Ptr qiymati: {}", *olindi);
    }
    // Ptr qiymati: 42

    // swap — pointer ni almashtirish
    // swap — замена указателя
    let mut yangi_qiymat = 99i32;
    let eski_ptr = ptr.swap(&mut yangi_qiymat, Ordering::SeqCst);
    unsafe {
        println!("Eski: {}, Yangi: {}", *eski_ptr, *ptr.load(Ordering::SeqCst));
    }
    // Eski: 42, Yangi: 99
}

// Treiber Stack — klassik lock-free stack
// Стек Трейбера — классический lock-free стек
struct LockFreeStack<T> {
    bosh: AtomicPtr<Tugun<T>>,
}

struct Tugun<T> {
    qiymat: T,
    keyingi: *mut Tugun<T>,
}

impl<T> LockFreeStack<T> {
    fn new() -> Self {
        LockFreeStack { bosh: AtomicPtr::new(std::ptr::null_mut()) }
    }

    fn push(&self, qiymat: T) {
        let yangi = Box::into_raw(Box::new(Tugun {
            qiymat,
            keyingi: std::ptr::null_mut(),
        }));

        loop {
            let joriy_bosh = self.bosh.load(Ordering::Relaxed);
            unsafe { (*yangi).keyingi = joriy_bosh; }

            if self.bosh
                .compare_exchange(joriy_bosh, yangi, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                return;
            }
        }
    }

    fn pop(&self) -> Option<T> {
        loop {
            let joriy_bosh = self.bosh.load(Ordering::Acquire);
            if joriy_bosh.is_null() { return None; }

            let keyingi = unsafe { (*joriy_bosh).keyingi };

            if self.bosh
                .compare_exchange(joriy_bosh, keyingi, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                let tugun = unsafe { Box::from_raw(joriy_bosh) };
                return Some(tugun.qiymat);
            }
        }
    }
}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

fn lock_free_stack_misoli() {

    let stek = LockFreeStack::new();
    stek.push(1);
    stek.push(2);
    stek.push(3);

    println!("{:?}", stek.pop()); // Some(3) — LIFO
    println!("{:?}", stek.pop()); // Some(2)
    println!("{:?}", stek.pop()); // Some(1)
    println!("{:?}", stek.pop()); // None
    // Some(3)
    // Some(2)
    // Some(1)
    // None

    // Thread-safe test
    let stek2 = Arc::new(LockFreeStack::new());
    let mut handlar = vec![];

    for i in 0..4 {
        let s = Arc::clone(&stek2);
        handlar.push(thread::spawn(move || {
            for j in 0..5 {
                s.push(i * 10 + j);
            }
        }));
    }
    for h in handlar { h.join().unwrap(); }

    let mut count = 0;
    while stek2.pop().is_some() { count += 1; }
    println!("Jami elementlar: {}", count); // 20
    // Jami elementlar: 20
}

// Thread-safe hisob yurituvchi
// Потокобезопасный счётчик
struct Hisoblagich {
    jami: AtomicU64,
    muvaffaqiyatlar: AtomicU64,
    xatolar: AtomicU64,
}

impl Hisoblagich {
    fn new() -> Self {
        Hisoblagich {
            jami: AtomicU64::new(0),
            muvaffaqiyatlar: AtomicU64::new(0),
            xatolar: AtomicU64::new(0),
        }
    }

    fn so_rov(&self, muvaffaqiyatlimi: bool) {
        self.jami.fetch_add(1, Ordering::Relaxed);
        if muvaffaqiyatlimi {
            self.muvaffaqiyatlar.fetch_add(1, Ordering::Relaxed);
        } else {
            self.xatolar.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn statistika(&self) -> (u64, u64, u64) {
        (
            self.jami.load(Ordering::Relaxed),
            self.muvaffaqiyatlar.load(Ordering::Relaxed),
            self.xatolar.load(Ordering::Relaxed),
        )
    }
}

// Once-flag — bir marta bajariladigan kod
// Once-flag — код выполняемый один раз
struct OnceFlag {
    bajarildi: AtomicBool,
}

impl OnceFlag {
    fn new() -> Self { OnceFlag { bajarildi: AtomicBool::new(false) } }

    fn bajar<F: FnOnce()>(&self, f: F) {
        // compare_exchange — faqat bitta thread muvaffaqiyatli bo'ladi
        // compare_exchange — только один поток успешен
        if self.bajarildi
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            f();
        }
    }
}

fn real_hayot_misollari() {

    // Thread-safe hisoblagich
    let hisob = Arc::new(Hisoblagich::new());
    let mut handlar = vec![];

    for i in 0..5 {
        let h = Arc::clone(&hisob);
        handlar.push(thread::spawn(move || {
            for j in 0..100 {
                h.so_rov(j % 5 != 0); // 20% xato
            }
        }));
    }
    for h in handlar { h.join().unwrap(); }

    let (jami, muv, xat) = hisob.statistika();
    println!("Jami: {}, Muvaffaqiyat: {}, Xato: {}", jami, muv, xat);
    // Jami: 500, Muvaffaqiyat: 400, Xato: 100

    // OnceFlag — bir marta init
    let once = Arc::new(OnceFlag::new());
    let mut handlar2 = vec![];

    for i in 0..5 {
        let o = Arc::clone(&once);
        handlar2.push(thread::spawn(move || {
            o.bajar(|| println!("Faqat bir marta! Thread {}", i));
        }));
    }
    for h in handlar2 { h.join().unwrap(); }
    // Faqat bir marta! Thread X (faqat bitta!)

    // Atomic vs Mutex — performance taqqoslash
    // Atomic vs Mutex — сравнение производительности
    let n = 100_000;

    // Atomic
    let atomic_hisob = Arc::new(AtomicU64::new(0));
    let boshlanish = Instant::now();
    let handlar3: Vec<_> = (0..4).map(|_| {
        let h = Arc::clone(&atomic_hisob);
        thread::spawn(move || {
            for _ in 0..n { h.fetch_add(1, Ordering::Relaxed); }
        })
    }).collect();
    for h in handlar3 { h.join().unwrap(); }
    let atomic_vaqt = boshlanish.elapsed();

    // Mutex
    let mutex_hisob = Arc::new(Mutex::new(0u64));
    let boshlanish2 = Instant::now();
    let handlar4: Vec<_> = (0..4).map(|_| {
        let h = Arc::clone(&mutex_hisob);
        thread::spawn(move || {
            for _ in 0..n { *h.lock().unwrap() += 1; }
        })
    }).collect();
    for h in handlar4 { h.join().unwrap(); }
    let mutex_vaqt = boshlanish2.elapsed();

    println!("Atomic: {:?} natija: {}", atomic_vaqt, atomic_hisob.load(Ordering::Relaxed));
    println!("Mutex:  {:?} natija: {}", mutex_vaqt, mutex_hisob.lock().unwrap());
    // Atomic: ~Xms natija: 400000  ← tezroq!
    // Mutex:  ~Yms natija: 400000

    // Lock-free stack
    println!("\n--- Lock-Free Stack ---");
    lock_free_stack_misoli();
}

fn main() {

    println!("=== ASOSIY ATOMIK OPERATSIYALAR ===");
    asosiy_atomik_misollari();

    println!("\n=== CAS — COMPARE-AND-SWAP ===");
    cas_misollari();

    println!("\n=== ORDERING ===");
    ordering_misollari();

    println!("\n=== SPINLOCK ===");
    spinlock_misollari();

    println!("\n=== ATOMIC POINTER ===");
    atomic_ptr_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        ASOSIY OPERATSIYALAR                                                                                  |
// #================================================================================================================================================#
// # |   1 | .load(Ordering)                 | Qiymatni o'qish                            | Чтение значения                                         |
// # |   2 | .store(val, Ordering)           | Qiymatni yozish                            | Запись значения                                         |
// # |   3 | .swap(val, Ordering)            | Atomik almashtirish                        | Атомарная замена                                        |
// # |   4 | .fetch_add(n, Ordering)         | Qo'shib eski qiymatni qaytarish            | Сложение и возврат старого значения                     |
// # |   5 | .fetch_sub(n, Ordering)         | Ayirib eski qiymatni qaytarish             | Вычитание и возврат старого значения                    |
// # |   6 | .fetch_or/and/xor               | Bit operatsiyalar                          | Битовые операции                                        |
// # |   7 | .fetch_max / .fetch_min         | Maksimum/minimum yangilash                 | Обновление максимума/минимума                           |
// #================================================================================================================================================#
// # |                                        CAS                                                                                                   |
// #================================================================================================================================================#
// # |   8 | .compare_exchange(exp,new,s,f)  | CAS — kutilgan bo'lsa yangilash           | CAS — обновить если ожидаемое                            |
// # |   9 | .compare_exchange_weak(...)     | CAS — spurious failure mumkin (loop uchun)| CAS — возможен spurious failure (для цикла)              |
// # |  10 | CAS loop                        | Lock-free algoritmlar asosi                | Основа lock-free алгоритмов                             |
// #================================================================================================================================================#
// # |                                        ORDERING                                                                                              |
// #================================================================================================================================================#
// # |  11 | Ordering::Relaxed               | Faqat atomiklik, tartib kafolat yo'q       | Только атомарность, нет порядка                         |
// # |  12 | Ordering::Acquire               | Load — keyingilari kafolatlangan           | Load — последующие гарантированы                        |
// # |  13 | Ordering::Release               | Store — oldingilari kafolatlangan          | Store — предшествующие гарантированы                    |
// # |  14 | Ordering::AcqRel                | Acquire + Release (RMW uchun)              | Acquire + Release (для RMW)                             |
// # |  15 | Ordering::SeqCst                | To'liq tartib, eng xavfsiz, eng sekin      | Полный порядок, самый безопасный, медленный             |
// #================================================================================================================================================#
// # |                                        PATTERNLAR                                                                                            |
// #================================================================================================================================================#
// # |  16 | Spinlock                        | CPU spin — qisqa kutish uchun              | CPU spin — для кратких ожиданий                         |
// # |  17 | Lock-free stack (Treiber)       | CAS bilan thread-safe stek                 | Потокобезопасный стек с CAS                             |
// # |  18 | AtomicPtr                       | Raw pointer ni atomik boshqarish           | Атомарное управление указателем                         |
// # |  19 | Atomic vs Mutex                 | Atomic ~2-5x tezroq (benchmark ga qarab)   | Atomic ~2-5x быстрее (зависит от benchmark)             |
// # |  20 | spin_loop hint                  | CPU ga spin-wait optimizatsiyasi           | Подсказка CPU об оптимизации spin-wait                  |
// #================================================================================================================================================#