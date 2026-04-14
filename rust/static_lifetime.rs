// #================================================================================================================================================#
// #                                                                    'STATIC LIFETIME                                                            #
// #                                'STATIC — DASTUR DAVOMIDA YASHAYDI. ENG UZUN LIFETIME. STRING LITERALLAR 'STATIC DIR.                           #
// #                                'STATIC — ЖИВЁТ В ТЕЧЕНИЕ ВСЕЙ ПРОГРАММЫ. САМЫЙ ДЛИННЫЙ LIFETIME. СТРОКОВЫЕ ЛИТЕРАЛЫ 'STATIC.                   #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// 'static — dastur boshlangandan oxirigacha yashaydi
// 'static — живёт от начала до конца программы
//
// Ikki xil 'static:
// Два вида 'static:
//
//   1. &'static T — static reference (xotirada saqlanadi)
//      &'static T — статическая ссылка (хранится в памяти)
//
//   2. T: 'static — T turi 'static lifetime talab qiladi
//      T: 'static — тип T требует 'static lifetime
//
// String literallar HAR DOIM 'static:
// Строковые литералы ВСЕГДА 'static:
//   let s: &'static str = "salom";
//   let s: &str = "salom";  // ikkalasi bir xil!

fn string_literal_misollari() {

    // &'static str — kompilyatsiya vaqtida binary ichiga yoziladi
    // &'static str — записывается в бинарник во время компиляции
    let s1: &'static str = "salom dunyo";
    let s2: &str = "bu ham static";
    println!("{}", s1);
    println!("{}", s2);
    // salom dunyo
    // bu ham static

    // static — dastur oxirigacha ishlaydi
    // static — работает до конца программы
    let r: &'static str;
    {
        r = "bu literal static";
        // r bu blokdan chiqsa ham ishlaydi!
        // r работает даже после выхода из блока!
    }
    println!("{}", r);
    // bu literal static

    // &'static str qaytaruvchi funksiya
    // функция возвращающая &'static str
    fn versiya() -> &'static str {
        "1.0.0"
    }
    fn platform() -> &'static str {
        if cfg!(target_os = "linux") { "Linux" }
        else if cfg!(target_os = "windows") { "Windows" }
        else { "Boshqa" }
    }
    println!("{}", versiya());
    println!("{}", platform());
    // 1.0.0
    // Linux
}

// static — global o'zgarmas qiymat
// static — глобальная неизменяемая переменная
static DASTUR_NOMI: &str = "MyApp";
static VERSIYA: &str = "2.0.0";
static MAX_ULANISH: u32 = 1000;
static PI: f64 = 3.141592653589793;

// static array
// статический массив
static KUNLAR: [&str; 7] = [
    "Dushanba", "Seshanba", "Chorshanba",
    "Payshanba", "Juma", "Shanba", "Yakshanba"
];

// static struct — const bilan
// статическая структура — с const
#[derive(Debug)]
struct Konfiguratsiya {
    port: u16,
    host: &'static str,
    debug: bool,
}

static STANDART_KONFIGURATSIYA: Konfiguratsiya = Konfiguratsiya {
    port: 8080,
    host: "localhost",
    debug: false,
};

fn static_ozgaruvchilar_misollari() {
    println!("{} v{}", DASTUR_NOMI, VERSIYA);
    println!("Max ulanish: {}", MAX_ULANISH);
    println!("PI: {}", PI);
    // MyApp v2.0.0
    // Max ulanish: 1000
    // PI: 3.141592653589793

    for kun in &KUNLAR {
        print!("{} ", kun);
    }
    println!();
    // Dushanba Seshanba Chorshanba Payshanba Juma Shanba Yakshanba

    println!("{:?}", STANDART_KONFIGURATSIYA);
    // Konfiguratsiya { port: 8080, host: "localhost", debug: false }
}

// T: 'static — T reference saqlamaydi yoki faqat 'static reference saqlaydi
// T: 'static — T не содержит ссылок или содержит только 'static ссылки
//
// 'static bo'lgan turlar:
// Типы с 'static:
//   i32, f64, bool, char     — Copy turlar
//   String, Vec<T>, Box<T>   — Owned turlar (reference yo'q)
//   &'static str             — static reference
//   fn() pointer             — funksiya pointer
//
// 'static BO'LMAGAN turlar:
// Типы БЕЗ 'static:
//   &'a str (non-static)     — vaqtinchalik reference
//   &'a T                    — vaqtinchalik reference

fn static_bound_misollari() {

    // T: 'static — thread::spawn uchun shart
    // T: 'static — обязательно для thread::spawn
    // thread::spawn(|| { ... }) — closure 'static bo'lishi kerak
    // thread::spawn(|| { ... }) — замыкание должно быть 'static

    // 'static turlar
    // типы 'static
    let son: i32 = 42;            // 'static ✅
    let matn: String = String::from("salom");  // 'static ✅
    let statik: &'static str = "literal";      // 'static ✅

    fn static_kabul_qil<T: 'static + fmt::Debug>(qiymat: T) {
        println!("{:?}", qiymat);
    }

    static_kabul_qil(42i32);
    static_kabul_qil(String::from("salom"));
    static_kabul_qil("literal");
    static_kabul_qil(vec![1, 2, 3]);
    // 42
    // "salom"
    // "literal"
    // [1, 2, 3]
}

// &'static str qaytaruvchi funksiyalar
// функции возвращающие &'static str
fn xato_xabari(kod: u16) -> &'static str {
    match kod {
        400 => "Noto'g'ri so'rov",
        401 => "Autentifikatsiya talab etiladi",
        403 => "Ruxsat yo'q",
        404 => "Topilmadi",
        500 => "Ichki server xatosi",
        _   => "Noma'lum xato",
    }
}

fn kun_nomi(raqam: u8) -> &'static str {
    match raqam {
        1 => "Dushanba",
        2 => "Seshanba",
        3 => "Chorshanba",
        4 => "Payshanba",
        5 => "Juma",
        6 => "Shanba",
        7 => "Yakshanba",
        _ => "Noma'lum",
    }
}

fn oy_nomi(raqam: u8) -> &'static str {
    const OYLAR: [&str; 12] = [
        "Yanvar", "Fevral", "Mart", "Aprel", "May", "Iyun",
        "Iyul", "Avgust", "Sentabr", "Oktabr", "Noyabr", "Dekabr",
    ];
    if raqam >= 1 && raqam <= 12 {
        OYLAR[(raqam - 1) as usize]
    } else {
        "Noma'lum"
    }
}

// &'static str field — struct 'static reference saqlaydi
// поле &'static str — структура хранит 'static ссылку
#[derive(Debug)]
struct XatoTuri {
    kod: u16,
    xabar: &'static str,
    tavsif: &'static str,
}

impl XatoTuri {
    const fn new(kod: u16, xabar: &'static str, tavsif: &'static str) -> Self {
        XatoTuri { kod, xabar, tavsif }
    }
}

impl fmt::Display for XatoTuri {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.kod, self.xabar, self.tavsif)
    }
}

// Compile-time konstantalar
// Константы времени компиляции
const TARMOQ_XATO: XatoTuri = XatoTuri::new(
    1001, "TARMOQ_XATO", "Tarmoqqa ulanib bo'lmadi"
);
const FAYL_XATO: XatoTuri = XatoTuri::new(
    1002, "FAYL_XATO", "Fayl topilmadi yoki o'qib bo'lmadi"
);
const RUXSAT_XATO: XatoTuri = XatoTuri::new(
    1003, "RUXSAT_XATO", "Operatsiyaga ruxsat yo'q"
);

fn thread_static_misollari() {
    use std::thread;

    // 'static owned qiymat — thread ga o'tkazish
    // owned 'static значение — передача в поток
    let matn: String = String::from("bu thread ichida");
    let handle = thread::spawn(move || {
        // matn moved — 'static ✅
        println!("{}", matn);
    });
    handle.join().unwrap();
    // bu thread ichida

    // &'static str — thread ga o'tkazish
    // &'static str — передача в поток
    let statik: &'static str = "statik literal";
    let handle2 = thread::spawn(move || {
        println!("{}", statik);
    });
    handle2.join().unwrap();
    // statik literal

    // static o'zgaruvchi — thread dan foydalanish
    // статическая переменная — использование из потока
    let handle3 = thread::spawn(|| {
        println!("{}", DASTUR_NOMI);
        println!("{}", MAX_ULANISH);
    });
    handle3.join().unwrap();
    // MyApp
    // 1000
}

use std::sync::OnceLock;

// OnceLock — bir marta initsializatsiya
// OnceLock — инициализация один раз
static GLOBAL_KONFIGURATSIYA: OnceLock<String> = OnceLock::new();

fn global_konfiguratsiyani_ol() -> &'static str {
    GLOBAL_KONFIGURATSIYA.get_or_init(|| {
        String::from("port=8080,host=localhost")
    })
}

// Box<dyn Error> aslida Box<dyn Error + 'static>
// Box<dyn Error> это фактически Box<dyn Error + 'static>
trait Ishlov {
    fn ishla(&self) -> String;
}

struct SonIshlov {
    qiymat: i32,
}

struct MatnIshlov {
    matn: String,
}

impl Ishlov for SonIshlov {
    fn ishla(&self) -> String {
        format!("Son: {}", self.qiymat * 2)
    }
}

impl Ishlov for MatnIshlov {
    fn ishla(&self) -> String {
        format!("Matn: {}", self.matn.to_uppercase())
    }
}

// Box<dyn Ishlov + 'static> — 'static bound
// Box<dyn Ishlov + 'static> — ограничение 'static
fn ishlovchi_yasash(tur: &str) -> Box<dyn Ishlov + 'static> {
    match tur {
        "son"  => Box::new(SonIshlov { qiymat: 21 }),
        _      => Box::new(MatnIshlov { matn: String::from("salom") }),
    }
}

// 1. Xato kodlari registri — static da saqlash
// 1. Реестр кодов ошибок — хранение в static
static XATO_TURLARI: [XatoTuri; 3] = [
    XatoTuri::new(1001, "TARMOQ", "Tarmoq xatosi"),
    XatoTuri::new(1002, "FAYL", "Fayl xatosi"),
    XatoTuri::new(1003, "RUXSAT", "Ruxsat xatosi"),
];

fn xato_topish(kod: u16) -> Option<&'static XatoTuri> {
    XATO_TURLARI.iter().find(|x| x.kod == kod)
}

// 2. Enum dan &'static str
// 2. &'static str из enum
#[derive(Debug)]
enum Holat {
    Faol,
    Nofaol,
    Kutmoqda,
    Bloklangan,
}

impl Holat {
    fn nomi(&self) -> &'static str {
        match self {
            Holat::Faol       => "FAOL",
            Holat::Nofaol     => "NOFAOL",
            Holat::Kutmoqda   => "KUTMOQDA",
            Holat::Bloklangan => "BLOKLANGAN",
        }
    }

    fn tavsif(&self) -> &'static str {
        match self {
            Holat::Faol       => "Foydalanuvchi faol",
            Holat::Nofaol     => "Foydalanuvchi nofaol",
            Holat::Kutmoqda   => "Tasdiqlash kutilmoqda",
            Holat::Bloklangan => "Foydalanuvchi bloklangan",
        }
    }
}

// 3. Static lookup table
// 3. Статическая таблица поиска
static ASCII_HARFLAR: [&str; 26] = [
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T",
    "U", "V", "W", "X", "Y", "Z",
];

fn harf_indeksi(harf: char) -> Option<usize> {
    let katta = harf.to_ascii_uppercase();
    if katta.is_ascii_alphabetic() {
        Some((katta as u8 - b'A') as usize)
    } else {
        None
    }
}

fn main() {

    string_literal_misollari();

    static_ozgaruvchilar_misollari();

    static_bound_misollari();

    // xato_xabari — &'static str qaytaradi
    // xato_xabari — возвращает &'static str
    println!("{}", xato_xabari(404));
    println!("{}", xato_xabari(500));
    println!("{}", xato_xabari(999));
    // Topilmadi
    // Ichki server xatosi
    // Noma'lum xato

    // kun_nomi va oy_nomi
    // kun_nomi и oy_nomi
    for i in 1..=7 {
        print!("{} ", kun_nomi(i));
    }
    println!();
    // Dushanba Seshanba Chorshanba Payshanba Juma Shanba Yakshanba

    for i in 1..=12 {
        print!("{} ", oy_nomi(i));
    }
    println!();
    // Yanvar Fevral Mart Aprel May Iyun Iyul Avgust Sentabr Oktabr Noyabr Dekabr

    // XatoTuri — static field
    // XatoTuri — static поля
    println!("{}", TARMOQ_XATO);
    println!("{}", FAYL_XATO);
    println!("{}", RUXSAT_XATO);
    // [1001] TARMOQ_XATO: Tarmoqqa ulanib bo'lmadi
    // [1002] FAYL_XATO: Fayl topilmadi yoki o'qib bo'lmadi
    // [1003] RUXSAT_XATO: Operatsiyaga ruxsat yo'q

    thread_static_misollari();

    // OnceLock — bir marta initsializatsiya
    // OnceLock — инициализация один раз
    let config1: &str = global_konfiguratsiyani_ol();
    let config2: &str = global_konfiguratsiyani_ol();
    println!("{}", config1);
    println!("{}", config2);
    println!("Bir xilmi: {}", std::ptr::eq(config1, config2));
    // port=8080,host=localhost
    // port=8080,host=localhost
    // Bir xilmi: true  ← bir xil xotira manzili!

    // Box<dyn Ishlov + 'static>
    // Box<dyn Ishlov + 'static>
    let son_ishlov: Box<dyn Ishlov> = ishlovchi_yasash("son");
    let matn_ishlov: Box<dyn Ishlov> = ishlovchi_yasash("matn");
    println!("{}", son_ishlov.ishla());
    println!("{}", matn_ishlov.ishla());
    // Son: 42
    // Matn: SALOM

    // 1. Xato kodlari registri
    // 1. Реестр кодов ошибок
    println!("{:?}", xato_topish(1001));
    println!("{:?}", xato_topish(9999));
    // Some(XatoTuri { kod: 1001, xabar: "TARMOQ", tavsif: "Tarmoq xatosi" })
    // None

    // 2. Enum dan static str
    // 2. Статическая строка из enum
    let holatlar: Vec<Holat> = vec![
        Holat::Faol,
        Holat::Nofaol,
        Holat::Kutmoqda,
        Holat::Bloklangan,
    ];
    for h in &holatlar {
        println!("{}: {}", h.nomi(), h.tavsif());
    }
    // FAOL: Foydalanuvchi faol
    // NOFAOL: Foydalanuvchi nofaol
    // KUTMOQDA: Tasdiqlash kutilmoqda
    // BLOKLANGAN: Foydalanuvchi bloklangan

    // 3. Static lookup table
    // 3. Статическая таблица поиска
    for harf in ['R', 'u', 's', 't'] {
        if let Some(i) = harf_indeksi(harf) {
            print!("{} ", ASCII_HARFLAR[i]);
        }
    }
    println!();
    // R U S T
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                       'STATIC ASOSLARI                                                                                       |
// #================================================================================================================================================#
// # |   1 | &'static str             | String literal — dastur oxirigacha yashaydi          | Строковый литерал — живёт до конца программы         |
// # |   2 | &'static T               | Static reference — eng uzun lifetime                 | Статическая ссылка — самый длинный lifetime          |
// # |   3 | T: 'static               | T reference saqlamaydi yoki static reference         | T не содержит ссылок или только static               |
// # |   4 | static X: T              | Global o'zgarmas — dastur davomida yashaydi          | Глобальная константа — живёт всю программу           |
// #================================================================================================================================================#
// # |                                       STATIC O'ZGARUVCHILAR                                                                                  |
// #================================================================================================================================================#
// # |   5 | static X: &str = "..."   | &'static str saqlovchi global                        | Глобальная переменная с &'static str                 |
// # |   6 | static X: u32 = 100      | Global son konstanta                                 | Глобальная числовая константа                        |
// # |   7 | static ARRAY: [T; N]     | Global array                                         | Глобальный массив                                    |
// # |   8 | const fn new(...)        | Compile-time const struct yaratish                   | Создание const структуры во время компиляции         |
// #================================================================================================================================================#
// # |                                       QACHON T: 'STATIC KERAK                                                                                |
// #================================================================================================================================================#
// # |   9 | thread::spawn            | Closure 'static bo'lishi shart                       | Замыкание должно быть 'static                        |
// # |  10 | Box<dyn Error>           | Aslida Box<dyn Error + 'static>                      | Фактически Box<dyn Error + 'static>                  |
// # |  11 | OnceLock<T>              | T: 'static bo'lishi kerak                            | T должен быть 'static                                |
// # |  12 | Arc<T>                   | T: 'static bo'lishi tavsiya etiladi                  | T рекомендуется быть 'static                         |
// #================================================================================================================================================#
// # |                                       'STATIC TURLAR                                                                                         |
// #================================================================================================================================================#
// # |  13 | i32, f64, bool, char     | Copy turlar — 'static ✅                             | Copy типы — 'static ✅                               |
// # |  14 | String, Vec<T>, Box<T>   | Owned turlar — 'static ✅                            | Owned типы — 'static ✅                              |
// # |  15 | &'static str             | Static reference — 'static ✅                        | Статическая ссылка — 'static ✅                      |
// # |  16 | &'a str (non-static)     | Vaqtinchalik reference — 'static EMAS ❌             | Временная ссылка — НЕ 'static ❌                     |
// #================================================================================================================================================#
// # |                                       REAL HAYOT                                                                                             |
// #================================================================================================================================================#
// # |  17 | Xato xabarlari           | match → &'static str qaytarish                       | Возврат &'static str через match                     |
// # |  18 | Enum.nomi()              | Enum variantdan static string                        | Статическая строка из варианта enum                  |
// # |  19 | Lookup tables            | static array da ma'lumot saqlash                     | Хранение данных в статическом массиве                |
// # |  20 | OnceLock<String>         | Bir marta initsializatsiya + 'static reference       | Одноразовая инициализация + 'static ссылка           |
// #================================================================================================================================================#