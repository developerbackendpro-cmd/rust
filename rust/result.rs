// #================================================================================================================================================#
// #                                                        RESULT<T, E>                                                                            #
// #                              RESULT — MUVAFFAQIYAT YOKI XATO. RUST DA ERROR HANDLING ASOSI.                                                    #
// #                              RESULT — УСПЕХ ИЛИ ОШИБКА. ОСНОВА ОБРАБОТКИ ОШИБОК В RUST.                                                        #
// #================================================================================================================================================#

fn main() {

    // Ok(x) — muvaffaqiyat, qiymat bor
    // успех, есть значение
    let a: Result<i32, &str> = Ok(42);
    println!("{:?}", a);
    // Ok(42)

    // Err(e) — xato yuz berdi
    // произошла ошибка
    let b: Result<i32, &str> = Err("xato!");
    println!("{:?}", b);
    // Err("xato!")

    // .is_ok() — muvaffaqiyatlimi?
    // успешно?
    let a: Result<i32, &str> = Ok(1);
    println!("{}", a.is_ok());
    // true

    // .is_err() — xatolik bormi?
    // есть ошибка?
    let b: Result<i32, &str> = Err("xato");
    println!("{}", b.is_err());
    // true

    // .unwrap() — qiymatni olish (Err bo'lsa panic!)
    // получить значение (паника если Err!)
    let a: Result<i32, &str> = Ok(5);
    println!("{}", a.unwrap());
    // 5

    // .expect("xabar") — unwrap + o'z xabar bilan
    // unwrap + своё сообщение об ошибке
    let a: Result<i32, &str> = Ok(99);
    println!("{}", a.expect("kutilmagan xato!"));
    // 99

    // .unwrap_or(default) — Err bo'lsa default qaytarish
    // вернуть default если Err
    let a: Result<i32, &str> = Err("xato");
    println!("{}", a.unwrap_or(0));
    // 0

    // .unwrap_or_else(|e| ...) — Err bo'lsa closure ishlatish
    // использовать замыкание если Err
    let a: Result<i32, &str> = Err("xato");
    println!("{}", a.unwrap_or_else(|_| 100));
    // 100

    // .map(|x| ...) — Ok bo'lsa qiymatni o'zgartirish
    // преобразовать значение если Ok
    let a: Result<i32, &str> = Ok(5);
    let b = a.map(|x| x * 10);
    println!("{:?}", b);
    // Ok(50)

    // .map_err(|e| ...) — Err bo'lsa xatoni o'zgartirish
    // преобразовать ошибку если Err
    let a: Result<i32, &str> = Err("xato");
    let b = a.map_err(|e| format!("{}!", e));
    println!("{:?}", b);
    // Err("xato!")

    // .and_then(|x| ...) — Ok bo'lsa yangi Result qaytarish
    // вернуть новый Result если Ok (flatMap)
    let a: Result<i32, &str> = Ok(4);
    let b = a.and_then(|x| if x > 2 { Ok(x * 2) } else { Err("kichik") });
    println!("{:?}", b);
    // Ok(8)

    // .or(other) — Err bo'lsa boshqa Result qaytarish
    // вернуть другой Result если Err
    let a: Result<i32, &str> = Err("xato");
    let b: Result<i32, &str> = a.or(Ok(99));
    println!("{:?}", b);
    // Ok(99)

    // .ok() — Result ni Option ga aylantirish
    // преобразовать Result в Option
    let a: Result<i32, &str> = Ok(5);
    println!("{:?}", a.ok());
    // Some(5)

    // if let Ok / Err — pattern bilan olish
    // получение через паттерн
    let a: Result<i32, &str> = Ok(10);
    if let Ok(val) = a {
        println!("{}", val);
    }
    // 10

    // match — to'liq pattern matching
    // полный pattern matching
    let a: Result<i32, &str> = Err("topilmadi");
    match a {
        Ok(val) => println!("Qiymat: {}", val),
        Err(e)  => println!("Xato: {}", e),
    }
    // Xato: topilmadi

    // ? operatori — Err bo'lsa funksiyadan qaytish
    // вернуться из функции если Err
    fn ikki_baravar(x: Result<i32, &str>) -> Result<i32, &str> {
        let val = x?;
        Ok(val * 2)
    }
    println!("{:?}", ikki_baravar(Ok(5)));
    println!("{:?}", ikki_baravar(Err("xato")));
    // Ok(10)
    // Err("xato")

    // .unwrap_err() — Err qiymatini olish (Ok bo'lsa panic!)
    // получить значение ошибки (паника если Ok!)
    let a: Result<i32, &str> = Err("muammo");
    println!("{}", a.unwrap_err());
    // muammo

    // String::parse() — real hayot misoli
    // пример из реальной жизни
    let son: Result<i32, _> = "42".parse();
    println!("{:?}", son);
    let xato: Result<i32, _> = "abc".parse();
    println!("{:?}", xato);
    // Ok(42)
    // Err(ParseIntError { kind: InvalidDigit })
}

// #================================================================================================================================================#
// # |  №  | Metod                    | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |   1 | Ok(x)                    | Muvaffaqiyat, qiymat bor                             | Успех, есть значение                                 |
// # |   2 | Err(e)                   | Xato yuz berdi                                       | Произошла ошибка                                     |
// # |   3 | is_ok()                  | Muvaffaqiyatligini tekshirish                        | Проверка успешности                                  |
// # |   4 | is_err()                 | Xatolik borligini tekshirish                         | Проверка наличия ошибки                              |
// # |   5 | unwrap()                 | Qiymatni olish (Err = panic!)                        | Получить значение (Err = паника!)                    |
// # |   6 | expect()                 | unwrap + o'z xabar bilan                             | unwrap + своё сообщение об ошибке                    |
// # |   7 | unwrap_or()              | Err bo'lsa default qaytarish                         | Вернуть default если Err                             |
// # |   8 | unwrap_or_else()         | Err bo'lsa closure ishlatish                         | Использовать замыкание если Err                      |
// # |   9 | map()                    | Ok bo'lsa qiymatni o'zgartirish                      | Преобразовать значение если Ok                       |
// # |  10 | map_err()                | Err bo'lsa xatoni o'zgartirish                       | Преобразовать ошибку если Err                        |
// # |  11 | and_then()               | Ok bo'lsa yangi Result qaytarish                     | Вернуть новый Result если Ok                         |
// # |  12 | or()                     | Err bo'lsa boshqa Result qaytarish                   | Вернуть другой Result если Err                       |
// # |  13 | ok()                     | Result ni Option ga aylantirish                      | Преобразовать Result в Option                        |
// # |  14 | if let Ok/Err            | Pattern bilan qiymat olish                           | Получение через паттерн                              |
// # |  15 | match                    | To'liq pattern matching                              | Полный pattern matching                              |
// # |  16 | ? operatori              | Err bo'lsa funksiyadan qaytish                       | Вернуться из функции если Err                        |
// # |  17 | unwrap_err()             | Err qiymatini olish (Ok = panic!)                    | Получить значение ошибки (Ok = паника!)              |
// # |  18 | parse()                  | String dan qiymat olish (real misol)                 | Получить значение из строки (реальный пример)        |
// #================================================================================================================================================#