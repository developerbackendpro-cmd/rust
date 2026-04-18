// #================================================================================================================================================#
// #                                                             RC  |  ARC  |  WEAK                                                                #
// #                         RC — BITTA THREAD, KO'P OWNER. ARC — KO'P THREAD, KO'P OWNER. WEAK — DAVRIY REFERENSDAN HIMOYA.                        #
// #                         RC — ОДИН ПОТОК, МНОГО ВЛАДЕЛЬЦЕВ. ARC — МНОГО ПОТОКОВ. WEAK — ЗАЩИТА ОТ ЦИКЛИЧЕСКИХ ССЫЛОК.                           #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::rc::{Rc, Weak as RcWeak};
use std::sync::{Arc, Weak as ArcWeak};
use std::cell::RefCell;
use std::fmt;

// Rc<T>   — Reference Counted, bitta thread, non-atomic
// Rc<T>   — Подсчёт ссылок, один поток, не атомарный
// Arc<T>  — Atomically Reference Counted, ko'p thread, atomic
// Arc<T>  — Атомарный подсчёт ссылок, много потоков, атомарный
// Weak<T> — Kuchsiz referens, egalikni olmaydi, davriy referensdan himoya
// Weak<T> — Слабая ссылка, не владеет, защита от циклических ссылок
//
// Rc vs Arc:
//   Rc  — ~2x tez (non-atomic), faqat bitta thread
//   Arc — thread-safe, lekin kichik overhead (atomic ops)
//
// Clone semantikasi:
// Семантика Clone:
//   Rc::clone(&rc)  — yangi owner, hisob += 1 (chuqur nusxa emas!)
//   Arc::clone(&arc) — yangi owner, hisob += 1 (chuqur nusxa emas!)

fn rc_asosiy_misollari() {

    // Rc::new — birinchi owner
    // Rc::new — первый владелец
    let rc1: Rc<String> = Rc::new(String::from("salom"));
    println!("{}", rc1);
    println!("Hisob: {}", Rc::strong_count(&rc1));
    // salom
    // Hisob: 1

    // Rc::clone — yangi owner (deep copy emas!)
    // Rc::clone — новый владелец (не глубокая копия!)
    let rc2: Rc<String> = Rc::clone(&rc1);
    let rc3: Rc<String> = Rc::clone(&rc1);
    println!("Hisob: {}", Rc::strong_count(&rc1)); // 3
    println!("{} {} {}", rc1, rc2, rc3);
    // Hisob: 3
    // salom salom salom

    // Scope dan chiqganda hisob kamayadi
    // При выходе из области счётчик уменьшается
    {
        let rc4: Rc<String> = Rc::clone(&rc1);
        println!("Ichki hisob: {}", Rc::strong_count(&rc1)); // 4
    }
    println!("Tashqi hisob: {}", Rc::strong_count(&rc1)); // 3
    // Ichki hisob: 4
    // Tashqi hisob: 3

    // Rc::try_unwrap — faqat bitta owner bo'lsa qiymat olinadi
    // Rc::try_unwrap — значение берётся если один владелец
    let rc5: Rc<i32> = Rc::new(42);
    let rc6: Rc<i32> = Rc::clone(&rc5);
    println!("{:?}", Rc::try_unwrap(rc5)); // Err — 2 ta owner bor
    println!("{:?}", Rc::try_unwrap(rc6)); // Ok(42) — 1 ta owner
    // Err(42)
    // Ok(42)

    // Rc::get_mut — faqat bitta owner bo'lsa mut reference
    // Rc::get_mut — mut ссылка если один владелец
    let mut rc7: Rc<Vec<i32>> = Rc::new(vec![1, 2, 3]);
    if let Some(v) = Rc::get_mut(&mut rc7) {
        v.push(4);
    }
    println!("{:?}", rc7);
    // [1, 2, 3, 4]

    // Rc — Vec da turli ownerlar
    // Rc — разные владельцы в Vec
    let shared: Rc<Vec<i32>> = Rc::new(vec![1, 2, 3, 4, 5]);
    let ownerlar: Vec<Rc<Vec<i32>>> = (0..3).map(|_| Rc::clone(&shared)).collect();
    println!("Hisob: {}", Rc::strong_count(&shared)); // 4
    for owner in &ownerlar {
        print!("{:?} | ", owner);
    }
    println!();
    // Hisob: 4
    // [1, 2, 3, 4, 5] | [1, 2, 3, 4, 5] | [1, 2, 3, 4, 5] |
}

fn rc_refcell_misollari() {

    // Rc<RefCell<T>> — shared mutable state (bitta thread)
    // Rc<RefCell<T>> — разделяемое изменяемое состояние (один поток)
    let qiymat: Rc<RefCell<i32>> = Rc::new(RefCell::new(0));

    let q1: Rc<RefCell<i32>> = Rc::clone(&qiymat);
    let q2: Rc<RefCell<i32>> = Rc::clone(&qiymat);
    let q3: Rc<RefCell<i32>> = Rc::clone(&qiymat);

    *q1.borrow_mut() += 10;
    *q2.borrow_mut() += 20;
    *q3.borrow_mut() += 30;

    println!("{}", qiymat.borrow()); // 60
    // 60

    // Graf tugunlari — Rc<RefCell<T>> bilan
    // Узлы графа — с Rc<RefCell<T>>
    #[derive(Debug)]
    struct Tugun {
        qiymat: i32,
        qo_shnilar: Vec<Rc<RefCell<Tugun>>>,
    }

    impl Tugun {
        fn new(qiymat: i32) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Tugun { qiymat, qo_shnilar: vec![] }))
        }

        fn qo_shni_qo_sh(bu: &Rc<RefCell<Self>>, qo_shni: Rc<RefCell<Tugun>>) {
            bu.borrow_mut().qo_shnilar.push(qo_shni);
        }
    }

    let a = Tugun::new(1);
    let b = Tugun::new(2);
    let c = Tugun::new(3);

    Tugun::qo_shni_qo_sh(&a, Rc::clone(&b));
    Tugun::qo_shni_qo_sh(&a, Rc::clone(&c));
    Tugun::qo_shni_qo_sh(&b, Rc::clone(&c));

    println!("A qiymati: {}", a.borrow().qiymat);
    println!("A qo'shnilar soni: {}", a.borrow().qo_shnilar.len());
    // A qiymati: 1
    // A qo'shnilar soni: 2
}

fn weak_misollari() {

    // Weak — kuchsiz referens, egalikni olmaydi
    // Weak — слабая ссылка, не владеет
    let rc: Rc<String> = Rc::new(String::from("qiymat"));
    let weak: RcWeak<String> = Rc::downgrade(&rc);

    println!("Strong: {}", Rc::strong_count(&rc)); // 1
    println!("Weak:   {}", Rc::weak_count(&rc));   // 1
    // Strong: 1
    // Weak:   1

    // Weak::upgrade — qiymat hali bormi?
    // Weak::upgrade — значение ещё существует?
    if let Some(val) = weak.upgrade() {
        println!("{}", val);
    }
    // qiymat

    // rc drop bo'lgandan keyin — weak invalid
    // после drop rc — weak становится невалидным
    drop(rc);
    println!("{:?}", weak.upgrade()); // None — rc o'chirildi
    // None

    // Davriy referens muammosi va yechimi
    // Проблема циклических ссылок и решение
    #[derive(Debug)]
    struct Bola {
        ism: String,
        ota: RcWeak<Ota>, // Weak — davriy referensdan himoya
    }

    #[derive(Debug)]
    struct Ota {
        ism: String,
        bola: Option<Rc<Bola>>,
    }

    let ota: Rc<Ota> = Rc::new(Ota {
        ism: String::from("Salim"),
        bola: None,
    });

    let bola: Rc<Bola> = Rc::new(Bola {
        ism: String::from("Jamshid"),
        ota: Rc::downgrade(&ota), // Weak referens
    });

    // Ota ni o'zgartirish uchun — RefCell kerak bo'lardi
    // Для изменения Ota — нужен был бы RefCell
    println!("Ota: {}", ota.ism);
    println!("Bola: {}", bola.ism);
    if let Some(ota_ref) = bola.ota.upgrade() {
        println!("Bolaning otasi: {}", ota_ref.ism);
    }
    // Ota: Salim
    // Bola: Jamshid
    // Bolaning otasi: Salim
}

fn arc_misollari() {

    // Arc — thread-safe Rc
    // Arc — потокобезопасный Rc
    let arc1: Arc<String> = Arc::new(String::from("thread-safe"));
    let arc2: Arc<String> = Arc::clone(&arc1);
    let arc3: Arc<String> = Arc::clone(&arc1);

    println!("Hisob: {}", Arc::strong_count(&arc1)); // 3
    // Hisob: 3

    // Arc — thread da ishlatish
    // Arc — использование в потоке
    let shared: Arc<Vec<i32>> = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];

    for i in 0..3 {
        let shared_clone: Arc<Vec<i32>> = Arc::clone(&shared);
        let handle = std::thread::spawn(move || {
            let yig_indi: i32 = shared_clone.iter().sum();
            println!("Thread {}: yig'indi = {}", i, yig_indi);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    // Thread 0: yig'indi = 15
    // Thread 1: yig'indi = 15
    // Thread 2: yig'indi = 15

    // Arc<Mutex<T>> — thread-safe mutable state
    // Arc<Mutex<T>> — потокобезопасное изменяемое состояние
    use std::sync::Mutex;
    let hisob: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles2 = vec![];

    for _ in 0..5 {
        let hisob_clone: Arc<Mutex<i32>> = Arc::clone(&hisob);
        let h = std::thread::spawn(move || {
            let mut qiymat = hisob_clone.lock().unwrap();
            *qiymat += 1;
        });
        handles2.push(h);
    }

    for h in handles2 {
        h.join().unwrap();
    }
    println!("Hisob: {}", hisob.lock().unwrap()); // 5
    // Hisob: 5
}

fn arc_weak_misollari() {

    // Arc::downgrade — Weak<T> yaratish
    // Arc::downgrade — создание Weak<T>
    let arc: Arc<String> = Arc::new(String::from("arc qiymat"));
    let weak: ArcWeak<String> = Arc::downgrade(&arc);

    println!("Strong: {}", Arc::strong_count(&arc)); // 1
    println!("Weak:   {}", Arc::weak_count(&arc));   // 1
    // Strong: 1
    // Weak:   1

    // upgrade — hali bormi tekshirish
    // upgrade — проверка существования
    match weak.upgrade() {
        Some(val) => println!("{}", val),
        None      => println!("O'chirilgan"),
    }
    // arc qiymat

    drop(arc);
    println!("{:?}", weak.upgrade()); // None
    // None
}

fn real_hayot_misollari() {

    // 1. Kesh tizimi — Rc<RefCell<HashMap>>
    // 1. Система кэша — Rc<RefCell<HashMap>>
    use std::collections::HashMap;

    let kesh: Rc<RefCell<HashMap<String, String>>> =
        Rc::new(RefCell::new(HashMap::new()));

    let kesh1 = Rc::clone(&kesh);
    let kesh2 = Rc::clone(&kesh);

    kesh1.borrow_mut().insert("kalit1".to_string(), "qiymat1".to_string());
    kesh2.borrow_mut().insert("kalit2".to_string(), "qiymat2".to_string());

    println!("{:?}", kesh.borrow().get("kalit1"));
    println!("{:?}", kesh.borrow().get("kalit2"));
    // Some("qiymat1")
    // Some("qiymat2")

    // 2. Observer pattern — Weak referens bilan
    // 2. Паттерн Observer — со Weak ссылками
    trait Kuzatuvchi {
        fn xabarlash(&self, xabar: &str);
    }

    struct LogKuzatuvchi { id: u32 }
    impl Kuzatuvchi for LogKuzatuvchi {
        fn xabarlash(&self, xabar: &str) {
            println!("[Kuzatuvchi {}]: {}", self.id, xabar);
        }
    }

    struct Voqea {
        kuzatuvchilar: Vec<RcWeak<dyn Kuzatuvchi>>,
    }

    impl Voqea {
        fn new() -> Self { Voqea { kuzatuvchilar: vec![] } }

        fn qo_sh(&mut self, k: &Rc<dyn Kuzatuvchi>) {
            self.kuzatuvchilar.push(Rc::downgrade(k));
        }

        fn xabarlash(&mut self, xabar: &str) {
            self.kuzatuvchilar.retain(|weak| {
                if let Some(k) = weak.upgrade() {
                    k.xabarlash(xabar);
                    true
                } else {
                    false // o'chirilgan — ro'yxatdan chiqarish
                }
            });
        }
    }

    let mut voqea = Voqea::new();
    let k1: Rc<dyn Kuzatuvchi> = Rc::new(LogKuzatuvchi { id: 1 });
    let k2: Rc<dyn Kuzatuvchi> = Rc::new(LogKuzatuvchi { id: 2 });

    voqea.qo_sh(&k1);
    voqea.qo_sh(&k2);
    voqea.xabarlash("Birinchi voqea");
    // [Kuzatuvchi 1]: Birinchi voqea
    // [Kuzatuvchi 2]: Birinchi voqea

    drop(k2); // k2 o'chirildi
    voqea.xabarlash("Ikkinchi voqea");
    // [Kuzatuvchi 1]: Ikkinchi voqea

    // 3. Arc<RwLock<T>> — ko'p o'quvchi, bitta yozuvchi
    // 3. Arc<RwLock<T>> — много читателей, один писатель
    use std::sync::RwLock;
    let ma_lumot: Arc<RwLock<Vec<i32>>> = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Ko'p thread bir vaqtda o'qiydi
    // Несколько потоков читают одновременно
    let mut o_qish_handles = vec![];
    for i in 0..3 {
        let ma_lumot_clone = Arc::clone(&ma_lumot);
        let h = std::thread::spawn(move || {
            let o_qish = ma_lumot_clone.read().unwrap();
            println!("O'quvchi {}: {:?}", i, *o_qish);
        });
        o_qish_handles.push(h);
    }
    for h in o_qish_handles { h.join().unwrap(); }

    // Bitta thread yozadi
    // Один поток пишет
    {
        let mut yozish = ma_lumot.write().unwrap();
        yozish.push(4);
        yozish.push(5);
    }
    println!("Yangilangan: {:?}", ma_lumot.read().unwrap());
    // O'quvchi 0: [1, 2, 3]
    // O'quvchi 1: [1, 2, 3]
    // O'quvchi 2: [1, 2, 3]
    // Yangilangan: [1, 2, 3, 4, 5]
}

fn main() {

    println!("=== RC ASOSIY ===");
    rc_asosiy_misollari();

    println!("\n=== RC + REFCELL ===");
    rc_refcell_misollari();

    println!("\n=== WEAK ===");
    weak_misollari();

    println!("\n=== ARC ===");
    arc_misollari();

    println!("\n=== ARC WEAK ===");
    arc_weak_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                  | Описание (RU)                                               |
// #================================================================================================================================================#
// # |   1 | Rc::new(val)              | Bitta thread, ko'p owner                     | Один поток, много владельцев                                |
// # |   2 | Rc::clone(&rc)            | Yangi owner, hisob += 1                      | Новый владелец, счётчик += 1                                |
// # |   3 | Rc::strong_count(&rc)     | Ownerlar soni                                | Количество владельцев                                       |
// # |   4 | Rc::weak_count(&rc)       | Weak referenslar soni                        | Количество слабых ссылок                                    |
// # |   5 | Rc::downgrade(&rc)        | Weak<T> yaratish                             | Создание Weak<T>                                            |
// # |   6 | weak.upgrade()            | Option<Rc<T>> — hali bormi?                  | Option<Rc<T>> — ещё существует?                             |
// # |   7 | Rc<RefCell<T>>            | Shared mutable state, bitta thread           | Разделяемое состояние, один поток                           |
// # |   8 | Arc::new(val)             | Ko'p thread, thread-safe                     | Много потоков, потокобезопасный                             |
// # |   9 | Arc::clone(&arc)          | Rc::clone ga o'xshash, atomic                | Аналогично Rc::clone, атомарный                             |
// # |  10 | Arc<Mutex<T>>             | Thread-safe mutable state                    | Потокобезопасное изменяемое состояние                       |
// # |  11 | Arc<RwLock<T>>            | Ko'p o'quvchi, bitta yozuvchi                | Много читателей, один писатель                              |
// # |  12 | Rc vs Arc                 | Rc — ~2x tez, faqat bitta thread             | Rc — ~2x быстрее, только один поток                         |
// # |  13 | Weak — davriy referens    | Cycle'dan himoya, egalikni olmaydi           | Защита от циклов, не владеет                                |
// #================================================================================================================================================#