// #================================================================================================================================================#
// #                                                                ENUM                                                                            #
// #                                    Enum — bir nechta variant ichidan birini ifodalovchi type                                                   #
// #                                    Enum — тип, представляющий одно из нескольких возможных значений                                            #
// #================================================================================================================================================#
// #                                    Enum — asosiy tushunchalar / Основные концепции                                                             #
// #                                    Enum — algebraic data type (ADT), har variant o'z ma'lumotini saqlashi mumkin                               #
// #                                    Option<T> — qiymat bor yoki yo'q (Some/None)                                                                #
// #                                    Result<T,E> — muvaffaqiyat yoki xato (Ok/Err)                                                               #
// #                                    match — enum ni ochish uchun eng kuchli operator                                                            #
// #================================================================================================================================================#
#![allow(dead_code, unused_variables)]

fn main() {

    // Oddiy enum — variantlar
    // Простое перечисление — варианты
    enum Yonalish {
        Shimol,
        Janub,
        Sharq,
        Garb,
    }
    let yo = Yonalish::Shimol;
    match yo {
        Yonalish::Shimol => println!("Shimol"),
        Yonalish::Janub  => println!("Janub"),
        Yonalish::Sharq  => println!("Sharq"),
        Yonalish::Garb   => println!("G'arb"),
    }
    // Shimol

    // Enum ichida ma'lumot saqlash
    // Хранение данных внутри enum
    enum Xabar {
        Matn(String),
        Son(i32),
        Koordinata(f64, f64),
    }
    let x1 = Xabar::Matn(String::from("Salom!"));
    let _x2 = Xabar::Son(42);
    let _x3 = Xabar::Koordinata(10.5, 20.3);
    match x1 {
        Xabar::Matn(s)          => println!("Matn: {}", s),
        Xabar::Son(n)           => println!("Son: {}", n),
        Xabar::Koordinata(x, y) => println!("({}, {})", x, y),
    }
    // Matn: Salom!

    // Enum + impl — methodlar qo'shish
    // Добавление методов к enum
    enum Shakl {
        Doira(f64),
        Kvadrat(f64),
        Uchburchak(f64, f64),
    }
    impl Shakl {
        fn maydon(&self) -> f64 {
            match self {
                Shakl::Doira(r)           => 3.14 * r * r,
                Shakl::Kvadrat(a)         => a * a,
                Shakl::Uchburchak(a, b)   => 0.5 * a * b,
            }
        }
    }
    let d = Shakl::Doira(5.0);
    let k = Shakl::Kvadrat(4.0);
    println!("{:.2}", d.maydon()); // 78.50
    println!("{:.2}", k.maydon()); // 16.00

    // Option<T> — qiymat bor yoki yo'q
    // Option<T> — значение есть или нет
    let bor: Option<i32> = Some(42);
    let _yoq: Option<i32> = None;
    match bor {
        Some(x) => println!("Qiymat: {}", x),
        None    => println!("Bo'sh"),
    }
    // Qiymat: 42

    // Option methodlari
    // Методы Option
    let son: Option<i32> = Some(10);
    println!("{}", son.unwrap());            // 10
    println!("{}", son.unwrap_or(0));        // 10
    println!("{}", son.is_some());           // true
    println!("{}", son.is_none());           // false

    let yoq: Option<i32> = None;
    println!("{}", yoq.unwrap_or(99));       // 99
    println!("{}", yoq.unwrap_or_default()); // 0

    // Option map — ichidagi qiymatni o'zgartirish
    // Option map — преобразование значения внутри
    let son: Option<i32> = Some(5);
    let ikki_kat = son.map(|x| x * 2);
    println!("{:?}", ikki_kat);
    // Some(10)

    // Option and_then — zanjir
    // Option and_then — цепочка
    let son: Option<i32> = Some(10);
    let natija = son.and_then(|x| if x > 5 { Some(x * 2) } else { None });
    println!("{:?}", natija);
    // Some(20)

    // if let — Option ni ochish
    // Раскрытие Option через if let
    let son: Option<i32> = Some(42);
    if let Some(x) = son {
        println!("Topildi: {}", x);
    }
    // Topildi: 42

    // Result<T, E> — muvaffaqiyat yoki xato
    // Result<T, E> — успех или ошибка
    fn bo_lish(a: i32, b: i32) -> Result<i32, String> {
        if b == 0 {
            Err(String::from("Nolga bo'lib bo'lmaydi!"))
        } else {
            Ok(a / b)
        }
    }
    println!("{:?}", bo_lish(10, 2)); // Ok(5)
    println!("{:?}", bo_lish(10, 0)); // Err("Nolga bo'lib bo'lmaydi!")

    // Result methodlari
    // Методы Result
    let ok: Result<i32, &str> = Ok(10);
    println!("{}", ok.unwrap());     // 10
    println!("{}", ok.unwrap_or(0)); // 10
    println!("{}", ok.is_ok());      // true
    println!("{}", ok.is_err());     // false

    let err: Result<i32, &str> = Err("xato");
    println!("{}", err.unwrap_or(99)); // 99

    // match bilan Result
    // Result через match
    match bo_lish(10, 2) {
        Ok(n)  => println!("Natija: {}", n),
        Err(e) => println!("Xato: {}", e),
    }
    // Natija: 5

    // #[derive] — avtomatik trait qo'shish
    // #[derive] — автоматическое добавление трейтов
    #[derive(Debug, PartialEq)]
    enum Rang {
        Qizil,
        Yashil,
        Kok,
    }
    let r = Rang::Qizil;
    println!("{:?}", r);               // Qizil
    println!("{}", r == Rang::Qizil);  // true

    // Enum struct variant
    // Вариант enum со структурой
    enum Hodisa {
        Bosish { x: i32, y: i32 },
        Yozish(String),
        Chiqish,
    }
    let h = Hodisa::Bosish { x: 10, y: 20 };
    match h {
        Hodisa::Bosish { x, y } => println!("Bosish: ({}, {})", x, y),
        Hodisa::Yozish(s)       => println!("Yozish: {}", s),
        Hodisa::Chiqish         => println!("Chiqish"),
    }
    // Bosish: (10, 20)
}
// #================================================================================================================================================#
// # |  №  | Mavzu                    | Tavsif (UZ)                                              | Описание (RU)                                    |
// #================================================================================================================================================#
// # |   1 | enum Tur { A, B }        | Variantlar ro'yxati                                      | Перечисление вариантов                           |
// # |   2 | enum Tur { A(T) }        | Variant ichida ma'lumot saqlash                          | Хранение данных в варианте                       |
// # |   3 | impl Enum { fn ... }     | Enum ga method qo'shish                                  | Добавление методов к enum                        |
// # |   4 | Option<T>                | Some(x) yoki None — qiymat bor/yo'q                      | Значение есть или отсутствует                    |
// # |   5 | Result<T,E>              | Ok(x) yoki Err(e) — muvaffaqiyat/xato                    | Успех или ошибка                                 |
// # |   6 | .unwrap()                | Ichidagi qiymatni ol (None/Err bo'lsa panic)             | Извлечь значение (паника при None/Err)           |
// # |   7 | .unwrap_or(x)            | Yoki qiymat, yoki default                                | Значение или запасной вариант                    |
// # |   8 | .is_some() / .is_none()  | Option tekshirish                                        | Проверка Option                                  |
// # |   9 | .is_ok() / .is_err()     | Result tekshirish                                        | Проверка Result                                  |
// # |  10 | .map(|x| ...)            | Ichidagi qiymatni o'zgartirish                           | Преобразование значения внутри                   |
// # |  11 | .and_then(|x| ...)       | Zanjir — natija ham Option/Result                        | Цепочка — результат тоже Option/Result           |
// # |  12 | if let Some(x) = ...     | Option ni qulay ochish                                   | Удобное раскрытие Option                         |
// # |  13 | #[derive(Debug)]         | {:?} bilan chiqarish                                     | Вывод через {:?}                                 |
// # |  14 | struct variant           | { x: i32, y: i32 } — nomlangan maydonlar                 | Именованные поля в варианте                      |
// #================================================================================================================================================#