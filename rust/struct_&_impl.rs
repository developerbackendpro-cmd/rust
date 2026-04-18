// #================================================================================================================================================#
// #                                                           STRUCT + IMPL                                                                        #
// #                                    Struct — bog'liq ma'lumotlarni guruhlash uchun maxsus type                                                  #
// #                                    Struct — пользовательский тип для группировки связанных данных                                              #
// #================================================================================================================================================#
// #                                    Struct — asosiy tushunchalar / Основные концепции                                                           #
// #                                    struct    — maydonlar (fields) bilan ma'lumot turi yaratish                                                 #
// #                                    impl      — struct ga method va funksiyalar qo'shish                                                        #
// #                                    &self     — immutable reference (faqat o'qish)                                                              #
// #                                    &mut self — mutable reference (o'zgartirish)                                                                #
// #                                    Self      — o'z typiga ishora                                                                               #
// #                                    new()     — constructor (associated function)                                                               #
// #================================================================================================================================================#

#![allow(dead_code, unused_variables)]

fn main() {

    // Oddiy struct — maydonlar bilan
    // Простая структура с полями
    struct Odam {
        ism: String,
        yosh: u32,
        faol: bool,
    }
    let o = Odam {
        ism: String::from("Dilshod"),
        yosh: 25,
        faol: true,
    };
    println!("{} {} {}", o.ism, o.yosh, o.faol);
    // Dilshod 25 true

    // Mutable struct — o'zgartirish
    // Изменяемая структура
    let mut o = Odam {
        ism: String::from("Dilshod"),
        yosh: 25,
        faol: true,
    };
    o.yosh = 26;
    println!("{}", o.yosh);
    // 26

    // Struct update syntax — boshqa struct dan nusxa
    // Синтаксис обновления структуры
    let o1 = Odam {
        ism: String::from("Dilshod"),
        yosh: 25,
        faol: true,
    };
    let o2 = Odam {
        ism: String::from("Jasur"),
        ..o1 // qolgan maydonlar o1 dan
    };
    println!("{} {}", o2.ism, o2.yosh);
    // Jasur 25

    // Tuple struct — nomsiz maydonlar
    // Кортежная структура — безымянные поля
    struct Rang(u8, u8, u8);
    let qizil = Rang(255, 0, 0);
    println!("{} {} {}", qizil.0, qizil.1, qizil.2);
    // 255 0 0

    // Unit struct — bo'sh struct
    // Единичная структура — пустая
    struct Marker;
    let _m = Marker;

    // #[derive] — avtomatik traitlar
    // Автоматические трейты
    #[derive(Debug, Clone, PartialEq)]
    struct Nuqta {
        x: f64,
        y: f64,
    }
    let n1 = Nuqta { x: 1.0, y: 2.0 };
    let n2 = n1.clone();
    println!("{:?}", n1);        // Nuqta { x: 1.0, y: 2.0 }
    println!("{}", n1 == n2);    // true

    // impl — method qo'shish
    // Добавление методов через impl
    struct Aylana {
        radius: f64,
    }
    impl Aylana {
        // Associated function — new() constructor
        // Ассоциированная функция — конструктор
        fn new(radius: f64) -> Self {
            Aylana { radius }
        }

        // &self — immutable method (faqat o'qish)
        // &self — неизменяемый метод
        fn yuza(&self) -> f64 {
            3.14 * self.radius * self.radius
        }

        // &self — perimetr
        fn perimetr(&self) -> f64 {
            2.0 * 3.14 * self.radius
        }

        // &mut self — mutable method (o'zgartirish)
        // &mut self — изменяемый метод
        fn kattalashtir(&mut self, miqdor: f64) {
            self.radius += miqdor;
        }

        // self — ownership olish (consume)
        // self — поглощение (consume)
        fn radius_ni_ol(self) -> f64 {
            self.radius
        }
    }

    let mut a = Aylana::new(5.0);
    println!("{:.2}", a.yuza());      // 78.50
    println!("{:.2}", a.perimetr());  // 31.40
    a.kattalashtir(2.0);
    println!("{:.2}", a.yuza());      // 113.04
    let r = a.radius_ni_ol();
    println!("{}", r);                // 7
    // a endi yo'q — ownership o'tdi!

    // Bir nechta impl blok — mumkin
    // Несколько блоков impl — допустимо
    struct Tortburchak {
        kenglik: f64,
        balandlik: f64,
    }
    impl Tortburchak {
        fn new(k: f64, b: f64) -> Self {
            Tortburchak { kenglik: k, balandlik: b }
        }
        fn yuza(&self) -> f64 {
            self.kenglik * self.balandlik
        }
    }
    impl Tortburchak {
        fn perimetr(&self) -> f64 {
            2.0 * (self.kenglik + self.balandlik)
        }
        fn kvadratmi(&self) -> bool {
            self.kenglik == self.balandlik
        }
    }

    let t = Tortburchak::new(4.0, 6.0);
    println!("{:.1}", t.yuza());       // 24.0
    println!("{:.1}", t.perimetr());   // 20.0
    println!("{}", t.kvadratmi());     // false

    // Struct ichida struct
    // Структура внутри структуры
    #[derive(Debug)]
    struct Manzil {
        shahar: String,
        ko_cha: String,
    }
    #[derive(Debug)]
    struct Xodim {
        ism: String,
        yosh: u32,
        manzil: Manzil,
    }
    let x = Xodim {
        ism: String::from("Dilshod"),
        yosh: 25,
        manzil: Manzil {
            shahar: String::from("Toshkent"),
            ko_cha: String::from("Amir Temur"),
        },
    };
    println!("{} — {}", x.ism, x.manzil.shahar);
    // Dilshod — Toshkent

    // Destructuring — struct ni ajratish
    // Деструктуризация структуры
    let Nuqta { x, y } = Nuqta { x: 3.0, y: 4.0 };
    println!("{} {}", x, y);
    // 3 4

    // Display trait — chiroyli chiqarish
    // Display trait — красивый вывод
    use std::fmt;
    struct Talaba {
        ism: String,
        ball: f64,
    }
    impl fmt::Display for Talaba {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} ({})", self.ism, self.ball)
        }
    }
    let t = Talaba { ism: String::from("Dilshod"), ball: 95.5 };
    println!("{}", t);
    // Dilshod (95.5)
}
// #================================================================================================================================================#
// # |  №  | Mavzu                      | Tavsif (UZ)                                             | Описание (RU)                                   |
// #================================================================================================================================================#
// # |   1 | struct Ism { ... }         | Maydonlar bilan struct yaratish                         | Создание структуры с полями                     |
// # |   2 | let mut s = Struct { }     | Mutable struct                                          | Изменяемая структура                            |
// # |   3 | ..other                    | Update syntax — qolgan maydonlarni nusxalash            | Синтаксис обновления — копирование полей        |
// # |   4 | struct Rang(u8, u8, u8)    | Tuple struct — nomsiz maydonlar                         | Кортежная структура — безымянные поля           |
// # |   5 | struct Marker              | Unit struct — bo'sh                                     | Единичная структура — пустая                    |
// # |   6 | #[derive(Debug, Clone)]    | Avtomatik traitlar                                      | Автоматические трейты                           |
// # |   7 | fn new() -> Self           | Constructor (associated function)                       | Конструктор (ассоциированная функция)           |
// # |   8 | fn f(&self)                | Immutable method — faqat o'qish                         | Неизменяемый метод — только чтение              |
// # |   9 | fn f(&mut self)            | Mutable method — o'zgartirish                           | Изменяемый метод — изменение                    |
// # |  10 | fn f(self)                 | Consuming method — ownership olish                      | Поглощающий метод — забирает владение           |
// # |  11 | impl Blok { ... }          | Method qo'shish, bir nechta impl mumkin                 | Добавление методов, несколько impl допустимо    |
// # |  12 | struct ichida struct       | Nested struct                                           | Вложенная структура                             |
// # |  13 | let Struct { x, y } = s    | Destructuring                                           | Деструктуризация                                |
// # |  14 | impl Display for Struct    | {} bilan chiqarish                                      | Вывод через {}                                  |
// #================================================================================================================================================#