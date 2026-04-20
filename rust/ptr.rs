// #================================================================================================================================================#
// #                                                                STD::PTR                                                                        #
// #                STD::PTR — RAW POINTER OPERATSIYALARI. READ, WRITE, COPY, SWAP, NULL CHECK. UNSAFE XOTIRA BOSHQARUVI.                           #
// #                STD::PTR — ОПЕРАЦИИ С СЫРЫМИ УКАЗАТЕЛЯМИ. READ, WRITE, COPY, SWAP, NULL CHECK. НЕБЕЗОПАСНОЕ УПРАВЛЕНИЕ ПАМЯТЬЮ.                 #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::ptr;
use std::alloc::{alloc, dealloc, Layout};
use std::fmt;

// std::ptr nima:
// Что такое std::ptr:
//
//   Raw pointer — xotiraga to'g'ridan-to'g'ri kirish
//   Raw pointer — прямой доступ к памяти
//
//   *const T — o'zgarmas raw pointer (faqat o'qish)
//   *const T — неизменяемый сырой указатель (только чтение)
//   *mut T   — o'zgaruvchi raw pointer (o'qish va yozish)
//   *mut T   — изменяемый сырой указатель (чтение и запись)
//
//   Xavfsiz operatsiyalar (unsafe shart emas):
//   Безопасные операции (unsafe не нужен):
//     ptr::null()      — null *const T
//     ptr::null_mut()  — null *mut T
//     ptr.is_null()    — null tekshiruvi
//     ptr::addr_of!()  — reference olmay manzil olish
//     ptr::addr_of_mut!() — mut reference olmay manzil olish
//     ptr::eq(a, b)    — manzil tengligini tekshirish
//
//   Xavfli operatsiyalar (unsafe kerak):
//   Небезопасные операции (нужен unsafe):
//     ptr.read()       — o'qish
//     ptr.write(val)   — yozish
//     ptr.read_unaligned() — hizalanmagan o'qish
//     ptr.write_unaligned()— hizalanmagan yozish
//     ptr.add(n)       — n element oldinga
//     ptr.sub(n)       — n element orqaga
//     ptr.offset(n)    — n element (manfiy ham mumkin)
//     ptr.copy_to()    — xotirani nusxalash
//     ptr.swap()       — almashtirish
//     ptr::drop_in_place() — joyida drop

fn ptr_asosiy_misollari() {

    // Null pointer — bo'sh ko'rsatkich
    // Null pointer — пустой указатель
    let null_const: *const i32 = ptr::null();
    let null_mut: *mut i32 = ptr::null_mut();

    println!("null is_null: {}", null_const.is_null());
    println!("null_mut is_null: {}", null_mut.is_null());
    // null is_null: true
    // null_mut is_null: true

    // Reference dan raw pointer olish
    // Получение сырого указателя из ссылки
    let n: i32 = 42;
    let ptr_const: *const i32 = &n as *const i32;
    println!("ptr is_null: {}", ptr_const.is_null()); // false

    let mut m: i32 = 100;
    let ptr_mut: *mut i32 = &mut m as *mut i32;
    println!("ptr_mut is_null: {}", ptr_mut.is_null()); // false
    // false
    // false

    // ptr::addr_of! — reference olmay manzil olish
    // ptr::addr_of! — получение адреса без создания ссылки
    // Bu xavfsizroq — aligned/unaligned muammolarsiz
    // Это безопаснее — без проблем aligned/unaligned
    #[repr(packed)]
    struct Packed { a: u8, b: u32 }

    let packed = Packed { a: 1, b: 42 };
    let b_ptr: *const u32 = ptr::addr_of!(packed.b);
    // &packed.b — xavfli (packed struct da unaligned reference)
    // &packed.b — опасно (unaligned reference в packed struct)
    // ptr::addr_of!(packed.b) — xavfsiz
    // ptr::addr_of!(packed.b) — безопасно

    unsafe {
        println!("packed.b: {}", ptr::read_unaligned(b_ptr));
    }
    // packed.b: 42

    // ptr::eq — pointer manzillarini solishtirish
    // ptr::eq — сравнение адресов указателей
    let x = 10i32;
    let y = 10i32;
    let px: *const i32 = &x;
    let py: *const i32 = &y;
    let px2: *const i32 = &x;

    println!("px == px2 (bir xil manzil): {}", ptr::eq(px, px2)); // true
    println!("px == py (turli manzil):    {}", ptr::eq(px, py));   // false
    // px == px2 (bir xil manzil): true
    // px == py (turli manzil):    false

    // Manzilni son sifatida ko'rish
    // Просмотр адреса как числа
    let val = 42i32;
    let ptr = &val as *const i32;
    println!("Manzil: {:p}", ptr);    // 0x... (hex manzil)
    println!("Manzil (son): {}", ptr as usize);
}

fn ptr_read_write_misollari() {

    // ptr.read() — qiymatni o'qish
    // ptr.read() — чтение значения
    let qiymat = 42i32;
    let ptr: *const i32 = &qiymat;

    let o_qilgan: i32 = unsafe { ptr.read() };
    println!("{}", o_qilgan); // 42

    // ptr.write() — qiymatni yozish
    // ptr.write() — запись значения
    let mut hedef = 0i32;
    let ptr_mut: *mut i32 = &mut hedef;

    unsafe { ptr_mut.write(99); }
    println!("{}", hedef); // 99
    // 42
    // 99

    // ptr.read_volatile() / ptr.write_volatile()
    // Kompilyator optimizatsiyasini oldini olish (hardware register uchun)
    // Предотвращение оптимизации компилятора (для аппаратных регистров)
    let mut reg = 0u32;
    let reg_ptr: *mut u32 = &mut reg;
    unsafe {
        reg_ptr.write_volatile(0xFF);
        let v = reg_ptr.read_volatile();
        println!("Volatile: {:#010b}", v); // Volatile: 0b11111111
    }

    // ptr.replace() — eski qiymatni qaytarib yangisin yozish
    // ptr.replace() — вернуть старое значение и записать новое
    let mut data = 10i32;
    let data_ptr: *mut i32 = &mut data;
    let eski: i32 = unsafe { data_ptr.replace(20) };
    println!("Eski: {}, Yangi: {}", eski, data); // Eski: 10, Yangi: 20
    // Eski: 10, Yangi: 20

    // ptr.swap() — ikki pointer qiymatini almashtirish
    // ptr.swap() — обмен значениями двух указателей
    let mut a = 100i32;
    let mut b = 200i32;
    let pa: *mut i32 = &mut a;
    let pb: *mut i32 = &mut b;
    unsafe { pa.swap(pb); }
    println!("a={}, b={}", a, b); // a=200, b=100
    // a=200, b=100

    // ptr::swap_nonoverlapping() — nooverlay garantiya bilan
    // ptr::swap_nonoverlapping() — с гарантией неперекрытия
    let mut v1 = [1i32, 2, 3];
    let mut v2 = [4i32, 5, 6];
    unsafe {
        ptr::swap_nonoverlapping(v1.as_mut_ptr(), v2.as_mut_ptr(), 3);
    }
    println!("{:?} {:?}", v1, v2); // [4, 5, 6] [1, 2, 3]
    // [4, 5, 6] [1, 2, 3]
}

fn ptr_arifmetika_misollari() {

    let massiv: [i32; 5] = [10, 20, 30, 40, 50];
    let ptr: *const i32 = massiv.as_ptr();

    unsafe {
        // ptr.add(n) — n element oldinga
        // ptr.add(n) — n элементов вперёд
        println!("{}", *ptr);             // 10 — birinchi
        println!("{}", *ptr.add(1));      // 20 — ikkinchi
        println!("{}", *ptr.add(4));      // 50 — beshinchi

        // ptr.offset(n) — manfiy ham bo'lishi mumkin
        // ptr.offset(n) — может быть отрицательным
        let oxir = ptr.add(4);
        println!("{}", *oxir.offset(-2)); // 30 — 5-dan 2 orqaga

        // ptr.sub(n) — n element orqaga
        println!("{}", *oxir.sub(3));     // 20
    }
    // 10
    // 20
    // 50
    // 30
    // 20

    // Massiv elementlarini pointer bilan iteratsiya
    // Итерация элементов массива через указатель
    let ma_lumot: [f64; 4] = [1.1, 2.2, 3.3, 4.4];
    let ptr_f64: *const f64 = ma_lumot.as_ptr();

    let mut yig_indi = 0.0f64;
    unsafe {
        for i in 0..ma_lumot.len() {
            yig_indi += *ptr_f64.add(i);
        }
    }
    println!("Yig'indi: {:.1}", yig_indi); // 11.0
    // Yig'indi: 11.0

    // ptr.wrapping_add — overflow bo'lmasligi kafolatlangan (wraparound)
    // ptr.wrapping_add — гарантия без overflow (wraparound)
    let base: *const u8 = ptr::null::<u8>().wrapping_add(0xFF);
    println!("Wrapping ptr: {:p}", base);

    // offset_from — ikki pointer orasidagi farq
    // offset_from — разница между двумя указателями
    let v = [1i32, 2, 3, 4, 5];
    let start = v.as_ptr();
    let end = unsafe { start.add(5) };
    let farq: isize = unsafe { end.offset_from(start) };
    println!("Farq: {}", farq); // 5
    // Farq: 5
}

fn ptr_copy_misollari() {

    // ptr::copy — xotirani nusxalash (overlap bo'lishi mumkin)
    // ptr::copy — копирование памяти (может перекрываться)
    // memmove ga o'xshash
    // Похоже на memmove
    let manba: [i32; 5] = [1, 2, 3, 4, 5];
    let mut hedef: [i32; 5] = [0; 5];

    unsafe {
        ptr::copy(manba.as_ptr(), hedef.as_mut_ptr(), 5);
    }
    println!("{:?}", hedef); // [1, 2, 3, 4, 5]
    // [1, 2, 3, 4, 5]

    // ptr::copy_nonoverlapping — overlap yo'q kafolati (memcpy)
    // ptr::copy_nonoverlapping — гарантия без перекрытия (memcpy)
    let mut v1: [u8; 4] = [10, 20, 30, 40];
    let mut v2: [u8; 4] = [0; 4];
    unsafe {
        ptr::copy_nonoverlapping(v1.as_ptr(), v2.as_mut_ptr(), 4);
    }
    println!("{:?}", v2); // [10, 20, 30, 40]
    // [10, 20, 30, 40]

    // Overlap bilan nusxalash — ptr::copy
    // Копирование с перекрытием — ptr::copy
    let mut v3: [i32; 8] = [1, 2, 3, 4, 5, 0, 0, 0];
    unsafe {
        let src = v3.as_ptr().add(2); // &v3[2] = 3
        let dst = v3.as_mut_ptr().add(5); // &v3[5]
        ptr::copy(src, dst, 3); // [3, 4, 5] → [5..8]
    }
    println!("{:?}", v3); // [1, 2, 3, 4, 5, 3, 4, 5]
    // [1, 2, 3, 4, 5, 3, 4, 5]

    // ptr.copy_to() — method syntax
    // ptr.copy_to() — синтаксис метода
    let manba2: [i32; 3] = [100, 200, 300];
    let mut hedef2: [i32; 3] = [0; 3];
    unsafe {
        manba2.as_ptr().copy_to(hedef2.as_mut_ptr(), 3);
    }
    println!("{:?}", hedef2); // [100, 200, 300]

    // ptr::write_bytes — xotirani bir qiymat bilan to'ldirish (memset)
    // ptr::write_bytes — заполнение памяти одним значением (memset)
    let mut buf: [u8; 8] = [0xFF; 8];
    unsafe {
        ptr::write_bytes(buf.as_mut_ptr(), 0x00, 4); // dastlabki 4 ni nollash
    }
    println!("{:?}", buf); // [0, 0, 0, 0, 255, 255, 255, 255]
    // [0, 0, 0, 0, 255, 255, 255, 255]
}

fn drop_in_place_misoli() {

    // ptr::drop_in_place — xotira bo'shatmasdan drop chaqirish
    // ptr::drop_in_place — вызов drop без освобождения памяти
    // ManuallyDrop bilan birga ishlatiladi
    // Используется вместе с ManuallyDrop
    use std::mem::ManuallyDrop;

    struct Resurs { nomi: String }
    impl Drop for Resurs {
        fn drop(&mut self) { println!("'{}' tushirildi", self.nomi); }
    }

    let mut md: ManuallyDrop<Resurs> = ManuallyDrop::new(
        Resurs { nomi: "muhim_resurs".to_string() }
    );

    println!("Resurs: {}", md.nomi);

    unsafe {
        // Drop chaqirish — xotira bo'shatilmaydi
        // Вызов drop — память не освобождается
        ptr::drop_in_place(&mut *md as *mut Resurs);
    }
    println!("drop_in_place chaqirildi");
    // Resurs: muhim_resurs
    // 'muhim_resurs' tushirildi
    // drop_in_place chaqirildi
}

fn custom_allocator_misoli() {

    // Layout — xotira joylashuvi
    // Layout — расположение памяти
    let layout = Layout::new::<i32>();
    println!("i32 size: {}, align: {}", layout.size(), layout.align());
    // i32 size: 4, align: 4

    let layout_arr = Layout::array::<i32>(5).unwrap();
    println!("[i32; 5] size: {}", layout_arr.size());
    // [i32; 5] size: 20

    unsafe {
        // alloc — xotira ajratish
        // alloc — выделение памяти
        let ptr: *mut u8 = alloc(layout_arr);
        if ptr.is_null() {
            panic!("Xotira ajratish muvaffaqiyatsiz!");
        }
        let arr_ptr = ptr as *mut i32;

        // Qiymatlar yozish
        for i in 0..5 {
            arr_ptr.add(i).write(i as i32 * 10);
        }

        // Qiymatlar o'qish
        for i in 0..5 {
            print!("{} ", arr_ptr.add(i).read());
        }
        println!();

        // dealloc — xotirani qaytarish
        // dealloc — возврат памяти
        dealloc(ptr, layout_arr);
        println!("Xotira bo'shatildi");
    }
    // 0 10 20 30 40
    // Xotira bo'shatildi
}

// Sodda Vec implementatsiyasi — raw pointer bilan
// Простая реализация Vec — с сырым указателем
struct SodaVec<T> {
    ptr: *mut T,
    uzunlik: usize,
    sig_im: usize,
}

impl<T> SodaVec<T> {
    fn new() -> Self {
        SodaVec {
            ptr: ptr::null_mut(),
            uzunlik: 0,
            sig_im: 0,
        }
    }

    fn push(&mut self, qiymat: T) {
        if self.uzunlik == self.sig_im {
            let yangi_sig_im = if self.sig_im == 0 { 4 } else { self.sig_im * 2 };
            let yangi_layout = Layout::array::<T>(yangi_sig_im).unwrap();

            let yangi_ptr = unsafe {
                if self.sig_im == 0 {
                    alloc(yangi_layout) as *mut T
                } else {
                    let eski_layout = Layout::array::<T>(self.sig_im).unwrap();
                    std::alloc::realloc(self.ptr as *mut u8, eski_layout, yangi_layout.size()) as *mut T
                }
            };

            if yangi_ptr.is_null() { panic!("Xotira ajratish xatosi!"); }
            self.ptr = yangi_ptr;
            self.sig_im = yangi_sig_im;
        }

        unsafe { self.ptr.add(self.uzunlik).write(qiymat); }
        self.uzunlik += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.uzunlik == 0 { return None; }
        self.uzunlik -= 1;
        Some(unsafe { self.ptr.add(self.uzunlik).read() })
    }

    fn ol(&self, indeks: usize) -> Option<&T> {
        if indeks >= self.uzunlik { return None; }
        Some(unsafe { &*self.ptr.add(indeks) })
    }

    fn uzunlik(&self) -> usize { self.uzunlik }
}

impl<T: fmt::Debug> fmt::Debug for SodaVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.uzunlik {
            if i > 0 { write!(f, ", ")?; }
            let v = unsafe { &*self.ptr.add(i) };
            write!(f, "{:?}", v)?;
        }
        write!(f, "]")
    }
}

impl<T> Drop for SodaVec<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() && self.sig_im > 0 {
            unsafe {
                // Har elementni drop qilish
                for i in 0..self.uzunlik {
                    ptr::drop_in_place(self.ptr.add(i));
                }
                // Xotirani qaytarish
                let layout = Layout::array::<T>(self.sig_im).unwrap();
                dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}

fn soda_vec_misoli() {

    let mut v: SodaVec<i32> = SodaVec::new();
    println!("Boshlang'ich: {:?}", v);

    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    v.push(50); // sig'im 2x bo'ladi

    println!("5 element: {:?}", v);
    println!("Uzunlik: {}", v.uzunlik());
    println!("v[2] = {:?}", v.ol(2));

    println!("Pop: {:?}", v.pop());
    println!("Pop: {:?}", v.pop());
    println!("Keyin: {:?}", v);
    // Boshlang'ich: []
    // 5 element: [10, 20, 30, 40, 50]
    // Uzunlik: 5
    // v[2] = Some(30)
    // Pop: Some(50)
    // Pop: Some(40)
    // Keyin: [10, 20, 30]
}

fn main() {

    println!("=== PTR ASOSIY ===");
    ptr_asosiy_misollari();

    println!("\n=== READ VA WRITE ===");
    ptr_read_write_misollari();

    println!("\n=== PTR ARIFMETIKA ===");
    ptr_arifmetika_misollari();

    println!("\n=== COPY ===");
    ptr_copy_misollari();

    println!("\n=== DROP_IN_PLACE ===");
    drop_in_place_misoli();

    println!("\n=== CUSTOM ALLOCATOR ===");
    custom_allocator_misoli();

    println!("\n=== SODDA VEC ===");
    soda_vec_misoli();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        XAVFSIZ OPERATSIYALAR                                                                                 |
// #================================================================================================================================================#
// # |   1 | ptr::null()                     | Null *const T                              | Null *const T                                           |
// # |   2 | ptr::null_mut()                 | Null *mut T                                | Null *mut T                                             |
// # |   3 | ptr.is_null()                   | Null tekshiruvi                            | Проверка на null                                        |
// # |   4 | ptr::addr_of!(val)              | Reference olmay manzil                     | Адрес без создания ссылки                               |
// # |   5 | ptr::addr_of_mut!(val)          | Mut reference olmay manzil                 | Адрес без создания mut ссылки                           |
// # |   6 | ptr::eq(a, b)                   | Manzil tengligini tekshirish               | Сравнение адресов                                       |
// #================================================================================================================================================#
// # |                                        XAVFLI OPERATSIYALAR (unsafe)                                                                         |
// #================================================================================================================================================#
// # |   7 | ptr.read()                      | Qiymatni o'qish                            | Чтение значения                                         |
// # |   8 | ptr.write(val)                  | Qiymatni yozish                            | Запись значения                                         |
// # |   9 | ptr.read_unaligned()            | Hizalanmagan o'qish                        | Чтение без выравнивания                                 |
// # |  10 | ptr.write_unaligned(val)        | Hizalanmagan yozish                        | Запись без выравнивания                                 |
// # |  11 | ptr.read_volatile()             | Optimizatsiyasiz o'qish                    | Чтение без оптимизации                                  |
// # |  12 | ptr.replace(val)                | Eski qaytarib yangi yozish                 | Вернуть старое, записать новое                          |
// # |  13 | ptr.swap(other)                 | Ikkisini almashtirish                      | Обмен двух значений                                     |
// # |  14 | ptr.add(n)                      | n element oldinga                          | n элементов вперёд                                      |
// # |  15 | ptr.sub(n)                      | n element orqaga                           | n элементов назад                                       |
// # |  16 | ptr.offset(n)                   | n element (manfiy ham)                     | n элементов (может быть отрицательным)                  |
// # |  17 | ptr.offset_from(other)          | Pointer farqini topish                     | Нахождение разницы указателей                           |
// # |  18 | ptr::copy(src, dst, n)          | xotira nusxalash (memmove)                 | Копирование памяти (memmove)                            |
// # |  19 | ptr::copy_nonoverlapping(s,d,n) | xotira nusxalash (memcpy)                  | Копирование памяти (memcpy)                             |
// # |  20 | ptr::write_bytes(ptr, val, n)   | xotirani to'ldirish (memset)               | Заполнение памяти (memset)                              |
// # |  21 | ptr::drop_in_place(ptr)         | Joyida drop, xotira bo'shatmasdan          | Drop на месте без освобождения памяти                   |
// # |  22 | ptr.wrapping_add(n)             | Overflow bo'lmay oldinga                   | Вперёд без overflow                                     |
// #================================================================================================================================================#