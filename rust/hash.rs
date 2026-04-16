// #================================================================================================================================================#
// #                                                           STD::HASH                                                                            #
// #                     HASH — QIYMATDAN ANIQ RAQAM HOSIL QILISH. HASHMAP VA HASHSET ASOSI. CUSTOM HASH IMPLEMENT.                                 #
// #                     HASH — ПОЛУЧЕНИЕ ЧИСЛА ИЗ ЗНАЧЕНИЯ. ОСНОВА HASHMAP И HASHSET. CUSTOM РЕАЛИЗАЦИЯ HASH.                                      #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::hash::{Hash, Hasher, DefaultHasher, BuildHasher, BuildHasherDefault};
use std::collections::{HashMap, HashSet};
use std::fmt;

// Hash trait:
//   trait Hash {
//       fn hash<H: Hasher>(&self, state: &mut H);
//   }
//
// Hasher trait:
//   trait Hasher {
//       fn finish(&self) -> u64;
//       fn write(&mut self, bytes: &[u8]);
//   }
//
// Qoidalar:
// Правила:
//   1. k1 == k2  →  hash(k1) == hash(k2)  SHART
//      k1 == k2  →  hash(k1) == hash(k2)  ОБЯЗАТЕЛЬНО
//   2. hash(k1) == hash(k2)  →  k1 == k2  bo'lmasligi mumkin (collision)
//      hash(k1) == hash(k2)  →  k1 == k2  не обязательно (коллизия)
//   3. PartialEq va Hash bir xil implementatsiyaga ega bo'lishi kerak
//      PartialEq и Hash должны иметь согласованную реализацию
//   4. #[derive(Hash)] — avtomatik, barcha fieldlar Hash bo'lsa
//      #[derive(Hash)] — автоматически, если все поля реализуют Hash

fn hash_hisoblash_misollari() {

    // DefaultHasher bilan hash hisoblash
    // Вычисление хеша через DefaultHasher
    fn hash_qiymat<T: Hash>(qiymat: &T) -> u64 {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        qiymat.hash(&mut hasher);
        hasher.finish()
    }

    // Turli turlar uchun hash
    // Хеш для разных типов
    println!("{}", hash_qiymat(&42i32));
    println!("{}", hash_qiymat(&"salom"));
    println!("{}", hash_qiymat(&String::from("salom")));
    println!("{}", hash_qiymat(&vec![1, 2, 3]));
    println!("{}", hash_qiymat(&(1, 2, 3)));
    println!("{}", hash_qiymat(&true));
    // (har xil qiymatlar — system va versiyadependent)

    // Bir xil qiymat → bir xil hash (kafolat)
    // Одинаковое значение → одинаковый хеш (гарантия)
    let h1: u64 = hash_qiymat(&"salom");
    let h2: u64 = hash_qiymat(&"salom");
    println!("Bir xilmi: {}", h1 == h2);
    // Bir xilmi: true

    // &str va String — bir xil hash (Borrow kafolati)
    // &str и String — одинаковый хеш (гарантия Borrow)
    let h_str: u64 = hash_qiymat(&"kalit");
    let h_string: u64 = hash_qiymat(&String::from("kalit"));
    println!("str == String hash: {}", h_str == h_string);
    // str == String hash: true

    // Farqli qiymatlar — farqli hash (odatda)
    // Разные значения — разный хеш (обычно)
    println!("{}", hash_qiymat(&42i32) == hash_qiymat(&43i32));
    // false
}

// #[derive(Hash)] — avtomatik implement
// #[derive(Hash)] — автоматическая реализация
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Nuqta {
    x: i32,
    y: i32,
}

impl Nuqta {
    fn new(x: i32, y: i32) -> Self { Nuqta { x, y } }
}

// Qo'lda Hash implement — maxsus mantiq
// Ручная реализация Hash — специальная логика
#[derive(Debug, Clone)]
struct KasbiyFoydalanuvchi {
    id: u64,
    ism: String,
    email: String,
    // email ni hashlashda e'tiborsiz qoldiramiz
    // игнорируем email при хешировании
}

impl KasbiyFoydalanuvchi {
    fn new(id: u64, ism: &str, email: &str) -> Self {
        KasbiyFoydalanuvchi {
            id,
            ism: ism.to_string(),
            email: email.to_string(),
        }
    }
}

// PartialEq — id va ism bo'yicha
// PartialEq — по id и ism
impl PartialEq for KasbiyFoydalanuvchi {
    fn eq(&self, b: &Self) -> bool {
        self.id == b.id && self.ism == b.ism
        // email e'tiborsiz!
        // email игнорируется!
    }
}

impl Eq for KasbiyFoydalanuvchi {}

// Hash — faqat id va ism bo'yicha (PartialEq bilan mos!)
// Hash — только по id и ism (согласовано с PartialEq!)
impl Hash for KasbiyFoydalanuvchi {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.ism.hash(state);
        // email hashlash ga kirmaydi
        // email не входит в хеш
    }
}

fn custom_hash_misollari() {

    // Nuqta — HashMap kalit sifatida
    // Nuqta — в качестве ключа HashMap
    let mut xarita: HashMap<Nuqta, &str> = HashMap::new();
    xarita.insert(Nuqta::new(0, 0), "markaz");
    xarita.insert(Nuqta::new(1, 0), "o'ng");
    xarita.insert(Nuqta::new(0, 1), "yuqori");

    println!("{:?}", xarita.get(&Nuqta::new(0, 0)));
    println!("{:?}", xarita.get(&Nuqta::new(1, 1)));
    // Some("markaz")
    // None

    // HashSet da Nuqta
    // Nuqta в HashSet
    let mut set: HashSet<Nuqta> = HashSet::new();
    set.insert(Nuqta::new(0, 0));
    set.insert(Nuqta::new(1, 1));
    set.insert(Nuqta::new(0, 0)); // takroran — qabul qilinmaydi
    // повтор — не принимается
    println!("{}", set.len());
    // 2

    // KasbiyFoydalanuvchi — email farqli bo'lsa ham bir xil
    // KasbiyFoydalanuvchi — одинаковы даже при разных email
    let f1 = KasbiyFoydalanuvchi::new(1, "Dilshod", "old@mail.com");
    let f2 = KasbiyFoydalanuvchi::new(1, "Dilshod", "new@mail.com");

    let mut f_set: HashSet<KasbiyFoydalanuvchi> = HashSet::new();
    f_set.insert(f1);
    f_set.insert(f2); // bir xil — qabul qilinmaydi
    // одинаковые — не принимается
    println!("{}", f_set.len());
    // 1

    // Nuqta — HashSet da matematik amallar
    // Nuqta — математические операции в HashSet
    let set1: HashSet<Nuqta> = vec![
        Nuqta::new(1,1), Nuqta::new(2,2), Nuqta::new(3,3)
    ].into_iter().collect();
    let set2: HashSet<Nuqta> = vec![
        Nuqta::new(2,2), Nuqta::new(3,3), Nuqta::new(4,4)
    ].into_iter().collect();

    let kesishuv: HashSet<&Nuqta> = set1.intersection(&set2).collect();
    let birlashuv: HashSet<&Nuqta> = set1.union(&set2).collect();
    let farq: HashSet<&Nuqta> = set1.difference(&set2).collect();

    println!("Kesishuv: {}", kesishuv.len());
    println!("Birlashuv: {}", birlashuv.len());
    println!("Farq: {}", farq.len());
    // Kesishuv: 2
    // Birlashuv: 4
    // Farq: 1
}

// Oddiy FNV hasher implementatsiyasi
// Простая реализация FNV хешера
#[derive(Default)]
struct FnvHasher {
    state: u64,
}

impl FnvHasher {
    const FNV_PRIME: u64 = 1_099_511_628_211;
    const FNV_OFFSET: u64 = 14_695_981_039_346_656_037;

    fn new() -> Self {
        FnvHasher { state: Self::FNV_OFFSET }
    }
}

impl Hasher for FnvHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(Self::FNV_PRIME);
        }
    }
}

// BuildHasher — HashMap/HashSet uchun
// BuildHasher — для HashMap/HashSet
type FnvBuildHasher = BuildHasherDefault<FnvHasher>;
type FnvHashMap<K, V> = HashMap<K, V, FnvBuildHasher>;
type FnvHashSet<T> = HashSet<T, FnvBuildHasher>;

fn custom_hasher_misollari() {

    // FNV hasher bilan hash hisoblash
    // Вычисление хеша с FNV хешером
    let mut fnv: FnvHasher = FnvHasher::new();
    "salom".hash(&mut fnv);
    println!("FNV hash: {}", fnv.finish());

    // FNV HashMap
    // FNV HashMap
    let mut fnv_map: FnvHashMap<&str, i32> = FnvHashMap::default();
    fnv_map.insert("bir", 1);
    fnv_map.insert("ikki", 2);
    fnv_map.insert("uch", 3);
    println!("{:?}", fnv_map.get("ikki"));
    // Some(2)

    // FNV HashSet
    // FNV HashSet
    let mut fnv_set: FnvHashSet<i32> = FnvHashSet::default();
    fnv_set.extend([1, 2, 3, 2, 1]);
    println!("{}", fnv_set.len());
    // 3

    // Standart DefaultHasher bilan taqqoslash
    // Сравнение со стандартным DefaultHasher
    fn hash_default<T: Hash>(v: &T) -> u64 {
        let mut h = DefaultHasher::new();
        v.hash(&mut h);
        h.finish()
    }
    fn hash_fnv<T: Hash>(v: &T) -> u64 {
        let mut h = FnvHasher::new();
        v.hash(&mut h);
        h.finish()
    }

    println!("Default: {}", hash_default(&"test"));
    println!("FNV:     {}", hash_fnv(&"test"));
    // Default: XXXX (boshqa qiymat)
    // FNV:     YYYY (boshqa qiymat)
}

fn real_hayot_misollari() {

    // 1. Cache — Hash asosida (K: Clone talab qilinadi, chunki eviction uchun kalitni klonlash kerak)
    struct SimpleCache<K: Hash + Eq + Clone, V> {
        ichki: HashMap<K, V>,
        sig_imish: usize,
    }

    impl<K: Hash + Eq + Clone, V> SimpleCache<K, V> {
        fn new(sig_imish: usize) -> Self {
            SimpleCache { ichki: HashMap::new(), sig_imish }
        }

        fn ol_yoki_hisoblash<F: Fn(&K) -> V>(&mut self, kalit: K, f: F) -> &V {
            if !self.ichki.contains_key(&kalit) {
                if self.ichki.len() >= self.sig_imish {
                    // keys().next() -> Option<&K>, cloned() -> Option<K> (K: Clone)
                    // Immutable borrow keys() dan keyin tugaydi, shuning uchun mutable borrow xavfsiz
                    if let Some(eski_kalit) = self.ichki.keys().next().cloned() {
                        self.ichki.remove(&eski_kalit);
                    }
                }
                let qiymat = f(&kalit);
                self.ichki.insert(kalit, qiymat);
                return self.ichki.values().last().unwrap();
            }
            self.ichki.get(&kalit).unwrap()
        }
    }

    let mut cache: SimpleCache<String, u64> = SimpleCache::new(100);

    fn sekin_hisoblash(s: &String) -> u64 {
        let mut h = DefaultHasher::new();
        s.hash(&mut h);
        h.finish()
    }

    let v1 = cache.ol_yoki_hisoblash("kalit1".to_string(), sekin_hisoblash);
    println!("Cache natija: {}", v1);

    // 2. So'zlar chastotasi — HashMap bilan
    let matn: &str = "rust tili rust yaxshi rust xavfsiz va tez rust tili";
    let mut chastota: HashMap<&str, u32> = HashMap::new();
    for soz in matn.split_whitespace() {
        *chastota.entry(soz).or_insert(0) += 1;
    }
    let mut juftlar: Vec<(&&str, &u32)> = chastota.iter().collect();
    juftlar.sort_by(|a, b| b.1.cmp(a.1));
    for (soz, n) in juftlar.iter().take(3) {
        println!("{}: {}", soz, n);
    }

    // 3. Noyob qiymatlar — HashSet bilan
    let sonlar: Vec<i32> = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let noyob: HashSet<i32> = sonlar.into_iter().collect();
    let mut sorted: Vec<i32> = noyob.into_iter().collect();
    sorted.sort();
    println!("{:?}", sorted);

    // 4. Graph qo'shni ro'yxati — HashMap<HashSet>
    let mut graf: HashMap<&str, HashSet<&str>> = HashMap::new();
    let qo_shnilar: &[(&str, &str)] = &[
        ("A", "B"), ("A", "C"),
        ("B", "C"), ("B", "D"),
        ("C", "D"),
    ];
    for &(dan, ga) in qo_shnilar {
        graf.entry(dan).or_default().insert(ga);
        graf.entry(ga).or_default().insert(dan);
    }
    if let Some(a_qo_shnilar) = graf.get("A") {
        let mut sorted: Vec<&&str> = a_qo_shnilar.iter().collect();
        sorted.sort();
        println!("A qo'shnilari: {:?}", sorted);
    }

    // 5. Entry API — HashMap da eng kuchli pattern
    let sozlar: Vec<&str> = vec!["bir", "ikki", "bir", "uch", "ikki", "bir"];
    let mut hisob: HashMap<&str, usize> = HashMap::new();
    for soz in &sozlar {
        hisob.entry(soz).and_modify(|n| *n += 1).or_insert(1);
    }
    let mut sorted_hisob: Vec<(&&str, &usize)> = hisob.iter().collect();
    // Pattern tuzatildi: qo'shimcha & qo'shildi
    sorted_hisob.sort_by_key(|&(&k, _)| k);
    for (soz, n) in sorted_hisob {
        println!("{}: {}", soz, n);
    }
}

fn main() {

    println!("=== HASH HISOBLASH ===");
    hash_hisoblash_misollari();

    println!("\n=== CUSTOM HASH ===");
    custom_hash_misollari();

    println!("\n=== CUSTOM HASHER ===");
    custom_hasher_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                | Tavsif (UZ)                                | Описание (RU)                                               |
// #================================================================================================================================================#
// # |                                        HASH TRAIT                                                                                            |
// #================================================================================================================================================#
// # |   1 | #[derive(Hash)]             | Avtomatik Hash — barcha field Hash bo'lsa  | Автоматически если все поля Hash                            |
// # |   2 | impl Hash for T             | Qo'lda Hash implement                      | Ручная реализация Hash                                      |
// # |   3 | fn hash<H: Hasher>()        | Hasher ga ma'lumot yozish                  | Запись данных в Hasher                                      |
// # |   4 | Hash + PartialEq mos bo'lsin| Asosiy qoida                               | Основное правило                                            |
// #================================================================================================================================================#
// # |                                        HASHER TRAIT                                                                                          |
// #================================================================================================================================================#
// # |   5 | DefaultHasher               | Standart hasher — HashMap/HashSet uchun    | Стандартный хешер — для HashMap/HashSet                     |
// # |   6 | impl Hasher for T           | Custom hasher (FNV, SipHash, ...)          | Пользовательский хешер                                      |
// # |   7 | .finish() -> u64            | Hash qiymatini olish                       | Получение значения хеша                                     |
// # |   8 | .write(&[u8])               | Baytlarni hashga qo'shish                  | Добавление байтов в хеш                                     |
// #================================================================================================================================================#
// # |                                        HASHMAP VA HASHSET                                                                                    |
// #================================================================================================================================================#
// # |   9 | HashMap<K,V,S>              | S — BuildHasher (custom hasher)            | S — BuildHasher (пользовательский хешер)                    |
// # |  10 | .entry(k).or_insert(v)      | Kalit yo'q bo'lsa qo'shish                 | Добавить если ключ отсутствует                              |
// # |  11 | .entry(k).and_modify(f)     | Kalit bor bo'lsa o'zgartirish              | Изменить если ключ существует                               |
// # |  12 | set.intersection(&other)    | Kesishuv                                   | Пересечение                                                 |
// # |  13 | set.union(&other)           | Birlashuv                                  | Объединение                                                 |
// # |  14 | set.difference(&other)      | Farq                                       | Разность                                                    |
// # |  15 | BuildHasherDefault<H>       | Custom hasher HashMap uchun                | Пользовательский хешер для HashMap                          |
// #================================================================================================================================================#