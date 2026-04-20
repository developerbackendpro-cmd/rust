// #================================================================================================================================================#
// #                                                         VARIANCE (CO/CONTRA/INVARIANCE)                                                        #
// #                        VARIANCE — GENERIC TURLAR ORASIDAGI KICHIK/KATTA MUNOSABAT. LIFETIME VA GENERIC BILAN ISHLAYDI.                         #
// #                        VARIANCE — ОТНОШЕНИЕ МЕНЬШЕ/БОЛЬШЕ МЕЖДУ GENERIC ТИПАМИ. РАБОТАЕТ С LIFETIME И GENERIC.                                 #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::marker::PhantomData;


// Variance — agar A: B bo'lsa, F<A>: F<B> munosabati qanday?
// Variance — если A: B, какое соотношение F<A>: F<B>?
//
// Uch xil variance:
// Три вида variance:
//
//   COVARIANCE (birga o'zgarish):
//     'long: 'short → &'long T: &'short T ✅
//     Uzunroq lifetime qisqaroq lifetime o'rnida ishlatilishi mumkin
//
//   CONTRAVARIANCE (qarama-qarshi o'zgarish):
//     'long: 'short → fn('short T): fn('long T) ✅
//     Qisqa argument qabul qiluvchi fn uzun argument qabul qilishi mumkin
//
//   INVARIANCE (o'zgarmaslik):
//     'long: 'short → &mut 'long T: &mut 'short T ❌
//     Ikkalasi aynan bir xil bo'lishi kerak

// &'a T — covariant 'a bo'yicha
// &'a T — ковариантен по 'a
//
// 'long: 'short → &'long T qabul qilinadi &'short T kerak bo'lganda
// 'long: 'short → &'long T принимается там где нужен &'short T

fn covariance_misoli() {
    // 'long — uzunroq lifetime
    // 'long — более длинный lifetime
    let uzun_string: String = String::from("uzun yashaydi");

    // 'short — qisqaroq lifetime
    // 'short — более короткий lifetime
    let qisqa_ref: &str;
    {
        // uzun_string hali tirik — biz undan reference olamiz
        // uzun_string ещё жива — берём ссылку на неё
        let ref_uzun: &str = &uzun_string;  // 'long reference

        // &'long str → &'short str o'rnida ishlatish mumkin (covariance)
        // &'long str → можно использовать вместо &'short str (covariance)
        qisqa_ref = ref_uzun;
        println!("{}", qisqa_ref);
    }
    // uzun_string hali tirik, qisqa_ref ni ishlatishimiz mumkin
    println!("{}", qisqa_ref);
    // uzun yashaydi
    // uzun yashaydi

    // Vec<T> — T bo'yicha covariant
    // Vec<T> — ковариантен по T
    // Vec<&'long str> → Vec<&'short str> o'rnida ishlatish mumkin
    let s1: String = String::from("birinchi");
    let s2: String = String::from("ikkinchi");
    let uzun_vec: Vec<&str> = vec![&s1, &s2];

    // uzun_vec ni qisqaroq lifetime bilan ishlatamiz
    // используем uzun_vec с более коротким lifetime
    fn vec_chiqar(v: &[&str]) {
        for s in v { print!("{} ", s); }
        println!();
    }
    vec_chiqar(&uzun_vec);
    // birinchi ikkinchi
}

// fn(T) — T bo'yicha contravariant
// fn(T) — контравариантен по T
//
// Agar fn qisqaroq lifetime qabul qilsa — uzunroq ham qabul qila oladi
// Если fn принимает более короткий lifetime — может принять и более длинный

fn contravariance_misoli() {
    // Qisqa lifetime qabul qiluvchi funksiya
    // Функция принимающая короткий lifetime
    fn qisqa_qabul(s: &str) {
        println!("Qabul: {}", s);
    }

    // uzun string yaratamiz
    // создаём длинную строку
    let uzun: String = String::from("uzun string");

    // qisqa_qabul funksiyasi uzun lifetime ni ham qabul qila oladi
    // qisqa_qabul может принять и длинный lifetime
    qisqa_qabul(&uzun);
    // Qabul: uzun string

    // fn pointer contravariance misoli
    // пример контравариантности fn pointer
    // fn(&'static str) qabul qiluvchi joy → fn(&'short str) ham ishlatilishi mumkin
    let f: fn(&str) = |s| println!("fn: {}", s);
    f("literal");
    f(&uzun);
    // fn: literal
    // fn: uzun string
}

// &mut T — T bo'yicha invariant
// &mut T — инвариантен по T
//
// &mut 'long T ≠ &mut 'short T
// Ikkalasi AYNAN bir xil bo'lishi kerak!
// Они должны быть ТОЧНО одинаковы!

fn invariance_misoli() {
    // &mut T invariant — turlar aynan bir xil bo'lishi kerak
    // &mut T invariant — типы должны совпадать точно

    let mut s: String = String::from("salom");

    // Bu to'g'ri — bir xil tur
    // Это правильно — одинаковый тип
    let r: &mut String = &mut s;
    r.push_str(" dunyo");
    println!("{}", s);
    // salom dunyo

    // NIMA UCHUN &mut invariant?
    // ПОЧЕМУ &mut инвариантен?
    //
    // Agar &mut covariant bo'lsa:
    // Если бы &mut был ковариантен:
    //
    //   let mut s: &str = "qisqa";
    //   {
    //       let uzun = String::from("uzun");
    //       let r: &mut &str = &mut s;
    //       *r = &uzun;  // ← bu xavfli! uzun o'ladi, s dangling bo'ladi
    //   }
    //   println!("{}", s);  // ← memory unsafety!
    //
    // Rust bu holatni &mut invariant qilib oldini oladi!
    // Rust предотвращает это, делая &mut инвариантным!

    println!("Invariance - xavfsizlik ta'minlandi");
}

// PhantomData — variance ni belgilash uchun
// PhantomData — для указания variance

// Covariant T bo'yicha
// Ковариантен по T
struct Covariant<T> {
    _marker: PhantomData<T>,
}

// Contravariant T bo'yicha
// Контравариантен по T
struct Contravariant<T> {
    _marker: PhantomData<fn(T)>,
}

// Invariant T bo'yicha
// Инвариантен по T
struct Invariant<T> {
    _marker: PhantomData<fn(T) -> T>,
}

// Rust da built-in turlarning variance jadvali:
// Таблица variance встроенных типов в Rust:
//
//  Tur                | 'a bo'yicha    | T bo'yicha
//  ─────────────────────────────────────────────────
//  &'a T              | covariant      | covariant
//  &'a mut T          | covariant      | invariant
//  *const T           | —              | covariant
//  *mut T             | —              | invariant
//  Box<T>             | —              | covariant
//  Vec<T>             | —              | covariant
//  Cell<T>            | —              | invariant
//  RefCell<T>         | —              | invariant
//  Mutex<T>           | —              | invariant
//  fn(T) -> U         | —              | T: contra, U: co
//  Option<T>          | —              | covariant
//  Result<T, E>       | —              | T: co, E: co

fn variance_jadvali_misoli() {

    // &'a T — 'a va T bo'yicha COVARIANT
    // &'a T — КОВАРИАНТЕН по 'a и T
    let s: String = String::from("salom");
    let r: &str = &s;  // &String → &str (covariant T bo'yicha)
    println!("{}", r);
    // salom

    // Box<T> — T bo'yicha COVARIANT
    // Box<T> — КОВАРИАНТЕН по T
    let boxed_string: Box<String> = Box::new(String::from("salom"));
    // Box<String> → Box<str> bo'lmaydi (turlar farqli)
    // lekin &Box<String> → &String → &str bo'ladi (Deref orqali)
    let r2: &str = boxed_string.as_str();
    println!("{}", r2);
    // salom

    // fn(T) — T bo'yicha CONTRAVARIANT, qaytarish U bo'yicha COVARIANT
    // fn(T) — КОНТРАВАРИАНТЕН по T, КОВАРИАНТЕН по U (возвращаемое)
    let f: fn(&'static str) -> String = |s| s.to_string();
    // fn(&'static str) — faqat static qabul qiladi
    // lekin fn(&str) — istalgan lifetime qabul qiladi (contravariant)
    let natija: String = f("salom");
    println!("{}", natija);
    // salom
}

// 'long: 'short — 'long kamida 'short qadар yashaydi
// 'long: 'short — 'long живёт не меньше чем 'short
//
// Bu degani: 'long — 'short ning subtip (kichikroq qiymat, lekin ko'proq umr)
// То есть: 'long — подтип 'short (меньший тип, но дольше живёт)

fn lifetime_subtyping_misoli() {

    // 'long: 'short — 'long uzunroq yashaydi
    // 'long: 'short — 'long живёт дольше
    fn qabul_qil<'short>(r: &'short str) -> &'short str {
        r
    }

    let uzun: String = String::from("uzun umrli");
    let natija: &str;
    {
        let ref_uzun: &str = &uzun;  // 'long reference
        natija = qabul_qil(ref_uzun);  // 'long → 'short (covariance)
        println!("{}", natija);
    }
    // uzun umrli

    // Explicit lifetime bound: 'a: 'b
    // Явное ограничение lifetime: 'a: 'b
    fn uzun_dan_qisqa<'a: 'b, 'b>(uzun: &'a str, _qisqa: &'b str) -> &'b str {
        uzun  // 'a: 'b bo'lgani uchun 'a ni 'b o'rnida ishlatish mumkin
    }

    let s1: String = String::from("uzun");
    let natija2: &str;
    {
        let s2: String = String::from("qisqa");
        natija2 = uzun_dan_qisqa(&s1, &s2);
        println!("{}", natija2);
    }
    // uzun
}

// 1. Iterator — covariant
// 1. Iterator — ковариантен
struct StrIterator<'a> {
    elementlar: Vec<&'a str>,
    pozitsiya: usize,
}

impl<'a> StrIterator<'a> {
    fn new(elementlar: Vec<&'a str>) -> Self {
        StrIterator { elementlar, pozitsiya: 0 }
    }
}

impl<'a> Iterator for StrIterator<'a> {
    type Item = &'a str;  // 'a bo'yicha covariant

    fn next(&mut self) -> Option<&'a str> {
        if self.pozitsiya < self.elementlar.len() {
            let element = self.elementlar[self.pozitsiya];
            self.pozitsiya += 1;
            Some(element)
        } else {
            None
        }
    }
}

// 2. Callback handler — contravariant (fn(T))
// 2. Обработчик callback — контравариантен (fn(T))
struct EventHandler {
    // fn(&str) — &str bo'yicha contravariant
    // fn(&str) — контравариантен по &str
    handler: fn(&str),
}

impl EventHandler {
    fn new(handler: fn(&str)) -> Self {
        EventHandler { handler }
    }

    fn chaqirish(&self, xabar: &str) {
        (self.handler)(xabar);
    }
}

// 3. Xavfsiz Cell — invariant (ichki mutability)
// 3. Безопасная Cell — инвариантна (внутренняя мутабельность)
use std::cell::Cell;

struct XavfsizCounter {
    qiymat: Cell<i32>,  // Cell invariant — xavfsizlik ta'minlanadi
}

impl XavfsizCounter {
    fn new(qiymat: i32) -> Self {
        XavfsizCounter { qiymat: Cell::new(qiymat) }
    }

    fn oshirish(&self) {
        self.qiymat.set(self.qiymat.get() + 1);
    }

    fn qiymat(&self) -> i32 {
        self.qiymat.get()
    }
}

// 4. Generic container — covariant
// 4. Generic контейнер — ковариантен
struct ReadOnly<T> {
    ichki: T,
}

impl<T: fmt::Display> ReadOnly<T> {
    fn new(qiymat: T) -> Self {
        ReadOnly { ichki: qiymat }
    }

    fn o_qi(&self) -> &T {
        &self.ichki
    }
}

fn main() {

    println!("=== COVARIANCE ===");
    covariance_misoli();

    println!("=== CONTRAVARIANCE ===");
    contravariance_misoli();

    println!("=== INVARIANCE ===");
    invariance_misoli();

    println!("=== BUILT-IN VARIANCE ===");
    variance_jadvali_misoli();

    println!("=== LIFETIME SUBTYPING ===");
    lifetime_subtyping_misoli();

    println!("=== REAL HAYOT ===");

    // 1. StrIterator — covariant
    // 1. StrIterator — ковариантен
    let s1 = String::from("salom");
    let s2 = String::from("dunyo");
    let s3 = String::from("rust");
    let iter = StrIterator::new(vec![&s1, &s2, &s3]);
    for s in iter {
        print!("{} ", s);
    }
    println!();
    // salom dunyo rust

    // 2. EventHandler — contravariant fn(T)
    // 2. EventHandler — контравариантный fn(T)
    let handler = EventHandler::new(|xabar| {
        println!("Event: {}", xabar);
    });
    handler.chaqirish("tizim boshlandi");
    handler.chaqirish("foydalanuvchi kirdi");
    // Event: tizim boshlandi
    // Event: foydalanuvchi kirdi

    // 3. Cell — invariant
    // 3. Cell — инвариантна
    let counter = XavfsizCounter::new(0);
    counter.oshirish();
    counter.oshirish();
    counter.oshirish();
    println!("Counter: {}", counter.qiymat());
    // Counter: 3

    // 4. ReadOnly — covariant
    // 4. ReadOnly — ковариантен
    let son: ReadOnly<i32> = ReadOnly::new(42);
    let matn: ReadOnly<String> = ReadOnly::new(String::from("salom"));
    println!("{}", son.o_qi());
    println!("{}", matn.o_qi());
    // 42
    // salom

    // PhantomData — variance nazorat
    // PhantomData — управление variance
    let _cov: Covariant<String> = Covariant { _marker: PhantomData };
    let _con: Contravariant<String> = Contravariant { _marker: PhantomData };
    let _inv: Invariant<String> = Invariant { _marker: PhantomData };
    println!("PhantomData variance nazorat — OK");
    // PhantomData variance nazorat — OK
}
// #================================================================================================================================================#
// # |  №  | Tushuncha                | Tavsif (UZ)                                           | Описание (RU)                                       |
// #================================================================================================================================================#
// # |                                       VARIANCE TURLARI                                                                                       |
// #================================================================================================================================================#
// # |   1 | Covariance               | F<'long>: F<'short> — uzun o'rniga qisqa ishlatish    | F<'long>: F<'short> — длинный вместо короткого      |
// # |   2 | Contravariance           | F<'short>: F<'long> — qarama-qarshi yo'nalish         | F<'short>: F<'long> — обратное направление          |
// # |   3 | Invariance               | F<'a> faqat F<'a> — aynan bir xil bo'lishi kerak      | F<'a> только F<'a> — должны совпадать точно         |
// #================================================================================================================================================#
// # |                                       BUILT-IN TURLAR                                                                                        |
// #================================================================================================================================================#
// # |   4 | &'a T                    | 'a va T bo'yicha COVARIANT                            | КОВАРИАНТЕН по 'a и T                               |
// # |   5 | &'a mut T                | 'a COVARIANT, T INVARIANT                             | 'a КОВАРИАНТЕН, T ИНВАРИАНТЕН                       |
// # |   6 | Box<T>, Vec<T>           | T bo'yicha COVARIANT                                  | КОВАРИАНТЕН по T                                    |
// # |   7 | Cell<T>, RefCell<T>      | T bo'yicha INVARIANT                                  | ИНВАРИАНТЕН по T                                    |
// # |   8 | fn(T) -> U               | T CONTRAVARIANT, U COVARIANT                          | T КОНТРАВАРИАНТЕН, U КОВАРИАНТЕН                    |
// #================================================================================================================================================#
// # |                                       PHANTOMDATA                                                                                            |
// #================================================================================================================================================#
// # |   9 | PhantomData<T>           | T bo'yicha covariant                                  | Ковариантен по T                                    |
// # |  10 | PhantomData<fn(T)>       | T bo'yicha contravariant                              | Контравариантен по T                                |
// # |  11 | PhantomData<fn(T)->T>    | T bo'yicha invariant                                  | Инвариантен по T                                    |
// # |  12 | PhantomData<Cell<T>>     | T bo'yicha invariant (alternativa)                    | Инвариантен по T (альтернатива)                     |
// #================================================================================================================================================#
// # |                                       LIFETIME SUBTYPING                                                                                     |
// #================================================================================================================================================#
// # |  13 | 'long: 'short            | 'long kamida 'short qadар yashaydi                    | 'long живёт не меньше 'short                        |
// # |  14 | 'static: 'a              | 'static barcha lifetime dan uzunroq                   | 'static длиннее любого lifetime                     |
// # |  15 | fn<'a: 'b>               | Explicit lifetime subtype bound                       | Явное ограничение подтипа lifetime                  |
// #================================================================================================================================================#
// # |                                       NIMA UCHUN MUHIM                                                                                       |
// #================================================================================================================================================#
// # |  16 | Memory safety            | Invariance &mut xotirani himoya qiladi                | Инвариантность &mut защищает память                 |
// # |  17 | Borrow checker           | Variance qoidalari borrow checker asosi               | Правила variance — основа borrow checker            |
// # |  18 | PhantomData              | Unsafe kodda variance nazorat qilish                  | Управление variance в unsafe коде                   |
// # |  19 | Lifetime elision         | Covariance tufayli elision ishlaydi                   | Элизия работает благодаря ковариантности            |
// #================================================================================================================================================#