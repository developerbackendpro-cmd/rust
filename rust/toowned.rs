// #================================================================================================================================================#
// #                                                                TOOWNED                                                                         #
// #                        TOOWNED — BORROWED TURDAN OWNED TUR YARATISH. CLONE DAN FARQI: DST TURLAR UCHUN HAM ISHLAYDI.                           #
// #                        TOOWNED — СОЗДАНИЕ OWNED ТИПА ИЗ BORROWED. ОТЛИЧИЕ ОТ CLONE: РАБОТАЕТ И С DST ТИПАМИ.                                   #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::borrow::ToOwned;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt;

// ToOwned — borrowed turdan owned tur yaratish
// ToOwned — создание owned типа из borrowed типа
//
// Clone   — &T → T (bir xil tur, T: Clone bo'lishi kerak)
// ToOwned — &T → T::Owned (boshqa tur bo'lishi mumkin — DST uchun!)
//
// MUHIM QOIDA:
// ВАЖНОЕ ПРАВИЛО:
//   T: Clone bo'lsa → ToOwned avtomatik keladi (blanket impl)
//   impl<T: Clone> ToOwned for T { type Owned = T; }
//
// Shuning uchun Custom ToOwned faqat Clone bo'LMAGAN DST uchun yoziladi
// Поэтому Custom ToOwned пишется только для DST без Clone
//
// Eng muhim built-in:
// Наиболее важные встроенные:
//   str  → String  (str: ToOwned<Owned = String>)
//   [T]  → Vec<T>  ([T]: ToOwned<Owned = Vec<T>>)

fn built_in_toowned_misollari() {

    // &str → String (eng ko'p ishlatiladi)
    // &str → String (используется чаще всего)
    let str_ref: &str = "salom dunyo";
    let string_owned: String = str_ref.to_owned();
    println!("{}", string_owned);
    println!("&str hajmi:   {} bayt", std::mem::size_of_val(str_ref));
    println!("String hajmi: {} bayt", std::mem::size_of::<String>());
    // salom dunyo
    // &str hajmi:   11 bayt
    // String hajmi: 24 bayt

    // &[T] → Vec<T>
    // &[T] → Vec<T>
    let slice: &[i32] = &[1, 2, 3, 4, 5];
    let vektor: Vec<i32> = slice.to_owned();
    println!("{:?}", vektor);
    // [1, 2, 3, 4, 5]

    // &[u8] → Vec<u8>
    // &[u8] → Vec<u8>
    let baytlar: &[u8] = &[72, 101, 108, 108, 111];
    let baytlar_owned: Vec<u8> = baytlar.to_owned();
    println!("{:?}", baytlar_owned);
    // [72, 101, 108, 108, 111]

    // &i32 → i32 (Copy turlar — to_owned = clone)
    // &i32 → i32 (Copy типы — to_owned = clone)
    let son_ref: &i32 = &42;
    let son_owned: i32 = son_ref.to_owned();
    println!("{}", son_owned);
    // 42

    // &bool → bool
    // &bool → bool
    let bool_ref: &bool = &true;
    let bool_owned: bool = bool_ref.to_owned();
    println!("{}", bool_owned);
    // true

    // &String → String
    // &String → String
    let string_ref: &String = &String::from("rust");
    let string_owned2: String = string_ref.to_owned();
    println!("{}", string_owned2);
    // rust

    // &Vec<T> → Vec<T>
    // &Vec<T> → Vec<T>
    let vec_ref: &Vec<i32> = &vec![1, 2, 3];
    let vec_owned: Vec<i32> = vec_ref.to_owned();
    println!("{:?}", vec_owned);
    // [1, 2, 3]
}

fn to_owned_vs_clone() {
    let s: &str = "salom";

    // &str.clone() → &str (bir xil reference tur)
    // &str.clone() → &str (тот же тип ссылки)
    let s_clone: &str = s;
    println!("clone: {}", s_clone);
    // clone: salom

    // &str.to_owned() → String (BOSHQA tur — owned!)
    // &str.to_owned() → String (ДРУГОЙ тип — owned!)
    let s_owned: String = s.to_owned();
    println!("to_owned: {}", s_owned);
    // to_owned: salom

    // slice uchun farq yaqqol ko'rinadi
    // для slice разница особенно заметна
    let slice: &[i32] = &[1, 2, 3];

    // slice.clone() → &[i32] (bir xil reference)
    // slice.clone() → &[i32] (та же ссылка)
    let slice_clone: &[i32] = slice;
    println!("slice clone: {:?}", slice_clone);
    // slice clone: [1, 2, 3]

    // slice.to_owned() → Vec<i32> (owned tur!)
    // slice.to_owned() → Vec<i32> (owned тип!)
    let slice_owned: Vec<i32> = slice.to_owned();
    println!("slice to_owned: {:?}", slice_owned);
    // slice to_owned: [1, 2, 3]

    // String uchun to_owned == clone (ikkalasi String qaytaradi)
    // для String to_owned == clone (оба возвращают String)
    let string: String = String::from("rust");
    let clone1: String = string.clone();
    let owned1: String = string.to_owned();
    println!("{} {}", clone1, owned1);
    // rust rust
}

// std da mavjud:
// существует в std:
//   impl<T: Clone> ToOwned for T {
//       type Owned = T;
//       fn to_owned(&self) -> T { self.clone() }
//   }
//
// Bu degani: Clone bo'lgan BARCHA turlar uchun ToOwned allaqachon bor!
// Это значит: ToOwned уже есть для ВСЕХ типов с Clone!
//
// Shuning uchun:
// Поэтому:
//   ❌ impl ToOwned for Talaba  — xato (blanket impl bilan ziddiyat)
//   ❌ impl ToOwned for str     — xato (orphan rule)
//   ✅ Clone implement qil, ToOwned o'zi keladi!
//   ✅ Реализуй Clone, ToOwned придёт сам!

#[derive(Debug, Clone)]
struct Talaba {
    ism: String,
    baho: f64,
}

impl fmt::Display for Talaba {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.ism, self.baho)
    }
}

fn talaba_toowned_misoli() {
    let talaba = Talaba {
        ism: String::from("Dilshod"),
        baho: 9.5,
    };

    // Clone bo'lgani uchun to_owned avtomatik ishlaydi
    // to_owned работает автоматически так как есть Clone
    let talaba_owned: Talaba = talaba.to_owned();
    println!("{}", talaba_owned);
    // Dilshod (9.5)
}

fn cow_toowned_misollari() {
    // &str → Cow<str>
    // &str → Cow<str>
    let cow1: Cow<str> = Cow::Borrowed("salom");
    println!("{}", cow1);
    println!("Borrowed: {}", matches!(cow1, Cow::Borrowed(_)));
    // salom
    // Borrowed: true

    // to_owned — Cow ni owned ga o'tkazish
    // to_owned — перевод Cow в owned
    let cow2: Cow<str> = Cow::Borrowed("dunyo");
    let owned: String = cow2.into_owned();
    println!("{}", owned);
    // dunyo

    // into_owned — har doim String qaytaradi
    // into_owned — всегда возвращает String
    let cow3: Cow<str> = Cow::Borrowed("rust");
    let owned2: String = cow3.into_owned();
    println!("{}", owned2);
    // rust

    // &[T] bilan Cow
    // Cow с &[T]
    let slice: &[i32] = &[1, 2, 3];
    let cow4: Cow<[i32]> = Cow::Borrowed(slice);
    println!("{:?}", cow4);
    // [1, 2, 3]

    let owned3: Vec<i32> = cow4.into_owned();
    println!("{:?}", owned3);
    // [1, 2, 3]

    // Cow — lazy clone: faqat kerak bo'lganda
    // Cow — ленивое клонирование: только когда нужно
    let mut cow5: Cow<str> = Cow::Borrowed("salom dunyo");
    let uzunlik: usize = cow5.len();
    println!("Uzunlik (clone yo'q): {}", uzunlik);
    // Uzunlik (clone yo'q): 11
}

// T: ToOwned + ?Sized — DST turlar bilan ishlash
// T: ToOwned + ?Sized — работа с DST типами
fn nusxa_ol<T>(qiymat: &T) -> T::Owned
where
    T: ToOwned + ?Sized,
{
    qiymat.to_owned()
}

// shartli nusxa — Cow bilan
// условная копия — с Cow
fn shartli_nusxa<'a>(matn: &'a str, kerakmi: bool) -> Cow<'a, str> {
    if kerakmi {
        Cow::Owned(matn.to_owned())
    } else {
        Cow::Borrowed(matn)
    }
}

// 1. Struct ichida &str ni String ga saqlash
// 1. Сохранение &str как String внутри структуры
#[derive(Debug)]
struct Konfiguratsiya {
    nomi: String,
    qiymat: String,
}

impl Konfiguratsiya {
    fn new(nomi: &str, qiymat: &str) -> Self {
        Konfiguratsiya {
            nomi: nomi.to_owned(),
            qiymat: qiymat.to_owned(),
        }
    }
}

// 2. Cache tizimi
// 2. Система кэша
struct Cache {
    ichki: std::collections::HashMap<String, String>,
}

impl Cache {
    fn new() -> Self {
        Cache { ichki: std::collections::HashMap::new() }
    }

    fn qosh(&mut self, kalit: &str, qiymat: &str) {
        self.ichki.insert(kalit.to_owned(), qiymat.to_owned());
    }

    fn ol(&self, kalit: &str) -> Option<&String> {
        self.ichki.get(kalit)
    }
}

// 3. &str larni String ga aylantirish — iterator bilan
// 3. Преобразование &str в String — с итератором
fn str_larni_string_ga_aylantir(sozlar: &[&str]) -> Vec<String> {
    sozlar.iter().map(|s| s.to_owned().to_string()).collect()
}

// 4. Xato xabarlarini saqlash
// 4. Сохранение сообщений об ошибках
struct XatoLog {
    xabarlar: Vec<String>,
}

impl XatoLog {
    fn new() -> Self {
        XatoLog { xabarlar: Vec::new() }
    }

    fn qosh(&mut self, xabar: &str) {
        self.xabarlar.push(xabar.to_owned());
    }

    fn chiqar(&self) {
        for (i, xabar) in self.xabarlar.iter().enumerate() {
            println!("{}: {}", i + 1, xabar);
        }
    }
}

// 5. Lazy clone — Cow bilan
// 5. Ленивое клонирование — с Cow
fn xavfsiz_tahrirla<'a>(matn: &'a str, tahrirlansinmi: bool) -> Cow<'a, str> {
    if tahrirlansinmi {
        Cow::Owned(format!("[TAHRIRLANGAN] {}", matn))
    } else {
        Cow::Borrowed(matn)
    }
}

fn main() {
    built_in_toowned_misollari();

    to_owned_vs_clone();

    talaba_toowned_misoli();

    cow_toowned_misollari();

    // nusxa_ol — &str → String
    // nusxa_ol — &str → String
    let str_nusxa: String = nusxa_ol("salom");
    println!("{}", str_nusxa);
    // salom

    // nusxa_ol — &[i32] → Vec<i32>
    // nusxa_ol — &[i32] → Vec<i32>
    let slice_nusxa: Vec<i32> = nusxa_ol([1, 2, 3].as_ref());
    println!("{:?}", slice_nusxa);
    // [1, 2, 3]

    // shartli_nusxa
    // shartli_nusxa
    let kerak: Cow<str> = shartli_nusxa("salom", true);
    let kerak_emas: Cow<str> = shartli_nusxa("dunyo", false);
    println!("{} (owned: {})", kerak, matches!(kerak, Cow::Owned(_)));
    println!("{} (borrowed: {})", kerak_emas, matches!(kerak_emas, Cow::Borrowed(_)));
    // salom (owned: true)
    // dunyo (borrowed: true)

    // 1. Konfiguratsiya — &str → String saqlash
    // 1. Конфигурация — сохранение &str как String
    let k1 = Konfiguratsiya::new("port", "8080");
    let k2 = Konfiguratsiya::new("host", "localhost");
    println!("{:#?}", k1);
    println!("{:#?}", k2);
    // Konfiguratsiya { nomi: "port", qiymat: "8080" }
    // Konfiguratsiya { nomi: "host", qiymat: "localhost" }

    // 2. Cache
    // 2. Кэш
    let mut cache = Cache::new();
    cache.qosh("user:1", "Dilshod");
    cache.qosh("user:2", "Ali");
    println!("{:?}", cache.ol("user:1"));
    println!("{:?}", cache.ol("user:3"));
    // Some("Dilshod")
    // None

    // 3. &str → Vec<String>
    // 3. &str → Vec<String>
    let str_listi: &[&str] = &["salom", "dunyo", "rust"];
    let string_listi: Vec<String> = str_larni_string_ga_aylantir(str_listi);
    println!("{:?}", string_listi);
    // ["salom", "dunyo", "rust"]

    // 4. XatoLog
    // 4. XatoLog
    let mut log = XatoLog::new();
    log.qosh("fayl topilmadi");
    log.qosh("ulanish rad etildi");
    log.qosh("xotira yetarli emas");
    log.chiqar();
    // 1: fayl topilmadi
    // 2: ulanish rad etildi
    // 3: xotira yetarli emas

    // 5. Lazy clone — Cow bilan
    // 5. Ленивое клонирование — с Cow
    let asl_matn: &str = "bu matnga tegilmasin";
    let tahrirlangan: Cow<str> = xavfsiz_tahrirla(asl_matn, true);
    let tahrir_yoq: Cow<str> = xavfsiz_tahrirla(asl_matn, false);
    println!("{}", tahrirlangan);
    println!("{}", tahrir_yoq);
    // [TAHRIRLANGAN] bu matnga tegilmasin
    // bu matnga tegilmasin

    // struct fieldga &str saqlashda
    // при сохранении &str в поле структуры
    let nomi: &str = "Dilshod";
    let struct_ism: String = nomi.to_owned();
    println!("{}", struct_ism);
    // Dilshod

    // HashMap ga &str ni kalit sifatida saqlashda
    // при сохранении &str как ключа в HashMap
    let mut xarita: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    let kalit: &str = "salom";
    xarita.insert(kalit.to_owned(), 42);
    println!("{:?}", xarita.get("salom"));
    // Some(42)

    // iterator da &str → String
    // в итераторе &str → String
    let sozlar: Vec<&str> = vec!["bir", "ikki", "uch"];
    let owned_sozlar: Vec<String> = sozlar.iter()
        .map(|s| s.to_owned().to_string())
        .collect();
    println!("{:?}", owned_sozlar);
    // ["bir", "ikki", "uch"]
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                          TOOWNED TRAIT                                                                                       |
// #================================================================================================================================================#
// # |   1 | .to_owned()              | &T → T::Owned (owned nusxa olish)                    | &T → T::Owned (получение owned копии)                |
// # |   2 | T: Clone → ToOwned bepul | Clone implement qilsak ToOwned avtomatik keladi       | Реализуя Clone, ToOwned приходит автоматически      |
// # |   3 | type Owned = T;          | Clone turlarda Owned = T (bir xil tur)               | Для Clone типов Owned = T (тот же тип)               |
// #================================================================================================================================================#
// # |                                    BUILT-IN IMPLEMENT                                                                                        |
// #================================================================================================================================================#
// # |   4 | str.to_owned()           | &str → String (eng ko'p ishlatiladi)                 | &str → String (наиболее часто)                       |
// # |   5 | [T].to_owned()           | &[T] → Vec<T>                                        | &[T] → Vec<T>                                        |
// # |   6 | i32.to_owned()           | &i32 → i32 (Copy turlar — sodda nusxa)               | &i32 → i32 (Copy типы — простая копия)               |
// # |   7 | String.to_owned()        | &String → String (clone bilan bir xil)               | &String → String (то же что clone)                   |
// #================================================================================================================================================#
// # |                                    TO_OWNED VS CLONE                                                                                         |
// #================================================================================================================================================#
// # |   8 | Clone                    | &T → T (bir xil tur, T: Clone bo'lishi kerak)        | &T → T (тот же тип, T должен реализовать Clone)      |
// # |   9 | ToOwned                  | &T → T::Owned (tur farqli bo'lishi mumkin)           | &T → T::Owned (тип может отличаться)                 |
// # |  10 | &str.clone()             | &str (reference copy — Clone emas!)                  | &str (копия ссылки — не Clone!)                      |
// # |  11 | &str.to_owned()          | String (owned tur — farqli tur!)                     | String (owned тип — другой тип!)                     |
// # |  12 | &[T].clone()             | &[T] (reference copy)                                | &[T] (копия ссылки)                                  |
// # |  13 | &[T].to_owned()          | Vec<T> (owned tur — farqli tur!)                     | Vec<T> (owned тип — другой тип!)                     |
// #================================================================================================================================================#
// # |                                    BLANKET IMPL QOIDASI                                                                                      |
// #================================================================================================================================================#
// # |  14 | impl<T:Clone> ToOwned    | std da bor — barcha Clone turlar uchun               | Есть в std — для всех типов с Clone                  |
// # |  15 | Custom ToOwned yozma     | Clone implement qil — ToOwned o'zi keladi            | Реализуй Clone — ToOwned придёт сам                  |
// # |  16 | DST uchun custom         | str, [T] — std da implement qilingan                 | str, [T] — реализовано в std                         |
// #================================================================================================================================================#
// # |                                       COW BILAN                                                                                              |
// #================================================================================================================================================#
// # |  17 | Cow<'a, B: ToOwned>      | Cow asosi — ToOwned                                  | Основа Cow — ToOwned                                 |
// # |  18 | cow.into_owned()         | Cow → B::Owned (har doim owned)                      | Cow → B::Owned (всегда owned)                        |
// # |  19 | Cow::Borrowed + len()    | Clone yo'q — faqat o'qish                            | Без clone — только чтение                            |
// # |  20 | Cow::Owned               | to_owned chaqirilgan — clone bo'lgan                 | to_owned вызван — clone произошёл                    |
// #================================================================================================================================================#
// # |                                    REAL HAYOT QOLLASH                                                                                        |
// #================================================================================================================================================#
// # |  21 | nomi.to_owned()          | &str → String struct fieldga saqlash                 | &str → String для поля структуры                     |
// # |  22 | kalit.to_owned()         | &str → String HashMap ga saqlash                     | &str → String для HashMap                            |
// # |  23 | .map(|s| s.to_owned())   | Iterator da &str → String                           | &str → String в итераторе                             |
// # |  24 | xabar.to_owned()         | Log va xato xabarlarini saqlash                      | Сохранение лога и сообщений об ошибках               |
// #================================================================================================================================================#
// # |                                    GENERIC BILAN                                                                                             |
// #================================================================================================================================================#
// # |  25 | T: ToOwned + ?Sized      | DST turlar bilan generic funksiya                    | Generic функция с DST типами                         |
// # |  26 | Cow<'a, str>             | &str yoki String — kerak bo'lganda clone             | &str или String — clone только когда нужно           |
// #================================================================================================================================================#