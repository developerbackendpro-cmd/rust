// #================================================================================================================================================#
// #                                                                    EXTEND                                                                      #
// #                            EXTEND — MAVJUD KOLLEKSIYAGA ITERATOR DAN ELEMENTLAR QO'SHISH. FROMITERATOR DAN FARQI: YANGI EMAS.                  #
// #                            EXTEND — ДОБАВЛЕНИЕ ЭЛЕМЕНТОВ ИЗ ИТЕРАТОРА В СУЩЕСТВУЮЩУЮ КОЛЛЕКЦИЮ. ОТЛИЧИЕ ОТ FROMITERATOR: НЕ НОВАЯ.             #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::collections::{HashMap, HashSet, BTreeMap};

// Extend trait:
//   trait Extend<A> {
//       fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I);
//   }
//
// FromIterator — yangi kolleksiya yaratadi (collect())
// FromIterator — создаёт новую коллекцию (collect())
// Extend       — mavjud kolleksiyaga qo'shadi (extend())
// Extend       — добавляет в существующую коллекцию (extend())

fn builtin_extend_misollari() {

    // Vec — extend bilan qo'shish
    // Vec — добавление через extend
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.extend([4, 5, 6]);
    println!("{:?}", v);
    // [1, 2, 3, 4, 5, 6]

    // Vec — iterator dan qo'shish
    // Vec — добавление из итератора
    let mut v2: Vec<i32> = vec![1, 2, 3];
    v2.extend(4..=6);
    println!("{:?}", v2);
    // [1, 2, 3, 4, 5, 6]

    // Vec — filter + extend
    // Vec — filter + extend
    let mut asosiy: Vec<i32> = vec![1, 2, 3];
    let qo_shimcha: Vec<i32> = vec![4, 5, 6, 7, 8, 9, 10];
    asosiy.extend(qo_shimcha.iter().filter(|&&x| x % 2 == 0));
    println!("{:?}", asosiy);
    // [1, 2, 3, 4, 6, 8, 10]

    // Vec — map + extend
    // Vec — map + extend
    let mut natijalar: Vec<i32> = vec![1, 4, 9];
    natijalar.extend((4..=6).map(|x| x * x));
    println!("{:?}", natijalar);
    // [1, 4, 9, 16, 25, 36]

    // String — extend bilan
    // String — через extend
    let mut s: String = String::from("salom");
    s.extend(" dunyo".chars());
    println!("{}", s);
    // salom dunyo

    // String — &str dan extend
    // String — extend из &str
    let mut s2: String = String::from("Rust");
    s2.extend([" ", "tili", " ", "ajoyib"]);
    println!("{}", s2);
    // Rust tili ajoyib

    // String — Vec<char> dan extend
    // String — extend из Vec<char>
    let mut s3: String = String::from("salom");
    s3.extend(vec!['!', '!', '!']);
    println!("{}", s3);
    // salom!!!

    // HashMap — extend bilan
    // HashMap — через extend
    let mut xarita: HashMap<&str, i32> = HashMap::new();
    xarita.insert("bir", 1);
    xarita.extend([("ikki", 2), ("uch", 3), ("tort", 4)]);
    let mut sorted: Vec<(&&str, &i32)> = xarita.iter().collect();
    sorted.sort_by_key(|&(&k, _)| k);
    println!("{:?}", sorted);
    // [("bir", 1), ("ikki", 2), ("tort", 4), ("uch", 3)]

    // HashMap — boshqa HashMap dan extend
    // HashMap — extend из другой HashMap
    let mut xarita1: HashMap<&str, i32> = [("a", 1), ("b", 2)].into_iter().collect();
    let xarita2: HashMap<&str, i32> = [("c", 3), ("d", 4)].into_iter().collect();
    xarita1.extend(xarita2);
    println!("{}", xarita1.len());
    // 4

    // HashSet — extend bilan
    // HashSet — через extend
    let mut set: HashSet<i32> = HashSet::from([1, 2, 3]);
    set.extend([3, 4, 5, 6]);
    let mut set_sorted: Vec<i32> = set.into_iter().collect();
    set_sorted.sort();
    println!("{:?}", set_sorted);
    // [1, 2, 3, 4, 5, 6]

    // BTreeMap — extend bilan (tartiblangan)
    // BTreeMap — через extend (отсортированный)
    let mut btree: BTreeMap<&str, i32> = BTreeMap::new();
    btree.insert("c", 3);
    btree.extend([("a", 1), ("b", 2), ("d", 4)]);
    for (k, v) in &btree {
        print!("{}:{} ", k, v);
    }
    println!();
    // a:1 b:2 c:3 d:4
}

// Custom kolleksiya
// Пользовательская коллекция
#[derive(Debug)]
struct Stek<T> {
    ichki: Vec<T>,
    max_hajm: usize,
}

impl<T> Stek<T> {
    fn new(max_hajm: usize) -> Self {
        Stek { ichki: Vec::new(), max_hajm }
    }

    fn push(&mut self, element: T) -> bool {
        if self.ichki.len() < self.max_hajm {
            self.ichki.push(element);
            true
        } else {
            false
        }
    }

    fn pop(&mut self) -> Option<T> {
        self.ichki.pop()
    }

    fn uzunlik(&self) -> usize {
        self.ichki.len()
    }
}

// Stek uchun Extend — max_hajm ni hurmat qiladi
// Extend для Stek — уважает max_hajm
impl<T> Extend<T> for Stek<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for element in iter {
            if !self.push(element) {
                break;  // max_hajm ga yetdi — to'xtaydi
                // достигнут max_hajm — останавливается
            }
        }
    }
}

// Log kolleksiyasi — Extend bilan
// Коллекция логов — с Extend
#[derive(Debug)]
struct LogKolleksiyasi {
    xabarlar: Vec<String>,
    filtr: Option<String>,
}

impl LogKolleksiyasi {
    fn new() -> Self {
        LogKolleksiyasi { xabarlar: Vec::new(), filtr: None }
    }

    fn filtr_bilan(filtr: &str) -> Self {
        LogKolleksiyasi {
            xabarlar: Vec::new(),
            filtr: Some(filtr.to_string()),
        }
    }
}

// Extend — filtrlab qo'shish
// Extend — добавление с фильтрацией
impl Extend<String> for LogKolleksiyasi {
    fn extend<I: IntoIterator<Item = String>>(&mut self, iter: I) {
        for xabar in iter {
            if let Some(ref filtr) = self.filtr {
                if xabar.contains(filtr.as_str()) {
                    self.xabarlar.push(xabar);
                }
            } else {
                self.xabarlar.push(xabar);
            }
        }
    }
}

// &str dan ham qabul qilish uchun
// Для приёма и &str
impl<'a> Extend<&'a str> for LogKolleksiyasi {
    fn extend<I: IntoIterator<Item = &'a str>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|s| s.to_string()));
    }
}

fn custom_extend_misollari() {

    // Stek — extend bilan to'ldirish
    // Stек — заполнение через extend
    let mut stek: Stek<i32> = Stek::new(5);
    stek.extend([1, 2, 3, 4, 5, 6, 7, 8]);  // 6,7,8 sig'madi
    println!("Stek uzunligi: {}", stek.uzunlik());
    println!("{:?}", stek.ichki);
    // Stek uzunligi: 5
    // [1, 2, 3, 4, 5]

    // Stek — Range dan extend
    // Стек — extend из Range
    let mut stek2: Stek<i32> = Stek::new(3);
    stek2.extend(10..=20);
    println!("Stek2 uzunligi: {}", stek2.uzunlik());
    println!("{:?}", stek2.ichki);
    // Stek2 uzunligi: 3
    // [10, 11, 12]

    // LogKolleksiyasi — filtr bilan
    // LogKolleksiyasi — с фильтром
    let mut log = LogKolleksiyasi::filtr_bilan("XATO");
    log.extend([
        String::from("[INFO] dastur boshlandi"),
        String::from("[XATO] fayl topilmadi"),
        String::from("[WARN] xotira kam"),
        String::from("[XATO] ulanish uzildi"),
        String::from("[INFO] operatsiya tugadi"),
    ]);
    println!("{:?}", log.xabarlar);
    // ["[XATO] fayl topilmadi", "[XATO] ulanish uzildi"]

    // LogKolleksiyasi — filtr yo'q
    // LogKolleksiyasi — без фильтра
    let mut barcha_log = LogKolleksiyasi::new();
    barcha_log.extend(["birinchi", "ikkinchi", "uchinchi"]);
    println!("{:?}", barcha_log.xabarlar);
    // ["birinchi", "ikkinchi", "uchinchi"]
}

fn extend_vs_fromiterator() {

    // FromIterator — yangi yaratadi
    // FromIterator — создаёт новую
    let yangi_vec: Vec<i32> = (1..=5).collect();
    println!("{:?}", yangi_vec);
    // [1, 2, 3, 4, 5]

    // Extend — mavjudga qo'shadi
    // Extend — добавляет в существующую
    let mut mavjud_vec: Vec<i32> = vec![0];
    mavjud_vec.extend(1..=5);
    println!("{:?}", mavjud_vec);
    // [0, 1, 2, 3, 4, 5]

    // Extend — bir nechta marta chaqirish mumkin
    // Extend — можно вызывать несколько раз
    let mut v: Vec<i32> = Vec::new();
    v.extend([1, 2, 3]);
    v.extend([4, 5, 6]);
    v.extend(7..=9);
    println!("{:?}", v);
    // [1, 2, 3, 4, 5, 6, 7, 8, 9]

    // += operatori — extend bilan ekvivalent (String uchun)
    // оператор += — эквивалентно extend (для String)
    let mut s: String = String::from("salom");
    s += " dunyo";
    println!("{}", s);
    // salom dunyo

    // push_str vs extend — String uchun bir xil
    // push_str vs extend — одинаково для String
    let mut s2: String = String::from("Rust");
    s2.push_str(" tili");
    s2.extend([" ", "ajoyib"]);
    println!("{}", s2);
    // Rust tili ajoyib
}

// So'z indeksi — Extend bilan qurish
// Индекс слов — построение через Extend
fn indeks_qurish(matnlar: &[&str]) -> HashMap<String, Vec<usize>> {
    let mut indeks: HashMap<String, Vec<usize>> = HashMap::new();

    for (i, &matn) in matnlar.iter().enumerate() {
        for soz in matn.split_whitespace() {
            indeks
                .entry(soz.to_lowercase())
                .or_default()
                .extend([i]);  // extend bilan qo'shish
            // добавление через extend
        }
    }
    indeks
}

// Konfiguratsiya — bir nechta manbadan yig'ish
// Конфигурация — сбор из нескольких источников
fn konfiguratsiya_yig_ish(
    standart: &[(&str, &str)],
    foydalanuvchi: &[(&str, &str)],
) -> HashMap<String, String> {
    let mut konfiguratsiya: HashMap<String, String> = HashMap::new();

    // Avval standart
    // Сначала стандартные
    konfiguratsiya.extend(
        standart.iter().map(|&(k, v)| (k.to_string(), v.to_string()))
    );

    // Keyin foydalanuvchi — ustidan yozadi
    // Затем пользовательские — перезаписывают
    konfiguratsiya.extend(
        foydalanuvchi.iter().map(|&(k, v)| (k.to_string(), v.to_string()))
    );

    konfiguratsiya
}

// Batch qo'shish — bir nechta manbadan Vec ga
// Пакетное добавление — из нескольких источников в Vec
fn batch_qo_shish() -> Vec<i32> {
    let mut natija: Vec<i32> = Vec::new();

    // Turli manbalardan qo'shish
    // Добавление из разных источников
    natija.extend(1..=5);
    natija.extend(vec![10, 20, 30]);
    natija.extend([100, 200, 300].iter().copied());
    natija.extend((1000..=1003).filter(|x| x % 2 == 0));

    natija
}

fn main() {

    println!("=== BUILT-IN EXTEND ===");
    builtin_extend_misollari();

    println!("\n=== CUSTOM EXTEND ===");
    custom_extend_misollari();

    println!("\n=== EXTEND VS FROMITERATOR ===");
    extend_vs_fromiterator();

    println!("\n=== REAL HAYOT ===");

    // So'z indeksi
    // Индекс слов
    let matnlar: &[&str] = &[
        "rust tili xavfsiz",
        "rust tili tez",
        "rust ajoyib til",
    ];
    let mut indeks = indeks_qurish(matnlar);
    let mut rust_pos = indeks.get("rust").cloned().unwrap_or_default();
    rust_pos.sort();
    println!("'rust' topildi: {:?}", rust_pos);
    let mut tili_pos = indeks.get("tili").cloned().unwrap_or_default();
    tili_pos.sort();
    println!("'tili' topildi: {:?}", tili_pos);
    // 'rust' topildi: [0, 1, 2]
    // 'tili' topildi: [0, 1]

    // Konfiguratsiya
    // Конфигурация
    let standart: &[(&str, &str)] = &[
        ("host", "localhost"),
        ("port", "8080"),
        ("debug", "false"),
        ("timeout", "30"),
    ];
    let foydalanuvchi: &[(&str, &str)] = &[
        ("port", "3000"),
        ("debug", "true"),
    ];
    let config = konfiguratsiya_yig_ish(standart, foydalanuvchi);
    println!("{:?}", config.get("port"));
    println!("{:?}", config.get("debug"));
    println!("{:?}", config.get("host"));
    // Some("3000")      ← foydalanuvchi ustidan yozdi
    // Some("true")      ← foydalanuvchi ustidan yozdi
    // Some("localhost") ← standart qoldi

    // Batch qo'shish
    // Пакетное добавление
    let natija = batch_qo_shish();
    println!("{:?}", natija);
    // [1, 2, 3, 4, 5, 10, 20, 30, 100, 200, 300, 1000, 1002]
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                   | Tavsif (UZ)                               | Описание (RU)                                             |
// #================================================================================================================================================#
// # |                                          EXTEND ASOSLARI                                                                                     |
// #================================================================================================================================================#
// # |   1 | impl Extend<A> for T           | Mavjud T ga A elementlar qo'shish         | Добавление элементов A в существующий T                   |
// # |   2 | fn extend<I:IntoIterator>(&mut)| Iterator dan qo'shish                     | Добавление из итератора                                   |
// # |   3 | v.extend(iter)                 | Vec ga iterator dan qo'shish              | Добавление из итератора в Vec                             |
// # |   4 | s.extend(chars)                | String ga char qo'shish                   | Добавление char в String                                  |
// # |   5 | map.extend(iter)               | HashMap ga (K,V) qo'shish                 | Добавление (K,V) в HashMap                                |
// #================================================================================================================================================#
// # |                                    EXTEND VS FROMITERATOR                                                                                    |
// #================================================================================================================================================#
// # |   6 | FromIterator                   | Yangi kolleksiya yaratadi (collect)       | Создаёт новую коллекцию (collect)                         |
// # |   7 | Extend                         | Mavjud kolleksiyaga qo'shadi (extend)     | Добавляет в существующую (extend)                         |
// # |   8 | Ko'p marta chaqirish           | extend() bir necha marta chaqiriladi      | extend() вызывается несколько раз                         |
// #================================================================================================================================================#
// # |                                    CUSTOM EXTEND                                                                                             |
// #================================================================================================================================================#
// # |   9 | Maxsus mantiq                  | Extend ichida filtr yoki limit            | Фильтр или лимит внутри Extend                            |
// # |  10 | Bir nechta impl Extend<A>      | &str va String — alohida implement        | &str и String — отдельные реализации                      |
// # |  11 | break — limit                  | Hajm limitga yetganda to'xtash            | Остановка при достижении лимита                           |
// #================================================================================================================================================#
// # |                                    REAL HAYOT                                                                                                |
// #================================================================================================================================================#
// # |  12 | Bir nechta manbadan yig'ish    | Turli iteratorlardan extend               | extend из разных итераторов                               |
// # |  13 | Konfiguratsiya                 | Standart + foydalanuvchi ustidan yozish   | Стандартные + перезапись пользователем                    |
// # |  14 | Batch qo'shish                 | Ko'p marta extend — samarali              | Многократный extend — эффективно                          |
// # |  15 | So'z indeksi                   | extend([i]) — bitta elementni qo'shish    | extend([i]) — добавление одного элемента                  |
// #================================================================================================================================================#