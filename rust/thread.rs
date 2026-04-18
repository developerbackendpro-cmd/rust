// #================================================================================================================================================#
// #                                                                   THREADS                                                                      #
// #                    THREADLAR CHUQUR — SPAWN, JOIN, SCOPED, THREAD-LOCAL, PARK/UNPARK, RAYON PATTERN, WORK STEALING.                            #
// #                    ПОТОКИ ГЛУБОКО — SPAWN, JOIN, SCOPED, THREAD-LOCAL, PARK/UNPARK, RAYON ПАТТЕРН, WORK STEALING.                              #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::thread;
use std::sync::{Arc, Mutex, Barrier, Condvar};
use std::time::{Duration, Instant};
use std::collections::HashMap;

// Rust threadlari:
// Потоки Rust:
//
//   - OS native threadlar (1:1 threading model)
//   - Нативные потоки ОС (модель потоков 1:1)
//   - Har bir thread o'z stack ga ega (~8MB default)
//   - Каждый поток имеет свой стек (~8MB по умолчанию)
//   - 'static bound — thread da faqat 'static ma'lumot
//   - 'static bound — только 'static данные в потоке
//   - move closure — ownership thread ga o'tkazish
//   - move closure — передача владения потоку
//   - JoinHandle — thread natijasini kutish
//   - JoinHandle — ожидание результата потока

fn spawn_join_misollari() {

    // Oddiy thread yaratish
    // Простое создание потока
    let handle = thread::spawn(|| {
        println!("Yangi thread ishlamoqda");
        42
    });

    println!("Asosiy thread ishlayapti");
    let natija = handle.join().unwrap();
    println!("Thread natijasi: {}", natija);
    // Asosiy thread ishlayapti
    // Yangi thread ishlamoqda
    // Thread natijasi: 42

    // Ko'p thread yaratish
    // Создание нескольких потоков
    let handlar: Vec<_> = (0..5).map(|i| {
        thread::spawn(move || {
            let kvadrat = i * i;
            println!("Thread {}: {}² = {}", i, i, kvadrat);
            kvadrat
        })
    }).collect();

    let natijalar: Vec<i32> = handlar.into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    println!("Barcha natijalar: {:?}", natijalar);
    // Thread X: X² = XX (tartib farqli bo'lishi mumkin)
    // Barcha natijalar: [0, 1, 4, 9, 16]

    // Thread bilan ma'lumot almashish — move bilan
    // Обмен данными с потоком — через move
    let ma_lumot = vec![1, 2, 3, 4, 5];
    let handle2 = thread::spawn(move || {
        let yig: i32 = ma_lumot.iter().sum();
        yig
    });
    println!("Yig'indi: {}", handle2.join().unwrap());
    // Yig'indi: 15

    // Thread panic — join Err qaytaradi
    // Thread panic — join возвращает Err
    let xatoli_thread = thread::spawn(|| {
        panic!("Thread ichida panic!");
    });
    match xatoli_thread.join() {
        Ok(_)  => println!("Muvaffaqiyat"),
        Err(e) => println!("Thread panic: {:?}", e.downcast_ref::<&str>()),
    }
    // Thread panic: Some("Thread ichida panic!")
}

fn thread_builder_misollari() {

    // Thread::Builder — nom va stack hajmi
    // Thread::Builder — имя и размер стека
    let handle = thread::Builder::new()
        .name("hisob-thread".to_string())
        .stack_size(2 * 1024 * 1024) // 2MB stack
        .spawn(|| {
            let nomi = thread::current().name().unwrap_or("noma'lum").to_string();
            println!("Thread nomi: {}", nomi);
            nomi
        })
        .unwrap();

    println!("{}", handle.join().unwrap());
    // Thread nomi: hisob-thread
    // hisob-thread

    // Thread ID — har thread unikal ID ga ega
    // Thread ID — у каждого потока уникальный ID
    let asosiy_id = thread::current().id();
    println!("Asosiy thread ID: {:?}", asosiy_id);

    let handle2 = thread::spawn(move || {
        let yangi_id = thread::current().id();
        println!("Yangi thread ID: {:?}", yangi_id);
        println!("Bir xilmi? {}", asosiy_id == yangi_id);
    });
    handle2.join().unwrap();
    // Asosiy thread ID: ThreadId(1)
    // Yangi thread ID: ThreadId(2)
    // Bir xilmi? false

    // Thread sleep
    // Сон потока
    println!("Kutilmoqda...");
    thread::sleep(Duration::from_millis(10));
    println!("Davom etmoqda");
    // Kutilmoqda...
    // Davom etmoqda

    // Thread yield — boshqa threadlarga imkoniyat berish
    // Thread yield — уступить другим потокам
    thread::yield_now();
    println!("yield_now bajardi");
}

// Scoped thread — 'static bo'lmagan ma'lumot bilan ishlash
// Scoped thread — работа с не-'static данными
fn scoped_thread_misollari() {

    let ma_lumotlar = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // thread::scope — scoped threadlar
    // thread::scope — скоупированные потоки
    let natija = thread::scope(|s| {
        // s.spawn — 'static bo'lmagan ma'lumot bilan
        // s.spawn — с не-'static данными
        let handle1 = s.spawn(|| {
            let yig: i32 = ma_lumotlar[..4].iter().sum();
            println!("Birinchi yarmi: {}", yig);
            yig
        });

        let handle2 = s.spawn(|| {
            let yig: i32 = ma_lumotlar[4..].iter().sum();
            println!("Ikkinchi yarmi: {}", yig);
            yig
        });

        // Scope tugashidan oldin barcha threadlar join bo'ladi
        // Все потоки join'ятся до конца скоупа
        handle1.join().unwrap() + handle2.join().unwrap()
    });

    println!("Jami: {} (kutilgan: {})", natija, ma_lumotlar.iter().sum::<i32>());
    // Birinchi yarmi: 10
    // Ikkinchi yarmi: 26
    // Jami: 36 (kutilgan: 36)

    // Scoped thread — &mut borrow ham mumkin
    // Scoped thread — &mut borrow тоже возможен
    let mut natijalar = vec![0i32; 4];

    thread::scope(|s| {
        let (chap, ong) = natijalar.split_at_mut(2);

        s.spawn(|| {
            chap[0] = 10;
            chap[1] = 20;
        });

        s.spawn(|| {
            ong[0] = 30;
            ong[1] = 40;
        });
    });

    println!("{:?}", natijalar);
    // [10, 20, 30, 40]
}

// thread_local! — har thread o'z qiymatiga ega
// thread_local! — у каждого потока своё значение
thread_local! {
    static THREAD_HISOB: std::cell::RefCell<u32> = std::cell::RefCell::new(0);
    static THREAD_NOMI: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
}

fn thread_local_misollari() {

    // Asosiy threadda
    // В главном потоке
    THREAD_HISOB.with(|h| {
        *h.borrow_mut() += 100;
        println!("Asosiy thread hisob: {}", h.borrow());
    });

    THREAD_NOMI.with(|n| {
        *n.borrow_mut() = "asosiy".to_string();
    });

    // Yangi thread — o'z qiymatiga ega
    // Новый поток — имеет своё значение
    let handle = thread::spawn(|| {
        THREAD_HISOB.with(|h| {
            *h.borrow_mut() += 200; // asosiy threaddan mustaqil!
            println!("Yangi thread hisob: {}", h.borrow());
        });

        THREAD_NOMI.with(|n| {
            *n.borrow_mut() = "yangi".to_string();
            println!("Yangi thread nomi: {}", n.borrow());
        });
    });
    handle.join().unwrap();

    // Asosiy thread o'z qiymatini saqlaydi
    // Главный поток сохраняет своё значение
    THREAD_HISOB.with(|h| {
        println!("Asosiy thread hisob (o'zgarmadi): {}", h.borrow());
    });
    THREAD_NOMI.with(|n| {
        println!("Asosiy thread nomi (o'zgarmadi): {}", n.borrow());
    });
    // Asosiy thread hisob: 100
    // Yangi thread hisob: 200     ← mustaqil!
    // Yangi thread nomi: yangi
    // Asosiy thread hisob (o'zgarmadi): 100
    // Asosiy thread nomi (o'zgarmadi): asosiy

    // Thread-local — performance optimizatsiyasi uchun
    // Thread-local — для оптимизации производительности
    thread_local! {
        static BUF: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::with_capacity(1024));
    }

    let handle2 = thread::spawn(|| {
        BUF.with(|buf| {
            buf.borrow_mut().extend_from_slice(b"thread data");
            println!("Buffer: {:?}", std::str::from_utf8(&buf.borrow()));
        });
    });
    handle2.join().unwrap();
    // Buffer: Ok("thread data")
}

// park — thread ni to'xtatish, unpark — davom ettirish
// park — остановить поток, unpark — продолжить
fn park_unpark_misollari() {
    let ishchi = thread::spawn(|| {
        println!("Ishchi: boshlanmoqda");

        for i in 0..3 {
            thread::park(); // Signal kutish
            println!("Ishchi: {} ish bajarildi", i + 1);
        }

        println!("Ishchi: tugadi");
        "ish tugadi"
    });

    // Ishchiga signal yuborish
    // Отправка сигналов рабочему потоку
    for i in 0..3 {
        thread::sleep(Duration::from_millis(20));
        println!("Asosiy: signal #{} yuborildi", i + 1);
        ishchi.thread().unpark();
    }

    println!("{}", ishchi.join().unwrap());
    // Ishchi: boshlanmoqda
    // Asosiy: signal #1 yuborildi
    // Ishchi: 1 ish bajarildi
    // Asosiy: signal #2 yuborildi
    // Ishchi: 2 ish bajarildi
    // Asosiy: signal #3 yuborildi
    // Ishchi: 3 ish bajarildi
    // Ishchi: tugadi
    // ish tugadi
}

// Ma'lumotni bo'lib parallel hisoblash
// Параллельное вычисление с разбиением данных
fn parallel_hisoblash<T, R, F>(
    ma_lumot: Vec<T>,
    thread_soni: usize,
    funksiya: F,
) -> Vec<R>
where
    T: Send + 'static + Clone,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
{
    let funksiya = Arc::new(funksiya);
    let bo_lim = (ma_lumot.len() + thread_soni - 1) / thread_soni;

    let handlar: Vec<_> = ma_lumot
        .into_iter()
        .enumerate()
        .collect::<Vec<_>>()
        .chunks(bo_lim)
        .map(|qism| {
            let qism: Vec<_> = qism.to_vec();
            let f = Arc::clone(&funksiya);
            thread::spawn(move || {
                qism.into_iter().map(|(_, x)| f(x)).collect::<Vec<R>>()
            })
        })
        .collect();

    handlar.into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect()
}

// Thread pool sodda implementatsiya
// Простая реализация пула потоков
use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    ishchilar: Vec<thread::JoinHandle<()>>,
    yuboruvchi: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(hajm: usize) -> Self {
        let (yuboruvchi, qabul_qiluvchi) = mpsc::channel::<Job>();
        let qabul_qiluvchi = Arc::new(Mutex::new(qabul_qiluvchi));

        let ishchilar: Vec<_> = (0..hajm).map(|id| {
            let qabul = Arc::clone(&qabul_qiluvchi);
            thread::Builder::new()
                .name(format!("pool-worker-{}", id))
                .spawn(move || {
                    loop {
                        let ish = qabul.lock().unwrap().recv();
                        match ish {
                            Ok(f) => {
                                println!("[Worker-{}] ish bajarilmoqda", id);
                                f();
                            }
                            Err(_) => {
                                println!("[Worker-{}] to'xtatildi", id);
                                break;
                            }
                        }
                    }
                })
                .unwrap()
        }).collect();

        ThreadPool { ishchilar, yuboruvchi }
    }

    fn bajar<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.yuboruvchi.send(Box::new(f)).unwrap();
    }

    fn tugatish(self) {
        drop(self.yuboruvchi); // channel yopiladi — workerlar to'xtaydi
        for ishchi in self.ishchilar {
            ishchi.join().unwrap();
        }
    }
}

fn thread_pool_misollari() {

    let pool = ThreadPool::new(3);

    for i in 0..6 {
        pool.bajar(move || {
            thread::sleep(Duration::from_millis(10));
            println!("Ish {} bajarildi", i);
        });
    }

    thread::sleep(Duration::from_millis(200));
    pool.tugatish();
    // [Worker-X] ish bajarilmoqda
    // Ish X bajarildi (tartib farqli bo'lishi mumkin)
}

// Fork-Join — bo'lib hisoblash, keyin birlashtirish
// Fork-Join — разбиение, вычисление, объединение
fn parallel_merge_sort(mut v: Vec<i32>) -> Vec<i32> {
    if v.len() <= 1 { return v; }

    let o_rta = v.len() / 2;
    let ong_qism = v.split_off(o_rta);
    let chap_qism = v;

    // Parallel sort
    let handle = thread::spawn(move || parallel_merge_sort(ong_qism));
    let chap_sorted = parallel_merge_sort(chap_qism);
    let ong_sorted = handle.join().unwrap();

    // Merge
    let mut natija = Vec::with_capacity(chap_sorted.len() + ong_sorted.len());
    let (mut i, mut j) = (0, 0);
    while i < chap_sorted.len() && j < ong_sorted.len() {
        if chap_sorted[i] <= ong_sorted[j] {
            natija.push(chap_sorted[i]); i += 1;
        } else {
            natija.push(ong_sorted[j]); j += 1;
        }
    }
    natija.extend_from_slice(&chap_sorted[i..]);
    natija.extend_from_slice(&ong_sorted[j..]);
    natija
}

fn real_hayot_misollari() {

    // 1. Parallel map
    // 1. Параллельный map
    let sonlar: Vec<i64> = (1..=8).collect();
    let boshlanish = Instant::now();
    let natijalar = parallel_hisoblash(sonlar, 4, |n| {
        thread::sleep(Duration::from_millis(5)); // sekin operatsiya
        n * n
    });
    println!("Parallel: {:?} ({:.1?})", natijalar, boshlanish.elapsed());
    // Parallel: [1, 4, 9, 16, 25, 36, 49, 64] (taxminan 10ms)

    // 2. Parallel merge sort
    // 2. Параллельная сортировка слиянием
    let tartiblanmagan: Vec<i32> = vec![64, 34, 25, 12, 22, 11, 90, 1, 45, 33];
    let tartiblangan = parallel_merge_sort(tartiblanmagan);
    println!("Sorted: {:?}", tartiblangan);
    // Sorted: [1, 11, 12, 22, 25, 33, 34, 45, 64, 90]

    // 3. Thread pool bilan vazifalar
    // 3. Задачи через пул потоков
    println!("\n--- Thread Pool ---");
    thread_pool_misollari();

    // 4. Barrier — barcha threadlar bir joyga yetguncha kutish
    // 4. Barrier — ожидание пока все потоки достигнут точки
    let to_siq = 4;
    let barrier = Arc::new(Barrier::new(to_siq));
    let natijalar = Arc::new(Mutex::new(vec![0i32; to_siq]));

    let handlar: Vec<_> = (0..to_siq).map(|i| {
        let b = Arc::clone(&barrier);
        let n = Arc::clone(&natijalar);
        thread::spawn(move || {
            // 1-bosqich: hisoblash
            let qiymat = (i as i32 + 1) * 10;
            thread::sleep(Duration::from_millis(i as u64 * 5));
            n.lock().unwrap()[i] = qiymat;
            println!("Thread {} 1-bosqich tugadi: {}", i, qiymat);

            // Barrier — barcha 1-bosqichni tugatguncha kutish
            // Barrier — ожидание завершения 1-го этапа всеми
            b.wait();
            println!("Thread {} 2-bosqichga o'tdi", i);
        })
    }).collect();

    for h in handlar { h.join().unwrap(); }
    println!("Natijalar: {:?}", natijalar.lock().unwrap());
    // Thread X 1-bosqich tugadi: XX
    // Thread X 2-bosqichga o'tdi
    // Natijalar: [10, 20, 30, 40]

    // 5. Condvar — shartli kutish
    // 5. Condvar — условное ожидание
    let juft = Arc::new((Mutex::new(false), Condvar::new()));
    let juft2 = Arc::clone(&juft);

    let ishchi = thread::spawn(move || {
        let (qulf, cvar) = &*juft2;
        let mut tayyor = qulf.lock().unwrap();
        while !*tayyor {
            println!("Ishchi: signal kutmoqda...");
            tayyor = cvar.wait(tayyor).unwrap();
        }
        println!("Ishchi: signal olindi!");
    });

    thread::sleep(Duration::from_millis(30));
    let (qulf, cvar) = &*juft;
    *qulf.lock().unwrap() = true;
    cvar.notify_one();
    println!("Asosiy: signal yuborildi");

    ishchi.join().unwrap();
    // Ishchi: signal kutmoqda...
    // Asosiy: signal yuborildi
    // Ishchi: signal olindi!
}

fn main() {

    println!("=== SPAWN VA JOIN ===");
    spawn_join_misollari();

    println!("\n=== THREAD BUILDER ===");
    thread_builder_misollari();

    println!("\n=== SCOPED THREADS ===");
    scoped_thread_misollari();

    println!("\n=== THREAD-LOCAL ===");
    thread_local_misollari();

    println!("\n=== PARK/UNPARK ===");
    park_unpark_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        THREAD ASOSLARI                                                                                       |
// #================================================================================================================================================#
// # |   1 | thread::spawn(|| ...)           | Yangi thread yaratish                      | Создание нового потока                                  |
// # |   2 | handle.join()                   | Thread tugashini kutish                    | Ожидание завершения потока                              |
// # |   3 | move || { ... }                 | Ownership thread ga o'tkazish              | Передача владения потоку                                |
// # |   4 | thread::Builder::new()          | Thread sozlash (nom, stack)                | Настройка потока (имя, стек)                            |
// # |   5 | thread::current().id()          | Thread identifikatori                      | Идентификатор потока                                    |
// # |   6 | thread::sleep(Duration)         | Thread ni to'xtatish                       | Остановка потока                                        |
// # |   7 | thread::yield_now()             | Boshqalarga imkoniyat berish               | Уступить другим потокам                                 |
// #================================================================================================================================================#
// # |                                        SCOPED VA THREAD-LOCAL                                                                                |
// #================================================================================================================================================#
// # |   8 | thread::scope(|s| ...)          | 'static bo'lmagan ma'lumot bilan           | С не-'static данными                                    |
// # |   9 | s.spawn(|| ...)                 | Scoped thread yaratish                     | Создание scoped потока                                  |
// # |  10 | thread_local! { static X: T }   | Har thread o'z qiymatiga ega               | У каждого потока своё значение                          |
// # |  11 | X.with(|val| ...)               | Thread-local qiymatga kirish               | Доступ к thread-local значению                          |
// #===============================================================================════════════════════════════════════════════════════════════════#
// # |                                        PARK/UNPARK VA PATTERNLAR                                                                             |
// #================================================================================================================================================#
// # |  12 | thread::park()                  | Thread ni to'xtatish (signal kutish)       | Остановка потока (ожидание сигнала)                     |
// # |  13 | handle.thread().unpark()        | Thread ni davom ettirish                   | Продолжение потока                                      |
// # |  14 | Barrier::new(n)                 | N thread bir joyga yetguncha kutish        | Ожидание N потоков в точке                              |
// # |  15 | Condvar::wait(guard)            | Shartli kutish                             | Условное ожидание                                       |
// # |  16 | Thread pool pattern             | Vazifalarni reuse qilish                   | Переиспользование для задач                             |
// # |  17 | Fork-Join pattern               | Bo'lib hisoblash, birlashtirish            | Разбиение, вычисление, объединение                      |
// # |  18 | Parallel map pattern            | Ma'lumotni parallel qayta ishlash          | Параллельная обработка данных                           |
// #================================================================================================================================================#