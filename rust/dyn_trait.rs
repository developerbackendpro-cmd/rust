// #================================================================================================================================================#
// #                                                            DYN TRAIT  +  OBJECT SAFETY                                                         #
// #                                DYN TRAIT — RUNTIME DA ANIQLANADIGAN TRAIT. OBJECT SAFETY — DYN UCHUN QOIDALAR.                                 #
// #                                DYN TRAIT — ТРЕЙТ ОПРЕДЕЛЯЕМЫЙ ВО ВРЕМЯ ВЫПОЛНЕНИЯ. OBJECT SAFETY — ПРАВИЛА ДЛЯ DYN.                            #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// dyn Trait — runtime da qaysi struct ekanligini bilamiz
// dyn Trait — во время выполнения узнаём какая это структура
trait Chizish {
    fn chiz(&self);
    fn nomi(&self) -> &str;
}

struct Doira {
    radius: f64,
}

struct Turtburchak {
    eni: f64,
    boyi: f64,
}

struct Uchburchak {
    asos: f64,
    balandlik: f64,
}

impl Chizish for Doira {
    fn chiz(&self) {
        println!("Doira chizildi (radius: {})", self.radius);
    }
    fn nomi(&self) -> &str { "Doira" }
}

impl Chizish for Turtburchak {
    fn chiz(&self) {
        println!("Turtburchak chizildi ({}x{})", self.eni, self.boyi);
    }
    fn nomi(&self) -> &str { "Turtburchak" }
}

impl Chizish for Uchburchak {
    fn chiz(&self) {
        println!("Uchburchak chizildi (asos: {}, h: {})", self.asos, self.balandlik);
    }
    fn nomi(&self) -> &str { "Uchburchak" }
}

// &dyn Trait — reference orqali trait object
// &dyn Trait — объект трейта через ссылку
fn bitta_chiz(shakl: &dyn Chizish) {
    shakl.chiz();
    println!("Shakl: {}", shakl.nomi());
}

// Box<dyn Trait> — heap da ownership bilan
// Box<dyn Trait> — в куче с владением
fn shakllar_yasash() -> Vec<Box<dyn Chizish>> {
    vec![
        Box::new(Doira { radius: 5.0 }),
        Box::new(Turtburchak { eni: 4.0, boyi: 3.0 }),
        Box::new(Uchburchak { asos: 6.0, balandlik: 4.0 }),
    ]
}

// dyn Trait = fat pointer: (data ptr, vtable ptr)
// dyn Trait = толстый указатель: (указатель данных, указатель vtable)
//
// vtable ichida:
// внутри vtable:
//   - drop funksiyasi
//   - size va alignment
//   - har bir metod uchun funksiya pointeri
//
// &dyn Trait  = 16 bayt (2 pointer)
// &i32        = 8 bayt  (1 pointer)

fn fat_pointer_olchami() {
    let ref_hajmi: usize = std::mem::size_of::<&dyn Chizish>();
    let box_hajmi: usize = std::mem::size_of::<Box<dyn Chizish>>();
    let oddiy_ref: usize = std::mem::size_of::<&i32>();

    println!("&dyn Chizish hajmi: {} bayt", ref_hajmi);
    println!("Box<dyn Chizish> hajmi: {} bayt", box_hajmi);
    println!("&i32 hajmi: {} bayt", oddiy_ref);
    // &dyn Chizish hajmi: 16 bayt
    // Box<dyn Chizish> hajmi: 16 bayt
    // &i32 hajmi: 8 bayt
}

// generic — compile time, monomorphization, zero cost
// generic — время компиляции, мономорфизация, без затрат
fn generic_chiz<T: Chizish>(shakl: &T) {
    shakl.chiz();
}

// dyn — runtime, dynamic dispatch, kichik overhead
// dyn — время выполнения, динамическая диспетчеризация, небольшие затраты
fn dynamic_chiz(shakl: &dyn Chizish) {
    shakl.chiz();
}

// OBJECT SAFE bo'lishi uchun:
// ДЛЯ OBJECT SAFETY:
//
// 1. Metod Self ni qaytarmasligi kerak
//    Метод не должен возвращать Self
//
// 2. Metod generic bo'lmasligi kerak
//    Метод не должен быть generic
//
// 3. const associated items bo'lmasligi kerak
//    Не должно быть const associated items
//
// 4. Metod Self: Sized talab qilmasligi kerak
//    Метод не должен требовать Self: Sized

// OBJECT SAFE — barcha metodlar oddiy
// OBJECT SAFE — все методы обычные
trait ObjectSafeTrait {
    fn metod1(&self) -> i32;
    fn metod2(&self, x: i32) -> String;
    fn metod3(&mut self);
}

struct OddiyStruct {
    qiymat: i32,
}

impl ObjectSafeTrait for OddiyStruct {
    fn metod1(&self) -> i32 { self.qiymat }
    fn metod2(&self, x: i32) -> String { format!("{}", x) }
    fn metod3(&mut self) { self.qiymat += 1; }
}

// ❌ OBJECT UNSAFE — Self qaytaradi
// ❌ OBJECT UNSAFE — возвращает Self
// trait Klonlash {
//     fn klon(&self) -> Self;  // Self qaytarib bo'lmaydi dyn bilan!
// }

// ✅ YECHIM — Box<dyn Trait> qaytarish
// ✅ РЕШЕНИЕ — возврат Box<dyn Trait>
trait Klonlash {
    fn klon(&self) -> Box<dyn Klonlash>;
    fn nomi(&self) -> &str;
}

struct KlonlanuvchiA {
    qiymat: i32,
}

struct KlonlanuvchiB {
    matn: String,
}

impl Klonlash for KlonlanuvchiA {
    fn klon(&self) -> Box<dyn Klonlash> {
        Box::new(KlonlanuvchiA { qiymat: self.qiymat })
    }
    fn nomi(&self) -> &str { "A" }
}

impl Klonlash for KlonlanuvchiB {
    fn klon(&self) -> Box<dyn Klonlash> {
        Box::new(KlonlanuvchiB { matn: self.matn.clone() })
    }
    fn nomi(&self) -> &str { "B" }
}

// ❌ OBJECT UNSAFE — generic metod
// ❌ OBJECT UNSAFE — обобщённый метод
// trait GenericMetod {
//     fn f<T>(&self, x: T);  // generic metod dyn bilan ishlamaydi!
// }

// ✅ YECHIM — generic metodni where Self: Sized bilan cheklash
// ✅ РЕШЕНИЕ — ограничение generic метода через where Self: Sized
trait AralashTrait {
    // bu metod dyn bilan ishlamaydi — faqat generic bilan
    // этот метод не работает с dyn — только с generic
    fn generic_metod<T: fmt::Display>(&self, x: T) where Self: Sized {
        println!("{}", x);
    }

    // bu metod dyn bilan ishlaydi
    // этот метод работает с dyn
    fn oddiy_metod(&self) -> String;
}

struct AralashStruct {
    nomi: String,
}

impl AralashTrait for AralashStruct {
    fn oddiy_metod(&self) -> String {
        self.nomi.clone()
    }
}

// ❌ OBJECT UNSAFE — associated const
// ❌ OBJECT UNSAFE — ассоциированная константа
// trait ConstBilan {
//     const QIYMAT: i32;  // const dyn bilan ishlamaydi!
// }

// ✅ YECHIM — const o'rniga fn ishlatish
// ✅ РЕШЕНИЕ — использовать fn вместо const
trait ConstBilan {
    fn qiymat(&self) -> i32;  // fn — dyn bilan ishlaydi
}

struct ConstStruct;

impl ConstBilan for ConstStruct {
    fn qiymat(&self) -> i32 { 42 }
}

// plugin tizimi — dyn Trait eng yaxshi tanlov
// система плагинов — dyn Trait лучший выбор
trait Plugin {
    fn nomi(&self) -> &str;
    fn ishga_tush(&self);
    fn toxtat(&self);
}

struct LogPlugin {
    fayl_nomi: String,
}

struct MetrikaPlugin {
    interval: u32,
}

impl Plugin for LogPlugin {
    fn nomi(&self) -> &str { "Log Plugin" }
    fn ishga_tush(&self) {
        println!("Log plugin boshlandi: {}", self.fayl_nomi);
    }
    fn toxtat(&self) {
        println!("Log plugin to'xtatildi");
    }
}

impl Plugin for MetrikaPlugin {
    fn nomi(&self) -> &str { "Metrika Plugin" }
    fn ishga_tush(&self) {
        println!("Metrika plugin boshlandi (interval: {}s)", self.interval);
    }
    fn toxtat(&self) {
        println!("Metrika plugin to'xtatildi");
    }
}

struct PluginManager {
    pluginlar: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    fn new() -> Self {
        PluginManager { pluginlar: Vec::new() }
    }

    fn qo_sh(&mut self, plugin: Box<dyn Plugin>) {
        println!("Plugin qo'shildi: {}", plugin.nomi());
        self.pluginlar.push(plugin);
    }

    fn hammasini_ishga_tush(&self) {
        for plugin in &self.pluginlar {
            plugin.ishga_tush();
        }
    }

    fn hammasini_toxtat(&self) {
        for plugin in &self.pluginlar {
            plugin.toxtat();
        }
    }
}

// std::error::Error — object safe trait
// std::error::Error — object safe трейт
use std::error::Error;

#[derive(Debug)]
struct TarmoqXato {
    xabar: String,
}

#[derive(Debug)]
struct FaylXato {
    fayl: String,
}

impl fmt::Display for TarmoqXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tarmoq xatosi: {}", self.xabar)
    }
}

impl fmt::Display for FaylXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fayl xatosi: {}", self.fayl)
    }
}

impl Error for TarmoqXato {}
impl Error for FaylXato {}

// Box<dyn Error> — turli xatolarni birga qaytarish
// Box<dyn Error> — возврат разных ошибок вместе
fn amaliyot_bajari(muvaffaqiyatlimi: bool) -> Result<String, Box<dyn Error>> {
    if muvaffaqiyatlimi {
        Ok(String::from("Muvaffaqiyatli!"))
    } else {
        Err(Box::new(TarmoqXato {
            xabar: String::from("ulanish rad etildi"),
        }))
    }
}

fn fayl_o_qi(mavjudmi: bool) -> Result<String, Box<dyn Error>> {
    if mavjudmi {
        Ok(String::from("fayl mazmuni"))
    } else {
        Err(Box::new(FaylXato {
            fayl: String::from("config.toml"),
        }))
    }
}

use std::rc::Rc;
use std::sync::Arc;

// Rc<dyn Trait> — shared ownership (single thread)
// Rc<dyn Trait> — совместное владение (один поток)
fn rc_dyn_misol() {
    let shakl: Rc<dyn Chizish> = Rc::new(Doira { radius: 3.0 });
    let shakl2: Rc<dyn Chizish> = Rc::clone(&shakl);

    shakl.chiz();
    shakl2.chiz();
    println!("RC soni: {}", Rc::strong_count(&shakl));
    // Doira chizildi (radius: 3)
    // Doira chizildi (radius: 3)
    // RC soni: 2
}

// Arc<dyn Trait + Send + Sync> — multi-thread uchun
// Arc<dyn Trait + Send + Sync> — для многопоточности
fn arc_dyn_misol() {
    let shakl: Arc<dyn Chizish + Send + Sync> = Arc::new(Turtburchak { eni: 5.0, boyi: 3.0 });
    let shakl2 = Arc::clone(&shakl);

    let handle = std::thread::spawn(move || {
        shakl2.chiz();
    });

    shakl.chiz();
    handle.join().unwrap();
    // Turtburchak chizildi (5x3)
    // Turtburchak chizildi (5x3)
}

// callback — dyn Fn bilan
// callback — с dyn Fn
struct Hodisa {
    nomi: String,
    tinglovchilar: Vec<Box<dyn Fn(&str)>>,
}

impl Hodisa {
    fn new(nomi: &str) -> Self {
        Hodisa {
            nomi: nomi.to_string(),
            tinglovchilar: Vec::new(),
        }
    }

    fn tinglovchi_qo_sh(&mut self, f: Box<dyn Fn(&str)>) {
        self.tinglovchilar.push(f);
    }

    fn yuborish(&self, xabar: &str) {
        for tinglovchi in &self.tinglovchilar {
            tinglovchi(xabar);
        }
    }
}

// thread safe dyn trait
// потокобезопасный dyn trait
trait ThreadSafe: Send + Sync {
    fn ishlash(&self) -> String;
}

struct XavfsizStruct {
    qiymat: i32,
}

impl ThreadSafe for XavfsizStruct {
    fn ishlash(&self) -> String {
        format!("qiymat: {}", self.qiymat)
    }
}

fn thread_safe_ishlash(narsa: &(dyn ThreadSafe)) {
    println!("{}", narsa.ishlash());
}

fn main() {

    // &dyn Trait — reference orqali
    // &dyn Trait — через ссылку
    let doira = Doira { radius: 5.0 };
    let turtburchak = Turtburchak { eni: 4.0, boyi: 3.0 };
    bitta_chiz(&doira);
    bitta_chiz(&turtburchak);
    // Doira chizildi (radius: 5)
    // Turtburchak chizildi (4x3)

    // Box<dyn Trait> — heap da ownership
    // Box<dyn Trait> — владение в куче
    let shakllar = shakllar_yasash();
    for shakl in &shakllar {
        shakl.chiz();
    }
    // Doira chizildi (radius: 5)
    // Turtburchak chizildi (4x3)
    // Uchburchak chizildi (asos: 6, h: 4)

    fat_pointer_olchami();
    // &dyn Chizish hajmi: 16 bayt
    // Box<dyn Chizish> hajmi: 16 bayt
    // &i32 hajmi: 8 bayt

    // generic — compile time
    // generic — время компиляции
    let doira2 = Doira { radius: 2.0 };
    generic_chiz(&doira2);
    // Doira chizildi (radius: 2)

    // dyn — runtime
    // dyn — время выполнения
    let shakl: &dyn Chizish = &doira2;
    dynamic_chiz(shakl);
    // Doira chizildi (radius: 2)

    // object safe trait — dyn bilan ishlaydi
    // object safe трейт — работает с dyn
    let mut oddiy = OddiyStruct { qiymat: 10 };
    let dyn_ref: &dyn ObjectSafeTrait = &oddiy;
    println!("{}", dyn_ref.metod1());
    println!("{}", dyn_ref.metod2(42));
    // 10
    // 42

    // Klonlash — Box<dyn Klonlash> qaytaradi
    // Klonlash — возвращает Box<dyn Klonlash>
    let a: Box<dyn Klonlash> = Box::new(KlonlanuvchiA { qiymat: 99 });
    let a_klon = a.klon();
    println!("Original: {}", a.nomi());
    println!("Klon: {}", a_klon.nomi());
    // Original: A
    // Klon: A

    // where Self: Sized bilan generic metod
    // generic метод с where Self: Sized
    let aralash = AralashStruct { nomi: String::from("test") };
    println!("{}", aralash.oddiy_metod());
    aralash.generic_metod(42);
    // test
    // 42

    // const o'rniga fn
    // fn вместо const
    let c = ConstStruct;
    let dyn_c: &dyn ConstBilan = &c;
    println!("{}", dyn_c.qiymat());
    // 42

    // plugin manager — dyn Trait
    // менеджер плагинов — dyn Trait
    let mut manager = PluginManager::new();
    manager.qo_sh(Box::new(LogPlugin {
        fayl_nomi: String::from("app.log"),
    }));
    manager.qo_sh(Box::new(MetrikaPlugin { interval: 30 }));
    println!("--- Boshlash ---");
    manager.hammasini_ishga_tush();
    println!("--- To'xtatish ---");
    manager.hammasini_toxtat();
    // Plugin qo'shildi: Log Plugin
    // Plugin qo'shildi: Metrika Plugin
    // --- Boshlash ---
    // Log plugin boshlandi: app.log
    // Metrika plugin boshlandi (interval: 30s)
    // --- To'xtatish ---
    // Log plugin to'xtatildi
    // Metrika plugin to'xtatildi

    // turli xatolarni Box<dyn Error> bilan qaytarish
    // возврат разных ошибок через Box<dyn Error>
    let muvaffaqiyatli = amaliyot_bajari(true);
    let xatoli = amaliyot_bajari(false);
    println!("{:?}", muvaffaqiyatli);
    println!("{:?}", xatoli);
    // Ok("Muvaffaqiyatli!")
    // Err(TarmoqXato { xabar: "ulanish rad etildi" })

    let mavjud = fayl_o_qi(true);
    let mavjud_emas = fayl_o_qi(false);
    println!("{:?}", mavjud);
    println!("{:?}", mavjud_emas);
    // Ok("fayl mazmuni")
    // Err(FaylXato { fayl: "config.toml" })

    rc_dyn_misol();
    arc_dyn_misol();

    // hodisa va tinglovchilar
    // событие и слушатели
    let mut hodisa = Hodisa::new("yangi_foydalanuvchi");
    hodisa.tinglovchi_qo_sh(Box::new(|xabar| {
        println!("Tinglovchi 1: {}", xabar);
    }));
    hodisa.tinglovchi_qo_sh(Box::new(|xabar| {
        println!("Tinglovchi 2: {}", xabar.to_uppercase());
    }));
    hodisa.yuborish("dilshod ro'yxatdan o'tdi");
    // Tinglovchi 1: dilshod ro'yxatdan o'tdi
    // Tinglovchi 2: DILSHOD RO'YXATDAN O'TDI

    // Send + Sync bilan dyn trait
    // dyn trait с Send + Sync
    let xavfsiz = XavfsizStruct { qiymat: 42 };
    thread_safe_ishlash(&xavfsiz);
    // qiymat: 42
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya               | Tavsif (UZ)                                        | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                          DYN TRAIT ASOSLARI                                                                                  |
// #================================================================================================================================================#
// # |   1 | &dyn Trait                 | Reference orqali trait object                      | Объект трейта через ссылку                           |
// # |   2 | Box<dyn Trait>             | Heap da trait object (ownership)                   | Объект трейта в куче (с владением)                   |
// # |   3 | Rc<dyn Trait>              | Shared ownership (single thread)                   | Совместное владение (один поток)                     |
// # |   4 | Arc<dyn Trait+Send+Sync>   | Shared ownership (multi thread)                    | Совместное владение (многопоточность)                |
// # |   5 | Vec<Box<dyn Trait>>        | Turli turlarni birga saqlash                       | Хранение разных типов вместе                         |
// #================================================================================================================================================#
// # |                                          FAT POINTER                                                                                         |
// #================================================================================================================================================#
// # |   6 | &dyn T = 16 bayt           | Data ptr + vtable ptr                              | Указатель данных + указатель vtable                  |
// # |   7 | vtable                     | Drop + size + har bir metod uchun pointer          | Drop + size + указатель для каждого метода           |
// #================================================================================================================================================#
// # |                                       GENERIC VS DYN FARQI                                                                                   |
// #================================================================================================================================================#
// # |   8 | generic<T: Trait>          | Compile time, zero cost, monomorphization          | Компиляция, без затрат, мономорфизация               |
// # |   9 | &dyn Trait                 | Runtime, dynamic dispatch, kichik overhead         | Выполнение, динамическая диспетчеризация             |
// # |  10 | generic                    | Vec<T> — bitta tur                                 | Vec<T> — один тип                                    |
// # |  11 | dyn Trait                  | Vec<Box<dyn T>> — turli turlar                     | Vec<Box<dyn T>> — разные типы                        |
// #================================================================================================================================================#
// # |                                        OBJECT SAFETY QOIDALARI                                                                               |
// #================================================================================================================================================#
// # |  12 | Self qaytarmasin           | fn klon(&self) -> Self — ❌ dyn bilan emas          | fn klon(&self) -> Self — ❌ нельзя с dyn            |
// # |  13 | Generic metod bo'lmasin    | fn f<T>(&self) — ❌ dyn bilan emas                  | fn f<T>(&self) — ❌ нельзя с dyn                    |
// # |  14 | const bo'lmasin            | const QIYMAT: i32 — ❌ dyn bilan emas               | const QIYMAT: i32 — ❌ нельзя с dyn                 |
// # |  15 | Self: Sized bo'lmasin      | where Self: Sized orqali cheklash mumkin            | Можно ограничить через where Self: Sized            |
// #================================================================================================================================================#
// # |                                        OBJECT SAFETY YECHIMLARI                                                                              |
// #================================================================================================================================================#
// # |  16 | Self o'rniga Box<dyn T>    | fn klon(&self) -> Box<dyn Trait> — ✅               | fn klon(&self) -> Box<dyn Trait>  ✅                |
// # |  17 | const o'rniga fn           | fn qiymat(&self) -> i32 — ✅                        | fn qiymat(&self) -> i32  ✅                         |
// # |  18 | where Self: Sized          | Generic metodni cheklash — ✅                       | Ограничение generic метода  ✅                      |
// #================================================================================================================================================#
// # |                                         REAL HAYOT QOLLASH                                                                                   |
// #================================================================================================================================================#
// # |  19 | Plugin tizimi              | Turli pluginlarni birga boshqarish                 | Управление разными плагинами вместе                  |
// # |  20 | Box<dyn Error>             | Turli xatolarni birga qaytarish                    | Возврат разных ошибок вместе                         |
// # |  21 | Box<dyn Fn(...)>           | Callback pattern                                   | Паттерн обратного вызова                             |
// # |  22 | dyn Trait + Send + Sync    | Thread safe trait object                           | Потокобезопасный объект трейта                       |
// #================================================================================================================================================#