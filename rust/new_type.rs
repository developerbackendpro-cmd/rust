// #================================================================================================================================================#
// #                                                                NEWTYPE PATTERN                                                                 #
// #                    NEWTYPE — TUR XAVFSIZLIGI, ABSTRAKTSIYA, ORPHAN RULE YECHIM. ZERO-COST WRAPPER. DEREF, DISPLAY.                             #
// #                    NEWTYPE — ТИПОБЕЗОПАСНОСТЬ, АБСТРАКЦИЯ, РЕШЕНИЕ ORPHAN RULE. ZERO-COST ОБЁРТКА. DEREF, DISPLAY.                             #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign};
use std::cmp::Ordering;
use std::collections::HashMap;

// Newtype Pattern nima:
// Что такое Newtype Pattern:
//
//   Mavjud turni yangi turdagi struct ga o'rash
//   Обёртка существующего типа в новую структуру
//   struct Yangi(EskiTur);
//
//   Nima uchun:
//   Зачем:
//   1. Tur xavfsizligi — aralashmaslik (metр vs sekund)
//      Типобезопасность — не перепутать
//   2. Orphan rule yechim — tashqi tur + tashqi trait
//      Решение Orphan rule — внешний тип + внешний трейт
//   3. Abstraktsiya — ichki implementatsiyani yashirish
//      Абстракция — скрытие внутренней реализации
//   4. API dizayn — yangi semantika berish
//      Дизайн API — придание новой семантики
//   5. Zero-cost — runtime qo'shimcha xarajat yo'q
//      Zero-cost — нет дополнительных затрат в runtime

// O'lchovlar — aralashmaslik
// Единицы измерения — не перепутать
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Metr(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Kilogramm(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Sekund(f64);

impl Metr {
    fn new(val: f64) -> Self { Metr(val) }
    fn qiymat(&self) -> f64 { self.0 }
}

impl Kilogramm {
    fn new(val: f64) -> Self { Kilogramm(val) }
    fn qiymat(&self) -> f64 { self.0 }
}

impl Sekund {
    fn new(val: f64) -> Self { Sekund(val) }
    fn qiymat(&self) -> f64 { self.0 }
}

impl fmt::Display for Metr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} m", self.0)
    }
}

impl fmt::Display for Kilogramm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} kg", self.0)
    }
}

impl fmt::Display for Sekund {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} s", self.0)
    }
}

impl Add for Metr {
    type Output = Self;
    fn add(self, b: Self) -> Self { Metr(self.0 + b.0) }
}

impl Sub for Metr {
    type Output = Self;
    fn sub(self, b: Self) -> Self { Metr(self.0 - b.0) }
}

impl Mul<f64> for Metr {
    type Output = Self;
    fn mul(self, k: f64) -> Self { Metr(self.0 * k) }
}

fn tur_xavfsizligi_misoli() {

    let uzunlik: Metr = Metr::new(5.0);
    let kenglik: Metr = Metr::new(3.0);
    let massa: Kilogramm = Kilogramm::new(70.0);
    let vaqt: Sekund = Sekund::new(9.58);

    println!("Uzunlik: {}", uzunlik);
    println!("Kenglik: {}", kenglik);
    println!("Massa:   {}", massa);
    println!("Vaqt:    {}", vaqt);
    // Uzunlik: 5.00 m
    // Kenglik: 3.00 m
    // Massa:   70.00 kg
    // Vaqt:    9.58 s

    let jami = uzunlik + kenglik;
    println!("Jami: {}", jami);         // 8.00 m

    let ikkilangan = uzunlik * 2.0;
    println!("Ikkilangan: {}", ikkilangan); // 10.00 m

    // Bu KOMPILE BO'LMAYDI — tur xavfsizligi!
    // Это НЕ СКОМПИЛИРУЕТСЯ — типобезопасность!
    // let xato = uzunlik + massa; // ← Metr + Kilogramm = XATO!
    println!("Tur xavfsizligi kafolatlangan ✅");

    // Tezlik hisoblash
    let masofa = Metr::new(100.0);
    let t = Sekund::new(9.58);
    let tezlik = masofa.qiymat() / t.qiymat(); // m/s — alohida tür yoki f64
    println!("Tezlik: {:.2} m/s", tezlik);
    // Tezlik: 10.44 m/s
}

// Orphan rule: tashqi tur uchun tashqi trait implement qilib bo'lmaydi
// Orphan rule: нельзя реализовать внешний трейт для внешнего типа
//
// Vec<i32> uchun Display implement qilib bo'lmaydi
// Нельзя реализовать Display для Vec<i32>
// Yechim: Newtype!

struct KozatorVec(Vec<i32>);

impl fmt::Display for KozatorVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, v) in self.0.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

impl fmt::Debug for KozatorVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KozatorVec{}", self)
    }
}

impl std::ops::Deref for KozatorVec {
    type Target = Vec<i32>;
    fn deref(&self) -> &Vec<i32> { &self.0 }
}

// HashMap uchun Display
// Display для HashMap
struct KozatorMap(HashMap<String, i32>);

impl fmt::Display for KozatorMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        let mut juftlar: Vec<_> = self.0.iter().collect();
        juftlar.sort_by_key(|(k, _)| k.as_str());
        for (i, (k, v)) in juftlar.iter().enumerate() {
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", k, v)?;
        }
        write!(f, "}}")
    }
}

fn orphan_rule_misoli() {

    let v = KozatorVec(vec![1, 2, 3, 4, 5]);
    println!("{}", v);      // [1, 2, 3, 4, 5]
    println!("{:?}", v);    // KozatorVec[1, 2, 3, 4, 5]
    println!("uzunlik: {}", v.len()); // Deref → Vec metodlari
    // [1, 2, 3, 4, 5]
    // KozatorVec[1, 2, 3, 4, 5]
    // uzunlik: 5

    let mut m = KozatorMap(HashMap::new());
    m.0.insert("banan".to_string(), 3);
    m.0.insert("olma".to_string(), 5);
    m.0.insert("anor".to_string(), 2);
    println!("{}", m);
    // {anor: 2, banan: 3, olma: 5}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct FoydalanuvchiId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct MahsulotId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct BuyurtmaId(u64);

impl FoydalanuvchiId {
    fn new(id: u64) -> Self { FoydalanuvchiId(id) }
    fn qiymat(&self) -> u64 { self.0 }
}

impl MahsulotId {
    fn new(id: u64) -> Self { MahsulotId(id) }
}

impl BuyurtmaId {
    fn new(id: u64) -> Self { BuyurtmaId(id) }
}

impl fmt::Display for FoydalanuvchiId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "U#{:06}", self.0)
    }
}

impl fmt::Display for MahsulotId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "P#{:06}", self.0)
    }
}

impl fmt::Display for BuyurtmaId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "O#{:06}", self.0)
    }
}

#[derive(Debug)]
struct Buyurtma {
    id: BuyurtmaId,
    foydalanuvchi: FoydalanuvchiId,
    mahsulotlar: Vec<MahsulotId>,
}

impl Buyurtma {
    fn new(id: BuyurtmaId, foydalanuvchi: FoydalanuvchiId, mahsulotlar: Vec<MahsulotId>) -> Self {
        Buyurtma { id, foydalanuvchi, mahsulotlar }
    }
}

fn id_tizimi_misoli() {

    let f_id = FoydalanuvchiId::new(1001);
    let m_id1 = MahsulotId::new(2001);
    let m_id2 = MahsulotId::new(2002);
    let b_id = BuyurtmaId::new(3001);

    println!("Foydalanuvchi: {}", f_id); // U#001001
    println!("Mahsulot:      {}", m_id1); // P#002001
    println!("Buyurtma:      {}", b_id);  // O#003001

    let buyurtma = Buyurtma::new(b_id, f_id, vec![m_id1, m_id2]);
    println!("{:?}", buyurtma);

    // Bu KOMPILE BO'LMAYDI — ID lar aralashmaydi!
    // Это НЕ СКОМПИЛИРУЕТСЯ — ID не перепутать!
    // let xato: FoydalanuvchiId = MahsulotId::new(1); // ← XATO!
    // if f_id == m_id1 { } // ← XATO! turli turlar

    // HashMap da xavfsiz ishlatish
    let mut f_xarita: HashMap<FoydalanuvchiId, String> = HashMap::new();
    f_xarita.insert(f_id, "Dilshod".to_string());

    let mut m_xarita: HashMap<MahsulotId, String> = HashMap::new();
    m_xarita.insert(m_id1, "Olma".to_string());

    println!("{:?}", f_xarita.get(&f_id));  // Some("Dilshod")
    println!("{:?}", m_xarita.get(&m_id1)); // Some("Olma")
    // Some("Dilshod")
    // Some("Olma")
}

// Email — validatsiya bilan
// Email — с валидацией
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Email(String);

impl Email {
    fn new(s: &str) -> Result<Self, String> {
        let s = s.trim().to_lowercase();
        if s.contains('@') && s.contains('.') && s.len() >= 5 {
            Ok(Email(s))
        } else {
            Err(format!("'{}' — yaroqsiz email", s))
        }
    }

    fn qiymat(&self) -> &str { &self.0 }

    fn domen(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }

    fn foydalanuvchi(&self) -> &str {
        self.0.split('@').next().unwrap_or("")
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Parol — xavfsiz saqlash
// Пароль — безопасное хранение
#[derive(Clone)]
struct Parol(String);

impl Parol {
    fn new(s: &str) -> Result<Self, String> {
        if s.len() < 8 {
            return Err("Parol kamida 8 ta belgi bo'lishi kerak".to_string());
        }
        if !s.chars().any(|c| c.is_uppercase()) {
            return Err("Parolda katta harf bo'lishi kerak".to_string());
        }
        if !s.chars().any(|c| c.is_numeric()) {
            return Err("Parolda raqam bo'lishi kerak".to_string());
        }
        Ok(Parol(s.to_string()))
    }

    fn tekshir(&self, kiritilgan: &str) -> bool {
        self.0 == kiritilgan
    }
}

// Debug — parolni ko'rsatmaydi
// Debug — не показывает пароль
impl fmt::Debug for Parol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parol(***)")
    }
}

impl fmt::Display for Parol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "***")
    }
}

// Musbat son — manfiy bo'lmasligi kafolat
// Положительное число — гарантия неотрицательности
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct MusbatSon(f64);

impl MusbatSon {
    fn new(val: f64) -> Result<Self, String> {
        if val > 0.0 {
            Ok(MusbatSon(val))
        } else {
            Err(format!("{} musbat emas!", val))
        }
    }

    fn qiymat(&self) -> f64 { self.0 }
}

fn abstraktsiya_misoli() {

    // Email validatsiya
    match Email::new("dilshod@rust.uz") {
        Ok(e) => {
            println!("Email: {}", e);
            println!("Foydalanuvchi: {}", e.foydalanuvchi());
            println!("Domen: {}", e.domen());
        }
        Err(e) => println!("Xato: {}", e),
    }
    // Email: dilshod@rust.uz
    // Foydalanuvchi: dilshod
    // Domen: rust.uz

    match Email::new("yaroqsiz_email") {
        Ok(e) => println!("{}", e),
        Err(e) => println!("Xato: {}", e),
    }
    // Xato: 'yaroqsiz_email' — yaroqsiz email

    // Parol
    match Parol::new("Rust2024!") {
        Ok(p) => {
            println!("Parol: {}", p);         // ***
            println!("Debug: {:?}", p);       // Parol(***)
            println!("To'g'ri: {}", p.tekshir("Rust2024!"));  // true
            println!("Xato:   {}", p.tekshir("xato_parol")); // false
        }
        Err(e) => println!("Parol xato: {}", e),
    }
    // Parol: ***
    // Debug: Parol(***)
    // To'g'ri: true
    // Xato: false

    // MusbatSon
    println!("{:?}", MusbatSon::new(5.0));   // Ok(MusbatSon(5.0))
    println!("{:?}", MusbatSon::new(-3.0));  // Err("-3 musbat emas!")
    println!("{:?}", MusbatSon::new(0.0));   // Err("0 musbat emas!")
    // Ok(MusbatSon(5.0))
    // Err("-3 musbat emas!")
    // Err("0 musbat emas!")
}

// Deref — ichki metodlarga kirish
// Deref — доступ к методам внутреннего типа
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug)]
struct KengaytirilganVec<T> {
    ichki: Vec<T>,
    operatsiyalar: usize,
}

impl<T> KengaytirilganVec<T> {
    fn new() -> Self {
        KengaytirilganVec { ichki: Vec::new(), operatsiyalar: 0 }
    }

    fn qo_sh(&mut self, val: T) {
        self.operatsiyalar += 1;
        self.ichki.push(val);
    }

    fn operatsiyalar_soni(&self) -> usize { self.operatsiyalar }
}

impl<T> Deref for KengaytirilganVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> { &self.ichki }
}

impl<T> DerefMut for KengaytirilganVec<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> { &mut self.ichki }
}

fn deref_misoli() {

    let mut v: KengaytirilganVec<i32> = KengaytirilganVec::new();
    v.qo_sh(1);
    v.qo_sh(2);
    v.qo_sh(3);

    // Deref — Vec metodlari to'g'ridan
    // Deref — методы Vec напрямую
    println!("uzunlik: {}", v.len());          // 3
    println!("birinchi: {:?}", v.first());     // Some(1)
    println!("contains(2): {}", v.contains(&2)); // true

    // Iteratsiya — Deref orqali
    let yig: i32 = v.iter().sum();
    println!("yig'indi: {}", yig); // 6

    // O'z metodlari
    println!("operatsiyalar: {}", v.operatsiyalar_soni()); // 3
    // uzunlik: 3
    // birinchi: Some(1)
    // contains(2): true
    // yig'indi: 6
    // operatsiyalar: 3
}

// Pul miqdori — float xatolaridan himoya
// Денежная сумма — защита от ошибок float
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Tiyin(i64); // Tiyinlarda saqlash — float xatosi yo'q

impl Tiyin {
    fn new(tiyin: i64) -> Self { Tiyin(tiyin) }

    fn so_mdan(som: f64) -> Self {
        Tiyin((som * 100.0).round() as i64)
    }

    fn so_mga(&self) -> f64 { self.0 as f64 / 100.0 }
    fn tiyin(&self) -> i64 { self.0 }
}

impl fmt::Display for Tiyin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let so_m = self.0 / 100;
        let tiyin = (self.0 % 100).abs();
        write!(f, "{}.{:02} so'm", so_m, tiyin)
    }
}

impl Add for Tiyin {
    type Output = Self;
    fn add(self, b: Self) -> Self { Tiyin(self.0 + b.0) }
}

impl Sub for Tiyin {
    type Output = Self;
    fn sub(self, b: Self) -> Self { Tiyin(self.0 - b.0) }
}

impl Mul<i64> for Tiyin {
    type Output = Self;
    fn mul(self, k: i64) -> Self { Tiyin(self.0 * k) }
}

// Foiz — 0..100 orasida kafolat
// Процент — гарантия 0..100
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Foiz(f64);

impl Foiz {
    fn new(val: f64) -> Result<Self, String> {
        if (0.0..=100.0).contains(&val) {
            Ok(Foiz(val))
        } else {
            Err(format!("{} foiz 0-100 orasida bo'lishi kerak", val))
        }
    }

    fn qiymat(&self) -> f64 { self.0 }

    fn miqdorni_hisoblash(&self, jami: Tiyin) -> Tiyin {
        Tiyin((jami.0 as f64 * self.0 / 100.0).round() as i64)
    }
}

impl fmt::Display for Foiz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

fn real_hayot_misollari() {

    println!("--- Tiyin (Pul miqdori) ---");
    let narx1 = Tiyin::so_mdan(15000.50);
    let narx2 = Tiyin::so_mdan(8999.99);
    let jami = narx1 + narx2;
    println!("Narx1: {}", narx1);    // 15000.50 so'm
    println!("Narx2: {}", narx2);    // 8999.99 so'm
    println!("Jami:  {}", jami);     // 24000.49 so'm
    println!("Taqqoslash: {}", narx1 > narx2); // true

    let chegirma_summa = Tiyin::so_mdan(5000.0);
    let chegirmadan_keyin = jami - chegirma_summa;
    println!("Chegirmadan keyin: {}", chegirmadan_keyin);
    // 15000.50 so'm
    // 8999.99 so'm
    // 24000.49 so'm
    // Taqqoslash: true
    // Chegirmadan keyin: 19000.49 so'm

    println!("\n--- Foiz ---");
    let qqs = Foiz::new(12.0).unwrap();     // QQS 12%
    let chegirma = Foiz::new(15.0).unwrap(); // 15% chegirma

    println!("QQS: {}", qqs);
    println!("Chegirma: {}", chegirma);

    let mahsulot_narxi = Tiyin::so_mdan(100000.0);
    let qqs_miqdori = qqs.miqdorni_hisoblash(mahsulot_narxi);
    let chegirma_miqdori = chegirma.miqdorni_hisoblash(mahsulot_narxi);

    println!("Mahsulot: {}", mahsulot_narxi);
    println!("QQS ({qqs}): {}", qqs_miqdori);
    println!("Chegirma ({chegirma}): {}", chegirma_miqdori);
    println!("Jami (QQS bilan, chegirmasi bilan): {}",
             (mahsulot_narxi + qqs_miqdori) - chegirma_miqdori);
    // Mahsulot: 100000.00 so'm
    // QQS (12%): 12000.00 so'm
    // Chegirma (15%): 15000.00 so'm
    // Jami ...: 97000.00 so'm

    // Xatoli foiz
    println!("\n{:?}", Foiz::new(101.0)); // Err(...)
    println!("{:?}", Foiz::new(-5.0));  // Err(...)
    // Err("101 foiz 0-100 orasida bo'lishi kerak")
    // Err("-5 foiz 0-100 orasida bo'lishi kerak")
}

fn main() {

    println!("=== TUR XAVFSIZLIGI ===");
    tur_xavfsizligi_misoli();

    println!("\n=== ORPHAN RULE ===");
    orphan_rule_misoli();

    println!("\n=== ID TIZIMI ===");
    id_tizimi_misoli();

    println!("\n=== ABSTRAKTSIYA ===");
    abstraktsiya_misoli();

    println!("\n=== DEREF ===");
    deref_misoli();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                               | Описание (RU)                                            |
// #================================================================================================================================================#
// # |                                        NEWTYPE ASOSLARI                                                                                      |
// #================================================================================================================================================#
// # |   1 | struct Yangi(EskiTur)           | Zero-cost wrapper                          | Zero-cost обёртка                                       |
// # |   2 | .0 — ichki qiymat               | Ichki qiymatga kirish                      | Доступ к внутреннему значению                           |
// # |   3 | size_of<Yangi> == size_of<Eski> | Runtime xarajat yo'q                       | Нет затрат в runtime                                    |
// #================================================================================================================================================#
// # |                                        NIMA UCHUN KERAK                                                                                      |
// #================================================================================================================================================#
// # |   4 | Tur xavfsizligi                 | Metr + Sekund = KOMPILE XATO                           | Metr + Sekund = ОШИБКА КОМПИЛЯЦИИ           |
// # |   5 | Orphan rule yechim              | Vec<T> uchun Display                                   | Display для Vec<T>                          |
// # |   6 | Abstraktsiya                    | Validatsiya, ichki implementatsiya yashirish           | Валидация, скрытие реализации               |
// # |   7 | API dizayn                      | Email, Parol, MusbatSon                                | Email, Пароль, ПоложительноеЧисло           |
// # |   8 | Deref/DerefMut                  | Ichki tur metodlariga kirish                           | Доступ к методам внутреннего типа           |
// # |   9 | ID tizimi                       | FoydalanuvchiId vs MahsulotId                          | FoydalanuvchiId vs MahsulotId               |
// # |  10 | Pul miqdori                     | Float xatolaridan himoya (i64 tiyin)                   | Защита от ошибок float (i64 копеек)         |
// #================================================================================================================================================#