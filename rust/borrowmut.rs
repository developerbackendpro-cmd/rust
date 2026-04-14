// #================================================================================================================================================#
// #                                                            BORROW  |  BORROWMUT                                                                #
// #                            BORROW — HASH VA TAQQOSLASH UCHUN REFERENCE OLISH. ASREF DAN FARQI: EQ+HASH KAFOLATI.                               #
// #                            BORROW — ПОЛУЧЕНИЕ ССЫЛКИ ДЛЯ HASH И СРАВНЕНИЯ. ОТЛИЧИЕ ОТ ASREF: ГАРАНТИЯ EQ+HASH.                                 #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::hash::Hash;

// AsRef  — faqat reference olish, Hash/Eq kafolat yo'q
// AsRef  — только получение ссылки, нет гарантии Hash/Eq
// Borrow — Hash, Eq, Ord bir xil bo'lishi KAFOLATLANGAN
// Borrow — Hash, Eq, Ord ГАРАНТИРОВАННО одинаковы

// Qoida:
// Правило:
//   if x.borrow() == y.borrow() → x == y bo'lishi SHART
//   hash(x.borrow()) == hash(x) bo'lishi SHART
//
// HashMap va HashSet faqat Borrow ishlatadi, AsRef emas!
// HashMap и HashSet используют только Borrow, не AsRef!

// String implements Borrow<str>   → HashMap<String, V> ga &str bilan kirish imkoni
// String implements Borrow<str>   → доступ к HashMap<String, V> через &str
// Vec<T> implements Borrow<[T]>   → HashMap<Vec<T>, V> ga &[T] bilan kirish
// Vec<T> implements Borrow<[T]>   → доступ к HashMap<Vec<T>, V> через &[T]

fn built_in_borrow_misollari() {
    // String → Borrow<str>
    // String → Borrow<str>
    let string: String = String::from("salom");
    let str_borrow: &str = string.borrow();
    println!("{}", str_borrow);
    // salom

    // Vec → Borrow<[T]>
    // Vec → Borrow<[T]>
    let vektor: Vec<i32> = vec![1, 2, 3];
    let slice_borrow: &[i32] = vektor.borrow();
    println!("{:?}", slice_borrow);
    // [1, 2, 3]

    // i32 → Borrow<i32>
    // i32 → Borrow<i32>
    let son: i32 = 42;
    let son_borrow: &i32 = son.borrow();
    println!("{}", son_borrow);
    // 42

    // &T → Borrow<T>
    // &T → Borrow<T>
    let reference: &i32 = &42;
    let borrow_ref: &i32 = reference;
    println!("{}", borrow_ref);
    // 42

    // Box<T> → Borrow<T>
    // Box<T> → Borrow<T>
    let boxed: Box<i32> = Box::new(99);
    let boxed_borrow: &i32 = boxed.borrow();
    println!("{}", boxed_borrow);
    // 99
}

fn hashmap_borrow_misollari() {
    let mut xarita: HashMap<String, i32> = HashMap::new();
    xarita.insert(String::from("salom"), 1);
    xarita.insert(String::from("dunyo"), 2);
    xarita.insert(String::from("rust"), 3);

    // String kalit — &str bilan qidirish (Borrow orqali)
    // Ключ String — поиск через &str (через Borrow)
    let qidirilgan1 = xarita.get("salom");
    let qidirilgan2 = xarita.get("rust");
    let qidirilgan3 = xarita.get("yo_q");
    println!("{:?}", qidirilgan1);
    println!("{:?}", qidirilgan2);
    println!("{:?}", qidirilgan3);
    // Some(1)
    // Some(3)
    // None

    // contains_key — &str bilan String kalitni tekshirish
    // contains_key — проверка String ключа через &str
    let bormi: bool = xarita.contains_key("dunyo");
    println!("{}", bormi);
    // true

    // remove — &str bilan o'chirish
    // remove — удаление через &str
    let olingan = xarita.remove("salom");
    println!("{:?}", olingan);
    println!("{}", xarita.len());
    // Some(1)
    // 2
}

fn hashset_borrow_misollari() {
    let mut toplam: HashSet<String> = HashSet::new();
    toplam.insert(String::from("olma"));
    toplam.insert(String::from("nok"));
    toplam.insert(String::from("uzum"));

    // contains — &str bilan tekshirish
    // contains — проверка через &str
    let bor1: bool = toplam.contains("olma");
    let bor2: bool = toplam.contains("banan");
    println!("{}", bor1);
    println!("{}", bor2);
    // true
    // false

    // get — &str bilan qidirish
    // get — поиск через &str
    let topilgan = toplam.get("nok");
    println!("{:?}", topilgan);
    // Some("nok")

    // remove — &str bilan o'chirish
    // remove — удаление через &str
    let ochirildi: bool = toplam.remove("uzum");
    println!("{}", ochirildi);
    // true
}

fn vec_kalit_hashmap_misoli() {
    let mut xarita: HashMap<Vec<i32>, &str> = HashMap::new();
    xarita.insert(vec![1, 2, 3], "birinchi");
    xarita.insert(vec![4, 5, 6], "ikkinchi");

    // Vec kalit — &[i32] bilan qidirish (Borrow orqali)
    // Ключ Vec — поиск через &[i32] (через Borrow)
    let slice_kalit: &[i32] = &[1, 2, 3];
    let topilgan = xarita.get(slice_kalit);
    println!("{:?}", topilgan);
    // Some("birinchi")

    let slice_kalit2: &[i32] = &[4, 5, 6];
    let topilgan2 = xarita.get(slice_kalit2);
    println!("{:?}", topilgan2);
    // Some("ikkinchi")
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Identifikator {
    qiymat: String,
}

impl Identifikator {
    fn new(qiymat: &str) -> Self {
        Identifikator { qiymat: qiymat.to_string() }
    }
}

// Identifikator dan &str olish — Hash va Eq bir xil
// получение &str из Identifikator — Hash и Eq одинаковы
impl Borrow<str> for Identifikator {
    fn borrow(&self) -> &str {
        &self.qiymat
    }
}

impl fmt::Display for Identifikator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.qiymat)
    }
}

fn custom_borrow_misoli() {
    let mut xarita: HashMap<Identifikator, u32> = HashMap::new();
    xarita.insert(Identifikator::new("foydalanuvchi_1"), 100);
    xarita.insert(Identifikator::new("foydalanuvchi_2"), 200);
    xarita.insert(Identifikator::new("foydalanuvchi_3"), 300);

    // &str bilan qidirish
    // поиск через &str
    let topilgan1 = xarita.get("foydalanuvchi_1");
    let topilgan2 = xarita.get("foydalanuvchi_3");
    let yoq = xarita.get("mavjud_emas");

    println!("{:?}", topilgan1);
    println!("{:?}", topilgan2);
    println!("{:?}", yoq);
    // Some(100)
    // Some(300)
    // None
}

fn built_in_borrowmut_misollari() {
    // Vec → BorrowMut<[T]>
    // Vec → BorrowMut<[T]>
    let mut vektor: Vec<i32> = vec![3, 1, 4, 1, 5, 9];
    let slice_mut: &mut [i32] = vektor.borrow_mut();
    slice_mut.sort();
    println!("{:?}", vektor);
    // [1, 1, 3, 4, 5, 9]

    // String → BorrowMut<str>
    // String → BorrowMut<str>
    let mut matn: String = String::from("salom");
    let str_mut: &mut str = matn.borrow_mut();
    str_mut.make_ascii_uppercase();
    println!("{}", matn);
    // SALOM

    // Box<T> → BorrowMut<T>
    // Box<T> → BorrowMut<T>
    let mut boxed: Box<i32> = Box::new(42);
    let val_mut: &mut i32 = boxed.borrow_mut();
    *val_mut = 99;
    println!("{}", boxed);
    // 99

    // i32 → BorrowMut<i32>
    // i32 → BorrowMut<i32>
    let mut son: i32 = 10;
    let son_mut: &mut i32 = son.borrow_mut();
    *son_mut *= 2;
    println!("{}", son);
    // 20
}

// ✅ lifetime qo'shildi — xarita dan qiymat qaytariladi
// ✅ добавлен lifetime — значение возвращается из xarita
fn xavfsiz_qidirish<'a, K, Q, V>(
    xarita: &'a HashMap<K, V>,
    kalit: &Q,
) -> Option<&'a V>
where
    K: Borrow<Q> + Eq + Hash,
    Q: Eq + Hash + ?Sized,
{
    xarita.get(kalit)
}

// Borrow bilan generic taqqoslash
// generic сравнение с Borrow
fn teng_ekanligini_tekshir<T, U>(a: &T, b: &U) -> bool
where
    T: Borrow<str>,
    U: Borrow<str>,
{
    a.borrow() == b.borrow()
}

// Borrow bilan generic eng kichik topish
// поиск наименьшего с Borrow
// ✅ soddalashtirildi — aniq tur bilan
// ✅ упрощено — с конкретным типом
fn eng_kichik_topish<'a>(elementlar: &'a [String]) -> Option<&'a String> {
    elementlar.iter().min()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct CacheKalit {
    namespace: String,
    kalit: String,
}

impl CacheKalit {
    fn new(namespace: &str, kalit: &str) -> Self {
        CacheKalit {
            namespace: namespace.to_string(),
            kalit: kalit.to_string(),
        }
    }
}

impl Borrow<str> for CacheKalit {
    fn borrow(&self) -> &str {
        &self.kalit
    }
}

struct Cache {
    ma_lumotlar: HashMap<CacheKalit, String>,
}

impl Cache {
    fn new() -> Self {
        Cache { ma_lumotlar: HashMap::new() }
    }

    fn qosh(&mut self, namespace: &str, kalit: &str, qiymat: &str) {
        self.ma_lumotlar.insert(
            CacheKalit::new(namespace, kalit),
            qiymat.to_string(),
        );
    }

    // &str bilan qidirish (Borrow orqali)
    // поиск через &str (через Borrow)
    fn ol(&self, kalit: &str) -> Option<&String> {
        self.ma_lumotlar.get(kalit)
    }
}

use std::borrow::Cow;

fn cow_borrow_misoli(kiritish: &str) -> Cow<'_, str> {
    if kiritish.contains(' ') {
        // O'zgartirish kerak — owned
        // Нужно изменить — owned
        let ozgartirilgan: String = kiritish.replace(' ', "_");
        Cow::Owned(ozgartirilgan)
    } else {
        // O'zgartirish kerak emas — borrowed
        // Изменение не нужно — borrowed
        Cow::Borrowed(kiritish)
    }
}

// ✅ Versiya uchun Borrow<Versiya> yozmaymiz
// ✅ core da T: Borrow<T> allaqachon mavjud
// ✅ Не пишем Borrow<Versiya> для Versiya
// ✅ В core уже есть impl<T> Borrow<T> for T

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Versiya {
    major: u32,
    minor: u32,
}

impl Versiya {
    fn new(major: u32, minor: u32) -> Self {
        Versiya { major, minor }
    }
}

impl fmt::Display for Versiya {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

fn main() {

    built_in_borrow_misollari();

    hashmap_borrow_misollari();

    hashset_borrow_misollari();

    vec_kalit_hashmap_misoli();

    custom_borrow_misoli();

    built_in_borrowmut_misollari();

    // xavfsiz_qidirish — generic HashMap
    // xavfsiz_qidirish — generic HashMap
    let mut xarita: HashMap<String, u32> = HashMap::new();
    xarita.insert(String::from("birinchi"), 1);
    xarita.insert(String::from("ikkinchi"), 2);

    let n1 = xavfsiz_qidirish(&xarita, "birinchi");
    let n2 = xavfsiz_qidirish(&xarita, "uchinchi");
    println!("{:?}", n1);
    println!("{:?}", n2);
    // Some(1)
    // None

    // teng_ekanligini_tekshir — &str va String
    // teng_ekanligini_tekshir — &str и String
    let str1: &str = "salom";
    let string1: String = String::from("salom");
    let string2: String = String::from("dunyo");

    println!("{}", teng_ekanligini_tekshir(&str1, &string1));
    println!("{}", teng_ekanligini_tekshir(&string1, &string2));
    // true
    // false

    // eng_kichik_topish — String slice
    // eng_kichik_topish — срез String
    let sozlar: Vec<String> = vec![
        String::from("banan"),
        String::from("olma"),
        String::from("anor"),
        String::from("uzum"),
    ];
    let eng_kichik = eng_kichik_topish(&sozlar);
    println!("{:?}", eng_kichik);
    // Some("anor")

    let mut cache = Cache::new();
    cache.qosh("users", "dilshod", "Dilshod Toshmatov");
    cache.qosh("users", "ali", "Ali Karimov");
    cache.qosh("config", "timeout", "30");

    let foydalanuvchi = cache.ol("dilshod");
    let timeout = cache.ol("timeout");
    let yoq = cache.ol("mavjud_emas");

    println!("{:?}", foydalanuvchi);
    println!("{:?}", timeout);
    println!("{:?}", yoq);
    // Some("Dilshod Toshmatov")
    // Some("30")
    // None

    // bo'sh joy yo'q — borrowed (clone yo'q)
    // нет пробелов — borrowed (без clone)
    let cow1: Cow<str> = cow_borrow_misoli("salom");
    println!("{:?}", cow1);
    println!("Borrowed: {}", matches!(cow1, Cow::Borrowed(_)));
    // "salom"
    // Borrowed: true

    // bo'sh joy bor — owned (clone qilingan)
    // есть пробелы — owned (с clone)
    let cow2: Cow<str> = cow_borrow_misoli("salom dunyo");
    println!("{:?}", cow2);
    println!("Owned: {}", matches!(cow2, Cow::Owned(_)));
    // "salom_dunyo"
    // Owned: true

    // Cow — borrow() orqali &str olish
    // Cow — получение &str через borrow()
    let cow_str: &str = cow1.borrow();
    println!("{}", cow_str);
    // salom

    // T: Borrow<T> — core da avtomatik bor
    // T: Borrow<T> — автоматически есть в core
    let mut versiya_xarita: HashMap<Versiya, &str> = HashMap::new();
    versiya_xarita.insert(Versiya::new(1, 0), "Birinchi versiya");
    versiya_xarita.insert(Versiya::new(2, 0), "Ikkinchi versiya");

    let v = Versiya::new(1, 0);
    let topilgan = versiya_xarita.get(&v);
    println!("{:?}", topilgan);
    // Some("Birinchi versiya")

    // String va str uchun Hash bir xil — Borrow kafolati
    // Hash одинаков для String и str — гарантия Borrow
    use std::hash::{DefaultHasher, Hasher};
    let mut h1 = DefaultHasher::new();
    let mut h2 = DefaultHasher::new();
    "kalit".hash(&mut h1);
    String::from("kalit").hash(&mut h2);
    println!("str hash:    {}", h1.finish());
    println!("String hash: {}", h2.finish());
    println!("Teng: {}", h1.finish() == h2.finish());
    // str hash:    XXXX
    // String hash: XXXX
    // Teng: true  ← Borrow kafolati!
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                          BORROW TRAIT                                                                                        |
// #================================================================================================================================================#
// # |   1 | impl Borrow<T> for S     | S dan &T olish (Hash+Eq kafolati bilan)              | Получение &T из S (с гарантией Hash+Eq)              |
// # |   2 | .borrow()                | &T reference olish                                   | Получение ссылки &T                                  |
// # |   3 | String: Borrow<str>      | HashMap<String,V> ga &str bilan kirish               | Доступ к HashMap<String,V> через &str                |
// # |   4 | Vec<T>: Borrow<[T]>      | HashMap<Vec<T>,V> ga &[T] bilan kirish               | Доступ к HashMap<Vec<T>,V> через &[T]                |
// # |   5 | T: Borrow<T>             | Har T o'zi uchun Borrow — core da avtomatik          | Каждый T заимствует сам себя — автоматически в core  |
// #================================================================================================================================================#
// # |                                        BORROWMUT TRAIT                                                                                       |
// #================================================================================================================================================#
// # |   6 | impl BorrowMut<T> for S  | S dan &mut T olish                                   | Получение &mut T из S                                |
// # |   7 | .borrow_mut()            | &mut T reference olish                               | Получение изменяемой ссылки &mut T                   |
// # |   8 | Vec: BorrowMut<[T]>      | slice mut orqali Vec o'zgartirish                    | Изменение Vec через мутабельный slice                |
// #================================================================================================================================================#
// # |                                    BORROW VS ASREF                                                                                           |
// #================================================================================================================================================#
// # |   9 | Borrow                   | Hash+Eq+Ord KAFOLATLANGAN — HashMap uchun            | Hash+Eq+Ord ГАРАНТИРОВАНЫ — для HashMap              |
// # |  10 | AsRef                    | Faqat reference — Hash/Eq kafolat yo'q               | Только ссылка — нет гарантии Hash/Eq                 |
// # |  11 | HashMap::get             | Borrow ishlatadi (AsRef emas)                        | Использует Borrow (не AsRef)                         |
// # |  12 | generic API              | AsRef — ko'proq qulay (kafolat kerak bo'lmasa)       | AsRef — удобнее (если гарантия не нужна)             |
// #================================================================================================================================================#
// # |                                    HASHMAP VA HASHSET BILAN                                                                                  |
// #================================================================================================================================================#
// # |  13 | map.get("str")           | String kalit → &str bilan qidirish                   | Поиск String ключа через &str                        |
// # |  14 | map.contains_key("str")  | String kalit → &str bilan tekshirish                 | Проверка String ключа через &str                     |
// # |  15 | map.remove("str")        | String kalit → &str bilan o'chirish                  | Удаление String ключа через &str                     |
// # |  16 | set.contains("str")      | String toplam → &str bilan tekshirish                | Проверка String множества через &str                 |
// #================================================================================================================================================#
// # |                                    KAFOLATLAR                                                                                                |
// #================================================================================================================================================#
// # |  17 | x.borrow()==y.borrow()   | x==y bo'lishi SHART                                  | x==y ОБЯЗАТЕЛЬНО                                     |
// # |  18 | hash(x.borrow())         | hash(x) bilan bir xil bo'lishi SHART                 | ДОЛЖЕН совпадать с hash(x)                           |
// # |  19 | x.borrow().cmp(y.borrow) | x.cmp(y) bilan bir xil bo'lishi SHART                | ДОЛЖЕН совпадать с x.cmp(y)                          |
// #================================================================================================================================================#
// # |                                    COW BILAN BOG'LIQLIGI                                                                                     |
// #================================================================================================================================================#
// # |  20 | Cow<'a, B>               | B: ToOwned + ?Sized — Borrow asosida qurilgan        | Построена на основе Borrow                           |
// # |  21 | Cow::Borrowed(&str)      | Clone qilinmagan — Borrow                            | Без clone — Borrow                                   |
// # |  22 | Cow::Owned(String)       | Clone qilingan — owned                               | С clone — owned                                      |
// #================================================================================================================================================#
// # |                                    GENERIC BILAN                                                                                             |
// #================================================================================================================================================#
// # |  23 | K: Borrow<Q>+Eq+Hash     | Generic HashMap qidirish                             | Generic поиск в HashMap                              |
// # |  24 | T: Borrow<str>           | &str va String ikkalasini qabul qilish               | Принятие &str и String                               |
// # |  25 | lifetime 'a qo'shish     | xarita dan qiymat qaytarilganda lifetime kerak        | Lifetime нужен при возврате значения из xarita      |
// #================================================================================================================================================#