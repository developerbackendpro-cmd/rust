// #================================================================================================================================================#
// #                                                                ITERATOR IMPL                                                                   #
// #                            O'Z ITERATOR INI YOZISH. ITERATOR TRAIT NI IMPLEMENT QILISH. NEXT() METODI ASOSIY.                                  #
// #                            НАПИСАНИЕ СОБСТВЕННОГО ИТЕРАТОРА. РЕАЛИЗАЦИЯ ТРЕЙТА ITERATOR. МЕТОД NEXT() — ОСНОВНОЙ.                              #
// #================================================================================================================================================#

// ════════════════════════════════════════════════════════════════════════════
//  VAZIFA (TASK)
// ════════════════════════════════════════════════════════════════════════════
//
//  Iterator trait — Rust iteratsiya tizimining yadrosidir.
//  O'z iterator ini yozish orqali:
//    - Maxsus ma'lumot strukturalari uchun iteratsiya qo'shiladi
//    - .map(), .filter(), .collect() va 90+ built-in metod bepul keladi
//    - for loop bilan ishlash imkoni paydo bo'ladi
//    - Lazy (dangasa) hisoblash ta'minlanadi
//
//  Трейт Iterator — ядро системы итерации в Rust.
//  Написав свой итератор вы:
//    - Добавляете итерацию для своих структур данных
//    - Бесплатно получаете .map(), .filter(), .collect() и 90+ методов
//    - Получаете возможность работы с for loop
//    - Обеспечиваете ленивые вычисления

// ════════════════════════════════════════════════════════════════════════════
//  QONUN-QOIDALAR (RULES)
// ════════════════════════════════════════════════════════════════════════════
//
//  1. MAJBURIY: faqat next() implement qilinadi — HAMMASI shu
//     ОБЯЗАТЕЛЬНО: реализуется только next() — ВСЁ через него
//
//     trait Iterator {
//         type Item;                                    ← associated type
//         fn next(&mut self) -> Option<Self::Item>;    ← yagona majburiy metod
//     }
//
//  2. next() qoidasi:
//     QOIDA: Элемент бор → Some(element) qaytariladi
//     QOIDA: Элемент tugadi → None qaytariladi
//     QOIDA: None qaytarilgandan keyin yana next() chaqirilsa — undefined behaviour
//            (FusedIterator kafolat beradi)
//
//  3. Item — associated type:
//     QOIDA: type Item = T; — har next() da qaytariladigan tur
//     QOIDA: &T, &mut T, T — istalgan tur bo'lishi mumkin
//
//  4. Holat (state):
//     QOIDA: Iterator o'z holatini &mut self orqali saqlaydi
//     QOIDA: Holat odatda struct fieldi sifatida saqlanadi
//
//  5. 90+ metod bepul:
//     QOIDA: next() implement qilinsa — map, filter, fold, collect... HAMMASI keladi
//     QOIDA: Ularni o'zingiz yozmasangiz ham ishlaydi
//
//  6. Cheksiz iterator:
//     QOIDA: Iterator cheksiz bo'lishi mumkin — None hech qachon qaytmasligi mumkin
//     QOIDA: Bunday iterator da .take(n) ishlatish kerak
//
//  7. Lazy hisoblash:
//     QOIDA: Iterator zanjiri .collect() yoki terminal metod bo'lmaguncha hisoblanmaydi
//     QOIDA: Har next() chaqiruvida faqat bir element hisoblanadi

// ════════════════════════════════════════════════════════════════════════════
//  QACHON ISHLATILADI (WHEN TO USE)
// ════════════════════════════════════════════════════════════════════════════
//
//  ✅ Custom ma'lumot strukturasi uchun iteratsiya kerak bo'lganda
//     Tree, Graph, LinkedList, Matrix — bularni iteratsiya qilish kerak bo'lganda
//
//  ✅ Maxsus tartibda iteratsiya kerak bo'lganda
//     Fibonacci, faktorial, arifmetik progressiya, geometrik progressiya
//
//  ✅ Lazy hisoblash kerak bo'lganda
//     Katta ma'lumotlar, to'liq yuklash o'rniga birin-birin qayta ishlash
//
//  ✅ Generator pattern kerak bo'lganda
//     Ma'lumotlarni yaratib chiqarish — file satrlari, network paketlari
//
//  ✅ Adaptorlar yozish kerak bo'lganda
//     Mavjud iteratorni wrapping qilib yangi xulq berish
//
//  ❌ Oddiy Vec, slice — ular allaqachon Iterator implement qilgan
//  ❌ Bir marta ishlatiladigan oddiy loop — for loop yetarli

// ════════════════════════════════════════════════════════════════════════════
//  NIMA BILAN BIRGA ISHLAYDI (WORKS WITH)
// ════════════════════════════════════════════════════════════════════════════
//
//  ✅ for loop             — IntoIterator orqali for loop da ishlatish
//  ✅ .map()               — har elementni o'zgartirish
//  ✅ .filter()            — shartga mos elementlar
//  ✅ .collect()           — Vec, HashMap, String ga yig'ish
//  ✅ .fold()              — akkumulator bilan yig'ish
//  ✅ .zip()               — boshqa iterator bilan juftlashtirish
//  ✅ .chain()             — boshqa iteratorga ulanish
//  ✅ .take(n)             — n ta element olish (cheksiz iteratorlar uchun zarur)
//  ✅ .enumerate()         — indeks bilan
//  ✅ .peekable()          — keyingisiga qarash
//  ✅ FusedIterator        — None dan keyin xavfsiz chaqirish
//  ✅ DoubleEndedIterator  — ikki tomondan iteratsiya
//  ✅ ExactSizeIterator    — aniq o'lcham berish
//  ✅ IntoIterator         — for loop integratsiyasi
//  ✅ FromIterator         — collect() integratsiyasi

// ════════════════════════════════════════════════════════════════════════════
//  INTERVYU SAVOLLARI VA JAVOBLARI
// ════════════════════════════════════════════════════════════════════════════
//
//  S: Iterator trait da nechta majburiy metod bor?
//  J: Bitta — next(). Qolgan 90+ metod default implement qilingan.
//
//  S: next() nima qaytaradi?
//  J: Option<Self::Item>. Element bor bo'lsa Some(x), tugasa None.
//
//  S: Iterator va IntoIterator farqi nima?
//  J: Iterator — next() bor, iteratsiya qilish mumkin.
//     IntoIterator — into_iter() bor, Iterator ga aylanishi mumkin.
//     for loop IntoIterator ishlatadi.
//
//  S: Cheksiz iterator xavfli emasmi?
//  J: Xavfsiz — Rust memory safe. Lekin .collect() cheksiz loop da qoladi.
//     .take(n) ishlatish kerak.
//
//  S: iter(), iter_mut(), into_iter() farqi?
//  J: iter()      → &T    (borrow, asl qoladi)
//     iter_mut()  → &mut T (mutable borrow)
//     into_iter() → T     (ownership o'tadi, asl yo'qoladi)
//
//  S: Iterator lazy degani nima?
//  J: .map().filter() zanjirlanganda hech narsa hisoblanmaydi.
//     Faqat .collect(), .sum(), .for_each() kabi terminal metod
//     chaqirilganda hisoblash boshlanadi.
//
//  S: FusedIterator nima uchun kerak?
//  J: Oddiy Iterator da None qaytarilgandan keyin yana next() chaqirilsa
//     undefined behaviour. FusedIterator None dan keyin doim None qaytarilishini
//     kafolatlaydi.
//
//  S: DoubleEndedIterator qachon kerak?
//  J: Ikki tomondan iteratsiya kerak bo'lganda — .rev() uchun.
//     next_back() implement qilinadi.
//
//  S: ExactSizeIterator qachon kerak?
//  J: Iterator nechta element qaytarishini oldindan bilganda.
//     len() metodi aniq son qaytaradi. collect() uchun pre-allocation imkoni.
//
//  S: Iterator adaptor nima?
//  J: Mavjud iteratorni wrapping qilib yangi xulq beruvchi Iterator.
//     map, filter, take, zip — bular adaptor. next() ichida boshqa iterator.next() chaqiriladi.

#![allow(dead_code, unused)]

use std::fmt;

// Holat: joriy qiymat va maksimum
// Состояние: текущее значение и максимум
struct Hisoblagich {
    joriy: u32,
    max: u32,
}

impl Hisoblagich {
    fn new(max: u32) -> Self {
        Hisoblagich { joriy: 0, max }
    }
}

// Faqat next() — shu yetarli!
// Только next() — этого достаточно!
impl Iterator for Hisoblagich {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.joriy < self.max {
            self.joriy += 1;
            Some(self.joriy)
        } else {
            None  // ← tugadi
        }
    }
}

// Cheksiz iterator — None hech qaytmaydi
// Бесконечный итератор — None никогда не возвращается
struct Fibonacci {
    joriy: u64,
    keyingi: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { joriy: 0, keyingi: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let natija: u64 = self.joriy;
        let yangi_keyingi: u64 = self.joriy + self.keyingi;
        self.joriy = self.keyingi;
        self.keyingi = yangi_keyingi;
        Some(natija)  // ← hech qachon None emas — cheksiz!
    }
}

// Ma'lumot strukturasi
// Структура данных
#[derive(Debug)]
struct MatnQolip {
    jumlalar: Vec<String>,
}

impl MatnQolip {
    fn new(jumlalar: Vec<&str>) -> Self {
        MatnQolip {
            jumlalar: jumlalar.iter().map(|s| s.to_string()).collect(),
        }
    }

    // iter() metodi — reference iterator qaytaradi
    // метод iter() — возвращает итератор ссылок
    fn iter(&self) -> MatnQolipIter<'_> {
        MatnQolipIter { qolip: self, indeks: 0 }
    }
}

// Iterator struct — holat saqlovchi
// Структура итератора — хранит состояние
struct MatnQolipIter<'a> {
    qolip: &'a MatnQolip,
    indeks: usize,
}

impl<'a> Iterator for MatnQolipIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.indeks < self.qolip.jumlalar.len() {
            let natija: &str = &self.qolip.jumlalar[self.indeks];
            self.indeks += 1;
            Some(natija)
        } else {
            None
        }
    }
}

// DoubleEndedIterator — next_back() ham implement qilinadi
// DoubleEndedIterator — реализуется и next_back()
struct QiymatlarIter {
    qiymatlar: Vec<i32>,
    old_indeks: usize,   // olddan
    orqa_indeks: usize,  // orqadan
}

impl QiymatlarIter {
    fn new(v: Vec<i32>) -> Self {
        let uzunlik: usize = v.len();
        QiymatlarIter {
            qiymatlar: v,
            old_indeks: 0,
            orqa_indeks: uzunlik,
        }
    }
}

impl Iterator for QiymatlarIter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.old_indeks < self.orqa_indeks {
            let natija: i32 = self.qiymatlar[self.old_indeks];
            self.old_indeks += 1;
            Some(natija)
        } else {
            None
        }
    }
}

// next_back() — orqadan olish
// next_back() — взять с конца
impl DoubleEndedIterator for QiymatlarIter {
    fn next_back(&mut self) -> Option<i32> {
        if self.old_indeks < self.orqa_indeks {
            self.orqa_indeks -= 1;
            Some(self.qiymatlar[self.orqa_indeks])
        } else {
            None
        }
    }
}

// ExactSizeIterator — len() qaytaradi
// ExactSizeIterator — возвращает len()
struct AniqIter {
    qiymatlar: Vec<i32>,
    pozitsiya: usize,
}

impl AniqIter {
    fn new(v: Vec<i32>) -> Self {
        AniqIter { qiymatlar: v, pozitsiya: 0 }
    }
}

impl Iterator for AniqIter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.pozitsiya < self.qiymatlar.len() {
            let n: i32 = self.qiymatlar[self.pozitsiya];
            self.pozitsiya += 1;
            Some(n)
        } else {
            None
        }
    }

    // size_hint — (minimum, Option<maksimum>)
    // size_hint — (минимум, Option<максимум>)
    fn size_hint(&self) -> (usize, Option<usize>) {
        let qolgan: usize = self.qiymatlar.len() - self.pozitsiya;
        (qolgan, Some(qolgan))
    }
}

// ExactSizeIterator — size_hint aniq bo'lsa implement qilinadi
// ExactSizeIterator — реализуется когда size_hint точен
impl ExactSizeIterator for AniqIter {
    fn len(&self) -> usize {
        self.qiymatlar.len() - self.pozitsiya
    }
}

// Adaptor — boshqa iteratorni wrapping qilib yangi xulq beradi
// Адаптор — оборачивает другой итератор, давая новое поведение
struct StepIterator<I: Iterator> {
    ichki: I,
    qadam: usize,
    hozir: usize,
}

impl<I: Iterator> StepIterator<I> {
    fn new(ichki: I, qadam: usize) -> Self {
        StepIterator { ichki, qadam, hozir: 0 }
    }
}

impl<I: Iterator> Iterator for StepIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        loop {
            let element = self.ichki.next()?;
            if self.hozir % self.qadam == 0 {
                self.hozir += 1;
                return Some(element);
            }
            self.hozir += 1;
        }
    }
}

// Juft elementlarni birlashtiruvchi adaptor
// Адаптор объединяющий парные элементы
struct JuftBirlashtirgich<I: Iterator> {
    ichki: I,
}

impl<I: Iterator> JuftBirlashtirgich<I> {
    fn new(ichki: I) -> Self {
        JuftBirlashtirgich { ichki }
    }
}

impl<I: Iterator<Item = i32>> Iterator for JuftBirlashtirgich<I> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<(i32, i32)> {
        let birinchi: i32 = self.ichki.next()?;
        let ikkinchi: i32 = self.ichki.next()?;
        Some((birinchi, ikkinchi))
    }
}

// Ikkilik daraxt
// Двоичное дерево
#[derive(Debug)]
enum Daraxt {
    Barg(i32),
    Tugun(Box<Daraxt>, i32, Box<Daraxt>),
}

impl Daraxt {
    fn barg(qiymat: i32) -> Self {
        Daraxt::Barg(qiymat)
    }

    fn tugun(chap: Daraxt, qiymat: i32, ong: Daraxt) -> Self {
        Daraxt::Tugun(Box::new(chap), qiymat, Box::new(ong))
    }

    // in-order traversal iterator — stack bilan
    // итератор обхода в порядке (in-order) — со стеком
    fn iter(&self) -> DaraxtIter<'_> {
        let mut iter = DaraxtIter { stack: Vec::new() };
        iter.chap_ga_tush(self);
        iter
    }
}

struct DaraxtIter<'a> {
    stack: Vec<&'a Daraxt>,
}

impl<'a> DaraxtIter<'a> {
    fn chap_ga_tush(&mut self, mut tugun: &'a Daraxt) {
        loop {
            self.stack.push(tugun);
            match tugun {
                Daraxt::Tugun(chap, _, _) => tugun = chap,
                Daraxt::Barg(_)           => break,
            }
        }
    }
}

impl<'a> Iterator for DaraxtIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        let tugun: &Daraxt = self.stack.pop()?;
        match tugun {
            Daraxt::Barg(q) => Some(*q),
            Daraxt::Tugun(_, q, ong) => {
                self.chap_ga_tush(ong);
                Some(*q)
            }
        }
    }
}

// IntoIterator — for loop da ishlatish uchun
// IntoIterator — для использования в for loop
struct Royxat {
    elementlar: Vec<String>,
}

impl Royxat {
    fn new(elementlar: Vec<&str>) -> Self {
        Royxat {
            elementlar: elementlar.iter().map(|s| s.to_string()).collect(),
        }
    }
}

// &Ro'yxat uchun — reference iteratsiya
// Для &Ro'yxat — итерация по ссылке
impl<'a> IntoIterator for &'a Royxat {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.elementlar.iter()
    }
}

// Ro'yxat uchun — ownership iteratsiya
// Для Ro'yxat — итерация с владением
impl IntoIterator for Royxat {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.elementlar.into_iter()
    }
}

fn main() {

    // for loop — IntoIterator orqali
    // for loop — через IntoIterator
    let hisob = Hisoblagich::new(5);
    for x in hisob {
        print!("{} ", x);
    }
    println!();
    // 1 2 3 4 5

    // 90+ metod bepul keladi!
    // 90+ методов бесплатно!
    let kvadratlar: Vec<u32> = Hisoblagich::new(5)
        .map(|x| x * x)
        .collect();
    println!("{:?}", kvadratlar);
    // [1, 4, 9, 16, 25]

    let yig_indi: u32 = Hisoblagich::new(5).sum();
    println!("{}", yig_indi);
    // 15

    let juft_soni: usize = Hisoblagich::new(10)
        .filter(|x| x % 2 == 0)
        .count();
    println!("{}", juft_soni);
    // 5

    // zip — ikki hisoblagich
    // zip — два счётчика
    let juftlar: Vec<(u32, u32)> = Hisoblagich::new(3)
        .zip(Hisoblagich::new(3).skip(1))
        .collect();
    println!("{:?}", juftlar);
    // [(1, 2), (2, 3), (3, 4)] — oxirgisi chiqmasligi mumkin chunki uzunliklar farqli

    // .take(n) — cheksiz iteratordan n ta element
    // .take(n) — n элементов из бесконечного итератора
    let fib10: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{:?}", fib10);
    // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

    // birinchi 100 dan katta Fibonacci
    // первое число Фибоначчи больше 100
    let birinchi_katta: Option<u64> = Fibonacci::new().find(|&x| x > 100);
    println!("{:?}", birinchi_katta);
    // Some(144)

    // 20 ta Fibonacci soni yig'indisi
    // сумма 20 чисел Фибоначчи
    let yig_indi_fib: u64 = Fibonacci::new().take(20).sum();
    println!("{}", yig_indi_fib);
    // 10945

    let qolip = MatnQolip::new(vec![
        "Rust — xavfsiz",
        "Rust — tez",
        "Rust — zamonaviy",
    ]);

    // iter() bilan
    // с iter()
    for jumla in qolip.iter() {
        println!("{}", jumla);
    }
    // Rust — xavfsiz
    // Rust — tez
    // Rust — zamonaviy

    // map + collect
    // map + collect
    let katta: Vec<String> = qolip.iter()
        .map(|s| s.to_uppercase())
        .collect();
    println!("{:?}", katta);
    // ["RUST — XAVFSIZ", "RUST — TEZ", "RUST — ZAMONAVIY"]

    let iter = QiymatlarIter::new(vec![1, 2, 3, 4, 5]);

    // .rev() — teskari tartib
    // .rev() — обратный порядок
    let teskari: Vec<i32> = iter.rev().collect();
    println!("{:?}", teskari);
    // [5, 4, 3, 2, 1]

    // oldindan ham, orqadan ham
    // и спереди и сзади
    let mut iter2 = QiymatlarIter::new(vec![1, 2, 3, 4, 5]);
    println!("{:?}", iter2.next());
    println!("{:?}", iter2.next_back());
    println!("{:?}", iter2.next());
    println!("{:?}", iter2.next_back());
    // Some(1)
    // Some(5)
    // Some(2)
    // Some(4)

    let mut aniq = AniqIter::new(vec![10, 20, 30, 40, 50]);
    println!("Uzunlik: {}", aniq.len());
    aniq.next();
    aniq.next();
    println!("Uzunlik (2 ta oldi): {}", aniq.len());
    // Uzunlik: 5
    // Uzunlik (2 ta oldi): 3

    // StepIterator — har n chi element
    // StepIterator — каждый n-й элемент
    let step_iter = StepIterator::new(0..20, 3);
    let natija: Vec<i32> = step_iter.collect();
    println!("{:?}", natija);
    // [0, 3, 6, 9, 12, 15, 18]

    // JuftBirlashtirgich — 2 tadan juftlashtirish
    // JuftBirlashtirgich — объединение по 2
    let juft_iter = JuftBirlashtirgich::new(vec![1, 2, 3, 4, 5, 6].into_iter());
    let juftlar2: Vec<(i32, i32)> = juft_iter.collect();
    println!("{:?}", juftlar2);
    // [(1, 2), (3, 4), (5, 6)]

    //       4
    //      / \
    //     2   6
    //    / \ / \
    //   1  3 5  7
    let daraxt = Daraxt::tugun(
        Daraxt::tugun(Daraxt::barg(1), 2, Daraxt::barg(3)),
        4,
        Daraxt::tugun(Daraxt::barg(5), 6, Daraxt::barg(7)),
    );

    // in-order traversal — tartiblangan chiqish
    // обход in-order — отсортированный вывод
    let tartib: Vec<i32> = daraxt.iter().collect();
    println!("{:?}", tartib);
    // [1, 2, 3, 4, 5, 6, 7]

    // Iterator metodlari daraxt ustida ham ishlaydi!
    // Методы Iterator работают и с деревом!
    let juft_tugunlar: Vec<i32> = daraxt.iter()
        .filter(|&x| x % 2 == 0)
        .collect();
    println!("{:?}", juft_tugunlar);
    // [2, 4, 6]

    let royxat = Royxat::new(vec!["bir", "ikki", "uch"]);

    // &Ro'yxat — reference for loop
    // &Ro'yxat — for loop по ссылке
    for element in &royxat {
        print!("{} ", element);
    }
    println!();
    // bir ikki uch

    // Ro'yxat — ownership for loop
    // Ro'yxat — for loop с владением
    for element in royxat {
        print!("{} ", element);
    }
    println!();
    // bir ikki uch

    // Bu zanjir hech narsa hisoblamaydi!
    // Эта цепочка ничего не вычисляет!
    let _zanjir = Hisoblagich::new(1_000_000)
        .map(|x| { x * x })        // hisob yo'q
        .filter(|x| x % 2 == 0)    // hisob yo'q
        .take(5);                   // hisob yo'q
    // Faqat collect() yoki terminal metod chaqirilganda hisob boshlanadi!
    // Вычисление начинается только при вызове collect() или терминального метода!

    let natija2: Vec<u32> = Hisoblagich::new(1_000_000)
        .map(|x| x * x)
        .filter(|x| x % 2 == 0)
        .take(5)
        .collect();  // ← SHU YERDA hisoblash boshlanadi
    println!("{:?}", natija2);
    // [4, 16, 36, 64, 100]
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                 | Tavsif (UZ)                              | Описание (RU)                                                |
// #================================================================================================================================================#
// # |                                         ITERATOR IMPL ASOSLARI                                                                               |
// #================================================================================================================================================#
// # |   1 | type Item = T;               | Har next() da qaytariladigan tur          | Тип возвращаемый каждым next()                              |
// # |   2 | fn next(&mut self)->Option<T>| Yagona majburiy metod                     | Единственный обязательный метод                             |
// # |   3 | Some(element)                | Element bor — qaytariladi                 | Есть элемент — возвращается                                 |
// # |   4 | None                         | Tugadi — iterator tamom                   | Закончились — итератор завершён                             |
// # |   5 | 90+ metod bepul              | next() implementdan so'ng                 | После реализации next()                                     |
// #================================================================================================================================================#
// # |                                         KENGAYTIRILGAN TRAITLAR                                                                              |
// #================================================================================================================================================#
// # |   6 | DoubleEndedIterator          | next_back() — orqadan olish               | next_back() — взять с конца                                 |
// # |   7 | ExactSizeIterator            | len() — aniq o'lcham                      | len() — точный размер                                       |
// # |   8 | FusedIterator                | None dan keyin xavfsiz chaqirish          | Безопасный вызов после None                                 |
// # |   9 | size_hint()                  | (min, Option<max>) — taxminiy o'lcham     | (min, Option<max>) — приблизительный размер                 |
// #================================================================================================================================================#
// # |                                         INTOITERATOR                                                                                         |
// #================================================================================================================================================#
// # |  10 | impl IntoIterator for T      | for loop integratsiyasi                   | Интеграция с for loop                                       |
// # |  11 | impl IntoIterator for &T     | Reference for loop                        | For loop по ссылке                                          |
// # |  12 | impl IntoIterator for &mut T | Mutable reference for loop                | For loop по мутабельной ссылке                              |
// #================================================================================================================================================#
// # |                                         QACHON NIMA ISHLATISH                                                                                |
// #================================================================================================================================================#
// # |  13 | Cheksiz iterator             | None qaytarilmaydi + .take(n) shart       | None не возвращается + .take(n) обязательно                 |
// # |  14 | Adaptor pattern              | ichki: I field + I::Item qaytarish        | поле ichki: I + возврат I::Item                             |
// # |  15 | Daraxt/Graf iterator         | Stack yordamida holat saqlash             | Хранение состояния через Stack                              |
// # |  16 | Lazy hisoblash               | Terminal metod bo'lguncha hech narsa yo'q | Ничего нет до терминального метода                          |
// #================================================================================================================================================#