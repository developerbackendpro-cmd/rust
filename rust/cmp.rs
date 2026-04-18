// #================================================================================================================================================#
// #                                                                STD::CMP                                                                        #
// #                            STD::CMP — TAQQOSLASH TRAITLARI VA FUNKSIYALARI. PARTIALEQ, EQ, PARTIALORD, ORD, ORDERING.                          #
// #                            STD::CMP — ТРЕЙТЫ И ФУНКЦИИ СРАВНЕНИЯ. PARTIALEQ, EQ, PARTIALORD, ORD, ORDERING.                                    #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::cmp::{Ordering, min, max, min_by, max_by, min_by_key, max_by_key};
use std::fmt;

// std::cmp ichida nima bor:
// Что внутри std::cmp:
//
//   PartialEq — == va != operatorlari
//   PartialEq — операторы == и !=
//   Eq        — PartialEq + to'liq tenglik kafolati (reflexive)
//   Eq        — PartialEq + гарантия полного равенства (рефлексивность)
//   PartialOrd— <, >, <=, >= operatorlari (ba'zi juftlar taqqoslanmasligi mumkin)
//   PartialOrd— операторы <, >, <=, >= (некоторые пары могут быть несравнимы)
//   Ord       — PartialOrd + barcha qiymatlar taqqoslanadi
//   Ord       — PartialOrd + все значения сравнимы
//   Ordering  — Less, Equal, Greater
//   Ordering  — Less, Equal, Greater
//
// PartialEq vs Eq:
//   f64: PartialEq lekin Eq EMAS — NaN != NaN
//   f64: PartialEq но НЕ Eq — NaN != NaN
//   i32: ham PartialEq ham Eq
//   i32: и PartialEq и Eq
//
// PartialOrd vs Ord:
//   f64: PartialOrd lekin Ord EMAS — NaN taqqoslanmaydi
//   f64: PartialOrd но НЕ Ord — NaN несравним
//   i32: ham PartialOrd ham Ord
//   i32: и PartialOrd и Ord

fn ordering_misollari() {

    // Ordering enum — uch variant
    // Enum Ordering — три варианта
    println!("{:?}", Ordering::Less);
    println!("{:?}", Ordering::Equal);
    println!("{:?}", Ordering::Greater);
    // Less
    // Equal
    // Greater

    // cmp() — Ord turlar uchun
    // cmp() — для типов Ord
    println!("{:?}", 1i32.cmp(&2));   // Less
    println!("{:?}", 2i32.cmp(&2));   // Equal
    println!("{:?}", 3i32.cmp(&2));   // Greater
    // Less
    // Equal
    // Greater

    // partial_cmp() — PartialOrd turlar uchun
    // partial_cmp() — для типов PartialOrd
    println!("{:?}", 1.0f64.partial_cmp(&2.0)); // Some(Less)
    println!("{:?}", f64::NAN.partial_cmp(&1.0));// None  ← NaN taqqoslanmaydi
    // Some(Less)
    // None

    // Ordering metodlari
    // Методы Ordering
    let ord: Ordering = Ordering::Less;
    println!("{}", ord.is_lt()); // true
    println!("{}", ord.is_gt()); // false
    println!("{}", ord.is_eq()); // false
    println!("{}", ord.is_le()); // true
    println!("{}", ord.is_ge()); // false
    // true
    // false
    // false
    // true
    // false

    // reverse() — tartibni teskari qilish
    // reverse() — обращение порядка
    println!("{:?}", Ordering::Less.reverse());    // Greater
    println!("{:?}", Ordering::Equal.reverse());   // Equal
    println!("{:?}", Ordering::Greater.reverse()); // Less
    // Greater
    // Equal
    // Less

    // then() — birinchi Equal bo'lsa ikkinchisini ishlatish
    // then() — если первый Equal, использовать второй
    let a: Ordering = Ordering::Equal;
    let b: Ordering = Ordering::Less;
    println!("{:?}", a.then(b)); // Less
    println!("{:?}", b.then(a)); // Less  ← b allaqachon Less, a e'tiborga olinmaydi
    // Less
    // Less

    // then_with() — lazy version
    // then_with() — ленивая версия
    let natija: Ordering = Ordering::Equal.then_with(|| {
        println!("Ikkinchi taqqoslash...");
        Ordering::Greater
    });
    println!("{:?}", natija);
    // Ikkinchi taqqoslash...
    // Greater
}

fn partialeq_eq_misollari() {

    // PartialEq — == va != operatorlari
    // PartialEq — операторы == и !=
    println!("{}", 42 == 42);
    println!("{}", 42 != 43);
    println!("{}", "salom" == "salom");
    println!("{}", vec![1,2,3] == vec![1,2,3]);
    // true
    // true
    // true
    // true

    // NaN — PartialEq lekin Eq emas
    // NaN — PartialEq но не Eq
    let nan: f64 = f64::NAN;
    println!("{}", nan == nan);  // false! NaN != NaN
    println!("{}", nan != nan);  // true!
    // false
    // true

    // Custom PartialEq
    // Пользовательский PartialEq
    #[derive(Debug)]
    struct Temperatur {
        kelvin: f64,
    }

    impl PartialEq for Temperatur {
        fn eq(&self, b: &Self) -> bool {
            // 0.001 aniqlikda tenglik
            // Равенство с точностью 0.001
            (self.kelvin - b.kelvin).abs() < 0.001
        }
    }

    let t1: Temperatur = Temperatur { kelvin: 273.15 };
    let t2: Temperatur = Temperatur { kelvin: 273.150001 };
    let t3: Temperatur = Temperatur { kelvin: 300.0 };
    println!("{}", t1 == t2); // true  (0.001 dan kichik farq)
    println!("{}", t1 == t3); // false
    // true
    // false

    // #[derive(PartialEq)] — avtomatik implement
    // #[derive(PartialEq)] — автоматическая реализация
    #[derive(Debug, PartialEq)]
    struct Nuqta { x: i32, y: i32 }

    let n1: Nuqta = Nuqta { x: 1, y: 2 };
    let n2: Nuqta = Nuqta { x: 1, y: 2 };
    let n3: Nuqta = Nuqta { x: 3, y: 4 };
    println!("{}", n1 == n2); // true
    println!("{}", n1 == n3); // false
    println!("{}", n1 != n3); // true
    // true
    // false
    // true
}

fn partialord_ord_misollari() {

    // Ord — to'liq tartibli turlar
    // Ord — типы с полным порядком
    let mut v: Vec<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6];
    v.sort(); // Ord kerak
    println!("{:?}", v);
    // [1, 1, 2, 3, 4, 5, 6, 9]

    // sort_by — custom taqqoslash
    // sort_by — пользовательское сравнение
    let mut v2: Vec<i32> = vec![3, 1, 4, 1, 5, 9];
    v2.sort_by(|a, b| b.cmp(a)); // teskari tartib
    println!("{:?}", v2);
    // [9, 5, 4, 3, 1, 1]

    // sort_by_key — kalit bo'yicha tartiblash
    // sort_by_key — сортировка по ключу
    let mut sozlar: Vec<&str> = vec!["banan", "olma", "anor", "nok"];
    sozlar.sort_by_key(|s| s.len());
    println!("{:?}", sozlar);
    // ["nok", "olma", "anor", "banan"]

    // Custom Ord
    // Пользовательский Ord
    #[derive(Debug, Eq, PartialEq)]
    struct Talaba {
        ism: String,
        baho: u32,
    }

    impl PartialOrd for Talaba {
        fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
            Some(self.cmp(b))
        }
    }

    impl Ord for Talaba {
        fn cmp(&self, b: &Self) -> Ordering {
            // Avval baho, keyin ism
            // Сначала оценка, потом имя
            b.baho.cmp(&self.baho)
                .then_with(|| self.ism.cmp(&b.ism))
        }
    }

    let mut talabalar: Vec<Talaba> = vec![
        Talaba { ism: "Dilshod".to_string(), baho: 85 },
        Talaba { ism: "Ali".to_string(),     baho: 92 },
        Talaba { ism: "Vali".to_string(),    baho: 85 },
        Talaba { ism: "Soli".to_string(),    baho: 78 },
    ];

    talabalar.sort();
    for t in &talabalar {
        println!("{}: {}", t.ism, t.baho);
    }
    // Ali: 92
    // Dilshod: 85
    // Vali: 85
    // Soli: 78

    // PartialOrd — f64 uchun
    // PartialOrd — для f64
    let mut floatlar: Vec<f64> = vec![3.14, 1.0, 2.72, 0.5];
    floatlar.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:?}", floatlar);
    // [0.5, 1.0, 2.72, 3.14]
}

fn cmp_funksiyalari() {

    // min() va max() — Ord uchun
    // min() и max() — для Ord
    println!("{}", min(3, 7));  // 3
    println!("{}", max(3, 7));  // 7
    println!("{}", min('a', 'z')); // a
    println!("{}", max("salom", "dunyo")); // salom (leksikografik)
    // 3
    // 7
    // a
    // salom

    // min_by() va max_by() — custom taqqoslash
    // min_by() и max_by() — пользовательское сравнение
    let a: f64 = 3.14;
    let b: f64 = 2.72;
    let kichik: f64 = min_by(a, b, |x, y| x.partial_cmp(y).unwrap());
    let katta: f64  = max_by(a, b, |x, y| x.partial_cmp(y).unwrap());
    println!("{} {}", kichik, katta);
    // 2.72 3.14

    // min_by_key() va max_by_key() — kalit bo'yicha
    // min_by_key() и max_by_key() — по ключу
    let sozlar: &[&str] = &["banan", "olma", "anor"];
    let eng_qisqa: &str = min_by_key(sozlar[0], sozlar[1], |s: &&str| s.len());
    let eng_uzun: &str  = max_by_key(sozlar[0], sozlar[2], |s: &&str| s.len());
    println!("{} {}", eng_qisqa, eng_uzun);
    // olma banan

    // clamp() — diapazonga cheklash
    // clamp() — ограничение диапазоном
    println!("{}", 5i32.clamp(0, 10));   // 5  (ichida)
    println!("{}", (-5i32).clamp(0, 10));  // 0  (minimumdan kichik)
    println!("{}", 15i32.clamp(0, 10));  // 10 (maksimumdan katta)
    // 5
    // 0
    // 10

    // clamp — f32 bilan
    // clamp — с f32
    println!("{}", 0.5f32.clamp(0.0, 1.0));  // 0.5
    println!("{}", (-0.1f32).clamp(0.0, 1.0)); // 0.0
    println!("{}", 1.5f32.clamp(0.0, 1.0));  // 1.0
    // 0.5
    // 0.0
    // 1.0

    // Iterator metodlari — Ord uchun
    // Методы итератора — для Ord
    let v: Vec<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("{:?}", v.iter().min()); // Some(1)
    println!("{:?}", v.iter().max()); // Some(9)
    // Some(1)
    // Some(9)

    // min_by_key va max_by_key — Iterator da
    // min_by_key и max_by_key — в итераторе
    let sozlar2: Vec<&str> = vec!["banan", "olma", "anor", "nok"];
    println!("{:?}", sozlar2.iter().min_by_key(|s| s.len())); // Some("nok")
    println!("{:?}", sozlar2.iter().max_by_key(|s| s.len())); // Some("banan")
    // Some("nok")
    // Some("banan")
}

fn real_hayot_misollari() {

    // 1. Ko'p mezonli tartiblash
    // 1. Многокритериальная сортировка
    #[derive(Debug)]
    struct Mahsulot {
        nomi: String,
        narx: f64,
        reyting: f64,
        sotilgan: u32,
    }

    impl Mahsulot {
        fn new(nomi: &str, narx: f64, reyting: f64, sotilgan: u32) -> Self {
            Mahsulot { nomi: nomi.to_string(), narx, reyting, sotilgan }
        }
    }

    let mut mahsulotlar: Vec<Mahsulot> = vec![
        Mahsulot::new("Olma",    1500.0, 4.5, 100),
        Mahsulot::new("Banan",   3000.0, 4.8, 80),
        Mahsulot::new("Anor",    2500.0, 4.5, 120),
        Mahsulot::new("Uzum",    4000.0, 4.2, 60),
        Mahsulot::new("Nok",     2000.0, 4.8, 90),
    ];

    // Avval reyting (kamayib), keyin narx (o'sib), keyin nom
    // Сначала рейтинг (убывающий), затем цена (возрастающая), затем имя
    mahsulotlar.sort_by(|a, b| {
        b.reyting.partial_cmp(&a.reyting).unwrap()
            .then_with(|| a.narx.partial_cmp(&b.narx).unwrap())
            .then_with(|| a.nomi.cmp(&b.nomi))
    });

    for m in &mahsulotlar {
        println!("{:<10} narx:{:<8.0} reyting:{:.1}", m.nomi, m.narx, m.reyting);
    }
    // Nok        narx:2000     reyting:4.8
    // Banan      narx:3000     reyting:4.8
    // Anor       narx:2500     reyting:4.5
    // Olma       narx:1500     reyting:4.5
    // Uzum       narx:4000     reyting:4.2

    // 2. Binary search — Ord bilan
    // 2. Бинарный поиск — с Ord
    let tartiblangan: Vec<i32> = vec![1, 3, 5, 7, 9, 11, 13, 15];
    println!("{:?}", tartiblangan.binary_search(&7));   // Ok(3)
    println!("{:?}", tartiblangan.binary_search(&6));   // Err(3) — kiritilishi kerak bo'lgan joy
    // Ok(3)
    // Err(3)

    // binary_search_by — custom taqqoslash
    // binary_search_by — пользовательское сравнение
    let natija = tartiblangan.binary_search_by(|&x| x.cmp(&7));
    println!("{:?}", natija);
    // Ok(3)

    // binary_search_by_key — kalit bo'yicha
    // binary_search_by_key — по ключу
    let juftlar: Vec<(i32, &str)> = vec![(1,"bir"), (3,"uch"), (5,"besh"), (7,"yetti")];
    let natija2 = juftlar.binary_search_by_key(&5, |&(k, _)| k);
    println!("{:?}", natija2);
    // Ok(2)

    // 3. clamp — sensor ma'lumotlari
    // 3. clamp — данные датчиков
    fn sensor_qayta_ishlash(qiymat: f64, min_q: f64, max_q: f64) -> f64 {
        let cheklangan: f64 = qiymat.clamp(min_q, max_q);
        let normalangan: f64 = (cheklangan - min_q) / (max_q - min_q);
        (normalangan * 100.0).round() / 100.0
    }

    println!("{}", sensor_qayta_ishlash(75.0, 0.0, 100.0));   // 0.75
    println!("{}", sensor_qayta_ishlash(-10.0, 0.0, 100.0));  // 0.0
    println!("{}", sensor_qayta_ishlash(150.0, 0.0, 100.0));  // 1.0
    // 0.75
    // 0.0
    // 1.0

    // 4. Versiya taqqoslash — custom Ord
    // 4. Сравнение версий — custom Ord
    #[derive(Debug, Eq, PartialEq)]
    struct Versiya {
        major: u32,
        minor: u32,
        patch: u32,
    }

    impl Versiya {
        fn new(major: u32, minor: u32, patch: u32) -> Self {
            Versiya { major, minor, patch }
        }
    }

    impl fmt::Display for Versiya {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
        }
    }

    impl PartialOrd for Versiya {
        fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
            Some(self.cmp(b))
        }
    }

    impl Ord for Versiya {
        fn cmp(&self, b: &Self) -> Ordering {
            self.major.cmp(&b.major)
                .then_with(|| self.minor.cmp(&b.minor))
                .then_with(|| self.patch.cmp(&b.patch))
        }
    }

    let mut versiyalar: Vec<Versiya> = vec![
        Versiya::new(1, 2, 0),
        Versiya::new(2, 0, 0),
        Versiya::new(1, 0, 5),
        Versiya::new(1, 1, 0),
        Versiya::new(2, 0, 1),
    ];

    versiyalar.sort();
    for v in &versiyalar {
        println!("{}", v);
    }
    // 1.0.5
    // 1.1.0
    // 1.2.0
    // 2.0.0
    // 2.0.1

    let eng_yangi: &Versiya = versiyalar.iter().max().unwrap();
    println!("Eng yangi: {}", eng_yangi);
    // Eng yangi: 2.0.1
}

fn main() {

    println!("=== ORDERING ===");
    ordering_misollari();

    println!("\n=== PARTIALEQ VA EQ ===");
    partialeq_eq_misollari();

    println!("\n=== PARTIALORD VA ORD ===");
    partialord_ord_misollari();

    println!("\n=== CMP FUNKSIYALARI ===");
    cmp_funksiyalari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                | Tavsif (UZ)                                | Описание (RU)                                               |
// #================================================================================================================================================#
// # |                                        TRAITLAR                                                                                              |
// #================================================================================================================================================#
// # |   1 | #[derive(PartialEq)]        | == va != operatorlari                      | Операторы == и !=                                           |
// # |   2 | #[derive(Eq)]               | PartialEq + reflexive kafolat              | PartialEq + гарантия рефлексивности                         |
// # |   3 | #[derive(PartialOrd)]       | <, >, <=, >= (ba'zi juftlar None)          | <, >, <=, >= (некоторые пары None)                          |
// # |   4 | #[derive(Ord)]              | To'liq tartibli — barcha juftlar           | Полный порядок — все пары                                   |
// # |   5 | fn cmp(&self, b) -> Ordering| Ord uchun majburiy metod                   | Обязательный метод для Ord                                  |
// #================================================================================================================================================#
// # |                                        ORDERING                                                                                              |
// #================================================================================================================================================#
// # |   6 | Ordering::Less/Equal/Greater| Taqqoslash natijasi                        | Результат сравнения                                         |
// # |   7 | .is_lt() .is_gt() .is_eq()  | Ordering holatini tekshirish               | Проверка состояния Ordering                                 |
// # |   8 | .reverse()                  | Tartibni teskari qilish                    | Обращение порядка                                           |
// # |   9 | .then(other)                | Equal bo'lsa keyingisini ishlatish         | Если Equal — использовать следующий                         |
// # |  10 | .then_with(|| ...)          | Lazy then                                  | Ленивый then                                                |
// #================================================================================================================================================#
// # |                                        FUNKSIYALAR                                                                                           |
// #================================================================================================================================================#
// # |  11 | min(a, b)                   | Kichigini qaytarish                        | Вернуть меньший                                             |
// # |  12 | max(a, b)                   | Kattasini qaytarish                        | Вернуть больший                                             |
// # |  13 | min_by(a, b, |x,y| ...)     | Custom taqqoslash bilan kichik             | Меньший с пользовательским сравнением                       |
// # |  14 | max_by_key(a, b, |x| ...)   | Kalit bo'yicha katta                       | Больший по ключу                                            |
// # |  15 | clamp(val, min, max)        | Qiymatni diapazonga cheklash               | Ограничение значения диапазоном                             |
// #================================================================================================================================================#
// # |                                        QACHON NIMA ISHLATISH                                                                                 |
// #================================================================================================================================================#
// # |  16 | sort_by(|a,b| a.cmp(b))     | Custom tartiblash                          | Пользовательская сортировка                                 |
// # |  17 | .then_with() zanjiri        | Ko'p mezonli tartiblash                    | Многокритериальная сортировка                               |
// # |  18 | binary_search()             | Tartiblangan Vec da qidirish O(log n)      | Поиск в отсортированном Vec O(log n)                        |
// # |  19 | NaN — Eq emas               | f64 HashSet/HashMap kaliti bo'lolmaydi     | f64 не может быть ключом HashSet/HashMap                    |
// # |  20 | Reverse — teskari tartib    | BinaryHeap min-heap uchun                  | Для min-heap в BinaryHeap                                   |
// #================================================================================================================================================#