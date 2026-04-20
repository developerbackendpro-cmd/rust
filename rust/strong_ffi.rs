// #================================================================================================================================================#
// #                                                                FFI CHUQUR                                                                      #
// #                        FFI — C KUTUBXONALAR BILAN CHUQUR ISHLASH. LINKING, CBINDGEN, BINDGEN, CALLBACK, STRUCT LAYOUT.                         #
// #                        FFI — ГЛУБОКАЯ РАБОТА С БИБЛИОТЕКАМИ C. LINKING, CBINDGEN, BINDGEN, CALLBACK, LAYOUT СТРУКТУР.                          #
// #================================================================================================================================================#

#![allow(dead_code, unused, improper_ctypes_definitions)]

use std::ffi::{CStr, CString, c_void};
use std::os::raw::{c_char, c_int, c_uint, c_double, c_float, c_long, c_ulong};
use std::ptr;
use std::fmt;
use std::mem;

// FFI Chuqur nima:
// Что такое FFI Chuqur:
//
//   std::ffi mavzusidan farqi:
//   Отличие от темы std::ffi:
//   - Murakkab C struct lar (pointer ichida)
//   - Сложные C структуры (указатели внутри)
//   - Callback funksiyalar (C dan Rust ga)
//   - Callback функции (из C в Rust)
//   - Linking strategiyalari
//   - Стратегии линковки
//   - cbindgen — Rust → C header
//   - cbindgen — Rust → C заголовок
//   - bindgen — C header → Rust bindings
//   - bindgen — C заголовок → Rust привязки
//   - Error handling C da
//   - Обработка ошибок в C
//   - Memory ownership C bilan
//   - Владение памятью с C

// C struct — repr(C) kerak!
// C структура — нужен repr(C)!
// repr(C) bo'lmasa — padding farqli bo'lishi mumkin
// Без repr(C) — отступы могут отличаться

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CVektor3 {
    x: c_double,
    y: c_double,
    z: c_double,
}

#[repr(C)]
#[derive(Debug, Clone)]
struct CMatrix3x3 {
    m: [[c_double; 3]; 3],
}

// Nested struct
#[repr(C)]
#[derive(Debug)]
struct CTransform {
    pozitsiya: CVektor3,
    yo_nalish: CVektor3,
    masshtab: c_double,
}

// Pointer ichida struct
#[repr(C)]
pub struct CLinkedNode {
    qiymat: c_int,
    keyingi: *mut CLinkedNode,
}

// Bit fields — C ga o'xshash
// Bit поля — аналог в C
#[repr(C)]
#[derive(Debug)]
struct CFlags {
    qiymat: c_uint, // bitlar sifatida ishlatiladi
}

impl CFlags {
    const YOQILGAN: c_uint = 1 << 0;
    const DEBUG: c_uint = 1 << 1;
    const TLS: c_uint = 1 << 2;
    const COMPRESSION: c_uint = 1 << 3;

    fn new() -> Self { CFlags { qiymat: 0 } }
    fn o_rnat(&mut self, flag: c_uint) -> &mut Self { self.qiymat |= flag; self }
    fn o_chir(&mut self, flag: c_uint) -> &mut Self { self.qiymat &= !flag; self }
    fn tekshir(&self, flag: c_uint) -> bool { self.qiymat & flag != 0 }
}

impl fmt::Display for CFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Flags({:#010b})", self.qiymat)
    }
}

fn c_struct_misoli() {

    println!("--- C Struct Layout ---");

    // O'lchamlar tekshiruvi
    println!("CVektor3:   {} bayt", mem::size_of::<CVektor3>());   // 24
    println!("CMatrix3x3: {} bayt", mem::size_of::<CMatrix3x3>()); // 72
    println!("CTransform: {} bayt", mem::size_of::<CTransform>()); // 56
    // CVektor3:   24 bayt
    // CMatrix3x3: 72 bayt
    // CTransform: 56 bayt

    // Vektor yaratish
    let v = CVektor3 { x: 1.0, y: 2.0, z: 3.0 };
    println!("{:?}", v);

    // Transform
    let t = CTransform {
        pozitsiya: CVektor3 { x: 0.0, y: 0.0, z: 0.0 },
        yo_nalish: CVektor3 { x: 0.0, y: 1.0, z: 0.0 },
        masshtab: 1.0,
    };
    println!("{:?}", t);

    // CFlags — bit manipulation
    let mut flags = CFlags::new();
    flags.o_rnat(CFlags::YOQILGAN)
        .o_rnat(CFlags::TLS)
        .o_rnat(CFlags::COMPRESSION);

    println!("{}", flags);
    println!("YOQILGAN: {}", flags.tekshir(CFlags::YOQILGAN)); // true
    println!("DEBUG:    {}", flags.tekshir(CFlags::DEBUG));    // false
    println!("TLS:      {}", flags.tekshir(CFlags::TLS));      // true
    // Flags(0b00001101)
    // YOQILGAN: true
    // DEBUG:    false
    // TLS:      true

    // C Linked List
    println!("\n--- C LinkedList ---");
    let mut c_node3 = CLinkedNode { qiymat: 30, keyingi: ptr::null_mut() };
    let mut c_node2 = CLinkedNode { qiymat: 20, keyingi: &mut c_node3 };
    let mut c_node1 = CLinkedNode { qiymat: 10, keyingi: &mut c_node2 };

    // Iteratsiya
    let mut joriy: *const CLinkedNode = &c_node1;
    while !joriy.is_null() {
        unsafe {
            print!("{} ", (*joriy).qiymat);
            joriy = (*joriy).keyingi;
        }
    }
    println!();
    // 10 20 30
}

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *const c_char;
    fn sprintf(buf: *mut c_char, format: *const c_char, ...) -> c_int;
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(count: usize, size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn qsort(
        base: *mut c_void,
        n: usize,
        size: usize,
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
    );
    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        n: usize,
        size: usize,
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int,
    ) -> *mut c_void;
    fn atoi(s: *const c_char) -> c_int;
    fn atof(s: *const c_char) -> c_double;
}

fn c_stdlib_misoli() {

    println!("\n--- C Stdlib ---");

    // strlen
    let s = CString::new("Salom Rust!").unwrap();
    let uzunlik = unsafe { strlen(s.as_ptr()) };
    println!("strlen: {}", uzunlik); // 11
    // strlen: 11

    // strcmp
    let s1 = CString::new("salom").unwrap();
    let s2 = CString::new("salom").unwrap();
    let s3 = CString::new("dunyo").unwrap();
    println!("strcmp(s,s)={}  strcmp(s,d)={}",
             unsafe { strcmp(s1.as_ptr(), s2.as_ptr()) },  // 0
             unsafe { strcmp(s1.as_ptr(), s3.as_ptr()) }); // != 0
    // strcmp(s,s)=0  strcmp(s,d)=15

    // malloc va free — Rust Box bilan
    unsafe {
        let ptr: *mut c_int = malloc(5 * mem::size_of::<c_int>()) as *mut c_int;
        if !ptr.is_null() {
            for i in 0..5 { *ptr.add(i) = (i as c_int + 1) * 10; }
            for i in 0..5 { print!("{} ", *ptr.add(i)); }
            println!();
            free(ptr as *mut c_void);
        }
    }
    // 10 20 30 40 50

    // calloc — nol bilan to'ldirilgan
    unsafe {
        let ptr: *mut c_int = calloc(3, mem::size_of::<c_int>()) as *mut c_int;
        if !ptr.is_null() {
            println!("calloc: {} {} {}", *ptr, *ptr.add(1), *ptr.add(2));
            free(ptr as *mut c_void);
        }
    }
    // calloc: 0 0 0

    // qsort — C tartiblash
    unsafe extern "C" fn solishtirish(a: *const c_void, b: *const c_void) -> c_int {
        let a = unsafe { *(a as *const c_int) };
        let b = unsafe { *(b as *const c_void as *const c_int) };
        a - b
    }

    let mut arr: Vec<c_int> = vec![64, 34, 25, 12, 22, 11, 90];
    unsafe {
        qsort(
            arr.as_mut_ptr() as *mut c_void,
            arr.len(),
            mem::size_of::<c_int>(),
            solishtirish,
        );
    }
    println!("qsort: {:?}", arr);
    // qsort: [11, 12, 22, 25, 34, 64, 90]

    // atoi va atof
    let n_str = CString::new("42abc").unwrap();
    let f_str = CString::new("3.14xyz").unwrap();
    unsafe {
        println!("atoi(42abc): {}", atoi(n_str.as_ptr())); // 42
        println!("atof(3.14xyz): {}", atof(f_str.as_ptr())); // 3.14
    }
    // atoi(42abc): 42
    // atof(3.14xyz): 3.14
}

// C callback turlar
// Типы C callback
type CCallback = unsafe extern "C" fn(c_int) -> c_int;
type CEventHandler = unsafe extern "C" fn(*const c_char, c_int, *mut c_void);
type CComparator = unsafe extern "C" fn(*const c_void, *const c_void) -> c_int;

// Rust funksiyalarini C callback sifatida
// Rust функции как C callback
unsafe extern "C" fn kvadrat_callback(x: c_int) -> c_int { x * x }
unsafe extern "C" fn ikki_baravar_callback(x: c_int) -> c_int { x * 2 }
unsafe extern "C" fn absolut_callback(x: c_int) -> c_int { x.abs() }

// Callback qabul qiluvchi "C kutubxona" simulyatsiyasi
// Симуляция "C библиотеки" принимающей callback
unsafe fn c_massivga_tatbiq(
    arr: *mut c_int,
    n: usize,
    callback: CCallback,
) {
    for i in 0..n {
        unsafe {
            let eski = *arr.add(i);
            *arr.add(i) = callback(eski);
        }
    }
}

unsafe fn c_voqea_yuborish(
    voqea_nomi: *const c_char,
    kod: c_int,
    userdata: *mut c_void,
    handler: CEventHandler,
) {
    unsafe { handler(voqea_nomi, kod, userdata); }
}

// Event handler — userdata orqali Rust state
// Event handler — состояние Rust через userdata
unsafe extern "C" fn voqea_handler(
    nomi: *const c_char,
    kod: c_int,
    userdata: *mut c_void,
) {
    let log = unsafe { &mut *(userdata as *mut Vec<String>) };
    let nomi_str = unsafe { CStr::from_ptr(nomi).to_str().unwrap_or("?") };
    log.push(format!("Voqea: {} (kod={})", nomi_str, kod));
}

fn callback_misoli() {

    println!("\n--- Callback Pattern ---");

    let mut arr: Vec<c_int> = vec![1, 2, 3, 4, 5];

    // kvadrat_callback
    unsafe {
        c_massivga_tatbiq(arr.as_mut_ptr(), arr.len(), kvadrat_callback);
    }
    println!("Kvadrat: {:?}", arr); // [1, 4, 9, 16, 25]
    // Kvadrat: [1, 4, 9, 16, 25]

    // ikki_baravar_callback
    unsafe {
        c_massivga_tatbiq(arr.as_mut_ptr(), arr.len(), ikki_baravar_callback);
    }
    println!("Ikki baravar: {:?}", arr); // [2, 8, 18, 32, 50]
    // Ikki baravar: [2, 8, 18, 32, 50]

    // Event handler — userdata bilan
    let mut log: Vec<String> = Vec::new();
    let userdata: *mut c_void = &mut log as *mut Vec<String> as *mut c_void;

    let v1 = CString::new("click").unwrap();
    let v2 = CString::new("keypress").unwrap();
    let v3 = CString::new("resize").unwrap();

    unsafe {
        c_voqea_yuborish(v1.as_ptr(), 1, userdata, voqea_handler);
        c_voqea_yuborish(v2.as_ptr(), 65, userdata, voqea_handler);
        c_voqea_yuborish(v3.as_ptr(), 0, userdata, voqea_handler);
    }

    println!("Log:");
    for entry in &log { println!("  {}", entry); }
    // Log:
    //   Voqea: click (kod=1)
    //   Voqea: keypress (kod=65)
    //   Voqea: resize (kod=0)
}

// cbindgen — Rust → C header generatsiya
// cbindgen — генерация C заголовка из Rust
//
// Cargo.toml:
// [build-dependencies]
// cbindgen = "0.27"
//
// build.rs:
// fn main() {
//     cbindgen::Builder::new()
//         .with_crate(".")
//         .with_language(cbindgen::Language::C)
//         .generate()
//         .unwrap()
//         .write_to_file("include/mylib.h");
// }
//
// Generatsiya qilingan C header ko'rinishi:
// Внешний вид сгенерированного заголовка C:
// typedef struct { double x; double y; double z; } CVektor3;
// double rust_vektor_uzunlik(CVektor3 v);
// CVektor3 rust_vektor_normallashtir(CVektor3 v);

// Eksport qilinadigan Rust funksiyalar
// Экспортируемые Rust функции
#[unsafe(no_mangle)]
pub extern "C" fn rust_vektor_uzunlik(v: CVektor3) -> c_double {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_vektor_qo_shish(a: CVektor3, b: CVektor3) -> CVektor3 {
    CVektor3 { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_vektor_ko_paytirish(v: CVektor3, skalyar: c_double) -> CVektor3 {
    CVektor3 { x: v.x * skalyar, y: v.y * skalyar, z: v.z * skalyar }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_vektor_normallashtir(v: CVektor3) -> CVektor3 {
    let uzunlik = rust_vektor_uzunlik(v.clone());
    if uzunlik < f64::EPSILON {
        CVektor3 { x: 0.0, y: 0.0, z: 0.0 }
    } else {
        CVektor3 { x: v.x / uzunlik, y: v.y / uzunlik, z: v.z / uzunlik }
    }
}

// String qaytarish — C da xotira boshqaruvi
// Возврат строки — управление памятью в C
#[unsafe(no_mangle)]
pub extern "C" fn rust_versiya() -> *const c_char {
    // Static satr — C da free qilinmaydi
    // Static строка — не освобождается в C
    b"1.0.0\0".as_ptr() as *const c_char
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_format_vektor(v: CVektor3) -> *mut c_char {
    let s = format!("({:.2}, {:.2}, {:.2})", v.x, v.y, v.z);
    match CString::new(s) {
        Ok(cs) => cs.into_raw(), // C xotirani boshqaradi
        Err(_) => ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_string_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { drop(CString::from_raw(ptr)); }
    }
}

fn cbindgen_misoli() {

    println!("\n--- cbindgen: Rust → C Eksport ---");

    let v1 = CVektor3 { x: 3.0, y: 4.0, z: 0.0 };
    let v2 = CVektor3 { x: 1.0, y: 0.0, z: 0.0 };

    println!("Uzunlik(3,4,0): {}", rust_vektor_uzunlik(v1.clone())); // 5.0
    let qo_sh = rust_vektor_qo_shish(v1.clone(), v2.clone());
    println!("Qo'shish: {:?}", qo_sh);

    let norm = rust_vektor_normallashtir(v1.clone());
    println!("Normallashtirilgan: ({:.4}, {:.4}, {:.4})", norm.x, norm.y, norm.z);

    let versiya = unsafe { CStr::from_ptr(rust_versiya()).to_str().unwrap() };
    println!("Versiya: {}", versiya);

    let fmt_ptr = rust_format_vektor(v1);
    if !fmt_ptr.is_null() {
        let s = unsafe { CStr::from_ptr(fmt_ptr).to_str().unwrap().to_string() };
        rust_string_free(fmt_ptr);
        println!("Format: {}", s);
    }
    // Uzunlik(3,4,0): 5
    // Qo'shish: CVektor3 { x: 4.0, y: 4.0, z: 0.0 }
    // Normallashtirilgan: (0.6000, 0.8000, 0.0000)
    // Versiya: 1.0.0
    // Format: (3.00, 4.00, 0.00)
}

// bindgen — C header → Rust bindings
// bindgen — C заголовок → Rust привязки
//
// Cargo.toml:
// [build-dependencies]
// bindgen = "0.69"
//
// build.rs:
// fn main() {
//     let bindings = bindgen::Builder::default()
//         .header("include/mylib.h")
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
//         .generate()
//         .unwrap();
//     bindings.write_to_file("src/bindings.rs").unwrap();
// }
//
// Generatsiya natijasi (src/bindings.rs):
// Результат генерации:
// #[repr(C)]
// pub struct sqlite3 { ... }
// extern "C" {
//     pub fn sqlite3_open(filename: *const c_char, db: *mut *mut sqlite3) -> c_int;
//     pub fn sqlite3_close(db: *mut sqlite3) -> c_int;
//     pub fn sqlite3_exec(...) -> c_int;
// }

fn bindgen_tushuntirish() {

    println!("\n--- bindgen tushuntirish ---");
    println!("bindgen — C header faylidan Rust binding yaratadi");
    println!("Qadamlar:");
    println!("  1. Cargo.toml: bindgen qo'shish");
    println!("  2. build.rs: bindgen::Builder konfiguratsiya");
    println!("  3. cargo build — src/bindings.rs yaratiladi");
    println!("  4. include!(concat!(env!(\"OUT_DIR\"), \"/bindings.rs\"));");
    println!();
    println!("Mashhur FFI kutubxonalar:");
    println!("  sqlite — rusqlite crate (FFI wrapper)");
    println!("  openssl — openssl crate");
    println!("  zlib — flate2 crate");
    println!("  libgit2 — git2 crate");
    println!("  lua — mlua crate");
    println!("  python — pyo3 crate");
}

// Qoida 1: Kim ajratsa — o'sha bo'shatadi
// Правило 1: Кто выделяет — тот и освобождает
//
// Rust ajratsa → Rust bo'shatadi
// Rust выделяет → Rust освобождает
// C ajratsa → C bo'shatadi
// C выделяет → C освобождает

// Rust → C: Rust xotirani ajratadi, C dan keyin qaytarib oladi
// Rust → C: Rust выделяет, забирает обратно после C
#[unsafe(no_mangle)]
pub extern "C" fn rust_buffer_yaratish(uzunlik: usize) -> *mut u8 {
    let mut v: Vec<u8> = vec![0u8; uzunlik];
    let ptr = v.as_mut_ptr();
    std::mem::forget(v); // Drop chaqirilmaydi
    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_buffer_yozish(ptr: *mut u8, offset: usize, val: u8) {
    if ptr.is_null() { return; }
    unsafe { *ptr.add(offset) = val; }
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_buffer_o_chirish(ptr: *mut u8, uzunlik: usize) {
    if ptr.is_null() { return; }
    unsafe {
        drop(Vec::from_raw_parts(ptr, uzunlik, uzunlik));
    }
}

fn memory_ownership_misoli() {

    println!("\n--- Memory Ownership ---");

    // Rust xotira boshqaradi
    let ptr = rust_buffer_yaratish(10);
    println!("Buffer yaratildi: {:p}", ptr);

    for i in 0..10 {
        rust_buffer_yozish(ptr, i, (i as u8 + 65)); // A, B, C, ...
    }

    // O'qish
    let slice = unsafe { std::slice::from_raw_parts(ptr, 10) };
    println!("Buffer: {:?}", std::str::from_utf8(slice).unwrap());

    // Bo'shatish
    rust_buffer_o_chirish(ptr, 10);
    println!("Buffer bo'shatildi");
    // Buffer yaratildi: 0x...
    // Buffer: "ABCDEFGHIJ"
    // Buffer bo'shatildi

    // Box::into_raw / Box::from_raw — ownership C ga
    println!("\n--- Box transfer ---");
    let data = Box::new(CVektor3 { x: 1.0, y: 2.0, z: 3.0 });
    let raw = Box::into_raw(data); // Rust ownership → C

    unsafe {
        println!("C dan o'qish: {:?}", *raw);
        (*raw).x = 10.0; // C o'zgartiradi
    }

    let back = unsafe { Box::from_raw(raw) }; // C → Rust ownership
    println!("Rust qaytib oldi: {:?}", back);
    // C dan o'qish: CVektor3 { x: 1.0, y: 2.0, z: 3.0 }
    // Rust qaytib oldi: CVektor3 { x: 10.0, y: 2.0, z: 3.0 }
}

fn main() {

    println!("=== C STRUCT LAYOUT ===");
    c_struct_misoli();

    println!("\n=== C STDLIB ===");
    c_stdlib_misoli();

    println!("\n=== CALLBACK PATTERN ===");
    callback_misoli();

    println!("\n=== CBINDGEN ===");
    cbindgen_misoli();

    println!("\n=== BINDGEN ===");
    bindgen_tushuntirish();

    println!("\n=== MEMORY OWNERSHIP ===");
    memory_ownership_misoli();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        C STRUCT                                                                                              |
// #================================================================================================================================================#
// # |   1 | #[repr(C)]                      | C mos keluvchi struct tartibi              | Совместимый с C порядок структуры                       |
// # |   2 | c_int, c_double, ...            | C turlari                                  | Типы C                                                  |
// # |   3 | *mut T, *const T                | C pointer lar                              | Указатели C                                             |
// # |   4 | CFlags — bit manipulation       | Bit bayroqlar                              | Битовые флаги                                           |
// #================================================================================================================================================#
// # |                                        CALLBACK                                                                                              |
// #================================================================================================================================================#
// # |   5 | unsafe extern "C" fn f(x) -> T  | C callback turi                            | Тип C callback                                          |
// # |   6 | userdata: *mut c_void           | Rust state ni C ga uzatish                 | Передача состояния Rust в C                             |
// # |   7 | &mut T as *mut c_void           | Rust referens → void* konversiya           | Конверсия Rust ссылки → void*                           |
// #================================================================================================================================================#
// # |                                        EKPORT VA IMPORT                                                                                      |
// #================================================================================================================================================#
// # |   8 | #[unsafe(no_mangle)]            | C dan chaqirish uchun nom saqlash          | Сохранение имени для вызова из C                        |
// # |   9 | pub extern "C" fn               | Rust → C eksport                           | Экспорт Rust → C                                        |
// # |  10 | cbindgen                        | Rust → C header avtomatik                  | Автоматически Rust → C заголовок                        |
// # |  11 | bindgen                         | C header → Rust binding avtomatik          | Автоматически C заголовок → Rust привязка               |
// #================================================================================================================================================#
// # |                                        MEMORY                                                                                                |
// #================================================================================================================================================#
// # |  12 | Box::into_raw(b)                | Rust → C ownership                         | Передача владения Rust → C                              |
// # |  13 | Box::from_raw(ptr)              | C → Rust ownership                         | Возврат владения C → Rust                               |
// # |  14 | mem::forget(val)                | Drop olmay pointer qaytarish               | Вернуть указатель без Drop                              |
// # |  15 | Vec::from_raw_parts(p,l,c)      | Raw pointer → Vec (ownership)              | Raw pointer → Vec (владение)                            |
// #================================================================================================================================================#