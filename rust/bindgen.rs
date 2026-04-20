// #================================================================================================================================================#
// #                                                                    FFI (BINDGEN)                                                               #
// #                            BINDGEN — C HEADER → RUST BINDING. BUILD.RS. WRAPPER. SAFE ABSTRAKTSIYA. PQALIB OLISH.                              #
// #                            BINDGEN — C HEADER → RUST BINDING. BUILD.RS. WRAPPER. SAFE АБСТРАКЦИЯ. ПРИМЕРЫ ИСПОЛЬЗОВАНИЯ.                       #
// #================================================================================================================================================#

#![allow(dead_code, unused, improper_ctypes_definitions)]

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_double, c_void, c_ulong};
use std::ptr;
use std::fmt;

// Bindgen nima:
// Что такое Bindgen:
//
//   C header faylidan avtomatik Rust binding yaratadi
//   Автоматически генерирует Rust привязки из заголовков C
//
//   Ishlatish:
//   Использование:
//   1. CLI: bindgen header.h -o src/bindings.rs
//   2. build.rs: avtomatik generatsiya
//
//   Cargo.toml:
//   [build-dependencies]
//   bindgen = "0.69"
//   cc = "1.0"
//
//   build.rs:
//   fn main() {
//       let bindings = bindgen::Builder::default()
//           .header("wrapper.h")
//           .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
//           .generate()
//           .expect("Binding generatsiya xatosi");
//
//       let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
//       bindings.write_to_file(out.join("bindings.rs")).unwrap();
//   }

fn bindgen_jarayoni() {

    println!("=== BINDGEN JARAYONI ===\n");

    println!(r#"// 1. C kutubxona header (mylib.h):
// ─────────────────────────────────
typedef struct {{
    double x, y, z;
}} Vector3;

typedef struct {{
    int width, height;
    unsigned char *pixels;
}} Image;

typedef void (*ErrorCallback)(int code, const char *msg);

int  mylib_init(void);
void mylib_cleanup(void);
Vector3 mylib_vec_add(Vector3 a, Vector3 b);
Image*  mylib_image_new(int w, int h);
void    mylib_image_free(Image *img);
int     mylib_image_pixel(Image *img, int x, int y);
void    mylib_set_error_handler(ErrorCallback cb);
const char* mylib_version(void);

// 2. bindgen generatsiya qilgan (src/bindings.rs):
// ─────────────────────────────────────────────────
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vector3 {{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Image {{
    pub width:  ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub pixels: *mut ::std::os::raw::c_uchar,
}}

pub type ErrorCallback = ::std::option::Option<
    unsafe extern "C" fn(
        code: ::std::os::raw::c_int,
        msg: *const ::std::os::raw::c_char,
    )
>;

unsafe extern "C" {{
    pub fn mylib_init() -> ::std::os::raw::c_int;
    pub fn mylib_cleanup();
    pub fn mylib_vec_add(a: Vector3, b: Vector3) -> Vector3;
    pub fn mylib_image_new(w: c_int, h: c_int) -> *mut Image;
    pub fn mylib_image_free(img: *mut Image);
    pub fn mylib_image_pixel(img: *mut Image, x: c_int, y: c_int) -> c_int;
    pub fn mylib_set_error_handler(cb: ErrorCallback);
    pub fn mylib_version() -> *const ::std::os::raw::c_char;
}}"#);

    println!("\n// 3. lib.rs da ishlatish:");
    println!(r#"// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));"#);
}

fn bindgen_konfiguratsiya() {

    println!("\n=== BINDGEN KONFIGURATSIYA ===\n");

    println!(r#"// build.rs — kengaytirilgan konfiguratsiya
fn main() {{
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=mylib.h");

    // C kutubxonani link qilish
    println!("cargo:rustc-link-lib=mylib");        // -lmylib
    println!("cargo:rustc-link-search=native=/usr/local/lib");

    let bindings = bindgen::Builder::default()
        // Asosiy header
        .header("wrapper.h")

        // Include yo'llari
        .clang_arg("-I/usr/local/include")
        .clang_arg("-I./third_party/include")

        // Faqat ma'lum funksiyalar
        .allowlist_function("mylib_.*")
        .allowlist_type("Vector3|Image")
        .allowlist_var("MYLIB_.*")

        // Bloklash
        .blocklist_function("mylib_internal_.*")
        .blocklist_type("_.*")

        // Rustified enum — C enum → Rust enum
        .rustified_enum("MyError")
        .rustified_enum("ColorSpace")

        // Derive
        .derive_debug(true)
        .derive_copy(true)
        .derive_clone(true)
        .derive_default(false)
        .derive_partialeq(true)

        // Callback (unsafe)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))

        // va'dalangan C nomlar
        .prepend_enum_name(false)

        // size_t → usize
        .size_t_is_usize(true)

        // Dynamic linking uchun
        // .dynamic_library_name("mylib")

        .generate()
        .expect("Binding generatsiya xatosi!");

    let out = std::path::PathBuf::from(
        std::env::var("OUT_DIR").unwrap()
    );
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Faylga yozish xatosi!");
}}"#);
}

// Bindgen generatsiya qilgan low-level binding simulyatsiyasi
// Симуляция low-level привязок, генерируемых bindgen

// --- LOW-LEVEL BINDINGS (bindgen generated) ---
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CVektor3 { pub x: c_double, pub y: c_double, pub z: c_double }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CImage {
    pub kenglik: c_int,
    pub balandlik: c_int,
    pub piksellar: *mut u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CRang { Qizil = 0, Yashil = 1, KoK = 2, Alpha = 3 }

pub type CErrorCallback = Option<unsafe extern "C" fn(c_int, *const c_char)>;

// C funksiyalar simulyatsiyasi (haqiqiy loyihada bu bindgen generatsiya qiladi)
// Симуляция C функций (в реальном проекте их генерирует bindgen)
unsafe extern "C" fn c_vektor_qo_shish(a: CVektor3, b: CVektor3) -> CVektor3 {
    CVektor3 { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z }
}

unsafe extern "C" fn c_vektor_uzunlik(v: CVektor3) -> c_double {
    (v.x*v.x + v.y*v.y + v.z*v.z).sqrt()
}

unsafe extern "C" fn c_vektor_normallashtir(v: CVektor3) -> CVektor3 {
    let uz = unsafe { c_vektor_uzunlik(v) };
    if uz < 1e-10 { return CVektor3 { x: 0.0, y: 0.0, z: 0.0 }; }
    CVektor3 { x: v.x/uz, y: v.y/uz, z: v.z/uz }
}

unsafe extern "C" fn c_image_yaratish(k: c_int, b: c_int) -> *mut CImage {
    let piksel_soni = (k * b * 4) as usize; // RGBA
    let mut piksellar = vec![0u8; piksel_soni];
    let ptr = piksellar.as_mut_ptr();
    std::mem::forget(piksellar);
    let img = Box::new(CImage { kenglik: k, balandlik: b, piksellar: ptr });
    Box::into_raw(img)
}

unsafe extern "C" fn c_image_o_chir(img: *mut CImage) {
    if img.is_null() { return; }
    let img_ref = unsafe { &*img };
    let soni = (img_ref.kenglik * img_ref.balandlik * 4) as usize;
    unsafe {
        drop(Vec::from_raw_parts(img_ref.piksellar, soni, soni));
        drop(Box::from_raw(img));
    }
}

unsafe extern "C" fn c_image_piksel_o_rnat(img: *mut CImage, x: c_int, y: c_int,
                                           r: u8, g: u8, b: u8, a: u8) -> c_int
{
    if img.is_null() { return -1; }
    let img_ref = unsafe { &*img };
    if x < 0 || y < 0 || x >= img_ref.kenglik || y >= img_ref.balandlik { return -1; }
    let offset = ((y * img_ref.kenglik + x) * 4) as usize;
    unsafe {
        *img_ref.piksellar.add(offset)     = r;
        *img_ref.piksellar.add(offset + 1) = g;
        *img_ref.piksellar.add(offset + 2) = b;
        *img_ref.piksellar.add(offset + 3) = a;
    }
    0
}

// --- HIGH-LEVEL SAFE WRAPPER ---
// Bindgen ustida qurilgan xavfsiz Rust API
// Безопасный Rust API, построенный поверх bindgen

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vektor3 { pub x: f64, pub y: f64, pub z: f64 }

impl Vektor3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self { Vektor3 { x, y, z } }

    fn to_c(&self) -> CVektor3 { CVektor3 { x: self.x, y: self.y, z: self.z } }
    fn from_c(c: CVektor3) -> Self { Vektor3 { x: c.x, y: c.y, z: c.z } }

    pub fn qo_shish(&self, b: &Vektor3) -> Vektor3 {
        Vektor3::from_c(unsafe { c_vektor_qo_shish(self.to_c(), b.to_c()) })
    }

    pub fn uzunlik(&self) -> f64 {
        unsafe { c_vektor_uzunlik(self.to_c()) }
    }

    pub fn normallashtir(&self) -> Vektor3 {
        Vektor3::from_c(unsafe { c_vektor_normallashtir(self.to_c()) })
    }

    pub fn nuqta_ko_paytma(&self, b: &Vektor3) -> f64 {
        self.x*b.x + self.y*b.y + self.z*b.z
    }
}

impl fmt::Display for Vektor3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }
}

impl std::ops::Add for Vektor3 {
    type Output = Self;
    fn add(self, b: Self) -> Self { self.qo_shish(&b) }
}

// RAII wrapper image uchun
// RAII обёртка для image
pub struct Image {
    ptr: *mut CImage,
}

impl Image {
    pub fn new(kenglik: u32, balandlik: u32) -> Option<Self> {
        let ptr = unsafe { c_image_yaratish(kenglik as c_int, balandlik as c_int) };
        if ptr.is_null() { None } else { Some(Image { ptr }) }
    }

    pub fn kenglik(&self) -> u32 { unsafe { (*self.ptr).kenglik as u32 } }
    pub fn balandlik(&self) -> u32 { unsafe { (*self.ptr).balandlik as u32 } }

    pub fn piksel_o_rnat(&mut self, x: u32, y: u32,
                         r: u8, g: u8, b: u8, a: u8) -> Result<(), String>
    {
        let natija = unsafe {
            c_image_piksel_o_rnat(self.ptr, x as c_int, y as c_int, r, g, b, a)
        };
        if natija == 0 { Ok(()) }
        else { Err(format!("Piksel ({},{}) o'rnatib bo'lmadi", x, y)) }
    }

    pub fn to_raw(&self) -> *const CImage { self.ptr }
}

impl Drop for Image {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { c_image_o_chir(self.ptr); }
        }
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image({}x{})", self.kenglik(), self.balandlik())
    }
}

fn xavfsiz_wrapper_misoli() {

    println!("\n=== XAVFSIZ WRAPPER ===");

    // Vektor3 — xavfsiz API
    let v1 = Vektor3::new(3.0, 4.0, 0.0);
    let v2 = Vektor3::new(1.0, 2.0, 0.0);

    println!("v1 = {}", v1);
    println!("v2 = {}", v2);
    println!("v1 + v2 = {}", v1 + v2);
    println!("|v1| = {:.3}", v1.uzunlik());
    println!("v1 norm = {}", v1.normallashtir());
    println!("v1 · v2 = {}", v1.nuqta_ko_paytma(&v2));
    // v1 = (3.000, 4.000, 0.000)
    // v2 = (1.000, 2.000, 0.000)
    // v1 + v2 = (4.000, 6.000, 0.000)
    // |v1| = 5.000
    // v1 norm = (0.600, 0.800, 0.000)
    // v1 · v2 = 11.000

    // Image — RAII bilan
    println!("\n--- Image RAII Wrapper ---");
    {
        let mut img = Image::new(4, 4).expect("Image yaratib bo'lmadi");
        println!("{:?}", img);

        img.piksel_o_rnat(0, 0, 255, 0, 0, 255).unwrap();   // Qizil
        img.piksel_o_rnat(1, 0, 0, 255, 0, 255).unwrap();   // Yashil
        img.piksel_o_rnat(2, 0, 0, 0, 255, 255).unwrap();   // Ko'k

        // Chegaradan tashqari
        let xato = img.piksel_o_rnat(10, 10, 0, 0, 0, 0);
        println!("Chegaradan tashqari: {:?}", xato);

        println!("Image: {}x{}", img.kenglik(), img.balandlik());
        println!("Scope tugamoqda...");
    } // Drop avtomatik — c_image_o_chir chaqiriladi
    println!("Image xotirasi bo'shatildi ✅");
    // Image(4x4)
    // Chegaradan tashqari: Err("Piksel (10,10) o'rnatib bo'lmadi")
    // Image: 4x4
    // Scope tugamoqda...
    // Image xotirasi bo'shatildi ✅
}

fn mashhur_kutubxonalar() {

    println!("\n=== MASHHUR KUTUBXONALAR ===\n");

    println!("1. SQLite — rusqlite (bindgen + wrapper):");
    println!(r#"   [dependencies]
   rusqlite = {{ version = "0.31", features = ["bundled"] }}

   use rusqlite::{{Connection, Result}};
   let conn = Connection::open("test.db")?;
   conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER, name TEXT)", [])?;
   conn.execute("INSERT INTO users VALUES (1, 'Dilshod')", [])?;
   let nomi: String = conn.query_row(
       "SELECT name FROM users WHERE id = ?1",
       [1], |row| row.get(0)
   )?;"#);

    println!("\n2. OpenSSL — openssl crate:");
    println!(r#"   [dependencies]
   openssl = "0.10"

   use openssl::ssl::{{SslConnector, SslMethod}};
   use openssl::hash::{{MessageDigest, hash}};
   let sha256 = hash(MessageDigest::sha256(), b"salom rust")?;"#);

    println!("\n3. libz (zlib) — flate2:");
    println!(r#"   [dependencies]
   flate2 = "1.0"

   use flate2::write::GzEncoder;
   use flate2::Compression;
   let mut e = GzEncoder::new(Vec::new(), Compression::default());
   e.write_all(b"salom dunyo")?;
   let compressed = e.finish()?;"#);

    println!("\n4. Python — pyo3:");
    println!(r#"   [dependencies]
   pyo3 = {{ version = "0.21", features = ["extension-module"] }}

   use pyo3::prelude::*;
   #[pyfunction]
   fn faktorial(n: u64) -> u64 {{ (1..=n).product() }}

   #[pymodule]
   fn mymodule(m: &Bound<'_, PyModule>) -> PyResult<()> {{
       m.add_function(wrap_pyfunction!(faktorial, m)?)
   }}"#);

    println!("\n5. libgit2 — git2:");
    println!(r#"   [dependencies]
   git2 = "0.18"

   let repo = git2::Repository::open(".")?;
   let head = repo.head()?;
   println!("Branch: {{}}", head.shorthand().unwrap_or("?"));"#);
}

fn dynamic_linking_misoli() {

    println!("\n=== DYNAMIC LINKING ===\n");

    println!(r#"// libloading crate bilan dynamic loading
// [dependencies]
// libloading = "0.8"

use libloading::{{Library, Symbol}};

type QoShishFn = unsafe extern "C" fn(i32, i32) -> i32;
type VersionFn = unsafe extern "C" fn() -> *const std::os::raw::c_char;

fn dynamic_lib_misol() -> Result<(), Box<dyn std::error::Error>> {{
    // Runtime da kutubxona yuklash
    let lib = unsafe {{ Library::new("libmylib.so") }}?;

    // Funksiya simbolini olish
    let qo_shish: Symbol<QoShishFn> = unsafe {{
        lib.get(b"mylib_qoshish\0")?
    }};

    let natija = unsafe {{ qo_shish(10, 32) }};
    println!("10 + 32 = {{}}", natija); // 42

    // Versiya
    let version: Symbol<VersionFn> = unsafe {{
        lib.get(b"mylib_version\0")?
    }};
    let v = unsafe {{ std::ffi::CStr::from_ptr(version()) }};
    println!("Versiya: {{}}", v.to_str()?);

    Ok(())
}}

// Plugin tizimi — dlopen bilan
struct Plugin {{
    lib: Library,
    ishga_tushir: Symbol<unsafe extern "C" fn() -> *mut c_void>,
    bajar: Symbol<unsafe extern "C" fn(*mut c_void, i32) -> i32>,
    to_xtat: Symbol<unsafe extern "C" fn(*mut c_void)>,
}}"#);
}

fn main() {

    bindgen_jarayoni();
    bindgen_konfiguratsiya();
    xavfsiz_wrapper_misoli();
    mashhur_kutubxonalar();
    dynamic_linking_misoli();

    println!("\n=== XULOSA ===");
    println!("Bindgen workflow:");
    println!("  1. C header (wrapper.h) → bindgen → src/bindings.rs");
    println!("  2. Low-level unsafe bindings (avtomatik)");
    println!("  3. High-level safe wrapper (qo'lda)");
    println!("  4. RAII — Drop bilan xotira boshqaruvi");
    println!("  5. Error handling — Result<T, E>");
    println!("  6. Tests — integration test");
    println!();
    println!("Qoidalar:");
    println!("  - allowlist_* — faqat keraklilarni kiriting");
    println!("  - cargo:rerun-if-changed — rebuild trigger");
    println!("  - wrapper.h — barcha headerlarni birlashtiring");
    println!("  - Safe wrapper DOIM yozing");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        BINDGEN                                                                                               |
// #================================================================================================================================================#
// # |   1 | bindgen::Builder::default()     | Builder yaratish                           | Создание Builder                                        |
// # |   2 | .header("wrapper.h")            | Asosiy header                              | Основной заголовок                                      |
// # |   3 | .allowlist_function("pat.*")    | Faqat mos funksiyalar                      | Только совпадающие функции                              |
// # |   4 | .allowlist_type("Type")         | Faqat mos turlar                           | Только совпадающие типы                                 |
// # |   5 | .blocklist_function("_.*")      | Bloklash                                   | Блокировка                                              |
// # |   6 | .rustified_enum("MyEnum")       | C enum → Rust enum                         | C enum → Rust enum                                      |
// # |   7 | .derive_debug(true)             | Debug derive                               | Derive Debug                                            |
// # |   8 | .clang_arg("-I/path")           | Include yo'li                              | Путь к заголовкам                                       |
// # |   9 | .generate() → bindings          | Binding generatsiya                        | Генерация привязок                                      |
// # |  10 | include!(concat!(...))          | Generatsiya qilinganlarni ulash            | Подключение сгенерированных привязок                    |
// #================================================================================================================================================#
// # |                                        WRAPPER PATTERNLARI                                                                                   |
// #================================================================================================================================================#
// # |  11 | RAII wrapper                    | Drop bilan avtomatik tozalash              | Автоматическая очистка через Drop                       |
// # |  12 | to_c() / from_c()               | Rust ↔ C konversiya                        | Конверсия Rust ↔ C                                      |
// # |  13 | Result<T, E>                    | C xato kodini Rust xatosiga                | Преобразование кода ошибки C в Rust                     |
// # |  14 | Dynamic loading                 | libloading bilan runtime yuklash           | Загрузка во время выполнения с libloading               |
// # |  15 | cargo:rustc-link-lib=name       | Kutubxonani link qilish                    | Линковка библиотеки                                     |
// #================================================================================================================================================#