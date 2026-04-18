// #================================================================================================================================================#
// #                                                    COLLECTIONS — BTREEMAP | VECDEQUE | BINARYHEAP                                              #
// #                            BTREEMAP — TARTIBLANGAN MAP. VECDEQUE — IKKI TOMONLI NAVBAT. BINARYHEAP — USTUNLIK NAVBATI.                         #
// #                            BTREEMAP — УПОРЯДОЧЕННАЯ MAP. VECDEQUE — ДВУСТОРОННЯЯ ОЧЕРЕДЬ. BINARYHEAP — ОЧЕРЕДЬ С ПРИОРИТЕТОМ.                  #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::collections::{BTreeMap, BTreeSet, VecDeque, BinaryHeap};
use std::cmp::Reverse;

// Qaysi kolleksiya qachon:
// Какую коллекцию когда:
//
//   HashMap   — eng tez, tartibsiz, O(1) avg
//   BTreeMap  — tartiblangan, O(log n), range query kerak bo'lsa
//   Vec       — ketma-ket, oxiridan qo'shish/olish, O(1)
//   VecDeque  — ikki tomondan qo'shish/olish, O(1)
//   BinaryHeap— eng katta/kichik elementni tez olish, O(log n)
//   HashSet   — noyob elementlar, tez tekshirish
//   BTreeSet  — noyob va tartiblangan elementlar

fn btreemap_misollari() {

    // BTreeMap — kalit bo'yicha avtomatik tartiblangan
    // BTreeMap — автоматически упорядочена по ключу
    let mut xarita: BTreeMap<&str, i32> = BTreeMap::new();
    xarita.insert("olma", 5);
    xarita.insert("banan", 3);
    xarita.insert("anor", 8);
    xarita.insert("uzum", 2);
    xarita.insert("nok", 6);

    // Tartiblangan iteratsiya — har doim kalit tartibida
    // Упорядоченная итерация — всегда по порядку ключей
    for (k, v) in &xarita {
        print!("{}: {} | ", k, v);
    }
    println!();
    // anor: 8 | banan: 3 | nok: 6 | olma: 5 | uzum: 2 |

    // get, contains_key, remove — HashMap ga o'xshash
    // get, contains_key, remove — аналогично HashMap
    println!("{:?}", xarita.get("olma"));
    println!("{}", xarita.contains_key("banan"));
    // Some(5)
    // true

    // range() — diapazondagi kalitlar
    // range() — ключи в диапазоне
    let diapazon: Vec<(&&str, &i32)> = xarita.range("b".."o").collect();
    println!("{:?}", diapazon);
    // [("banan", 3), ("nok", 6)]

    // first_key_value va last_key_value
    // first_key_value и last_key_value
    println!("{:?}", xarita.first_key_value());
    println!("{:?}", xarita.last_key_value());
    // Some(("anor", 8))
    // Some(("uzum", 2))

    // pop_first va pop_last
    // pop_first и pop_last
    let mut xarita2: BTreeMap<i32, &str> = BTreeMap::new();
    xarita2.insert(3, "uch");
    xarita2.insert(1, "bir");
    xarita2.insert(2, "ikki");
    println!("{:?}", xarita2.pop_first()); // eng kichik
    println!("{:?}", xarita2.pop_last());  // eng katta
    // Some((1, "bir"))
    // Some((3, "uch"))

    // entry API — HashMap bilan bir xil
    // entry API — аналогично HashMap
    let mut hisob: BTreeMap<char, usize> = BTreeMap::new();
    for harf in "salom dunyo rust".chars() {
        if harf != ' ' {
            *hisob.entry(harf).or_insert(0) += 1;
        }
    }
    for (h, n) in &hisob {
        print!("'{}': {} | ", h, n);
    }
    println!();
    // 'a': 1 | 'd': 1 | 'l': 1 | 'm': 1 | 'n': 1 | 'o': 2 | 'r': 1 | 's': 2 | 't': 1 | 'u': 2 | 'y': 1 |

    // BTreeSet — tartiblangan noyob elementlar
    // BTreeSet — упорядоченные уникальные элементы
    let mut set: BTreeSet<i32> = BTreeSet::new();
    set.extend([5, 2, 8, 1, 9, 3, 7]);
    println!("{:?}", set); // avtomatik tartiblangan
    // {1, 2, 3, 5, 7, 8, 9}

    // BTreeSet — range
    // BTreeSet — range
    let diapazon2: Vec<&i32> = set.range(3..8).collect();
    println!("{:?}", diapazon2);
    // [3, 5, 7]
}

fn vecdeque_misollari() {

    // VecDeque — ikki tomondan O(1) qo'shish va olish
    // VecDeque — O(1) добавление и извлечение с обоих концов
    let mut dq: VecDeque<i32> = VecDeque::new();

    // Orqadan qo'shish (Vec::push ga o'xshash)
    // Добавление сзади (аналогично Vec::push)
    dq.push_back(1);
    dq.push_back(2);
    dq.push_back(3);

    // Olddan qo'shish
    // Добавление спереди
    dq.push_front(0);
    dq.push_front(-1);

    println!("{:?}", dq);
    // [-1, 0, 1, 2, 3]

    // Olddan olish
    // Извлечение спереди
    println!("{:?}", dq.pop_front()); // Some(-1)
    println!("{:?}", dq.pop_front()); // Some(0)

    // Orqadan olish
    // Извлечение сзади
    println!("{:?}", dq.pop_back()); // Some(3)

    println!("{:?}", dq);
    // [1, 2]

    // front() va back() — ko'rish (o'chirmasdan)
    // front() и back() — просмотр (без удаления)
    dq.push_back(10);
    dq.push_back(20);
    println!("{:?}", dq.front()); // Some(1)
    println!("{:?}", dq.back());  // Some(20)
    // Some(1)
    // Some(20)

    // Indeks bilan kirish — Vec ga o'xshash
    // Доступ по индексу — аналогично Vec
    println!("{}", dq[0]);
    println!("{}", dq[1]);
    // 1
    // 2

    // rotate_left va rotate_right
    // rotate_left и rotate_right
    let mut dq2: VecDeque<i32> = vec![1, 2, 3, 4, 5].into();
    dq2.rotate_left(2);
    println!("{:?}", dq2);
    // [3, 4, 5, 1, 2]

    dq2.rotate_right(1);
    println!("{:?}", dq2);
    // [2, 3, 4, 5, 1]

    // Vec dan VecDeque va aksi
    // Vec из VecDeque и наоборот
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut dq3: VecDeque<i32> = v.into();
    dq3.push_front(0);
    let v2: Vec<i32> = dq3.into();
    println!("{:?}", v2);
    // [0, 1, 2, 3, 4, 5]

    // retain — shartga mos elementlarni saqlash
    // retain — сохранение элементов по условию
    let mut dq4: VecDeque<i32> = vec![1, 2, 3, 4, 5, 6].into();
    dq4.retain(|&x| x % 2 == 0);
    println!("{:?}", dq4);
    // [2, 4, 6]

    // make_contiguous — xotirani birlashtirish
    // make_contiguous — объединение памяти
    let mut dq5: VecDeque<i32> = VecDeque::new();
    dq5.push_back(3);
    dq5.push_back(4);
    dq5.push_front(2);
    dq5.push_front(1);
    let slice: &[i32] = dq5.make_contiguous();
    println!("{:?}", slice);
    // [1, 2, 3, 4]
}

fn binaryheap_misollari() {

    // BinaryHeap — max-heap (eng katta element tepada)
    // BinaryHeap — max-heap (наибольший элемент наверху)
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    heap.push(5);
    heap.push(1);
    heap.push(9);
    heap.push(3);
    heap.push(7);

    // peek — eng kattasini ko'rish (o'chirmasdan)
    // peek — просмотр наибольшего (без удаления)
    println!("{:?}", heap.peek()); // Some(9)

    // pop — eng kattasini olish
    // pop — извлечение наибольшего
    while let Some(qiymat) = heap.pop() {
        print!("{} ", qiymat);
    }
    println!();
    // 9 7 5 3 1 (kamayib boruvchi tartibda)

    // Vec dan BinaryHeap
    // BinaryHeap из Vec
    let v: Vec<i32> = vec![4, 2, 8, 6, 1];
    let mut heap2: BinaryHeap<i32> = v.into_iter().collect();
    println!("{:?}", heap2.peek()); // Some(8)
    // Some(8)

    // into_sorted_vec — tartiblangan Vec (o'sib boruvchi)
    // into_sorted_vec — отсортированный Vec (по возрастанию)
    let heap3: BinaryHeap<i32> = vec![5, 1, 9, 3, 7].into_iter().collect();
    let tartiblangan: Vec<i32> = heap3.into_sorted_vec();
    println!("{:?}", tartiblangan);
    // [1, 3, 5, 7, 9]

    // Reverse bilan min-heap
    // Min-heap с Reverse
    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(9));
    min_heap.push(Reverse(3));

    // eng kichigi birinchi chiqadi
    // наименьший выходит первым
    while let Some(Reverse(v)) = min_heap.pop() {
        print!("{} ", v);
    }
    println!();
    // 1 3 5 9

    // len, is_empty, capacity
    // len, is_empty, capacity
    let mut heap4: BinaryHeap<i32> = BinaryHeap::with_capacity(10);
    heap4.extend([1, 2, 3]);
    println!("uzunlik: {}", heap4.len());
    println!("sig'im: {}", heap4.capacity());
    // uzunlik: 3
    // sig'im: 10

    // BinaryHeap — custom tur bilan
    // BinaryHeap — с custom типом
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct Vazifa {
        ustunlik: u8,
        nomi: String,
    }

    impl Vazifa {
        fn new(ustunlik: u8, nomi: &str) -> Self {
            Vazifa { ustunlik, nomi: nomi.to_string() }
        }
    }

    let mut vazifalar: BinaryHeap<Vazifa> = BinaryHeap::new();
    vazifalar.push(Vazifa::new(3, "Xabar yuborish"));
    vazifalar.push(Vazifa::new(1, "Hisobot yozish"));
    vazifalar.push(Vazifa::new(5, "Xavfsizlik muammosi"));
    vazifalar.push(Vazifa::new(2, "Yangi xususiyat"));

    // Ustunlik bo'yicha qayta ishlash
    // Обработка по приоритету
    while let Some(v) = vazifalar.pop() {
        println!("[{}] {}", v.ustunlik, v.nomi);
    }
    // [5] Xavfsizlik muammosi
    // [3] Xabar yuborish
    // [2] Yangi xususiyat
    // [1] Hisobot yozish
}

fn real_hayot_misollari() {

    // 1. BTreeMap — versiyalar tarixi
    // 1. BTreeMap — история версий
    let mut versiyalar: BTreeMap<(u32, u32, u32), &str> = BTreeMap::new();
    versiyalar.insert((1, 0, 0), "Birinchi chiqarilish");
    versiyalar.insert((1, 2, 0), "Yangi xususiyatlar");
    versiyalar.insert((2, 0, 0), "Katta yangilanish");
    versiyalar.insert((1, 1, 0), "Xato tuzatishlar");

    // Tartiblangan chiqish
    // Упорядоченный вывод
    for ((major, minor, patch), xabar) in &versiyalar {
        println!("v{}.{}.{}: {}", major, minor, patch, xabar);
    }
    // v1.0.0: Birinchi chiqarilish
    // v1.1.0: Xato tuzatishlar
    // v1.2.0: Yangi xususiyatlar
    // v2.0.0: Katta yangilanish

    // 2. VecDeque — sliding window
    // 2. VecDeque — скользящее окно
    let ma_lumotlar: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let oyna: usize = 3;
    let mut dq: VecDeque<f64> = VecDeque::with_capacity(oyna);
    let mut ortachalar: Vec<f64> = Vec::new();

    for &qiymat in &ma_lumotlar {
        if dq.len() == oyna {
            dq.pop_front();
        }
        dq.push_back(qiymat);
        if dq.len() == oyna {
            let ortacha: f64 = dq.iter().sum::<f64>() / oyna as f64;
            ortachalar.push(ortacha);
        }
    }
    println!("{:?}", ortachalar);
    // [2.0, 3.0, 4.0, 5.0, 6.0, 7.0]

    // 3. BinaryHeap — K ta eng katta element
    // 3. BinaryHeap — K наибольших элементов
    let sonlar: Vec<i32> = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let k: usize = 3;

    let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for &son in &sonlar {
        min_heap.push(Reverse(son));
        if min_heap.len() > k {
            min_heap.pop();
        }
    }
    let mut top_k: Vec<i32> = min_heap.into_iter().map(|Reverse(v)| v).collect();
    top_k.sort_unstable_by(|a, b| b.cmp(a));
    println!("Top {}: {:?}", k, top_k);
    // Top 3: [9, 6, 5]

    // 4. VecDeque — FIFO navbat (server so'rovlari)
    // 4. VecDeque — FIFO очередь (запросы сервера)
    let mut navbat: VecDeque<String> = VecDeque::new();

    // Yangi so'rovlar keldi
    // Пришли новые запросы
    navbat.push_back("GET /api/users".to_string());
    navbat.push_back("POST /api/login".to_string());
    navbat.push_back("GET /api/data".to_string());

    // So'rovlarni ketma-ket qayta ishlash
    // Последовательная обработка запросов
    while let Some(sorov) = navbat.pop_front() {
        println!("Qayta ishlash: {}", sorov);
    }
    // Qayta ishlash: GET /api/users
    // Qayta ishlash: POST /api/login
    // Qayta ishlash: GET /api/data

    // 5. BTreeMap — oraliq so'rov (range query)
    // 5. BTreeMap — запрос диапазона (range query)
    let mut narxlar: BTreeMap<&str, f64> = BTreeMap::new();
    narxlar.insert("Olma",    1500.0);
    narxlar.insert("Banan",   3000.0);
    narxlar.insert("Anor",    2500.0);
    narxlar.insert("Nok",     2000.0);
    narxlar.insert("Uzum",    4000.0);
    narxlar.insert("Shaftoli",3500.0);

    // Faqat A dan N gacha mahsulotlar
    // Только продукты от A до N
    let a_dan_n: Vec<(&&str, &f64)> = narxlar.range("A".."O").collect();
    for (nom, narx) in &a_dan_n {
        println!("{}: {} so'm", nom, narx);
    }
    // Anor: 2500 so'm
    // Banan: 3000 so'm
    // Nok: 2000 so'm
}

fn main() {

    println!("=== BTREEMAP ===");
    btreemap_misollari();

    println!("\n=== VECDEQUE ===");
    vecdeque_misollari();

    println!("\n=== BINARYHEAP ===");
    binaryheap_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                  | Описание (RU)                                               |
// #================================================================================================================================================#
// # |                                       BTREEMAP                                                                                               |
// #================================================================================================================================================#
// # |   1 | BTreeMap::new()           | Tartiblangan bo'sh map                        | Упорядоченная пустая map                                   |
// # |   2 | .range(a..b)              | Kalit diapazoni bo'yicha iteratsiya           | Итерация по диапазону ключей                               |
// # |   3 | .first_key_value()        | Eng kichik kalitli element                    | Элемент с наименьшим ключом                                |
// # |   4 | .last_key_value()         | Eng katta kalitli element                     | Элемент с наибольшим ключом                                |
// # |   5 | .pop_first()              | Eng kichik kalitni olib o'chirish             | Извлечение и удаление наименьшего ключа                    |
// # |   6 | .pop_last()               | Eng katta kalitni olib o'chirish              | Извлечение и удаление наибольшего ключа                    |
// # |   7 | BTreeSet                  | Tartiblangan noyob elementlar                 | Упорядоченные уникальные элементы                          |
// #================================================================================================================================================#
// # |                                       VECDEQUE                                                                                               |
// #================================================================================================================================================#
// # |   8 | VecDeque::new()           | Bo'sh ikki tomonli navbat                     | Пустая двусторонняя очередь                                |
// # |   9 | .push_front(x)            | Olddan qo'shish O(1)                          | Добавление спереди O(1)                                    |
// # |  10 | .push_back(x)             | Orqadan qo'shish O(1)                         | Добавление сзади O(1)                                      |
// # |  11 | .pop_front()              | Olddan olish O(1)                             | Извлечение спереди O(1)                                    |
// # |  12 | .pop_back()               | Orqadan olish O(1)                            | Извлечение сзади O(1)                                      |
// # |  13 | .rotate_left(n)           | n pozitsiya chapga aylantirish                | Поворот на n позиций влево                                 |
// # |  14 | .make_contiguous()        | Xotirada yaxlit slice                         | Цельный срез в памяти                                      |
// # |  15 | Vec ↔ VecDeque            | .into() bilan aylantirish                     | Преобразование через .into()                               |
// #================================================================================================================================================#
// # |                                       BINARYHEAP                                                                                             |
// #================================================================================================================================================#
// # |  16 | BinaryHeap::new()         | Bo'sh max-heap                                | Пустая max-heap                                            |
// # |  17 | .push(x)                  | Element qo'shish O(log n)                     | Добавление элемента O(log n)                               |
// # |  18 | .pop()                    | Eng kattasini olish O(log n)                  | Извлечение наибольшего O(log n)                            |
// # |  19 | .peek()                   | Eng kattasini ko'rish O(1)                    | Просмотр наибольшего O(1)                                  |
// # |  20 | Reverse<T>                | Min-heap uchun                                | Для min-heap                                               |
// # |  21 | .into_sorted_vec()        | Tartiblangan Vec (o'sib boruvchi)             | Отсортированный Vec (по возрастанию)                       |
// #================================================================================================================================================#
// # |                                       QACHON NIMA ISHLATISH                                                                                  |
// #================================================================================================================================================#
// # |  22 | BTreeMap vs HashMap       | Tartib va range kerak → BTreeMap              | Нужен порядок и range → BTreeMap                           |
// # |  23 | VecDeque vs Vec           | Ikki tomondan O(1) kerak → VecDeque           | Нужен O(1) с обеих сторон → VecDeque                       |
// # |  24 | BinaryHeap                | Eng katta/kichik tez kerak → BinaryHeap       | Нужен быстрый max/min → BinaryHeap                         |
// #================================================================================================================================================#