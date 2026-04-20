// #================================================================================================================================================#
// #                                                        INTOITERATOR  |  FROMITERATOR                                                           #
// #                INTOITERATOR — TURNI ITERATÖRGA AYLANTIRISH (for loop). FROMITERATOR — ITERATORDAN TUR YARATISH (collect()).                    #
// #                INTOITERATOR — ПРЕОБРАЗОВАНИЕ В ИТЕРАТОР (for loop). FROMITERATOR — СОЗДАНИЕ ТИПА ИЗ ИТЕРАТОРА (collect()).                     #
// #================================================================================================================================================#

// ════════════════════════════════════════════════════════════════════════════
//  VAZIFA (TASK)
// ════════════════════════════════════════════════════════════════════════════
//
//  IntoIterator:
//    - Har qanday turni for loop da ishlatish imkonini beradi
//    - for x in collection {} — bu aslida IntoIterator::into_iter() chaqiradi
//    - Vec, HashMap, BTreeMap, String — barchasi IntoIterator implement qilgan
//    - Custom struct ni for loop da ishlatish uchun implement qilinadi
//
//  FromIterator:
//    - Iterator dan yangi tur yaratish imkonini beradi
//    - iterator.collect::<T>() — bu aslida T::from_iter() chaqiradi
//    - Vec, HashMap, String, BTreeMap — barchasi FromIterator implement qilgan
//    - Custom kolleksiyani collect() bilan to'ldirish uchun implement qilinadi
//
//  IntoIterator — для использования любого типа в for loop
//    - for x in collection {} — вызывает IntoIterator::into_iter()
//  FromIterator — для создания типа из итератора
//    - iterator.collect::<T>() — вызывает T::from_iter()

// ════════════════════════════════════════════════════════════════════════════
//  QONUN-QOIDALAR (RULES)
// ════════════════════════════════════════════════════════════════════════════
//
//  IntoIterator qoidalari:
//  Правила IntoIterator:
//
//    trait IntoIterator {
//        type Item;
//        type IntoIter: Iterator<Item = Self::Item>;
//        fn into_iter(self) -> Self::IntoIter;
//    }
//
//  1. type Item — iterator qaytaradigan element turi
//     QOIDA: &T, &mut T, T — istalgan bo'lishi mumkin
//
//  2. type IntoIter — qaytariladigan iterator turi
//     QOIDA: Iterator<Item = Self::Item> bo'lishi shart
//
//  3. Uch xil implement:
//     QOIDA: impl IntoIterator for T      → T ownership oladi (into_iter)
//     QOIDA: impl IntoIterator for &T     → &T reference (iter)
//     QOIDA: impl IntoIterator for &mut T → &mut T mutable (iter_mut)
//
//  4. for loop desugaring:
//     QOIDA: for x in v { ... }
//            ≡ let mut iter = v.into_iter();
//              while let Some(x) = iter.next() { ... }
//
//  ─────────────────────────────────────────────────────────────────────────
//
//  FromIterator qoidalari:
//  Правила FromIterator:
//
//    trait FromIterator<A> {
//        fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self;
//    }
//
//  1. A — iteratordagi element turi
//     QOIDA: Iterator<Item = A> bo'lishi kerak
//
//  2. collect() desugaring:
//     QOIDA: iter.collect::<Vec<i32>>()
//            ≡ Vec::<i32>::from_iter(iter)
//
//  3. Turbo fish kerak bo'ladi:
//     QOIDA: let v: Vec<i32> = iter.collect();       — tur annotatsiya
//     QOIDA: let v = iter.collect::<Vec<i32>>();      — turbo fish
//
//  4. Result<Vec<T>, E> ga collect:
//     QOIDA: Vec<Result<T,E>> → Result<Vec<T>,E>
//            Bitta xato bo'lsa — butun Result::Err qaytadi

// ════════════════════════════════════════════════════════════════════════════
//  QACHON ISHLATILADI (WHEN TO USE)
// ════════════════════════════════════════════════════════════════════════════
//
//  IntoIterator:
//  ✅ Custom struct ni for loop da ishlatish kerak bo'lganda
//  ✅ API foydalanuvchi uchun qulay iteratsiya interfeysini ta'minlash
//  ✅ Iterator qabul qiluvchi funksiyalarga custom struct berish
//  ✅ &T, &mut T, T — uchala variant kerak bo'lganda
//
//  FromIterator:
//  ✅ Custom kolleksiyani .collect() bilan to'ldirish kerak bo'lganda
//  ✅ Iterator dan custom tur yaratish
//  ✅ Pipeline oxirida natijani maxsus strukturada saqlash
//  ✅ Result<Collection, E> pattern — xatolarni yig'ish

// ════════════════════════════════════════════════════════════════════════════
//  NIMA BILAN BIRGA ISHLAYDI (WORKS WITH)
// ════════════════════════════════════════════════════════════════════════════
//
//  IntoIterator:
//  ✅ for loop              — asosiy qo'llanish joyi
//  ✅ Iterator::chain()     — IntoIterator qabul qiladi
//  ✅ Iterator::zip()       — IntoIterator qabul qiladi
//  ✅ Funksiya argumentlari — impl IntoIterator<Item=T>
//  ✅ Vec, HashMap, BTreeMap, LinkedList, String, Range
//  ✅ &[T] slice            — iter() qaytaradi
//
//  FromIterator:
//  ✅ .collect()            — asosiy qo'llanish joyi
//  ✅ Vec<T>                — eng ko'p ishlatiladigan
//  ✅ HashMap<K, V>         — (K, V) tuple dan
//  ✅ String                — char yoki &str dan
//  ✅ BTreeMap, BTreeSet    — tartiblangan kolleksiyalar
//  ✅ Result<Vec<T>, E>     — xatolarni to'plash
//  ✅ Option<Vec<T>>        — None ni tekshirish
//  ✅ HashSet<T>            — takrorlanmaslar

// ════════════════════════════════════════════════════════════════════════════
//  INTERVYU SAVOLLARI VA JAVOBLARI
// ════════════════════════════════════════════════════════════════════════════
//
//  S: for x in v {} ichida nima sodir bo'ladi?
//  J: v.into_iter() chaqiriladi (IntoIterator::into_iter).
//     Keyin har iteratsiyada iter.next() chaqiriladi.
//     None qaytganda loop tugaydi.
//
//  S: for x in &v vs for x in v farqi?
//  J: for x in &v  → &v.into_iter() → &T reference, v saqlanadi
//     for x in v   → v.into_iter()  → T ownership, v yo'qoladi
//     for x in &mut v → &mut T, v o'zgartirilishi mumkin
//
//  S: .collect() qanday ishlaydi?
//  J: Iterator::collect<B>() → B::from_iter(self) chaqiriladi.
//     B — FromIterator implement qilgan istalgan tur.
//     Kompilyator B ni tur annotatsiyasi yoki turbo fish dan aniqlaydi.
//
//  S: Vec<Result<T,E>> ni Result<Vec<T>,E> ga qanday aylantirish?
//  J: iter.collect::<Result<Vec<T>, E>>()
//     Bitta Err bo'lsa — butun Result::Err qaytadi.
//     Hammasi Ok bo'lsa — Result::Ok(Vec<T>) qaytadi.
//
//  S: IntoIterator va Iterator farqi nima?
//  J: Iterator — next() bor, iteratsiya qilish mumkin.
//     IntoIterator — into_iter() bor, Iterator ga aylanishi mumkin.
//     Har Iterator IntoIterator implement qiladi (o'zini qaytaradi).
//     Lekin har IntoIterator Iterator emas.
//
//  S: Nima uchun IntoIterator uchta implement kerak (&T, &mut T, T)?
//  J: for x in &v   → T ni borrow qilish, v keyin ham ishlatiladi
//     for x in &mut v → T ni o'zgartirish
//     for x in v    → T ownership olish, keyin v yo'qoladi
//     Uchala variantni qo'llab-quvvatlash API ni qulay qiladi.
//
//  S: impl IntoIterator<Item=T> vs Vec<T> argument farqi?
//  J: Vec<T> — faqat Vec qabul qiladi
//     impl IntoIterator<Item=T> — Vec, slice, Range, custom struct — hammasi
//     Bu API ni yanada flexible qiladi.
//
//  S: Extend trait IntoIterator bilan qanday bog'liq?
//  J: extend() IntoIterator qabul qiladi.
//     v.extend(other_iter) — other_iter ni v ga qo'shadi.
//     FromIterator bir marta yaratsa, Extend mavjud ga qo'shadi.

#![allow(dead_code, unused)]

use std::collections::HashMap;
use std::iter::FromIterator;

fn for_loop_desugaring() {

    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    // for loop — qulay yozuv
    // for loop — удобная запись
    for x in &v {
        print!("{} ", x);
    }
    println!();
    // 1 2 3 4 5

    // for loop — aslida shu!
    // for loop — на самом деле вот это!
    {
        let mut iter = (&v).into_iter();
        while let Some(x) = iter.next() {
            print!("{} ", x);
        }
        println!();
    }
    // 1 2 3 4 5  ← bir xil natija!

    // for x in &v    → &T reference — v saqlanadi
    // for x in &v    → &T reference — v сохраняется
    for x in &v { print!("{} ", x); }
    println!();
    println!("v hali bor: {:?}", v);
    // 1 2 3 4 5
    // v hali bor: [1, 2, 3, 4, 5]

    // for x in v     → T ownership — v yo'qoladi
    // for x in v     → T ownership — v исчезает
    let v2: Vec<i32> = vec![10, 20, 30];
    for x in v2 {
        print!("{} ", x);
    }
    println!();
    // println!("{:?}", v2);  // ← xato! v2 moved
    // 10 20 30
}

fn builtin_intoiterator_misollari() {

    // Range — IntoIterator
    // Range — IntoIterator
    for x in 1..=5 {
        print!("{} ", x);
    }
    println!();
    // 1 2 3 4 5

    // HashMap — IntoIterator
    // HashMap — IntoIterator
    let mut xarita: HashMap<&str, i32> = HashMap::new();
    xarita.insert("bir", 1);
    xarita.insert("ikki", 2);
    xarita.insert("uch", 3);

    // &HashMap → (&K, &V) beradi
    // &HashMap → даёт (&K, &V)
    let mut juftlar: Vec<_> = (&xarita).into_iter().collect();
    juftlar.sort_by_key(|&(&k, _)| k);
    println!("{:?}", juftlar);
    // [("bir", 1), ("ikki", 2), ("uch", 3)]

    // String — chars() orqali
    // String — через chars()
    let s: String = String::from("salom");
    for ch in s.chars() {
        print!("{} ", ch);
    }
    println!();
    // s a l o m

    // Option — IntoIterator (0 yoki 1 element)
    // Option — IntoIterator (0 или 1 элемент)
    let bor: Option<i32> = Some(42);
    let yoq: Option<i32> = None;

    if let Some(x) = bor { println!("Some: {}", x); }
    // Some: 42
    if let Some(x) = yoq { println!("Bu chiqmaydi"); }
    // (hech narsa)

    // Option flat_map da
    // Option в flat_map
    let qiymatlar: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
    let natija: Vec<i32> = qiymatlar.into_iter().flatten().collect();
    println!("{:?}", natija);
    // [1, 3, 5]
}

#[derive(Debug, Clone)]
struct Toplam {
    elementlar: Vec<i32>,
}

impl Toplam {
    fn new(elementlar: Vec<i32>) -> Self {
        Toplam { elementlar }
    }
}

// 1. T — ownership (into_iter)
// 1. T — владение (into_iter)
impl IntoIterator for Toplam {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.elementlar.into_iter()
    }
}

// 2. &T — reference (iter)
// 2. &T — ссылка (iter)
impl<'a> IntoIterator for &'a Toplam {
    type Item = &'a i32;
    type IntoIter = std::slice::Iter<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.elementlar.iter()
    }
}

// 3. &mut T — mutable reference (iter_mut)
// 3. &mut T — мутабельная ссылка (iter_mut)
impl<'a> IntoIterator for &'a mut Toplam {
    type Item = &'a mut i32;
    type IntoIter = std::slice::IterMut<'a, i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.elementlar.iter_mut()
    }
}

fn custom_intoiterator_misoli() {

    let toplam = Toplam::new(vec![1, 2, 3, 4, 5]);

    // &T — reference for loop — toplam saqlanadi
    // &T — reference for loop — toplam сохраняется
    for x in &toplam {
        print!("{} ", x);
    }
    println!();
    // 1 2 3 4 5

    // &mut T — mutable for loop
    // &mut T — мутабельный for loop
    let mut o_zgaruvchan = toplam.clone();
    for x in &mut o_zgaruvchan {
        *x *= 2;
    }
    println!("{:?}", o_zgaruvchan.elementlar);
    // [2, 4, 6, 8, 10]

    // T — ownership for loop — toplam yo'qoladi
    // T — ownership for loop — toplam исчезает
    let yig_indi: i32 = toplam.into_iter().sum();
    println!("{}", yig_indi);
    // 15

    // chain bilan — IntoIterator qabul qiladi
    // с chain — принимает IntoIterator
    let a = Toplam::new(vec![1, 2, 3]);
    let b = Toplam::new(vec![4, 5, 6]);
    let birga: Vec<i32> = a.into_iter().chain(b).collect();
    println!("{:?}", birga);
    // [1, 2, 3, 4, 5, 6]
}

// impl IntoIterator — Vec, slice, Range, custom — hammasi qabul qilinadi
// impl IntoIterator — принимает Vec, slice, Range, custom — всё

fn yig_indi_hisoblash(elementlar: impl IntoIterator<Item = i32>) -> i32 {
    elementlar.into_iter().sum()
}

fn chiqarish(elementlar: impl IntoIterator<Item = i32>) {
    for x in elementlar {
        print!("{} ", x);
    }
    println!();
}

fn o_rtacha(elementlar: impl IntoIterator<Item = f64>) -> f64 {
    let mut sum: f64 = 0.0;
    let mut count: usize = 0;
    for x in elementlar {
        sum += x;
        count += 1;
    }
    if count == 0 { 0.0 } else { sum / count as f64 }
}

fn impl_intoiterator_arg_misoli() {

    // Vec bilan
    // с Vec
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{}", yig_indi_hisoblash(v));
    // 15

    // Range bilan
    // с Range
    println!("{}", yig_indi_hisoblash(1..=10));
    // 55

    // Custom Toplam bilan
    // с Custom Toplam
    let t = Toplam::new(vec![10, 20, 30]);
    println!("{}", yig_indi_hisoblash(t));
    // 60

    // Array bilan
    // с Array
    chiqarish([100, 200, 300]);
    // 100 200 300

    // o'rtacha — turli turlar
    // среднее — разные типы
    println!("{:.2}", o_rtacha(vec![1.0, 2.0, 3.0, 4.0, 5.0]));
    println!("{:.2}", o_rtacha([10.0, 20.0, 30.0]));
    // 3.00
    // 20.00
}

fn builtin_collect_misollari() {

    // Vec<T> ga collect
    // collect в Vec<T>
    let v: Vec<i32> = (1..=5).collect();
    println!("{:?}", v);
    // [1, 2, 3, 4, 5]

    // Vec<String> ga collect
    // collect в Vec<String>
    let suzlar: Vec<String> = vec!["bir", "ikki", "uch"]
        .iter()
        .map(|s| s.to_uppercase())
        .collect();
    println!("{:?}", suzlar);
    // ["BIR", "IKKI", "UCH"]

    // String ga collect — char dan
    // collect в String — из char
    let s: String = "salom".chars().map(|c| c.to_ascii_uppercase()).collect();
    println!("{}", s);
    // SALOM

    // String ga collect — &str dan join
    // collect в String — join из &str
    let s2: String = vec!["bir", "ikki", "uch"].join(", ");
    println!("{}", s2);
    // bir, ikki, uch

    // HashMap ga collect — (K, V) tuple dan
    // collect в HashMap — из кортежей (K, V)
    let xarita: HashMap<&str, i32> = vec![("bir", 1), ("ikki", 2), ("uch", 3)]
        .into_iter()
        .collect();
    println!("{:?}", xarita.get("ikki"));
    // Some(2)

    // HashSet ga collect — takrorlanmaslar
    // collect в HashSet — уникальные элементы
    use std::collections::HashSet;
    let set: HashSet<i32> = vec![1, 2, 2, 3, 3, 3, 4].into_iter().collect();
    let mut sorted: Vec<i32> = set.into_iter().collect();
    sorted.sort();
    println!("{:?}", sorted);
    // [1, 2, 3, 4]

    // BTreeMap ga collect — tartiblangan
    // collect в BTreeMap — отсортированный
    use std::collections::BTreeMap;
    let btree: BTreeMap<&str, i32> = vec![("c", 3), ("a", 1), ("b", 2)]
        .into_iter()
        .collect();
    for (k, v) in &btree {
        print!("{}:{} ", k, v);
    }
    println!();
    // a:1 b:2 c:3  ← tartiblangan!

    // unzip — (A, B) dan (Vec<A>, Vec<B>)
    // unzip — из (A, B) получить (Vec<A>, Vec<B>)
    let (kalit, qiymat): (Vec<&str>, Vec<i32>) = vec![
        ("bir", 1), ("ikki", 2), ("uch", 3)
    ].into_iter().unzip();
    println!("{:?}", kalit);
    println!("{:?}", qiymat);
    // ["bir", "ikki", "uch"]
    // [1, 2, 3]
}

fn result_option_collect_misollari() {

    // Vec<Result<T,E>> → Result<Vec<T>,E>
    // Vec<Result<T,E>> → Result<Vec<T>,E>
    let to_g_ri_stringlar: Vec<&str> = vec!["1", "2", "3", "4", "5"];
    let natija: Result<Vec<i32>, _> = to_g_ri_stringlar.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{:?}", natija);
    // Ok([1, 2, 3, 4, 5])

    // Bitta xato — butun Result::Err
    // Одна ошибка — весь Result::Err
    let xatoli_stringlar: Vec<&str> = vec!["1", "ikki", "3"];
    let xato_natija: Result<Vec<i32>, _> = xatoli_stringlar.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{}", xato_natija.is_err());
    // true

    // Vec<Option<T>> → Option<Vec<T>>
    // Vec<Option<T>> → Option<Vec<T>>
    let barchasi_some: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
    let birga: Option<Vec<i32>> = barchasi_some.into_iter().collect();
    println!("{:?}", birga);
    // Some([1, 2, 3])

    // Bitta None — butun None
    // Одна None — весь None
    let biri_none: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
    let none_natija: Option<Vec<i32>> = biri_none.into_iter().collect();
    println!("{:?}", none_natija);
    // None

    // Partition — shartga qarab ikki guruh
    // Partition — разделение на две группы по условию
    let sonlar: Vec<i32> = (1..=10).collect();
    let (juftlar, toqlar): (Vec<i32>, Vec<i32>) = sonlar.iter()
        .partition(|x| *x % 2 == 0);
    println!("{:?}", juftlar);
    println!("{:?}", toqlar);
    // [2, 4, 6, 8, 10]
    // [1, 3, 5, 7, 9]
}

// Custom kolleksiya — FromIterator bilan
// Пользовательская коллекция — с FromIterator
#[derive(Debug)]
struct SonlarToplami {
    elementlar: Vec<i32>,
    yig_indi: i32,
    o_rtacha: f64,
}

impl SonlarToplami {
    fn yangilash(&mut self) {
        self.yig_indi = self.elementlar.iter().sum();
        self.o_rtacha = if self.elementlar.is_empty() {
            0.0
        } else {
            self.yig_indi as f64 / self.elementlar.len() as f64
        };
    }
}

// Iterator dan SonlarToplami yaratish
// Создание SonlarToplami из итератора
impl FromIterator<i32> for SonlarToplami {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let elementlar: Vec<i32> = iter.into_iter().collect();
        let yig_indi: i32 = elementlar.iter().sum();
        let o_rtacha: f64 = if elementlar.is_empty() {
            0.0
        } else {
            yig_indi as f64 / elementlar.len() as f64
        };
        SonlarToplami { elementlar, yig_indi, o_rtacha }
    }
}

// So'zlar statistikasi — collect bilan to'ldiriladi
// Статистика слов — заполняется через collect
#[derive(Debug)]
struct SozStatistikasi {
    jami_sozlar: usize,
    noyob_sozlar: std::collections::HashSet<String>,
    eng_uzun: Option<String>,
}

impl FromIterator<String> for SozStatistikasi {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut jami: usize = 0;
        let mut noyob: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut eng_uzun: Option<String> = None;

        for soz in iter {
            jami += 1;
            if eng_uzun.as_ref().map_or(true, |s| soz.len() > s.len()) {
                eng_uzun = Some(soz.clone());
            }
            noyob.insert(soz);
        }

        SozStatistikasi {
            jami_sozlar: jami,
            noyob_sozlar: noyob,
            eng_uzun,
        }
    }
}

// CSV satrlardan HashMap ga
// Из строк CSV в HashMap
fn csv_dan_xarita(csv: &str) -> HashMap<String, String> {
    csv.lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let mut qismlar = line.splitn(2, '=');
            let kalit = qismlar.next()?.trim().to_string();
            let qiymat = qismlar.next()?.trim().to_string();
            Some((kalit, qiymat))
        })
        .collect()
}

// So'zlar chastotasi
// Частота слов
fn sozlar_chastotasi(matn: &str) -> HashMap<String, usize> {
    matn.split_whitespace()
        .map(|s| s.to_lowercase())
        .fold(HashMap::new(), |mut acc, soz| {
            *acc.entry(soz).or_insert(0) += 1;
            acc
        })
}

// Validatsiya — xatolarni yig'ish
// Валидация — сбор ошибок
fn validatsiya_xatolari(qiymatlar: &[&str]) -> Result<Vec<i32>, Vec<String>> {
    let (muvaffaqiyatlilar, xatolar): (Vec<_>, Vec<_>) = qiymatlar.iter()
        .map(|s| s.parse::<i32>().map_err(|_| format!("'{}' raqam emas", s)))
        .partition(Result::is_ok);

    if xatolar.is_empty() {
        Ok(muvaffaqiyatlilar.into_iter().map(Result::unwrap).collect())
    } else {
        Err(xatolar.into_iter().map(Result::unwrap_err).collect())
    }
}

fn main() {

    println!("=== FOR LOOP DESUGARING ===");
    for_loop_desugaring();

    println!("\n=== BUILT-IN INTOITERATOR ===");
    builtin_intoiterator_misollari();

    println!("\n=== CUSTOM INTOITERATOR ===");
    custom_intoiterator_misoli();

    println!("\n=== IMPL INTOITERATOR ARGUMENT ===");
    impl_intoiterator_arg_misoli();

    println!("\n=== BUILT-IN COLLECT ===");
    builtin_collect_misollari();

    println!("\n=== RESULT VA OPTION COLLECT ===");
    result_option_collect_misollari();

    println!("\n=== CUSTOM FROMITERATOR ===");

    // SonlarToplami — collect bilan
    // SonlarToplami — с collect
    let toplam: SonlarToplami = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .into_iter()
        .collect();
    println!("Elementlar: {:?}", toplam.elementlar);
    println!("Yig'indi:   {}", toplam.yig_indi);
    println!("O'rtacha:   {:.1}", toplam.o_rtacha);
    // Elementlar: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    // Yig'indi:   55
    // O'rtacha:   5.5

    // filter + collect → SonlarToplami
    // filter + collect → SonlarToplami
    let juft_toplam: SonlarToplami = (1..=20)
        .filter(|x| x % 2 == 0)
        .collect();
    println!("Juft yig'indi: {}", juft_toplam.yig_indi);
    println!("Juft o'rtacha: {:.1}", juft_toplam.o_rtacha);
    // Juft yig'indi: 110
    // Juft o'rtacha: 11.0

    // SozStatistikasi — collect bilan
    // SozStatistikasi — с collect
    let matn: &str = "rust tili rust xavfsiz va tez rust";
    let stat: SozStatistikasi = matn.split_whitespace()
        .map(String::from)
        .collect();
    println!("Jami so'zlar: {}", stat.jami_sozlar);
    println!("Noyob so'zlar: {}", stat.noyob_sozlar.len());
    println!("Eng uzun: {:?}", stat.eng_uzun);
    // Jami so'zlar: 7
    // Noyob so'zlar: 5
    // Eng uzun: Some("xavfsiz")

    println!("\n=== REAL HAYOT ===");

    // CSV → HashMap
    // CSV → HashMap
    let csv: &str = "
        host = localhost
        port = 8080
        debug = true
        timeout = 30
    ";
    let konfiguratsiya: HashMap<String, String> = csv_dan_xarita(csv);
    println!("{:?}", konfiguratsiya.get("port"));
    println!("{:?}", konfiguratsiya.get("host"));
    // Some("8080")
    // Some("localhost")

    // So'zlar chastotasi
    // Частота слов
    let matn2: &str = "rust tili rust yaxshi rust xavfsiz va tez";
    let mut chastota: Vec<(String, usize)> = sozlar_chastotasi(matn2)
        .into_iter()
        .collect();
    chastota.sort_by(|a, b| b.1.cmp(&a.1));
    for (soz, n) in chastota.iter().take(3) {
        println!("{}: {} marta", soz, n);
    }
    // rust: 3 marta
    // tili: 1 marta
    // yaxshi: 1 marta

    // Validatsiya
    // Валидация
    let to_g_ri: Result<Vec<i32>, _> = validatsiya_xatolari(&["1", "2", "3"]);
    let xatoli: Result<Vec<i32>, _> = validatsiya_xatolari(&["1", "ikki", "3", "to'rt"]);
    println!("{:?}", to_g_ri);
    println!("{:?}", xatoli);
    // Ok([1, 2, 3])
    // Err(["'ikki' raqam emas", "'to'rt' raqam emas"])
}

// #===============================================================================════════════════════════════════════════════════════════════════#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                              | Описание (RU)                                            |
// #===============================================================================════════════════════════════════════════════════════════════════#
// # |                                            INTOITERATOR                                                                                     |
// #===============================================================================════════════════════════════════════════════════════════════════#
// # |   1 | impl IntoIterator for T         | Ownership for loop                       | For loop с владением                                     |
// # |   2 | impl IntoIterator for &T        | Reference for loop — T saqlanadi         | For loop по ссылке — T сохраняется                       |
// # |   3 | impl IntoIterator for &mut T    | Mutable for loop                         | Мутабельный for loop                                     |
// # |   4 | type IntoIter: Iterator<Item=.> | Qaytariladigan iterator turi             | Тип возвращаемого итератора                              |
// # |   5 | for x in v → v.into_iter()      | for loop desugaring                      | Разворачивание for loop                                  |
// # |   6 | impl IntoIterator<Item=T> arg   | Vec, Range, custom — hammasi qabul       | Vec, Range, custom — всё принимается                     |
// #===============================================================================════════════════════════════════════════════════════════════════#
// # |                                            FROMITERATOR                                                                                     |
// #===============================================================================════════════════════════════════════════════════════════════════#
// # |   7 | impl FromIterator<A> for T      | collect() integratsiyasi                 | Интеграция с collect()                                   |
// # |   8 | fn from_iter<I:IntoIter>(i)->T  | Iterator dan T yaratish                  | Создание T из итератора                                  |
// # |   9 | .collect::<Vec<T>>()            | Turbo fish bilan collect                 | Collect с turbofish                                      |
// # |  10 | let v: Vec<T> = iter.collect()  | Tur annotatsiya bilan collect            | Collect с аннотацией типа                                |
// #===============================================================================════════════════════════════════════════════════════════════════#
// # |                                        COLLECT TURLARI                                                                                      |
// #===============================================================================════════════════════════════════════════════════════════════════#
// # |  11 | Vec<T>                          | Eng ko'p ishlatiladigan                  | Наиболее часто используемый                              |
// # |  12 | HashMap<K,V>                    | (K,V) tuple dan                          | Из кортежей (K,V)                                        |
// # |  13 | String                          | char yoki &str dan                       | Из char или &str                                         |
// # |  14 | HashSet<T>                      | Takrorlanmaslar                          | Уникальные элементы                                      |
// # |  15 | BTreeMap<K,V>                   | Tartiblangan HashMap                     | Отсортированный HashMap                                  |
// # |  16 | Result<Vec<T>,E>                | Bitta Err → butun Err                    | Одна Err → весь Err                                      |
// # |  17 | Option<Vec<T>>                  | Bitta None → butun None                  | Одна None → весь None                                    |
// # |  18 | (Vec<A>, Vec<B>) unzip          | Juftlarni ajratish                       | Разделение пар                                           |
// # |  19 | (Vec<T>, Vec<T>) partition      | Shartga qarab ikki guruh                 | Разделение на две группы                                 |
// #===============================================================================════════════════════════════════════════════════════════════════#