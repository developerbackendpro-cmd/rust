// #================================================================================================================================================#
// #                                                                REFCELL  |  CELL                                                                #
// #                            INTERIOR MUTABILITY — IMMUTABLE REFERENS ORQALI O'ZGARTIRISH. RUNTIME BORROW TEKSHIRUVI.                            #
// #                            INTERIOR MUTABILITY — ИЗМЕНЕНИЕ ЧЕРЕЗ ИММУТАБЕЛЬНУЮ ССЫЛКУ. ПРОВЕРКА ЗАИМСТВОВАНИЯ В RUNTIME.                       #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::cell::{Cell, RefCell, OnceCell, LazyCell, UnsafeCell};
use std::rc::Rc;
use std::fmt;

// Interior Mutability nima:
// Что такое Interior Mutability:
//
//   Rust qoidasi: &T → o'zgartirish mumkin emas
//   Правило Rust: &T → нельзя изменять
//   Interior Mutability: &T orqali ham o'zgartirish mumkin (runtime tekshiruv)
//   Interior Mutability: можно изменять даже через &T (проверка в runtime)
//
//   Cell<T>    — Copy turlar uchun, runtime xavfsiz, single thread
//   Cell<T>    — Для Copy типов, безопасно в runtime, один поток
//   RefCell<T> — Har qanday T uchun, runtime borrow check, single thread
//   RefCell<T> — Для любого T, проверка заимствования в runtime, один поток
//   OnceCell<T>— Bir marta yoziladigan, keyin faqat o'qish
//   OnceCell<T>— Записывается один раз, потом только чтение
//
//   Compile time vs Runtime borrow check:
//     &T / &mut T → Compile time (kompilyator tekshiradi)
//     RefCell<T>  → Runtime (dastur ishlashda tekshiriladi, panic mumkin)

fn cell_misollari() {

    // Cell<T> — Copy turlar uchun
    // Cell<T> — для Copy типов
    let c: Cell<i32> = Cell::new(0);
    println!("{}", c.get()); // 0

    // &c (immutable) orqali o'zgartirish!
    // Изменение через &c (иммутабельную)!
    c.set(42);
    println!("{}", c.get()); // 42

    // Cell — shared mutable state
    // Cell — разделяемое изменяемое состояние
    let shared: Cell<i32> = Cell::new(0);
    let r1: &Cell<i32> = &shared;
    let r2: &Cell<i32> = &shared;
    r1.set(10);
    r2.set(20);
    println!("{}", shared.get()); // 20

    // Cell::update — get + set qisqartmasi
    // Cell::update — сокращение get + set
    let c2: Cell<i32> = Cell::new(5);
    c2.update(|x| x * 2);
    println!("{}", c2.get()); // 10

    // Cell::take — qiymatni olib default qo'yish
    // Cell::take — взять значение и поставить default
    let c3: Cell<i32> = Cell::new(99);
    let olingan: i32 = c3.take();
    println!("{} {}", olingan, c3.get()); // 99 0

    // Cell bool bilan
    // Cell с bool
    let flag: Cell<bool> = Cell::new(false);
    flag.set(true);
    println!("{}", flag.get()); // true

    // Struct ichida Cell — hisoblash
    // Cell в структуре — подсчёт
    struct Hisoblagich {
        qiymat: i32,
        chaqiruv_soni: Cell<u32>,
    }

    impl Hisoblagich {
        fn new(qiymat: i32) -> Self {
            Hisoblagich { qiymat, chaqiruv_soni: Cell::new(0) }
        }

        fn qiymatni_ol(&self) -> i32 {
            self.chaqiruv_soni.set(self.chaqiruv_soni.get() + 1);
            self.qiymat
        }

        fn necha_marta(&self) -> u32 {
            self.chaqiruv_soni.get()
        }
    }

    let h = Hisoblagich::new(42);
    println!("{}", h.qiymatni_ol());
    println!("{}", h.qiymatni_ol());
    println!("{}", h.qiymatni_ol());
    println!("Chaqiruv: {}", h.necha_marta());
    // 42
    // 42
    // 42
    // Chaqiruv: 3
}

fn refcell_misollari() {

    // RefCell<T> — runtime borrow check
    // RefCell<T> — проверка заимствования в runtime
    let rc: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3]);

    // borrow() — &T olish (o'qish)
    // borrow() — получение &T (чтение)
    let r: std::cell::Ref<Vec<i32>> = rc.borrow();
    println!("{:?}", *r);
    println!("{}", r.len());
    drop(r); // borrow tugaydi
    // [1, 2, 3]
    // 3

    // borrow_mut() — &mut T olish (o'zgartirish)
    // borrow_mut() — получение &mut T (изменение)
    let mut rm: std::cell::RefMut<Vec<i32>> = rc.borrow_mut();
    rm.push(4);
    rm.push(5);
    drop(rm);
    println!("{:?}", rc.borrow());
    // [1, 2, 3, 4, 5]

    // try_borrow() — panic o'rniga Result
    // try_borrow() — Result вместо panic
    let try_r = rc.try_borrow();
    let try_rm = rc.try_borrow_mut();
    println!("{}", try_r.is_ok());
    println!("{}", try_rm.is_ok());
    // true
    // true

    // Bir vaqtda ko'p o'qish — OK
    // Одновременное чтение — OK
    let r1 = rc.borrow();
    let r2 = rc.borrow();
    println!("{} {}", r1.len(), r2.len());
    drop(r1);
    drop(r2);
    // 5 5

    // Bir vaqtda o'qish + yozish — PANIC!
    // Одновременное чтение + запись — PANIC!
    // let r3 = rc.borrow();
    // let rm3 = rc.borrow_mut(); // ← PANIC: already borrowed!

    // try_borrow bilan xavfsiz
    // Безопасно с try_borrow
    {
        let r3 = rc.borrow();
        match rc.try_borrow_mut() {
            Ok(_)  => println!("Muvaffaqiyat"),
            Err(e) => println!("Xato: {}", e),
        }
    }
    // Xato: already borrowed: BorrowMutError

    // RefCell::into_inner — qiymatni olish
    // RefCell::into_inner — получение значения
    let rc2: RefCell<String> = RefCell::new(String::from("salom"));
    let s: String = rc2.into_inner();
    println!("{}", s);
    // salom

    // Struct ichida RefCell
    // RefCell в структуре
    struct Kesh {
        hisoblangan: RefCell<Option<i32>>,
        kiritish: i32,
    }

    impl Kesh {
        fn new(kiritish: i32) -> Self {
            Kesh { hisoblangan: RefCell::new(None), kiritish }
        }

        // &self orqali cache — interior mutability
        // кэш через &self — interior mutability
        fn natija(&self) -> i32 {
            if self.hisoblangan.borrow().is_none() {
                println!("Hisoblanyapti...");
                let n = self.kiritish * self.kiritish;
                *self.hisoblangan.borrow_mut() = Some(n);
            }
            self.hisoblanned_qiymat()
        }

        fn hisoblanned_qiymat(&self) -> i32 {
            self.hisoblangan.borrow().unwrap()
        }
    }

    let k = Kesh::new(7);
    println!("{}", k.natija()); // hisoblaydi
    println!("{}", k.natija()); // keshdan oladi
    println!("{}", k.natija()); // keshdan oladi
    // Hisoblanyapti...
    // 49
    // 49
    // 49
}

fn oncecell_misollari() {

    // OnceCell<T> — bir marta yoziladi
    // OnceCell<T> — записывается один раз
    let once: OnceCell<String> = OnceCell::new();

    // get() — hali yozilmagan
    // get() — ещё не записано
    println!("{:?}", once.get()); // None

    // set() — birinchi marta yozish
    // set() — запись в первый раз
    once.set(String::from("bir marta")).unwrap();
    println!("{:?}", once.get()); // Some("bir marta")

    // set() — ikkinchi marta — Err
    // set() — второй раз — Err
    let natija = once.set(String::from("ikkinchi marta"));
    println!("{}", natija.is_err()); // true
    // None
    // Some("bir marta")
    // true

    // get_or_init() — yo'q bo'lsa init qilish
    // get_or_init() — инициализация если нет
    let once2: OnceCell<i32> = OnceCell::new();
    let v1 = once2.get_or_init(|| {
        println!("Init qilinmoqda...");
        42
    });
    let v2 = once2.get_or_init(|| {
        println!("Bu chiqmaydi!");
        99
    });
    println!("{} {}", v1, v2);
    // Init qilinmoqda...
    // 42 42

    // Struct ichida OnceCell — lazy initialization
    // OnceCell в структуре — ленивая инициализация
    struct Config {
        _rawdata: String,
        parsed: OnceCell<Vec<(String, String)>>,
    }

    impl Config {
        fn new(rawdata: &str) -> Self {
            Config { _rawdata: rawdata.to_string(), parsed: OnceCell::new() }
        }

        fn kalitlar(&self) -> &Vec<(String, String)> {
            self.parsed.get_or_init(|| {
                println!("Parse qilinmoqda...");
                vec![
                    ("host".to_string(), "localhost".to_string()),
                    ("port".to_string(), "8080".to_string()),
                ]
            })
        }
    }

    let cfg = Config::new("host=localhost\nport=8080");
    println!("{:?}", cfg.kalitlar());
    println!("{:?}", cfg.kalitlar()); // ikkinchi marta — parse yo'q
    // Parse qilinmoqda...
    // [("host", "localhost"), ("port", "8080")]
    // [("host", "localhost"), ("port", "8080")]
}

fn real_hayot_misollari() {

    // 1. Rc<RefCell<T>> — shared mutable graph
    // 1. Rc<RefCell<T>> — разделяемый изменяемый граф
    #[derive(Debug)]
    struct Node {
        qiymat: i32,
        chap: Option<Rc<RefCell<Node>>>,
        ong: Option<Rc<RefCell<Node>>>,
    }

    impl Node {
        fn new(qiymat: i32) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Node { qiymat, chap: None, ong: None }))
        }
    }

    let root = Node::new(1);
    let left = Node::new(2);
    let right = Node::new(3);

    root.borrow_mut().chap = Some(Rc::clone(&left));
    root.borrow_mut().ong = Some(Rc::clone(&right));
    left.borrow_mut().chap = Some(Node::new(4));

    println!("Root: {}", root.borrow().qiymat);
    println!("Left: {}", root.borrow().chap.as_ref().unwrap().borrow().qiymat);
    println!("Right: {}", root.borrow().ong.as_ref().unwrap().borrow().qiymat);
    // Root: 1
    // Left: 2
    // Right: 3

    // 2. Cell — statistika yig'ish
    // 2. Cell — сбор статистики
    struct API {
        so_rovlar: Cell<u64>,
        xatolar: Cell<u64>,
    }

    impl API {
        fn new() -> Self { API { so_rovlar: Cell::new(0), xatolar: Cell::new(0) } }

        fn so_rov(&self, muvaffaqiyatlimi: bool) {
            self.so_rovlar.update(|n| n + 1);
            if !muvaffaqiyatlimi {
                self.xatolar.update(|n| n + 1);
            }
        }

        fn statistika(&self) {
            println!("So'rovlar: {}, Xatolar: {}", self.so_rovlar.get(), self.xatolar.get());
        }
    }

    let api = API::new();
    api.so_rov(true);
    api.so_rov(true);
    api.so_rov(false);
    api.so_rov(true);
    api.so_rov(false);
    api.statistika();
    // So'rovlar: 5, Xatolar: 2

    // 3. RefCell — undo/redo
    // 3. RefCell — undo/redo
    struct Muharrir {
        matn: RefCell<String>,
        tarix: RefCell<Vec<String>>,
    }

    impl Muharrir {
        fn new() -> Self {
            Muharrir {
                matn: RefCell::new(String::new()),
                tarix: RefCell::new(vec![]),
            }
        }

        fn yoz(&self, belgi: &str) {
            self.tarix.borrow_mut().push(self.matn.borrow().clone());
            self.matn.borrow_mut().push_str(belgi);
        }

        fn bekor_qil(&self) {
            if let Some(avvalgi) = self.tarix.borrow_mut().pop() {
                *self.matn.borrow_mut() = avvalgi;
            }
        }

        fn hozir(&self) -> String {
            self.matn.borrow().clone()
        }
    }

    let m = Muharrir::new();
    m.yoz("sal");
    m.yoz("om");
    m.yoz(" dunyo");
    println!("{}", m.hozir()); // salom dunyo
    m.bekor_qil();
    println!("{}", m.hozir()); // salom
    m.bekor_qil();
    println!("{}", m.hozir()); // sal
    // salom dunyo
    // salom
    // sal
}

fn main() {

    println!("=== CELL ===");
    cell_misollari();

    println!("\n=== REFCELL ===");
    refcell_misollari();

    println!("\n=== ONCECELL ===");
    oncecell_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                  | Описание (RU)                                               |
// #================================================================================================================================================#
// # |   1 | Cell::new(val)            | Copy turlar uchun interior mutability         | Interior mutability для Copy типов                         |
// # |   2 | cell.get()                | Qiymatni olish (Copy)                         | Получение значения (Copy)                                  |
// # |   3 | cell.set(val)             | Qiymatni o'zgartirish                         | Изменение значения                                         |
// # |   4 | cell.update(|x| ...)      | get + set qisqartmasi                         | Сокращение get + set                                       |
// # |   5 | RefCell::new(val)         | Har qanday T uchun, runtime borrow check      | Для любого T, проверка в runtime                           |
// # |   6 | refcell.borrow()          | &T olish — Ref<T> (o'qish)                    | Получение &T — Ref<T> (чтение)                             |
// # |   7 | refcell.borrow_mut()      | &mut T olish — RefMut<T>                      | Получение &mut T — RefMut<T>                               |
// # |   8 | refcell.try_borrow()      | Panic o'rniga Result qaytaradi                | Возвращает Result вместо panic                             |
// # |   9 | OnceCell::new()           | Bir marta yoziladi, keyin o'qish              | Записывается один раз, потом только чтение                 |
// # |  10 | once.get_or_init(|| ...)  | Yo'q bo'lsa lazy init                         | Ленивая инициализация если нет значения                    |
// # |  11 | Rc<RefCell<T>>            | Shared mutable state, bitta thread            | Разделяемое состояние, один поток                          |
// # |  12 | Arc<Mutex<T>>             | Shared mutable state, ko'p thread             | Разделяемое состояние, много потоков                       |
// # |  13 | Runtime panic xavfi       | borrow + borrow_mut — PANIC!                  | borrow + borrow_mut — PANIC!                               |
// # |  14 | try_borrow xavfsiz        | Panic o'rniga Result                          | Result вместо panic                                        |
// #================================================================================================================================================#