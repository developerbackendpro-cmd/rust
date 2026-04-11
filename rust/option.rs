// #================================================================================================================================================#
// #                                                              OPTION<T>                                                                         #
// #                                     OPTION — QIYMAT BOR YOKI YO'Q. NULL O'RNIGA ISHLATILADI.                                                   #
// #                                     OPTION — ЕСТЬ ЗНАЧЕНИЕ ИЛИ НЕТ. ИСПОЛЬЗУЕТСЯ ВМЕСТО NULL.                                                  #
// #================================================================================================================================================#

fn main() {

    // Some(x) — qiymat bor
    // есть значение
    let a: Option<i32> = Some(42);
    println!("{:?}", a);
    // Some(42)

    // None — qiymat yo'q
    // значения нет
    let b: Option<i32> = None;
    println!("{:?}", b);
    // None

    // .is_some() — qiymat bormi?
    // есть ли значение?
    let a = Some(5);
    println!("{}", a.is_some());
    // true

    // .is_none() — qiymat yo'qmi?
    // значения нет?
    let b: Option<i32> = None;
    println!("{}", b.is_none());
    // true

    // .unwrap() — qiymatni olish (None bo'lsa panic!)
    // получить значение (паника если None!)
    let a = Some(10);
    println!("{}", a.unwrap());
    // 10

    // .expect("xabar") — unwrap + o'z xabar bilan
    // unwrap + своё сообщение об ошибке
    let a = Some(99);
    println!("{}", a.expect("qiymat yo'q!"));
    // 99

    // .unwrap_or(default) — None bo'lsa default qaytarish
    // вернуть default если None
    let a: Option<i32> = None;
    println!("{}", a.unwrap_or(0));
    // 0

    // .unwrap_or_else(|| ...) — None bo'lsa closure ishlatish
    // использовать замыкание если None
    let a: Option<i32> = None;
    println!("{}", a.unwrap_or_else(|| 2 + 2));
    // 4

    // .map(|x| ...) — Some bo'lsa qiymatni o'zgartirish
    // преобразовать значение если Some
    let a = Some(5);
    let b = a.map(|x| x * 10);
    println!("{:?}", b);
    // Some(50)

    // .and_then(|x| ...) — Some bo'lsa yangi Option qaytarish
    // вернуть новый Option если Some (flatMap)
    let a = Some(4);
    let b = a.and_then(|x| if x > 2 { Some(x * 2) } else { None });
    println!("{:?}", b);
    // Some(8)

    // .or(other) — None bo'lsa boshqa Option qaytarish
    // вернуть другой Option если None
    let a: Option<i32> = None;
    let b = a.or(Some(99));
    println!("{:?}", b);
    // Some(99)

    // .filter(|x| ...) — shartga mos bo'lmasa None
    // None если условие не выполнено
    let a = Some(10);
    let b = a.filter(|&x| x > 5);
    println!("{:?}", b);
    // Some(10)

    // .take() — qiymatni olish, o'zini None qilish
    // взять значение, оставив None
    let mut a = Some(42);
    let b = a.take();
    println!("{:?}", a);
    println!("{:?}", b);
    // None
    // Some(42)

    // .replace(x) — yangi qiymat qo'yish, eskisini qaytarish
    // заменить значение, вернуть старое
    let mut a = Some(1);
    let old = a.replace(99);
    println!("{:?}", a);
    println!("{:?}", old);
    // Some(99)
    // Some(1)

    // .zip(other) — ikki Option ni birlashtirish
    // объединить два Option в пару
    let a = Some(1);
    let b = Some("salom");
    println!("{:?}", a.zip(b));
    // Some((1, "salom"))

    // .flatten() — Option<Option<T>> ni Option<T> ga aylantirish
    // преобразовать Option<Option<T>> в Option<T>
    let a: Option<Option<i32>> = Some(Some(5));
    println!("{:?}", a.flatten());
    // Some(5)

    // if let — None ni ignore qilib Some ni olish
    // получить Some игнорируя None
    let a = Some(7);
    if let Some(val) = a {
        println!("{}", val);
    }
    // 7

    // ? operatori — None bo'lsa funksiyadan qaytish
    // вернуться из функции если None
    fn ikki_baravar(x: Option<i32>) -> Option<i32> {
        let val = x?;
        Some(val * 2)
    }
    println!("{:?}", ikki_baravar(Some(5)));
    println!("{:?}", ikki_baravar(None));
    // Some(10)
    // None

    // .ok_or(err) — Option ni Result ga aylantirish
    // преобразовать Option в Result
    let a: Option<i32> = None;
    let b: Result<i32, &str> = a.ok_or("topilmadi");
    println!("{:?}", b);
    // Err("topilmadi")
}

// #================================================================================================================================================#
// # |  №  | Metod                    | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |   1 | Some(x)                  | Qiymat bor                                           | Есть значение                                        |
// # |   2 | None                     | Qiymat yo'q                                          | Значения нет                                         |
// # |   3 | is_some()                | Qiymat borligini tekshirish                          | Проверка наличия значения                            |
// # |   4 | is_none()                | Qiymat yo'qligini tekshirish                         | Проверка отсутствия значения                         |
// # |   5 | unwrap()                 | Qiymatni olish (None = panic!)                       | Получить значение (None = паника!)                   |
// # |   6 | expect()                 | unwrap + o'z xabar bilan                             | unwrap + своё сообщение об ошибке                    |
// # |   7 | unwrap_or()              | None bo'lsa default qaytarish                        | Вернуть default если None                            |
// # |   8 | unwrap_or_else()         | None bo'lsa closure ishlatish                        | Использовать замыкание если None                     |
// # |   9 | map()                    | Some bo'lsa qiymatni o'zgartirish                    | Преобразовать значение если Some                     |
// # |  10 | and_then()               | Some bo'lsa yangi Option qaytarish                   | Вернуть новый Option если Some                       |
// # |  11 | or()                     | None bo'lsa boshqa Option qaytarish                  | Вернуть другой Option если None                      |
// # |  12 | filter()                 | Shartga mos bo'lmasa None                            | None если условие не выполнено                       |
// # |  13 | take()                   | Qiymatni olish, o'zini None qilish                   | Взять значение, оставив None                         |
// # |  14 | replace()                | Yangi qiymat qo'yish, eskisini qaytarish             | Заменить значение, вернуть старое                    |
// # |  15 | zip()                    | Ikki Option ni birlashtirish                         | Объединить два Option в пару                         |
// # |  16 | flatten()                | Option<Option<T>> → Option<T>                        | Option<Option<T>> → Option<T>                        |
// # |  17 | if let Some(x)           | None ni ignore qilib Some ni olish                   | Получить Some игнорируя None                         |
// # |  18 | ? operatori              | None bo'lsa funksiyadan qaytish                      | Вернуться из функции если None                       |
// # |  19 | ok_or()                  | Option ni Result ga aylantirish                      | Преобразовать Option в Result                        |
// #================================================================================================================================================#