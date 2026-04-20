// #================================================================================================================================================#
// #                                                            STD::FFI                                                                            #
// #                        STD::FFI — C BILAN O'ZARO ISH. CSTR, CSTRING, OSSTR, OSSTRING. EXTERN "C". UNSAFE FFI.                                  #
// #                        STD::FFI — ВЗАИМОДЕЙСТВИЕ С C. CSTR, CSTRING, OSSTR, OSSTRING. EXTERN "C". UNSAFE FFI.                                  #
// #================================================================================================================================================#

#![allow(dead_code, unused, improper_ctypes_definitions)]

use std::ffi::{CStr, CString, OsStr, OsString, c_void};
use std::os::raw::{c_char, c_int, c_uint, c_long, c_double, c_float};
use std::ptr;
use std::fmt;

// std::ffi nima:
// Что такое std::ffi:
//
//   FFI = Foreign Function Interface
//   FFI = Интерфейс внешних функций
//
//   CStr    — C satr (&str ga o'xshash, null-terminated, borrowed)
//   CStr    — C строка (как &str, null-terminated, borrowed)
//   CString — Owned CStr (String ga o'xshash, null-terminated)
//   CString — Owned CStr (как String, null-terminated)
//   OsStr   — OS satr (&str ga o'xshash, platform-native)
//   OsStr   — OS строка (как &str, platform-native)
//   OsString— Owned OsStr (String ga o'xshash)
//   OsString— Owned OsStr (как String)
//
//   c_char, c_int, c_long, ... — C turlari
//   c_char, c_int, c_long, ... — типы C
//
//   extern "C" — C ABI funksiya deklaratsiyasi
//   extern "C" — объявление функции с C ABI

fn cstring_cstr_misollari() {

    // CString::new — Rust String → C satr
    // CString::new — Rust String → C строка
    let cs: CString = CString::new("salom dunyo").unwrap();
    println!("{:?}", cs); // "salom dunyo"
    println!("Uzunlik: {}", cs.as_bytes().len()); // 11 (null yo'q)
    println!("Null bilan: {}", cs.as_bytes_with_nul().len()); // 12 (null bor)
    // "salom dunyo"
    // Uzunlik: 11
    // Null bilan: 12

    // Null bayt xatosi
    // Ошибка нулевого байта
    let xatoli = CString::new("sal\0om"); // ichida null!
    println!("{:?}", xatoli); // Err(NulError)
    // Err(NulError(3, ...))

    // CString → *const c_char (C ga berish)
    // CString → *const c_char (передача в C)
    let cs2 = CString::new("Rust dan C ga").unwrap();
    let ptr: *const c_char = cs2.as_ptr();
    println!("Pointer: {:p}", ptr);
    // Pointer: 0x...

    // CString::into_raw — ownership C ga o'tkazish
    // CString::into_raw — передача владения в C
    let cs3 = CString::new("ownership C ga").unwrap();
    let raw: *mut c_char = cs3.into_raw();
    // C ishlatadi...
    let cs4 = unsafe { CString::from_raw(raw) }; // qaytarib olish
    println!("{:?}", cs4); // "ownership C ga"
    // "ownership C ga"

    // CStr — C dan kelgan satr (&CStr)
    // CStr — строка из C (&CStr)
    let satr = b"C dan kelgan satr\0";
    let cstr: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(satr) };
    println!("{:?}", cstr); // "C dan kelgan satr"
    // "C dan kelgan satr"

    // CStr → &str
    // CStr → &str
    match cstr.to_str() {
        Ok(s)  => println!("&str: {}", s),
        Err(e) => println!("UTF-8 xato: {}", e),
    }
    // &str: C dan kelgan satr

    // CStr::from_bytes_with_nul
    // CStr::from_bytes_with_nul
    match CStr::from_bytes_with_nul(b"test\0") {
        Ok(s)  => println!("CStr: {:?}", s),
        Err(e) => println!("Xato: {}", e),
    }
    // CStr: "test"

    // CStr::from_bytes_until_nul — birinchi null gacha
    // CStr::from_bytes_until_nul — до первого null
    let buf = b"salom\0qolgan";
    if let Ok(s) = CStr::from_bytes_until_nul(buf) {
        println!("Until nul: {:?}", s); // "salom"
    }
    // Until nul: "salom"

    // to_string_lossy() — UTF-8 bo'lmasa ? bilan
    // to_string_lossy() — с ? если не UTF-8
    let cstr2: &CStr = CStr::from_bytes_with_nul(b"xabar\0").unwrap();
    let s: std::borrow::Cow<str> = cstr2.to_string_lossy();
    println!("{}", s); // xabar
    // xabar
}

fn osstr_osstring_misollari() {

    // OsStr — platform-native satr (Path da ishlatiladi)
    // OsStr — платформно-нативная строка (используется в Path)
    let os: &OsStr = OsStr::new("fayl nomi.txt");
    println!("{:?}", os); // "fayl nomi.txt"
    println!("UTF-8: {:?}", os.to_str()); // Some("fayl nomi.txt")
    // "fayl nomi.txt"
    // UTF-8: Some("fayl nomi.txt")

    // OsString — owned OsStr
    // OsString — owned OsStr
    let mut oss: OsString = OsString::new();
    oss.push("birinchi");
    oss.push(" ikkinchi");
    println!("{:?}", oss); // "birinchi ikkinchi"
    // "birinchi ikkinchi"

    // OsString → OsStr
    // OsString → OsStr
    let oss2: OsString = OsString::from("test string");
    let os2: &OsStr = &oss2;
    println!("{:?}", os2); // "test string"
    // "test string"

    // String → OsString → String
    // String → OsString → String
    let s1 = String::from("normal satr");
    let oss3: OsString = OsString::from(s1);
    let s2: Option<&str> = oss3.to_str();
    println!("{:?}", s2); // Some("normal satr")
    // Some("normal satr")

    // OsStr metodlari
    // Методы OsStr
    let os3: &OsStr = OsStr::new("fayl.rs");
    println!("is_empty: {}", os3.is_empty()); // false
    println!("len: {}", os3.len());           // 7 (baytlar)
    // is_empty: false
    // len: 7

    // Path bilan OsStr
    // OsStr с Path
    use std::path::Path;
    let path = Path::new("/home/user/fayl.toml");
    println!("extension OsStr: {:?}", path.extension()); // Some("toml")
    println!("file_name OsStr: {:?}", path.file_name()); // Some("fayl.toml")
    // extension OsStr: Some("toml")
    // file_name OsStr: Some("fayl.toml")

    // OsString capacity
    // Ёмкость OsString
    let mut oss4 = OsString::with_capacity(100);
    oss4.push("Salom");
    oss4.push(" Dunyo");
    println!("uzunlik: {}, sig'im: {}", oss4.len(), oss4.capacity());
    // uzunlik: 11, sig'im: 100
}

fn c_turlari_misollari() {

    // C turlari platformaga qarab o'lcham
    // Размеры типов C зависят от платформы
    println!("c_char:   {} bayt", std::mem::size_of::<c_char>());   // 1
    println!("c_int:    {} bayt", std::mem::size_of::<c_int>());    // 4
    println!("c_uint:   {} bayt", std::mem::size_of::<c_uint>());   // 4
    println!("c_long:   {} bayt", std::mem::size_of::<c_long>());   // 4 yoki 8
    println!("c_float:  {} bayt", std::mem::size_of::<c_float>());  // 4
    println!("c_double: {} bayt", std::mem::size_of::<c_double>()); // 8
    // c_char:   1 bayt
    // c_int:    4 bayt
    // c_uint:   4 bayt
    // c_long:   8 bayt (Linux 64-bit)
    // c_float:  4 bayt
    // c_double: 8 bayt

    // c_void — ixtiyoriy pointer (void*)
    // c_void — произвольный указатель (void*)
    let n: i32 = 42;
    let void_ptr: *const c_void = &n as *const i32 as *const c_void;
    let back: *const i32 = void_ptr as *const i32;
    unsafe {
        println!("void* → i32: {}", *back); // 42
    }
    // void* → i32: 42

    // C struct — repr(C) bilan
    // C структура — с repr(C)
    #[repr(C)]
    #[derive(Debug)]
    struct CPoint {
        x: c_double,
        y: c_double,
    }

    let p = CPoint { x: 3.14, y: 2.71 };
    println!("{:?}", p);
    println!("CPoint o'lcham: {}", std::mem::size_of::<CPoint>());
    // CPoint { x: 3.14, y: 2.71 }
    // CPoint o'lcham: 16

    // C union — repr(C) bilan
    // C union — с repr(C)
    #[repr(C)]
    union CUnion {
        buton: i32,
        bayt: [u8; 4],
    }

    let u = CUnion { buton: 0x01020304 };
    unsafe {
        println!("union.buton: {:#010X}", u.buton);
        println!("union.bayt: {:?}", u.bayt);
    }
    // union.buton: 0x01020304
    // union.bayt: [4, 3, 2, 1] (little-endian)
}

// C standart kutubxona funksiyalari
// Функции стандартной библиотеки C
unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn abs(n: c_int) -> c_int;
    fn sqrt(n: c_double) -> c_double;
    fn pow(base: c_double, exp: c_double) -> c_double;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(ptr: *mut c_void, val: c_int, n: usize) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn rand() -> c_int;
    fn srand(seed: c_uint);
    fn printf(format: *const c_char, ...) -> c_int;
}

fn extern_c_misollari() {

    // strlen — satr uzunligi
    // strlen — длина строки
    let s = CString::new("salom dunyo").unwrap();
    let uzunlik = unsafe { strlen(s.as_ptr()) };
    println!("strlen: {}", uzunlik); // 11
    // strlen: 11

    // abs — mutlaq qiymat
    // abs — абсолютное значение
    let n: c_int = -42;
    let abs_n = unsafe { abs(n) };
    println!("abs(-42): {}", abs_n); // 42
    // abs(-42): 42

    // sqrt va pow — matematik funksiyalar
    // sqrt и pow — математические функции
    let ildiz = unsafe { sqrt(16.0) };
    let daraja = unsafe { pow(2.0, 10.0) };
    println!("sqrt(16): {}", ildiz);   // 4
    println!("pow(2,10): {}", daraja); // 1024
    // sqrt(16): 4
    // pow(2,10): 1024

    // strcmp — satr taqqoslash
    // strcmp — сравнение строк
    let s1 = CString::new("salom").unwrap();
    let s2 = CString::new("salom").unwrap();
    let s3 = CString::new("dunyo").unwrap();
    let r1 = unsafe { strcmp(s1.as_ptr(), s2.as_ptr()) };
    let r2 = unsafe { strcmp(s1.as_ptr(), s3.as_ptr()) };
    println!("strcmp(salom,salom): {}", r1); // 0 (teng)
    println!("strcmp(salom,dunyo): {}", r2); // > 0 (salom > dunyo)
    // strcmp(salom,salom): 0
    // strcmp(salom,dunyo): 15 (yoki boshqa musbat son)

    // malloc va free — xotira boshqaruvi
    // malloc и free — управление памятью
    unsafe {
        let ptr: *mut c_void = malloc(4 * std::mem::size_of::<i32>());
        if ptr.is_null() { panic!("malloc xatosi!"); }

        let arr: *mut i32 = ptr as *mut i32;
        for i in 0..4 { *arr.add(i) = (i as i32 + 1) * 10; }

        println!("malloc array: [{}, {}, {}, {}]",
                 *arr.add(0), *arr.add(1), *arr.add(2), *arr.add(3));

        free(ptr);
        println!("free bajarildi");
    }
    // malloc array: [10, 20, 30, 40]
    // free bajarildi

    // memset va memcpy
    // memset и memcpy
    unsafe {
        let mut buf = [0u8; 8];
        memset(buf.as_mut_ptr() as *mut c_void, 0xFF, 4);
        println!("memset: {:?}", buf); // [255,255,255,255,0,0,0,0]

        let manba = [1u8, 2, 3, 4];
        memcpy(buf.as_mut_ptr() as *mut c_void,
               manba.as_ptr() as *const c_void, 4);
        println!("memcpy: {:?}", &buf[..4]); // [1,2,3,4]
    }
    // memset: [255, 255, 255, 255, 0, 0, 0, 0]
    // memcpy: [1, 2, 3, 4]

    // rand va srand — psevdo-random
    // rand и srand — псевдослучайные
    unsafe {
        srand(42);
        let r1 = rand() % 100;
        let r2 = rand() % 100;
        println!("rand: {} {}", r1, r2);
    }
    // rand: XX YY
}

// #[unsafe(no_mangle)] — C dan chaqirish uchun
// #[unsafe(no_mangle)] — для вызова из C
#[unsafe(no_mangle)]
pub extern "C" fn rust_qo_shish(a: c_int, b: c_int) -> c_int {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_faktorial(n: c_uint) -> c_uint {
    (1..=n).product()
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_satr_uzunlik(s: *const c_char) -> usize {
    if s.is_null() { return 0; }
    unsafe { CStr::from_ptr(s) }.to_bytes().len()
}

// Callback — C dan funksiya pointerni qabul qilish
// Callback — принятие указателя на функцию из C
#[unsafe(no_mangle)]
pub extern "C" fn rust_har_biriga(
    arr: *const c_int,
    n: c_int,
    callback: extern "C" fn(c_int) -> c_int,
) -> *mut c_int {
    if arr.is_null() || n <= 0 { return ptr::null_mut(); }
    let natijalar: Vec<c_int> = (0..n as usize)
        .map(|i| callback(unsafe { *arr.add(i) }))
        .collect();
    let mut boxed = natijalar.into_boxed_slice();
    let ptr = boxed.as_mut_ptr();
    std::mem::forget(boxed);
    ptr
}

fn rust_dan_c_ga_eksport() {

    // Eksport qilingan funksiyalarni Rust ichida ham ishlatish
    // Использование экспортированных функций внутри Rust
    let yig = rust_qo_shish(10, 32);
    println!("rust_qo'shish(10,32): {}", yig); // 42
    // rust_qo'shish(10,32): 42

    let fakt = rust_faktorial(6);
    println!("rust_faktorial(6): {}", fakt); // 720
    // rust_faktorial(6): 720

    let cs = CString::new("salom").unwrap();
    let uzunlik = rust_satr_uzunlik(cs.as_ptr());
    println!("rust_satr_uzunlik: {}", uzunlik); // 5
    // rust_satr_uzunlik: 5

    // Callback misoli
    extern "C" fn ikki_baravar(x: c_int) -> c_int { x * 2 }

    let arr: Vec<c_int> = vec![1, 2, 3, 4, 5];
    let natija_ptr = rust_har_biriga(arr.as_ptr(), arr.len() as c_int, ikki_baravar);
    if !natija_ptr.is_null() {
        let natijalar = unsafe {
            Vec::from_raw_parts(natija_ptr, arr.len(), arr.len())
        };
        println!("callback natija: {:?}", natijalar); // [2, 4, 6, 8, 10]
    }
    // callback natija: [2, 4, 6, 8, 10]
}

// C kutubxona wrapper — xavfsiz interfeys
// Обёртка C библиотеки — безопасный интерфейс
struct CAllocator {
    ptr: *mut c_void,
    o_lcham: usize,
}

impl CAllocator {
    fn ajrat(o_lcham: usize) -> Option<Self> {
        let ptr = unsafe { malloc(o_lcham) };
        if ptr.is_null() {
            None
        } else {
            unsafe { memset(ptr, 0, o_lcham); }
            Some(CAllocator { ptr, o_lcham })
        }
    }

    fn as_ptr(&self) -> *mut c_void { self.ptr }
    fn o_lcham(&self) -> usize { self.o_lcham }

    fn yoz_i32(&self, offset: usize, val: i32) -> bool {
        if offset + 4 > self.o_lcham { return false; }
        unsafe {
            let p = (self.ptr as *mut u8).add(offset) as *mut i32;
            *p = val;
        }
        true
    }

    fn o_qi_i32(&self, offset: usize) -> Option<i32> {
        if offset + 4 > self.o_lcham { return None; }
        unsafe {
            let p = (self.ptr as *const u8).add(offset) as *const i32;
            Some(*p)
        }
    }
}

impl Drop for CAllocator {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { free(self.ptr); }
            println!("[CAllocator] {} bayt bo'shatildi", self.o_lcham);
        }
    }
}

// CString pool — satrlar uchun xotira pool
// Пул CString — пул памяти для строк
struct CStringPool {
    satrlar: Vec<CString>,
}

impl CStringPool {
    fn new() -> Self { CStringPool { satrlar: Vec::new() } }

    fn qo_sh(&mut self, s: &str) -> Option<*const c_char> {
        let cs = CString::new(s).ok()?;
        self.satrlar.push(cs);
        Some(self.satrlar.last().unwrap().as_ptr())
    }

    fn uzunlik(&self) -> usize { self.satrlar.len() }
}

fn real_hayot_misollari() {

    println!("--- CAllocator ---");
    {
        let alloc = CAllocator::ajrat(32).expect("Xotira ajratish xatosi");
        println!("Ajratildi: {} bayt", alloc.o_lcham());

        alloc.yoz_i32(0, 100);
        alloc.yoz_i32(4, 200);
        alloc.yoz_i32(8, 300);

        println!("offset 0: {:?}", alloc.o_qi_i32(0)); // Some(100)
        println!("offset 4: {:?}", alloc.o_qi_i32(4)); // Some(200)
        println!("offset 8: {:?}", alloc.o_qi_i32(8)); // Some(300)
        println!("offset 30: {:?}", alloc.o_qi_i32(30)); // None (oshib ketadi)
    } // Drop — avtomatik free
    // Ajratildi: 32 bayt
    // offset 0: Some(100)
    // ...
    // [CAllocator] 32 bayt bo'shatildi

    println!("\n--- CString Pool ---");
    let mut pool = CStringPool::new();
    let p1 = pool.qo_sh("birinchi satr").unwrap();
    let p2 = pool.qo_sh("ikkinchi satr").unwrap();
    let p3 = pool.qo_sh("uchinchi satr").unwrap();

    println!("Pool uzunlik: {}", pool.uzunlik()); // 3
    unsafe {
        println!("p1: {}", CStr::from_ptr(p1).to_str().unwrap());
        println!("p2: {}", CStr::from_ptr(p2).to_str().unwrap());
        println!("p3: {}", CStr::from_ptr(p3).to_str().unwrap());
    }
    // Pool uzunlik: 3
    // p1: birinchi satr
    // p2: ikkinchi satr
    // p3: uchinchi satr

    println!("\n--- Eksport Funksiyalar ---");
    rust_dan_c_ga_eksport();

    println!("\n--- CStr/CString xavfsizlik ---");
    // Xavfsiz CStr yaratish
    let xavfsiz_satrlar = ["salom\0", "dunyo\0", "rust\0"];
    for s in &xavfsiz_satrlar {
        let cstr = CStr::from_bytes_with_nul(s.as_bytes()).unwrap();
        println!("CStr: {:?}", cstr.to_str().unwrap());
    }
    // CStr: "salom"
    // CStr: "dunyo"
    // CStr: "rust"
}

fn main() {

    println!("=== CSTRING VA CSTR ===");
    cstring_cstr_misollari();

    println!("\n=== OSSTR VA OSSTRING ===");
    osstr_osstring_misollari();

    println!("\n=== C TURLARI ===");
    c_turlari_misollari();

    println!("\n=== EXTERN C ===");
    extern_c_misollari();

    println!("\n=== RUST → C EKSPORT ===");
    rust_dan_c_ga_eksport();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        CSTRING VA CSTR                                                                                       |
// #================================================================================================================================================#
// # |   1 | CString::new("s")               | Rust → null-terminated C satr              | Rust → null-terminated C строка                         |
// # |   2 | cs.as_ptr()                     | *const c_char — C ga berish                | *const c_char — передача в C                            |
// # |   3 | cs.into_raw()                   | Ownership C ga o'tkazish                   | Передача владения в C                                   |
// # |   4 | CString::from_raw(ptr)          | C dan ownership qaytarish                  | Возврат владения из C                                   |
// # |   5 | CStr::from_ptr(ptr)             | C pointerdan CStr                          | CStr из C указателя                                     |
// # |   6 | CStr::from_bytes_with_nul(b)    | Baytlardan CStr (null bilan)               | CStr из байтов (с null)                                 |
// # |   7 | cstr.to_str()                   | CStr → &str (UTF-8 tekshiruvi bilan)       | CStr → &str (с проверкой UTF-8)                         |
// # |   8 | cstr.to_string_lossy()          | CStr → Cow<str> (UTF-8 bo'lmasa ?)         | CStr → Cow<str> (? если не UTF-8)                       |
// #================================================================================================================================================#
// # |                                        OSSTR VA OSSTRING                                                                                     |
// #================================================================================================================================================#
// # |   9 | OsStr::new("s")                 | Platform-native satr (borrowed)            | Платформно-нативная строка (borrowed)                   |
// # |  10 | OsString::from("s")             | Platform-native satr (owned)               | Платформно-нативная строка (owned)                      |
// # |  11 | os.to_str()                     | OsStr → Option<&str>                       | OsStr → Option<&str>                                    |
// # |  12 | os.to_string_lossy()            | OsStr → Cow<str>                           | OsStr → Cow<str>                                        |
// #================================================================================================================================================#
// # |                                        C TURLARI                                                                                             |
// #================================================================================================================================================#
// # |  13 | c_char, c_int, c_uint           | C asosiy integer turlari                   | Основные целочисленные типы C                           |
// # |  14 | c_float, c_double               | C suzuvchi vergul turlari                  | Типы с плавающей запятой C                              |
// # |  15 | c_void                          | Ixtiyoriy pointer (void*)                  | Произвольный указатель (void*)                          |
// # |  16 | #[repr(C)]                      | C mos keluvchi struct tartibi              | Совместимый с C порядок структуры                       |
// #================================================================================================================================================#
// # |                                        EXTERN C                                                                                              |
// #================================================================================================================================================#
// # |  17 | extern "C" { fn f(...); }                | C funksiyasini import qilish               | Импорт функции C                               |
// # |  18 | #[unsafe(no_mangle)] pub extern "C" fn f | Rust → C eksport                           | Экспорт Rust → C                               |
// # |  19 | extern "C" fn callback(...)              | C callback funksiya pointeri               | Указатель на функцию обратного вызова C        |
// # |  20 | malloc/free                              | C xotira boshqaruvi                        | Управление памятью C                           |
// #================================================================================================================================================#