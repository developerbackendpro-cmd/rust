// #================================================================================================================================================#
// #                                                   ONCECELL  |  ONCELOCK  |  LAZYLOCK                                                           #
// #                     BIR MARTA INITSIALIZATSIYA. ONCECELL — SINGLE THREAD. ONCELOCK — MULTI THREAD. LAZYLOCK — LAZY INIT.                       #
// #                     ОДНОРАЗОВАЯ ИНИЦИАЛИЗАЦИЯ. ONCECELL — ОДИН ПОТОК. ONCELOCK — МНОГО ПОТОКОВ. LAZYLOCK — ЛЕНИВАЯ INIT.                       #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::cell::OnceCell;
use std::sync::{OnceLock, LazyLock};
use std::fmt;

// Uchala tur solishtirmasi:
// Сравнение трёх типов:
//
//   OnceCell<T>  — single thread, runtime init, std::cell
//   OnceCell<T>  — один поток, инициализация в runtime, std::cell
//   OnceLock<T>  — multi thread (Send+Sync), runtime init, std::sync
//   OnceLock<T>  — много потоков (Send+Sync), инициализация в runtime, std::sync
//   LazyLock<T>  — multi thread, lazy init (closure bilan), std::sync
//   LazyLock<T>  — много потоков, ленивая инициализация (с замыканием), std::sync
//
//   OnceCell  — OnceLock ga o'xshash lekin thread-safe emas
//   OnceLock  — bir marta yoziladigan global state
//   LazyLock  — OnceLock + avtomatik lazy init

fn oncecell_misollari() {

    // OnceCell::new() — bo'sh
    // OnceCell::new() — пустая
    let once: OnceCell<String> = OnceCell::new();

    // get() — hali yo'q
    // get() — ещё нет
    println!("{:?}", once.get()); // None

    // set() — bir marta yozish
    // set() — запись один раз
    once.set(String::from("birinchi qiymat")).unwrap();
    println!("{:?}", once.get()); // Some("birinchi qiymat")

    // set() — ikkinchi marta — Err
    // set() — второй раз — Err
    let natija = once.set(String::from("ikkinchi qiymat"));
    println!("{}", natija.is_err()); // true — o'zgartirish mumkin emas
    println!("{:?}", once.get());    // Some("birinchi qiymat") — o'zgarmadi
    // None
    // Some("birinchi qiymat")
    // true
    // Some("birinchi qiymat")

    // get_or_init() — yo'q bo'lsa init, bor bo'lsa shu
    // get_or_init() — если нет — init, если есть — вернуть
    let once2: OnceCell<i32> = OnceCell::new();
    let v1: &i32 = once2.get_or_init(|| {
        println!("Hisoblanyapti...");
        42
    });
    let v2: &i32 = once2.get_or_init(|| {
        println!("Bu chiqmaydi!");
        99
    });
    println!("{} {}", v1, v2); // 42 42 — faqat bitta hisoblash
    // Hisoblanyapti...
    // 42 42

    // get_or_try_init() — unstable (nightly), o'rniga manual yechim
    // get_or_try_init() — нестабильно (nightly), вместо него ручное решение
    let once3: OnceCell<i32> = OnceCell::new();
    // Manual try_init pattern
    if once3.get().is_none() {
        let _ = once3.set(100i32);
    }
    println!("{:?}", once3.get()); // Some(100)
    // Ikkinchi marta o'zgarmaydi
    // Второй раз не изменится
    let _ = once3.set(999i32); // Err — ignore
    println!("{:?}", once3.get()); // Some(100)
    // Some(100)
    // Some(100)

    // into_inner() — qiymatni olish (consume)
    // into_inner() — взять значение (consume)
    let once4: OnceCell<String> = OnceCell::new();
    once4.set(String::from("olish")).unwrap();
    let s: Option<String> = once4.into_inner();
    println!("{:?}", s); // Some("olish")
    // Some("olish")

    // Struct ichida OnceCell
    // OnceCell в структуре
    struct LazyConfig {
        raw: String,
        parsed: OnceCell<Vec<(String, String)>>,
    }

    impl LazyConfig {
        fn new(raw: &str) -> Self {
            LazyConfig { raw: raw.to_string(), parsed: OnceCell::new() }
        }

        fn kalitlar(&self) -> &Vec<(String, String)> {
            self.parsed.get_or_init(|| {
                println!("Parsing...");
                self.raw.lines()
                    .filter_map(|line| {
                        let mut parts = line.splitn(2, '=');
                        let k = parts.next()?.trim().to_string();
                        let v = parts.next()?.trim().to_string();
                        Some((k, v))
                    })
                    .collect()
            })
        }
    }

    let cfg = LazyConfig::new("host=localhost\nport=8080\ndebug=true");
    println!("{:?}", cfg.kalitlar()); // parsing qiladi
    println!("{:?}", cfg.kalitlar()); // keshdan oladi
    // Parsing...
    // [("host", "localhost"), ("port", "8080"), ("debug", "true")]
    // [("host", "localhost"), ("port", "8080"), ("debug", "true")]
}

// Static OnceLock — global bir marta initsializatsiya
// Static OnceLock — глобальная одноразовая инициализация
static GLOBAL_CONFIG: OnceLock<String> = OnceLock::new();
static MAX_THREAD: OnceLock<usize> = OnceLock::new();

fn global_config_ol() -> &'static str {
    GLOBAL_CONFIG.get_or_init(|| {
        println!("Global config initsializatsiya...");
        String::from("host=localhost,port=8080")
    })
}

fn oncelock_misollari() {

    // OnceLock — thread-safe OnceCell
    // OnceLock — потокобезопасный OnceCell
    let once: OnceLock<String> = OnceLock::new();

    // set() — bir marta
    // set() — один раз
    once.set(String::from("salom")).unwrap();
    println!("{:?}", once.get()); // Some("salom")

    // get_or_init() — lazy init
    // get_or_init() — ленивая инициализация
    let once2: OnceLock<Vec<i32>> = OnceLock::new();
    let v: &Vec<i32> = once2.get_or_init(|| vec![1, 2, 3, 4, 5]);
    println!("{:?}", v);
    // Some("salom")
    // [1, 2, 3, 4, 5]

    // Global OnceLock — ko'p thread bilan
    // Global OnceLock — с несколькими потоками
    let mut handles = vec![];
    for i in 0..3 {
        let h = std::thread::spawn(move || {
            let config: &str = global_config_ol();
            println!("Thread {}: {}", i, config);
        });
        handles.push(h);
    }
    for h in handles { h.join().unwrap(); }
    // Global config initsializatsiya... (faqat bir marta!)
    // Thread 0: host=localhost,port=8080
    // Thread 1: host=localhost,port=8080
    // Thread 2: host=localhost,port=8080

    // MAX_THREAD — dastur boshlanishida o'rnatish
    // MAX_THREAD — установка при старте программы
    MAX_THREAD.set(8).ok();
    println!("Max thread: {}", MAX_THREAD.get().unwrap());
    // Max thread: 8

    // OnceLock::into_inner() — qiymatni olish
    // OnceLock::into_inner() — взять значение
    let once3: OnceLock<i32> = OnceLock::new();
    once3.set(42).unwrap();
    println!("{:?}", once3.into_inner());
    // Some(42)
}

// Static LazyLock — birinchi marta ishlatilganda init bo'ladi
// Static LazyLock — инициализируется при первом использовании
static REGEX_PATTERN: LazyLock<String> = LazyLock::new(|| {
    println!("Regex pattern yaratilmoqda...");
    String::from(r"^\d{4}-\d{2}-\d{2}$")
});

static STANDART_SOZLAMALAR: LazyLock<std::collections::HashMap<&'static str, &'static str>> =
    LazyLock::new(|| {
        println!("Standart sozlamalar yuklanmoqda...");
        let mut m = std::collections::HashMap::new();
        m.insert("host", "localhost");
        m.insert("port", "8080");
        m.insert("debug", "false");
        m.insert("timeout", "30");
        m
    });

fn lazylock_misollari() {

    // LazyLock — birinchi ishlatishda init
    // LazyLock — инициализация при первом использовании
    println!("LazyLock yaratildi (hali init emas)");

    // Birinchi ishlatish — init qilinadi
    // Первое использование — инициализируется
    println!("Pattern: {}", *REGEX_PATTERN);
    println!("Pattern: {}", *REGEX_PATTERN); // ikkinchi marta — keshdan
    // LazyLock yaratildi (hali init emas)
    // Regex pattern yaratilmoqda...
    // Pattern: ^\d{4}-\d{2}-\d{2}$
    // Pattern: ^\d{4}-\d{2}-\d{2}$

    // Sozlamalar
    // Настройки
    println!("{:?}", STANDART_SOZLAMALAR.get("host"));
    println!("{:?}", STANDART_SOZLAMALAR.get("port"));
    // Standart sozlamalar yuklanmoqda...
    // Some("localhost")
    // Some("8080")

    // LazyLock — thread safe
    // LazyLock — потокобезопасный
    static QIMMAT_HISOBLASH: LazyLock<Vec<u64>> = LazyLock::new(|| {
        println!("Qimmat hisoblash...");
        (1u64..=10).map(|n| n * n).collect()
    });

    let mut handles = vec![];
    for i in 0..3 {
        let h = std::thread::spawn(move || {
            let natija: &Vec<u64> = &QIMMAT_HISOBLASH;
            println!("Thread {}: {:?}", i, &natija[..3]);
        });
        handles.push(h);
    }
    for h in handles { h.join().unwrap(); }
    // Qimmat hisoblash... (bir marta!)
    // Thread X: [1, 4, 9]  (tartib farqli bo'lishi mumkin)
}

// 1. Singleton pattern — OnceLock bilan
// 1. Паттерн Singleton — с OnceLock
struct DatabaseConnection {
    url: String,
    pool_size: usize,
}

impl DatabaseConnection {
    fn new(url: &str, pool_size: usize) -> Self {
        println!("DB ulanish yaratilmoqda: {}", url);
        DatabaseConnection { url: url.to_string(), pool_size }
    }

    fn so_rov(&self, query: &str) -> String {
        format!("[{}] {} natijasi", self.url, query)
    }
}

static DB: OnceLock<DatabaseConnection> = OnceLock::new();

fn db_olish() -> &'static DatabaseConnection {
    DB.get_or_init(|| DatabaseConnection::new("postgres://localhost/mydb", 10))
}

// 2. Konfiguratsiya — LazyLock bilan
// 2. Конфигурация — с LazyLock
#[derive(Debug)]
struct AppConfig {
    nomi: String,
    versiya: String,
    debug: bool,
    port: u16,
}

static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| {
    AppConfig {
        nomi: String::from("MyApp"),
        versiya: String::from("1.0.0"),
        debug: false,
        port: 8080,
    }
});

fn real_hayot_misollari() {

    // 1. Singleton DB
    // 1. Singleton DB
    let db1: &DatabaseConnection = db_olish();
    let db2: &DatabaseConnection = db_olish();
    println!("{}", db1.so_rov("SELECT * FROM users"));
    println!("Bir xil instance: {}", std::ptr::eq(db1, db2));
    // DB ulanish yaratilmoqda: postgres://localhost/mydb
    // [postgres://localhost/mydb] SELECT * FROM users natijasi
    // Bir xil instance: true

    // DB — ko'p threaddan
    // DB — из нескольких потоков
    let mut handles = vec![];
    for i in 0..3 {
        let h = std::thread::spawn(move || {
            let db: &DatabaseConnection = db_olish();
            println!("Thread {}: pool={}", i, db.pool_size);
        });
        handles.push(h);
    }
    for h in handles { h.join().unwrap(); }
    // Thread X: pool=10 (tartib farqli)

    // 2. App konfiguratsiya
    // 2. Конфигурация приложения
    println!("{:#?}", *APP_CONFIG);
    println!("Port: {}", APP_CONFIG.port);
    // AppConfig {
    //     nomi: "MyApp",
    //     versiya: "1.0.0",
    //     debug: false,
    //     port: 8080,
    // }
    // Port: 8080

    // 3. OnceCell — lazy field
    // 3. OnceCell — ленивое поле
    struct Hisobot {
        ma_lumotlar: Vec<i32>,
        kesh: OnceCell<(i32, i32, f64)>, // (min, max, ortacha)
    }

    impl Hisobot {
        fn new(ma_lumotlar: Vec<i32>) -> Self {
            Hisobot { ma_lumotlar, kesh: OnceCell::new() }
        }

        fn statistika(&self) -> (i32, i32, f64) {
            *self.kesh.get_or_init(|| {
                println!("Statistika hisoblanmoqda...");
                let min: i32 = *self.ma_lumotlar.iter().min().unwrap();
                let max: i32 = *self.ma_lumotlar.iter().max().unwrap();
                let ortacha: f64 = self.ma_lumotlar.iter().sum::<i32>() as f64
                    / self.ma_lumotlar.len() as f64;
                (min, max, ortacha)
            })
        }
    }

    let hisobot = Hisobot::new(vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3]);
    let (min, max, ort) = hisobot.statistika();
    println!("Min:{} Max:{} O'rtacha:{:.1}", min, max, ort);
    let (min2, max2, ort2) = hisobot.statistika(); // keshdan
    println!("Min:{} Max:{} O'rtacha:{:.1}", min2, max2, ort2);
    // Statistika hisoblanmoqda...
    // Min:1 Max:9 O'rtacha:3.9
    // Min:1 Max:9 O'rtacha:3.9
}

fn main() {

    println!("=== ONCECELL ===");
    oncecell_misollari();

    println!("\n=== ONCELOCK ===");
    oncelock_misollari();

    println!("\n=== LAZYLOCK ===");
    lazylock_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                | Tavsif (UZ)                                | Описание (RU)                                               |
// #================================================================================================================================================#
// # |                                        ONCECELL                                                                                              |
// #================================================================================================================================================#
// # |   1 | OnceCell::new()             | Bo'sh, single thread                       | Пустая, один поток                                          |
// # |   2 | .set(val)                   | Bir marta yozish, Ok/Err                   | Запись один раз, Ok/Err                                     |
// # |   3 | .get()                      | Option<&T>                                 | Option<&T>                                                  |
// # |   4 | .get_or_init(|| ...)        | Yo'q bo'lsa init, bor bo'lsa shu           | Если нет — init, если есть — вернуть                        |
// # |   5 | .get_or_try_init(|| ...)    | xato qaytarishi mumkin                     | Может вернуть ошибку                                        |
// # |   6 | .into_inner()               | Qiymatni consume qilib olish               | Взять значение с consume                                    |
// #================================================================================================================================================#
// # |                                        ONCELOCK                                                                                              |
// #================================================================================================================================================#
// # |   7 | OnceLock::new()             | Bo'sh, multi thread (Send+Sync)            | Пустая, много потоков (Send+Sync)                           |
// # |   8 | static X: OnceLock<T>       | Global bir marta initsializatsiya          | Глобальная одноразовая инициализация                        |
// # |   9 | .get_or_init(|| ...)        | Thread-safe lazy init                      | Потокобезопасная ленивая инициализация                      |
// # |  10 | Singleton pattern           | DB, Config, Registry uchun                 | Для DB, Config, Registry                                    |
// #================================================================================================================================================#
// # |                                        LAZYLOCK                                                                                              |
// #================================================================================================================================================#
// # |  11 | LazyLock::new(|| ...)       | Closure bilan, birinchi ishlatishda init   | С замыканием, инициализация при первом use                  |
// # |  12 | static X: LazyLock<T>       | Global lazy static                         | Глобальный lazy static                                      |
// # |  13 | *lazy_lock                  | Deref — qiymatga kirish                    | Deref — доступ к значению                                   |
// # |  14 | OnceLock vs LazyLock        | OnceLock — set() qo'lda, Lazy — avtomatik  | OnceLock — set() вручную, Lazy — авто                       |
// #================================================================================================================================================#