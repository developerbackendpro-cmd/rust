// #================================================================================================================================================#
// #                                                           ADV LIFETIMES                                                                        #
// #                YUQORI DARAJALI LIFETIMELAR — NAMED, STATIC, ELISION, STRUCT, VARIANCE, HRTB, NLL, SELF-REFERENTIAL.                            #
// #                ПРОДВИНУТЫЕ LIFETIME — NAMED, STATIC, ELISION, STRUCT, VARIANCE, HRTB, NLL, SELF-REFERENTIAL.                                   #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::marker::PhantomData;

// Lifetime nima:
// Что такое lifetime:
//
//   - Reference qancha vaqt yaroqli ekanini bildiruvchi belgi
//   - Метка указывающая как долго действительна ссылка
//   - Kompilyator lifetimelarni tekshiradi — runtime xarajat yo'q
//   - Компилятор проверяет lifetimes — нет затрат в runtime
//   - Lifetime elision — ko'p holda yozmaslik mumkin
//   - Lifetime elision — в большинстве случаев можно не писать
//
// Elision qoidalari (3 ta):
// Правила Elision (3 штуки):
//   1. Har bir input reference — o'z lifetimeni oladi
//      Каждая входная ссылка — получает свой lifetime
//   2. Bitta input reference bo'lsa — output shu lifetime oladi
//      Если одна входная ссылка — output получает её lifetime
//   3. &self / &mut self bo'lsa — output self lifetime oladi
//      Если &self / &mut self — output получает lifetime self

fn uzun_satr<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn birinchi<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

fn ikki_input<'a>(x: &'a str, _y: &str) -> &'a str { x }

fn trim(s: &str) -> &str { s.trim() }

fn asosiy_lifetime_misollari() {

    let s1 = String::from("uzun satr");
    let s2 = String::from("qisqa");
    println!("{}", uzun_satr(&s1, &s2));
    // uzun satr

    let x: &str = "salom";
    let y: &str = "dunyo rust";
    println!("{}", uzun_satr(x, y));
    // dunyo rust

    println!("{}", birinchi("birinchi", "ikkinchi"));
    // birinchi

    println!("{}", trim("  salom  "));
    // salom
}

#[derive(Debug)]
struct Tahlil<'a> {
    matn: &'a str,
    boshlanish: usize,
    uzunlik: usize,
}

impl<'a> Tahlil<'a> {
    fn new(matn: &'a str, boshlanish: usize, uzunlik: usize) -> Self {
        Tahlil { matn, boshlanish, uzunlik }
    }

    fn qism(&self) -> &str {
        &self.matn[self.boshlanish..self.boshlanish + self.uzunlik]
    }
}

impl<'a> fmt::Display for Tahlil<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tahlil('{}' dan {}+{})", self.matn, self.boshlanish, self.uzunlik)
    }
}

#[derive(Debug)]
struct Juftlik<'a, 'b> {
    birinchi: &'a str,
    ikkinchi: &'b str,
}

impl<'a, 'b> Juftlik<'a, 'b> {
    fn new(birinchi: &'a str, ikkinchi: &'b str) -> Self {
        Juftlik { birinchi, ikkinchi }
    }

    fn eng_uzun(&self) -> &str {
        if self.birinchi.len() >= self.ikkinchi.len() { self.birinchi }
        else { self.ikkinchi }
    }
}

fn struct_lifetime_misollari() {

    let matn = String::from("salom dunyo rust dasturlash tili");
    let tahlil = Tahlil::new(&matn, 6, 5);
    println!("{}", tahlil);
    println!("Qism: '{}'", tahlil.qism());
    // Tahlil('salom dunyo rust dasturlash tili' dan 6+5)
    // Qism: 'dunyo'

    let a = String::from("uzunroq satr");
    let b = String::from("qisqa");
    let j = Juftlik::new(&a, &b);
    println!("Eng uzun: '{}'", j.eng_uzun());
    // Eng uzun: 'uzunroq satr'
}

fn static_misol() {

    // String literal — har doim 'static
    // Строковый литерал — всегда 'static
    let s: &'static str = "Bu satr dastur tugaguncha yashaydi";
    println!("{}", s);
    // Bu satr dastur tugaguncha yashaydi

    // T: 'static bound
    // T: 'static ограничение
    fn static_bound<T: 'static + fmt::Debug>(val: T) -> T {
        println!("{:?}", val);
        val
    }
    static_bound(42i32);
    static_bound(String::from("salom"));
    // 42
    // "salom"

    // Box::leak — heap ma'lumotini 'static ga aylantirish
    // Box::leak — превращение данных кучи в 'static
    let s = String::from("heap da yaratilgan");
    let leaked: &'static str = Box::leak(s.into_boxed_str());
    println!("{}", leaked);
    // heap da yaratilgan
    // OGOHLANTIRISH: Bu memory leak — faqat kerak bo'lganda ishlatiladi
    // ПРЕДУПРЕЖДЕНИЕ: Это утечка памяти — используйте только при необходимости

    // 'static — thread da ishlatish uchun
    // 'static — для использования в потоке
    let static_str: &'static str = "thread ga o'tkaziladi";
    let handle = std::thread::spawn(move || {
        println!("{}", static_str);
    });
    handle.join().unwrap();
    // thread ga o'tkaziladi
}

// 'a: 'b — 'a kamida 'b qadar yashaydi
// 'a: 'b — 'a живёт не меньше 'b
fn uzunroq<'a: 'b, 'b>(x: &'a str, _y: &'b str) -> &'b str { x }

// T: 'a — T 'a dan uzoq yashaydi
// T: 'a — T живёт дольше 'a
fn reference_qaytarish<'a, T: fmt::Display>(val: &'a T) -> &'a T {
    println!("{}", val);
    val
}

struct Wrapper<'a, T: 'a> {
    ichki: &'a T,
}

impl<'a, T: fmt::Display> Wrapper<'a, T> {
    fn new(ichki: &'a T) -> Self { Wrapper { ichki } }
    fn chiqar(&self) { println!("{}", self.ichki); }
}

fn lifetime_bound_misollari() {

    let s1 = String::from("uzun lifetime");
    let s2 = String::from("qisqa");
    let natija = uzunroq(&s1, &s2);
    println!("{}", natija);
    // uzun lifetime

    let n = 42i32;
    let r = reference_qaytarish(&n);
    println!("{}", r);
    // 42
    // 42

    let qiymat = 100i32;
    let w = Wrapper::new(&qiymat);
    w.chiqar();
    // 100
}

fn bir_input(x: &str) -> &str { x }
// ≡ fn bir_input<'a>(x: &'a str) -> &'a str

fn bitta_input_dan(x: &str) -> &str { x }
// ≡ fn bitta_input_dan<'a>(x: &'a str) -> &'a str

struct Muharrir { matn: String }

impl Muharrir {
    fn qism_olish(&self, boshlanish: usize, uzunlik: usize) -> &str {
        &self.matn[boshlanish..boshlanish + uzunlik]
        // ≡ fn qism_olish<'a>(&'a self, ...) -> &'a str
    }
}

fn elision_misollari() {

    println!("{}", bir_input("salom"));
    // salom

    let m = Muharrir { matn: String::from("salom dunyo") };
    println!("{}", m.qism_olish(0, 5));
    println!("{}", m.qism_olish(6, 5));
    // salom
    // dunyo
}

// NLL — Rust 2018+ dan borrow scope blok oxirigacha emas,
//        oxirgi ishlatilgan joyda tugaydi
// NLL — с Rust 2018+ borrow заканчивается не в конце блока,
//        а в последнем месте использования

fn nll_misollari() {

    // NLL bilan — bu ishlaydi (eski Rust da ishlamas edi)
    // С NLL — это работает (в старом Rust не работало)
    let mut v = vec![1, 2, 3, 4, 5];
    let r = &v[0]; // borrow boshlandi
    println!("{}", r); // r oxirgi marta ishlatildi — borrow tugaydi
    v.push(6);         // OK — r endi aktiv emas
    println!("{:?}", v);
    // 1
    // [1, 2, 3, 4, 5, 6]

    // Yana bir NLL misoli — if/else da
    // Ещё один пример NLL — в if/else
    let mut data = String::from("salom");
    let len = data.len(); // &data — borrow
    // borrow tugadi (len ishlatildi)
    data.push_str(" dunyo"); // OK — mutable borrow mumkin
    println!("{} (uzunlik={})", data, len);
    // salom dunyo (uzunlik=5)

    // Murakkab NLL — loop da
    // Сложный NLL — в цикле
    let mut sozlar = vec!["bir", "ikki", "uch"];
    for i in 0..sozlar.len() {
        let s = sozlar[i]; // borrow
        println!("{}", s); // borrow tugaydi
        if i == 0 {
            sozlar.push("to'rt"); // OK — avvalgi borrow tugagan
        }
    }
    // bir
    // ikki
    // uch
    // to'rt
}

// Variance — subtyping munosabati
// Variance — отношение подтипирования
//
//   Covariant (kovariant) — 'uzun → 'qisqa o'rnida ishlatilishi mumkin
//   Covariant — 'long может быть использован вместо 'short
//   &'a T      — 'a va T bo'yicha covariant
//   &'a T      — covariant по 'a и T
//
//   Contravariant (kontravariant) — aksincha
//   Contravariant — наоборот
//   fn(T) → T bo'yicha contravariant (argument pozitsiyasi)
//   fn(T) → contravariant по T (позиция аргумента)
//
//   Invariant (invariant) — na katta, na kichik
//   Invariant — ни больше, ни меньше
//   &'a mut T  — T bo'yicha invariant (MUHIM!)
//   &'a mut T  — invariant по T (ВАЖНО!)

fn covariance_misoli() {

    // Covariance — uzun lifetime qisqa o'rnida ishlatilishi mumkin
    // Covariance — длинный lifetime можно использовать вместо короткого
    fn qisqa_lifetime_qabul<'a>(x: &'a str) -> &'a str { x }

    let uzun_string = String::from("uzun yashaydi");
    let natija: &str;
    {
        // uzun_string qisqa_string dan uzunroq yashaydi
        // uzun_string живёт дольше чем qisqa_string
        let natija_ref = qisqa_lifetime_qabul(&uzun_string);
        // 'uzun 'qisqa o'rnida ishlatildi — covariant
        // 'uzun использован вместо 'qisqa — covariant
        println!("Covariant: {}", natija_ref);
    }
    println!("Hali bor: {}", uzun_string);
    // Covariant: uzun yashaydi
    // Hali bor: uzun yashaydi
}

fn invariance_misoli() {

    // Invariance — &mut T T bo'yicha invariant
    // Invariance — &mut T invariant по T
    // Sababi: agar &mut T covariant bo'lsa, xotira xavfsizligi buziladi
    // Причина: если &mut T был бы covariant, нарушилась бы безопасность памяти

    let mut uzun = String::from("uzun");
    let mut qisqa = String::from("q");

    // Bu ishlaydi — bir xil tur
    // Это работает — одинаковый тип
    let r1: &mut String = &mut uzun;
    *r1 = String::from("yangi qiymat");
    println!("{}", uzun);
    // yangi qiymat

    // &mut T invariant bo'lgani uchun — bu KOMPILE BO'LMAYDI:
    // Из-за invariance &mut T — это НЕ СКОМПИЛИРУЕТСЯ:
    // fn invariant_test<'a>(x: &'a mut &'a str) { ... }
    // let mut s: &'static str = "statik";
    // invariant_test(&mut s); // ← xavfli bo'lar edi
    println!("Invariance — &mut T turni o'zgartirish imkoni yo'q");
    // Invariance — нельзя изменить тип &mut T
}

fn contravariance_misoli() {

    // Contravariance — funksiya argumenti
    // Contravariance — аргумент функции
    // fn(T) — T bo'yicha contravariant
    // fn(T) — contravariant по T

    // Fn trait — argument contravariant, return covariant
    // Fn trait — аргумент contravariant, возврат covariant
    fn qabul_qiluvchi<F>(f: F, s: &str)
    where
        F: Fn(&str),
    {
        f(s);
    }

    // Keng argument qabul qiluvchi closure — tor tur o'rnida ishlatilishi mumkin
    // Closure принимающая широкий тип — может использоваться вместо узкого
    let f = |s: &str| println!("Qabul qilindi: {}", s);
    qabul_qiluvchi(f, "salom");
    // Qabul qilindi: salom
}

// PhantomData va variance
// PhantomData и variance
struct Covariant<'a, T> {
    qiymat: &'a T,  // covariant — &T covariant
}

struct Invariant<'a, T> {
    qiymat: &'a mut T,  // invariant — &mut T invariant
}

struct ContravariantFn<T> {
    // fn(T) — T bo'yicha contravariant
    // fn(T) — contravariant по T
    _marker: PhantomData<fn(T)>,
}

fn variance_misollari() {
    covariance_misoli();
    invariance_misoli();
    contravariance_misoli();

    // Variance jadvali
    // Таблица variance
    println!("\nVariance jadvali:");
    println!("&'a T      — 'a va T bo'yicha COVARIANT");
    println!("&'a mut T  — 'a bo'yicha covariant, T bo'yicha INVARIANT");
    println!("fn(T) -> U — T bo'yicha CONTRAVARIANT, U bo'yicha covariant");
    println!("Box<T>     — T bo'yicha COVARIANT");
    println!("Vec<T>     — T bo'yicha COVARIANT");
    println!("Cell<T>    — T bo'yicha INVARIANT");
    // Variance jadvali:
    // &'a T      — 'a va T bo'yicha COVARIANT
    // &'a mut T  — 'a bo'yicha covariant, T bo'yicha INVARIANT
    // fn(T) -> U — T bo'yicha CONTRAVARIANT, U bo'yicha covariant
    // Box<T>     — T bo'yicha COVARIANT
    // Vec<T>     — T bo'yicha COVARIANT
    // Cell<T>    — T bo'yicha INVARIANT
}

// HRTB — funksiya BARCHA lifetimelar uchun ishlashi kerak
// HRTB — функция должна работать для ВСЕХ lifetime

// for<'a> — "barcha mumkin bo'lgan 'a uchun"
// for<'a> — "для всех возможных 'a"

fn hrtb_qabul<F>(f: F, s: &str) -> &str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(s)
}

fn hrtb_fn_qabul(f: fn(&str) -> &str, s: &str) -> &str {
    f(s)
}

// HRTB bilan trait
// Трейт с HRTB
trait StrQayta {
    fn qayta_ishla<'a>(&self, s: &'a str) -> &'a str;
}

struct Trimlovchi;
struct KattaHarflovchi;

impl StrQayta for Trimlovchi {
    fn qayta_ishla<'a>(&self, s: &'a str) -> &'a str {
        s.trim()
    }
}

// fn pointer — for<'a> avtomatik implement qiladi
// fn pointer — автоматически реализует for<'a>
fn birinchi_soz(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn hrtb_misollari() {

    // fn pointer — HRTB avtomatik
    // fn pointer — HRTB автоматически
    let natija = hrtb_qabul(birinchi_soz, "salom dunyo rust");
    println!("{}", natija);
    // salom

    // fn pointer to'g'ridan
    // fn pointer напрямую
    let natija2 = hrtb_fn_qabul(str::trim, "  salom  ");
    println!("{}", natija2);
    // salom

    // Turli lifetimelar bilan ishlash
    // Работа с разными lifetime
    let s1 = String::from("birinchi string");
    let natija3 = hrtb_qabul(birinchi_soz, &s1);
    println!("{}", natija3);
    // birinchi

    let s2: &str = "statik string";
    let natija4 = hrtb_qabul(birinchi_soz, s2);
    println!("{}", natija4);
    // statik

    // StrQayta trait
    // Трейт StrQayta
    let trimlovchi = Trimlovchi;
    let s3 = String::from("  salom  ");
    let natija5 = trimlovchi.qayta_ishla(&s3);
    println!("{}", natija5);
    // salom
}

// Self-referential struct — o'ziga reference saqlash
// Self-referential struct — хранение ссылки на себя
//
// Bu muammo — Rust lifetime tizimi bilan hal qilib bo'lmaydi
// Это проблема — не решается системой lifetime Rust
//
// YECHIMLAR:
// РЕШЕНИЯ:
//   1. Indeks ishlatish — reference o'rniga
//      Использование индексов — вместо ссылок
//   2. Rc<RefCell<T>> — egalik va o'zgartirishni ajratish
//      Rc<RefCell<T>> — разделение владения и изменяемости
//   3. Pin<Box<T>> — xotirada o'rnini o'zgartirmaslik
//      Pin<Box<T>> — запрет перемещения в памяти

// 1-yechim: Indeks bilan
// Решение 1: С индексами
struct Daraxt {
    tugunlar: Vec<DaraxtTugun>,
}

struct DaraxtTugun {
    qiymat: i32,
    ota_indeks: Option<usize>,  // reference emas — indeks
    bolalar_indekslari: Vec<usize>,
}

impl Daraxt {
    fn new() -> Self { Daraxt { tugunlar: Vec::new() } }

    fn tugun_qo_sh(&mut self, qiymat: i32, ota: Option<usize>) -> usize {
        let indeks = self.tugunlar.len();
        self.tugunlar.push(DaraxtTugun {
            qiymat,
            ota_indeks: ota,
            bolalar_indekslari: Vec::new(),
        });
        if let Some(ota_idx) = ota {
            self.tugunlar[ota_idx].bolalar_indekslari.push(indeks);
        }
        indeks
    }

    fn qiymat(&self, indeks: usize) -> i32 {
        self.tugunlar[indeks].qiymat
    }
}

// 2-yechim: Rc<RefCell<T>>
// Решение 2: Rc<RefCell<T>>
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct RcTugun {
    qiymat: i32,
    bolalar: Vec<Rc<RefCell<RcTugun>>>,
}

impl RcTugun {
    fn new(qiymat: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(RcTugun { qiymat, bolalar: Vec::new() }))
    }

    fn bola_qo_sh(ota: &Rc<RefCell<Self>>, bola: Rc<RefCell<Self>>) {
        ota.borrow_mut().bolalar.push(bola);
    }
}

fn self_referential_misollari() {

    // 1-yechim: Indeks
    // Решение 1: Индекс
    let mut daraxt = Daraxt::new();
    let ildiz = daraxt.tugun_qo_sh(1, None);
    let chap = daraxt.tugun_qo_sh(2, Some(ildiz));
    let ong = daraxt.tugun_qo_sh(3, Some(ildiz));
    let chap_chap = daraxt.tugun_qo_sh(4, Some(chap));

    println!("Ildiz: {}", daraxt.qiymat(ildiz));
    println!("Chap: {}", daraxt.qiymat(chap));
    println!("Ong: {}", daraxt.qiymat(ong));
    println!("Bolalar soni: {}", daraxt.tugunlar[ildiz].bolalar_indekslari.len());
    // Ildiz: 1
    // Chap: 2
    // Ong: 3
    // Bolalar soni: 2

    // 2-yechim: Rc<RefCell<T>>
    // Решение 2: Rc<RefCell<T>>
    let ildiz_rc = RcTugun::new(10);
    let chap_rc = RcTugun::new(20);
    let ong_rc = RcTugun::new(30);

    RcTugun::bola_qo_sh(&ildiz_rc, Rc::clone(&chap_rc));
    RcTugun::bola_qo_sh(&ildiz_rc, Rc::clone(&ong_rc));

    println!("Ildiz: {}", ildiz_rc.borrow().qiymat);
    println!("Bolalar: {}", ildiz_rc.borrow().bolalar.len());
    // Ildiz: 10
    // Bolalar: 2
}

struct Cache<'a> {
    ma_lumotlar: Vec<&'a str>,
    kesh: std::collections::HashMap<&'a str, usize>,
}

impl<'a> Cache<'a> {
    fn new() -> Self {
        Cache { ma_lumotlar: Vec::new(), kesh: std::collections::HashMap::new() }
    }

    fn qo_sh(&mut self, element: &'a str) -> usize {
        if let Some(&i) = self.kesh.get(element) { return i; }
        let i = self.ma_lumotlar.len();
        self.ma_lumotlar.push(element);
        self.kesh.insert(element, i);
        i
    }

    fn ol(&self, i: usize) -> Option<&'a str> {
        self.ma_lumotlar.get(i).copied()
    }
}

struct Parser<'a> {
    kiritish: &'a str,
    pozitsiya: usize,
}

impl<'a> Parser<'a> {
    fn new(kiritish: &'a str) -> Self {
        Parser { kiritish, pozitsiya: 0 }
    }

    fn keyingi_soz(&mut self) -> Option<&'a str> {
        while self.pozitsiya < self.kiritish.len()
            && self.kiritish.as_bytes()[self.pozitsiya] == b' ' {
            self.pozitsiya += 1;
        }
        if self.pozitsiya >= self.kiritish.len() { return None; }
        let bosh = self.pozitsiya;
        while self.pozitsiya < self.kiritish.len()
            && self.kiritish.as_bytes()[self.pozitsiya] != b' ' {
            self.pozitsiya += 1;
        }
        Some(&self.kiritish[bosh..self.pozitsiya])
    }

    fn barcha_sozlar(&mut self) -> Vec<&'a str> {
        let mut v = Vec::new();
        while let Some(s) = self.keyingi_soz() { v.push(s); }
        v
    }
}

fn real_hayot_misollari() {

    let s1 = String::from("salom");
    let s2 = String::from("dunyo");
    let s3 = String::from("rust");

    let mut kesh = Cache::new();
    let i1 = kesh.qo_sh(&s1);
    let i2 = kesh.qo_sh(&s2);
    let i3 = kesh.qo_sh(&s3);
    let i1_2 = kesh.qo_sh(&s1);

    println!("Indekslar: {} {} {} {}", i1, i2, i3, i1_2);
    println!("{:?}", kesh.ol(1));
    // Indekslar: 0 1 2 0
    // Some("dunyo")

    let matn = String::from("salom dunyo rust tili ajoyib");
    let mut parser = Parser::new(&matn);
    let sozlar = parser.barcha_sozlar();
    println!("{:?}", sozlar);
    // ["salom", "dunyo", "rust", "tili", "ajoyib"]
}

fn main() {

    println!("=== ASOSIY LIFETIME ===");
    asosiy_lifetime_misollari();

    println!("\n=== STRUCT LIFETIME ===");
    struct_lifetime_misollari();

    println!("\n=== 'STATIC LIFETIME ===");
    static_misol();

    println!("\n=== LIFETIME BOUNDS ===");
    lifetime_bound_misollari();

    println!("\n=== ELISION QOIDALARI ===");
    elision_misollari();

    println!("\n=== NLL — NON-LEXICAL LIFETIMES ===");
    nll_misollari();

    println!("\n=== VARIANCE ===");
    variance_misollari();

    println!("\n=== HRTB — for<'a> ===");
    hrtb_misollari();

    println!("\n=== SELF-REFERENTIAL ===");
    self_referential_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                | Tavsif (UZ)                                | Описание (RU)                                               |
// #================================================================================================================================================#
// # |   1 | fn f<'a>(x: &'a T) -> &'a U| Named lifetime                              | Именованный lifetime                                        |
// # |   2 | struct S<'a> { r: &'a T }   | Struct ichida reference                    | Ссылка в структуре                                          |
// # |   3 | &'static T                  | Dastur tugaguncha                          | До конца программы                                          |
// # |   4 | T: 'static                  | T dastur davomida yashaydi                 | T живёт всё время программы                                 |
// # |   5 | Box::leak(val)              | 'static reference yaratish (leak!)         | Создание 'static ссылки (утечка!)                           |
// # |   6 | 'a: 'b                      | 'a kamida 'b qadar yashaydi                | 'a живёт не меньше 'b                                       |
// # |   7 | T: 'a                       | T 'a dan uzoq yashaydi                     | T живёт дольше 'a                                           |
// # |   8 | Elision qoida 1,2,3         | Ko'p holda yozmaslik mumkin                | Можно не писать в большинстве случаев                       |
// # |   9 | NLL                         | Borrow oxirgi ishlatilganda tugaydi        | Borrow заканчивается в последнем use                        |
// # |  10 | Covariant (&'a T)           | Uzun lifetime qisqa o'rnida                | Длинный lifetime вместо короткого                           |
// # |  11 | Invariant (&'a mut T)       | T bo'yicha invariant — tur o'zgarmaydi     | Invariant по T — тип не меняется                            |
// # |  12 | Contravariant (fn(T))       | Argument pozitsiyasi                       | Позиция аргумента                                           |
// # |  13 | for<'a> Fn(&'a str)         | HRTB — barcha lifetimelar uchun            | HRTB — для всех lifetime                                    |
// # |  14 | Indeks pattern              | Self-ref yechimi — safe                    | Решение self-ref — безопасно                                |
// # |  15 | Rc<RefCell<T>>              | Self-ref yechimi — shared ownership        | Решение self-ref — общее владение                           |
// #================================================================================================================================================#