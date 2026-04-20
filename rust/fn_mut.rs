// #================================================================================================================================================#
// #                                                       FN  |  FNMUT  |  FNONCE                                                                  #
// #                FN TRAITLAR CHUQUR — CLOSURE CAPTURE, IERARXIYA, DYN FN, MOVE, CURRY, MEMOIZE, DISPATCH, COMPOSITION.                           #
// #                FN ТРЕЙТЫ ГЛУБОКО — ЗАХВАТ CLOSURE, ИЕРАРХИЯ, DYN FN, MOVE, CURRY, MEMOIZE, DISPATCH, COMPOSITION.                              #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::collections::HashMap;

// Fn Traitlar ierarxiyasi:
// Иерархия Fn трейтов:
//
//   FnOnce — eng keng: self consume, bir marta chaqiriladi
//   FnOnce — самый широкий: потребляет self, вызывается один раз
//   FnMut  — FnOnce ni implement qiladi: &mut self, ko'p marta
//   FnMut  — реализует FnOnce: &mut self, много раз
//   Fn     — FnMut ni implement qiladi: &self, ko'p marta
//   Fn     — реализует FnMut: &self, много раз
//
//   FnOnce ⊇ FnMut ⊇ Fn
//
// Capture turlari:
// Виды захвата:
//   move move  — T ownership: FnOnce (yoki Fn/FnMut agar T: Copy)
//   &mut borrow — mutable: FnMut
//   & borrow    — immutable: Fn
//
// Trait definition:
//   pub trait FnOnce<Args: Tuple> {
//       type Output;
//       fn call_once(self, args: Args) -> Self::Output;
//   }
//   pub trait FnMut<Args: Tuple>: FnOnce<Args> {
//       fn call_mut(&mut self, args: Args) -> Self::Output;
//   }
//   pub trait Fn<Args: Tuple>: FnMut<Args> {
//       fn call(&self, args: Args) -> Self::Output;
//   }

fn fn_misollari() {

    // Fn — immutable capture, ko'p marta
    // Fn — иммутабельный захват, много раз
    let n = 10;
    let qo_sh = |x| x + n; // n immutable borrow
    println!("{}", qo_sh(5));
    println!("{}", qo_sh(15));
    println!("{}", n); // n hali bor
    // 15
    // 25
    // 10

    // Fn — funksiya argumenti
    // Fn — аргумент функции
    fn ikki_marta_chaqir<F: Fn(i32) -> i32>(f: F, x: i32) -> (i32, i32) {
        (f(x), f(x + 1))
    }

    let kvadrat = |x: i32| x * x;
    println!("{:?}", ikki_marta_chaqir(kvadrat, 4));
    // (16, 25)

    // Fn — Box<dyn Fn>
    // Fn — Box<dyn Fn>
    let operatsiyalar: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x * x),
        Box::new(|x| x - 5),
    ];

    let boshlangich = 10;
    let natijalar: Vec<i32> = operatsiyalar.iter().map(|f| f(boshlangich)).collect();
    println!("{:?}", natijalar);
    // [11, 20, 100, 5]

    // Fn + Copy — bir xil closure ko'p o'zgaruvchida
    // Fn + Copy — одно closure в нескольких переменных
    let multiplier = 3;
    let ko_payt = move |x: i32| x * multiplier;
    let f1 = ko_payt;
    let f2 = ko_payt; // Copy — closure Copy bo'lsa mumkin (i32 capture)
    println!("{} {}", f1(5), f2(7));
    // 15 21

    // fn pointer — Fn implement qiladi
    // fn pointer — реализует Fn
    fn kvadrat_fn(x: i32) -> i32 { x * x }
    fn qo_sh_fn(x: i32) -> i32 { x + 10 }

    let fp: fn(i32) -> i32 = kvadrat_fn;
    println!("{}", fp(6));
    // 36

    // fn pointer Vec da
    // fn pointer в Vec
    let fnlar: Vec<fn(i32) -> i32> = vec![kvadrat_fn, qo_sh_fn];
    for f in &fnlar {
        print!("{} ", f(5));
    }
    println!();
    // 25 15
}

fn fnmut_misollari() {

    // FnMut — mutable capture
    // FnMut — мутабельный захват
    let mut hisob = 0;
    let mut oshirish = || {
        hisob += 1;
        hisob
    };
    println!("{}", oshirish());
    println!("{}", oshirish());
    println!("{}", oshirish());
    // 1
    // 2
    // 3

    // FnMut — funksiya argumenti
    // FnMut — аргумент функции
    fn uch_marta_chaqir<F: FnMut() -> i32>(mut f: F) -> Vec<i32> {
        vec![f(), f(), f()]
    }

    let mut n = 0;
    let v = uch_marta_chaqir(|| { n += 10; n });
    println!("{:?}", v);
    // [10, 20, 30]

    // FnMut — Iterator adapter pattern
    // FnMut — паттерн адаптера итератора
    let mut counter = 0;
    let v: Vec<i32> = (0..5).map(|x| {
        counter += 1;
        x * counter
    }).collect();
    println!("{:?}", v);
    // [0, 2, 6, 12, 20]  (0*1, 1*2, 2*3, 3*4, 4*5)

    // FnMut — log yozuvchi
    // FnMut — записыватель логов
    let mut log: Vec<String> = Vec::new();
    let mut yoz = |xabar: &str| {
        log.push(format!("[LOG] {}", xabar));
    };
    yoz("boshlandi");
    yoz("jarayon");
    yoz("tugadi");
    println!("{:?}", log);
    // ["[LOG] boshlandi", "[LOG] jarayon", "[LOG] tugadi"]

    // sort_by — FnMut
    // sort_by — FnMut
    let mut sozlar = vec!["banan", "olma", "anor", "nok", "uzum"];
    sozlar.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    println!("{:?}", sozlar);
    // ["nok", "anor", "olma", "uzum", "banan"]

    // FnMut — generator pattern
    // FnMut — паттерн генератора
    fn generator(mut boshlanish: i32, qadam: i32) -> impl FnMut() -> i32 {
        move || {
            let joriy = boshlanish;
            boshlanish += qadam;
            joriy
        }
    }

    let mut gen_iter = generator(0, 5);
    println!("{} {} {} {} {}", gen_iter(), gen_iter(), gen_iter(), gen_iter(), gen_iter());
    // 0 5 10 15 20
}

fn fnonce_misollari() {

    // FnOnce — owned qiymatni consume qiladi
    // FnOnce — потребляет owned значение
    let s = String::from("salom");
    let consume = move || {
        println!("{}", s);
        s // s qaytariladi — consume bo'ladi
    };
    let natija = consume();
    // consume(); // ← xato! ikkinchi marta chaqirib bo'lmaydi
    println!("Qaytarildi: {}", natija);
    // salom
    // Qaytarildi: salom

    // FnOnce — funksiya argumenti
    // FnOnce — аргумент функции
    fn bir_marta_chaqir<F: FnOnce() -> String>(f: F) -> String {
        f()
    }

    let ism = String::from("Dilshod");
    let natija2 = bir_marta_chaqir(move || ism);
    println!("{}", natija2);
    // Dilshod

    // FnOnce — thread spawn
    // FnOnce — запуск потока
    let ma_lumot = vec![1, 2, 3, 4, 5];
    let handle = std::thread::spawn(move || {
        let yig: i32 = ma_lumot.iter().sum();
        println!("Thread yig'indi: {}", yig);
        yig
    });
    let natija3 = handle.join().unwrap();
    println!("Asosiy: {}", natija3);
    // Thread yig'indi: 15
    // Asosiy: 15

    // FnOnce — Drop bilan
    // FnOnce — с Drop
    struct Resurs { nomi: String }
    impl Drop for Resurs {
        fn drop(&mut self) { println!("'{}' tushirildi", self.nomi); }
    }

    let r = Resurs { nomi: "muhim".to_string() };
    let ishlatuvchi = move || {
        println!("'{}' ishlatildi", r.nomi);
    };
    ishlatuvchi();
    // 'muhim' ishlatildi
    // 'muhim' tushirildi

    // Option::unwrap_or_else — FnOnce
    // Option::unwrap_or_else — FnOnce
    let opt: Option<String> = None;
    let fallback = String::from("standart");
    let natija4 = opt.unwrap_or_else(move || fallback);
    println!("{}", natija4);
    // standart
}

fn ierarxiya_misollari() {

    // Fn qabul qiluvchi — Fn, FnMut, FnOnce bera oladi
    // Принимающий Fn — может принять Fn, FnMut, FnOnce

    // Fn argument — faqat Fn qabul qiladi
    fn fn_argument<F: Fn() -> i32>(f: F) -> i32 { f() }

    // FnMut argument — FnMut va FnOnce qabul qiladi
    fn fnmut_argument<F: FnMut() -> i32>(mut f: F) -> i32 { f() }

    // FnOnce argument — hammani qabul qiladi
    fn fnonce_argument<F: FnOnce() -> i32>(f: F) -> i32 { f() }

    // Fn closure — hamma joyga berilishi mumkin
    let fn_closure = || 42;
    println!("{}", fn_argument(fn_closure));
    println!("{}", fnmut_argument(fn_closure));
    println!("{}", fnonce_argument(fn_closure));
    // 42
    // 42
    // 42

    // FnMut closure — FnMut va FnOnce ga berilishi mumkin
    let mut n = 0;
    let fnmut_closure = || { n += 1; n };
    // fn_argument(fnmut_closure);   // ← xato: FnMut Fn emas
    println!("{}", fnmut_argument(fnmut_closure));
    // 1

    // FnOnce closure — faqat FnOnce ga berilishi mumkin
    let s = String::from("bir marta");
    let fnonce_closure = move || { let _ = s; 99 };
    // fn_argument(fnonce_closure);    // ← xato
    // fnmut_argument(fnonce_closure); // ← xato
    println!("{}", fnonce_argument(fnonce_closure));
    // 99

    // Ierarxiya jadvali:
    // Таблица иерархии:
    println!("\nIerarxiya:");
    println!("Fn     → Fn ✅  FnMut ✅  FnOnce ✅");
    println!("FnMut  → Fn ❌  FnMut ✅  FnOnce ✅");
    println!("FnOnce → Fn ❌  FnMut ❌  FnOnce ✅");
}

fn move_closure_misollari() {

    // move — barcha captured qiymatlarni own qiladi
    // move — владеет всеми захваченными значениями
    let s = String::from("owned");
    let n = 42i32; // Copy — move bilan ham copy bo'ladi

    let f = move || {
        println!("{} {}", s, n);
    };
    // println!("{}", s); // ← xato: s moved
    println!("{}", n); // ← OK: n Copy bo'lgani uchun hali bor
    f();
    f(); // move bo'lsa ham Fn bo'lishi mumkin (agar consume bo'lmasa)
    // 42
    // owned 42
    // owned 42

    // move — thread uchun SHART
    // move — ОБЯЗАТЕЛЬНО для потока
    let data = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        println!("{:?}", data);
    });
    handle.join().unwrap();
    // [1, 2, 3]

    // move + Copy tur — original saqlanadi
    // move + Copy тип — оригинал сохраняется
    let x = 10i32;
    let f2 = move || x * 2; // x copy bo'ldi
    println!("{}", x); // hali bor
    println!("{}", f2());
    // 10
    // 20

    // move + non-Copy — original yo'qoladi
    // move + non-Copy — оригинал исчезает
    let v = vec![1, 2, 3];
    let f3 = move || v.len(); // v moved
    // println!("{:?}", v); // ← xato: v moved
    println!("{}", f3());
    // 3
}

// impl Fn — static dispatch, zero cost
// impl Fn — статическая диспетчеризация, без затрат
fn qo_shuvchi_yasash(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

fn ko_paytuvchi_yasash(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}

fn hisoblagich_yasash(boshlanish: i32) -> impl FnMut() -> i32 {
    let mut n = boshlanish;
    move || { let v = n; n += 1; v }
}

// Box<dyn Fn> — dynamic dispatch, heap, turli turlar
// Box<dyn Fn> — динамическая диспетчеризация, куча, разные типы
fn operatsiya_tanlash(tur: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match tur {
        "qo'sh"   => Box::new(|a, b| a + b),
        "ayir"    => Box::new(|a, b| a - b),
        "ko'payt" => Box::new(|a, b| a * b),
        "bo'l"    => Box::new(|a, b| if b != 0 { a / b } else { 0 }),
        _         => Box::new(|a, _| a),
    }
}

fn closure_qaytarish_misollari() {

    // impl Fn qaytarish
    // Возврат impl Fn
    let beshga = qo_shuvchi_yasash(5);
    let uchga = ko_paytuvchi_yasash(3);
    println!("{}", beshga(10));
    println!("{}", uchga(7));
    // 15
    // 21

    // impl FnMut qaytarish
    // Возврат impl FnMut
    let mut hisoblagich = hisoblagich_yasash(0);
    println!("{} {} {} {}", hisoblagich(), hisoblagich(), hisoblagich(), hisoblagich());
    // 0 1 2 3

    // Box<dyn Fn> — runtime da tanlash
    // Box<dyn Fn> — выбор в runtime
    let operatsiyalar = ["qo'sh", "ayir", "ko'payt", "bo'l"];
    for op in &operatsiyalar {
        let f = operatsiya_tanlash(op);
        println!("{}: {}", op, f(10, 3));
    }
    // qo'sh: 13
    // ayir: 7
    // ko'payt: 30
    // bo'l: 3

    // Zanjir — closure composition
    // Цепочка — composition замыканий
    let pipeline: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x - 3),
    ];

    let natija = pipeline.iter().fold(5, |acc, f| f(acc));
    println!("{}", natija);
    // ((5+1)*2)-3 = 9
}

// 1. Currying — ko'p argumentli closure dan bitta argumentlilar
// 1. Каррирование — из многоаргументного в одноаргументные
fn curry_i32<F>(f: F) -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32>
where
    F: Fn(i32, i32) -> i32 + Copy + 'static,
{
    move |a| Box::new(move |b| f(a, b))
}

fn curry_str<F>(f: F) -> impl Fn(&'static str) -> Box<dyn Fn(&'static str) -> String>
where
    F: Fn(&'static str, &'static str) -> String + Copy + 'static,
{
    move |a| Box::new(move |b| f(a, b))
}

// 2. Memoization — natijalarni keshlashtirish
// 2. Мемоизация — кэширование результатов
struct Memo<A, B>
where
    A: std::hash::Hash + Eq + Clone,
    B: Clone,
{
    funksiya: Box<dyn Fn(A) -> B>,
    kesh: HashMap<A, B>,
}

impl<A, B> Memo<A, B>
where
    A: std::hash::Hash + Eq + Clone,
    B: Clone,
{
    fn new<F: Fn(A) -> B + 'static>(f: F) -> Self {
        Memo { funksiya: Box::new(f), kesh: HashMap::new() }
    }

    fn chaqir(&mut self, arg: A) -> B {
        if let Some(v) = self.kesh.get(&arg) {
            return v.clone();
        }
        let natija = (self.funksiya)(arg.clone());
        self.kesh.insert(arg, natija.clone());
        natija
    }
}

// 3. Function composition — f ∘ g
// 3. Композиция функций — f ∘ g
fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

// 4. Middleware zanjiri
// 4. Цепочка Middleware
type Handler = Box<dyn Fn(&str) -> String>;

fn middleware_qo_sh(handler: Handler, middleware: impl Fn(String) -> String + 'static) -> Handler {
    Box::new(move |req| middleware(handler(req)))
}

fn advanced_pattern_misollari() {

    // Currying
    // Каррирование
    let curried_qo_sh = curry_i32(|a: i32, b: i32| a + b);
    let beshga_qo_sh = curried_qo_sh(5);
    println!("{}", beshga_qo_sh(3));
    println!("{}", beshga_qo_sh(10));
    // 8
    // 15

    let curried_format = curry_str(|prefix: &str, msg: &str| format!("{}: {}", prefix, msg));
    let xato_xabari = curried_format("XATO");
    println!("{}", xato_xabari("fayl topilmadi"));
    println!("{}", xato_xabari("ulanish uzildi"));
    // XATO: fayl topilmadi
    // XATO: ulanish uzildi

    // Memoization
    // Мемоизация
    let mut memo = Memo::new(|n: u64| {
        println!("Hisoblanyapti: {}", n);
        (1..=n).product::<u64>()
    });
    println!("{}", memo.chaqir(5));
    println!("{}", memo.chaqir(5)); // keshdan — hisob yo'q
    println!("{}", memo.chaqir(6));
    // Hisoblanyapti: 5
    // 120
    // 120
    // Hisoblanyapti: 6
    // 720

    // Function composition
    // Композиция функций
    let trim_fn = |s: String| s.trim().to_string();
    let katta = |s: String| s.to_uppercase();
    let undov = |s: String| format!("{}!", s);

    let pipeline = compose(compose(trim_fn, katta), undov);
    println!("{}", pipeline("  salom dunyo  ".to_string()));
    // SALOM DUNYO!

    // Middleware
    // Middleware
    let asosiy: Handler = Box::new(|req| format!("Javob: {}", req));
    let log_mw = |resp: String| { println!("[LOG] {}", resp); resp };
    let katta_mw = |resp: String| resp.to_uppercase();

    let handler = middleware_qo_sh(asosiy, log_mw);
    let handler = middleware_qo_sh(handler, katta_mw);
    println!("{}", handler("GET /api"));
    // [LOG] Javob: GET /api
    // JAVOB: GET /API
}

// Event system — Box<dyn Fn>
// Система событий — Box<dyn Fn>
struct EventBus<T: Clone> {
    tinglovchilar: Vec<Box<dyn Fn(&T)>>,
}

impl<T: Clone> EventBus<T> {
    fn new() -> Self { EventBus { tinglovchilar: Vec::new() } }

    fn obuna_bo_l(&mut self, f: impl Fn(&T) + 'static) {
        self.tinglovchilar.push(Box::new(f));
    }

    fn yuborish(&self, voqea: &T) {
        for tinglovchi in &self.tinglovchilar {
            tinglovchi(voqea);
        }
    }
}

// Retry mexanizmi
// Механизм повтора
fn retry<T, E, F>(mut f: F, urinishlar: u32, kechiktirish_ms: u64) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut oxirgi_xato = None;
    for urinish in 0..urinishlar {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) => {
                println!("Urinish {} muvaffaqiyatsiz", urinish + 1);
                oxirgi_xato = Some(e);
                if urinish + 1 < urinishlar {
                    std::thread::sleep(std::time::Duration::from_millis(kechiktirish_ms));
                }
            }
        }
    }
    Err(oxirgi_xato.unwrap())
}

fn real_hayot_misollari() {

    // EventBus
    // EventBus
    let mut bus: EventBus<String> = EventBus::new();
    let mut log: Vec<String> = Vec::new();

    bus.obuna_bo_l(|voqea| println!("[Konsol] {}", voqea));

    // Ikkinchi tinglovchi
    // Второй слушатель
    bus.obuna_bo_l(|voqea| {
        if voqea.contains("XATO") {
            println!("[Alert] Xato aniqlandi: {}", voqea);
        }
    });

    bus.yuborish(&String::from("Foydalanuvchi kirdi"));
    bus.yuborish(&String::from("XATO: fayl topilmadi"));
    bus.yuborish(&String::from("Operatsiya tugadi"));
    // [Konsol] Foydalanuvchi kirdi
    // [Konsol] XATO: fayl topilmadi
    // [Alert] Xato aniqlandi: XATO: fayl topilmadi
    // [Konsol] Operatsiya tugadi

    // Retry
    // Retry
    let mut urinish_soni = 0;
    let natija: Result<String, &str> = retry(|| {
        urinish_soni += 1;
        if urinish_soni < 3 {
            Err("vaqtinchalik xato")
        } else {
            Ok(format!("Muvaffaqiyat: {} urinishda", urinish_soni))
        }
    }, 5, 0);

    println!("{:?}", natija);
    // Urinish 1 muvaffaqiyatsiz
    // Urinish 2 muvaffaqiyatsiz
    // Ok("Muvaffaqiyat: 3 urinishda")

    // FnMut bilan sorting custom comparator
    // Кастомный компаратор сортировки с FnMut
    let mut mahsulotlar = vec![
        ("Olma", 1500.0, 4.5),
        ("Banan", 3000.0, 4.8),
        ("Anor", 2500.0, 4.5),
        ("Uzum", 4000.0, 4.2),
    ];

    // Avval reyting (kamayib), keyin narx (o'sib)
    // Сначала рейтинг (убывающий), потом цена (возрастающая)
    mahsulotlar.sort_by(|a, b| {
        b.2.partial_cmp(&a.2).unwrap()
            .then(a.1.partial_cmp(&b.1).unwrap())
    });

    for (nom, narx, reyting) in &mahsulotlar {
        println!("{:<10} {:<8.0} {:.1}", nom, narx, reyting);
    }
    // Banan      3000     4.8
    // Anor       2500     4.5
    // Olma       1500     4.5
    // Uzum       4000     4.2
}

fn main() {

    println!("=== FN ===");
    fn_misollari();

    println!("\n=== FNMUT ===");
    fnmut_misollari();

    println!("\n=== FNONCE ===");
    fnonce_misollari();

    println!("\n=== IERARXIYA ===");
    ierarxiya_misollari();

    println!("\n=== MOVE CLOSURE ===");
    move_closure_misollari();

    println!("\n=== CLOSURE QAYTARISH ===");
    closure_qaytarish_misollari();

    println!("\n=== ADVANCED PATTERNLAR ===");
    advanced_pattern_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                               | Описание (RU)                               |
// #================================================================================================================================================#
// # |                                        FN TRAITLAR                                                                                            |
// #================================================================================================================================================#
// # |   1 | Fn(&self)                       | Immutable capture, ko'p marta             | Иммутабельный захват, много раз                          |
// # |   2 | FnMut(&mut self)                | Mutable capture, ko'p marta               | Мутабельный захват, много раз                            |
// # |   3 | FnOnce(self)                    | Ownership, bir marta                      | Владение, один раз                                       |
// # |   4 | FnOnce ⊇ FnMut ⊇ Fn             | Ierarxiya — Fn eng tor                    | Иерархия — Fn самый узкий                                |
// # |   5 | move ||                         | Barcha captured qiymatlarni own qilish    | Владение всеми захваченными значениями                   |
// #================================================================================================================================================#
// # |                                        CAPTURE TURLARI                                                                                       |
// #================================================================================================================================================#
// # |   6 | &T capture                      | Fn — immutable borrow                     | Fn — иммутабельное заимствование                         |
// # |   7 | &mut T capture                  | FnMut — mutable borrow                    | FnMut — мутабельное заимствование                        |
// # |   8 | T capture (move)                | FnOnce — ownership                        | FnOnce — владение                                        |
// # |   9 | T: Copy + move                  | Copy bo'lsa — Fn bo'lishi mumkin          | Если Copy — может быть Fn                                |
// #================================================================================================================================================#
// # |                                        QAYTARISH                                                                                             |
// #================================================================================================================================================#
// # |  10 | -> impl Fn(T) -> U              | Static dispatch, zero cost                | Статическая диспетчеризация                              |
// # |  11 | -> Box<dyn Fn(T) -> U>          | Dynamic dispatch, heap, turli turlar      | Динамическая диспетчеризация, разные типы                |
// # |  12 | -> impl FnMut() -> T            | State bilan closure qaytarish             | Возврат closure с состоянием                             |
// #================================================================================================================================================#
// # |                                        ADVANCED PATTERNLAR                                                                                   |
// #================================================================================================================================================#
// # |  13 | Curry pattern                   | Ko'p arg → ketma-ket bitta arg            | Много арг → последовательно один арг                     |
// # |  14 | Memoization                     | Natijalarni keshlashtirish                | Кэширование результатов                                  |
// # |  15 | Compose (f ∘ g)                 | Funksiyalar zanjiri                       | Цепочка функций                                          |
// # |  16 | Middleware pattern              | Handler + dekoratsiya                     | Handler + декорирование                                  |
// # |  17 | fn pointer: fn(T) -> U          | Fn implement qiladi, zero capture         | Реализует Fn, нет захвата                                |
// # |  18 | EventBus<Box<dyn Fn>>           | Ko'p tinglovchi, dinamik dispatch         | Несколько слушателей, динамический dispatch              |
// #================================================================================================================================================#