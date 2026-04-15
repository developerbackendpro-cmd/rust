// #================================================================================================================================================#
// #                                                                ASREF  |  ASMUT                                                                 #
// #                            ASREF — O'ZGARMAS REFERENCE OLISH. ASMUT — O'ZGARUVCHAN REFERENCE OLISH. GENERIC API UCHUN.                         #
// #                            ASREF — ПОЛУЧЕНИЕ НЕИЗМЕНЯЕМОЙ ССЫЛКИ. ASMUT — ИЗМЕНЯЕМОЙ ССЫЛКИ. ДЛЯ GENERIC API.                                  #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// AsRef<T> — &T reference olish imkoniyati
// AsRef<T> — возможность получить &T
// AsMut<T> — &mut T reference olish imkoniyati
// AsMut<T> — возможность получить &mut T

// Qachon ishlatiladi:
// Когда используется:
// fn f(x: impl AsRef<str>) — &str ham, String ham, &String ham qabul qiladi
// fn f(x: impl AsRef<str>) — принимает &str, String, и &String

// String → AsRef<str>   ✅
// String → AsRef<[u8]>  ✅
// &str   → AsRef<str>   ✅
// Vec<T> → AsRef<[T]>   ✅
// [T;N]  → AsRef<[T]>   ✅
// Path   → AsRef<Path>  ✅

fn built_in_asref_misollari() {
    // String va &str — ikkalasi AsRef<str>
    // String и &str — оба AsRef<str>
    let string_qiymati: String = String::from("salom dunyo");
    let str_literal: &str = "salom dunyo";

    let str_ref1: &str = string_qiymati.as_ref();
    let str_ref2: &str = str_literal.as_ref();
    println!("{}", str_ref1);
    println!("{}", str_ref2);
    // salom dunyo
    // salom dunyo

    // String — AsRef<[u8]>
    // String — AsRef<[u8]>
    let matn: String = String::from("ABC");
    let baytlar: &[u8] = matn.as_ref();
    println!("{:?}", baytlar);
    // [65, 66, 67]

    // Vec<T> — AsRef<[T]>
    // Vec<T> — AsRef<[T]>
    let vektor: Vec<i32> = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = vektor.as_ref();
    println!("{:?}", slice);
    // [1, 2, 3, 4, 5]

    // array — AsRef<[T]>
    // массив — AsRef<[T]>
    let array: [i32; 4] = [10, 20, 30, 40];
    let slice2: &[i32] = array.as_ref();
    println!("{:?}", slice2);
    // [10, 20, 30, 40]

    // Box<T> — AsRef<T>
    // Box<T> — AsRef<T>
    let boxed: Box<i32> = Box::new(42);
    let ref_val: &i32 = boxed.as_ref();
    println!("{}", ref_val);
    // 42
}

// impl AsRef<str> — &str ham, String ham qabul qiladi
// impl AsRef<str> — принимает и &str и String
fn uzunlikni_hisoblash(matn: impl AsRef<str>) -> usize {
    matn.as_ref().len()
}

// &str, String, &String hammasini qabul qiladi
// принимает &str, String, &String
fn katta_harfga_aylantir(matn: impl AsRef<str>) -> String {
    matn.as_ref().to_uppercase()
}

// AsRef<[i32]> — Vec, array, slice hammasini qabul qiladi
// AsRef<[i32]> — принимает Vec, array, slice
fn yig_indi_hisoblash(raqamlar: impl AsRef<[i32]>) -> i32 {
    raqamlar.as_ref().iter().sum()
}

// bir nechta AsRef bound
// несколько ограничений AsRef
fn chiqar_va_uzunlik<T: AsRef<str> + fmt::Debug>(qiymat: T) {
    println!("{:?} — uzunlik: {}", qiymat, qiymat.as_ref().len());
}

// custom struct uchun AsRef
// AsRef для пользовательской структуры
#[derive(Debug)]
struct Ism {
    qiymat: String,
}

impl Ism {
    fn new(ism: &str) -> Self {
        Ism { qiymat: ism.to_string() }
    }
}

impl AsRef<str> for Ism {
    fn as_ref(&self) -> &str {
        &self.qiymat
    }
}

impl AsRef<String> for Ism {
    fn as_ref(&self) -> &String {
        &self.qiymat
    }
}

// Newtype pattern — AsRef bilan
// Паттерн Newtype — с AsRef
#[derive(Debug)]
struct FilePath(String);

impl FilePath {
    fn new(path: &str) -> Self {
        FilePath(path.to_string())
    }
}

impl AsRef<str> for FilePath {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<std::path::Path> for FilePath {
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(&self.0)
    }
}

fn built_in_asmut_misollari() {
    // String — AsMut<str>
    // String — AsMut<str>
    let mut matn: String = String::from("salom");
    let str_mut: &mut str = matn.as_mut();
    println!("{}", str_mut);
    // salom

    // Vec<T> — AsMut<[T]>
    // Vec<T> — AsMut<[T]>
    let mut vektor: Vec<i32> = vec![1, 2, 3, 4, 5];
    let slice_mut: &mut [i32] = vektor.as_mut();
    slice_mut[0] = 99;
    println!("{:?}", vektor);
    // [99, 2, 3, 4, 5]

    // array — AsMut<[T]>
    // массив — AsMut<[T]>
    let mut array: [i32; 4] = [10, 20, 30, 40];
    let slice_mut2: &mut [i32] = array.as_mut();
    slice_mut2[0] = 100;
    println!("{:?}", array);
    // [100, 20, 30, 40]

    // Box<T> — AsMut<T>
    // Box<T> — AsMut<T>
    let mut boxed: Box<i32> = Box::new(42);
    let ref_mut: &mut i32 = boxed.as_mut();
    *ref_mut = 99;
    println!("{}", boxed);
    // 99
}

// impl AsMut<[T]> — Vec, array, slice o'zgartirish
// impl AsMut<[T]> — изменение Vec, array, slice
fn birinchisini_nollash(raqamlar: &mut impl AsMut<[i32]>) {
    let slice: &mut [i32] = raqamlar.as_mut();
    if !slice.is_empty() {
        slice[0] = 0;
    }
}

// AsMut<str> — string o'zgartirish
// AsMut<str> — изменение строки
fn bosh_harfni_katta_qil(matn: &mut impl AsMut<str>) {
    let s: &mut str = matn.as_mut();
    if let Some(c) = s.get_mut(0..1) {
        c.make_ascii_uppercase();
    }
}

// AsMut bilan tartiblash
// сортировка с AsMut
fn tartiblash<T: Ord>(raqamlar: &mut impl AsMut<[T]>) {
    raqamlar.as_mut().sort();
}

// AsMut bilan to'ldirish
// заполнение с AsMut
fn nollar_bilan_toldirish(raqamlar: &mut impl AsMut<[i32]>) {
    for x in raqamlar.as_mut().iter_mut() {
        *x = 0;
    }
}

// custom struct uchun AsMut
// AsMut для пользовательской структуры
#[derive(Debug)]
struct Buffer {
    ichki: Vec<u8>,
}

impl Buffer {
    fn new(hajm: usize) -> Self {
        Buffer { ichki: vec![0; hajm] }
    }
}

impl AsRef<[u8]> for Buffer {
    fn as_ref(&self) -> &[u8] {
        &self.ichki
    }
}

impl AsMut<[u8]> for Buffer {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.ichki
    }
}

// Buffer bilan ishlaydigan funksiyalar
// функции работающие с Buffer
fn buffer_toldirish(buf: &mut impl AsMut<[u8]>, qiymat: u8) {
    for byte in buf.as_mut().iter_mut() {
        *byte = qiymat;
    }
}

fn buffer_o_qish(buf: &impl AsRef<[u8]>) -> Vec<u8> {
    buf.as_ref().to_vec()
}

#[derive(Debug)]
struct MatnBuffer {
    ichki: String,
}

impl MatnBuffer {
    fn new(matn: &str) -> Self {
        MatnBuffer { ichki: matn.to_string() }
    }
}

impl AsRef<str> for MatnBuffer {
    fn as_ref(&self) -> &str {
        &self.ichki
    }
}

impl AsMut<String> for MatnBuffer {
    fn as_mut(&mut self) -> &mut String {
        &mut self.ichki
    }
}

impl fmt::Display for MatnBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ichki)
    }
}

// fayl nomi qabul qilish — &str, String, Path hammasini
// принятие имени файла — &str, String, Path
fn fayl_nomi_chiqar(yo_l: impl AsRef<std::path::Path>) {
    let path: &std::path::Path = yo_l.as_ref();
    if let Some(nomi) = path.file_name() {
        println!("Fayl nomi: {:?}", nomi);
    }
}

// HashMap ga xavfsiz kirishfn
// безопасный доступ к HashMap
fn qiymat_ol<'a>(
    xarita: &'a std::collections::HashMap<String, String>,
    kalit: impl AsRef<str>,
) -> Option<&'a String> {
    xarita.get(kalit.as_ref())
}

// generic log funksiyasi
// обобщённая функция логирования
fn log_xabar(daraja: impl AsRef<str>, xabar: impl AsRef<str>) {
    println!("[{}] {}", daraja.as_ref(), xabar.as_ref());
}

// Deref — avtomatik, kompilyator qo'shadi
// Deref — автоматически, компилятор добавляет
// AsRef — qo'lda chaqiriladi, generic API uchun
// AsRef — вызывается вручную, для generic API
//
// String implements:
//   Deref<Target = str>  → *string → &str (avtomatik)
//   AsRef<str>           → string.as_ref() (qo'lda)
//   AsRef<[u8]>          → string.as_ref() (qo'lda)
//   AsRef<String>        → string.as_ref() (qo'lda)

fn deref_vs_asref_farqi() {
    let string: String = String::from("salom");

    // Deref — avtomatik (*) yoki coercion
    // Deref — автоматически (*) или приведение типов
    let deref_str: &str = &*string;
    let coercion: &str = &string;

    // AsRef — qo'lda
    // AsRef — вручную
    let as_ref_str: &str = string.as_ref();
    let as_ref_bytes: &[u8] = string.as_ref();

    println!("{}", deref_str);
    println!("{}", coercion);
    println!("{}", as_ref_str);
    println!("{:?}", as_ref_bytes);
    // salom
    // salom
    // salom
    // [115, 97, 108, 111, 109]
}

fn main() {

    built_in_asref_misollari();

    // uzunlik — &str, String, &String hammasini qabul qiladi
    // uzunlik — принимает &str, String, &String
    let uzunlik1: usize = uzunlikni_hisoblash("salom");
    let uzunlik2: usize = uzunlikni_hisoblash(String::from("salom dunyo"));
    let string_ref: String = String::from("rust");
    let uzunlik3: usize = uzunlikni_hisoblash(&string_ref);
    println!("{}", uzunlik1);
    println!("{}", uzunlik2);
    println!("{}", uzunlik3);
    // 5
    // 11
    // 4

    // katta_harfga_aylantir — turli turlar
    // katta_harfga_aylantir — разные типы
    let k1: String = katta_harfga_aylantir("salom");
    let k2: String = katta_harfga_aylantir(String::from("dunyo"));
    println!("{}", k1);
    println!("{}", k2);
    // SALOM
    // DUNYO

    // yig_indi — Vec, array, slice
    // yig_indi — Vec, array, slice
    let vec_sum: i32 = yig_indi_hisoblash(vec![1, 2, 3, 4, 5]);
    let arr_sum: i32 = yig_indi_hisoblash([10, 20, 30]);
    let slice_sum: i32 = yig_indi_hisoblash([1, 2, 3, 4].as_ref());
    println!("{}", vec_sum);
    println!("{}", arr_sum);
    println!("{}", slice_sum);
    // 15
    // 60
    // 10

    // Ism — AsRef<str>
    // Ism — AsRef<str>
    let ism = Ism::new("Dilshod");
    let ism_str: &str = ism.as_ref();
    println!("{}", ism_str);
    println!("{}", uzunlikni_hisoblash(&ism));
    println!("{}", katta_harfga_aylantir(&ism));
    // Dilshod
    // 7
    // DILSHOD

    // FilePath — AsRef<str> + AsRef<Path>
    // FilePath — AsRef<str> + AsRef<Path>
    let yo_l = FilePath::new("/home/user/fayl.txt");
    let str_yo_l: &str = yo_l.as_ref();
    println!("{}", str_yo_l);
    fayl_nomi_chiqar(&yo_l);
    // /home/user/fayl.txt
    // Fayl nomi: "fayl.txt"

    built_in_asmut_misollari();

    // birinchisini_nollash — Vec va array
    // birinchisini_nollash — Vec и array
    let mut vektor: Vec<i32> = vec![1, 2, 3, 4, 5];
    birinchisini_nollash(&mut vektor);
    println!("{:?}", vektor);
    // [0, 2, 3, 4, 5]

    let mut array: [i32; 4] = [10, 20, 30, 40];
    birinchisini_nollash(&mut array);
    println!("{:?}", array);
    // [0, 20, 30, 40]

    // tartiblash — Vec va array
    // tartiblash — Vec и array
    let mut tartibsiz_vec: Vec<i32> = vec![5, 2, 8, 1, 9, 3];
    tartiblash(&mut tartibsiz_vec);
    println!("{:?}", tartibsiz_vec);
    // [1, 2, 3, 5, 8, 9]

    let mut tartibsiz_arr: [i32; 5] = [5, 3, 1, 4, 2];
    tartiblash(&mut tartibsiz_arr);
    println!("{:?}", tartibsiz_arr);
    // [1, 2, 3, 4, 5]

    // nollar_bilan_toldirish
    // заполнение нулями
    let mut to_ldiriladigan: Vec<i32> = vec![1, 2, 3, 4, 5];
    nollar_bilan_toldirish(&mut to_ldiriladigan);
    println!("{:?}", to_ldiriladigan);
    // [0, 0, 0, 0, 0]

    // bosh_harfni_katta_qil
    // сделать первую букву заглавной
    let mut matn1: String = String::from("salom");
    bosh_harfni_katta_qil(&mut matn1);
    println!("{}", matn1);
    // Salom

    // Buffer — AsRef<[u8]> + AsMut<[u8]>
    // Buffer — AsRef<[u8]> + AsMut<[u8]>
    let mut buf = Buffer::new(5);
    println!("Bo'sh buffer: {:?}", buffer_o_qish(&buf));
    buffer_toldirish(&mut buf, 0xFF);
    println!("To'ldirilgan buffer: {:?}", buffer_o_qish(&buf));
    // Bo'sh buffer: [0, 0, 0, 0, 0]
    // To'ldirilgan buffer: [255, 255, 255, 255, 255]

    // MatnBuffer — AsRef<str> + AsMut<String>
    // MatnBuffer — AsRef<str> + AsMut<String>
    let mut matn_buf = MatnBuffer::new("salom dunyo");
    println!("Asl: {}", matn_buf);

    // o'qish — AsRef<str>
    // чтение — AsRef<str>
    let uzunlik: usize = uzunlikni_hisoblash(&matn_buf);
    println!("Uzunlik: {}", uzunlik);
    // Uzunlik: 11

    // o'zgartirish — AsMut<String>
    // изменение — AsMut<String>
    let string_ref: &mut String = matn_buf.as_mut();
    string_ref.push_str("!!!");
    println!("Yangi: {}", matn_buf);
    // Yangi: salom dunyo!!!

    // fayl nomi — &str, String, Path hammasini qabul qiladi
    // имя файла — принимает &str, String, Path
    fayl_nomi_chiqar("/home/user/hujjat.txt");
    fayl_nomi_chiqar(String::from("/tmp/fayl.rs"));
    // Fayl nomi: "hujjat.txt"
    // Fayl nomi: "fayl.rs"

    // HashMap ga kirishfn
    // доступ к HashMap
    let mut xarita: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    xarita.insert(String::from("ism"), String::from("Dilshod"));
    xarita.insert(String::from("shahar"), String::from("Toshkent"));

    // &str bilan String kalitlarida qidirish
    // поиск по ключам &str в HashMap со String ключами
    let ism1 = qiymat_ol(&xarita, "ism");
    let ism2 = qiymat_ol(&xarita, String::from("shahar"));
    let yo_q = qiymat_ol(&xarita, "yoq");
    println!("{:?}", ism1);
    println!("{:?}", ism2);
    println!("{:?}", yo_q);
    // Some("Dilshod")
    // Some("Toshkent")
    // None

    // log_xabar — &str va String ikkalasi
    // log_xabar — и &str и String
    log_xabar("INFO", "dastur boshlandi");
    log_xabar(String::from("WARN"), "xotira kam");
    log_xabar("ERROR", String::from("ulanish rad etildi"));
    // [INFO] dastur boshlandi
    // [WARN] xotira kam
    // [ERROR] ulanish rad etildi

    deref_vs_asref_farqi();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                          ASREF TRAIT                                                                                         |
// #================================================================================================================================================#
// # |   1 | impl AsRef<T> for S      | S dan &T olish imkoniyati                            | Возможность получить &T из S                         |
// # |   2 | .as_ref()                | &T reference olish                                   | Получение ссылки &T                                  |
// # |   3 | fn f(x: impl AsRef<str>) | &str, String, &String hammasini qabul qilish         | Принимает &str, String, &String                      |
// # |   4 | fn f(x: impl AsRef<[T]>) | Vec, array, slice hammasini qabul qilish             | Принимает Vec, array, slice                          |
// #================================================================================================================================================#
// # |                                          ASMUT TRAIT                                                                                         |
// #================================================================================================================================================#
// # |   5 | impl AsMut<T> for S      | S dan &mut T olish imkoniyati                        | Возможность получить &mut T из S                     |
// # |   6 | .as_mut()                | &mut T reference olish                               | Получение изменяемой ссылки &mut T                   |
// # |   7 | fn f(x: &mut impl AsMut) | Vec va array ikkalasini o'zgartirish                 | Изменение Vec и array                                |
// #================================================================================================================================================#
// # |                                    BUILT-IN IMPLEMENT                                                                                        |
// #================================================================================================================================================#
// # |   8 | String → AsRef<str>      | String dan &str olish                                | Получение &str из String                             |
// # |   9 | String → AsRef<[u8]>     | String dan baytlar olish                             | Получение байтов из String                           |
// # |  10 | Vec<T> → AsRef<[T]>      | Vec dan slice olish                                  | Получение slice из Vec                               |
// # |  11 | [T;N] → AsRef<[T]>       | Array dan slice olish                                | Получение slice из массива                           |
// # |  12 | Box<T> → AsRef<T>        | Box dan reference olish                              | Получение ссылки из Box                              |
// # |  13 | Vec<T> → AsMut<[T]>      | Vec dan mut slice olish                              | Получение mut slice из Vec                           |
// #================================================================================================================================================#
// # |                                    DEREF VS ASREF                                                                                            |
// #================================================================================================================================================#
// # |  14 | Deref                    | Avtomatik, * operatori, coercion                     | Автоматически, оператор *, приведение типов          |
// # |  15 | AsRef                    | Qo'lda chaqiriladi, generic API uchun                | Вызывается вручную, для generic API                  |
// # |  16 | AsRef ko'p tur           | Bir struct ko'p AsRef implement qilishi mumkin       | Одна структура может реализовать несколько AsRef     |
// #================================================================================================================================================#
// # |                                    REAL HAYOT QOLLASH                                                                                        |
// #================================================================================================================================================#
// # |  17 | fayl yo'li funksiyalar   | &str, String, Path hammasini qabul qilish            | Принятие &str, String, Path                          |
// # |  18 | HashMap::get             | &str bilan String kalitlarida qidirish               | Поиск &str в HashMap со String ключами               |
// # |  19 | log funksiyalar          | Turli matn turlarini qabul qilish                    | Принятие различных текстовых типов                   |
// # |  20 | Buffer pattern           | AsRef + AsMut — o'qish va yozish                     | AsRef + AsMut — чтение и запись                      |
// #================================================================================================================================================#