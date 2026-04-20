// #================================================================================================================================================#
// #                                                     BARRIER  |  CONDVAR  |  ONCE                                                               #
// #                BARRIER — N THREAD SINXRONIZATSIYA. CONDVAR — SHARTLI KUTISH. ONCE — BIR MARTA INIT. CHUQUR PATTERNLAR.                         #
// #                BARRIER — СИНХРОНИЗАЦИЯ N ПОТОКОВ. CONDVAR — УСЛОВНОЕ ОЖИДАНИЕ. ONCE — ИНИЦИАЛИЗАЦИЯ ОДИН РАЗ. ПАТТЕРНЫ.                        #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::sync::{Arc, Barrier, Condvar, Mutex, Once, OnceLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::VecDeque;

// Barrier — N thread bir nuqtaga yetguncha kutadi
// Barrier — N потоков ждут пока все достигнут точки
//
// Foydalanish holatlari:
// Случаи использования:
//   - Parallel hisoblash bosqichlari orasida sinxronizatsiya
//   - Синхронизация между этапами параллельных вычислений
//   - Test — barcha threadlar tayyor bo'lganda boshlash
//   - Тест — начать когда все потоки готовы
//   - Phased computation — bosqichma-bosqich hisoblash

fn barrier_asosiy_misollari() {

    let to_siq = 4;
    let barrier = Arc::new(Barrier::new(to_siq));
    let natijalar = Arc::new(Mutex::new(vec![0i32; to_siq]));
    let mut handlar = vec![];

    for i in 0..to_siq {
        let b = Arc::clone(&barrier);
        let n = Arc::clone(&natijalar);

        handlar.push(thread::spawn(move || {
            // 1-bosqich: ma'lumot tayyorlash
            let qiymat = (i as i32 + 1) * 10;
            thread::sleep(Duration::from_millis(i as u64 * 10));
            n.lock().unwrap()[i] = qiymat;
            println!("[Thread {}] 1-bosqich: qiymat={}", i, qiymat);

            // Barrier — barcha 1-bosqichni tugataversin
            // Barrier — ждать пока все закончат 1-й этап
            let wait_natija = b.wait();
            if wait_natija.is_leader() {
                println!("[LEADER] Barcha 1-bosqichni tugatdi!");
            }

            // 2-bosqich — barcha ma'lumotlar tayyor
            let jami: i32 = n.lock().unwrap().iter().sum();
            println!("[Thread {}] 2-bosqich: jami={}", i, jami);
        }));
    }

    for h in handlar { h.join().unwrap(); }
    // [Thread X] 1-bosqich: qiymat=XX
    // [LEADER] Barcha 1-bosqichni tugatdi!
    // [Thread X] 2-bosqich: jami=100
}

fn barrier_phased_hisoblash() {

    // Phased computation — ko'p bosqichli parallel hisoblash
    // Phased computation — многоэтапные параллельные вычисления
    let thread_soni = 4;
    let bosqich_soni = 3;
    let barrier = Arc::new(Barrier::new(thread_soni));
    let ma_lumot = Arc::new(Mutex::new(vec![1.0f64; thread_soni]));

    let mut handlar = vec![];

    for i in 0..thread_soni {
        let b = Arc::clone(&barrier);
        let m = Arc::clone(&ma_lumot);

        handlar.push(thread::spawn(move || {
            for bosqich in 0..bosqich_soni {
                // O'z hissasini hisoblash
                {
                    let mut v = m.lock().unwrap();
                    v[i] *= 2.0;
                    println!("[T{}] Bosqich {}: qiymat={:.1}", i, bosqich, v[i]);
                }

                // Sinxronizatsiya — keyingi bosqichga o'tish
                b.wait();

                // Birinchi thread umumiy holatni chiqaradi
                if i == 0 {
                    let v = m.lock().unwrap();
                    let yig: f64 = v.iter().sum();
                    println!("[Bosqich {} tugadi] Jami: {:.1}\n", bosqich, yig);
                }
                b.wait(); // ikkinchi barrier — chiqarish tugaguncha
            }
        }));
    }

    for h in handlar { h.join().unwrap(); }
}

// Condvar — shart bajarilguncha thread ni to'xtatish
// Condvar — остановить поток пока условие не выполнится
//
// Har doim Mutex bilan birga ishlatiladi
// Всегда используется вместе с Mutex
//
// Pattern:
//   let (mutex, cvar) = ...;
//   let mut guard = mutex.lock().unwrap();
//   while !shart { guard = cvar.wait(guard).unwrap(); }
//   // Shart bajarildi — davom etish

fn condvar_asosiy_misollari() {

    // Oddiy signal — bitta producer, bitta consumer
    // Простой сигнал — один producer, один consumer
    let juft = Arc::new((Mutex::new(false), Condvar::new()));
    let juft2 = Arc::clone(&juft);

    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*juft2;
        let mut tayyor = lock.lock().unwrap();
        while !*tayyor {
            println!("[Consumer] Signal kutmoqda...");
            tayyor = cvar.wait(tayyor).unwrap();
        }
        println!("[Consumer] Signal olindi! Ishlayapman.");
    });

    thread::sleep(Duration::from_millis(30));
    let (lock, cvar) = &*juft;
    *lock.lock().unwrap() = true;
    cvar.notify_one();
    println!("[Producer] Signal yuborildi");

    consumer.join().unwrap();
    // [Consumer] Signal kutmoqda...
    // [Producer] Signal yuborildi
    // [Consumer] Signal olindi! Ishlayapman.
}

fn condvar_wait_timeout_misoli() {

    // wait_timeout — vaqt tugasa ham davom etish
    // wait_timeout — продолжить даже если время истекло
    let juft = Arc::new((Mutex::new(false), Condvar::new()));
    let juft2 = Arc::clone(&juft);

    let thread_h = thread::spawn(move || {
        let (lock, cvar) = &*juft2;
        let mut tayyor = lock.lock().unwrap();

        let (yangi_tayyor, timeout_natija) = cvar
            .wait_timeout(tayyor, Duration::from_millis(50))
            .unwrap();
        tayyor = yangi_tayyor;

        if timeout_natija.timed_out() {
            println!("[Thread] Timeout! Signal kelmadi. tayyor={}", *tayyor);
        } else {
            println!("[Thread] Signal olindi!");
        }
    });

    // Signal yubormayapmiz — timeout bo'ladi
    thread_h.join().unwrap();
    // [Thread] Timeout! Signal kelmadi. tayyor=false
}

fn condvar_notify_all_misoli() {

    // notify_all — barcha kutayotgan threadlarga xabar berish
    // notify_all — уведомить все ожидающие потоки
    let juft = Arc::new((Mutex::new(false), Condvar::new()));
    let mut handlar = vec![];

    for i in 0..5 {
        let j = Arc::clone(&juft);
        handlar.push(thread::spawn(move || {
            let (lock, cvar) = &*j;
            let mut tayyor = lock.lock().unwrap();
            while !*tayyor {
                tayyor = cvar.wait(tayyor).unwrap();
            }
            println!("[Thread {}] Signal olindi!", i);
        }));
    }

    thread::sleep(Duration::from_millis(30));
    let (lock, cvar) = &*juft;
    *lock.lock().unwrap() = true;
    cvar.notify_all(); // Barcha threadlarga!
    println!("[Main] notify_all yuborildi");

    for h in handlar { h.join().unwrap(); }
    // [Main] notify_all yuborildi
    // [Thread X] Signal olindi! (barcha 5 ta)
}

// Chegarali kanal — to'lsa producer kutadi, bo'sh bo'lsa consumer kutadi
// Ограниченный канал — producer ждёт если полно, consumer ждёт если пусто
struct BoundedChannel<T> {
    navbat: Mutex<VecDeque<T>>,
    to_lmas_emas: Condvar, // consumer uchun — ma'lumot bor
    bosh_emas: Condvar,    // producer uchun — joy bor
    sig_im: usize,
}

impl<T> BoundedChannel<T> {
    fn new(sig_im: usize) -> Self {
        BoundedChannel {
            navbat: Mutex::new(VecDeque::new()),
            to_lmas_emas: Condvar::new(),
            bosh_emas: Condvar::new(),
            sig_im,
        }
    }

    fn yuborish(&self, qiymat: T) {
        let mut navbat = self.navbat.lock().unwrap();
        // To'lsa — joy bo'lguncha kutish
        while navbat.len() >= self.sig_im {
            println!("[Producer] Navbat to'la, kutmoqda...");
            navbat = self.bosh_emas.wait(navbat).unwrap();
        }
        navbat.push_back(qiymat);
        self.to_lmas_emas.notify_one(); // consumer ga xabar
    }

    fn qabul_qilish(&self) -> T {
        let mut navbat = self.navbat.lock().unwrap();
        // Bo'sh bo'lsa — ma'lumot kelguncha kutish
        while navbat.is_empty() {
            navbat = self.to_lmas_emas.wait(navbat).unwrap();
        }
        let qiymat = navbat.pop_front().unwrap();
        self.bosh_emas.notify_one(); // producer ga xabar
        qiymat
    }

    fn uzunlik(&self) -> usize {
        self.navbat.lock().unwrap().len()
    }
}

fn bounded_channel_misoli() {

    let kanal = Arc::new(BoundedChannel::new(3)); // faqat 3 ta sig'adi

    let producer = {
        let k = Arc::clone(&kanal);
        thread::spawn(move || {
            for i in 0..6 {
                k.yuborish(i);
                println!("[Producer] {} yuborildi, navbat uzunlik: {}", i, k.uzunlik());
                thread::sleep(Duration::from_millis(10));
            }
        })
    };

    let consumer = {
        let k = Arc::clone(&kanal);
        thread::spawn(move || {
            for _ in 0..6 {
                thread::sleep(Duration::from_millis(25)); // sekinroq
                let v = k.qabul_qilish();
                println!("[Consumer] {} qabul qilindi", v);
            }
        })
    };

    producer.join().unwrap();
    consumer.join().unwrap();
}

// Once — bir marta bajarilishi kafolatlangan
// Once — гарантированно выполняется один раз
// Thread-safe, panic-safe

static ONCE: Once = Once::new();
static GLOBAL_CONFIG: OnceLock<String> = OnceLock::new();

fn once_misollari() {

    // Once::call_once — bir marta chaqiriladi
    // Once::call_once — вызывается один раз
    for i in 0..3 {
        ONCE.call_once(|| {
            println!("Faqat bir marta! i={}", i);
            GLOBAL_CONFIG.set(String::from("initsializatsiya qilindi")).ok();
        });
        println!("Iteratsiya {}: is_completed={}", i, ONCE.is_completed());
    }
    // Faqat bir marta! i=0
    // Iteratsiya 0: is_completed=true
    // Iteratsiya 1: is_completed=true
    // Iteratsiya 2: is_completed=true

    println!("Config: {:?}", GLOBAL_CONFIG.get());
    // Config: Some("initsializatsiya qilindi")

    // Once — thread-safe initsializatsiya
    // Once — потокобезопасная инициализация
    static THREAD_ONCE: Once = Once::new();
    let mut handlar = vec![];

    for i in 0..5 {
        handlar.push(thread::spawn(move || {
            THREAD_ONCE.call_once(|| {
                println!("[Thread {}] Initsializatsiya!", i);
                thread::sleep(Duration::from_millis(10));
            });
            println!("[Thread {}] Tayyor, is_completed={}", i, THREAD_ONCE.is_completed());
        }));
    }
    for h in handlar { h.join().unwrap(); }
    // [Thread X] Initsializatsiya! (faqat bitta thread)
    // [Thread X] Tayyor, is_completed=true (hammasi)

    // call_once_force — panic bo'lsa ham bir marta
    // call_once_force — один раз даже при панике
    static FORCE_ONCE: Once = Once::new();

    // Birinchi chaqiruv — panic
    let _ = std::panic::catch_unwind(|| {
        FORCE_ONCE.call_once(|| {
            panic!("Initsializatsiyada panic!");
        });
    });

    println!("Poisoned: {}", FORCE_ONCE.is_completed());
    // Poisoned: false

    // call_once_force — poisoned holatda ham ishlatish
    FORCE_ONCE.call_once_force(|_state| {
        println!("Force initsializatsiya!");
    });
    println!("is_completed: {}", FORCE_ONCE.is_completed());
    // Force initsializatsiya!
    // is_completed: true
}

static DB_ULANISH: OnceLock<String> = OnceLock::new();
static APP_VERSIYA: OnceLock<(u32, u32, u32)> = OnceLock::new();

fn db_ulanish_ol() -> &'static str {
    DB_ULANISH.get_or_init(|| {
        println!("DB ulanish yaratilmoqda...");
        String::from("postgresql://localhost:5432/mydb")
    })
}

fn oncelock_misollari() {

    // Bir nechta thread — faqat bitta init
    // Несколько потоков — только одна инициализация
    let mut handlar = vec![];
    for i in 0..4 {
        handlar.push(thread::spawn(move || {
            let url = db_ulanish_ol();
            println!("[Thread {}] DB URL: {}", i, url);
        }));
    }
    for h in handlar { h.join().unwrap(); }
    // DB ulanish yaratilmoqda... (faqat bir marta)
    // [Thread X] DB URL: postgresql://... (hammasi)

    // Versiya — OnceLock
    let v = APP_VERSIYA.get_or_init(|| (1, 2, 3));
    println!("Versiya: {}.{}.{}", v.0, v.1, v.2);
    // Versiya: 1.2.3

    // OnceLock::set — bir marta yozish
    let once2: OnceLock<Vec<i32>> = OnceLock::new();
    once2.set(vec![1, 2, 3]).unwrap();
    println!("{:?}", once2.get());
    // Some([1, 2, 3])

    // set() ikkinchi marta — Err
    let xato = once2.set(vec![4, 5, 6]);
    println!("Ikkinchi set: {:?}", xato.err().map(|v| format!("{:?}", v)));
    // Ikkinchi set: Some("[4, 5, 6]")
}

// Work stealing pattern — Barrier va Condvar bilan
// Паттерн work stealing — с Barrier и Condvar
struct WorkQueue {
    vazifalar: Mutex<VecDeque<Box<dyn FnOnce() + Send>>>,
    ish_bor: Condvar,
    tugadimi: Mutex<bool>,
}

impl WorkQueue {
    fn new() -> Self {
        WorkQueue {
            vazifalar: Mutex::new(VecDeque::new()),
            ish_bor: Condvar::new(),
            tugadimi: Mutex::new(false),
        }
    }

    fn qo_sh<F: FnOnce() + Send + 'static>(&self, f: F) {
        self.vazifalar.lock().unwrap().push_back(Box::new(f));
        self.ish_bor.notify_one();
    }

    fn ishchi_loop(&self) {
        loop {
            let vazifa = {
                let mut v = self.vazifalar.lock().unwrap();
                loop {
                    if let Some(vazifa) = v.pop_front() {
                        break Some(vazifa);
                    }
                    if *self.tugadimi.lock().unwrap() {
                        break None;
                    }
                    v = self.ish_bor.wait(v).unwrap();
                }
            };

            match vazifa {
                Some(f) => f(),
                None => break,
            }
        }
    }

    fn tugatish(&self) {
        *self.tugadimi.lock().unwrap() = true;
        self.ish_bor.notify_all();
    }
}

fn real_hayot_misollari() {

    println!("--- Barrier: Phased ---");
    barrier_phased_hisoblash();

    println!("--- Condvar: wait_timeout ---");
    condvar_wait_timeout_misoli();

    println!("--- Condvar: notify_all ---");
    condvar_notify_all_misoli();

    println!("--- Bounded Channel ---");
    bounded_channel_misoli();

    println!("\n--- Work Queue ---");
    let wq = Arc::new(WorkQueue::new());
    let hisob = Arc::new(AtomicUsize::new(0));

    // 2 ta ishchi thread
    let mut ishchilar = vec![];
    for worker_id in 0..2 {
        let wq2 = Arc::clone(&wq);
        ishchilar.push(thread::spawn(move || {
            println!("[Worker-{}] Boshlandi", worker_id);
            wq2.ishchi_loop();
            println!("[Worker-{}] Tugadi", worker_id);
        }));
    }

    // Vazifalar qo'shish
    for i in 0..6 {
        let h = Arc::clone(&hisob);
        wq.qo_sh(move || {
            h.fetch_add(1, Ordering::Relaxed);
            println!("[Vazifa {}] Bajarildi", i);
            thread::sleep(Duration::from_millis(5));
        });
    }

    thread::sleep(Duration::from_millis(100));
    wq.tugatish();
    for w in ishchilar { w.join().unwrap(); }

    println!("Bajarilgan vazifalar: {}", hisob.load(Ordering::Relaxed));
    // [Worker-X] Boshlandi
    // [Vazifa X] Bajarildi (barcha 6 ta)
    // Bajarilgan vazifalar: 6
}

fn main() {

    println!("=== BARRIER ASOSIY ===");
    barrier_asosiy_misollari();

    println!("\n=== CONDVAR ASOSIY ===");
    condvar_asosiy_misollari();

    println!("\n=== ONCE ===");
    once_misollari();

    println!("\n=== ONCELOCK ===");
    oncelock_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        BARRIER                                                                                               |
// #================================================================================================================================================#
// # |   1 | Barrier::new(n)                 | N thread sinxronizatsiya nuqtasi           | Точка синхронизации N потоков                           |
// # |   2 | barrier.wait()                  | Boshqa threadlarni kutish                  | Ожидание других потоков                                 |
// # |   3 | wait_natija.is_leader()         | Bitta thread leader deb belgilanadi        | Один поток помечается лидером                           |
// # |   4 | Phased computation              | Ko'p bosqichli parallel hisoblash          | Многоэтапные параллельные вычисления                    |
// #================================================================================================================================================#
// # |                                        CONDVAR                                                                                               |
// #================================================================================================================================================#
// # |   5 | Condvar::new()                  | Yangi condvar                              | Новая condvar                                           |
// # |   6 | cvar.wait(guard)                | Shart bajarilguncha kutish                 | Ожидание выполнения условия                             |
// # |   7 | cvar.wait_timeout(guard, dur)   | Vaqt chegara bilan kutish                  | Ожидание с временным ограничением                       |
// # |   8 | cvar.notify_one()               | Bitta thread ga xabar                      | Уведомить один поток                                    |
// # |   9 | cvar.notify_all()               | Barcha threadlarga xabar                   | Уведомить все потоки                                    |
// # |  10 | while !shart { cvar.wait() }    | Spurious wakeup dan himoya                 | Защита от ложных пробуждений                            |
// # |  11 | Bounded channel                 | Condvar bilan chegarali kanal              | Ограниченный канал с Condvar                            |
// #================================================================================================================================================#
// # |                                        ONCE VA ONCELOCK                                                                                      |
// #================================================================================================================================================#
// # |  12 | Once::new()                     | Statik once                                | Статическая once                                        |
// # |  13 | once.call_once(|| ...)          | Bir marta bajarish                         | Выполнить один раз                                      |
// # |  14 | once.is_completed()             | Bajarilganligini tekshirish                | Проверка выполнения                                     |
// # |  15 | once.call_once_force(|s| ...)   | Poisoning bo'lsa ham bajarish              | Выполнить даже при poisoning                            |
// # |  16 | OnceLock::new()                 | Zamonaviy Once alternativ                  | Современная альтернатива Once                           |
// # |  17 | OnceLock::get_or_init(|| ...)   | Lazy init — thread-safe                    | Ленивая инициализация — потокобезопасно                 |
// # |  18 | static X: OnceLock<T>           | Global lazy static                          | Глобальный lazy static                                 |
// #================================================================================================================================================#