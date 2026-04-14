// #================================================================================================================================================#
// #                                                                    ITERATORS                                                                   #
// #                                ITERATOR — ELEMENTLARNI KETMA-KET QAYTA ISHLASH. LAZY — FAQAT KERAK BO'LGANDA HISOBLAYDI.                       #
// #                                ITERATOR — ПОСЛЕДОВАТЕЛЬНАЯ ОБРАБОТКА ЭЛЕМЕНТОВ. LAZY — ВЫЧИСЛЯЕТ ТОЛЬКО КОГДА НУЖНО.                           #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::collections::HashMap;

// Iterator trait:
//   trait Iterator {
//       type Item;
//       fn next(&mut self) -> Option<Self::Item>;
//   }
//
// Lazy — next() chaqirilmaguncha hech narsa hisoblanmaydi
// Lazy — ничего не вычисляется пока не вызван next()
//
// into_iter() — ownership oladi
// iter()      — &T reference beradi
// iter_mut()  — &mut T reference beradi

fn iterator_yaratish_misollari() {

    // Vec — iter(), iter_mut(), into_iter()
    // Vec — iter(), iter_mut(), into_iter()
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];

    // iter() — &T beradi
    // iter() — даёт &T
    let mut it = v.iter();
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    // Some(1)
    // Some(2)
    // Some(3)

    // into_iter() — T beradi (ownership)
    // into_iter() — даёт T (владение)
    let v2: Vec<i32> = vec![10, 20, 30];
    let mut it2 = v2.into_iter();
    println!("{:?}", it2.next());
    println!("{:?}", it2.next());
    // Some(10)
    // Some(20)

    // Range iterator
    // Итератор диапазона
    let mut range_it = (1..=5).into_iter();
    println!("{:?}", range_it.next());
    println!("{:?}", range_it.next());
    // Some(1)
    // Some(2)

    // for loop — iterator ishlatadi
    // for loop — использует итератор
    for x in &v {
        print!("{} ", x);
    }
    println!();
    // 1 2 3 4 5

    // chars() — string iterator
    // chars() — итератор строки
    let mut chars_it = "salom".chars();
    println!("{:?}", chars_it.next());
    println!("{:?}", chars_it.next());
    // Some('s')
    // Some('a')

    // lines() — qator iteratori
    // lines() — итератор строк
    let matn: &str = "birinchi\nikkinchi\nuchinchi";
    let qatorlar: Vec<&str> = matn.lines().collect();
    println!("{:?}", qatorlar);
    // ["birinchi", "ikkinchi", "uchinchi"]

    // split() — ajratuvchi bilan
    // split() — с разделителем
    let sozlar: Vec<&str> = "salom,dunyo,rust".split(',').collect();
    println!("{:?}", sozlar);
    // ["salom", "dunyo", "rust"]

    // HashMap iter
    // итератор HashMap
    let mut xarita: HashMap<&str, i32> = HashMap::new();
    xarita.insert("bir", 1);
    xarita.insert("ikki", 2);
    let mut kalit_qiymatlar: Vec<(&&str, &i32)> = xarita.iter().collect();
    kalit_qiymatlar.sort_by_key(|&(&k, _)| k);
    println!("{:?}", kalit_qiymatlar);
    // [("bir", 1), ("ikki", 2)]
}

fn map_misollari() {

    let sonlar: Vec<i32> = vec![1, 2, 3, 4, 5];

    // map — har elementni o'zgartirish
    // map — преобразование каждого элемента
    let kvadratlar: Vec<i32> = sonlar.iter()
        .map(|&x| x * x)
        .collect();
    println!("{:?}", kvadratlar);
    // [1, 4, 9, 16, 25]

    // map — tur o'zgartirish
    // map — изменение типа
    let stringlar: Vec<String> = sonlar.iter()
        .map(|x| x.to_string())
        .collect();
    println!("{:?}", stringlar);
    // ["1", "2", "3", "4", "5"]

    // map zanjiri
    // цепочка map
    let natija: Vec<i32> = (1..=5)
        .map(|x| x * 2)
        .map(|x| x + 1)
        .map(|x| x * x)
        .collect();
    println!("{:?}", natija);
    // [9, 25, 49, 81, 121]

    // map — struct ga aylantirish
    // map — преобразование в struct
    #[derive(Debug)]
    struct Talaba { ism: String, baho: i32 }
    let ismlar: Vec<&str> = vec!["Ali", "Vali", "Soli"];
    let talabalar: Vec<Talaba> = ismlar.iter()
        .enumerate()
        .map(|(i, &ism)| Talaba {
            ism: ism.to_string(),
            baho: (i as i32 + 1) * 30,
        })
        .collect();
    for t in &talabalar {
        println!("{}: {}", t.ism, t.baho);
    }
    // Ali: 30
    // Vali: 60
    // Soli: 90
}

fn filter_misollari() {

    let sonlar: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter — shartga mos elementlar
    // filter — элементы по условию
    let juftlar: Vec<&i32> = sonlar.iter()
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("{:?}", juftlar);
    // [2, 4, 6, 8, 10]

    // filter + map zanjiri
    // цепочка filter + map
    let toq_kvadratlar: Vec<i32> = sonlar.iter()
        .filter(|&&x| x % 2 != 0)
        .map(|&x| x * x)
        .collect();
    println!("{:?}", toq_kvadratlar);
    // [1, 9, 25, 49, 81]

    // String filtrlash
    // фильтрация строк
    let sozlar: Vec<&str> = vec!["salom", "dunyo", "rust", "tili", "ajoyib"];
    let uzunlar: Vec<&&str> = sozlar.iter()
        .filter(|s| s.len() > 4)
        .collect();
    println!("{:?}", uzunlar);
    // ["dunyo", "ajoyib"]

    // filter_map — filter + map birga
    // filter_map — filter + map вместе
    let aralash: Vec<&str> = vec!["1", "ikki", "3", "to'rt", "5"];
    let sonlar2: Vec<i32> = aralash.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("{:?}", sonlar2);
    // [1, 3, 5]
}

fn fold_reduce_misollari() {

    let sonlar: Vec<i32> = vec![1, 2, 3, 4, 5];

    // fold — akkumulator bilan
    // fold — с аккумулятором
    let yig_indi: i32 = sonlar.iter().fold(0, |acc, &x| acc + x);
    println!("{}", yig_indi);
    // 15

    // fold — ko'paytma
    // fold — произведение
    let ko_paytma: i32 = sonlar.iter().fold(1, |acc, &x| acc * x);
    println!("{}", ko_paytma);
    // 120

    // fold — maksimum
    // fold — максимум
    let max: i32 = sonlar.iter().fold(i32::MIN, |acc, &x| acc.max(x));
    println!("{}", max);
    // 5

    // fold — String yaratish
    // fold — создание String
    let sozlar: Vec<&str> = vec!["salom", "dunyo", "rust"];
    let birlashgan: String = sozlar.iter()
        .fold(String::new(), |mut acc, &s| {
            if !acc.is_empty() { acc.push(' '); }
            acc.push_str(s);
            acc
        });
    println!("{}", birlashgan);
    // salom dunyo rust

    // reduce — birinchi element akkumulator
    // reduce — первый элемент аккумулятор
    let yig_indi2: Option<i32> = sonlar.iter().copied().reduce(|acc, x| acc + x);
    println!("{:?}", yig_indi2);
    // Some(15)

    // sum va product — maxsus fold
    // sum и product — специальный fold
    let sum: i32 = sonlar.iter().sum();
    let product: i32 = sonlar.iter().product();
    println!("sum={}, product={}", sum, product);
    // sum=15, product=120
}

fn enumerate_zip_chain_misollari() {

    let sozlar: Vec<&str> = vec!["salom", "dunyo", "rust"];

    // enumerate — indeks bilan
    // enumerate — с индексом
    for (i, s) in sozlar.iter().enumerate() {
        println!("{}: {}", i, s);
    }
    // 0: salom
    // 1: dunyo
    // 2: rust

    // enumerate + map
    // enumerate + map
    let raqamlangan: Vec<String> = sozlar.iter()
        .enumerate()
        .map(|(i, s)| format!("{}. {}", i + 1, s))
        .collect();
    println!("{:?}", raqamlangan);
    // ["1. salom", "2. dunyo", "3. rust"]

    // zip — ikki iteratorni juftlashtirish
    // zip — объединение двух итераторов в пары
    let kalit: Vec<&str> = vec!["ism", "yosh", "shahar"];
    let qiymat: Vec<&str> = vec!["Dilshod", "22", "Toshkent"];
    let juftlar: Vec<(&&str, &&str)> = kalit.iter().zip(qiymat.iter()).collect();
    println!("{:?}", juftlar);
    // [("ism", "Dilshod"), ("yosh", "22"), ("shahar", "Toshkent")]

    // zip → HashMap
    // zip → HashMap
    let xarita: HashMap<&&str, &&str> = kalit.iter().zip(qiymat.iter()).collect();
    println!("{:?}", xarita.get(&"ism"));
    // Some("Dilshod")

    // chain — ketma-ket birlashtirish
    // chain — последовательное объединение
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![4, 5, 6];
    let c: Vec<i32> = vec![7, 8, 9];
    let birga: Vec<&i32> = a.iter().chain(b.iter()).chain(c.iter()).collect();
    println!("{:?}", birga);
    // [1, 2, 3, 4, 5, 6, 7, 8, 9]
}

fn take_skip_flatten_misollari() {

    let sonlar: Vec<i32> = (1..=10).collect();

    // take — n ta element olish
    // take — взять n элементов
    let birinchi_uch: Vec<&i32> = sonlar.iter().take(3).collect();
    println!("{:?}", birinchi_uch);
    // [1, 2, 3]

    // skip — n ta elementni o'tkazish
    // skip — пропустить n элементов
    let oxirgi_besh: Vec<&i32> = sonlar.iter().skip(5).collect();
    println!("{:?}", oxirgi_besh);
    // [6, 7, 8, 9, 10]

    // take_while — shart bajarilguncha
    // take_while — пока выполняется условие
    let kichiklar: Vec<&i32> = sonlar.iter().take_while(|&&x| x < 5).collect();
    println!("{:?}", kichiklar);
    // [1, 2, 3, 4]

    // skip_while — shart bajarilguncha o'tkazish
    // skip_while — пропускать пока выполняется условие
    let kattalar: Vec<&i32> = sonlar.iter().skip_while(|&&x| x < 5).collect();
    println!("{:?}", kattalar);
    // [5, 6, 7, 8, 9, 10]

    // flatten — ichki iteratorlarni yassilash
    // flatten — сплющивание вложенных итераторов
    let ichki_veklar: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let yassi: Vec<i32> = ichki_veklar.into_iter().flatten().collect();
    println!("{:?}", yassi);
    // [1, 2, 3, 4, 5, 6]

    // flat_map — map + flatten
    // flat_map — map + flatten
    let sozlar: Vec<&str> = vec!["salom dunyo", "rust tili"];
    let barcha_sozlar: Vec<&str> = sozlar.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("{:?}", barcha_sozlar);
    // ["salom", "dunyo", "rust", "tili"]

    // Step_by — qadam bilan
    // Step_by — с шагом
    let qadam: Vec<i32> = (0..20).step_by(3).collect();
    println!("{:?}", qadam);
    // [0, 3, 6, 9, 12, 15, 18]
}

fn qidiruv_tekshirish_misollari() {

    let sonlar: Vec<i32> = vec![3, 7, 1, 9, 4, 6, 2, 8, 5];

    // any — kamida bitta
    // any — хотя бы один
    let katta_bormi: bool = sonlar.iter().any(|&x| x > 8);
    println!("{}", katta_bormi);
    // true

    // all — hammasi
    // all — все
    let hammasi_musbat: bool = sonlar.iter().all(|&x| x > 0);
    println!("{}", hammasi_musbat);
    // true

    // find — birinchisini topish
    // find — найти первый
    let birinchi_juft: Option<&i32> = sonlar.iter().find(|&&x| x % 2 == 0);
    println!("{:?}", birinchi_juft);
    // Some(4)

    // find_map — topish + aylantirish
    // find_map — найти + преобразовать
    let aralash: Vec<&str> = vec!["bir", "2", "uch", "4"];
    let birinchi_son: Option<i32> = aralash.iter()
        .find_map(|s| s.parse::<i32>().ok());
    println!("{:?}", birinchi_son);
    // Some(2)

    // position — indeksini topish
    // position — найти индекс
    let indeks: Option<usize> = sonlar.iter().position(|&x| x == 9);
    println!("{:?}", indeks);
    // Some(3)

    // max va min
    // max и min
    let max: Option<&i32> = sonlar.iter().max();
    let min: Option<&i32> = sonlar.iter().min();
    println!("max={:?}, min={:?}", max, min);
    // max=Some(9), min=Some(1)

    // max_by_key va min_by_key
    // max_by_key и min_by_key
    let sozlar: Vec<&str> = vec!["salom", "hi", "dunyo", "rust"];
    let eng_uzun: Option<&&str> = sozlar.iter().max_by_key(|s| s.len());
    let eng_qisqa: Option<&&str> = sozlar.iter().min_by_key(|s| s.len());
    println!("{:?}", eng_uzun);
    println!("{:?}", eng_qisqa);
    // Some("dunyo")
    // Some("hi")

    // count — soni
    // count — количество
    let juft_soni: usize = sonlar.iter().filter(|&&x| x % 2 == 0).count();
    println!("{}", juft_soni);
    // 4
}

fn collect_misollari() {

    let sonlar: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Vec ga collect
    // collect в Vec
    let v: Vec<i32> = sonlar.iter().map(|&x| x * 2).collect();
    println!("{:?}", v);
    // [2, 4, 6, 8, 10]

    // String ga collect
    // collect в String
    let s: String = vec!['s', 'a', 'l', 'o', 'm'].into_iter().collect();
    println!("{}", s);
    // salom

    // String → Vec<char>
    // String → Vec<char>
    let harflar: Vec<char> = "salom".chars().collect();
    println!("{:?}", harflar);
    // ['s', 'a', 'l', 'o', 'm']

    // HashMap ga collect
    // collect в HashMap
    let juftlar: Vec<(&str, i32)> = vec![("bir", 1), ("ikki", 2), ("uch", 3)];
    let xarita: HashMap<&str, i32> = juftlar.into_iter().collect();
    println!("{:?}", xarita.get("ikki"));
    // Some(2)

    // Result<Vec<T>, E> ga collect
    // collect в Result<Vec<T>, E>
    let stringlar: Vec<&str> = vec!["1", "2", "3", "4"];
    let natija: Result<Vec<i32>, _> = stringlar.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{:?}", natija);
    // Ok([1, 2, 3, 4])

    let xatoli: Vec<&str> = vec!["1", "ikki", "3"];
    let xato_natija: Result<Vec<i32>, _> = xatoli.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{}", xato_natija.is_err());
    // true

    // unzip — juftlarni ajratish
    // unzip — разделение пар
    let juftlar2: Vec<(i32, &str)> = vec![(1, "bir"), (2, "ikki"), (3, "uch")];
    let (sonlar2, sozlar2): (Vec<i32>, Vec<&str>) = juftlar2.into_iter().unzip();
    println!("{:?}", sonlar2);
    println!("{:?}", sozlar2);
    // [1, 2, 3]
    // ["bir", "ikki", "uch"]
}

fn peekable_windows_misollari() {

    // peekable — keyingisiga qaraish
    // peekable — посмотреть следующий элемент
    let mut it = vec![1, 2, 3, 4, 5].into_iter().peekable();
    while let Some(&keyingi) = it.peek() {
        if keyingi % 2 == 0 {
            it.next();
            println!("Juft: {}", keyingi);
        } else {
            it.next();
            println!("Toq: {}", keyingi);
        }
    }
    // Toq: 1
    // Juft: 2
    // Toq: 3
    // Juft: 4
    // Toq: 5

    // windows — oynalar (slice)
    // windows — окна (slice)
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let oynalar: Vec<&[i32]> = v.windows(3).collect();
    println!("{:?}", oynalar);
    // [[1, 2, 3], [2, 3, 4], [3, 4, 5]]

    // chunks — bo'laklar
    // chunks — части
    let bo_laklar: Vec<&[i32]> = v.chunks(2).collect();
    println!("{:?}", bo_laklar);
    // [[1, 2], [3, 4], [5]]

    // scan — fold + yield
    // scan — fold + yield
    let kumulyativ: Vec<i32> = vec![1, 2, 3, 4, 5]
        .iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    println!("{:?}", kumulyativ);
    // [1, 3, 6, 10, 15]
}

#[derive(Debug, Clone)]
struct Mahsulot {
    nomi: String,
    narx: f64,
    kategoriya: String,
    soni: u32,
}

impl Mahsulot {
    fn new(nomi: &str, narx: f64, kategoriya: &str, soni: u32) -> Self {
        Mahsulot {
            nomi: nomi.to_string(),
            narx,
            kategoriya: kategoriya.to_string(),
            soni,
        }
    }
}

fn real_hayot_misollari() {

    let mahsulotlar: Vec<Mahsulot> = vec![
        Mahsulot::new("Olma",    1500.0, "Meva",      100),
        Mahsulot::new("Non",      800.0, "Non",        50),
        Mahsulot::new("Sut",     2000.0, "Sut mahsulot", 30),
        Mahsulot::new("Banan",   3000.0, "Meva",       80),
        Mahsulot::new("Pishloq", 8000.0, "Sut mahsulot", 20),
        Mahsulot::new("Sharbat", 4000.0, "Ichimlik",   60),
        Mahsulot::new("Anor",    2500.0, "Meva",       40),
        Mahsulot::new("Yogurt",  1800.0, "Sut mahsulot", 45),
    ];

    // 1. Barcha mevalar
    // 1. Все фрукты
    let mevalar: Vec<&Mahsulot> = mahsulotlar.iter()
        .filter(|m| m.kategoriya == "Meva")
        .collect();
    println!("Mevalar:");
    for m in &mevalar {
        println!("  {} — {} so'm", m.nomi, m.narx);
    }
    // Mevalar:
    //   Olma — 1500 so'm
    //   Banan — 3000 so'm
    //   Anor — 2500 so'm

    // 2. Jami narx (soni * narx)
    // 2. Общая стоимость (количество * цена)
    let jami_narx: f64 = mahsulotlar.iter()
        .map(|m| m.narx * m.soni as f64)
        .sum();
    println!("Jami qiymat: {} so'm", jami_narx);
    // Jami qiymat: 1295000 so'm

    // 3. Kategoriya bo'yicha guruhlashtirish
    // 3. Группировка по категории
    let mut kategoriyalar: HashMap<&str, Vec<&Mahsulot>> = HashMap::new();
    for m in &mahsulotlar {
        kategoriyalar.entry(&m.kategoriya).or_default().push(m);
    }
    let mut k_list: Vec<(&str, usize)> = kategoriyalar.iter()
        .map(|(&k, v)| (k, v.len()))
        .collect();
    k_list.sort_by_key(|(k, _)| *k);
    for (kat, soni) in &k_list {
        println!("{}: {} ta mahsulot", kat, soni);
    }
    // Ichimlik: 1 ta mahsulot
    // Meva: 3 ta mahsulot
    // Non: 1 ta mahsulot
    // Sut mahsulot: 3 ta mahsulot

    // 4. Eng qimmat 3 ta
    // 4. Три самых дорогих
    let mut narx_tartibi: Vec<&Mahsulot> = mahsulotlar.iter().collect();
    narx_tartibi.sort_by(|a, b| b.narx.partial_cmp(&a.narx).unwrap());
    let eng_qimmat: Vec<&&Mahsulot> = narx_tartibi.iter().take(3).collect();
    println!("Eng qimmat 3 ta:");
    for m in &eng_qimmat {
        println!("  {} — {}", m.nomi, m.narx);
    }
    // Eng qimmat 3 ta:
    //   Pishloq — 8000
    //   Sharbat — 4000
    //   Banan — 3000

    // 5. 5000 so'mdan arzon sut mahsulotlari nomlari
    // 5. Молочные продукты дешевле 5000
    let arzon_sut: Vec<String> = mahsulotlar.iter()
        .filter(|m| m.kategoriya == "Sut mahsulot" && m.narx < 5000.0)
        .map(|m| m.nomi.clone())
        .collect();
    println!("{:?}", arzon_sut);
    // ["Sut", "Yogurt"]

    // 6. O'rtacha narx
    // 6. Средняя цена
    let count: usize = mahsulotlar.len();
    let o_rtacha: f64 = mahsulotlar.iter().map(|m| m.narx).sum::<f64>() / count as f64;
    println!("O'rtacha narx: {:.1} so'm", o_rtacha);
    // O'rtacha narx: 3075.0 so'm

    // 7. CSV ko'rinishida chiqarish
    // 7. Вывод в виде CSV
    let csv: String = std::iter::once(String::from("Nomi,Narx,Kategoriya,Soni"))
        .chain(mahsulotlar.iter().map(|m| {
            format!("{},{},{},{}", m.nomi, m.narx, m.kategoriya, m.soni)
        }))
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", &csv[..csv.find('\n').unwrap_or(csv.len())]);
    // Nomi,Narx,Kategoriya,Soni
}

fn main() {

    println!("=== ITERATOR YARATISH ===");
    iterator_yaratish_misollari();

    println!("\n=== MAP ===");
    map_misollari();

    println!("\n=== FILTER ===");
    filter_misollari();

    println!("\n=== FOLD VA REDUCE ===");
    fold_reduce_misollari();

    println!("\n=== ENUMERATE, ZIP, CHAIN ===");
    enumerate_zip_chain_misollari();

    println!("\n=== TAKE, SKIP, FLATTEN ===");
    take_skip_flatten_misollari();

    println!("\n=== QIDIRUV VA TEKSHIRISH ===");
    qidiruv_tekshirish_misollari();

    println!("\n=== COLLECT ===");
    collect_misollari();

    println!("\n=== PEEKABLE VA WINDOWS ===");
    peekable_windows_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Metod                    | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                       ITERATOR YARATISH                                                                                      |
// #================================================================================================================================================#
// # |   1 | .iter()                  | &T beradi — borrow                                   | Даёт &T — заимствование                              |
// # |   2 | .iter_mut()              | &mut T beradi                                        | Даёт &mut T                                          |
// # |   3 | .into_iter()             | T beradi — ownership                                 | Даёт T — владение                                    |
// # |   4 | (a..b)                   | Range iterator                                       | Итератор диапазона                                   |
// # |   5 | .chars()                 | Char iteratori                                       | Итератор символов                                    |
// # |   6 | .lines()                 | Qator iteratori                                      | Итератор строк                                       |
// #================================================================================================================================================#
// # |                                       O'ZGARTIRISH                                                                                           |
// #================================================================================================================================================#
// # |   7 | .map(|x| ...)            | Har elementni o'zgartirish                           | Преобразование каждого элемента                      |
// # |   8 | .filter(|x| ...)         | Shartga mos elementlar                               | Элементы по условию                                  |
// # |   9 | .filter_map(|x| ...)     | Filter + Map birga                                   | Filter + Map вместе                                  |
// # |  10 | .flat_map(|x| ...)       | Map + Flatten birga                                  | Map + Flatten вместе                                 |
// # |  11 | .flatten()               | Ichki iteratorlarni yassilash                        | Сплющивание вложенных итераторов                     |
// #================================================================================================================================================#
// # |                                       YIG'ISH                                                                                                |
// #================================================================================================================================================#
// # |  12 | .fold(acc, |acc, x| ...) | Akkumulator bilan yig'ish                            | Свёртка с аккумулятором                              |
// # |  13 | .reduce(|acc, x| ...)    | Birinchi element akkumulator                         | Первый элемент — аккумулятор                         |
// # |  14 | .sum()                   | Yig'indi                                             | Сумма                                                |
// # |  15 | .product()               | Ko'paytma                                            | Произведение                                         |
// # |  16 | .collect::<Vec<_>>()     | Vec ga yig'ish                                       | Сбор в Vec                                           |
// #================================================================================================================================================#
// # |                                       QIDIRUV                                                                                                |
// #================================================================================================================================================#
// # |  17 | .any(|x| ...)            | Kamida bitta mos kelsa                               | Хотя бы один совпадает                               |
// # |  18 | .all(|x| ...)            | Hammasi mos kelsa                                    | Все совпадают                                        |
// # |  19 | .find(|x| ...)           | Birinchi mos elementni topish                        | Найти первый подходящий                              |
// # |  20 | .position(|x| ...)       | Birinchi mos elementning indeksi                     | Индекс первого подходящего                           |
// # |  21 | .max() / .min()          | Maksimum / minimum                                   | Максимум / минимум                                   |
// # |  22 | .count()                 | Elementlar soni                                      | Количество элементов                                 |
// #================================================================================================================================================#
// # |                                       KOMBINATSIYA                                                                                           |
// #================================================================================================================================================#
// # |  23 | .enumerate()             | (indeks, element) juftligi                           | Пары (индекс, элемент)                               |
// # |  24 | .zip(other)              | Ikki iteratorni juftlashtirish                       | Объединение двух итераторов                          |
// # |  25 | .chain(other)            | Ketma-ket birlashtirish                              | Последовательное объединение                         |
// # |  26 | .take(n)                 | N ta element olish                                   | Взять N элементов                                    |
// # |  27 | .skip(n)                 | N ta elementni o'tkazish                             | Пропустить N элементов                               |
// # |  28 | .peekable()              | Keyingisiga qarash                                   | Заглянуть вперёд                                     |
// # |  29 | .windows(n)              | N o'lchamli oynalar                                  | Окна размером N                                      |
// # |  30 | .chunks(n)               | N o'lchamli bo'laklar                                | Части размером N                                     |
// # |  31 | .scan(state, |s, x| ...) | Fold + har qadamda yield                             | Fold + yield на каждом шаге                          |
// #================================================================================================================================================#