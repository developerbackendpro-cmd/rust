// #================================================================================================================================================#
// #                                                                SIZED  |  ?SIZED                                                                #
// #                            SIZED — KOMPILE VAQTIDA O'LCHAMI MA'LUM TUR. ?SIZED — O'LCHAMI NOMA'LUM BO'LISHI MUMKIN.                            #
// #                            SIZED — ТИП С ИЗВЕСТНЫМ РАЗМЕРОМ НА ЭТАПЕ КОМПИЛЯЦИИ. ?SIZED — РАЗМЕР МОЖЕТ БЫТЬ НЕИЗВЕСТЕН.                        #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// Rust da har bir generic T avtomatik Sized bo'ladi
// В Rust каждый generic T автоматически Sized
// fn f<T>(x: T) {}  →  aslida: fn f<T: Sized>(x: T) {}

// Sized turlar — kompile vaqtida o'lchami ma'lum
// Sized типы — размер известен во время компиляции
// i32    = 4 bayt
// f64    = 8 bayt
// bool   = 1 bayt
// char   = 4 bayt
// &str   = 16 bayt (ptr + len)
// String = 24 bayt (ptr + len + cap)

// Sized EMAS turlar (DST — Dynamically Sized Types)
// Типы без Sized (DST — динамически размерные типы)
// str        — uzunligi ma'lum emas
// [T]        — uzunligi ma'lum emas
// dyn Trait  — qaysi struct ekanligini bilmaymiz

// Sized turlar — o'lchami
// Размеры Sized типов
fn sized_olchamlar() {
    let i32_hajmi: usize = std::mem::size_of::<i32>();
    let f64_hajmi: usize = std::mem::size_of::<f64>();
    let bool_hajmi: usize = std::mem::size_of::<bool>();
    let char_hajmi: usize = std::mem::size_of::<char>();
    let str_ref_hajmi: usize = std::mem::size_of::<&str>();
    let string_hajmi: usize = std::mem::size_of::<String>();
    let vec_hajmi: usize = std::mem::size_of::<Vec<i32>>();

    println!("i32    : {} bayt", i32_hajmi);
    println!("f64    : {} bayt", f64_hajmi);
    println!("bool   : {} bayt", bool_hajmi);
    println!("char   : {} bayt", char_hajmi);
    println!("&str   : {} bayt", str_ref_hajmi);
    println!("String : {} bayt", string_hajmi);
    println!("Vec    : {} bayt", vec_hajmi);
    // i32    : 4 bayt
    // f64    : 8 bayt
    // bool   : 1 bayt
    // char   : 4 bayt
    // &str   : 16 bayt
    // String : 24 bayt
    // Vec    : 24 bayt
}

// standart generic — T: Sized avtomatik
// стандартный generic — T: Sized автоматически
fn chop_et<T: fmt::Debug>(qiymat: T) {
    println!("{:?}", qiymat);
}

// T: Sized + boshqa trait
// T: Sized + другой трейт
fn ikki_baravar<T: std::ops::Add<Output = T> + Copy>(x: T) -> T {
    x + x
}

// Sized — struct uchun
// Sized для структуры
struct Quti<T> {
    ichki: T,
}

impl<T: fmt::Debug> Quti<T> {
    fn new(qiymat: T) -> Self {
        Quti { ichki: qiymat }
    }

    fn ko_rsat(&self) {
        println!("{:?}", self.ichki);
    }
}

// ?Sized — Sized bo'lishi ham, bo'lmasligi ham mumkin
// ?Sized — может быть Sized или нет
fn uzunlik<T: ?Sized>(qiymat: &T) -> usize {
    std::mem::size_of_val(qiymat)
}

// ?Sized — str va String ikkalasini qabul qilish
// ?Sized — принимает и str и String
fn chiqar<T: ?Sized + fmt::Display>(qiymat: &T) {
    println!("{}", qiymat);
}

// ?Sized — struct da
// ?Sized в структуре
// T: ?Sized bo'lganda T faqat reference orqali ishlatiladi
// при T: ?Sized, T используется только через ссылку
struct Wrapper<T: ?Sized> {
    qiymat: Box<T>,
}

impl<T: ?Sized + fmt::Display> Wrapper<T> {
    fn ko_rsat(&self) {
        println!("{}", self.qiymat);
    }
}

// str — DST, faqat & yoki Box orqali ishlatiladi
// str — DST, используется только через & или Box
fn str_bilan_ishlash(s: &str) {
    println!("uzunlik: {}", s.len());
    println!("bayt: {}", std::mem::size_of_val(s));
}

// [T] — DST, faqat & yoki Box orqali
// [T] — DST, только через & или Box
fn slice_bilan_ishlash(s: &[i32]) {
    println!("elementlar: {}", s.len());
    println!("bayt: {}", std::mem::size_of_val(s));
}

// dyn Trait — DST
// dyn Trait — DST
trait Hayvon {
    fn ovoz(&self) -> &str;
}

struct It;
struct Mushuk;

impl Hayvon for It {
    fn ovoz(&self) -> &str { "Hav!" }
}

impl Hayvon for Mushuk {
    fn ovoz(&self) -> &str { "Miyov!" }
}

fn hayvon_ovozi(hayvon: &dyn Hayvon) {
    println!("{}", hayvon.ovoz());
    println!("dyn Hayvon hajmi: {}", std::mem::size_of_val(hayvon));
}

// size_of — tur hajmi (Sized talab qiladi)
// size_of — размер типа (требует Sized)
fn size_of_misol() {
    let i32_hajmi: usize = std::mem::size_of::<i32>();
    let string_hajmi: usize = std::mem::size_of::<String>();
    println!("i32 hajmi: {}", i32_hajmi);
    println!("String hajmi: {}", string_hajmi);
    // i32 hajmi: 4
    // String hajmi: 24
}

// size_of_val — qiymat hajmi (?Sized bilan ishlaydi)
// size_of_val — размер значения (работает с ?Sized)
fn size_of_val_misol() {
    let str_val: &str = "salom";
    let slice_val: &[i32] = &[1, 2, 3, 4, 5];
    let son: i32 = 42;

    let str_hajmi: usize = std::mem::size_of_val(str_val);
    let slice_hajmi: usize = std::mem::size_of_val(slice_val);
    let son_hajmi: usize = std::mem::size_of_val(&son);

    println!("\"salom\" hajmi: {} bayt", str_hajmi);
    println!("[1,2,3,4,5] hajmi: {} bayt", slice_hajmi);
    println!("42 hajmi: {} bayt", son_hajmi);
    // "salom" hajmi: 5 bayt
    // [1,2,3,4,5] hajmi: 20 bayt
    // 42 hajmi: 4 bayt
}

// Box<str> — heap da str
// Box<str> — str в куче
fn box_str_misol() {
    let s: Box<str> = "salom dunyo".into();
    println!("{}", s);
    println!("{}", s.len());
    // salom dunyo
    // 11
}

// Box<[T]> — heap da slice
// Box<[T]> — slice в куче
fn box_slice_misol() {
    let v: Box<[i32]> = vec![1, 2, 3, 4, 5].into_boxed_slice();
    println!("{:?}", v);
    println!("{}", v.len());
    // [1, 2, 3, 4, 5]
    // 5
}

// Box<dyn Trait> — heap da trait object
// Box<dyn Trait> — объект трейта в куче
fn box_dyn_misol() {
    let hayvonlar: Vec<Box<dyn Hayvon>> = vec![
        Box::new(It),
        Box::new(Mushuk),
        Box::new(It),
    ];

    for hayvon in &hayvonlar {
        println!("{}", hayvon.ovoz());
    }
    // Hav!
    // Miyov!
    // Hav!
}

// ToString trait — ?Sized ishlatadi
// трейт ToString — использует ?Sized
fn string_ga_aylantir<T: fmt::Display + ?Sized>(qiymat: &T) -> String {
    format!("{}", qiymat)
}

// generic log funksiya — str va String ikkalasini qabul qiladi
// обобщённая функция логирования — принимает str и String
fn log<T: fmt::Display + ?Sized>(daraja: &str, xabar: &T) {
    println!("[{}] {}", daraja, xabar);
}

// Rc<T: ?Sized> — reference counting DST uchun
// Rc<T: ?Sized> — подсчёт ссылок для DST
fn rc_dst_misol() {
    use std::rc::Rc;

    let rc_str: Rc<str> = Rc::from("salom");
    let rc_slice: Rc<[i32]> = Rc::from(vec![1, 2, 3].as_slice());

    println!("{}", rc_str);
    println!("{:?}", rc_slice);
    // salom
    // [1, 2, 3]
}

fn main() {
    sized_olchamlar();

    // standart generic — Sized avtomatik
    // стандартный generic — Sized автоматически
    chop_et(42i32);
    chop_et("salom");
    chop_et(vec![1, 2, 3]);
    // 42
    // "salom"
    // [1, 2, 3]

    // ikki baravar
    // удвоение
    let ikkilangan_son: i32 = ikki_baravar(5i32);
    let ikkilangan_f64: f64 = ikki_baravar(3.14f64);
    println!("{}", ikkilangan_son);
    println!("{}", ikkilangan_f64);
    // 10
    // 6.28

    // Quti struct
    let quti_son: Quti<i32> = Quti::new(42);
    let quti_matn: Quti<String> = Quti::new(String::from("salom"));
    quti_son.ko_rsat();
    quti_matn.ko_rsat();
    // 42
    // "salom"

    // uzunlik — Sized va ?Sized ikkalasi
    // узунлик — и Sized и ?Sized
    let son: i32 = 42;
    let matn: &str = "salom";
    let slice: &[i32] = &[1, 2, 3];

    let son_hajmi: usize = uzunlik(&son);
    let matn_hajmi: usize = uzunlik(matn);
    let slice_hajmi: usize = uzunlik(slice);

    println!("i32 hajmi: {}", son_hajmi);
    println!("str hajmi: {}", matn_hajmi);
    println!("[i32;3] hajmi: {}", slice_hajmi);
    // i32 hajmi: 4
    // str hajmi: 5
    // [i32;3] hajmi: 12

    // chiqar — str va String ikkalasi
    // вывод — и str и String
    let str_literal: &str = "men str man";
    let string_qiymati: String = String::from("men String man");

    chiqar(str_literal);
    chiqar(&string_qiymati);
    chiqar("to'g'ridan-to'g'ri literal");
    // men str man
    // men String man
    // to'g'ridan-to'g'ri literal

    // Wrapper — Box<str> va Box<[T]>
    // Wrapper — Box<str> и Box<[T]>
    let wrapper_str: Wrapper<str> = Wrapper {
        qiymat: Box::from("salom dunyo"),
    };
    wrapper_str.ko_rsat();
    // salom dunyo

    // str — DST
    str_bilan_ishlash("salom dunyo");
    // uzunlik: 11
    // bayt: 11

    // [T] — DST
    slice_bilan_ishlash(&[1, 2, 3, 4, 5]);
    // elementlar: 5
    // bayt: 20

    // dyn Trait — DST
    let it = It;
    let mushuk = Mushuk;
    hayvon_ovozi(&it);
    hayvon_ovozi(&mushuk);
    // Hav!
    // Miyov!

    size_of_misol();
    size_of_val_misol();

    box_str_misol();
    box_slice_misol();
    box_dyn_misol();

    // string_ga_aylantir — str va String
    // string_ga_aylantir — str и String
    let str_natija: String = string_ga_aylantir("salom");
    let string_natija: String = string_ga_aylantir(&String::from("dunyo"));
    let son_natija: String = string_ga_aylantir(&42i32);
    println!("{}", str_natija);
    println!("{}", string_natija);
    println!("{}", son_natija);
    // salom
    // dunyo
    // 42

    // log — turli turlar
    // log — разные типы
    log("INFO", "dastur boshlandi");
    log("WARN", &String::from("xotira kam"));
    log("ERROR", &format!("xato kodi: {}", 404));
    // [INFO] dastur boshlandi
    // [WARN] xotira kam
    // [ERROR] xato kodi: 404

    // Rc<T: ?Sized>
    rc_dst_misol();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                            SIZED TRAIT                                                                                        |
// #================================================================================================================================================#
// # |   1 | T (default: T: Sized)    | Har bir generic avtomatik Sized                      | Каждый generic автоматически Sized                   |
// # |   2 | size_of::<T>()           | Tur hajmini olish (Sized talab qiladi)               | Получение размера типа (требует Sized)               |
// # |   3 | size_of_val(&val)        | Qiymat hajmini olish (?Sized bilan ishlaydi)         | Получение размера значения (работает с ?Sized)       |
// #================================================================================================================================================#
// # |                                           ?SIZED TRAIT                                                                                        |
// #================================================================================================================================================#
// # |   4 | T: ?Sized                | Sized bo'lishi ham, bo'lmasligi ham mumkin           | Может быть Sized или нет                             |
// # |   5 | fn f<T: ?Sized>(x: &T)   | ?Sized faqat reference orqali ishlatiladi            | ?Sized используется только через ссылку              |
// # |   6 | struct S<T: ?Sized>      | ?Sized struct — T faqat Box/& orqali                 | ?Sized struct — T только через Box/&                 |
// #================================================================================================================================================#
// # |                                      DST (DYNAMICALLY SIZED TYPES)                                                                            |
// #================================================================================================================================================#
// # |   7 | str                      | Uzunligi runtime da ma'lum bo'lgan string            | Строка с длиной, известной во время выполнения       |
// # |   8 | [T]                      | Uzunligi runtime da ma'lum bo'lgan slice             | Слайс с длиной, известной во время выполнения        |
// # |   9 | dyn Trait                | Runtime da aniqlanadigan trait object                | Объект трейта, определяемый во время выполнения      |
// # |  10 | &str, &[T], &dyn Trait   | DST faqat reference orqali ishlatiladi               | DST используется только через ссылку                 |
// #================================================================================================================================================#
// # |                                      BOX<T> DST BILAN                                                                                         |
// #================================================================================================================================================#
// # |  11 | Box<str>                 | Heap da str saqlash                                  | Хранение str в куче                                  |
// # |  12 | Box<[T]>                 | Heap da slice saqlash                                | Хранение slice в куче                                |
// # |  13 | Box<dyn Trait>           | Heap da trait object saqlash                         | Хранение объекта трейта в куче                       |
// # |  14 | Rc<str>, Arc<[T]>        | Shared ownership DST uchun                           | Совместное владение для DST                          |
// #================================================================================================================================================#
// # |                                         QACHON ISHLATISH                                                                                      |
// #================================================================================================================================================#
// # |  15 | T: Sized (default)       | Odatiy generic, stack da ishlash                     | Обычный generic, работа на стеке                     |
// # |  16 | T: ?Sized                | str, [T], dyn Trait qabul qilish kerak bo'lsa        | Когда нужно принять str, [T], dyn Trait              |
// # |  17 | Box<T: ?Sized>           | DST ni heap da saqlash kerak bo'lsa                  | Когда нужно хранить DST в куче                       |
// #================================================================================================================================================#