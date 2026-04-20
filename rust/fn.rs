// #================================================================================================================================================#
// #                                                                    FN() POINTERS                                                               #
// #                                FN POINTER — FUNKSIYAGA KO'RSATKICH. CLOSURE EMAS — FAQAT ODDIY FUNKSIYA. ZERO CAPTURE.                         #
// #                                FN POINTER — УКАЗАТЕЛЬ НА ФУНКЦИЮ. НЕ ЗАМЫКАНИЕ — ТОЛЬКО ОБЫЧНАЯ ФУНКЦИЯ. ZERO CAPTURE.                         #
// #================================================================================================================================================#

// #================================================================================================================================================#
// |                                                                                                                                                |
// |                            1. FN POINTER NIMA?                                                                                                 |
// |                               • fn pointer – bu funksiyaning xotira manzilini saqlovchi ko‘rsatkich turidir.                                   |
// |                               • Turi: `fn(param1, param2) -> QaytishTuri`.                                                                     |
// |                               • Closure emas – atrof‑muhitdan hech narsani capture qilmaydi (zero capture).                                    |
// |                               • fn pointer Fn, FnMut va FnOnce traitlarini avtomatik ravishda implement qiladi.                                |
// |                               • O‘lchami har doim 8 bayt (64‑bit tizimda) – oddiy pointer hajmi.                                               |
// |                                                                                                                                                |
// #================================================================================================================================================#
// |                                                                                                                                                |
// |               2. FN POINTER VS CLOSURE (ENG MUHIM FARQLAR)                                                                                     |
// |                                                                                                                                                |
// |               +---------------------------+----------------------------------+------------------------------------------------+                |
// |               | Xususiyat                 | fn pointer                       | Closure                                        |                |
// |               |---------------------------+----------------------------------+------------------------------------------------|                |
// |               | Capture                   | Yo‘q (zero capture)              | Ha, atrof‑muhitdan o‘zgaruvchilarni oladi      |                |
// |               | Turi                      | fn(T) -> U                       | Anonim tip (har bir closure alohida tip)       |                |
// |               | O‘lchami                  | 8 bayt (har doim)                | Capture qilingan ma’lumotga bog‘liq            |                |
// |               | Trait implementatsiyasi   | Fn, FnMut, FnOnce (avtomatik)    | Faqat keraklisini implement qiladi             |                |
// |               | Xotirada joylashuvi       | Kod segmentida                    | Stackda (yoki heapda Box bilan)               |                |
// |               | HRTB qo‘llab‑quvvatlashi  | Ha (for<'a> Fn(&'a str) -> &'a)  | Yo‘q (closure lifetime aniqlab bo‘lmaydi)      |                |
// |               | Qachon ishlatiladi        | Dispatch table, callback, C FFI   | Iterator adapterlari, qisqa amallar           |                |
// |               +---------------------------+----------------------------------+------------------------------------------------+                |
// |                                                                                                                                                |
// #================================================================================================================================================#
// |                                                                                                                                                |
// |               3. FN POINTER SINTAKSISI VA ASOSIY ISHLATILISHI                                                                                  |
// |                                                                                                                                                |
// |               +-------------------------------+--------------------------------------------------------------------------------+               |
// |               | Sintaksis                     | Tavsif                                                                         |               |
// |               |-------------------------------+--------------------------------------------------------------------------------|               |
// |               | `let f: fn(i32) -> i32 = func;` | fn pointer e’lon qilish va funksiyaga bog‘lash                               |               |
// |               | `let f = func;`               | Tur deduksiyasi bilan (kompilyator avtomatik aniqlaydi)                        |               |
// |               | `f(42)`                       | To‘g‘ridan to‘g‘ri chaqirish                                                   |               |
// |               | `(f)(42)`                     | Qavs ichida chaqirish                                                          |               |
// |               | `f.call((42,))`               | Fn trait orqali chaqirish (kam ishlatiladi)                                    |               |
// |               | `type Alias = fn(T) -> U;`    | Type alias yaratish (o‘qishni osonlashtiradi)                                  |               |
// |               +----------------------------------------------------------------------------------------------------------------+               |
// |                                                                                                                                                |
// #================================================================================================================================================#
// |                                                                                                                                                |
// |               4. FN POINTERNI ARGUMENT SIFATIDA QABUL QILISH                                                                                   |
// |                                                                                                                                                |
// |               +-------------------------------+--------------------------------------------------------------------------------+               |
// |               | Shakl                         | Tavsif                                                                         |               |
// |               +-------------------------------+--------------------------------------------------------------------------------+               |
// |               | `fn foo(f: fn(i32) -> i32)`   | To‘g‘ridan to‘g‘ri fn pointer tipi sifatida                                    |               |
// |               | `fn foo<F: Fn(i32) -> i32>(f: F)` | Trait bound bilan (closure ham qabul qiladi)                               |               |
// |               | `fn foo(f: &[fn(i32) -> i32])` | fn pointerlar massivi/slice qabul qilish                                      |               |
// |               +-------------------------------+--------------------------------------------------------------------------------+               |
// |                                                                                                                                                |
// #================================================================================================================================================#
// |                                                                                                                                                |
// |               5. FN POINTERNI QAYTARISH VA STRUKTURALAR ICHIDA ISHLATISH                                                                       |
// |                                                                                                                                                |
// |               +------------------------------------------+---------------------------------------------------------------------+               |
// |               | Shakl                                    | Tavsif                                                              |               |
// |               |-------------------------------+--------------------------------------------------------------------------------|               |
// |               | `fn tanla(c: char) -> fn(i32,i32)->i32`  | Funksiyadan fn pointer qaytarish (dispatch uchun)                   |               |
// |               | `struct S { f: fn(i32) -> i32 }`         | Struct maydoni sifatida fn pointer                                  |               |
// |               | `enum E { V(fn(i32) -> i32) }`           | Enum variantida fn pointer saqlash                                  |               |
// |               | `Vec<fn(i32) -> i32>`                    | Koleksiya ichida bir nechta fn pointer saqlash                      |               |
// |               +-------------------------------+--------------------------------------------------------------------------------+               |
// |                                                                                                                                                |
// #================================================================================================================================================#
// |                                                                                                                                                |
// |                6. FN POINTER VA HRTB (HIGHER‑RANKED TRAIT BOUNDS)                                                                              |
// |                                                                                                                                                |
// |                    • fn pointer `for<'a> Fn(&'a str) -> &'a str` traitini avtomatik implement qiladi.                                          |
// |                    • Bu degani, fn pointer istalgan lifetime bilan ishlay oladi (hatto closure qila olmaydigan holatlarda).                    |
// |                    • Misol: funksiyadan qaytgan referensning lifetime’ini aniqlash kerak bo‘lganda fn pointer ishlatiladi.                     |
// |                    • Closure’lar bunday umumiy HRTBni implement qila olmaydi, chunki ular konkret capture muhitiga bog‘langan.                 |
// |                                                                                                                                                |
// #================================================================================================================================================#
// |                                                                                                                                                |
// |                7. DISPATCH TABLE PATTERNI (FN POINTERLAR BILAN)                                                                                |
// |                                                                                                                                                |
// |                    Dispatch table – bu matnli kalit (yoki enum) bilan mos fn pointerni bog‘lovchi jadval.                                      |
// |                    Ko‘pincha quyidagilar uchun ishlatiladi:                                                                                    |
// |                    • Router (URL -> handler funksiya)                                                                                          |
// |                    • Kalkulyator (operator belgisi -> matematik amal)                                                                          |
// |                    • Holat mashinasi (state -> transition funksiya)                                                                            |
// |                                                                                                                                                |
// |                    Afzalliklari:                                                                                                               |
// |                    • match yoki if‑else zanjirlariga qaraganda tezroq va ixchamroq.                                                            |
// |                    • Dinamik ravishda yangi “buyruqlar” qo‘shish oson.                                                                         |
// |                                                                                                                                                |
// |                    Oddiy sxemasi:                                                                                                              |
// |                    `let table: Vec<(&str, fn(T) -> U)> = vec![("kalit1", func1), ("kalit2", func2)];`                                          |
// |                    `let natija = table.iter().find(|(k,_)| *k == kalit).map(|(_,f)| f(arg));`                                                  |
// |                                                                                                                                                |
// #================================================================================================================================================#
// |                                                                                                                                                |
// |                8. TEZ‑TEZ SO‘RALADIGAN SAVOLLAR VA JAVOBLAR                                                                         |
// |                                                                                                                                                |
// |                +------------------------------------------------------------------------------------------------------------------+            |
// |                |   Savol 1: fn pointer bilan closure o‘rtasidagi asosiy farq nima?                                                |            |
// |                |   Javob:   fn pointer atrof‑muhitdan hech narsani capture qilmaydi va har doim bir xil o‘lchamga ega.            |            |
// |                |            Closure esa o‘zgaruvchilarni capture qilishi mumkin va uning o‘lchami capture miqdoriga bog‘liq.      |            |
// |                |            fn pointer – oddiy funksiya manzili, closure – anonim turdagi ob’ekt.                                 |            |
// |                +------------------------------------------------------------------------------------------------------------------|            |
// |                |   Savol 2: Qachon fn pointer, qachon closure ishlatish kerak?                                                    |            |
// |                |   Javob:   Agar funksiya atrof‑muhitga muhtoj bo‘lmasa va takroran ishlatiladigan bo‘lsa – fn pointer.           |            |
// |                |            Agar qisqa, bir martalik va tashqi o‘zgaruvchilarni ishlatsa – closure.                               |            |
// |                |            Shuningdek, C kutubxonalari bilan ishlaganda faqat fn pointer ishlatiladi (C FFI).                    |            |
// |                +------------------------------------------------------------------------------------------------------------------|            |
// |                |   Savol 3: fn pointer Fn, FnMut, FnOnce traitlarini implement qila oladimi?                                      |            |
// |                |   Javob:   Ha, fn pointer uchala traitni ham avtomatik implement qiladi. Shuning uchun fn pointer                |            |
// |                |            `where F: Fn()` boundiga ega bo‘lgan istalgan joyga argument sifatida berilishi mumkin.               |            |
// |                +------------------------------------------------------------------------------------------------------------------|            |
// |                |   Savol 4: fn pointerdan closure sifatida foydalansa bo‘ladimi?                                                  |            |
// |                |   Javob:   Ha, chunki fn pointer Fn traitini implement qilgan. `Fn` qabul qiluvchi funksiyaga fn pointer         |            |
// |                |            bemalol uzatiladi. Lekin closure’ni fn pointerga aylantirib bo‘lmaydi (agar u capture qilsa).         |            |
// |                +------------------------------------------------------------------------------------------------------------------|            |
// |                |   Savol 5: fn pointer nima uchun closure’ga qaraganda HRTB (for<'a>) bilan yaxshi ishlaydi?                      |            |
// |                |   Javob:   fn pointer hech narsani capture qilmagani uchun uning tanasi hech qanday tashqi lifetime’ga           |            |
// |                |            bog‘liq emas. Closure esa capture qilgan o‘zgaruvchilarning lifetime’iga bog‘lanib qoladi,            |            |
// |                |            shuning uchun closure’lar ko‘pincha HRTB talab qiladigan joylarda ishlamaydi.                         |            |
// |                +------------------------------------------------------------------------------------------------------------------|            |
// |                |   Savol 6: Dispatch table nima va u Rust’da qanday yaratiladi?                                                   |            |
// |                |   Javob:   Dispatch table – bu kalit‑qiymat juftligi shaklida funksiya pointerlarini saqlovchi jadval.           |            |
// |                |            Rust’da u odatda `Vec<(&str, fn(...))>` yoki `HashMap<Key, fn(...)>` ko‘rinishida bo‘ladi.            |            |
// |                |            Router yoki kalkulyatorlar yaratishda juda foydali.                                                   |            |
// |                +------------------------------------------------------------------------------------------------------------------+            |
// |                                                                                                                                                |
// #================================================================================================================================================#
// |                                                                                                                                                |
// |                9. XULOSA – BILISH KERAK BO‘LGAN ENG MUHIM NUQTALAR                                                                             |
// |                                                                                                                                                |
// |                +-----------------------------------------------------------------------------------------------------------------+             |
// |                | 🔹 Turi: `fn(Param) -> Return`                                                                                  |             |
// |                | 🔹 Capture: Zero (atrof‑muhitdan mustaqil)                                                                      |             |
// |                | 🔹 Traitlar: Fn, FnMut, FnOnce avtomatik implement qilinadi                                                     |             |
// |                | 🔹 O‘lchami: Har doim usize (8 bayt 64‑bitda)                                                                   |             |
// |                | 🔹 Coercion: fn pointer closure kutilgan joyda ishlatilishi mumkin                                              |             |
// |                | 🔹 HRTB: Closure’lar ishlamaydigan ba’zi murakkab lifetime ssenariylarida fn pointer yordam beradi              |             |
// |                | 🔹 Dispatch Table: fn pointerlardan foydalanib tez va moslashuvchan marshrutizatsiya tizimlari yaratish mumkin  |             |
// |                +-----------------------------------------------------------------------------------------------------------------+             |
// |                                                                                                                                                |
// #================================================================================================================================================#


#![allow(dead_code, unused)]

use std::fmt;

// fn pointer — funksiyaning xotira manzili
// fn pointer — адрес функции в памяти
//
// Closure vs fn pointer:
//   closure — atrof-muhitdan capture qilishi mumkin
//   fn pointer — capture yo'q, faqat funksiya
//
// fn pointer Fn, FnMut, FnOnce ni implement qiladi
// fn pointer реализует Fn, FnMut, FnOnce
//
// Tur: fn(T1, T2) -> U
// Тип: fn(T1, T2) -> U

// oddiy funksiyalar
// обычные функции
fn qo_shish(a: i32, b: i32) -> i32 { a + b }
fn ayirish(a: i32, b: i32) -> i32 { a - b }
fn ko_paytirish(a: i32, b: i32) -> i32 { a * b }
fn bo_lish(a: i32, b: i32) -> i32 { if b != 0 { a / b } else { 0 } }

fn kvadrat(x: i32) -> i32 { x * x }
fn kub(x: i32) -> i32 { x * x * x }
fn abs_qiymat(x: i32) -> i32 { if x < 0 { -x } else { x } }

fn salom_chiqar(ism: &str) { println!("Salom, {}!", ism); }
fn xayir_chiqar(ism: &str) { println!("Xayr, {}!", ism); }

fn sintaksis_misollari() {

    // fn pointer — funksiyaga o'zlashtirish
    // fn pointer — присвоение функции
    let f: fn(i32, i32) -> i32 = qo_shish;
    let natija: i32 = f(10, 5);
    println!("{}", natija);
    // 15

    // fn pointer — turli funksiyalarga ko'rsatish
    // fn pointer — указание на разные функции
    let mut operatsiya: fn(i32, i32) -> i32 = qo_shish;
    println!("{}", operatsiya(10, 5));
    operatsiya = ayirish;
    println!("{}", operatsiya(10, 5));
    operatsiya = ko_paytirish;
    println!("{}", operatsiya(10, 5));
    // 15
    // 5
    // 50

    // fn pointer — tur deduksiyasi
    // fn pointer — вывод типа
    let f2 = qo_shish;  // tur: fn(i32, i32) -> i32
    println!("{}", f2(3, 4));
    // 7

    // fn pointer — Vec ichida
    // fn pointer — внутри Vec
    let funksiyalar: Vec<fn(i32) -> i32> = vec![kvadrat, kub, abs_qiymat];
    for f in &funksiyalar {
        print!("{} ", f(3));
    }
    println!();
    // 9 27 3

    // fn pointer — chaqirish usullari
    // fn pointer — способы вызова
    let g: fn(i32, i32) -> i32 = qo_shish;
    println!("{}", g(1, 2));       // to'g'ridan to'g'ri chaqirish ✅
    println!("{}", (g)(1, 2));     // qavs bilan ✅
    // 3
    // 3
    // 3
}

fn fn_vs_closure() {

    // fn pointer — faqat funksiya, capture yo'q
    // fn pointer — только функция, без захвата
    let f: fn(i32) -> i32 = kvadrat;
    println!("{}", f(5));
    // 25

    // closure — capture qiladi
    // closure — захватывает
    let n: i32 = 10;
    let cl = |x| x + n;  // n ni capture qildi
    println!("{}", cl(5));
    // 15

    // fn pointer — closure sifatida ishlatish mumkin
    // fn pointer — можно использовать как closure
    fn qabul_qil_fn<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
        f(x)
    }
    // fn pointer Fn trait ni implement qiladi
    // fn pointer реализует трейт Fn
    println!("{}", qabul_qil_fn(kvadrat, 4));
    println!("{}", qabul_qil_fn(|x| x * 3, 4));
    // 16
    // 12

    // fn pointer — o'lcham
    // fn pointer — размер
    println!("fn pointer: {} bayt", std::mem::size_of::<fn(i32) -> i32>());
    println!("i32:        {} bayt", std::mem::size_of::<i32>());
    // fn pointer: 8 bayt (pointer o'lchami)
    // i32:        4 bayt

    // closure — o'lcham capture ga bog'liq
    // closure — размер зависит от захвата
    let cl_zero = || 42;
    let n: i32 = 10;
    let cl_one = move || n + 1;
    let a: i32 = 1; let b: i32 = 2; let c: i32 = 3;
    let cl_three = move || a + b + c;
    println!("closure(0 capture): {} bayt", std::mem::size_of_val(&cl_zero));
    println!("closure(1 capture): {} bayt", std::mem::size_of_val(&cl_one));
    println!("closure(3 capture): {} bayt", std::mem::size_of_val(&cl_three));
    // closure(0 capture): 0 bayt
    // closure(1 capture): 4 bayt
    // closure(3 capture): 12 bayt
}

// fn pointer argument — aniq tur
// fn pointer аргумент — конкретный тип
fn fn_pointer_argument_misollari() {

    // fn pointer parametr
    // параметр fn pointer
    fn qo_lla(f: fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }
    println!("{}", qo_lla(kvadrat, 5));
    println!("{}", qo_lla(kub, 3));
    println!("{}", qo_lla(abs_qiymat, -7));
    // 25
    // 27
    // 7

    // ikki fn pointer parametr
    // два параметра fn pointer
    fn birlashtir(f: fn(i32) -> i32, g: fn(i32) -> i32, x: i32) -> i32 {
        f(g(x))
    }
    println!("{}", birlashtir(kvadrat, abs_qiymat, -3));
    // 9  (abs(-3)=3, 3^2=9)

    // fn pointer massiv bilan
    // fn pointer с массивом
    fn hammaga_qo_lla(funksiyalar: &[fn(i32) -> i32], x: i32) -> Vec<i32> {
        funksiyalar.iter().map(|f| f(x)).collect()
    }
    let natijalar: Vec<i32> = hammaga_qo_lla(&[kvadrat, kub, abs_qiymat], -4);
    println!("{:?}", natijalar);
    // [16, -64, 4]

    // fn pointer — &str parametrli
    // fn pointer — с параметром &str
    fn salomlash_tarzi(f: fn(&str), ism: &str) {
        f(ism);
    }
    salomlash_tarzi(salom_chiqar, "Dilshod");
    salomlash_tarzi(xayir_chiqar, "Dilshod");
    // Salom, Dilshod!
    // Xayr, Dilshod!
}

// fn pointer qaytarish
// возврат fn pointer
fn operatsiya_tanlash(tur: char) -> fn(i32, i32) -> i32 {
    match tur {
        '+' => qo_shish,
        '-' => ayirish,
        '*' => ko_paytirish,
        '/' => bo_lish,
        _   => qo_shish,
    }
}

fn transformatsiya_tanlash(tur: &str) -> fn(i32) -> i32 {
    match tur {
        "kvadrat" => kvadrat,
        "kub"     => kub,
        "abs"     => abs_qiymat,
        _         => |x| x,
    }
}

fn fn_pointer_qaytarish_misollari() {
    // fn pointer qaytaruvchi funksiya
    // функция возвращающая fn pointer
    let qo_sh = operatsiya_tanlash('+');
    let ayir = operatsiya_tanlash('-');
    let ko_payt = operatsiya_tanlash('*');
    println!("{}", qo_sh(10, 3));
    println!("{}", ayir(10, 3));
    println!("{}", ko_payt(10, 3));
    // 13
    // 7
    // 30

    // transformatsiya
    // трансформация
    let kv = transformatsiya_tanlash("kvadrat");
    let kb = transformatsiya_tanlash("kub");
    println!("{}", kv(5));
    println!("{}", kb(3));
    // 25
    // 27
}

// struct ichida fn pointer
// fn pointer внутри структуры
#[derive(Debug)]
struct Hisoblash {
    nomi: &'static str,
    funksiya: fn(i32, i32) -> i32,
}

impl Hisoblash {
    fn new(nomi: &'static str, funksiya: fn(i32, i32) -> i32) -> Self {
        Hisoblash { nomi, funksiya }
    }

    fn bajar(&self, a: i32, b: i32) -> i32 {
        (self.funksiya)(a, b)
    }
}

impl fmt::Display for Hisoblash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hisoblash({})", self.nomi)
    }
}

// Kalkulyator — fn pointer registri
// Калькулятор — реестр fn pointer
struct Kalkulyator {
    operatsiyalar: Vec<Hisoblash>,
}

impl Kalkulyator {
    fn new() -> Self {
        Kalkulyator { operatsiyalar: Vec::new() }
    }

    fn qo_sh_operatsiya(&mut self, h: Hisoblash) {
        self.operatsiyalar.push(h);
    }

    fn bajar(&self, nomi: &str, a: i32, b: i32) -> Option<i32> {
        self.operatsiyalar
            .iter()
            .find(|h| h.nomi == nomi)
            .map(|h| h.bajar(a, b))
    }
}

// Enum variant — fn pointer
// Enum вариант — fn pointer
#[derive(Debug, Clone, Copy)]
enum Transformatsiya {
    Kvadrat,
    Kub,
    Abs,
    Nollash,
}

impl Transformatsiya {
    fn funksiya(self) -> fn(i32) -> i32 {
        match self {
            Transformatsiya::Kvadrat => kvadrat,
            Transformatsiya::Kub    => kub,
            Transformatsiya::Abs    => abs_qiymat,
            Transformatsiya::Nollash => |_| 0,
        }
    }

    fn qo_lla(self, x: i32) -> i32 {
        (self.funksiya())(x)
    }
}

// fn pointer array — dispatch table
// массив fn pointer — таблица диспетчеризации
type HandlerFn = fn(&str) -> String;

fn bosh_sahifa(_: &str) -> String {
    String::from("<h1>Bosh sahifa</h1>")
}

fn haqida(_: &str) -> String {
    String::from("<h1>Biz haqimizda</h1>")
}

fn aloqa(_: &str) -> String {
    String::from("<h1>Aloqa</h1>")
}

fn topilmadi(yo_l: &str) -> String {
    format!("<h1>404: {} topilmadi</h1>", yo_l)
}

struct Router {
    marshrutlar: Vec<(&'static str, HandlerFn)>,
    not_found: HandlerFn,
}

impl Router {
    fn new() -> Self {
        Router {
            marshrutlar: Vec::new(),
            not_found: topilmadi,
        }
    }

    fn qo_sh(&mut self, yo_l: &'static str, handler: HandlerFn) {
        self.marshrutlar.push((yo_l, handler));
    }

    fn marshrutlash(&self, yo_l: &str) -> String {
        self.marshrutlar
            .iter()
            .find(|(r, _)| *r == yo_l)
            .map(|(_, h)| h(yo_l))
            .unwrap_or_else(|| (self.not_found)(yo_l))
    }
}

// fn pointer HRTB ni avtomatik implement qiladi
// fn pointer автоматически реализует HRTB
fn birinchi_soz_fn(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn oxirgi_soz_fn(s: &str) -> &str {
    s.split_whitespace().last().unwrap_or("")
}

fn trim_fn(s: &str) -> &str {
    s.trim()
}

// fn pointer for<'a> Fn(&'a str) -> &'a str implement qiladi
// fn pointer реализует for<'a> Fn(&'a str) -> &'a str
fn hrtb_qo_llash<F>(f: F, s: &str) -> &str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(s)
}

fn main() {

    sintaksis_misollari();

    fn_vs_closure();

    fn_pointer_argument_misollari();

    fn_pointer_qaytarish_misollari();

    // Hisoblash struct
    // Структура Hisoblash
    let qo_sh_h = Hisoblash::new("qo'shish", qo_shish);
    let ko_payt_h = Hisoblash::new("ko'paytirish", ko_paytirish);
    println!("{}: {}", qo_sh_h, qo_sh_h.bajar(10, 5));
    println!("{}: {}", ko_payt_h, ko_payt_h.bajar(10, 5));
    // Hisoblash(qo'shish): 15
    // Hisoblash(ko'paytirish): 50

    // Kalkulyator
    // Калькулятор
    let mut kalk = Kalkulyator::new();
    kalk.qo_sh_operatsiya(Hisoblash::new("qo'sh", qo_shish));
    kalk.qo_sh_operatsiya(Hisoblash::new("ayir", ayirish));
    kalk.qo_sh_operatsiya(Hisoblash::new("ko'payt", ko_paytirish));
    println!("{:?}", kalk.bajar("qo'sh", 10, 3));
    println!("{:?}", kalk.bajar("ayir", 10, 3));
    println!("{:?}", kalk.bajar("bo'l", 10, 3));
    // Some(13)
    // Some(7)
    // None

    // Transformatsiya enum
    // Enum Transformatsiya
    println!("{}", Transformatsiya::Kvadrat.qo_lla(5));
    println!("{}", Transformatsiya::Kub.qo_lla(3));
    println!("{}", Transformatsiya::Abs.qo_lla(-7));
    println!("{}", Transformatsiya::Nollash.qo_lla(999));
    // 25
    // 27
    // 7
    // 0

    // Vec bilan
    // С Vec
    let transformatsiyalar: Vec<Transformatsiya> = vec![
        Transformatsiya::Abs,
        Transformatsiya::Kvadrat,
        Transformatsiya::Kub,
    ];
    let natijalar: Vec<i32> = transformatsiyalar
        .iter()
        .map(|t| t.qo_lla(-3))
        .collect();
    println!("{:?}", natijalar);
    // [3, 9, -27]

    // Router — fn pointer dispatch table
    // Router — таблица диспетчеризации fn pointer
    let mut router = Router::new();
    router.qo_sh("/", bosh_sahifa);
    router.qo_sh("/haqida", haqida);
    router.qo_sh("/aloqa", aloqa);

    println!("{}", router.marshrutlash("/"));
    println!("{}", router.marshrutlash("/haqida"));
    println!("{}", router.marshrutlash("/aloqa"));
    println!("{}", router.marshrutlash("/mavjud-emas"));
    // <h1>Bosh sahifa</h1>
    // <h1>Biz haqimizda</h1>
    // <h1>Aloqa</h1>
    // <h1>404: /mavjud-emas topilmadi</h1>

    // fn pointer for<'a> Fn avtomatik implement qiladi
    // fn pointer автоматически реализует for<'a> Fn
    let gap: &str = "  salom dunyo rust  ";

    println!("{}", hrtb_qo_llash(trim_fn, gap));
    println!("{}", hrtb_qo_llash(birinchi_soz_fn, gap.trim()));
    println!("{}", hrtb_qo_llash(oxirgi_soz_fn, gap.trim()));
    // salom dunyo rust
    // salom
    // rust

    // fn pointer — turli lifetime bilan ishlaydi
    // fn pointer — работает с разными lifetime
    let s1: String = String::from("  birinchi string  ");
    let s2: &str = "  ikkinchi string  ";
    println!("{}", hrtb_qo_llash(trim_fn, &s1));
    println!("{}", hrtb_qo_llash(trim_fn, s2));
    // birinchi string
    // ikkinchi string

    // type alias — fn pointer uchun
    // type alias — для fn pointer
    type MathFn = fn(i32, i32) -> i32;
    type TransformFn = fn(i32) -> i32;

    let math_ops: Vec<(&str, MathFn)> = vec![
        ("qo'sh", qo_shish),
        ("ayir", ayirish),
        ("ko'payt", ko_paytirish),
    ];

    for (nomi, f) in &math_ops {
        println!("{}: {}", nomi, f(10, 3));
    }
    // qo'sh: 13
    // ayir: 7
    // ko'payt: 30

    let transform_ops: Vec<(&str, TransformFn)> = vec![
        ("kvadrat", kvadrat),
        ("kub", kub),
        ("abs", abs_qiymat),
    ];

    for (nomi, f) in &transform_ops {
        println!("{}: {}", nomi, f(-4));
    }
    // kvadrat: 16
    // kub: -64
    // abs: 4
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                       FN POINTER ASOSLARI                                                                                    |
// #================================================================================================================================================#
// # |   1 | fn(T) -> U               | Fn pointer turi                                      | Тип fn pointer                                       |
// # |   2 | let f: fn(T) -> U = func | Fn pointer o'zlashtirish                             | Присвоение fn pointer                                |
// # |   3 | f(args)                  | Fn pointer chaqirish                                 | Вызов fn pointer                                     |
// # |   4 | f.call((args,))          | Fn trait orqali chaqirish                            | Вызов через трейт Fn                                 |
// # |   5 | 8 bayt                   | Fn pointer o'lchami (pointer size)                   | Размер fn pointer (размер указателя)                 |
// #================================================================================================================================================#
// # |                                       FN POINTER VS CLOSURE                                                                                  |
// #================================================================================================================================================#
// # |   6 | Capture yo'q             | Fn pointer atrof-muhitdan hech narsa olmaydi         | fn pointer не захватывает окружение                  |
// # |   7 | Fn, FnMut, FnOnce        | Fn pointer uchala traitni implement qiladi           | fn pointer реализует все три трейта                  |
// # |   8 | fn pointer → closure     | Fn qabul qiluvchi joyga fn pointer berilishi mumkin  | fn pointer можно передать туда где нужен closure     |
// #================================================================================================================================================#
// # |                                       QOLLASH JOYLARI                                                                                        |
// #================================================================================================================================================#
// # |   9 | fn f(g: fn(T) -> U)      | Argument sifatida fn pointer                         | fn pointer как аргумент                              |
// # |  10 | fn f() -> fn(T) -> U     | Qaytarish qiymati sifatida fn pointer                | fn pointer как возвращаемое значение                 |
// # |  11 | struct S { f: fn(T)->U } | Struct fieldi sifatida fn pointer                    | fn pointer как поле структуры                        |
// # |  12 | Vec<fn(T) -> U>          | Koleksiya ichida fn pointer                          | fn pointer внутри коллекции                          |
// # |  13 | type Alias = fn(T) -> U  | Type alias bilan fn pointer                          | fn pointer с type alias                              |
// #================================================================================================================================================#
// # |                                       HRTB BILAN                                                                                             |
// #================================================================================================================================================#
// # |  14 | fn f(s: &str) -> &str    | fn pointer for<'a> Fn avtomatik                      | fn pointer автоматически for<'a> Fn                  |
// # |  15 | Closure vs fn pointer    | Closure HRTB qila olmaydi — fn pointer qiladi        | Closure не может HRTB — fn pointer может             |
// #================================================================================================================================================#
// # |                                       DISPATCH TABLE                                                                                         |
// #================================================================================================================================================#
// # |  16 | Vec<(&str, fn(...))>     | Nom bilan fn pointer juftligi                        | Пара имя + fn pointer                                |
// # |  17 | Router pattern           | Fn pointer bilan marshrutlash                        | Маршрутизация с fn pointer                           |
// # |  18 | match → fn pointer       | Enum dan fn pointer tanlash                          | Выбор fn pointer из enum                             |
// #================================================================================================================================================#