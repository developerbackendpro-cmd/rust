// #================================================================================================================================================#
// #                                                                    STD::PANIC                                                                  #
// #                            PANIC — DASTUR BUZILGANDA CHIQISH. CATCH_UNWIND, SET_HOOK, BACKTRACE. QACHON PANIC, QACHON RESULT.                  #
// #                            PANIC — ВЫХОД ПРИ КРИТИЧЕСКОЙ ОШИБКЕ. CATCH_UNWIND, SET_HOOK, BACKTRACE. КОГДА PANIC, КОГДА RESULT.                 #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::panic;
use std::fmt;

// std::panic nima:
// Что такое std::panic:
//
//   panic!()     — dasturni to'xtatuvchi macro
//   panic!()     — макрос останавливающий программу
//   catch_unwind — panic ni ushlash (test va FFI uchun)
//   catch_unwind — поймать panic (для тестов и FFI)
//   set_hook     — panic handler o'rnatish (logging uchun)
//   set_hook     — установка обработчика (для логирования)
//   take_hook     — standart hook ni olish
//   take_hook     — взять стандартный hook
//   resume_unwind — panic ni davom ettirish
//   resume_unwind — продолжить паник
//
// Qachon panic, qachon Result:
// Когда panic, когда Result:
//   panic!  — dastur mantig'i buzilganda (bug), hech qachon bo'lmasligi kerak
//   panic!  — при нарушении логики программы (баг), никогда не должно происходить
//   Result  — kutilgan xato holatlari (fayl yo'q, tarmoq uzildi)
//   Result  — ожидаемые случаи ошибок (нет файла, нет сети)

fn panic_asosiy_misollari() {

    // panic! — dasturni to'xtatadi
    // panic! — останавливает программу
    // panic!("Xato: {}", "biror narsa");  ← dastur to'xtaydi

    // catch_unwind — panic ni ushlash
    // catch_unwind — поймать panic
    let natija = panic::catch_unwind(|| {
        println!("Panic oldidan");
        panic!("Test panic!");
    });
    println!("Panic ushlandi: {}", natija.is_err());
    // Panic oldidan
    // Panic ushlandi: true

    // catch_unwind — muvaffaqiyatli holat
    // catch_unwind — успешный случай
    let muvaffaqiyat = panic::catch_unwind(|| {
        42
    });
    println!("{:?}", muvaffaqiyat);
    // Ok(42)

    // catch_unwind — xato xabarini olish
    // catch_unwind — получение сообщения об ошибке
    let xato = panic::catch_unwind(|| {
        panic!("aniq xato xabari");
    });
    match xato {
        Err(e) => {
            if let Some(s) = e.downcast_ref::<&str>() {
                println!("Panic xabari: {}", s);
            }
        }
        Ok(_) => {}
    }
    // Panic xabari: aniq xato xabari

    // catch_unwind — String panic
    // catch_unwind — String panic
    let xato2 = panic::catch_unwind(|| {
        panic!("{}", String::from("String panic xabari"));
    });
    match xato2 {
        Err(e) => {
            if let Some(s) = e.downcast_ref::<String>() {
                println!("String panic: {}", s);
            }
        }
        Ok(_) => {}
    }
    // String panic: String panic xabari
}

fn panic_hook_misollari() {

    // set_hook — panic handler o'rnatish
    // set_hook — установка обработчика panic
    panic::set_hook(Box::new(|info| {
        let xabar: &str = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "noma'lum xato"
        };

        let joy: String = if let Some(j) = info.location() {
            format!("{}:{}:{}", j.file(), j.line(), j.column())
        } else {
            "noma'lum joy".to_string()
        };

        println!("[CUSTOM PANIC] {} | Joy: {}", xabar, joy);
    }));

    // Hook ishlaydi
    // Hook работает
    let _ = panic::catch_unwind(|| {
        panic!("hook testi");
    });
    // [CUSTOM PANIC] hook testi | Joy: src/main.rs:XX:XX

    // take_hook — standart hookni tiklash
    // take_hook — восстановление стандартного hook
    let _eski_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {})); // jimgina panic
    let _ = panic::catch_unwind(|| panic!("jim panic"));
    // Hech narsa chiqmaydi

    // Standart hookni qaytarish
    // Восстановление стандартного hook
    let _ = panic::take_hook(); // jimgina hookni olib tashlash
}

fn unwind_safe_misollari() {

    // UnwindSafe — catch_unwind uchun xavfsiz turlar
    // UnwindSafe — безопасные типы для catch_unwind
    // Copy, Rc emas, RefCell emas turlar — UnwindSafe
    // Copy, не Rc, не RefCell — UnwindSafe

    // AssertUnwindSafe — xavfsizlikni kafolatlash
    // AssertUnwindSafe — гарантия безопасности
    let mut qiymat: i32 = 0;
    let natija = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        qiymat = 42;
        // panic!("test");  // bu yerda panic bo'lsa qiymat 42 bo'lib qoladi
    }));
    println!("qiymat: {}, natija: {:?}", qiymat, natija);
    // qiymat: 42, natija: Ok(())

    // Vec bilan AssertUnwindSafe
    // AssertUnwindSafe с Vec
    let mut v: Vec<i32> = vec![1, 2, 3];
    let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        v.push(4);
        v.push(5);
    }));
    println!("{:?}", v);
    // [1, 2, 3, 4, 5]
}

fn panic_turlari_misollari() {

    // index out of bounds — panic
    // index out of bounds — panic
    let v: Vec<i32> = vec![1, 2, 3];
    let natija = panic::catch_unwind(|| v[10]);
    println!("OOB: {}", natija.is_err());
    // OOB: true

    // unwrap() — None panic
    // unwrap() — panic от None
    let natija2 = panic::catch_unwind(|| {
        let opt: Option<i32> = None;
        opt.unwrap()
    });
    println!("unwrap None: {}", natija2.is_err());
    // unwrap None: true

    // integer overflow — debug mode panic, release mode wrapping
    // integer overflow — panic в debug, wrapping в release
    let natija3 = panic::catch_unwind(|| {
        let x: u8 = 255;
        x.checked_add(1)  // panic o'rniga None
    });
    println!("{:?}", natija3);
    // Ok(None)

    // divide by zero — panic
    // divide by zero — panic
    let natija4 = panic::catch_unwind(|| {
        let x: i32 = 10;
        let y: i32 = "0".parse::<i32>().unwrap(); // runtime da 0
        x / y
    });
    println!("Bo'lish nolga: {}", natija4.is_err());
    // Bo'lish nolga: true

    // stack overflow — catch_unwind tutmaydi!
    // stack overflow — catch_unwind не поймает!
    // (Buni test qilmaymiz — dastur o'lib ketadi)
    // (Не тестируем — программа упадёт)
}

fn panic_qachon_ishlatish() {

    // 1. Invariant buzilganda — always panic
    // 1. Нарушение инварианта — всегда panic
    fn sqrt_musbat(x: f64) -> f64 {
        // Bu funksiya HECH QACHON manfiy son bilan chaqirilmasligi kerak
        // Эта функция НИКОГДА не должна вызываться с отрицательным числом
        assert!(x >= 0.0, "sqrt manfiy songa chaqirildi: {}", x);
        x.sqrt()
    }

    println!("{}", sqrt_musbat(4.0));
    // 2.0

    let xato = panic::catch_unwind(|| sqrt_musbat(-1.0));
    println!("Invariant xato: {}", xato.is_err());
    // Invariant xato: true

    // 2. Kutilgan xatolar uchun — Result
    // 2. Ожидаемые ошибки — Result
    fn fayl_o_qi(yo_l: &str) -> Result<String, String> {
        if yo_l.is_empty() {
            Err("Yo'l bo'sh".to_string())
        } else {
            Ok(format!("{} mazmuni", yo_l))
        }
    }

    match fayl_o_qi("") {
        Ok(s)  => println!("{}", s),
        Err(e) => println!("Xato: {}", e),
    }
    // Xato: Yo'l bo'sh

    // 3. assert!, assert_eq!, assert_ne! — testlarda
    // 3. assert!, assert_eq!, assert_ne! — в тестах
    let x: i32 = 2 + 2;
    assert_eq!(x, 4, "2+2 = 4 bo'lishi kerak");
    assert_ne!(x, 5, "2+2 = 5 bo'lmasligi kerak");
    assert!(x > 0, "x musbat bo'lishi kerak");
    println!("Barcha assertlar o'tdi");
    // Barcha assertlar o'tdi

    // 4. unreachable! va todo!
    // 4. unreachable! и todo!
    let kun: u8 = 3;
    let kun_nomi: &str = match kun {
        1 => "Dushanba",
        2 => "Seshanba",
        3 => "Chorshanba",
        4 => "Payshanba",
        5 => "Juma",
        6 => "Shanba",
        7 => "Yakshanba",
        _ => unreachable!("Hafta kuni 1-7 orasida bo'lishi kerak"),
    };
    println!("{}", kun_nomi);
    // Chorshanba
}

fn real_hayot_misollari() {

    // 1. Test frameworki simulyatsiyasi
    // 1. Симуляция тестового фреймворка
    struct TestNatija {
        nomi: String,
        muvaffaqiyatli: bool,
        xato: Option<String>,
    }

    fn test_ishga_tush(nomi: &str, f: impl Fn() + panic::UnwindSafe) -> TestNatija {
        let natija = panic::catch_unwind(f);
        match natija {
            Ok(_) => TestNatija {
                nomi: nomi.to_string(),
                muvaffaqiyatli: true,
                xato: None,
            },
            Err(e) => {
                let xabar: String = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "noma'lum xato".to_string()
                };
                TestNatija {
                    nomi: nomi.to_string(),
                    muvaffaqiyatli: false,
                    xato: Some(xabar),
                }
            }
        }
    }

    let testlar: Vec<TestNatija> = vec![
        test_ishga_tush("qo'shish testi", || {
            assert_eq!(2 + 2, 4);
        }),
        test_ishga_tush("muvaffaqiyatsiz test", || {
            assert_eq!(2 + 2, 5, "Matematik xato!");
        }),
        test_ishga_tush("vec indeks testi", || {
            let v: Vec<i32> = vec![1, 2, 3];
            let _ = v[0];
        }),
    ];

    let mut o_tdi: usize = 0;
    let mut muvaffaqiyatsiz: usize = 0;

    for t in &testlar {
        if t.muvaffaqiyatli {
            println!("✅ {}", t.nomi);
            o_tdi += 1;
        } else {
            println!("❌ {} — {}", t.nomi, t.xato.as_deref().unwrap_or("xato"));
            muvaffaqiyatsiz += 1;
        }
    }
    println!("\n{} o'tdi, {} muvaffaqiyatsiz", o_tdi, muvaffaqiyatsiz);
    // ✅ qo'shish testi
    // ❌ muvaffaqiyatsiz test — Matematik xato!
    // ✅ vec indeks testi
    //
    // 2 o'tdi, 1 muvaffaqiyatsiz

    // 2. Resursni himoya qiluvchi catch_unwind
    // 2. Защита ресурсов с помощью catch_unwind
    struct Resurs {
        nomi: String,
    }
    impl Drop for Resurs {
        fn drop(&mut self) {
            println!("Resurs '{}' tozalandi", self.nomi);
        }
    }

    let _ = panic::catch_unwind(|| {
        let _r = Resurs { nomi: "muhim_resurs".to_string() };
        // panic bo'lsa ham Drop chaqiriladi!
        // даже при panic Drop вызывается!
        panic!("panic bilan chiqish");
    });
    // Resurs 'muhim_resurs' tozalandi  ← Drop chaqirildi!
    println!("Resurs himoyalandi");
    // Resurs himoyalandi
}

fn main() {

    println!("=== PANIC ASOSIY ===");
    panic_asosiy_misollari();

    println!("\n=== PANIC HOOK ===");
    panic_hook_misollari();

    println!("\n=== UNWIND SAFE ===");
    unwind_safe_misollari();

    println!("\n=== PANIC TURLARI ===");
    panic_turlari_misollari();

    println!("\n=== QACHON PANIC ===");
    panic_qachon_ishlatish();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                | Tavsif (UZ)                                | Описание (RU)                                               |
// #================================================================================================================================================#
// # |   1 | panic!("...")               | Dasturni to'xtatadi                        | Останавливает программу                                     |
// # |   2 | catch_unwind(|| ...)        | Panic ni ushlash — Result qaytaradi        | Поймать panic — возвращает Result                           |
// # |   3 | set_hook(Box::new(|info|))  | Custom panic handler                       | Пользовательский обработчик panic                           |
// # |   4 | take_hook()                 | Mavjud hook ni olish                       | Взять текущий hook                                          |
// # |   5 | AssertUnwindSafe(f)         | Xavfsizlikni kafolatlash                   | Гарантия безопасности                                       |
// # |   6 | info.payload()              | Panic xabari                               | Сообщение panic                                             |
// # |   7 | info.location()             | Panic joyi (fayl, qator)                   | Место panic (файл, строка)                                  |
// # |   8 | assert!(shart, "...")       | Shart bajarilmasa panic                    | Panic если условие не выполнено                             |
// # |   9 | assert_eq!(a, b, "...")     | Tenglik shart                              | Условие равенства                                           |
// # |  10 | unreachable!()              | Yetib bo'lmaydigan kod                     | Недостижимый код                                            |
// # |  11 | todo!()                     | Hali yozilmagan kod                        | Ещё не написанный код                                       |
// # |  12 | panic vs Result             | Bug → panic, kutilgan xato → Result        | Баг → panic, ожидаемая ошибка → Result                      |
// # |  13 | Drop + panic                | Panic bo'lsa ham Drop chaqiriladi          | Drop вызывается даже при panic                              |
// # |  14 | stack overflow              | catch_unwind tutmaydi                      | catch_unwind не поймает                                     |
// #================================================================================================================================================#