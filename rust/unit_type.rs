// #================================================================================================================================================#
// #                                                              UNIT TYPE ()                                                                      #
// #                                    UNIT TYPE — BO'SH TUR. HECH NARSA QAYTARMAYDIGAN FUNKSIYA NATIJASI.                                         #
// #                                    UNIT TYPE — ПУСТОЙ ТИП. РЕЗУЛЬТАТ ФУНКЦИИ, НЕ ВОЗВРАЩАЮЩЕЙ НИЧЕГО.                                          #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

fn main() {

    // () — unit type, bo'sh qiymat
    // пустой тип, пустое значение
    let x: () = ();
    println!("{:?}", x);
    // ()

    // () — funksiya hech narsa qaytarmasa () qaytaradi
    // функция, не возвращающая ничего, возвращает ()
    fn salom() {
        println!("salom!");
    }
    let natija: () = salom();
    println!("{:?}", natija);
    // salom!
    // ()

    // () — return type ochiq yozilganda
    // явное указание возвращаемого типа ()
    fn hisob() -> () {
        println!("hisoblayapman...");
    }
    hisob();
    // hisoblayapman...

    // () — if/else natijasi () bo'lsa
    // результат if/else равен ()
    let son = 5;
    let natija = if son > 3 {
        println!("katta");
    } else {
        println!("kichik");
    };
    println!("{:?}", natija);
    // katta
    // ()

    // () — match arm () qaytarsa
    // ветка match возвращает ()
    let x = 2;
    let natija = match x {
        1 => println!("bir"),
        2 => println!("ikki"),
        _ => println!("boshqa"),
    };
    println!("{:?}", natija);
    // ikki
    // ()

    // () — loop break qiymatsiz
    // break без значения в loop
    let mut i = 0;
    let natija = loop {
        i += 1;
        if i == 3 {
            break;
        }
    };
    println!("{:?}", natija);
    // ()

    // () — Result<(), E> — muvaffaqiyat qaytarmaydigan funksiya
    // Result<(), E> — функция успешно завершилась, но ничего не возвращает
    fn faylga_yoz(matn: &str) -> Result<(), String> {
        if matn.is_empty() {
            return Err(String::from("matn bo'sh!"));
        }
        println!("yozildi: {}", matn);
        Ok(())
    }
    println!("{:?}", faylga_yoz("salom"));
    println!("{:?}", faylga_yoz(""));
    // yozildi: salom
    // Ok(())
    // Err("matn bo'sh!")

    // () — Option<()> — bor yoki yo'q, qiymat emas
    // Option<()> — есть или нет, а не значение
    fn tekshir(x: i32) -> Option<()> {
        if x > 0 {
            Some(())
        } else {
            None
        }
    }
    println!("{:?}", tekshir(5));
    println!("{:?}", tekshir(-1));
    // Some(())
    // None

    // () — HashMap<K, ()> — HashSet o'rnida
    // HashMap<K, ()> — вместо HashSet
    use std::collections::HashMap;
    let mut set: HashMap<&str, ()> = HashMap::new();
    set.insert("rust", ());
    set.insert("python", ());
    println!("{}", set.contains_key("rust"));
    println!("{}", set.contains_key("go"));
    // true
    // false

    // () — size_of::<()>() == 0 — xotirada joy olmaydi
    // size_of::<()>() == 0 — не занимает место в памяти
    println!("{}", std::mem::size_of::<()>());
    // 0

    // () — Vec<()> — zero-cost abstraction misoli
    // Vec<()> — пример zero-cost абстракции
    let mut v: Vec<()> = Vec::new();
    v.push(());
    v.push(());
    v.push(());
    println!("{}", v.len());
    // 3

    // () — closure () qaytarsa
    // замыкание, возвращающее ()
    let chop_et = |x: i32| {
        println!("qiymat: {}", x);
    };
    let natija: () = chop_et(42);
    println!("{:?}", natija);
    // qiymat: 42
    // ()

    // () — trait method () qaytarsa
    // метод трейта, возвращающий ()
    trait Chop {
        fn chop(&self) -> ();
    }
    struct Son(i32);
    impl Chop for Son {
        fn chop(&self) -> () {
            println!("son: {}", self.0);
        }
    }
    let s = Son(99);
    s.chop();
    // son: 99
}

// #================================================================================================================================================#
// # |  №  | Ishlatilish              | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |   1 | let x: () = ()           | Bo'sh qiymat yaratish                                | Создание пустого значения                            |
// # |   2 | fn f() { }               | Qaytarish yo'q = () qaytaradi                        | Нет возврата = возвращает ()                         |
// # |   3 | fn f() -> () { }         | () qaytarishni ochiq yozish                          | Явное указание возвращаемого типа ()                 |
// # |   4 | if/else natijasi ()      | Ikkala arm () qaytarsa natija ()                     | Результат () если обе ветки возвращают ()            |
// # |   5 | match natijasi ()        | Har bir arm () qaytarsa natija ()                    | Результат () если каждая ветка возвращает ()         |
// # |   6 | loop break;              | Qiymatsiz break — () qaytaradi                       | break без значения — возвращает ()                   |
// # |   7 | Result<(), E>            | Muvaffaqiyat, lekin qiymat yo'q                      | Успех, но значения нет                               |
// # |   8 | Option<()>               | Bor/yo'q holati, qiymat emas                         | Состояние есть/нет, а не значение                    |
// # |   9 | HashMap<K, ()>           | HashSet o'rnida ishlatish                            | Использование вместо HashSet                         |
// # |  10 | size_of::<()>() == 0     | Xotirada 0 bayt egallaydi                            | Занимает 0 байт в памяти                             |
// # |  11 | Vec<()>                  | Zero-cost, faqat uzunlik muhim                       | Zero-cost, важна только длина                        |
// # |  12 | closure () qaytarsa      | Closure natijasi () bo'lishi mumkin                  | Замыкание может возвращать ()                        |
// # |  13 | trait method () -> ()    | Trait metodlari () qaytarishi mumkin                 | Методы трейта могут возвращать ()                    |
// #================================================================================================================================================#