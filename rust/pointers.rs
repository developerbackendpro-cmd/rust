// #================================================================================================================================================#
// #                                                                    RAW POINTERS                                                                #
// #                    RAW POINTERS — *CONST T VA *MUT T. UNSAFE XOTIRA. DANGLING, ALIASING, ALIGNMENT. QACHON VA NIMA UCHUN.                      #
// #                    RAW POINTERS — *CONST T И *MUT T. НЕБЕЗОПАСНАЯ ПАМЯТЬ. DANGLING, ALIASING, ALIGNMENT. КОГДА И ЗАЧЕМ.                        #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::ptr;
use std::fmt;
use std::mem;

// Raw pointer nima:
// Что такое raw pointer:
//
//   *const T — o'zgarmas raw pointer
//   *const T — неизменяемый сырой указатель
//   *mut T   — o'zgaruvchi raw pointer
//   *mut T   — изменяемый сырой указатель
//
//   Reference bilan farqi:
//   Отличие от ссылки:
//   ✅ Null bo'lishi mumkin     — может быть null
//   ✅ Lifetime yo'q           — нет lifetime
//   ✅ Borrow checker tekshirmaydi — borrow checker не проверяет
//   ✅ Bir vaqtda ko'p mut ptr — несколько mut ptr одновременно
//   ❌ Dereference — unsafe   — разыменование — unsafe
//   ❌ Xotira xavfsizligi yo'q — нет гарантии безопасности памяти
//
//   Qachon kerak:
//   Когда нужен:
//   - FFI (C kutubxonalari bilan)
//   - Custom allocator/collection
//   - Self-referential struct
//   - Yuqori unumdorlik talab qilinadigan joylar
//   - Unsafe abstraktsiya qurish

fn raw_ptr_yaratish_misollari() {

    // 1. Reference dan raw pointer
    // 1. Из ссылки в raw pointer
    let n: i32 = 42;
    let r: &i32 = &n;
    let ptr_const: *const i32 = r as *const i32;
    let ptr_const2: *const i32 = &n as *const i32;

    let mut m: i32 = 100;
    let ptr_mut: *mut i32 = &mut m as *mut i32;

    println!("ptr_const null: {}", ptr_const.is_null()); // false
    println!("ptr_mut null: {}", ptr_mut.is_null());     // false
    // false
    // false

    // 2. Box dan raw pointer
    // 2. Из Box в raw pointer
    let boxed: Box<i32> = Box::new(99);
    let ptr_box: *const i32 = &*boxed as *const i32;
    let owned: Box<i32> = Box::new(55);
    let raw_owned: *mut i32 = Box::into_raw(owned); // ownership berish

    unsafe {
        println!("Box ptr: {}", *ptr_box);   // 99
        println!("Owned ptr: {}", *raw_owned); // 55
        // Qaytarish — xotirani tozalash
        drop(Box::from_raw(raw_owned));
    }
    // Box ptr: 99
    // Owned ptr: 55

    // 3. Null pointer
    // 3. Null pointer
    let null_ptr: *const i32 = ptr::null();
    let null_mut: *mut i32 = ptr::null_mut();
    println!("null: {}", null_ptr.is_null());     // true
    println!("null_mut: {}", null_mut.is_null()); // true
    // true
    // true

    // 4. Vec dan raw pointer
    // 4. Из Vec в raw pointer
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let ptr_v: *mut i32 = v.as_mut_ptr();
    unsafe {
        for i in 0..5 {
            *ptr_v.add(i) *= 2;
        }
    }
    println!("{:?}", v); // [2, 4, 6, 8, 10]
    // [2, 4, 6, 8, 10]

    // 5. String dan raw pointer
    // 5. Из String в raw pointer
    let mut s = String::from("salom");
    let ptr_s: *mut u8 = s.as_mut_ptr();
    unsafe {
        // Birinchi harfni 'S' ga o'zgartirish
        *ptr_s = b'S';
    }
    println!("{}", s); // Salom
    // Salom

    // 6. Struct fieldiga to'g'ridan-to'g'ri pointer
    // 6. Указатель непосредственно на поле структуры
    struct Nuqta { x: f64, y: f64 }
    let mut p = Nuqta { x: 1.0, y: 2.0 };
    let x_ptr: *mut f64 = ptr::addr_of_mut!(p.x);
    unsafe { *x_ptr = 3.14; }
    println!("p.x = {}", p.x); // 3.14
    // p.x = 3.14
}

fn dereference_misollari() {

    let mut qiymat: i32 = 42;
    let ptr: *mut i32 = &mut qiymat;

    // *ptr — dereference (unsafe kerak)
    // *ptr — разыменование (нужен unsafe)
    unsafe {
        println!("{}", *ptr);     // 42 — o'qish
        *ptr = 100;               // yozish
        println!("{}", *ptr);     // 100
        println!("{}", qiymat);   // 100
    }
    // 42
    // 100
    // 100

    // *const — yozish mumkin emas
    // *const — нельзя писать
    let ptr_const: *const i32 = &qiymat;
    unsafe {
        println!("{}", *ptr_const); // faqat o'qish
        // *ptr_const = 200;        // ← xato! const pointer
    }
    // 100

    // Null pointer ni dereference qilmaslik!
    // Никогда не разыменовывать null pointer!
    let null: *const i32 = ptr::null();
    println!("null tekshiruv: {}", if null.is_null() { "null" } else { "not null" });
    // Quyidagini hech qachon qilmang:
    // Никогда не делайте следующее:
    // unsafe { *null; }  // ← UNDEFINED BEHAVIOR!

    // Xavfsiz null tekshiruv pattern
    // Безопасный паттерн проверки на null
    fn xavfsiz_dereference<T: fmt::Debug>(ptr: *const T) -> Option<&'static T> {
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(&*ptr) }
        }
    }

    let val = 77i32;
    let ptr2: *const i32 = &val;
    let null2: *const i32 = ptr::null();

    // XAVF: xavfsiz_dereference static lifetime qaytaradi — real kodda ishlatmang
    // ОПАСНО: xavfsiz_dereference возвращает static lifetime — не используйте в реальном коде
    // Bu faqat pattern ko'rsatish uchun
    // Это только для демонстрации паттерна
    println!("null: {:?}", null2.is_null());  // true
    println!("ptr2: {:?}", unsafe { *ptr2 }); // 77
    // true
    // 77
}

// Aliasing — bir xil xotira manzilini ko'rsatuvchi ko'p pointer
// Aliasing — несколько указателей на одну область памяти
// Rust reference da taqiqlangan, raw ptr da mumkin (lekin xavfli)
// В ссылках Rust запрещено, в raw ptr разрешено (но опасно)

fn aliasing_misoli() {

    let mut qiymat: i32 = 10;

    // Bir vaqtda ikkita mut raw pointer — ALIASING
    // Одновременно два mut raw pointer — ALIASING
    let p1: *mut i32 = &mut qiymat;
    let p2: *mut i32 = &mut qiymat; // Reference da bu xato bo'lardi!

    unsafe {
        *p1 = 20;
        println!("p1: {}, p2: {}", *p1, *p2); // 20, 20 — bir xil xotira
        *p2 = 30;
        println!("p1: {}, p2: {}", *p1, *p2); // 30, 30
    }
    // 20, 20
    // 30, 30

    // split_at_mut simulyatsiyasi — xavfsiz aliasing
    // Симуляция split_at_mut — безопасный aliasing
    let mut data: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let len = data.len();

    // Bu kod std::slice::split_at_mut kabi ishlaydi
    // Этот код работает как std::slice::split_at_mut
    let (chap, ong): (&mut [i32], &mut [i32]) = {
        let ptr = data.as_mut_ptr();
        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, len / 2),
                std::slice::from_raw_parts_mut(ptr.add(len / 2), len - len / 2),
            )
        }
    };

    for v in chap.iter_mut() { *v *= 2; }
    for v in ong.iter_mut() { *v *= 3; }
    println!("{:?}", data); // [2, 4, 6, 12, 15, 18]
    // [2, 4, 6, 12, 15, 18]
}

fn alignment_misollari() {

    // Har tur o'zining alignment talabiga ega
    // Каждый тип имеет своё требование выравнивания
    println!("u8  align: {}", mem::align_of::<u8>());   // 1
    println!("u16 align: {}", mem::align_of::<u16>());  // 2
    println!("u32 align: {}", mem::align_of::<u32>());  // 4
    println!("u64 align: {}", mem::align_of::<u64>());  // 8
    println!("f64 align: {}", mem::align_of::<f64>());  // 8
    // u8  align: 1
    // u16 align: 2
    // u32 align: 4
    // u64 align: 8
    // f64 align: 8

    // Hizalanmagan pointer — undefined behavior!
    // Невыровненный указатель — undefined behavior!
    // read_unaligned/write_unaligned — xavfsiz yechim
    // read_unaligned/write_unaligned — безопасное решение

    let bytes: [u8; 8] = [0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00];
    let ptr = bytes.as_ptr();

    // Hizalanmagan o'qish — read_unaligned
    // Невыровненное чтение — read_unaligned
    unsafe {
        let v1: u32 = ptr::read_unaligned(ptr as *const u32);
        let v2: u32 = ptr::read_unaligned(ptr.add(4) as *const u32);
        println!("v1={}, v2={}", v1, v2); // v1=1, v2=2 (little endian)
    }
    // v1=1, v2=2

    // Alignment tekshiruvi
    // Проверка выравнивания
    fn aligned_mi<T>(ptr: *const T) -> bool {
        let align = mem::align_of::<T>();
        (ptr as usize) % align == 0
    }

    let n = 42u32;
    let ptr_n = &n as *const u32;
    println!("Aligned: {}", aligned_mi(ptr_n)); // true (stack aligned)
    // true

    // from_raw_parts — slice yaratish
    // from_raw_parts — создание среза
    let v: Vec<i32> = vec![10, 20, 30, 40, 50];
    let slice: &[i32] = unsafe {
        std::slice::from_raw_parts(v.as_ptr(), v.len())
    };
    println!("{:?}", slice); // [10, 20, 30, 40, 50]
    // [10, 20, 30, 40, 50]
}

// Dangling pointer — scope tugagach yaroqsiz bo'lgan pointer
// Dangling pointer — указатель, ставший недействительным после окончания области видимости

fn dangling_pointer_tushuntirish() {

    // XAVFLI KOD — dangling pointer
    // ОПАСНЫЙ КОД — dangling pointer
    // Bu misol hech qachon ishlatilmasligi kerak!
    // Этот пример никогда не должен использоваться!
    /*
    fn dangling() -> *const i32 {
        let n = 42i32;
        &n as *const i32  // ← n scope tugaydi! pointer yaroqsiz!
    }
    let ptr = dangling();
    unsafe { *ptr }  // ← UNDEFINED BEHAVIOR! dangling!
    */

    // XAVFSIZ: Box orqali — xotira heap da saqlanadi
    // БЕЗОПАСНО: через Box — память хранится в куче
    fn xavfsiz_ptr() -> *mut i32 {
        let b = Box::new(42i32);
        Box::into_raw(b) // ownership beriladi, drop bo'lmaydi
    }

    let ptr = xavfsiz_ptr();
    unsafe {
        println!("Xavfsiz ptr: {}", *ptr);
        // Xotirani tozalash — bu shart!
        drop(Box::from_raw(ptr));
    }
    println!("Tozalandi");
    // Xavfsiz ptr: 42
    // Tozalandi

    // XAVFSIZ: Arc orqali — reference counting
    // БЕЗОПАСНО: через Arc — подсчёт ссылок
    use std::sync::Arc;
    let arc = Arc::new(99i32);
    let ptr2: *const i32 = Arc::as_ptr(&arc);

    // arc yashayotgan vaqtda ptr xavfsiz
    // ptr безопасен пока живёт arc
    unsafe { println!("Arc ptr: {}", *ptr2); }
    // Arc ptr: 99

    // Dangling pointer xavfini ko'rsatish (faqat tushuntirish)
    // Демонстрация опасности dangling pointer (только объяснение)
    println!("\nDangling pointer qoidalari:");
    println!("1. Stack variable dan raw ptr qaytarmang");
    println!("2. Drop bo'lgan qiymatni ko'rsatuvchi ptr ishlatmang");
    println!("3. Vec reallocsiyasidan keyin ptr yaroqsiz bo'ladi");
    println!("4. Box::into_raw → Box::from_raw bilan xotirani boshqaring");
}

// C ABI dan funksiyalar simulyatsiyasi
// Симуляция функций из C ABI
#[unsafe(no_mangle)]
pub extern "C" fn rust_qo_shish(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_string_uzunlik(ptr: *const u8, len: usize) -> usize {
    if ptr.is_null() { return 0; }
    len
}

// C struct simulyatsiyasi
// Симуляция C структуры
#[repr(C)]
#[derive(Debug)]
struct CPoint {
    x: f64,
    y: f64,
}

// C kutubxona funksiyasi simulyatsiyasi
// Симуляция функции C библиотеки
unsafe fn c_nuqta_yaratish(x: f64, y: f64, out: *mut CPoint) {
    if !out.is_null() {
        unsafe {
            (*out).x = x;
            (*out).y = y;
        }
    }
}

unsafe fn c_nuqtalar_yig_indisi(pts: *const CPoint, n: usize) -> f64 {
    if pts.is_null() || n == 0 { return 0.0; }
    let mut yig = 0.0;
    for i in 0..n {
        unsafe {
            let p = &*pts.add(i);
            yig += (p.x * p.x + p.y * p.y).sqrt();
        }
    }
    yig
}

fn ffi_misollari() {

    // Rust → C ga ma'lumot uzatish
    // Передача данных из Rust → C
    let mut nuqta = CPoint { x: 0.0, y: 0.0 };
    unsafe {
        c_nuqta_yaratish(3.0, 4.0, &mut nuqta as *mut CPoint);
    }
    println!("{:?}", nuqta);
    // CPoint { x: 3.0, y: 4.0 }

    // Array — C ga uzatish
    // Массив — передача в C
    let nuqtalar: Vec<CPoint> = vec![
        CPoint { x: 3.0, y: 4.0 },  // masofa: 5
        CPoint { x: 6.0, y: 8.0 },  // masofa: 10
        CPoint { x: 0.0, y: 1.0 },  // masofa: 1
    ];

    let yig = unsafe {
        c_nuqtalar_yig_indisi(nuqtalar.as_ptr(), nuqtalar.len())
    };
    println!("Masofalar yig'indisi: {}", yig); // 16.0
    // Masofalar yig'indisi: 16

    // String — C ga uzatish (*const c_char simulyatsiya)
    // String — передача в C (*const c_char симуляция)
    let matn = "Salom C kutubxona!\0"; // null-terminated
    let c_str_ptr: *const u8 = matn.as_ptr();
    let uzunlik = unsafe { rust_string_uzunlik(c_str_ptr, matn.len() - 1) };
    println!("C string uzunlik: {}", uzunlik);
    // C string uzunlik: 18

    // Box::into_raw — C ga ownership berish
    // Box::into_raw — передача владения в C
    let data = Box::new(CPoint { x: 1.0, y: 2.0 });
    let raw_ptr: *mut CPoint = Box::into_raw(data);

    // C "ishlatadi" ...
    // C "использует" ...
    unsafe {
        println!("C oldi: {:?}", *raw_ptr);
        // C qaytaradi — biz tozalaymiz
        drop(Box::from_raw(raw_ptr));
    }
    // C oldi: CPoint { x: 1.0, y: 2.0 }
}

// Intrusive linked list — raw pointer bilan
// Интрузивный связный список — с raw pointer
struct Tugun<T> {
    qiymat: T,
    keyingi: *mut Tugun<T>,
}

struct RoyhhatList<T> {
    bosh: *mut Tugun<T>,
    uzunlik: usize,
}

impl<T: fmt::Debug> RoyhhatList<T> {
    fn new() -> Self {
        RoyhhatList { bosh: ptr::null_mut(), uzunlik: 0 }
    }

    fn boshiga_qo_sh(&mut self, qiymat: T) {
        let yangi = Box::into_raw(Box::new(Tugun {
            qiymat,
            keyingi: self.bosh,
        }));
        self.bosh = yangi;
        self.uzunlik += 1;
    }

    fn boshini_ol(&mut self) -> Option<T> {
        if self.bosh.is_null() { return None; }
        unsafe {
            let tugun = Box::from_raw(self.bosh);
            self.bosh = tugun.keyingi;
            self.uzunlik -= 1;
            Some(tugun.qiymat)
        }
    }

    fn iter(&self) -> Vec<&T> {
        let mut natija = Vec::new();
        let mut joriy = self.bosh;
        while !joriy.is_null() {
            unsafe {
                natija.push(&(*joriy).qiymat);
                joriy = (*joriy).keyingi;
            }
        }
        natija
    }
}

impl<T> Drop for RoyhhatList<T> {
    fn drop(&mut self) {
        let mut joriy = self.bosh;
        while !joriy.is_null() {
            unsafe {
                let keyingi = (*joriy).keyingi;
                drop(Box::from_raw(joriy));
                joriy = keyingi;
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for RoyhhatList<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut joriy = self.bosh;
        let mut birinchi = true;
        while !joriy.is_null() {
            if !birinchi { write!(f, " → ")?; }
            unsafe { write!(f, "{:?}", (*joriy).qiymat)?; }
            joriy = unsafe { (*joriy).keyingi };
            birinchi = false;
        }
        write!(f, "]")
    }
}

fn linked_list_misoli() {

    let mut list: RoyhhatList<i32> = RoyhhatList::new();

    list.boshiga_qo_sh(1);
    list.boshiga_qo_sh(2);
    list.boshiga_qo_sh(3);
    list.boshiga_qo_sh(4);
    list.boshiga_qo_sh(5);

    println!("List: {:?}", list);
    println!("Uzunlik: {}", list.uzunlik);

    let iter_v: Vec<&i32> = list.iter();
    println!("Iter: {:?}", iter_v);

    println!("Pop: {:?}", list.boshini_ol());
    println!("Pop: {:?}", list.boshini_ol());
    println!("Keyin: {:?}", list);
    // List: [5 → 4 → 3 → 2 → 1]
    // Uzunlik: 5
    // Iter: [5, 4, 3, 2, 1]
    // Pop: Some(5)
    // Pop: Some(4)
    // Keyin: [3 → 2 → 1]
}

fn main() {

    println!("=== RAW PTR YARATISH ===");
    raw_ptr_yaratish_misollari();

    println!("\n=== DEREFERENCE ===");
    dereference_misollari();

    println!("\n=== ALIASING ===");
    aliasing_misoli();

    println!("\n=== ALIGNMENT ===");
    alignment_misollari();

    println!("\n=== DANGLING POINTER ===");
    dangling_pointer_tushuntirish();

    println!("\n=== FFI ===");
    ffi_misollari();

    println!("\n=== LINKED LIST ===");
    linked_list_misoli();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        RAW POINTER TURLARI                                                                                   |
// #================================================================================================================================================#
// # |   1 | *const T                        | O'zgarmas raw pointer                      | Неизменяемый сырой указатель                            |
// # |   2 | *mut T                          | O'zgaruvchi raw pointer                    | Изменяемый сырой указатель                              |
// # |   3 | &T as *const T                  | Reference → raw ptr                        | Ссылка → сырой указатель                                |
// # |   4 | Box::into_raw(b)                | Box → raw ptr (ownership berish)           | Box → сырой указатель (передача владения)               |
// # |   5 | Box::from_raw(ptr)              | raw ptr → Box (ownership qaytarish)        | Сырой указатель → Box (возврат владения)                |
// #================================================================================================================================================#
// # |                                        OPERATSIYALAR (unsafe)                                                                                |
// #================================================================================================================================================#
// # |   6 | *ptr                            | Dereference — o'qish                       | Разыменование — чтение                                  |
// # |   7 | *ptr = val                      | Dereference — yozish (*mut)                | Разыменование — запись (*mut)                           |
// # |   8 | ptr.add(n)                      | n element oldinga                          | n элементов вперёд                                      |
// # |   9 | ptr.sub(n)                      | n element orqaga                           | n элементов назад                                       |
// # |  10 | ptr.is_null()                   | Null tekshiruvi (xavfsiz)                  | Проверка null (безопасно)                               |
// #================================================================================================================================================#
// # |                                        XAVFLI HOLATLAR                                                                                       |
// #================================================================================================================================================#
// # |  11 | Dangling pointer                | Scope tugagan — pointer yaroqsiz           | Область видимости кончилась — указатель недействителен  |
// # |  12 | Aliasing                        | Ko'p mut ptr — bir xotira                  | Несколько mut ptr — одна память                         |
// # |  13 | Unaligned access                | read_unaligned/write_unaligned ishlatish   | Использовать read_unaligned/write_unaligned             |
// # |  14 | Null dereference                | Har doim is_null() tekshiring              | Всегда проверяйте is_null()                             |
// # |  15 | Use after free                  | Box::from_raw dan keyin ptr ishlatmang     | После Box::from_raw не используйте ptr                  |
// #================================================================================================================================================#
// # |                                        PATTERNLAR                                                                                            |
// #================================================================================================================================================#
// # |  16 | Box::into_raw / Box::from_raw   | FFI ownership boshqaruvi                   | Управление владением в FFI                              |
// # |  17 | Vec::as_ptr / as_mut_ptr        | Vec elementlariga raw kirish               | Сырой доступ к элементам Vec                            |
// # |  18 | from_raw_parts                  | Raw ptr + len → &[T]                       | Raw ptr + len → &[T]                                    |
// # |  19 | addr_of! / addr_of_mut!         | Reference olmay manzil                     | Адрес без создания ссылки                               |
// # |  20 | Linked list                     | Raw ptr bilan o'z-o'ziga referens           | Самоссылающиеся структуры с raw ptr                    |
// #================================================================================================================================================#