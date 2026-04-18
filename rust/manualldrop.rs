// #================================================================================================================================================#
// #                                                                MANUALLYDROP                                                                    #
// #                            MANUALLYDROP — DROP NI O'CHIRISH. XOTIRANI QACHON OZOD QILISHNI O'ZI BOSHQARADI.                                    #
// #                            MANUALLYDROP — ОТКЛЮЧЕНИЕ DROP. САМОСТОЯТЕЛЬНОЕ УПРАВЛЕНИЕ ОСВОБОЖДЕНИЕМ ПАМЯТИ.                                    #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::mem::ManuallyDrop;
use std::fmt;

// ManuallyDrop<T> nima:
// Что такое ManuallyDrop<T>:
//
//   - T ni wrapping qiladi, lekin Drop chaqirilmaydi
//   - Оборачивает T, но Drop не вызывается
//   - Xotirani qachon va qanday ozod qilishni dasturchi o'zi hal qiladi
//   - Разработчик сам решает когда и как освободить память
//   - Asosan unsafe kod va FFI da ishlatiladi
//   - Используется в unsafe коде и FFI
//   - zero-cost — runtime xarajat yo'q
//   - zero-cost — нет затрат в runtime
//
// Qachon kerak:
// Когда нужен:
//   1. Unsafe kod — xotirani qo'lda boshqarish
//   2. FFI — C kutubxonasiga ownership o'tkazish
//   3. Union — Drop'siz union fieldlari
//   4. Erta drop — std::mem::drop o'rniga aniqroq nazorat
//   5. Custom allocator — xotira boshqaruvi

fn manuallydrop_asosiy() {

    // ManuallyDrop::new — drop o'chirilgan wrapper
    // ManuallyDrop::new — обёртка с отключённым drop
    let md: ManuallyDrop<String> = ManuallyDrop::new(String::from("salom"));
    println!("{}", *md);
    // salom
    // ← Drop CHAQIRILMAYDI scope tugaganda!
    // ← Drop НЕ ВЫЗЫВАЕТСЯ при выходе из области!

    // ManuallyDrop::into_inner — qiymatni olish (drop tiklanadi)
    // ManuallyDrop::into_inner — получение значения (drop восстанавливается)
    let md2: ManuallyDrop<String> = ManuallyDrop::new(String::from("dunyo"));
    let s: String = ManuallyDrop::into_inner(md2);
    println!("{}", s);
    // dunyo
    // ← s endi oddiy String — scope tugaganda drop chaqiriladi

    // ManuallyDrop::drop — qo'lda drop chaqirish (unsafe)
    // ManuallyDrop::drop — ручной вызов drop (unsafe)
    let mut md3: ManuallyDrop<String> = ManuallyDrop::new(String::from("rust"));
    println!("{}", *md3);
    unsafe {
        ManuallyDrop::drop(&mut md3);
    }
    // ← md3 endi o'chirildi, lekin md3 hali scope da
    // ← md3 теперь удалён, но md3 ещё в области видимости
    // md3 ni endi ishlatmaslik kerak!
    // md3 больше нельзя использовать!
    // rust

    // Deref va DerefMut
    // Deref и DerefMut
    let mut md4: ManuallyDrop<Vec<i32>> = ManuallyDrop::new(vec![1, 2, 3]);
    md4.push(4); // DerefMut orqali
    println!("{:?}", *md4);
    // [1, 2, 3, 4]

    // O'lcham — ManuallyDrop<T> == T
    // Размер — ManuallyDrop<T> == T
    println!("String:              {} bayt", std::mem::size_of::<String>());
    println!("ManuallyDrop<String>: {} bayt", std::mem::size_of::<ManuallyDrop<String>>());
    // String:               24 bayt
    // ManuallyDrop<String>: 24 bayt  ← bir xil!
}

fn manuallydrop_unsafe_misoli() {

    // Unsafe — raw pointer bilan xotirani boshqarish
    // Unsafe — управление памятью через сырой указатель
    struct Resurs {
        qiymat: i32,
    }

    impl Drop for Resurs {
        fn drop(&mut self) {
            println!("Resurs ({}) drop bo'ldi", self.qiymat);
        }
    }

    // ManuallyDrop bilan drop nazorat qilinadi
    // ManuallyDrop управляет drop
    let mut md: ManuallyDrop<Resurs> = ManuallyDrop::new(Resurs { qiymat: 42 });
    println!("Resurs qiymati: {}", md.qiymat);

    // Qo'lda drop — aniq nazorat
    // Ручной drop — явный контроль
    unsafe {
        ManuallyDrop::drop(&mut md);
    }
    println!("Drop bo'lgandan keyin (md hali scope da)");
    // Resurs qiymati: 42
    // Resurs (42) drop bo'ldi
    // Drop bo'lgandan keyin (md hali scope da)

    // Raw pointer orqali yaratish va o'chirish
    // Создание и удаление через сырой указатель
    let ptr: *mut String = Box::into_raw(Box::new(String::from("heap string")));
    unsafe {
        println!("{}", *ptr);
        drop(Box::from_raw(ptr)); // xotirani ozod qilish
    }
    // heap string
}

fn manuallydrop_union_misoli() {

    // Union — fieldlar bir xil xotirani o'rtaqlashadi
    // Union — поля делят одну память
    // Union fieldlari Drop implement qilsa — xavfli
    // Если поля Union реализуют Drop — опасно
    // ManuallyDrop bilan — xavfsiz
    // С ManuallyDrop — безопасно

    union Qiymat {
        butun_son: ManuallyDrop<i64>,
        kasr_son:  ManuallyDrop<f64>,
        matn:      ManuallyDrop<String>,
    }

    // Union — faqat bitta field ishlatiladi bir vaqtda
    // Union — только одно поле используется одновременно
    let mut q: Qiymat = Qiymat { butun_son: ManuallyDrop::new(42i64) };
    unsafe {
        println!("Butun: {}", *q.butun_son);
    }
    // Butun: 42

    q = Qiymat { kasr_son: ManuallyDrop::new(3.14f64) };
    unsafe {
        println!("Kasr: {}", *q.kasr_son);
    }
    // Kasr: 3.14

    // String — drop qo'lda chaqiriladi
    // String — drop вызывается вручную
    q = Qiymat { matn: ManuallyDrop::new(String::from("union matn")) };
    unsafe {
        println!("Matn: {}", *q.matn);
        ManuallyDrop::drop(&mut q.matn); // Stringni ozod qilish
    }
    // Matn: union matn
}

fn manuallydrop_ffi_misoli() {

    // FFI — C kutubxonasiga ownership o'tkazish simulyatsiyasi
    // FFI — симуляция передачи владения в C библиотеку
    // (Haqiqiy FFI da: extern "C" funksiyalar ishlatiladi)
    // (В реальном FFI: используются extern "C" функции)

    // C kutubxona o'z ichida xotirani boshqaradi
    // C библиотека управляет памятью самостоятельно
    fn c_kutubxona_qabul_qiladi(ptr: *mut i32) {
        // C kutubxona xotirani o'z vaqtida ozod qiladi (simulyatsiya)
        // C библиотека освободит память сама (симуляция)
        unsafe {
            println!("C kutubxona qabul qildi: {}", *ptr);
            // Haqiqiy FFI da: c_lib_free(ptr)
            // В реальном FFI: c_lib_free(ptr)
            drop(Box::from_raw(ptr));
        }
    }

    // Rust tomonidan yaratilgan xotira — C ga o'tkaziladi
    // Память созданная в Rust — передаётся в C
    let boxed: Box<i32> = Box::new(100);
    let md: ManuallyDrop<Box<i32>> = ManuallyDrop::new(boxed);
    let ptr: *mut i32 = Box::into_raw(ManuallyDrop::into_inner(md));

    // C kutubxona pointer ni oladi va xotirani o'zi boshqaradi
    // C библиотека получает pointer и управляет памятью сама
    c_kutubxona_qabul_qiladi(ptr);
    // C kutubxona qabul qildi: 100
}

fn manuallydrop_vec_misoli() {

    // Vec ichidagi elementlarni ManuallyDrop bilan boshqarish
    // Управление элементами Vec через ManuallyDrop
    // (Bu pattern Vec::drain yoki mem::forget o'rnida)
    // (Этот паттерн вместо Vec::drain или mem::forget)

    let mut v: Vec<ManuallyDrop<String>> = vec![
        ManuallyDrop::new(String::from("bir")),
        ManuallyDrop::new(String::from("ikki")),
        ManuallyDrop::new(String::from("uch")),
    ];

    // Ba'zi elementlarni olish — drop qo'lda
    // Взятие некоторых элементов — drop вручную
    for i in 0..v.len() {
        let s: String = unsafe { ManuallyDrop::take(&mut v[i]) };
        println!("{}", s);
        // s endi oddiy String — scope tugaganda drop bo'ladi
        // s теперь обычный String — drop при выходе из области
    }
    // bir
    // ikki
    // uch
    // Vec drop bo'lganda — elementlar allaqachon olingan, xavfsiz
    // При drop Vec — элементы уже взяты, безопасно
}

fn real_hayot_misollari() {

    // 1. Custom allocator simulyatsiyasi
    // 1. Симуляция custom allocator
    struct Arena {
        xotira: Vec<u8>,
        pozitsiya: usize,
    }

    impl Arena {
        fn new(hajm: usize) -> Self {
            Arena { xotira: vec![0u8; hajm], pozitsiya: 0 }
        }

        fn ajratish(&mut self, hajm: usize) -> *mut u8 {
            if self.pozitsiya + hajm > self.xotira.len() {
                panic!("Arena to'lib ketdi!");
            }
            let ptr: *mut u8 = unsafe { self.xotira.as_mut_ptr().add(self.pozitsiya) };
            self.pozitsiya += hajm;
            ptr
        }

        fn tozalash(&mut self) {
            self.pozitsiya = 0;
        }
    }

    let mut arena: Arena = Arena::new(1024);
    let ptr1: *mut u8 = arena.ajratish(64);
    let ptr2: *mut u8 = arena.ajratish(128);
    println!("Arena ajratdi: {} va {} ptr", ptr1 as usize, ptr2 as usize);
    arena.tozalash();
    println!("Arena tozalandi");
    // Arena ajratdi: XXXX va YYYY ptr
    // Arena tozalandi

    // 2. Lazy initialization — ManuallyDrop bilan
    // 2. Ленивая инициализация — с ManuallyDrop
    struct LazyResurs {
        ichki: ManuallyDrop<String>,
        initsializatsiya_qilindimi: bool,
    }

    impl LazyResurs {
        fn new() -> Self {
            LazyResurs {
                ichki: ManuallyDrop::new(String::new()),
                initsializatsiya_qilindimi: false,
            }
        }

        fn qiymat(&mut self) -> &str {
            if !self.initsializatsiya_qilindimi {
                println!("Initsializatsiya qilinmoqda...");
                self.ichki = ManuallyDrop::new(String::from("lazy qiymat"));
                self.initsializatsiya_qilindimi = true;
            }
            &*self.ichki
        }
    }

    impl Drop for LazyResurs {
        fn drop(&mut self) {
            if self.initsializatsiya_qilindimi {
                unsafe { ManuallyDrop::drop(&mut self.ichki) };
                println!("LazyResurs drop bo'ldi");
            }
        }
    }

    let mut lazy: LazyResurs = LazyResurs::new();
    println!("{}", lazy.qiymat());
    println!("{}", lazy.qiymat()); // ikkinchi marta — init yo'q
    // Initsializatsiya qilinmoqda...
    // lazy qiymat
    // lazy qiymat
    // LazyResurs drop bo'ldi

    // 3. mem::forget vs ManuallyDrop
    // 3. mem::forget vs ManuallyDrop
    // mem::forget — xotirani "unutadi" (leak)
    // ManuallyDrop — aniq nazorat beradi

    let s1: String = String::from("forget misoli");
    std::mem::forget(s1); // s1 leak bo'ldi — drop chaqirilmaydi
    println!("mem::forget — drop chaqirilmadi (leak!)");

    // ManuallyDrop — aniqroq va xavfsizroq
    // ManuallyDrop — точнее и безопаснее
    let md: ManuallyDrop<String> = ManuallyDrop::new(String::from("md misoli"));
    // into_inner bilan drop tiklanadi
    let s2: String = ManuallyDrop::into_inner(md);
    println!("ManuallyDrop — drop tiklanadi: {}", s2);
    // mem::forget — drop chaqirilmadi (leak!)
    // ManuallyDrop — drop tiklanadi: md misoli
}

fn main() {

    println!("=== MANUALLYDROP ASOSIY ===");
    manuallydrop_asosiy();

    println!("\n=== UNSAFE MISOL ===");
    manuallydrop_unsafe_misoli();

    println!("\n=== UNION MISOL ===");
    manuallydrop_union_misoli();

    println!("\n=== FFI MISOL ===");
    manuallydrop_ffi_misoli();

    println!("\n=== VEC MISOL ===");
    manuallydrop_vec_misoli();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |   1 | ManuallyDrop::new(val)          | Drop o'chirilgan wrapper                   | Обёртка с отключённым drop                              |
// # |   2 | ManuallyDrop::into_inner(md)    | Qiymatni olish, drop tiklanadi             | Взять значение, drop восстанавливается                  |
// # |   3 | ManuallyDrop::drop(&mut md)     | Qo'lda drop (unsafe)                       | Ручной drop (unsafe)                                    |
// # |   4 | ManuallyDrop::take(&mut md)     | Qiymatni olib, joyini default qilish       | Взять значение, заменить default                        |
// # |   5 | *md                             | Deref — ichki qiymatga kirish              | Deref — доступ к внутреннему значению                   |
// # |   6 | md.field                        | Deref orqali field ga kirish               | Доступ к полю через Deref                               |
// # |   7 | zero-cost                       | Runtime xarajat yo'q                       | Нет затрат в runtime                                    |
// # |   8 | sizeof(MD<T>) == sizeof(T)      | O'lcham bir xil                            | Размер одинаковый                                       |
// # |   9 | Union + ManuallyDrop            | Drop'siz union fieldlari                   | Поля union без drop                                     |
// # |  10 | FFI ownership o'tkazish         | C kutubxonaga ownership berish             | Передача владения в C библиотеку                        |
// # |  11 | mem::forget vs ManuallyDrop     | ManuallyDrop — aniqroq nazorat             | ManuallyDrop — более точный контроль                    |
// # |  12 | Unsafe bilan ishlatiladi        | Yolg'iz ishlatilsa xavfli                  | Опасен без unsafe                                       |
// #================================================================================================================================================#