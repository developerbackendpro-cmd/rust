// #================================================================================================================================================#
// #                                                          COPY  |  CLONE                                                                        #
// #                     COPY — STACK DA BITLI NUSXA. CLONE — CHUQUR NUSXA, QO'LDA CHAQIRILADI. IKKALASI OWNERSHIP QOIDASINI BUZADI.                #
// #                     COPY — ПОБИТОВАЯ КОПИЯ НА СТЕКЕ. CLONE — ГЛУБОКАЯ КОПИЯ, ВЫЗЫВАЕТСЯ ВРУЧНУЮ. ОБА НАРУШАЮТ ПРАВИЛО ВЛАДЕНИЯ.                #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// Copy va Clone farqi:
// Разница Copy и Clone:
//
//   Copy  — implicit (avtomatik), stack da, zero-cost
//   Copy  — неявная (автоматическая), на стеке, без затрат
//   Clone — explicit (.clone() chaqiriladi), heap ni ham nusxalaydi
//   Clone — явная (.clone() вызывается), копирует и кучу
//
// Copy qoidalari:
// Правила Copy:
//   1. Faqat stack da saqlanadigan turlar Copy bo'lishi mumkin
//      Только типы хранящиеся на стеке могут быть Copy
//   2. Drop implement qilgan tur Copy bo'la olmaydi
//      Тип реализующий Drop не может быть Copy
//   3. Barcha fieldlar Copy bo'lsa — struct Copy bo'lishi mumkin
//      Если все поля Copy — структура может быть Copy
//   4. String, Vec, Box — Copy emas (heap allocate qiladi)
//      String, Vec, Box — не Copy (выделяют память в куче)
//
// Clone qoidalari:
// Правила Clone:
//   1. Copy: Clone bo'lishi shart (Copy: Clone supertrait)
//      Copy: должен быть Clone (Copy: Clone супертрейт)
//   2. Har qanday tur Clone bo'lishi mumkin
//      Любой тип может быть Clone
//   3. Clone qimmat bo'lishi mumkin (heap nusxalash)
//      Clone может быть дорогим (копирование кучи)

fn copy_builtin_misollari() {

    // Barcha integer turlar — Copy
    // Все целочисленные типы — Copy
    let x: i32 = 42;
    let y: i32 = x;  // Copy — x hali ishlatiladi
    // Copy — x всё ещё доступен
    println!("{} {}", x, y);
    // 42 42

    // Float turlar — Copy
    // Float типы — Copy
    let a: f64 = 3.14;
    let b: f64 = a;
    println!("{} {}", a, b);
    // 3.14 3.14

    // bool — Copy
    // bool — Copy
    let t: bool = true;
    let f_val: bool = t;
    println!("{} {}", t, f_val);
    // true true

    // char — Copy
    // char — Copy
    let c: char = 'R';
    let d: char = c;
    println!("{} {}", c, d);
    // R R

    // &T (reference) — Copy
    // &T (reference) — Copy
    let s: String = String::from("salom");
    let r1: &String = &s;
    let r2: &String = r1;  // r1 Copy bo'ldi
    // r1 скопировался
    println!("{} {}", r1, r2);
    // salom salom

    // Tuple — barcha elementlar Copy bo'lsa
    // Tuple — если все элементы Copy
    let t: (i32, f64, bool) = (1, 2.0, true);
    let t2: (i32, f64, bool) = t;
    println!("{:?} {:?}", t, t2);
    // (1, 2.0, true) (1, 2.0, true)

    // Array — element Copy bo'lsa
    // Array — если элемент Copy
    let arr: [i32; 3] = [1, 2, 3];
    let arr2: [i32; 3] = arr;
    println!("{:?} {:?}", arr, arr2);
    // [1, 2, 3] [1, 2, 3]

    // Funksiya ko'rsatkichi — Copy
    // Указатель на функцию — Copy
    fn kvadrat(x: i32) -> i32 { x * x }
    let f1: fn(i32) -> i32 = kvadrat;
    let f2: fn(i32) -> i32 = f1;
    println!("{} {}", f1(5), f2(5));
    // 25 25
}

fn copy_bolmagan_misollari() {

    // String — Copy emas (heap)
    // String — не Copy (куча)
    let s1: String = String::from("salom");
    // let s2 = s1;  // s1 moved — s1 endi ishlatib bo'lmaydi
    // let s2 = s1;  // s1 перемещён — s1 больше недоступен
    let s2: String = s1.clone();  // Clone kerak
    // нужен Clone
    let s3: String = String::from("salom");  // yoki yangi yaratish
    // или создать новый
    println!("{} {}", s2, s3);
    // salom salom

    // Vec — Copy emas (heap)
    // Vec — не Copy (куча)
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<i32> = v1.clone();
    println!("{:?} {:?}", v1, v2);
    // [1, 2, 3] [1, 2, 3]

    // Box — Copy emas (heap)
    // Box — не Copy (куча)
    let b1: Box<i32> = Box::new(42);
    let b2: Box<i32> = b1.clone();
    println!("{} {}", b1, b2);
    // 42 42

    // HashMap — Copy emas
    // HashMap — не Copy
    use std::collections::HashMap;
    let mut m1: HashMap<&str, i32> = HashMap::new();
    m1.insert("bir", 1);
    let m2: HashMap<&str, i32> = m1.clone();
    println!("{:?}", m2.get("bir"));
    // Some(1)
}

// Copy + Clone — ikkalasi birga kerak (Copy: Clone)
// Copy + Clone — нужны оба вместе (Copy: Clone)
#[derive(Debug, Copy, Clone, PartialEq)]
struct Nuqta {
    x: f64,
    y: f64,
}

impl Nuqta {
    fn new(x: f64, y: f64) -> Self {
        Nuqta { x, y }
    }

    fn masofa(&self, boshqa: &Nuqta) -> f64 {
        let dx: f64 = self.x - boshqa.x;
        let dy: f64 = self.y - boshqa.y;
        (dx * dx + dy * dy).sqrt()
    }
}

impl fmt::Display for Nuqta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

// Copy bo'la olmaydi — String field bor
// Не может быть Copy — есть поле String
#[derive(Debug, Clone)]
struct Foydalanuvchi {
    id: u32,
    ism: String,      // String — Copy emas!
    email: String,    // String — не Copy!
    yosh: u8,
}

impl Foydalanuvchi {
    fn new(id: u32, ism: &str, email: &str, yosh: u8) -> Self {
        Foydalanuvchi {
            id,
            ism: ism.to_string(),
            email: email.to_string(),
            yosh,
        }
    }
}

impl fmt::Display for Foydalanuvchi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. {} <{}>", self.id, self.ism, self.email)
    }
}

// Copy bilan — barcha fieldlar Copy
// С Copy — все поля Copy
#[derive(Debug, Copy, Clone)]
struct Rang {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Rang {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Rang { r, g, b, a: 255 }
    }
    fn shaffof(mut self, a: u8) -> Self {
        self.a = a;
        self
    }
}

fn custom_copy_clone_misollari() {

    // Nuqta — Copy
    // Nuqta — Copy
    let n1: Nuqta = Nuqta::new(0.0, 0.0);
    let n2: Nuqta = n1;  // Copy — n1 hali bor
    // Copy — n1 всё ещё доступен
    let n3: Nuqta = Nuqta::new(3.0, 4.0);
    println!("{} {} {}", n1, n2, n3);
    println!("Masofa: {:.1}", n1.masofa(&n3));
    // (0.0, 0.0) (0.0, 0.0) (3.0, 4.0)
    // Masofa: 5.0

    // Nuqta — funksiyaga berish (Copy)
    // Nuqta — передача в функцию (Copy)
    fn nuqtani_chiqar(n: Nuqta) {
        println!("Nuqta: {}", n);
    }
    nuqtani_chiqar(n1);
    println!("{}", n1);  // n1 hali bor (copy bo'ldi)
    // n1 всё ещё доступен (скопировался)
    // Nuqta: (0.0, 0.0)
    // (0.0, 0.0)

    // Foydalanuvchi — faqat Clone (Copy emas)
    // Foydalanuvchi — только Clone (не Copy)
    let f1: Foydalanuvchi = Foydalanuvchi::new(1, "Dilshod", "d@mail.com", 22);
    let f2: Foydalanuvchi = f1.clone();
    println!("{}", f1);
    println!("{}", f2);
    // 1. Dilshod <d@mail.com>
    // 1. Dilshod <d@mail.com>

    // Rang — Copy
    // Rang — Copy
    let qizil: Rang = Rang::new(255, 0, 0);
    let shaffof_qizil: Rang = qizil.shaffof(128);
    println!("{:?}", qizil);
    println!("{:?}", shaffof_qizil);
    // Rang { r: 255, g: 0, b: 0, a: 255 }
    // Rang { r: 255, g: 0, b: 0, a: 128 }
}

fn clone_chuqur_nusxa() {

    // String — clone chuqur nusxa
    // String — clone глубокая копия
    let s1: String = String::from("salom");
    let s2: String = s1.clone();
    // s1 va s2 — alohida xotira manzillari
    // s1 и s2 — разные адреса памяти
    println!("{} {}", s1, s2);
    println!("Bir xilmi: {}", std::ptr::eq(s1.as_ptr(), s2.as_ptr()));
    // salom salom
    // Bir xilmi: false  ← alohida xotira!

    // Vec — clone chuqur nusxa
    // Vec — clone глубокая копия
    let v1: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<i32> = v1.clone();
    v2.push(4);
    println!("{:?} {:?}", v1, v2);
    // [1, 2, 3] [1, 2, 3, 4]  ← v1 o'zgarmadi

    // clone() vs to_owned() — &str uchun
    // clone() vs to_owned() — для &str
    let s: &str = "salom";
    let owned: String = s.to_owned();  // &str → String
    let owned2: String = s.to_string(); // &str → String
    println!("{} {}", owned, owned2);
    // salom salom

    // Nested clone — ichma-ich
    // Nested clone — вложенное
    let v_of_v: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
    let mut v2_of_v: Vec<Vec<i32>> = v_of_v.clone();
    v2_of_v[0].push(99);
    println!("{:?}", v_of_v);
    println!("{:?}", v2_of_v);
    // [[1, 2], [3, 4]]
    // [[1, 2, 99], [3, 4]]  ← v_of_v o'zgarmadi
}

// Clone ni qo'lda implement qilish
// Ручная реализация Clone
#[derive(Debug)]
struct DeepStruct {
    id: u32,
    ma_lumotlar: Vec<String>,
    ichki: Box<i32>,
}

impl Clone for DeepStruct {
    fn clone(&self) -> Self {
        println!("DeepStruct clone chaqirildi (id={})", self.id);
        DeepStruct {
            id: self.id,
            ma_lumotlar: self.ma_lumotlar.clone(),
            ichki: self.ichki.clone(),
        }
    }
}

fn manual_clone_misoli() {
    let d1: DeepStruct = DeepStruct {
        id: 1,
        ma_lumotlar: vec!["bir".to_string(), "ikki".to_string()],
        ichki: Box::new(42),
    };

    let d2: DeepStruct = d1.clone();
    println!("{:?}", d1);
    println!("{:?}", d2);
    // DeepStruct clone chaqirildi (id=1)
    // DeepStruct { id: 1, ma_lumotlar: ["bir", "ikki"], ichki: 42 }
    // DeepStruct { id: 1, ma_lumotlar: ["bir", "ikki"], ichki: 42 }
}

fn real_hayot_misollari() {

    // 1. Copy — samarali, clone qilmasdan
    // 1. Copy — эффективно, без клонирования
    #[derive(Debug, Copy, Clone)]
    struct Koordinat {
        lat: f64,
        lon: f64,
    }

    let toshkent: Koordinat = Koordinat { lat: 41.2995, lon: 69.2401 };
    let koordinatlar: Vec<Koordinat> = vec![toshkent, toshkent, toshkent];
    // Copy — toshkent hali ishlatiladi
    // Copy — toshkent всё ещё доступен
    println!("{:?}", toshkent);
    println!("{}", koordinatlar.len());
    // Koordinat { lat: 41.2995, lon: 69.2401 }
    // 3

    // 2. Clone — thread ga ma'lumot yuborish
    // 2. Clone — отправка данных в поток
    let konfiguratsiya: Vec<String> = vec![
        "host=localhost".to_string(),
        "port=8080".to_string(),
    ];

    let konfig_clone: Vec<String> = konfiguratsiya.clone();
    let handle = std::thread::spawn(move || {
        for k in &konfig_clone {
            println!("Thread: {}", k);
        }
    });
    handle.join().unwrap();
    println!("Asl: {:?}", konfiguratsiya);
    // Thread: host=localhost
    // Thread: port=8080
    // Asl: ["host=localhost", "port=8080"]

    // 3. Clone — undo/redo pattern
    // 3. Clone — паттерн undo/redo
    #[derive(Debug, Clone)]
    struct RedaktorHolat {
        matn: String,
        kursor: usize,
    }

    let mut holat = RedaktorHolat { matn: String::from("salom"), kursor: 5 };
    let tarix: Vec<RedaktorHolat> = vec![holat.clone()];

    holat.matn.push_str(" dunyo");
    holat.kursor = 11;
    println!("Joriy: {:?}", holat);
    println!("Tarix: {:?}", tarix[0]);
    // Joriy: RedaktorHolat { matn: "salom dunyo", kursor: 11 }
    // Tarix: RedaktorHolat { matn: "salom", kursor: 5 }

    // 4. Rc::clone() — clone emas, reference hisoblash
    // 4. Rc::clone() — не клонирование, подсчёт ссылок
    use std::rc::Rc;
    let rc1: Rc<String> = Rc::new(String::from("shared"));
    let rc2: Rc<String> = Rc::clone(&rc1);  // clone emas — pointer copy
    // не clone — копия указателя
    println!("{} {}", rc1, rc2);
    println!("RC soni: {}", Rc::strong_count(&rc1));
    // shared shared
    // RC soni: 2
}

fn main() {

    println!("=== COPY BUILT-IN ===");
    copy_builtin_misollari();

    println!("\n=== COPY BOLMAGAN ===");
    copy_bolmagan_misollari();

    println!("\n=== CUSTOM COPY VA CLONE ===");
    custom_copy_clone_misollari();

    println!("\n=== CLONE CHUQUR NUSXA ===");
    clone_chuqur_nusxa();

    println!("\n=== MANUAL CLONE ===");
    manual_clone_misoli();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                    | Описание (RU)                                              |
// #================================================================================================================================================#
// # |                                       COPY TRAIT                                                                                             |
// #================================================================================================================================================#
// # |   1 | #[derive(Copy, Clone)]   | Copy + Clone birga (Copy: Clone)               | Copy + Clone вместе (Copy: Clone)                          |
// # |   2 | i32, f64, bool, char     | Barcha primitiv turlar — Copy                  | Все примитивные типы — Copy                                |
// # |   3 | &T reference             | Reference — Copy                               | Ссылка — Copy                                              |
// # |   4 | (T, U) tuple             | Elementlar Copy bo'lsa — Copy                  | Если элементы Copy — Copy                                  |
// # |   5 | [T; N] array             | Element Copy bo'lsa — Copy                     | Если элемент Copy — Copy                                   |
// # |   6 | String, Vec, Box         | Copy EMAS — heap allocate qiladi               | НЕ Copy — выделяет память в куче                           |
// # |   7 | Drop + Copy              | Drop implement qilsa — Copy bo'la olmaydi      | Реализует Drop — не может быть Copy                        |
// # |   8 | let y = x;               | Copy bo'lsa — x hali ishlatiladi               | Если Copy — x всё ещё доступен                             |
// #================================================================================================================================================#
// # |                                       CLONE TRAIT                                                                                            |
// #================================================================================================================================================#
// # |   9 | #[derive(Clone)]         | Clone avtomatik implement                      | Автоматическая реализация Clone                            |
// # |  10 | .clone()                 | Chuqur nusxa — heap ni ham nusxalaydi          | Глубокая копия — копирует и кучу                           |
// # |  11 | impl Clone for T         | Clone ni qo'lda implement qilish               | Ручная реализация Clone                                    |
// # |  12 | Rc::clone(&rc)           | Clone emas — pointer nusxa (arzon)             | Не clone — копия указателя (дёшево)                        |
// # |  13 | Arc::clone(&arc)         | Clone emas — atomic pointer nusxa              | Не clone — атомарная копия указателя                       |
// #================================================================================================================================================#
// # |                                       QACHON NIMA ISHLATISH                                                                                  |
// #================================================================================================================================================#
// # |  14 | Copy                     | Kichik, stack da — i32, f64, bool, struct      | Маленький, на стеке — i32, f64, bool, struct               |
// # |  15 | Clone                    | Katta, heap da — String, Vec, Box              | Большой, на куче — String, Vec, Box                        |
// # |  16 | Rc::clone                | Ko'p owner, bitta thread                       | Много владельцев, один поток                               |
// # |  17 | Arc::clone               | Ko'p owner, ko'p thread                        | Много владельцев, много потоков                            |
// #================================================================================================================================================#