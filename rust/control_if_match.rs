// #================================================================================================================================================#
// #                                                                 IF  / MATCH                                                                    #
// #                                              if  — shart operatori / if  — условный оператор                                                   #
// #                         match — kuchli pattern matching operatori / match — мощный оператор сопоставления с образцом                           #
// #================================================================================================================================================#

fn main() {

    // oddiy if — shart tekshirish
    // простое условие
    let x = 5;
    if x > 3 {
        println!("katta");
    }
    // katta

    // if / else
    let x = 2;
    if x > 3 {
        println!("katta");
    } else {
        println!("kichik");
    }
    // kichik

    // if / else if / else
    let x = 5;
    if x < 0 {
        println!("manfiy");
    } else if x == 0 {
        println!("nol");
    } else {
        println!("musbat");
    }
    // musbat

    // if — expression (qiymat qaytaradi)
    // if как выражение — возвращает значение
    let x = 10;
    let natija = if x > 5 { "katta" } else { "kichik" };
    println!("{}", natija);
    // katta

    // if let — Option/Result ni ochish
    // раскрытие Option/Result
    let son: Option<i32> = Some(42);
    if let Some(x) = son {
        println!("topildi: {}", x);
    }
    // topildi: 42

    // if let — None holat
    // случай None
    let son: Option<i32> = None;
    if let Some(x) = son {
        println!("topildi: {}", x);
    } else {
        println!("topilmadi");
    }
    // topilmadi

    // if let — Result bilan
    // с типом Result
    let natija: Result<i32, &str> = Ok(100);
    if let Ok(v) = natija {
        println!("muvaffaqiyat: {}", v);
    }
    // muvaffaqiyat: 100

    // &&, || — bir necha shart
    // несколько условий
    let x = 5;
    if x > 0 && x < 10 {
        println!("0 va 10 orasida");
    }
    // 0 va 10 orasida

    // match — asosiy pattern matching
    // основное сопоставление с образцом
    let x = 3;
    match x {
        1 => println!("bir"),
        2 => println!("ikki"),
        3 => println!("uch"),
        _ => println!("boshqa"), // default
    }
    // uch

    // match — qiymat qaytaradi
    // match возвращает значение
    let x = 2;
    let matn = match x {
        1 => "bir",
        2 => "ikki",
        3 => "uch",
        _ => "boshqa",
    };
    println!("{}", matn);
    // ikki

    // match — bir necha qiymat (|)
    // несколько значений в одном рукаве
    let x = 3;
    match x {
        1 | 2 => println!("bir yoki ikki"),
        3 | 4 => println!("uch yoki to'rt"),
        _ => println!("boshqa"),
    }
    // uch yoki to'rt

    // match — range bilan
    // с диапазоном
    let x = 7;
    match x {
        1..=5  => println!("1 dan 5 gacha"),
        6..=10 => println!("6 dan 10 gacha"),
        _      => println!("10 dan katta"),
    }
    // 6 dan 10 gacha

    // match — guard (qo'shimcha shart)
    // дополнительное условие (охранное выражение)
    let x = 5;
    match x {
        n if n < 0 => println!("manfiy: {}", n),
        n if n == 0 => println!("nol"),
        n => println!("musbat: {}", n),
    }
    // musbat: 5

    // match — tuple bilan
    // сопоставление кортежа
    let nuqta = (0, 1);
    match nuqta {
        (0, 0) => println!("markaz"),
        (x, 0) => println!("x o'qida: {}", x),
        (0, y) => println!("y o'qida: {}", y),
        (x, y) => println!("({}, {})", x, y),
    }
    // y o'qida: 1

    // match — Option bilan
    // сопоставление Option
    let son: Option<i32> = Some(42);
    match son {
        Some(x) => println!("qiymat: {}", x),
        None    => println!("bo'sh"),
    }
    // qiymat: 42

    // match — @ binding (qiymatni nom bilan olish)
    // привязка значения к имени
    let x = 7;
    match x {
        n @ 1..=10 => println!("1-10 orasida: {}", n),
        n @ 11..=20 => println!("11-20 orasida: {}", n),
        _ => println!("boshqa"),
    }
    // 1-10 orasida: 7

    // match — String bilan
    // сопоставление строк
    let til = "Rust";
    match til {
        "Python" => println!("Python"),
        "Rust"   => println!("Rust — eng zo'r!"),
        _        => println!("boshqa til"),
    }
    // Rust — eng zo'r!

    // match — bool bilan
    // сопоставление булевого значения
    let faol = true;
    match faol {
        true  => println!("faol"),
        false => println!("nofaol"),
    }
    // faol

    // while let — shart bajarilgunga qadar aylanish
    // цикл пока условие выполняется
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    // 3
    // 2
    // 1

    // matches! macro — qisqa tekshirish
    // краткая проверка совпадения
    let x = 5;
    let natija = matches!(x, 1..=10);
    println!("{}", natija);
    // true
}
// #================================================================================================================================================#
// # |  №  | Operator / Sintaksis          | Tavsif (UZ)                                          | Описание (RU)                                   |
// #================================================================================================================================================#
// # |   1 | if condition {}               | Oddiy shart tekshirish                               | Простое условие                                 |
// # |   2 | if / else {}                  | Ikki tarmoqli shart                                  | Двухветочное условие                            |
// # |   3 | if / else if / else           | Ko'p tarmoqli shart                                  | Многоветочное условие                           |
// # |   4 | if as expression              | Qiymat qaytaruvchi if (ternary o'rniga)              | If как выражение (вместо ternary)               |
// # |   5 | if let Some(x) = opt {}       | Option/Result ni ochish                              | Раскрытие Option/Result                         |
// # |   6 | if let ... else {}            | None/Err holatini ham tekshirish                     | Проверка с обработкой None/Err                  |
// # |   7 | && , ||                       | Bir necha shart (AND, OR)                            | Несколько условий (AND, OR)                     |
// # |   8 | match value { pattern => }    | Asosiy pattern matching                              | Основное сопоставление с образцом               |
// # |   9 | match as expression           | Qiymat qaytaruvchi match                             | Match как выражение                             |
// # |  10 | pattern1 | pattern2           | Bir necha pattern (OR)                               | Несколько паттернов (ИЛИ)                       |
// # |  11 | 1..=10                        | Range bilan matching                                 | Сопоставление с диапазоном                      |
// # |  12 | n if condition                | Guard (qo'shimcha shart)                             | Guard (дополнительное условие)                  |
// # |  13 | (x, y) pattern                | Tuple bilan matching                                 | Сопоставление кортежа                           |
// # |  14 | Some(x) / None                | Option bilan matching                                | Сопоставление Option                            |
// # |  15 | n @ 1..=10                    | @ binding (qiymatni nom bilan olish)                 | Привязка значения к имени                       |
// # |  16 | "String" pattern              | String bilan matching                                | Сопоставление строк                             |
// # |  17 | true / false                  | Bool bilan matching                                  | Сопоставление булевых значений                  |
// # |  18 | while let Some(x) = iter      | Shart bajarilguncha aylanish                         | Цикл с условием                                 |
// # |  19 | matches!(value, pattern)      | Qisqa pattern tekshiruvi (macro)                     | Краткая проверка паттерна (макрос)              |
// # |  20 | _                             | Default pattern (catch-all)                          | Паттерн по умолчанию                            |
// #================================================================================================================================================#