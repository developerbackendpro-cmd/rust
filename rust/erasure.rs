// #================================================================================================================================================#
// #                                                                TYPE ERASURE                                                                    #
// #                    TYPE ERASURE — TUR MA'LUMOTINI YASHIRISH. DYN TRAIT, BOX<DYN>, ARC<DYN>, FAT POINTER. OBJECT SAFETY.                        #
// #                    TYPE ERASURE — СКРЫТИЕ ИНФОРМАЦИИ О ТИПЕ. DYN TRAIT, BOX<DYN>, ARC<DYN>, FAT POINTER. OBJECT SAFETY.                        #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::any::{Any, TypeId};
use std::sync::Arc;
use std::collections::HashMap;

// Type Erasure nima:
// Что такое Type Erasure:
//
//   Konkret turni umumiy interfeys ortida yashirish
//   Скрытие конкретного типа за общим интерфейсом
//
//   Rust da usullari:
//   Способы в Rust:
//   1. dyn Trait — dynamic dispatch, fat pointer
//      dyn Trait — динамическая диспетчеризация, fat pointer
//   2. Box<dyn Trait> — heap da erased type
//      Box<dyn Trait> — erased type в куче
//   3. Arc<dyn Trait> — shared erased type
//      Arc<dyn Trait> — общий erased type
//   4. impl Trait — static dispatch, compile-time erasure
//      impl Trait — статическая диспетчеризация, compile-time
//   5. fn() pointer — closure erasure
//      fn() указатель — стирание замыкания
//
//   Fat pointer nima:
//   Что такое fat pointer:
//   &dyn Trait = (data_ptr, vtable_ptr) — ikki pointer
//   &dyn Trait = (data_ptr, vtable_ptr) — два указателя
//   Hajm: 2 * usize = 16 bayt (64-bit)
//   Размер: 2 * usize = 16 байт (64-bit)

trait Shakl: fmt::Debug {
    fn yuzi(&self) -> f64;
    fn perimetri(&self) -> f64;
    fn nomi(&self) -> &str;
}

#[derive(Debug)]
struct Aylana { radius: f64 }

#[derive(Debug)]
struct Togritortburchak { eni: f64, boyi: f64 }

#[derive(Debug)]
struct Uchburchak { a: f64, b: f64, c: f64 }

impl Shakl for Aylana {
    fn yuzi(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimetri(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
    fn nomi(&self) -> &str { "Aylana" }
}

impl Shakl for Togritortburchak {
    fn yuzi(&self) -> f64 { self.eni * self.boyi }
    fn perimetri(&self) -> f64 { 2.0 * (self.eni + self.boyi) }
    fn nomi(&self) -> &str { "To'g'rito'rtburchak" }
}

impl Shakl for Uchburchak {
    fn yuzi(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn perimetri(&self) -> f64 { self.a + self.b + self.c }
    fn nomi(&self) -> &str { "Uchburchak" }
}

fn dyn_trait_misoli() {

    // Box<dyn Trait> — type erasure
    // Box<dyn Trait> — стирание типа
    let shakllar: Vec<Box<dyn Shakl>> = vec![
        Box::new(Aylana { radius: 5.0 }),
        Box::new(Togritortburchak { eni: 4.0, boyi: 6.0 }),
        Box::new(Uchburchak { a: 3.0, b: 4.0, c: 5.0 }),
    ];

    for sh in &shakllar {
        println!("{}: yuzi={:.2}, perimetri={:.2}",
                 sh.nomi(), sh.yuzi(), sh.perimetri());
    }
    // Aylana: yuzi=78.54, perimetri=31.42
    // To'g'rito'rtburchak: yuzi=24.00, perimetri=20.00
    // Uchburchak: yuzi=6.00, perimetri=12.00

    // Umumiy hisoblash — konkret tur muhim emas
    // Общее вычисление — конкретный тип не важен
    let jami_yuzi: f64 = shakllar.iter().map(|s| s.yuzi()).sum();
    let eng_katta = shakllar.iter().max_by(|a, b| {
        a.yuzi().partial_cmp(&b.yuzi()).unwrap()
    });
    println!("Jami yuzi: {:.2}", jami_yuzi);
    println!("Eng katta: {}", eng_katta.map(|s| s.nomi()).unwrap_or("yo'q"));
    // Jami yuzi: 108.54
    // Eng katta: Aylana

    // Fat pointer o'lchami
    println!("\nO'lchamlar:");
    println!("&Aylana:      {} bayt", std::mem::size_of::<&Aylana>());     // 8
    println!("&dyn Shakl:   {} bayt", std::mem::size_of::<&dyn Shakl>()); // 16
    println!("Box<Aylana>:  {} bayt", std::mem::size_of::<Box<Aylana>>()); // 8
    println!("Box<dyn Sh>:  {} bayt", std::mem::size_of::<Box<dyn Shakl>>()); // 16
    // &Aylana:      8 bayt
    // &dyn Shakl:   16 bayt (fat pointer!)
    // Box<Aylana>:  8 bayt
    // Box<dyn Sh>:  16 bayt
}

// Object-safe trait — dyn bilan ishlatilishi mumkin
// Object-safe трейт — можно использовать с dyn
//
// Shartlar:
// Условия:
// 1. Self: Sized bo'lmasligi kerak
//    Self: Sized не должно быть
// 2. Metodlarda generic tur parametri bo'lmasligi kerak
//    Методы без параметров обобщённого типа
// 3. Metodlarda Self qaytarilmasligi kerak
//    Методы не должны возвращать Self
// 4. Statik metodlar bo'lmasligi kerak
//    Не должно быть статических методов

// Object-safe
trait Chiqaruvchi {
    fn chiqar(&self, matn: &str);
    fn flush(&mut self);
}

// Object-safe EMAS (generic metod)
// НЕ object-safe (обобщённый метод)
trait GenericTrait {
    fn generik<T>(&self, val: T) -> T;
    // dyn GenericTrait — XATO!
}

struct KonsoleChiqaruvchi;
struct FaylChiqaruvchi { bufer: Vec<String> }

impl Chiqaruvchi for KonsoleChiqaruvchi {
    fn chiqar(&self, matn: &str) { println!("[Console] {}", matn); }
    fn flush(&mut self) { /* no-op */ }
}

impl Chiqaruvchi for FaylChiqaruvchi {
    fn chiqar(&self, matn: &str) { println!("[Fayl] buffered: {}", matn); }
    fn flush(&mut self) {
        println!("[Fayl] flush: {} satr", self.bufer.len());
        self.bufer.clear();
    }
}

fn object_safety_misoli() {

    // dyn Chiqaruvchi — object-safe
    let mut chiqaruvchilar: Vec<Box<dyn Chiqaruvchi>> = vec![
        Box::new(KonsoleChiqaruvchi),
        Box::new(FaylChiqaruvchi { bufer: Vec::new() }),
    ];

    for ch in &mut chiqaruvchilar {
        ch.chiqar("Salom dunyo!");
        ch.flush();
    }
    // [Console] Salom dunyo!
    // [Fayl] buffered: Salom dunyo!
    // [Fayl] flush: 0 satr

    // where Self: Sized — metodni object-safe bo'lmagan tur uchun yashirish
    // where Self: Sized — скрыть метод для non-object-safe типов
    trait MixedTrait {
        fn oddiy_metod(&self) -> String;
        // Bu metod dyn bilan ishlatilmaydi:
        // Этот метод не используется с dyn:
        fn faqat_concrete(&self) -> Self where Self: Sized + Clone {
            self.clone()
        }
    }
    println!("Object safety tushuntirildi ✅");
}

// impl Trait — compile time type erasure, zero-cost
// impl Trait — стирание типа во время компиляции, zero-cost

fn birinchi_musbat(iter: impl Iterator<Item = i32>) -> impl Iterator<Item = i32> {
    iter.filter(|&x| x > 0)
}

fn yaratuvchi(tur: &str) -> Box<dyn Shakl> {
    match tur {
        "aylana" => Box::new(Aylana { radius: 1.0 }),
        "turtburchak" => Box::new(Togritortburchak { eni: 2.0, boyi: 3.0 }),
        _ => Box::new(Aylana { radius: 0.5 }),
    }
}

// impl Trait qaytarish — turli holatlarda
// Возврат impl Trait — в разных случаях
fn hisoblash(katta: bool) -> Box<dyn Fn(f64) -> f64> {
    if katta {
        Box::new(|x| x * x * x)   // kub
    } else {
        Box::new(|x| x * x)        // kvadrat
    }
}

fn impl_trait_misoli() {

    // impl Iterator — static dispatch
    let v = vec![-3, -1, 0, 2, 4, 6, -5, 8];
    let musbatlar: Vec<i32> = birinchi_musbat(v.into_iter()).collect();
    println!("{:?}", musbatlar); // [2, 4, 6, 8]
    // [2, 4, 6, 8]

    // impl Trait vs dyn Trait
    println!("\nimpl Trait vs dyn Trait:");
    println!("impl Trait: monomorphization, tez, binary katta");
    println!("dyn Trait:  runtime dispatch, sekin, binary kichik");

    // Box<dyn Fn> — closure type erasure
    let kvadrat = hisoblash(false);
    let kub = hisoblash(true);
    println!("kvadrat(3) = {}", kvadrat(3.0)); // 9
    println!("kub(3) = {}", kub(3.0));         // 27
    // kvadrat(3) = 9
    // kub(3) = 27

    // Turli shakllar — runtime da
    for tur in &["aylana", "turtburchak", "noma'lum"] {
        let sh = yaratuvchi(tur);
        println!("{}: {:.2}", sh.nomi(), sh.yuzi());
    }
    // Aylana: 3.14
    // To'g'rito'rtburchak: 6.00
    // Aylana: 0.79
}

// Any — erased type ni qaytarish imkoni
// Any — возможность восстановить erased type
trait Voqea: Any + fmt::Debug + Send + Sync {
    fn tur_nomi(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug, Clone)]
struct KirishVoqeasi { foydalanuvchi_id: u64, ip: String }

#[derive(Debug, Clone)]
struct SorovVoqeasi { url: String, metod: String, status: u16 }

#[derive(Debug, Clone)]
struct XatoVoqeasi { kod: i32, xabar: String }

impl Voqea for KirishVoqeasi {
    fn tur_nomi(&self) -> &str { "KirishVoqeasi" }
    fn as_any(&self) -> &dyn Any { self }
}

impl Voqea for SorovVoqeasi {
    fn tur_nomi(&self) -> &str { "SorovVoqeasi" }
    fn as_any(&self) -> &dyn Any { self }
}

impl Voqea for XatoVoqeasi {
    fn tur_nomi(&self) -> &str { "XatoVoqeasi" }
    fn as_any(&self) -> &dyn Any { self }
}

fn any_downcast_misoli() {

    let voqealar: Vec<Box<dyn Voqea>> = vec![
        Box::new(KirishVoqeasi { foydalanuvchi_id: 42, ip: "192.168.1.1".into() }),
        Box::new(SorovVoqeasi { url: "/api/users".into(), metod: "GET".into(), status: 200 }),
        Box::new(XatoVoqeasi { kod: 404, xabar: "Topilmadi".into() }),
        Box::new(KirishVoqeasi { foydalanuvchi_id: 99, ip: "10.0.0.1".into() }),
    ];

    println!("Barcha voqealar:");
    for v in &voqealar {
        println!("  {:?}", v);
    }

    println!("\nDowncast:");
    for v in &voqealar {
        if let Some(kirish) = v.as_any().downcast_ref::<KirishVoqeasi>() {
            println!("  👤 User #{} ({}) kirdi", kirish.foydalanuvchi_id, kirish.ip);
        } else if let Some(sorov) = v.as_any().downcast_ref::<SorovVoqeasi>() {
            println!("  📡 {} {} → {}", sorov.metod, sorov.url, sorov.status);
        } else if let Some(xato) = v.as_any().downcast_ref::<XatoVoqeasi>() {
            println!("  ❌ Xato {}: {}", xato.kod, xato.xabar);
        }
    }
    // 👤 User #42 (192.168.1.1) kirdi
    // 📡 GET /api/users → 200
    // ❌ Xato 404: Topilmadi
    // 👤 User #99 (10.0.0.1) kirdi
}

struct ErasedMap {
    ichki: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl ErasedMap {
    fn new() -> Self { ErasedMap { ichki: HashMap::new() } }

    fn qo_sh<T: Any + Send + Sync>(&mut self, kalit: &str, val: T) {
        self.ichki.insert(kalit.to_string(), Box::new(val));
    }

    fn ol<T: Any>(&self, kalit: &str) -> Option<&T> {
        self.ichki.get(kalit)?.downcast_ref::<T>()
    }

    fn ol_mut<T: Any>(&mut self, kalit: &str) -> Option<&mut T> {
        self.ichki.get_mut(kalit)?.downcast_mut::<T>()
    }

    fn mavjud(&self, kalit: &str) -> bool {
        self.ichki.contains_key(kalit)
    }

    fn uzunlik(&self) -> usize { self.ichki.len() }
}

fn erased_map_misoli() {

    let mut xarita = ErasedMap::new();

    xarita.qo_sh("ism", String::from("Dilshod"));
    xarita.qo_sh("yosh", 22u32);
    xarita.qo_sh("baland", 1.75f64);
    xarita.qo_sh("faol", true);
    xarita.qo_sh("ballar", vec![85u32, 90, 78]);

    println!("Uzunlik: {}", xarita.uzunlik());
    println!("{:?}", xarita.ol::<String>("ism"));
    println!("{:?}", xarita.ol::<u32>("yosh"));
    println!("{:?}", xarita.ol::<f64>("baland"));
    println!("{:?}", xarita.ol::<bool>("faol"));
    println!("{:?}", xarita.ol::<Vec<u32>>("ballar"));
    println!("{:?}", xarita.ol::<i32>("yosh")); // None — noto'g'ri tur
    // Uzunlik: 5
    // Some("Dilshod")
    // Some(22)
    // Some(1.75)
    // Some(true)
    // Some([85, 90, 78])
    // None

    // O'zgartirish
    if let Some(yosh) = xarita.ol_mut::<u32>("yosh") {
        *yosh += 1;
    }
    println!("Yangi yosh: {:?}", xarita.ol::<u32>("yosh")); // Some(23)
    // Some(23)
}

// Plugin tizimi — Box<dyn Trait>
// Система плагинов — Box<dyn Trait>
trait Plugin: fmt::Debug {
    fn ism(&self) -> &str;
    fn versiya(&self) -> &str;
    fn ishga_tushir(&self, konfig: &HashMap<String, String>) -> Result<String, String>;
}

#[derive(Debug)]
struct JsonPlugin;
#[derive(Debug)]
struct LogPlugin;
#[derive(Debug)]
struct CachePlugin { sig_im: usize }

impl Plugin for JsonPlugin {
    fn ism(&self) -> &str { "json" }
    fn versiya(&self) -> &str { "1.0.0" }
    fn ishga_tushir(&self, konfig: &HashMap<String, String>) -> Result<String, String> {
        let chuqurlik = konfig.get("depth").map(|s| s.as_str()).unwrap_or("5");
        Ok(format!("JSON plugin (chuqurlik: {})", chuqurlik))
    }
}

impl Plugin for LogPlugin {
    fn ism(&self) -> &str { "log" }
    fn versiya(&self) -> &str { "2.1.0" }
    fn ishga_tushir(&self, konfig: &HashMap<String, String>) -> Result<String, String> {
        let daraja = konfig.get("level").map(|s| s.as_str()).unwrap_or("info");
        Ok(format!("Log plugin (daraja: {})", daraja))
    }
}

impl Plugin for CachePlugin {
    fn ism(&self) -> &str { "cache" }
    fn versiya(&self) -> &str { "3.0.0" }
    fn ishga_tushir(&self, _konfig: &HashMap<String, String>) -> Result<String, String> {
        Ok(format!("Cache plugin (sig'im: {} KB)", self.sig_im))
    }
}

struct PluginRegistry {
    pluginlar: Vec<Box<dyn Plugin>>,
    konfig: HashMap<String, HashMap<String, String>>,
}

impl PluginRegistry {
    fn new() -> Self {
        PluginRegistry {
            pluginlar: Vec::new(),
            konfig: HashMap::new(),
        }
    }

    fn ro_yxatga_ol(&mut self, plugin: Box<dyn Plugin>) {
        println!("[Registry] '{}' v{} ro'yxatga olindi",
                 plugin.ism(), plugin.versiya());
        self.pluginlar.push(plugin);
    }

    fn konfig_qo_sh(&mut self, plugin_ism: &str, kalit: &str, qiymat: &str) {
        self.konfig
            .entry(plugin_ism.to_string())
            .or_default()
            .insert(kalit.to_string(), qiymat.to_string());
    }

    fn hammani_ishga_tushir(&self) {
        let bosh = HashMap::new();
        for plugin in &self.pluginlar {
            let konfig = self.konfig.get(plugin.ism()).unwrap_or(&bosh);
            match plugin.ishga_tushir(konfig) {
                Ok(xabar) => println!("[OK] {}", xabar),
                Err(e)    => println!("[XATO] {}: {}", plugin.ism(), e),
            }
        }
    }

    fn topish(&self, ism: &str) -> Option<&dyn Plugin> {
        self.pluginlar.iter()
            .find(|p| p.ism() == ism)
            .map(|p| p.as_ref())
    }
}

fn real_hayot_misollari() {

    println!("=== Plugin Tizimi ===");
    let mut registry = PluginRegistry::new();

    registry.ro_yxatga_ol(Box::new(JsonPlugin));
    registry.ro_yxatga_ol(Box::new(LogPlugin));
    registry.ro_yxatga_ol(Box::new(CachePlugin { sig_im: 512 }));

    registry.konfig_qo_sh("json", "depth", "10");
    registry.konfig_qo_sh("log", "level", "debug");

    println!("\nBarcha pluginlar:");
    registry.hammani_ishga_tushir();

    println!("\nPlugin topish:");
    if let Some(p) = registry.topish("log") {
        println!("Topildi: {} v{}", p.ism(), p.versiya());
    }
    // [Registry] 'json' v1.0.0 ro'yxatga olindi
    // ...
    // [OK] JSON plugin (chuqurlik: 10)
    // [OK] Log plugin (daraja: debug)
    // [OK] Cache plugin (sig'im: 512 KB)

    println!("\n=== Voqea Tizimi ===");
    any_downcast_misoli();

    println!("\n=== ErasedMap ===");
    erased_map_misoli();

    println!("\n=== Shakl Arifmetikasi ===");
    dyn_trait_misoli();
}

fn main() {

    println!("=== DYN TRAIT ===");
    dyn_trait_misoli();

    println!("\n=== OBJECT SAFETY ===");
    object_safety_misoli();

    println!("\n=== IMPL TRAIT ===");
    impl_trait_misoli();

    println!("\n=== ANY + DOWNCAST ===");
    any_downcast_misoli();

    println!("\n=== ERASED MAP ===");
    erased_map_misoli();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        DYN TRAIT                                                                                             |
// #================================================================================================================================================#
// # |   1 | &dyn Trait                      | Fat pointer (data + vtable)                | Fat pointer (data + vtable)                             |
// # |   2 | Box<dyn Trait>                  | Heap da erased type                        | Erased type в куче                                      |
// # |   3 | Arc<dyn Trait>                  | Shared erased type                         | Общий erased type                                       |
// # |   4 | dyn Trait o'lcham               | 16 bayt (2 * usize)                        | 16 байт (2 * usize)                                     |
// # |   5 | vtable                          | Virtual function table                     | Таблица виртуальных функций                             |
// #================================================================================================================================================#
// # |                                        OBJECT SAFETY                                                                                         |
// #================================================================================================================================================#
// # |   6 | Object-safe shartlar            | Generic metod yo'q, Self yo'q              | Нет обобщённых методов, нет Self                        |
// # |   7 | where Self: Sized               | Metodni dyn dan yashirish                  | Скрыть метод от dyn                                     |
// # |   8 | Sized: !dyn                     | Sized trait ob'ekt bo'la olmaydi           | Sized трейт не может быть объектом                      |
// #================================================================================================================================================#
// # |                                        IMPL TRAIT                                                                                            |
// #================================================================================================================================================#
// # |   9 | impl Trait argument             | Static dispatch, monomorphization          | Статическая диспетчеризация                             |
// # |  10 | -> impl Trait                   | Compile-time type erasure                  | Стирание типа во время компиляции                       |
// # |  11 | impl vs dyn                     | impl tez+katta, dyn sekin+kichik           | impl быстрый+большой, dyn медленный+малый               |
// #================================================================================================================================================#
// # |                                        ANY VA DOWNCAST                                                                                       |
// #================================================================================================================================================#
// # |  12 | as_any() -> &dyn Any            | Trait → Any konversiya                     | Конверсия Trait → Any                                   |
// # |  13 | downcast_ref::<T>()             | &dyn Any → Option<&T>                      | &dyn Any → Option<&T>                                   |
// # |  14 | downcast_mut::<T>()             | &mut dyn Any → Option<&mut T>              | &mut dyn Any → Option<&mut T>                           |
// # |  15 | TypeId::of::<T>()               | Tur identifikatori                         | Идентификатор типа                                      |
// #================================================================================================================================================#
// # |                                        PATTERNLAR                                                                                            |
// #================================================================================================================================================#
// # |  16 | Plugin tizimi                   | Box<dyn Plugin> ro'yxat                    | Список Box<dyn Plugin>                                  |
// # |  17 | ErasedMap                       | HashMap<String, Box<dyn Any>>              | HashMap<String, Box<dyn Any>>                           |
// # |  18 | Voqea tizimi                    | as_any() + downcast_ref bilan              | С as_any() + downcast_ref                               |
// # |  19 | Shakl arifmetikasi              | Turli shakllar bir ro'yxatda               | Разные фигуры в одном списке                            |
// #================================================================================================================================================#