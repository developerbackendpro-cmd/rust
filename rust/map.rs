// #================================================================================================================================================#
// #                                                        MAP  |  AND_THEN  |  OK_OR                                                              #
// #                                RESULT VA OPTION USTIDA ZANJIRLI OPERATSIYALAR. MATCH YOZMASDAN CHIROYLI KOD.                                   #
// #                                ЦЕПОЧЕЧНЫЕ ОПЕРАЦИИ НАД RESULT И OPTION. КРАСИВЫЙ КОД БЕЗ MATCH.                                                #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::num::ParseIntError;
use std::collections::HashMap;

fn option_metodlari() {

    // map() — Some ichidagi qiymatni o'zgartirish
    // map() — преобразование значения внутри Some
    // None bo'lsa — None qaytadi
    // если None — возвращает None
    let a: Option<i32> = Some(5);
    let b: Option<i32> = a.map(|x| x * 2);
    let c: Option<i32> = None;
    let d: Option<i32> = c.map(|x| x * 2);
    println!("{:?} {:?}", b, d);
    // Some(10) None

    // map() zanjiri
    // цепочка map()
    let natija: Option<String> = Some(42)
        .map(|x| x * 2)
        .map(|x| x + 1)
        .map(|x| x.to_string())
        .map(|s| format!("Natija: {}", s));
    println!("{:?}", natija);
    // Some("Natija: 85")

    // and_then() — flatMap — Option qaytaruvchi funksiya bilan
    // and_then() — flatMap — с функцией возвращающей Option
    // map()      → Option<Option<T>>  (ichki o'raladi)
    // and_then() → Option<T>          (yassilangan)
    let s: Option<&str> = Some("42");
    let n: Option<i32> = s.and_then(|x| x.parse::<i32>().ok());
    println!("{:?}", n);
    // Some(42)

    // and_then() zanjiri
    // цепочка and_then()
    let natija2: Option<i32> = Some("10")
        .and_then(|s| s.parse::<i32>().ok())
        .and_then(|n| if n > 0 { Some(n) } else { None })
        .and_then(|n| if n < 100 { Some(n * 2) } else { None });
    println!("{:?}", natija2);
    // Some(20)

    // or() — None bo'lsa alternativa qaytaradi
    // or() — при None возвращает альтернативу
    let x: Option<i32> = None;
    let y: Option<i32> = Some(5);
    println!("{:?}", x.or(Some(10)));
    println!("{:?}", y.or(Some(10)));
    // Some(10)
    // Some(5)

    // or_else() — lazy alternativa
    // or_else() — ленивая альтернатива
    let fallback: Option<i32> = None;
    let natija3 = fallback.or_else(|| {
        println!("Fallback chaqirildi");
        Some(99)
    });
    println!("{:?}", natija3);
    // Fallback chaqirildi
    // Some(99)

    // unwrap_or() — None bo'lsa default
    // unwrap_or() — при None default значение
    let val1: i32 = Some(42).unwrap_or(0);
    let val2: i32 = None.unwrap_or(0);
    println!("{} {}", val1, val2);
    // 42 0

    // unwrap_or_else() — lazy default
    // unwrap_or_else() — ленивое default значение
    let val3: i32 = None.unwrap_or_else(|| {
        println!("Default hisoblanmoqda");
        100
    });
    println!("{}", val3);
    // Default hisoblanmoqda
    // 100

    // unwrap_or_default() — Default trait orqali
    // unwrap_or_default() — через трейт Default
    let val4: i32 = None.unwrap_or_default();
    let val5: String = None.unwrap_or_default();
    let val6: Vec<i32> = None.unwrap_or_default();
    println!("{} '{}' {:?}", val4, val5, val6);
    // 0 '' []

    // filter() — shartni tekshirish
    // filter() — проверка условия
    let juft: Option<i32> = Some(4).filter(|x| x % 2 == 0);
    let toq: Option<i32> = Some(3).filter(|x| x % 2 == 0);
    println!("{:?} {:?}", juft, toq);
    // Some(4) None

    // flatten() — Option<Option<T>> → Option<T>
    // flatten() — Option<Option<T>> → Option<T>
    let ichki: Option<Option<i32>> = Some(Some(42));
    let yassi: Option<i32> = ichki.flatten();
    println!("{:?}", yassi);
    // Some(42)

    // zip() — ikki Option ni juftlashtirish
    // zip() — объединение двух Option
    let a2: Option<i32> = Some(1);
    let b2: Option<&str> = Some("salom");
    let juft2: Option<(i32, &str)> = a2.zip(b2);
    println!("{:?}", juft2);
    // Some((1, "salom"))

    // take() — qiymatni olib None qo'yish
    // take() — забрать значение, оставить None
    let mut opt: Option<i32> = Some(42);
    let olingan: Option<i32> = opt.take();
    println!("{:?} {:?}", olingan, opt);
    // Some(42) None

    // replace() — qiymatni almashtirish
    // replace() — замена значения
    let mut opt2: Option<i32> = Some(10);
    let eski: Option<i32> = opt2.replace(99);
    println!("{:?} {:?}", eski, opt2);
    // Some(10) Some(99)

    // get_or_insert() — None bo'lsa qiymat qo'yish
    // get_or_insert() — вставить значение если None
    let mut opt3: Option<i32> = None;
    let ref1: &mut i32 = opt3.get_or_insert(42);
    println!("{:?}", opt3);
    // Some(42)

    // as_ref() — Option<T> → Option<&T>
    // as_ref() — Option<T> → Option<&T>
    let s: Option<String> = Some(String::from("salom"));
    let s_ref: Option<&String> = s.as_ref();
    println!("{:?}", s_ref);
    println!("{:?}", s);  // s hali bor (move bo'lmadi)
    // Some("salom")
    // Some("salom")

    // is_some() va is_none()
    // is_some() и is_none()
    println!("{} {}", Some(1).is_some(), Some(1).is_none());
    println!("{} {}", None::<i32>.is_some(), None::<i32>.is_none());
    // true false
    // false true

    // ok_or() — Option → Result
    // ok_or() — Option → Result
    let opt_val: Option<i32> = Some(42);
    let none_val: Option<i32> = None;
    let r1: Result<i32, &str> = opt_val.ok_or("qiymat yo'q");
    let r2: Result<i32, &str> = none_val.ok_or("qiymat yo'q");
    println!("{:?} {:?}", r1, r2);
    // Ok(42) Err("qiymat yo'q")

    // ok_or_else() — lazy Err
    // ok_or_else() — ленивый Err
    let r3: Result<i32, String> = None.ok_or_else(|| {
        format!("Qiymat {}", "yo'q")
    });
    println!("{:?}", r3);
    // Err("Qiymat yo'q")
}

fn result_metodlari() {

    // map() — Ok ichidagi qiymatni o'zgartirish
    // map() — преобразование значения внутри Ok
    // Err bo'lsa — Err qaytadi
    // если Err — возвращает Err
    let ok: Result<i32, &str> = Ok(5);
    let err: Result<i32, &str> = Err("xato");
    let r1: Result<i32, &str> = ok.map(|x| x * 2);
    let r2: Result<i32, &str> = err.map(|x| x * 2);
    println!("{:?} {:?}", r1, r2);
    // Ok(10) Err("xato")

    // map() zanjiri
    // цепочка map()
    let natija: Result<String, &str> = Ok(21)
        .map(|x| x * 2)
        .map(|x| x.to_string())
        .map(|s| format!("[{}]", s));
    println!("{:?}", natija);
    // Ok("[42]")

    // map_err() — Err ichidagi xatoni o'zgartirish
    // map_err() — преобразование ошибки внутри Err
    let parse_result: Result<i32, ParseIntError> = "abc".parse();
    let mapped_err: Result<i32, String> = parse_result.map_err(|e| e.to_string());
    println!("{:?}", mapped_err);
    // Err("invalid digit found in string")

    // and_then() — Ok bo'lsa Result qaytaruvchi funksiya
    // and_then() — при Ok вызов функции возвращающей Result
    let r3: Result<i32, String> = "42"
        .parse::<i32>()
        .map_err(|e| e.to_string())
        .and_then(|n| if n > 0 { Ok(n) } else { Err("Manfiy".to_string()) })
        .and_then(|n| if n < 1000 { Ok(n * 2) } else { Err("Juda katta".to_string()) });
    println!("{:?}", r3);
    // Ok(84)

    // or() — Err bo'lsa alternativa
    // or() — при Err альтернатива
    let r4: Result<i32, &str> = Err("birinchi xato");
    let r5: Result<i32, &str> = r4.or(Ok(42));
    println!("{:?}", r5);
    // Ok(42)

    // or_else() — lazy alternativa
    // or_else() — ленивая альтернатива
    let r6: Result<i32, String> = Err("xato".to_string())
        .or_else(|e| {
            println!("Xato tuzatilmoqda: {}", e);
            Ok(0)
        });
    println!("{:?}", r6);
    // Xato tuzatilmoqda: xato
    // Ok(0)

    // unwrap_or() — Err bo'lsa default
    // unwrap_or() — при Err default значение
    let val1: i32 = Ok::<i32, &str>(42).unwrap_or(0);
    let val2: i32 = Err::<i32, &str>("xato").unwrap_or(0);
    println!("{} {}", val1, val2);
    // 42 0

    // unwrap_or_else() — lazy default
    // unwrap_or_else() — ленивое default значение
    let val3: i32 = Err::<i32, &str>("xato").unwrap_or_else(|e| {
        println!("Xato: {}", e);
        -1
    });
    println!("{}", val3);
    // Xato: xato
    // -1

    // ok() — Result → Option (Err → None)
    // ok() — Result → Option (Err → None)
    let opt1: Option<i32> = Ok::<i32, &str>(42).ok();
    let opt2: Option<i32> = Err::<i32, &str>("xato").ok();
    println!("{:?} {:?}", opt1, opt2);
    // Some(42) None

    // err() — Result → Option (Ok → None)
    // err() — Result → Option (Ok → None)
    let opt3: Option<&str> = Ok::<i32, &str>(42).err();
    let opt4: Option<&str> = Err::<i32, &str>("xato").err();
    println!("{:?} {:?}", opt3, opt4);
    // None Some("xato")

    // is_ok() va is_err()
    // is_ok() и is_err()
    println!("{} {}", Ok::<i32, &str>(1).is_ok(), Ok::<i32, &str>(1).is_err());
    println!("{} {}", Err::<i32, &str>("e").is_ok(), Err::<i32, &str>("e").is_err());
    // true false
    // false true

    // and() — ikki Ok bo'lsa ikkinchisini qaytaradi
    // and() — если оба Ok возвращает второй
    let r7: Result<&str, &str> = Ok(1).and(Ok("salom"));
    let r8: Result<&str, &str> = Err::<&str, &str>("xato").and(Ok("salom"));
    println!("{:?} {:?}", r7, r8);
    // Ok("salom") Err("xato")

    // flatten() — Result<Result<T,E>,E> → Result<T,E>
    // flatten() — Result<Result<T,E>,E> → Result<T,E>
    let ichki: Result<Result<i32, &str>, &str> = Ok(Ok(42));
    let yassi: Result<i32, &str> = ichki.flatten();
    println!("{:?}", yassi);
    // Ok(42)

    // map_or() — Ok bo'lsa map, aks holda default
    // map_or() — если Ok то map, иначе default
    let v1: i32 = Ok::<i32, &str>(5).map_or(0, |x| x * 2);
    let v2: i32 = Err::<i32, &str>("xato").map_or(0, |x| x * 2);
    println!("{} {}", v1, v2);
    // 10 0

    // map_or_else() — lazy
    // map_or_else() — ленивый
    let v3: String = Err::<i32, &str>("xato")
        .map_or_else(|e| format!("Xato: {}", e), |n| n.to_string());
    println!("{}", v3);
    // Xato: xato
}

fn real_hayot_misollari() {

    // 1. Foydalanuvchi ma'lumotlarini qayta ishlash
    // 1. Обработка данных пользователя
    let foydalanuvchilar: Vec<HashMap<&str, &str>> = vec![
        [("ism", "Dilshod"), ("yosh", "22"), ("email", "d@mail.com")].into_iter().collect(),
        [("ism", "Ali"),     ("yosh", "abc"), ("email", "a@mail.com")].into_iter().collect(),
        [("ism", "Vali"),    ("yosh", "30")].into_iter().collect(),
    ];

    for f in &foydalanuvchilar {
        let natija: Option<String> = f.get("ism")
            .and_then(|ism| f.get("yosh").map(|yosh| (*ism, *yosh)))
            .and_then(|(ism, yosh_str)| {
                yosh_str.parse::<u32>().ok().map(|yosh| (ism, yosh))
            })
            .and_then(|(ism, yosh)| {
                f.get("email").map(|email| {
                    format!("{} ({} yosh) <{}>", ism, yosh, email)
                })
            });
        match natija {
            Some(s) => println!("✅ {}", s),
            None    => println!("❌ Ma'lumot yetarli emas"),
        }
    }
    // ✅ Dilshod (22 yosh) <d@mail.com>
    // ❌ Ma'lumot yetarli emas
    // ❌ Ma'lumot yetarli emas

    // 2. Pipeline — Result zanjiri
    // 2. Pipeline — цепочка Result
    fn satr_dan_port(s: &str) -> Result<u16, String> {
        s.trim()
            .parse::<u16>()
            .map_err(|e| format!("Parse xato: {}", e))
            .and_then(|p| if p >= 1024 { Ok(p) }
            else { Err(format!("Tizim porti: {}", p)) })
    }

    println!("{:?}", satr_dan_port("8080"));
    println!("{:?}", satr_dan_port(" 3000 "));
    println!("{:?}", satr_dan_port("80"));
    println!("{:?}", satr_dan_port("abc"));
    // Ok(8080)
    // Ok(3000)
    // Err("Tizim porti: 80")
    // Err("Parse xato: invalid digit found in string")

    // 3. map va and_then farqi — amalda
    // 3. Разница map и and_then — на практике
    let s: &str = "42";

    // map — oddiy aylantirish (Result qaytarmaydi)
    // map — простое преобразование (не возвращает Result)
    let map_natija: Result<String, _> = s.parse::<i32>()
        .map(|n| n.to_string());
    println!("{:?}", map_natija);
    // Ok("42")

    // and_then — Result qaytaruvchi (flatMap)
    // and_then — возвращает Result (flatMap)
    let and_then_natija: Result<i32, _> = s.parse::<i32>()
        .and_then(|n| "10".parse::<i32>().map(|m| n + m));
    println!("{:?}", and_then_natija);
    // Ok(52)

    // 4. Fallback zanjiri — or_else
    // 4. Цепочка запасных вариантов — or_else
    fn manba1() -> Option<i32> { None }
    fn manba2() -> Option<i32> { None }
    fn manba3() -> Option<i32> { Some(42) }

    let natija2: Option<i32> = manba1()
        .or_else(manba2)
        .or_else(manba3)
        .map(|x| x * 2);
    println!("{:?}", natija2);
    // Some(84)
}

fn main() {

    println!("=== OPTION METODLARI ===");
    option_metodlari();

    println!("\n=== RESULT METODLARI ===");
    result_metodlari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Metod                    | Option uchun                         | Result uchun                                                         |
// #================================================================================================================================================#
// # |   1 | .map(|x| ...)            | Some(x) → Some(f(x)), None → None    | Ok(x) → Ok(f(x)), Err → Err                                          |
// # |   2 | .map_err(|e| ...)        | —                                    | Err(e) → Err(f(e)), Ok → Ok                                          |
// # |   3 | .and_then(|x| ...)       | Some(x) → f(x), None → None          | Ok(x) → f(x), Err → Err                                              |
// # |   4 | .or(alt)                 | None → alt, Some → Some              | Err → alt, Ok → Ok                                                   |
// # |   5 | .or_else(|| ...)         | None → f(), Some → Some              | Err → f(e), Ok → Ok                                                  |
// # |   6 | .unwrap_or(def)          | None → def, Some → val               | Err → def, Ok → val                                                  |
// # |   7 | .unwrap_or_else(|| ...)  | None → f(), Some → val               | Err → f(e), Ok → val                                                 |
// # |   8 | .unwrap_or_default()     | None → T::default(), Some → val      | —                                                                    |
// # |   9 | .filter(|x| ...)         | Some → None agar shart yolg'on       | —                                                                    |
// # |  10 | .flatten()               | Option<Option<T>> → Option<T>        | Result<Result<T,E>,E> → Result<T,E>                                  |
// # |  11 | .ok_or(err)              | None → Err(err), Some → Ok           | —                                                                    |
// # |  12 | .ok_or_else(|| err)      | None → Err(f()), Some → Ok           | —                                                                    |
// # |  13 | .ok()                    | —                                    | Err → None, Ok → Some                                                |
// # |  14 | .err()                   | —                                    | Ok → None, Err → Some                                                |
// # |  15 | .map_or(def, |x| ...)    | None → def, Some → f(x)              | Err → def, Ok → f(x)                                                 |
// # |  16 | .map_or_else(|e|, |x|)   | —                                    | Err → f(e), Ok → f(x)                                                |
// # |  17 | .is_some() / .is_none()  | Holat tekshirish                     | —                                                                    |
// # |  18 | .is_ok() / .is_err()     | —                                    | Holat tekshirish                                                     |
// # |  19 | .take()                  | Olib None qo'yish                    | —                                                                    |
// # |  20 | .zip(other)              | Ikki Option ni juftlashtirish        | —                                                                    |
// #================================================================================================================================================#