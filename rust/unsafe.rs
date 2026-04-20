// #================================================================================================================================================#
// #                                                                UNSAFE KOD YOZISH                                                               #
// #                                 UNSAFE — QACHON VA NIMA UCHUN. UNSAFE SUPERPOWERS 5 TA. XAVFSIZ ABSTRAKTSIYA QURISH.                           #
// #                                 UNSAFE — КОГДА И ЗАЧЕМ. 5 СУПЕРСИЛ UNSAFE. ПОСТРОЕНИЕ БЕЗОПАСНЫХ АБСТРАКЦИЙ.                                   #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::ptr;
use std::slice;
use std::mem;

// Unsafe nima:
// Что такое Unsafe:
//
//   Rust kompilyatori ba'zi tekshiruvlarni o'chiradi
//   Компилятор Rust отключает некоторые проверки
//
//   unsafe 5 ta "superpower":
//   5 "суперсил" unsafe:
//   1. Raw pointer dereference qilish
//      Разыменование сырых указателей
//   2. Unsafe funksiyani chaqirish
//      Вызов unsafe функции
//   3. Mutable static o'zgaruvchiga kirish
//      Доступ к изменяемой статической переменной
//   4. Unsafe trait implement qilish
//      Реализация unsafe трейта
//   5. Union maydoniga kirish
//      Доступ к полю union
//
//   Javobgarlik:
//   Ответственность:
//   - Unsafe blok ichidagi xavfsizlik SIZDA
//   - Безопасность внутри unsafe блока НА ВАС
//   - "Unsafe" = "Men xavfsizligini ta'minlayman"
//   - "Unsafe" = "Я гарантирую безопасность"
//
//   Qoida:
//   Правило:
//   Unsafe blok IMKON BORICHA KICHIK bo'lsin
//   Unsafe блок должен быть КАК МОЖНО МЕНЬШЕ

fn raw_pointer_misoli() {

    println!("--- 1. Raw Pointer Dereference ---");

    // Safe: pointer yaratish
    // Safe: создание указателя
    let n: i32 = 42;
    let ptr: *const i32 = &n as *const i32;
    let mut m: i32 = 100;
    let ptr_mut: *mut i32 = &mut m as *mut i32;

    println!("Pointer: {:p}", ptr);
    println!("Null: {}", ptr.is_null());
    // Pointer: 0x...
    // Null: false

    // Unsafe: dereference
    // Unsafe: разыменование
    unsafe {
        println!("*ptr = {}", *ptr);          // 42
        *ptr_mut = 200;
        println!("*ptr_mut = {}", *ptr_mut);  // 200
    }
    println!("m = {}", m); // 200
    // *ptr = 42
    // *ptr_mut = 200
    // m = 200

    // Null pointer tekshiruv — kerakli!
    // Проверка null указателя — обязательна!
    fn xavfsiz_dereference(ptr: *const i32) -> Option<i32> {
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { *ptr })
        }
    }

    let val = 77i32;
    println!("{:?}", xavfsiz_dereference(&val));          // Some(77)
    println!("{:?}", xavfsiz_dereference(ptr::null()));   // None
    // Some(77)
    // None

    // Raw pointer arifmetika
    // Арифметика сырых указателей
    let arr = [10i32, 20, 30, 40, 50];
    let ptr2 = arr.as_ptr();

    unsafe {
        for i in 0..arr.len() {
            print!("{} ", *ptr2.add(i));
        }
        println!();
    }
    // 10 20 30 40 50

    // Box::into_raw / Box::from_raw
    // Box::into_raw / Box::from_raw
    let boxed = Box::new(String::from("owned"));
    let raw: *mut String = Box::into_raw(boxed);
    unsafe {
        println!("{}", *raw);
        (*raw).push_str(" dunyo");
        println!("{}", *raw);
        drop(Box::from_raw(raw)); // xotirani qaytarish
    }
    // owned
    // owned dunyo
}

unsafe fn xavfli_funksiya(ptr: *const i32) -> i32 {
    unsafe { *ptr } // Rust 2024: unsafe fn ichida ham unsafe blok kerak
}

unsafe fn noldan_bo_shlash<T>(ptr: *mut T, uzunlik: usize) {
    for i in 0..uzunlik {
        unsafe { ptr.add(i).write(mem::zeroed()); }
    }
}

// Stdlib unsafe funksiyalar
// Unsafe функции стандартной библиотеки
fn stdlib_unsafe_misoli() {

    println!("\n--- 2. Unsafe Funksiya Chaqirish ---");

    let n = 42i32;
    let ptr = &n as *const i32;

    let natija = unsafe { xavfli_funksiya(ptr) };
    println!("Natija: {}", natija); // 42
    // Natija: 42

    // slice::from_raw_parts — raw pointer → &[T]
    // slice::from_raw_parts — raw pointer → &[T]
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = unsafe {
        slice::from_raw_parts(v.as_ptr(), v.len())
    };
    println!("{:?}", slice); // [1, 2, 3, 4, 5]
    // [1, 2, 3, 4, 5]

    // slice::from_raw_parts_mut
    // slice::from_raw_parts_mut
    let mut arr = [0u8; 10];
    let slice_mut: &mut [u8] = unsafe {
        slice::from_raw_parts_mut(arr.as_mut_ptr(), 5)
    };
    for (i, b) in slice_mut.iter_mut().enumerate() {
        *b = (i * 10) as u8;
    }
    println!("{:?}", arr); // [0, 10, 20, 30, 40, 0, 0, 0, 0, 0]
    // [0, 10, 20, 30, 40, 0, 0, 0, 0, 0]

    // str::from_utf8_unchecked — UTF-8 tekshiruvsiz
    // str::from_utf8_unchecked — без проверки UTF-8
    let baytlar = b"Salom Rust!";
    let s: &str = unsafe { std::str::from_utf8_unchecked(baytlar) };
    println!("{}", s); // Salom Rust!
    // Salom Rust!

    // mem::transmute — tur o'zgartirish
    // mem::transmute — преобразование типов
    let bits: u32 = 0x3F800000;
    let f: f32 = f32::from_bits(bits);
    println!("transmute: {}", f); // 1.0
    // transmute: 1.0
}

// Static mut — global o'zgaruvchi
// Static mut — глобальная переменная
static mut GLOBAL_HISOB: u64 = 0;
// static mut Vec<String> — Rust 2024 da taqiqlangan, OnceLock<Mutex<Vec>> ishlatiladi

// Xavfsiz wrapper — Mutex bilan
// Безопасная обёртка — с Mutex
use std::sync::{Mutex, OnceLock};

static XAVFSIZ_HISOB: OnceLock<Mutex<u64>> = OnceLock::new();

fn xavfsiz_hisob() -> &'static Mutex<u64> {
    XAVFSIZ_HISOB.get_or_init(|| Mutex::new(0))
}

fn mutable_static_misoli() {

    println!("\n--- 3. Mutable Static ---");

    // Unsafe static mut — thread-safe EMAS!
    // Unsafe static mut — НЕ потокобезопасно!
    unsafe {
        GLOBAL_HISOB += 1;
        GLOBAL_HISOB += 1;
        let hisob_val = GLOBAL_HISOB; // copy — shared reference emas
        println!("GLOBAL_HISOB: {}", hisob_val); // 2
    }
    // GLOBAL_HISOB: 2

    // Xavfsiz alternativ — OnceLock<Mutex<T>>
    // Безопасная альтернатива — OnceLock<Mutex<T>>
    {
        let mut h = xavfsiz_hisob().lock().unwrap();
        *h += 10;
        *h += 20;
        println!("Xavfsiz hisob: {}", *h); // 30
    }
    // Xavfsiz hisob: 30

    // AtomicU64 — eng yaxshi yechim
    // AtomicU64 — лучшее решение
    use std::sync::atomic::{AtomicU64, Ordering};
    static ATOMIC_HISOB: AtomicU64 = AtomicU64::new(0);

    ATOMIC_HISOB.fetch_add(5, Ordering::SeqCst);
    ATOMIC_HISOB.fetch_add(5, Ordering::SeqCst);
    println!("Atomic: {}", ATOMIC_HISOB.load(Ordering::SeqCst)); // 10
    // Atomic: 10
}

// Unsafe trait — implementor xavfsizlikni kafolatlaydi
// Unsafe трейт — implementor гарантирует безопасность

// Send va Sync — unsafe trait
// Send и Sync — unsafe трейты
// unsafe impl Send for T {}
// unsafe impl Sync for T {}

unsafe trait XotiraXavfsiz {
    fn xotira_manzili(&self) -> usize;
    fn o_lcham(&self) -> usize;
}

unsafe impl XotiraXavfsiz for i32 {
    fn xotira_manzili(&self) -> usize { self as *const i32 as usize }
    fn o_lcham(&self) -> usize { mem::size_of::<i32>() }
}

unsafe impl XotiraXavfsiz for f64 {
    fn xotira_manzili(&self) -> usize { self as *const f64 as usize }
    fn o_lcham(&self) -> usize { mem::size_of::<f64>() }
}

fn unsafe_trait_misoli() {

    println!("\n--- 4. Unsafe Trait ---");

    let n: i32 = 42;
    let f: f64 = 3.14;

    println!("i32 manzil: {:#x}", n.xotira_manzili());
    println!("i32 o'lcham: {} bayt", n.o_lcham());
    println!("f64 manzil: {:#x}", f.xotira_manzili());
    println!("f64 o'lcham: {} bayt", f.o_lcham());
    // i32 manzil: 0x...
    // i32 o'lcham: 4 bayt
    // f64 manzil: 0x...
    // f64 o'lcham: 8 bayt

    // ManuallyDrop + Send manualimpl
    use std::marker::PhantomData;

    struct ThreadXavfsizWrapper<T> {
        qiymat: T,
        _marker: PhantomData<T>,
    }

    unsafe impl<T: Send> Send for ThreadXavfsizWrapper<T> {}

    println!("Unsafe trait implement qilindi ✅");
}

#[repr(C)]
union BitPattern {
    tam_son: i32,
    baytlar: [u8; 4],
    kichik_i16: i16,
}

#[repr(C)]
union F32Bits {
    qiymat: f32,
    bits: u32,
}

fn union_misoli() {

    println!("\n--- 5. Union ---");

    // Union — xotira ulashish
    // Union — совместное использование памяти
    let u = BitPattern { tam_son: 0x01020304 };

    unsafe {
        println!("tam_son: {:#010X}", u.tam_son);
        println!("baytlar: {:?}", u.baytlar);   // little-endian
        println!("kichik_i16: {}", u.kichik_i16);
    }
    // tam_son: 0x01020304
    // baytlar: [4, 3, 2, 1] (little-endian)
    // kichik_i16: 772

    // f32 bit pattern ko'rish
    // Просмотр битового представления f32
    let f = F32Bits { qiymat: 1.0f32 };
    unsafe {
        println!("\nf32(1.0) bits: {:#010X}", f.bits); // 0x3F800000
        println!("f32(1.0) qiymat: {}", f.qiymat);
    }
    // f32(1.0) bits: 0x3F800000
    // f32(1.0) qiymat: 1

    let f2 = F32Bits { bits: 0x3F800000 };
    unsafe { println!("0x3F800000 → f32: {}", f2.qiymat); } // 1.0
    // 0x3F800000 → f32: 1

    // Union o'lcham — eng katta variantga teng
    // Размер union — равен наибольшему варианту
    println!("\nUnion o'lcham: {} bayt", mem::size_of::<BitPattern>()); // 4
    // Union o'lcham: 4 bayt
}

// Unsafe ichida, tashqi API xavfsiz
// Внутри unsafe, внешний API безопасный
pub struct SodaVec<T> {
    ptr: ptr::NonNull<T>,
    uzunlik: usize,
    sig_im: usize,
}

impl<T> SodaVec<T> {
    pub fn new() -> Self {
        SodaVec {
            ptr: ptr::NonNull::dangling(),
            uzunlik: 0,
            sig_im: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        if self.uzunlik == self.sig_im {
            let yangi_sig_im = if self.sig_im == 0 { 4 } else { self.sig_im * 2 };
            let layout = std::alloc::Layout::array::<T>(yangi_sig_im).unwrap();
            let yangi_ptr = if self.sig_im == 0 {
                unsafe { std::alloc::alloc(layout) as *mut T }
            } else {
                let eski = std::alloc::Layout::array::<T>(self.sig_im).unwrap();
                unsafe { std::alloc::realloc(self.ptr.as_ptr() as *mut u8, eski, layout.size()) as *mut T }
            };
            self.ptr = ptr::NonNull::new(yangi_ptr).expect("Alloc xatosi");
            self.sig_im = yangi_sig_im;
        }
        unsafe { self.ptr.as_ptr().add(self.uzunlik).write(val); }
        self.uzunlik += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.uzunlik == 0 { return None; }
        self.uzunlik -= 1;
        Some(unsafe { self.ptr.as_ptr().add(self.uzunlik).read() })
    }

    pub fn ol(&self, i: usize) -> Option<&T> {
        if i >= self.uzunlik { return None; }
        Some(unsafe { &*self.ptr.as_ptr().add(i) })
    }

    pub fn uzunlik(&self) -> usize { self.uzunlik }
    pub fn boshmi(&self) -> bool { self.uzunlik == 0 }

    pub fn as_slice(&self) -> &[T] {
        if self.uzunlik == 0 { return &[]; }
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.uzunlik) }
    }
}

impl<T> Drop for SodaVec<T> {
    fn drop(&mut self) {
        if self.sig_im > 0 {
            unsafe {
                for i in 0..self.uzunlik {
                    ptr::drop_in_place(self.ptr.as_ptr().add(i));
                }
                let layout = std::alloc::Layout::array::<T>(self.sig_im).unwrap();
                std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SodaVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.as_slice())
    }
}

fn xavfsiz_abstraktsiya_misoli() {

    println!("\n--- Xavfsiz Abstraktsiya: SodaVec ---");

    let mut v: SodaVec<String> = SodaVec::new();
    println!("Bosh: {}", v.boshmi()); // true

    v.push(String::from("salom"));
    v.push(String::from("dunyo"));
    v.push(String::from("rust"));
    v.push(String::from("tili"));
    v.push(String::from("ajoyib")); // sig'im 2x bo'ladi

    println!("{:?}", v);
    println!("Uzunlik: {}", v.uzunlik());
    println!("v[1]: {:?}", v.ol(1));
    println!("Pop: {:?}", v.pop());
    println!("Pop: {:?}", v.pop());
    println!("Keyin: {:?}", v);
    // ["salom", "dunyo", "rust", "tili", "ajoyib"]
    // Uzunlik: 5
    // v[1]: Some("dunyo")
    // Pop: Some("ajoyib")
    // Pop: Some("tili")
    // ["salom", "dunyo", "rust"]

    // Slice sifatida ishlatish
    for s in v.as_slice() {
        print!("{} ", s);
    }
    println!();
    // salom dunyo rust
}

fn unsafe_qoidalari() {

    println!("\n--- Unsafe Qoidalari ---");

    println!("1. Unsafe blok imkon boricha kichik bo'lsin");
    println!("2. Har bir unsafe blokga izoh yozing (XAVFSIZLIK kafolati)");
    println!("3. Invariantlarni aniq belgilang");
    println!("4. Xavfsiz API ni tashqiga chiqaring");
    println!("5. Null pointer tekshiring");
    println!("6. Alignment tekshiring");
    println!("7. Lifetime ni kuzating");
    println!("8. Thread xavfsizligini ta'minlang");
    println!("9. Miri bilan tekshiring (cargo miri test)");
    println!("10. ASAN bilan tekshiring");

    println!("\nXAVFSIZLIK kafolati namunasi:");
    println!("// XAVFSIZLIK: ptr null emas (yuqorida tekshirildi)");
    println!("// XAVFSIZLIK: uzunlik slice chegarasidan chiqmaydi");
    println!("// XAVFSIZLIK: T: Send implementatsiyasi tekshirildi");
}

fn main() {

    println!("=== 1. RAW POINTER ===");
    raw_pointer_misoli();

    println!("\n=== 2. UNSAFE FUNKSIYA ===");
    stdlib_unsafe_misoli();

    println!("\n=== 3. MUTABLE STATIC ===");
    mutable_static_misoli();

    println!("\n=== 4. UNSAFE TRAIT ===");
    unsafe_trait_misoli();

    println!("\n=== 5. UNION ===");
    union_misoli();

    println!("\n=== XAVFSIZ ABSTRAKTSIYA ===");
    xavfsiz_abstraktsiya_misoli();

    println!("\n=== QOIDALAR ===");
    unsafe_qoidalari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                               | Описание (RU)                                            |
// #================================================================================================================================================#
// # |                                        5 SUPERPOWER                                                                                          |
// #================================================================================================================================================#
// # |   1 | *ptr dereference                | Raw pointer orqali o'qish/yozish          | Чтение/запись через сырой указатель                      |
// # |   2 | unsafe fn qo'ng'iroq            | Xavfli funksiya chaqirish                 | Вызов опасной функции                                    |
// # |   3 | static mut kirish               | Global mutable o'zgaruvchi                | Глобальная изменяемая переменная                         |
// # |   4 | unsafe trait impl               | Implementor xavfsizlikni kafolatlaydi     | Implementor гарантирует безопасность                     |
// # |   5 | union maydon                    | Xotira ulashish                           | Совместное использование памяти                          |
// #================================================================================================================================================#
// # |                                        QOIDALAR                                                                                              |
// #================================================================================================================================================#
// # |   6 | Kichik unsafe blok              | Faqat zarur joyda                         | Только там где нужно                                     |
// # |   7 | XAVFSIZLIK kafolati izoh        | Nima uchun xavfsiz ekanligini yozing      | Написать почему это безопасно                            |
// # |   8 | Xavfsiz tashqi API              | Unsafe ichkarida, safe tashqarida         | Unsafe внутри, safe снаружи                              |
// # |   9 | Miri + ASAN                     | Tekshirish vositalari                     | Инструменты проверки                                     |
// # |  10 | Invariantlar                    | Pre/post shartlarni aniq belgilang        | Чётко определить pre/post условия                        |
// #================================================================================================================================================#