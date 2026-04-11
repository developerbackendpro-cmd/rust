// #================================================================================================================================================#
// #                                                          PATTERN MATCHING                                                                      #
// #                            Pattern Matching — qiymatni shaklga solishtirish va ichidagi ma'lumotni olish                                       #
// #                            Pattern Matching — сопоставление значения с образцом и извлечение данных                                            #
// #================================================================================================================================================#
// #                                      Pattern Matching — asosiy tushunchalar / Основные концепции                                               #
// #                                      match      — exhaustive (barcha holatlar yopilishi shart)                                                 #
// #                                      if let     — bitta pattern tekshirish (match ning qisqa shakli)                                           #
// #                                      while let  — pattern bajarilgunga qadar sikl                                                              #
// #                                      _          — default (boshqa hammasi)                                                                     #
// #                                      @          — qiymatni nom bilan bog'lash                                                                  #
// #                                      guard      — qo'shimcha shart (if n > 0)                                                                  #
// #                                      ..         — qolgan maydonlarni e'tiborsiz qoldirish                                                      #
// #================================================================================================================================================#
#![allow(dead_code, unused_variables)]

fn main() {

    // match — asosiy pattern matching
    // match — основное сопоставление
    let x = 3;
    match x {
        1 => println!("bir"),
        2 => println!("ikki"),
        3 => println!("uch"),
        _ => println!("boshqa"),
    }
    // uch

    // match — bir nechta qiymat (|)
    // match — несколько значений
    let x = 3;
    match x {
        1 | 2 => println!("bir yoki ikki"),
        3 | 4 => println!("uch yoki to'rt"),
        _     => println!("boshqa"),
    }
    // uch yoki to'rt

    // match — range bilan
    // match — с диапазоном
    let x = 7;
    match x {
        1..=5  => println!("1 dan 5 gacha"),
        6..=10 => println!("6 dan 10 gacha"),
        _      => println!("10 dan katta"),
    }
    // 6 dan 10 gacha

    // match — qiymat qaytaradi
    // match — возвращает значение
    let x = 2;
    let matn = match x {
        1 => "bir",
        2 => "ikki",
        _ => "boshqa",
    };
    println!("{}", matn);
    // ikki

    // match — guard (qo'shimcha shart)
    // match — охранное выражение
    let x = 5;
    match x {
        n if n < 0  => println!("manfiy: {}", n),
        n if n == 0 => println!("nol"),
        n           => println!("musbat: {}", n),
    }
    // musbat: 5

    // match — @ binding (qiymatni nom bilan olish)
    // match — привязка значения к имени
    let x = 7;
    match x {
        n @ 1..=10  => println!("1-10 orasida: {}", n),
        n @ 11..=20 => println!("11-20 orasida: {}", n),
        _           => println!("boshqa"),
    }
    // 1-10 orasida: 7

    // match — tuple bilan
    // match — с кортежем
    let nuqta = (0, 1);
    match nuqta {
        (0, 0) => println!("markaz"),
        (x, 0) => println!("x o'qida: {}", x),
        (0, y) => println!("y o'qida: {}", y),
        (x, y) => println!("({}, {})", x, y),
    }
    // y o'qida: 1

    // match — struct bilan destructuring
    // match — деструктуризация структуры
    struct Nuqta { x: i32, y: i32 }
    let n = Nuqta { x: 0, y: 5 };
    match n {
        Nuqta { x: 0, y } => println!("y o'qida: {}", y),
        Nuqta { x, y: 0 } => println!("x o'qida: {}", x),
        Nuqta { x, y }    => println!("({}, {})", x, y),
    }
    // y o'qida: 5

    // match — enum bilan
    // match — с enum
    enum Xabar {
        Chiqish,
        Harakat(i32, i32),
        Matn(String),
    }
    let x = Xabar::Harakat(10, 20);
    match x {
        Xabar::Chiqish        => println!("Chiqish"),
        Xabar::Harakat(a, b)  => println!("Harakat: ({}, {})", a, b),
        Xabar::Matn(s)        => println!("Matn: {}", s),
    }
    // Harakat: (10, 20)

    // match — Option bilan
    // match — с Option
    let son: Option<i32> = Some(42);
    match son {
        Some(x) => println!("Qiymat: {}", x),
        None    => println!("Bo'sh"),
    }
    // Qiymat: 42

    // match — Result bilan
    // match — с Result
    let natija: Result<i32, &str> = Ok(100);
    match natija {
        Ok(n)  => println!("Ok: {}", n),
        Err(e) => println!("Xato: {}", e),
    }
    // Ok: 100

    // match — .. (qolganlarni e'tiborsiz qoldirish)
    // match — игнорирование оставшихся полей
    struct Odam { ism: String, yosh: u32, shahar: String }
    let o = Odam {
        ism: String::from("Dilshod"),
        yosh: 25,
        shahar: String::from("Toshkent"),
    };
    match o {
        Odam { ism, .. } => println!("Ism: {}", ism),
    }
    // Ism: Dilshod

    // if let — bitta pattern (match ning qisqa shakli)
    // if let — одно условие (краткая форма match)
    let son: Option<i32> = Some(42);
    if let Some(x) = son {
        println!("Topildi: {}", x);
    }
    // Topildi: 42

    // if let — else bilan
    // if let — с else
    let son: Option<i32> = None;
    if let Some(x) = son {
        println!("Qiymat: {}", x);
    } else {
        println!("Bo'sh");
    }
    // Bo'sh

    // if let — Result bilan
    // if let — с Result
    let natija: Result<i32, &str> = Ok(5);
    if let Ok(v) = natija {
        println!("Muvaffaqiyat: {}", v);
    }
    // Muvaffaqiyat: 5

    // while let — pattern bajarilgunga qadar
    // while let — пока условие выполняется
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    // 3
    // 2
    // 1

    // matches! macro — qisqa tekshirish
    // matches! macro — краткая проверка
    let x = 5;
    println!("{}", matches!(x, 1..=10));  // true
    println!("{}", matches!(x, 11..=20)); // false

    // let else — pattern to'g'ri kelmasa chiqish
    // let else — выход если паттерн не совпал
    fn tekshir(son: Option<i32>) {
        let Some(x) = son else {
            println!("Bo'sh!");
            return;
        };
        println!("Qiymat: {}", x);
    }
    tekshir(Some(10)); // Qiymat: 10
    tekshir(None);     // Bo'sh!
}
// #================================================================================================================================================#
// # |  №  | Mavzu                      | Tavsif (UZ)                                            | Описание (RU)                                    |
// #================================================================================================================================================#
// # |   1 | match x { ... }            | Exhaustive — barcha holatlar yopilishi shart            | Все варианты должны быть покрыты                |
// # |   2 | 1 | 2 =>                   | Bir nechta qiymat bitta armda                           | Несколько значений в одном рукаве               |
// # |   3 | 1..=5 =>                   | Range pattern                                           | Диапазон как паттерн                            |
// # |   4 | _ =>                       | Default — qolgan hammasi                                | По умолчанию — все остальные                    |
// # |   5 | n if n > 0 =>              | Guard — qo'shimcha shart                                | Дополнительное условие (охрана)                 |
// # |   6 | n @ 1..=10 =>              | Binding — qiymatni nom bilan olish                      | Привязка значения к имени                       |
// # |   7 | (x, 0) =>                  | Tuple destructuring                                     | Деструктуризация кортежа                        |
// # |   8 | Struct { x, .. } =>        | Struct destructuring + qolganlarni o'tkazib yuborish    | Деструктуризация структуры игнорирование полей  |
// # |   9 | Some(x) / None             | Option pattern                                          | Паттерн Option                                  |
// # |  10 | Ok(n) / Err(e)             | Result pattern                                          | Паттерн Result                                  |
// # |  11 | if let Some(x) = ...       | Bitta pattern — qisqa match                             | Одно условие — краткий match                    |
// # |  12 | while let Some(x) = ...    | Pattern bajarilgunga qadar sikl                         | Цикл пока паттерн совпадает                     |
// # |  13 | matches!(x, 1..=10)        | Tez tekshirish — bool qaytaradi                         | Быстрая проверка — возвращает bool              |
// # |  14 | let Some(x) = ... else {}  | let-else — pattern to'g'ri kelmasa chiqish              | let-else — выход если паттерн не совпал         |
// #================================================================================================================================================#