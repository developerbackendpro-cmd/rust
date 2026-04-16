// #================================================================================================================================================#
// #                                                         TRAITS CHUQUR                                                                          #
// #                     TRAIT CHUQUR — SUPERTRAIT, BLANKET IMPL, DEFAULT METHOD, OBJECT SAFETY, MARKER TRAITS.                                     #
// #                     TRAIT ГЛУБОКО — SUPERTRAIT, BLANKET IMPL, DEFAULT METHOD, OBJECT SAFETY, MARKER TRAITS.                                    #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// Bu mavzuda o'rganiladiganlar:
// Что изучается в этой теме:
//
//   1. Supertrait — trait boshqa traitni talab qiladi
//      Supertrait — трейт требует другой трейт
//   2. Default method — implement kerak emas
//      Default method — не требует реализации
//   3. Blanket impl — generic uchun ommaviy impl
//      Blanket impl — публичный impl для generic
//   4. Object safety — dyn Trait uchun qoidalar
//      Object safety — правила для dyn Trait
//   5. Marker traits — Send, Sync, Copy, Sized
//      Marker traits — Send, Sync, Copy, Sized
//   6. Negative impl — !Trait
//      Negative impl — !Trait
//   7. Trait alias
//      Trait alias

// Supertrait — implement qilish uchun boshqa trait kerak
// Supertrait — для реализации нужен другой трейт
trait Chiqarish: fmt::Display + fmt::Debug {
    fn chiqar(&self) {
        // Display va Debug kafolatlangan
        // Display и Debug гарантированы
        println!("Display: {}", self);
        println!("Debug: {:?}", self);
    }
}

#[derive(Debug)]
struct Son(i32);

impl fmt::Display for Son {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Son: Display + Debug implement qilgan → Chiqarish implement qilish mumkin
// Son: реализовал Display + Debug → можно реализовать Chiqarish
impl Chiqarish for Son {}

// Murakkab supertrait zanjiri
// Сложная цепочка суперТрейтов
trait Asosiy {
    fn asosiy_metod(&self) -> String;
}

trait Kengaytirilgan: Asosiy + Clone + fmt::Debug {
    fn kengaytirilgan_metod(&self) -> String {
        format!("Kengaytirilgan: {}", self.asosiy_metod())
    }
}

trait EnKengaytirilgan: Kengaytirilgan {
    fn en_kengaytirilgan(&self) -> String {
        format!("EnKeng: {}", self.kengaytirilgan_metod())
    }
}

#[derive(Debug, Clone)]
struct Talaba {
    ism: String,
    baho: u32,
}

impl Asosiy for Talaba {
    fn asosiy_metod(&self) -> String {
        format!("{} ({})", self.ism, self.baho)
    }
}

impl Kengaytirilgan for Talaba {}
impl EnKengaytirilgan for Talaba {}

fn supertrait_misollari() {

    let son = Son(42);
    son.chiqar();
    // Display: 42
    // Debug: Son(42)

    let t = Talaba { ism: "Dilshod".to_string(), baho: 90 };
    println!("{}", t.asosiy_metod());
    println!("{}", t.kengaytirilgan_metod());
    println!("{}", t.en_kengaytirilgan());
    // Dilshod (90)
    // Kengaytirilgan: Dilshod (90)
    // EnKeng: Kengaytirilgan: Dilshod (90)
}

trait Shakl {
    // Majburiy metod — implement qilinishi kerak
    // Обязательный метод — должен быть реализован
    fn yuza(&self) -> f64;

    // Default metod — o'zgartirish ixtiyoriy
    // Default метод — изменение опционально
    fn perimetr(&self) -> f64 {
        0.0 // default implementatsiya
    }

    fn tavsif(&self) -> String {
        format!("Yuza: {:.2}, Perimetr: {:.2}", self.yuza(), self.perimetr())
    }

    fn kattami(&self, boshqa: &dyn Shakl) -> bool {
        self.yuza() > boshqa.yuza()
    }
}

struct Doira { radius: f64 }
struct Turtburchak { eni: f64, boyi: f64 }
struct Uchburchak { asos: f64, balandlik: f64 }

impl Shakl for Doira {
    fn yuza(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn perimetr(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Shakl for Turtburchak {
    fn yuza(&self) -> f64 { self.eni * self.boyi }
    fn perimetr(&self) -> f64 { 2.0 * (self.eni + self.boyi) }
}

impl Shakl for Uchburchak {
    fn yuza(&self) -> f64 { 0.5 * self.asos * self.balandlik }
    // perimetr() — default (0.0) ishlatiladi
    // perimetr() — используется default (0.0)
}

fn default_method_misollari() {

    let d = Doira { radius: 5.0 };
    let t = Turtburchak { eni: 4.0, boyi: 3.0 };
    let u = Uchburchak { asos: 6.0, balandlik: 4.0 };

    println!("{}", d.tavsif());
    println!("{}", t.tavsif());
    println!("{}", u.tavsif()); // perimetr = 0.0 (default)
    // Yuza: 78.54, Perimetr: 31.42
    // Yuza: 12.00, Perimetr: 14.00
    // Yuza: 12.00, Perimetr: 0.00

    println!("Doira kattami? {}", d.kattami(&t));
    // Doira kattami? true
}

// Blanket impl — T uchun boshqa trait bo'lsa — avtomatik implement
// Blanket impl — если T реализует другой трейт — автоматическая реализация

trait Xulosa {
    fn xulosa(&self) -> String;
}

// T: Display bo'lsa — Xulosa avtomatik
// Если T: Display — Xulosa автоматически
impl<T: fmt::Display> Xulosa for T {
    fn xulosa(&self) -> String {
        format!("Xulosa: {}", self)
    }
}

// std da mashhur blanket impllar:
// Известные blanket impl в std:
//   impl<T: Clone> ToOwned for T { ... }
//   impl<T: Display> ToString for T { ... }
//   impl<T: Iterator> IntoIterator for T { ... }

fn blanket_impl_misollari() {

    // Barcha Display turlar uchun Xulosa ishlaydi
    // Xulosa работает для всех типов Display
    println!("{}", 42.xulosa());
    println!("{}", "salom".xulosa());
    println!("{}", 3.14f64.xulosa());
    println!("{}", true.xulosa());
    // Xulosa: 42
    // Xulosa: salom
    // Xulosa: 3.14
    // Xulosa: true

    // Custom struct — Display implement qilsa Xulosa bepul
    // Custom struct — реализует Display → Xulosa бесплатно
    struct Mahsulot { nomi: String, narx: f64 }
    impl fmt::Display for Mahsulot {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} ({})", self.nomi, self.narx)
        }
    }
    let m = Mahsulot { nomi: "Olma".to_string(), narx: 1500.0 };
    println!("{}", m.xulosa());
    // Xulosa: Olma (1500)
}

// Object-safe trait qoidalari:
// Правила object-safe трейта:
//   ✅ fn metod(&self) → ishlatish mumkin
//   ✅ fn metod(&mut self) → ishlatish mumkin
//   ✅ fn metod(&self, x: i32) → ishlatish mumkin
//   ❌ fn metod<T>(&self) → generic — OBJECT SAFE EMAS
//   ❌ fn metod() → self yo'q — OBJECT SAFE EMAS
//   ❌ Sized talab qiluvchi — OBJECT SAFE EMAS
//   ❌ Self qaytaruvchi metod — OBJECT SAFE EMAS (ba'zi hollar)

// Object-safe trait
// Object-safe трейт
trait ObjectSafe {
    fn metod(&self) -> String;
    fn boshqa(&self, x: i32) -> i32;
}

// Object-safe emas (generic metod)
// Не object-safe (generic метод)
trait ObjectSafeEmas {
    fn generic_metod<T>(&self, x: T) -> T where T: Clone;
    // dyn ObjectSafeEmas — bu trait uchun kompile bo'lmaydi
    // dyn ObjectSafeEmas — не скомпилируется для этого трейта
}

struct A; struct B;
impl ObjectSafe for A {
    fn metod(&self) -> String { "A".to_string() }
    fn boshqa(&self, x: i32) -> i32 { x + 1 }
}
impl ObjectSafe for B {
    fn metod(&self) -> String { "B".to_string() }
    fn boshqa(&self, x: i32) -> i32 { x * 2 }
}

fn object_safety_misollari() {

    // dyn Trait — turli turlarni bir xil muomala
    // dyn Trait — одинаковое обращение с разными типами
    let shakllar: Vec<Box<dyn ObjectSafe>> = vec![Box::new(A), Box::new(B)];
    for s in &shakllar {
        println!("{}: {}", s.metod(), s.boshqa(5));
    }
    // A: 6
    // B: 10

    // where Self: Sized — object-unsafe metodlarni chiqarish
    // where Self: Sized — исключение object-unsafe методов
    trait MixedTrait {
        fn object_safe(&self) -> String;

        // where Self: Sized — dyn uchun mavjud emas, lekin generik uchun bor
        // where Self: Sized — недоступно для dyn, но есть для generic
        fn faqat_sized(&self) -> String where Self: Sized {
            "Faqat sized uchun".to_string()
        }
    }
}

// Marker trait — hech qanday metod yo'q, faqat belgi
// Marker trait — нет методов, только метка

// Custom marker trait
// Пользовательский marker трейт
trait Serializable {}
trait Cacheable {}
trait Auditable {}

#[derive(Debug)]
struct Foydalanuvchi { id: u32, ism: String }

// Marker implement — hech narsa yozish shart emas
// Реализация marker — ничего не нужно писать
impl Serializable for Foydalanuvchi {}
impl Cacheable for Foydalanuvchi {}

#[derive(Debug)]
struct Log { xabar: String }
impl Serializable for Log {}
impl Auditable for Log {}

// Marker trait bilan generic constraint
// Generic ограничение с marker трейтом
fn serialga_yoz<T: Serializable + fmt::Debug>(qiymat: &T) {
    println!("Serial: {:?}", qiymat);
}

fn keshga_saqlash<T: Cacheable + fmt::Debug>(qiymat: &T) {
    println!("Kesh: {:?}", qiymat);
}

fn marker_trait_misollari() {

    let f = Foydalanuvchi { id: 1, ism: "Dilshod".to_string() };
    let l = Log { xabar: "Kirdi".to_string() };

    serialga_yoz(&f);  // Serializable
    serialga_yoz(&l);  // Serializable
    keshga_saqlash(&f); // Cacheable
    // keshga_saqlash(&l); // ← xato: Log Cacheable emas
    // Serial: Foydalanuvchi { id: 1, ism: "Dilshod" }
    // Serial: Log { xabar: "Kirdi" }
    // Kesh: Foydalanuvchi { id: 1, ism: "Dilshod" }
}

// Repository pattern — trait bilan
// Паттерн Repository — с трейтом
trait Repository<T, Id> {
    fn topish(&self, id: Id) -> Option<&T>;
    fn saqlash(&mut self, element: T) -> Id;
    fn o_chirish(&mut self, id: Id) -> Option<T>;
    fn barchasi(&self) -> Vec<&T>;

    // Default metod
    fn soni(&self) -> usize {
        self.barchasi().len()
    }

    fn mavjudmi(&self, id: Id) -> bool where Id: Clone {
        self.topish(id).is_some()
    }
}

struct XotiraRepository<T> {
    ma_lumotlar: std::collections::HashMap<u64, T>,
    keyingi_id: u64,
}

impl<T> XotiraRepository<T> {
    fn new() -> Self {
        XotiraRepository {
            ma_lumotlar: std::collections::HashMap::new(),
            keyingi_id: 1,
        }
    }
}

impl<T: Clone> Repository<T, u64> for XotiraRepository<T> {
    fn topish(&self, id: u64) -> Option<&T> {
        self.ma_lumotlar.get(&id)
    }

    fn saqlash(&mut self, element: T) -> u64 {
        let id = self.keyingi_id;
        self.ma_lumotlar.insert(id, element);
        self.keyingi_id += 1;
        id
    }

    fn o_chirish(&mut self, id: u64) -> Option<T> {
        self.ma_lumotlar.remove(&id)
    }

    fn barchasi(&self) -> Vec<&T> {
        self.ma_lumotlar.values().collect()
    }
}

fn real_hayot_misollari() {

    #[derive(Debug, Clone)]
    struct Mahsulot { nomi: String, narx: f64 }

    let mut repo: XotiraRepository<Mahsulot> = XotiraRepository::new();

    let id1 = repo.saqlash(Mahsulot { nomi: "Olma".to_string(), narx: 1500.0 });
    let id2 = repo.saqlash(Mahsulot { nomi: "Banan".to_string(), narx: 3000.0 });
    let id3 = repo.saqlash(Mahsulot { nomi: "Anor".to_string(), narx: 2500.0 });

    println!("Soni: {}", repo.soni());
    println!("{:?}", repo.topish(id1));
    println!("Mavjud: {}", repo.mavjudmi(id2));
    println!("Mavjud: {}", repo.mavjudmi(999));
    // Soni: 3
    // Some(Mahsulot { nomi: "Olma", narx: 1500.0 })
    // Mavjud: true
    // Mavjud: false

    let o_chirildi = repo.o_chirish(id3);
    println!("O'chirildi: {:?}", o_chirildi);
    println!("Soni: {}", repo.soni());
    // O'chirildi: Some(Mahsulot { nomi: "Anor", narx: 2500.0 })
    // Soni: 2
}

fn main() {

    println!("=== SUPERTRAIT ===");
    supertrait_misollari();

    println!("\n=== DEFAULT METHOD ===");
    default_method_misollari();

    println!("\n=== BLANKET IMPL ===");
    blanket_impl_misollari();

    println!("\n=== OBJECT SAFETY ===");
    object_safety_misollari();

    println!("\n=== MARKER TRAITS ===");
    marker_trait_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                | Tavsif (UZ)                                | Описание (RU)                                               |
// #================================================================================================================================================#
// # |   1 | trait A: B + C              | Supertrait — B va C kerak                  | Суперт рейт — нужны B и C                                   |
// # |   2 | fn metod(&self) { ... }     | Default metod — o'zgartirish ixtiyoriy     | Default метод — изменение опционально                       |
// # |   3 | impl<T: Trait> X for T      | Blanket impl — T uchun ommaviy impl        | Blanket impl — публичный impl для T                         |
// # |   4 | dyn Trait                   | Object safety — generic metod bo'lmasin    | Object safety — нет generic метода                          |
// # |   5 | where Self: Sized           | Object-unsafe metodlarni chiqarish         | Исключение object-unsafe методов                            |
// # |   6 | trait Marker {}             | Marker trait — hech qanday metod yo'q      | Marker трейт — нет методов                                  |
// # |   7 | fn f<T: MarkerA + MarkerB>  | Marker bilan constraint                    | Ограничение с Marker                                        |
// # |   8 | Fn: FnMut: FnOnce           | Ierarxiya — supertrait zanjiri             | Иерархия — цепочка суперт рейтов                            |
// # |   9 | Copy: Clone                 | Copy — Clone ni talab qiladi               | Copy — требует Clone                                        |
// # |  10 | Eq: PartialEq               | Eq — PartialEq ni talab qiladi             | Eq — требует PartialEq                                      |
// #================================================================================================================================================#