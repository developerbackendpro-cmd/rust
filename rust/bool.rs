// #================================================================================================================================================#
// #                                                                        BOOL                                                                    #
// #                                    BOOL — MANTIQIY TUR. FAQAT true YOKI false. SHART TEKSHIRISHNING ASOSI.                                     #
// #                                    BOOL — ЛОГИЧЕСКИЙ ТИП. ТОЛЬКО true ИЛИ false. ОСНОВА ПРОВЕРКИ УСЛОВИЙ.                                      #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

fn main() {

    // bool — true yoki false
    // true или false
    let a: bool = true;
    let b: bool = false;
    println!("{} {}", a, b);
    // true false

    // size_of::<bool>() == 1 bayt
    // занимает 1 байт
    println!("{}", std::mem::size_of::<bool>());
    // 1

    // bool — if shartida
    // в условии if
    let x = true;
    if x {
        println!("rost!");
    }
    // rost!

    // && — va (AND) — ikkalasi ham true bo'lsa true
    // И (AND) — true только если оба true
    println!("{}", true && true);
    println!("{}", true && false);
    // true
    // false

    // || — yoki (OR) — bittasi true bo'lsa true
    // ИЛИ (OR) — true если хотя бы один true
    println!("{}", false || true);
    println!("{}", false || false);
    // true
    // false

    // ! — inkor (NOT) — teskarisi
    // НЕ (NOT) — противоположное
    println!("{}", !true);
    println!("{}", !false);
    // false
    // true

    // & — bitwise AND (short-circuit yo'q)
    // побитовое И (без короткого замыкания)
    println!("{}", true & false);
    // false

    // | — bitwise OR (short-circuit yo'q)
    // побитовое ИЛИ (без короткого замыкания)
    println!("{}", false | true);
    // true

    // ^ — XOR — faqat bittasi true bo'lsa true
    // XOR — true только если ровно один true
    println!("{}", true ^ false);
    println!("{}", true ^ true);
    // true
    // false

    // == != < > <= >=
    println!("{}", 5 == 5);
    println!("{}", 5 != 3);
    println!("{}", 3 < 5);
    println!("{}", 5 > 3);
    println!("{}", 5 >= 5);
    println!("{}", 3 <= 5);
    // true true true true true true

    // .then(|| value) — true bo'lsa Some(value), false bo'lsa None
    // Some(value) если true, None если false
    let x = true;
    println!("{:?}", x.then(|| 42));
    let y = false;
    println!("{:?}", y.then(|| 42));
    // Some(42)
    // None

    // .then_some(value) — then() ning soddaroq versiyasi
    // упрощённая версия then()
    println!("{:?}", true.then_some("topildi"));
    println!("{:?}", false.then_some("topildi"));
    // Some("topildi")
    // None

    // .then() — closure bilan amaliyot
    // с замыканием для выполнения действия
    let admin = true;
    let ruxsat = admin.then(|| {
        println!("kirish ruxsat etildi");
        "admin panel"
    });
    println!("{:?}", ruxsat);
    // kirish ruxsat etildi
    // Some("admin panel")

    // bool → i32 (as bilan)
    // bool → i32 (через as)
    println!("{}", true as i32);
    println!("{}", false as i32);
    // 1
    // 0

    // bool → u8
    // bool → u8
    println!("{}", true as u8);
    println!("{}", false as u8);
    // 1
    // 0

    // i32 → bool (to'g'ridan to'g'ri yo'q, shart kerak)
    // i32 → bool (напрямую нельзя, нужно условие)
    let n = 1;
    let b = n != 0;
    println!("{}", b);
    // true

    // && — chap false bo'lsa o'ng tekshirilmaydi
    // если левая часть false — правая не вычисляется
    fn chap() -> bool { println!("chap"); false }
    fn ong() -> bool { println!("ong"); true }
    let natija = chap() && ong();
    println!("{}", natija);
    // chap         ← ong() chaqirilmadi!
    // false

    // || — chap true bo'lsa o'ng tekshirilmaydi
    // если левая часть true — правая не вычисляется
    fn a_fn() -> bool { println!("a"); true }
    fn b_fn() -> bool { println!("b"); false }
    let natija = a_fn() || b_fn();
    println!("{}", natija);
    // a            ← b_fn() chaqirilmadi!
    // true

    // filter() bilan
    // с filter()
    let v = vec![1, 2, 3, 4, 5, 6];
    let juftlar: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).cloned().collect();
    println!("{:?}", juftlar);
    // [2, 4, 6]

    // all() — hammasi shartga mos kelsa true
    // true если все элементы соответствуют условию
    let v = vec![2, 4, 6, 8];
    println!("{}", v.iter().all(|&x| x % 2 == 0));
    // true

    // any() — bittasi shartga mos kelsa true
    // true если хотя бы один элемент соответствует условию
    let v = vec![1, 3, 4, 7];
    println!("{}", v.iter().any(|&x| x % 2 == 0));
    // true

    // bool — struct fieldda
    // bool в поле структуры
    struct Foydalanuvchi {
        ism: &'static str,
        faol: bool,
        admin: bool,
    }
    let f = Foydalanuvchi { ism: "Dilshod", faol: true, admin: false };
    if f.faol && !f.admin {
        println!("{} oddiy foydalanuvchi", f.ism);
    }
    // Dilshod oddiy foydalanuvchi

    // bool — match bilan
    // bool с match
    let ulangan = true;
    match ulangan {
        true  => println!("ulanish bor"),
        false => println!("ulanish yo'q"),
    }
    // ulanish bor

    // bool — ternary o'rnida (Rust da yo'q, if expression bor)
    // вместо тернарного оператора (в Rust нет, есть if выражение)
    let yoshmi = true;
    let xabar = if yoshmi { "yosh" } else { "keksa" };
    println!("{}", xabar);
    // yosh

    // bool — Vec<bool>
    // Vec<bool>
    let permissions = vec![true, false, true, true, false];
    let ruxsat_soni = permissions.iter().filter(|&&x| x).count();
    println!("{}", ruxsat_soni);
    // 3

    // bool — Default
    // bool Default = false
    let b: bool = Default::default();
    println!("{}", b);
    // false

    // bool — fmt bilan turli formatlar
    // различные форматы вывода
    println!("{}", true);
    println!("{:?}", true);
    println!("{:#?}", false);
    // true
    // true
    // false
}

// #================================================================================================================================================#
// # |  №  | Metod / Operator         | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                          OPERATORLAR                                                                                         |
// #================================================================================================================================================#
// # |   1 | &&                       | VA — ikkalasi true bo'lsa true (short-circuit)       | И — true если оба true (с коротким замыканием)       |
// # |   2 | ||                       | YOKI — bittasi true bo'lsa true (short-circuit)      | ИЛИ — true если один true (с коротким замыканием)    |
// # |   3 | !                        | INKOR — teskarisi                                    | НЕ — противоположное значение                        |
// # |   4 | &                        | Bitwise AND (short-circuit yo'q)                     | Побитовое И (без короткого замыкания)                |
// # |   5 | |                        | Bitwise OR (short-circuit yo'q)                      | Побитовое ИЛИ (без короткого замыкания)              |
// # |   6 | ^                        | XOR — faqat bittasi true bo'lsa true                 | XOR — true только если ровно один true               |
// #================================================================================================================================================#
// # |                                           METODLAR                                                                                           |
// #================================================================================================================================================#
// # |   7 | .then(|| value)          | true bo'lsa Some(value), false bo'lsa None           | Some(value) если true, None если false               |
// # |   8 | .then_some(value)        | then() ning soddaroq versiyasi                       | Упрощённая версия then()                             |
// #================================================================================================================================================#
// # |                                        TUR O'ZGARTIRISH                                                                                      |
// #================================================================================================================================================#
// # |   9 | true as i32 → 1          | bool → i32 (true=1, false=0)                         | bool → i32 (true=1, false=0)                         |
// # |  10 | true as u8 → 1           | bool → u8                                            | bool → u8                                            |
// # |  11 | n != 0                   | i32 → bool (shart orqali)                            | i32 → bool (через условие)                           |
// #================================================================================================================================================#
// # |                                        ITERATOR METODLAR                                                                                     |
// #================================================================================================================================================#
// # |  12 | .filter(|x| koʻp)        | Shartga mos elementlarni olish                       | Получить элементы по условию                         |
// # |  13 | .all(|x| shart)          | Hammasi shartga mosmi?                               | Все ли элементы соответствуют условию?               |
// # |  14 | .any(|x| shart)          | Bittasi shartga mosmi?                               | Хотя бы один соответствует условию?                  |
// #================================================================================================================================================#
// # |                                          QOLGANLAR                                                                                           |
// #================================================================================================================================================#
// # |  15 | Default::default()       | bool default qiymati = false                         | Значение по умолчанию bool = false                   |
// # |  16 | size_of::<bool>() == 1   | Xotirada 1 bayt egallaydi                            | Занимает 1 байт в памяти                             |
// # |  17 | match true/false         | Pattern matching bilan ishlatish                     | Использование с pattern matching                     |
// # |  18 | if bool { } else { }     | Ternary o'rniga if expression                        | if выражение вместо тернарного оператора             |
// #================================================================================================================================================#