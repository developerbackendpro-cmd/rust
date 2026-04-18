// #================================================================================================================================================#
// #                                                            SEND + SYNC                                                                         #
// #                    SEND VA SYNC — THREAD XAVFSIZLIGI MARKER TRAITLARI. AUTO TRAIT. NEGATIVE IMPL. PHANTOM DATA BILAN.                          #
// #                    SEND И SYNC — МАРКЕРНЫЕ ТРЕЙТЫ БЕЗОПАСНОСТИ ПОТОКОВ. AUTO TRAIT. NEGATIVE IMPL. С PHANTOM DATA.                             #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::sync::{Arc, Mutex, RwLock};
use std::cell::{Cell, RefCell, UnsafeCell};
use std::rc::Rc;
use std::marker::PhantomData;
use std::fmt;
use std::collections::HashMap;

// Send va Sync nima:
// Что такое Send и Sync:
//
//   Send  — T ni boshqa thread ga o'tkazish xavfsiz
//   Send  — безопасно перенести T в другой поток
//   Sync  — &T ni boshqa thread ga o'tkazish xavfsiz
//   Sync  — безопасно передать &T в другой поток
//
//   T: Sync  ≡  &T: Send
//   Agar T Sync bo'lsa — &T Send bo'ladi
//   Если T Sync — тогда &T является Send
//
//   Auto trait — kompilyator avtomatik implement qiladi
//   Auto trait — компилятор реализует автоматически
//   Agar barcha fieldlar Send/Sync bo'lsa — struct ham Send/Sync
//   Если все поля Send/Sync — struct тоже Send/Sync
//
// Send EMAS turlar:
// Типы НЕ Send:
//   Rc<T>       — reference hisob atomic emas
//   Rc<T>       — подсчёт ссылок не атомарный
//   *mut T      — raw pointer (unsafe)
//   RefCell<T>  — borrow runtime check, thread-safe emas
//
// Sync EMAS turlar:
// Типы НЕ Sync:
//   Cell<T>     — internal mutability, thread-safe emas
//   RefCell<T>  — borrow runtime check, thread-safe emas
//   UnsafeCell<T> — asosiy interior mutability, Sync emas
//   Rc<T>       — Send emas, Sync ham emas

fn send_misollari() {

    // Send turlar — thread ga o'tkazish mumkin
    // Send типы — можно передать в поток
    let n = 42i32;          // i32: Send
    let s = String::from("salom"); // String: Send
    let v = vec![1, 2, 3]; // Vec<T>: Send (T: Send bo'lsa)
    let b = Box::new(42);  // Box<T>: Send (T: Send bo'lsa)

    let h = std::thread::spawn(move || {
        println!("{} {} {:?} {}", n, s, v, b);
    });
    h.join().unwrap();
    // 42 salom [1, 2, 3] 42

    // Arc<T>: Send + Sync (T: Send + Sync bo'lsa)
    // Arc<T>: Send + Sync (если T: Send + Sync)
    let shared = Arc::new(Mutex::new(0i32));
    let shared2 = Arc::clone(&shared);

    let h2 = std::thread::spawn(move || {
        *shared2.lock().unwrap() += 10;
    });
    h2.join().unwrap();
    println!("{}", shared.lock().unwrap());
    // 10

    // Rc<T>: Send EMAS — compile xato bo'ladi:
    // Rc<T>: НЕ Send — будет ошибка компиляции:
    // let rc = Rc::new(42);
    // thread::spawn(move || println!("{}", rc)); // ← XATO!
    println!("Rc<T> thread ga o'tkazib bo'lmaydi (compile xato)");

    // Send ni tekshirish — funksiya bound bilan
    // Проверка Send — через bound функции
    fn faqat_send<T: Send>(qiymat: T) -> T { qiymat }

    let _ = faqat_send(42i32);
    let _ = faqat_send(String::from("ok"));
    let _ = faqat_send(vec![1, 2, 3]);
    // Rc<T> berib bo'lmaydi!
    println!("Send tekshiruvi o'tdi");
}

fn sync_misollari() {

    // Sync turlar — &T boshqa thread ga o'tkazish mumkin
    // Sync типы — &T можно передать в другой поток
    static GLOBAL: i32 = 42; // &'static i32: Send (i32: Sync)

    let h = std::thread::spawn(|| {
        println!("Global: {}", GLOBAL);
    });
    h.join().unwrap();
    // Global: 42

    // Arc<T> — T: Sync bo'lsa Sync
    // Arc<T> — Sync если T: Sync
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handlar = vec![];

    for i in 0..3 {
        let data = Arc::clone(&data);
        handlar.push(std::thread::spawn(move || {
            println!("Thread {}: {:?}", i, *data);
        }));
    }
    for h in handlar { h.join().unwrap(); }
    // Thread X: [1, 2, 3, 4, 5] (tartib farqli)

    // Mutex<T>: Sync — har vaqt faqat bitta thread kiradi
    // Mutex<T>: Sync — в любой момент только один поток
    let counter = Arc::new(Mutex::new(0u32));
    let mut handlar2 = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handlar2.push(std::thread::spawn(move || {
            for _ in 0..100 {
                *c.lock().unwrap() += 1;
            }
        }));
    }
    for h in handlar2 { h.join().unwrap(); }
    println!("Hisob: {}", counter.lock().unwrap()); // aniq 1000
    // Hisob: 1000

    // Cell<T>: Sync EMAS — thread ga o'tkazib bo'lmaydi:
    // Cell<T>: НЕ Sync — нельзя передать в поток:
    // let cell = Arc::new(Cell::new(0));
    // thread::spawn(move || cell.set(42)); // ← XATO!
    println!("Cell<T> Arc bilan ishlatib bo'lmaydi (compile xato)");

    // RwLock<T> — ko'p o'quvchi, bitta yozuvchi
    // RwLock<T> — много читателей, один писатель
    let rw = Arc::new(RwLock::new(vec![1, 2, 3]));
    let mut handlar3 = vec![];

    for i in 0..3 {
        let rw = Arc::clone(&rw);
        handlar3.push(std::thread::spawn(move || {
            let r = rw.read().unwrap();
            println!("O'quvchi {}: {:?}", i, *r);
        }));
    }
    for h in handlar3 { h.join().unwrap(); }

    rw.write().unwrap().push(4);
    println!("Yozilgandan keyin: {:?}", rw.read().unwrap());
    // O'quvchi X: [1, 2, 3]
    // Yozilgandan keyin: [1, 2, 3, 4]
}

// Auto trait — barcha fieldlar mos bo'lsa avtomatik
// Auto trait — автоматически если все поля подходят

// Bu struct avtomatik Send + Sync
// Эта структура автоматически Send + Sync
#[derive(Debug)]
struct AutoSendSync {
    n: i32,       // i32: Send + Sync
    s: String,    // String: Send + Sync
    v: Vec<f64>,  // Vec<f64>: Send + Sync
}

// Bu struct Send emas (Rc ichida)
// Эта структура НЕ Send (содержит Rc)
struct SendEmas {
    rc: Rc<i32>, // Rc: !Send
}

// Bu struct Sync emas (Cell ichida)
// Эта структура НЕ Sync (содержит Cell)
struct SyncEmas {
    cell: Cell<i32>, // Cell: !Sync
}

fn auto_trait_misollari() {

    // AutoSendSync — thread ga o'tkazish mumkin
    // AutoSendSync — можно передать в поток
    let a = AutoSendSync {
        n: 42,
        s: String::from("salom"),
        v: vec![1.0, 2.0, 3.0],
    };

    let h = std::thread::spawn(move || {
        println!("{:?}", a);
    });
    h.join().unwrap();
    // AutoSendSync { n: 42, s: "salom", v: [1.0, 2.0, 3.0] }

    // Tur tekshiruvi
    // Проверка типов
    fn send_tekshir<T: Send>(_: &T) { println!("T: Send") }
    fn sync_tekshir<T: Sync>(_: &T) { println!("T: Sync") }
    fn send_sync_tekshir<T: Send + Sync>(_: &T) { println!("T: Send + Sync") }

    let a2 = AutoSendSync { n: 1, s: String::new(), v: vec![] };
    send_tekshir(&a2);
    sync_tekshir(&a2);
    send_sync_tekshir(&a2);
    // T: Send
    // T: Sync
    // T: Send + Sync

    // SendEmas va SyncEmas — compile tekshiruvi
    // SendEmas и SyncEmas — проверка компилятором
    // Bu qatorlar compile xato beradi:
    // Эти строки дадут ошибку компиляции:
    // send_tekshir(&SendEmas { rc: Rc::new(1) });  // ← !Send
    // sync_tekshir(&SyncEmas { cell: Cell::new(1) }); // ← !Sync
    println!("Auto trait misollari OK");
}

// Ba'zida Send/Sync ni qo'lda implement qilish kerak
// Иногда нужно вручную реализовать Send/Sync

// Raw pointer — default !Send + !Sync
// Raw pointer — по умолчанию !Send + !Sync
struct ThreadXavfsizPointer<T> {
    ptr: *mut T,
    _marker: PhantomData<T>,
}

impl<T> ThreadXavfsizPointer<T> {
    fn new(qiymat: T) -> Self {
        ThreadXavfsizPointer {
            ptr: Box::into_raw(Box::new(qiymat)),
            _marker: PhantomData,
        }
    }

    fn qiymat(&self) -> &T {
        unsafe { &*self.ptr }
    }
}

impl<T> Drop for ThreadXavfsizPointer<T> {
    fn drop(&mut self) {
        unsafe { drop(Box::from_raw(self.ptr)); }
    }
}

// UNSAFE: Biz xavfsizligini ta'minlashimiz kerak
// UNSAFE: Мы должны обеспечить безопасность
unsafe impl<T: Send> Send for ThreadXavfsizPointer<T> {}
unsafe impl<T: Sync> Sync for ThreadXavfsizPointer<T> {}

// Negative impl — !Send yoki !Sync deb belgilash
// Negative impl — пометить как !Send или !Sync
struct FaqatAsosiyThread {
    // Bu struct faqat asosiy thread da ishlatilishi kerak
    // Эта структура должна использоваться только в главном потоке
    _unsend: PhantomData<*mut ()>, // *mut (): !Send + !Sync
}

impl FaqatAsosiyThread {
    fn new() -> Self {
        FaqatAsosiyThread { _unsend: PhantomData }
    }

    fn amaliyot(&self) {
        println!("Faqat asosiy thread da ishlaydi");
    }
}

// Bu qatorlar compile xato beradi:
// Эти строки дадут ошибку компиляции:
// thread::spawn(move || FaqatAsosiyThread::new()); // ← !Send!

fn manual_send_sync_misollari() {

    // ThreadXavfsizPointer — qo'lda Send implement qilingan
    // ThreadXavfsizPointer — вручную реализован Send
    let ptr = ThreadXavfsizPointer::new(42i32);
    println!("{}", ptr.qiymat());

    let h = std::thread::spawn(move || {
        println!("Thread da: {}", ptr.qiymat());
    });
    h.join().unwrap();
    // 42
    // Thread da: 42

    // FaqatAsosiyThread — !Send
    // FaqatAsosiyThread — !Send
    let asosiy = FaqatAsosiyThread::new();
    asosiy.amaliyot();
    // asosiy ni thread ga bera olmaydi!
    // asosiy нельзя передать в поток!
    // Faqat asosiy thread da ishlaydi
}

fn send_sync_jadvali() {

    println!("╔════════════════════════════════════════════════════════╗");
    println!("║                 SEND VA SYNC JADVALI                   ║");
    println!("╠══════════════════╦══════════╦══════════════════════════╣");
    println!("║ Tur              ║ Send     ║ Sync                     ║");
    println!("╠══════════════════╬══════════╬══════════════════════════╣");
    println!("║ i32, f64, bool   ║ ✅        ║ ✅                        ║");
    println!("║ String           ║ ✅        ║ ✅                        ║");
    println!("║ Vec<T: S+S>      ║ ✅        ║ ✅                        ║");
    println!("║ Box<T: S+S>      ║ ✅        ║ ✅                        ║");
    println!("║ Arc<T: S+S>      ║ ✅        ║ ✅                        ║");
    println!("║ Mutex<T: Send>   ║ ✅        ║ ✅                        ║");
    println!("║ RwLock<T: S+S>   ║ ✅          ✅                        ║");
    println!("║ Rc<T>            ║ ❌        ║ ❌                        ║");
    println!("║ Cell<T>          ║ ✅        ║ ❌                        ║");
    println!("║ RefCell<T>       ║ ✅        ║ ❌                        ║");
    println!("║ UnsafeCell<T>    ║ ✅        ║ ❌                        ║");
    println!("║ *mut T           ║ ❌        ║ ❌                        ║");
    println!("║ *const T         ║ ❌        ║ ❌                        ║");
    println!("╚══════════════════╩══════════╩══════════════════════════╝");
}

// Thread-safe singleton — Arc<Mutex<T>>
// Потокобезопасный синглтон — Arc<Mutex<T>>
use std::sync::OnceLock;

static GLOBAL_REGISTRY: OnceLock<Arc<Mutex<HashMap<String, String>>>> = OnceLock::new();

fn registry_ol() -> &'static Arc<Mutex<HashMap<String, String>>> {
    GLOBAL_REGISTRY.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

// Thread-safe producer-consumer
// Потокобезопасный producer-consumer
fn producer_consumer_misoli() {

    use std::sync::mpsc;

    let (yuboruvchi, qabul_qiluvchi) = mpsc::channel::<String>();
    let qabul = Arc::new(Mutex::new(qabul_qiluvchi));

    // Bir nechta producer
    // Несколько производителей
    let mut producerlar = vec![];
    for i in 0..3 {
        let tx = yuboruvchi.clone();
        producerlar.push(std::thread::spawn(move || {
            for j in 0..2 {
                let xabar = format!("Producer-{}: xabar-{}", i, j);
                tx.send(xabar).unwrap();
                std::thread::sleep(Duration::from_millis(5));
            }
        }));
    }

    // Bitta consumer
    // Один потребитель
    let consumer = {
        let qabul = Arc::clone(&qabul);
        std::thread::spawn(move || {
            let mut xabarlar = vec![];
            loop {
                match qabul.lock().unwrap().try_recv() {
                    Ok(xabar) => {
                        println!("Consumer: {}", xabar);
                        xabarlar.push(xabar);
                    }
                    Err(mpsc::TryRecvError::Empty) => {
                        std::thread::sleep(Duration::from_millis(1));
                    }
                    Err(mpsc::TryRecvError::Disconnected) => break,
                }
            }
            xabarlar
        })
    };

    // Producerlar tugaguncha kutish
    // Ожидание завершения производителей
    for p in producerlar { p.join().unwrap(); }
    drop(yuboruvchi); // channel yopiladi

    let xabarlar = consumer.join().unwrap();
    println!("Jami {} xabar qabul qilindi", xabarlar.len());
    // Consumer: Producer-X: xabar-X (tartib farqli)
    // Jami 6 xabar qabul qilindi
}

use std::time::Duration;

fn real_hayot_misollari() {

    // Global registry
    let registry = registry_ol();
    let r1 = Arc::clone(registry);
    let r2 = Arc::clone(registry);

    let h1 = std::thread::spawn(move || {
        r1.lock().unwrap().insert("host".to_string(), "localhost".to_string());
    });

    let h2 = std::thread::spawn(move || {
        r2.lock().unwrap().insert("port".to_string(), "8080".to_string());
    });

    h1.join().unwrap();
    h2.join().unwrap();

    let reg = registry.lock().unwrap();
    println!("host: {:?}", reg.get("host"));
    println!("port: {:?}", reg.get("port"));
    // host: Some("localhost")
    // port: Some("8080")
    drop(reg);

    // Producer-consumer
    println!("\n--- Producer-Consumer ---");
    producer_consumer_misoli();

    // Arc<RwLock<T>> — ko'p o'quvchi, bitta yozuvchi
    // Arc<RwLock<T>> — много читателей, один писатель
    let ma_lumot: Arc<RwLock<Vec<i32>>> = Arc::new(RwLock::new(vec![]));

    // Yozuvchi thread
    let yozuvchi = {
        let d = Arc::clone(&ma_lumot);
        std::thread::spawn(move || {
            for i in 0..5 {
                d.write().unwrap().push(i);
                std::thread::sleep(Duration::from_millis(5));
            }
        })
    };

    // O'quvchi threadlar
    let mut o_quvchilar = vec![];
    for i in 0..3 {
        let d = Arc::clone(&ma_lumot);
        o_quvchilar.push(std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(15));
            println!("O'quvchi {}: {:?}", i, *d.read().unwrap());
        }));
    }

    yozuvchi.join().unwrap();
    for o in o_quvchilar { o.join().unwrap(); }
    println!("Final: {:?}", ma_lumot.read().unwrap());
    // O'quvchi X: [0, 1, 2, ...] (qisman)
    // Final: [0, 1, 2, 3, 4]
}

fn main() {

    println!("=== SEND ===");
    send_misollari();

    println!("\n=== SYNC ===");
    sync_misollari();

    println!("\n=== AUTO TRAIT ===");
    auto_trait_misollari();

    println!("\n=== MANUAL SEND/SYNC ===");
    manual_send_sync_misollari();

    println!("\n=== SEND/SYNC JADVALI ===");
    send_sync_jadvali();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                               | Описание (RU)                                            |
// #================================================================================================================================================#
// # |                                        SEND VA SYNC                                                                                          |
// #================================================================================================================================================#
// # |   1 | T: Send                         | T ni thread ga o'tkazish xavfsiz          | Безопасно перенести T в другой поток                     |
// # |   2 | T: Sync                         | &T ni thread ga o'tkazish xavfsiz         | Безопасно передать &T в другой поток                     |
// # |   3 | T: Sync ≡ &T: Send              | Ekvivalentlik                             | Эквивалентность                                          |
// # |   4 | Auto trait                      | Barcha fieldlar mos → struct mos          | Все поля подходят → структура подходит                   |
// #================================================================================================================================================#
// # |                                        SEND TURLAR                                                                                           |
// #================================================================================================================================================#
// # |   5 | i32, String, Vec, Box           | Send ✅                                   | Send ✅                                                  |
// # |   6 | Arc<T: Send+Sync>               | Send ✅                                   | Send ✅                                                  |
// # |   7 | Mutex<T: Send>                  | Send ✅                                   | Send ✅                                                  |
// # |   8 | Rc<T>                           | Send ❌ — atomic emas                     | Send ❌ — не атомарный                                   |
// # |   9 | *mut T, *const T                | Send ❌ — raw pointer                     | Send ❌ — сырой указатель                                |
// #================================================================================================================================================#
// # |                                        SYNC TURLAR                                                                                           |
// #================================================================================================================================================#
// # |  10 | i32, String, Arc<T>             | Sync ✅                                   | Sync ✅                                                  |
// # |  11 | Mutex<T: Send>                  | Sync ✅                                   | Sync ✅                                                  |
// # |  12 | Cell<T>, RefCell<T>             | Sync ❌ — interior mutability             | Sync ❌ — внутренняя изменяемость                        |
// # |  13 | UnsafeCell<T>                   | Sync ❌ — asosiy interior mutability      | Sync ❌ — базовая внутренняя изменяемость                |
// #================================================================================================================================================#
// # |                                        MANUAL IMPL                                                                                           |
// #================================================================================================================================================#
// # |  14 | unsafe impl Send for T          | Qo'lda Send implement                     | Ручная реализация Send                                   |
// # |  15 | unsafe impl Sync for T          | Qo'lda Sync implement                     | Ручная реализация Sync                                   |
// # |  16 | PhantomData<*mut ()>            | !Send + !Sync qilish                      | Сделать !Send + !Sync                                    |
// # |  17 | unsafe — javobgarlik dasturchi  | Xavfsizlikni o'zing ta'minlaysan          | Сам обеспечиваешь безопасность                           |
// #================================================================================================================================================#