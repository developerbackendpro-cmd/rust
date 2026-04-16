// #================================================================================================================================================#
// #                                                EXACTSIZEITERATOR  |  DOUBLEENDEDITERATOR  |  FUSEDITERATOR                                     #
// #                                        ANIQ O'LCHAM | IKKI TOMONDAN ITERATSIYA | NONE DAN KEYIN XAVFSIZ CHAQIRISH                              #
// #                                        ТОЧНЫЙ РАЗМЕР | ИТЕРАЦИЯ С ДВУХ СТОРОН | БЕЗОПАСНЫЙ ВЫЗОВ ПОСЛЕ NONE                                    #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::iter::FusedIterator;

// ExactSizeIterator — size_hint() aniq bo'lganda
// ExactSizeIterator — когда size_hint() точен
//
//  trait ExactSizeIterator: Iterator {
//      fn len(&self) -> usize {
//          let (lower, upper) = self.size_hint();
//          upper.unwrap()  // aniq = lower == upper
//      }
//  }
//
// size_hint() shart: (n, Some(n)) — lower == upper bo'lishi kerak
// Требование size_hint(): (n, Some(n)) — lower должен равняться upper

struct AniqHisoblagich {
    joriy: usize,
    max: usize,
}

impl AniqHisoblagich {
    fn new(max: usize) -> Self {
        AniqHisoblagich { joriy: 0, max }
    }
}

impl Iterator for AniqHisoblagich {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.joriy < self.max {
            self.joriy += 1;
            Some(self.joriy)
        } else {
            None
        }
    }

    // size_hint — (qolgan, Some(qolgan)) — aniq bo'lishi shart
    // size_hint — (оставшиеся, Some(оставшиеся)) — должен быть точным
    fn size_hint(&self) -> (usize, Option<usize>) {
        let qolgan: usize = self.max - self.joriy;
        (qolgan, Some(qolgan))
    }
}

// ExactSizeIterator — size_hint aniq bo'lganda implement qilinadi
// ExactSizeIterator — реализуется когда size_hint точен
impl ExactSizeIterator for AniqHisoblagich {
    fn len(&self) -> usize {
        self.max - self.joriy
    }
}

// Matn tokenizator — aniq o'lcham
// Токенизатор текста — точный размер
struct TokenIter<'a> {
    tokenlar: &'a [&'a str],
    indeks: usize,
}

impl<'a> TokenIter<'a> {
    fn new(tokenlar: &'a [&'a str]) -> Self {
        TokenIter { tokenlar, indeks: 0 }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.indeks < self.tokenlar.len() {
            let token: &str = self.tokenlar[self.indeks];
            self.indeks += 1;
            Some(token)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let qolgan: usize = self.tokenlar.len() - self.indeks;
        (qolgan, Some(qolgan))
    }
}

impl<'a> ExactSizeIterator for TokenIter<'a> {}

fn exact_size_misollari() {

    // len() — qolgan elementlar soni
    // len() — количество оставшихся элементов
    let mut it = AniqHisoblagich::new(5);
    println!("{}", it.len());
    it.next();
    it.next();
    println!("{}", it.len());
    // 5
    // 3

    // Vec — ExactSizeIterator implement qilgan
    // Vec — реализует ExactSizeIterator
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut v_iter = v.iter();
    println!("{}", v_iter.len());
    v_iter.next();
    println!("{}", v_iter.len());
    // 5
    // 4

    // len() — pre-allocation uchun foydali
    // len() — полезно для предварительного выделения памяти
    let iter = AniqHisoblagich::new(10);
    let uzunlik: usize = iter.len();
    let mut natija: Vec<usize> = Vec::with_capacity(uzunlik);
    for x in AniqHisoblagich::new(10) {
        natija.push(x * x);
    }
    println!("{:?}", natija);
    // [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]

    // is_empty() — len() == 0
    // is_empty() — len() == 0
    let bosh = AniqHisoblagich::new(0);
    println!("{}", bosh.len());
    // 0

    // TokenIter — ExactSizeIterator
    // TokenIter — ExactSizeIterator
    let tokenlar: &[&str] = &["salom", "dunyo", "rust", "tili"];
    let token_iter = TokenIter::new(tokenlar);
    println!("Token soni: {}", token_iter.len());
    let yig_ilgan: Vec<&str> = token_iter.collect();
    println!("{:?}", yig_ilgan);
    // Token soni: 4
    // ["salom", "dunyo", "rust", "tili"]

    // zip — ExactSizeIterator bilan qulay
    // zip — удобно с ExactSizeIterator
    let a = AniqHisoblagich::new(3);
    let b = AniqHisoblagich::new(3);
    let juft: Vec<(usize, usize)> = a.zip(b).collect();
    println!("{:?}", juft);
    // [(1, 1), (2, 2), (3, 3)]
}

// DoubleEndedIterator — next_back() ham implement qilinadi
// DoubleEndedIterator — реализуется и next_back()
//
//  trait DoubleEndedIterator: Iterator {
//      fn next_back(&mut self) -> Option<Self::Item>;
//  }
//
// Shart: old_indeks va orqa_indeks kesishmaydi
// Требование: old_indeks и orqa_indeks не пересекаются

#[derive(Debug)]
struct IkkiTomonliIter {
    qiymatlar: Vec<i32>,
    old: usize,   // olddan
    orqa: usize,  // orqadan (exclusive)
}

impl IkkiTomonliIter {
    fn new(v: Vec<i32>) -> Self {
        let uzunlik: usize = v.len();
        IkkiTomonliIter { qiymatlar: v, old: 0, orqa: uzunlik }
    }
}

impl Iterator for IkkiTomonliIter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.old < self.orqa {
            let val: i32 = self.qiymatlar[self.old];
            self.old += 1;
            Some(val)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n: usize = self.orqa - self.old;
        (n, Some(n))
    }
}

// next_back() — orqadan olish
// next_back() — взять с конца
impl DoubleEndedIterator for IkkiTomonliIter {
    fn next_back(&mut self) -> Option<i32> {
        if self.old < self.orqa {
            self.orqa -= 1;
            Some(self.qiymatlar[self.orqa])
        } else {
            None
        }
    }
}

impl ExactSizeIterator for IkkiTomonliIter {}

// Matn satr iteratori — ikki tomondan
// Итератор строк текста — с двух сторон
struct SatrIter<'a> {
    satrlar: Vec<&'a str>,
    old: usize,
    orqa: usize,
}

impl<'a> SatrIter<'a> {
    fn new(matn: &'a str) -> Self {
        let satrlar: Vec<&str> = matn.lines().collect();
        let uzunlik: usize = satrlar.len();
        SatrIter { satrlar, old: 0, orqa: uzunlik }
    }
}

impl<'a> Iterator for SatrIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.old < self.orqa {
            let satr: &str = self.satrlar[self.old];
            self.old += 1;
            Some(satr)
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for SatrIter<'a> {
    fn next_back(&mut self) -> Option<&'a str> {
        if self.old < self.orqa {
            self.orqa -= 1;
            Some(self.satrlar[self.orqa])
        } else {
            None
        }
    }
}

fn double_ended_misollari() {

    // next() va next_back() — aralash
    // next() и next_back() — вперемешку
    let mut it = IkkiTomonliIter::new(vec![1, 2, 3, 4, 5]);
    println!("{:?}", it.next());
    println!("{:?}", it.next_back());
    println!("{:?}", it.next());
    println!("{:?}", it.next_back());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    // Some(1)
    // Some(5)
    // Some(2)
    // Some(4)
    // Some(3)
    // None

    // .rev() — teskari tartib
    // .rev() — обратный порядок
    let teskari: Vec<i32> = IkkiTomonliIter::new(vec![1, 2, 3, 4, 5])
        .rev()
        .collect();
    println!("{:?}", teskari);
    // [5, 4, 3, 2, 1]

    // .rev().map() — teskari tartibda o'zgartirish
    // .rev().map() — преобразование в обратном порядке
    let teskari_kvadrat: Vec<i32> = IkkiTomonliIter::new(vec![1, 2, 3, 4, 5])
        .rev()
        .map(|x| x * x)
        .collect();
    println!("{:?}", teskari_kvadrat);
    // [25, 16, 9, 4, 1]

    // built-in — Vec, slice — DoubleEndedIterator
    // встроенный — Vec, slice — DoubleEndedIterator
    let v: Vec<i32> = vec![10, 20, 30, 40, 50];
    let teskari_v: Vec<&i32> = v.iter().rev().collect();
    println!("{:?}", teskari_v);
    // [50, 40, 30, 20, 10]

    // .rfind() — orqadan qidirish
    // .rfind() — поиск с конца
    let oxirgi_juft: Option<i32> = IkkiTomonliIter::new(vec![1, 2, 3, 4, 5, 6])
        .rfind(|&x| x % 2 == 0);
    println!("{:?}", oxirgi_juft);
    // Some(6)

    // .rposition() — orqadan indeks
    // .rposition() — индекс с конца
    let v2: Vec<i32> = vec![1, 2, 3, 2, 1];
    let pos: Option<usize> = v2.iter().rposition(|&x| x == 2);
    println!("{:?}", pos);
    // Some(3)

    // SatrIter — ikki tomondan
    // SatrIter — с двух сторон
    let matn: &str = "birinchi\nikkinchi\nuchinchi\nto'rtinchi";
    let mut satr_it = SatrIter::new(matn);
    println!("{:?}", satr_it.next());
    println!("{:?}", satr_it.next_back());
    println!("{:?}", satr_it.next());
    println!("{:?}", satr_it.next_back());
    // Some("birinchi")
    // Some("to'rtinchi")
    // Some("ikkinchi")
    // Some("uchinchi")

    // .rev() + .zip() — teskari juftlash
    // .rev() + .zip() — обратное сопоставление
    let a: Vec<i32> = vec![1, 2, 3];
    let teskari_juft: Vec<(&i32, &i32)> = a.iter().zip(a.iter().rev()).collect();
    println!("{:?}", teskari_juft);
    // [(1, 3), (2, 2), (3, 1)]
}

// FusedIterator — None qaytarilgandan keyin yana next() chaqirilsa ham None
// FusedIterator — после возврата None, повторный вызов next() также вернёт None
//
//  trait FusedIterator: Iterator {}
//
// Oddiy Iterator da None dan keyin next() — undefined behaviour
// В обычном Iterator после None вызов next() — undefined behaviour
// FusedIterator bu muammoni kafolat bilan hal qiladi
// FusedIterator решает эту проблему с гарантией

// Xavfsiz iterator — FusedIterator implement qilgan
// Безопасный итератор — реализует FusedIterator
struct XavfsizHisoblagich {
    joriy: u32,
    max: u32,
    tugadimi: bool,
}

impl XavfsizHisoblagich {
    fn new(max: u32) -> Self {
        XavfsizHisoblagich { joriy: 0, max, tugadimi: false }
    }
}

impl Iterator for XavfsizHisoblagich {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        // tugadimi bayrog'i — None kafolati
        // флаг tugadimi — гарантия None
        if self.tugadimi {
            return None;
        }
        if self.joriy < self.max {
            self.joriy += 1;
            Some(self.joriy)
        } else {
            self.tugadimi = true;
            None
        }
    }
}

// FusedIterator — bo'sh trait, faqat kafolat beradi
// FusedIterator — пустой трейт, только гарантия
impl FusedIterator for XavfsizHisoblagich {}

// Xavfli iterator — FusedIterator implement qilmagan
// Небезопасный итератор — не реализует FusedIterator
struct XavfliHisoblagich {
    joriy: u32,
    max: u32,
}

impl XavfliHisoblagich {
    fn new(max: u32) -> Self {
        XavfliHisoblagich { joriy: 0, max }
    }
}

impl Iterator for XavfliHisoblagich {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.joriy < self.max {
            self.joriy += 1;
            Some(self.joriy)
        } else {
            // joriy oshib ketadi — None dan keyin Some qaytishi mumkin!
            // joriy может переполниться — после None может вернуть Some!
            self.joriy = self.joriy.wrapping_add(1);
            None
        }
    }
}

fn fused_misollari() {

    // XavfsizHisoblagich — None dan keyin ham None
    // XavfsizHisoblagich — после None тоже None
    let mut it = XavfsizHisoblagich::new(3);
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());  // None
    println!("{:?}", it.next());  // None — kafolatlangan!
    println!("{:?}", it.next());  // None — kafolatlangan!
    // Some(1)
    // Some(2)
    // Some(3)
    // None
    // None
    // None

    // .fuse() — istalgan iteratorni FusedIterator ga aylantirish
    // .fuse() — преобразование любого итератора в FusedIterator
    let mut xavfli = XavfliHisoblagich::new(3).fuse();
    println!("{:?}", xavfli.next());
    println!("{:?}", xavfli.next());
    println!("{:?}", xavfli.next());
    println!("{:?}", xavfli.next());  // None
    println!("{:?}", xavfli.next());  // None — fuse() kafolati!
    // Some(1)
    // Some(2)
    // Some(3)
    // None
    // None

    // built-in — Vec, slice — FusedIterator implement qilgan
    // встроенный — Vec, slice — реализует FusedIterator
    let v: Vec<i32> = vec![1, 2];
    let mut v_iter = v.iter();
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());
    println!("{:?}", v_iter.next());  // None
    println!("{:?}", v_iter.next());  // None — xavfsiz!
    // Some(1)
    // Some(2)
    // None
    // None

    // peekable() — FusedIterator bilan qulay
    // peekable() — удобно с FusedIterator
    let mut peekable = XavfsizHisoblagich::new(3).peekable();
    while let Some(&keyingi) = peekable.peek() {
        println!("Peek: {}", keyingi);
        peekable.next();
    }
    // Peek: 1
    // Peek: 2
    // Peek: 3
}

// Sahifalangan iterator — uchala trait birga
// Постраничный итератор — все три трейта вместе
#[derive(Debug)]
struct SahifalanganIter {
    ma_lumotlar: Vec<String>,
    old: usize,
    orqa: usize,
    tugadi: bool,
}

impl SahifalanganIter {
    fn new(ma_lumotlar: Vec<String>) -> Self {
        let uzunlik: usize = ma_lumotlar.len();
        SahifalanganIter {
            ma_lumotlar,
            old: 0,
            orqa: uzunlik,
            tugadi: false,
        }
    }
}

impl Iterator for SahifalanganIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.tugadi || self.old >= self.orqa {
            self.tugadi = true;
            return None;
        }
        let val: String = self.ma_lumotlar[self.old].clone();
        self.old += 1;
        Some(val)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n: usize = if self.tugadi { 0 } else { self.orqa - self.old };
        (n, Some(n))
    }
}

// ExactSizeIterator — aniq o'lcham
// ExactSizeIterator — точный размер
impl ExactSizeIterator for SahifalanganIter {
    fn len(&self) -> usize {
        if self.tugadi { 0 } else { self.orqa - self.old }
    }
}

// DoubleEndedIterator — ikki tomondan
// DoubleEndedIterator — с двух сторон
impl DoubleEndedIterator for SahifalanganIter {
    fn next_back(&mut self) -> Option<String> {
        if self.tugadi || self.old >= self.orqa {
            self.tugadi = true;
            return None;
        }
        self.orqa -= 1;
        Some(self.ma_lumotlar[self.orqa].clone())
    }
}

// FusedIterator — None kafolati
// FusedIterator — гарантия None
impl FusedIterator for SahifalanganIter {}

fn main() {

    println!("=== EXACTSIZEITERATOR ===");
    exact_size_misollari();

    println!("\n=== DOUBLEENDEDITERATOR ===");
    double_ended_misollari();

    println!("\n=== FUSEDITERATOR ===");
    fused_misollari();

    println!("\n=== UCHALA BIRGA ===");

    // SahifalanganIter — uchala trait
    // SahifalanganIter — все три трейта
    let ma_lumotlar: Vec<String> = (1..=6)
        .map(|i| format!("Yozuv {}", i))
        .collect();

    let iter = SahifalanganIter::new(ma_lumotlar.clone());

    // ExactSizeIterator — len()
    // ExactSizeIterator — len()
    println!("Jami: {}", iter.len());
    // Jami: 6

    // DoubleEndedIterator — rev()
    // DoubleEndedIterator — rev()
    let teskari: Vec<String> = SahifalanganIter::new(ma_lumotlar.clone())
        .rev()
        .collect();
    println!("{:?}", teskari);
    // ["Yozuv 6", "Yozuv 5", "Yozuv 4", "Yozuv 3", "Yozuv 2", "Yozuv 1"]

    // FusedIterator — None kafolati bilan
    // FusedIterator — с гарантией None
    let mut it = SahifalanganIter::new(ma_lumotlar.clone());
    for _ in 0..8 { it.next(); }  // 6 ta element + 2 ta None
    println!("{:?}", it.next());  // None — kafolatlangan
    // None

    // take() + rev() — oxirgi 3 ta teskari
    // take() + rev() — последние 3 в обратном порядке
    let oxirgi_3_teskari: Vec<String> = SahifalanganIter::new(ma_lumotlar.clone())
        .rev()
        .take(3)
        .collect();
    println!("{:?}", oxirgi_3_teskari);
    // ["Yozuv 6", "Yozuv 5", "Yozuv 4"]

    // skip() + take() — sahifalash
    // skip() + take() — постраничная навигация
    let sahifa_2: Vec<String> = SahifalanganIter::new(ma_lumotlar.clone())
        .skip(2)
        .take(2)
        .collect();
    println!("{:?}", sahifa_2);
    // ["Yozuv 3", "Yozuv 4"]

    // enumerate() + rev() — teskari tartibda raqamlash
    // enumerate() + rev() — нумерация в обратном порядке
    for (i, val) in SahifalanganIter::new(ma_lumotlar).rev().enumerate() {
        println!("{}: {}", i + 1, val);
    }
    // 1: Yozuv 6
    // 2: Yozuv 5
    // 3: Yozuv 4
    // 4: Yozuv 3
    // 5: Yozuv 2
    // 6: Yozuv 1
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                   | Tavsif (UZ)                               | Описание (RU)                                             |
// #================================================================================================================================================#
// # |                                          EXACTSIZEITERATOR                                                                                   |
// #================================================================================================================================================#
// # |   1 | impl ExactSizeIterator for T   | Aniq o'lcham kafolati                     | Гарантия точного размера                                  |
// # |   2 | fn len(&self) -> usize         | Qolgan elementlar soni                    | Количество оставшихся элементов                           |
// # |   3 | size_hint() → (n, Some(n))     | Lower == upper bo'lishi shart             | Lower должен равняться upper                              |
// # |   4 | Vec::with_capacity(iter.len()) | Pre-allocation — samarali xotira          | Pre-allocation — эффективная память                       |
// # |   5 | is_empty()                     | len() == 0 tekshirish                     | Проверка len() == 0                                       |
// #================================================================================================================================================#
// # |                                        DOUBLEENDEDITERATOR                                                                                   |
// #================================================================================================================================================#
// # |   6 | impl DoubleEndedIterator for T | Ikki tomondan iteratsiya                  | Итерация с двух сторон                                    |
// # |   7 | fn next_back(&mut self)        | Orqadan element olish                     | Взять элемент с конца                                     |
// # |   8 | .rev()                         | Teskari tartibda iteratsiya               | Итерация в обратном порядке                               |
// # |   9 | .rfind()                       | Orqadan birinchi mos elementni topish     | Найти первый подходящий с конца                           |
// # |  10 | .rposition()                   | Orqadan birinchi mos indeksni topish      | Найти первый подходящий индекс с конца                    |
// # |  11 | old va orqa kesishmaydi        | Invariant — o'rtada to'xtaydi             | Инвариант — останавливается посередине                    |
// #================================================================================================================================================#
// # |                                          FUSEDITERATOR                                                                                       |
// #================================================================================================================================================#
// # |  12 | impl FusedIterator for T       | None kafolati — bo'sh trait               | Гарантия None — пустой трейт                              |
// # |  13 | .fuse()                        | Istalgan iteratorni fuse qilish           | Преобразование любого итератора в fused                   |
// # |  14 | None → doim None               | None qaytgandan keyin xavfsiz             | Безопасно после возврата None                             |
// # |  15 | peekable() bilan qulay         | peek() None ga tushmaydi                  | peek() не доходит до None                                 |
// #================================================================================================================================================#
// # |                                        UCHALA BIRGA                                                                                          |
// #================================================================================================================================================#
// # |  16 | ExactSize + DoubleEnded + Fused| Professional iterator                     | Профессиональный итератор                                 |
// # |  17 | .rev().take(n)                 | Oxirgi n ta teskari tartibda              | Последние n в обратном порядке                            |
// # |  18 | .skip(n).take(m)               | Sahifalash pattern                        | Паттерн постраничной навигации                            |
// # |  19 | Vec, slice, String             | Uchala trait allaqachon implement qilgan  | Все три уже реализованы                                   |
// #================================================================================================================================================#