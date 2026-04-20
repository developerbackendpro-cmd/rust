// #================================================================================================================================================#
// #                                                           IMPL TRAIT                                                                           #
// #                     IMPL TRAIT — TRAITNI ANIQLASHTIRMASDAN QAYTARISH VA QABUL QILISH. STATIC DISPATCH. ZERO COST.                              #
// #                     IMPL TRAIT — ВОЗВРАТ И ПРИЁМ ТРЕЙТА БЕЗ УКАЗАНИЯ КОНКРЕТНОГО ТИПА. СТАТИЧЕСКАЯ ДИСПЕТЧЕРИЗАЦИЯ.                            #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// impl Trait ikki joyda ishlatiladi:
// impl Trait используется в двух местах:
//
//   1. Argument pozitsiyasi — "istalgan T: Trait qabul qil"
//      Позиция аргумента — "принять любой T: Trait"
//      fn f(x: impl Trait)  ≡  fn f<T: Trait>(x: T)
//
//   2. Return pozitsiyasi — "aniq tur aytmasdan Trait qaytarish"
//      Позиция возврата — "вернуть Trait без указания конкретного типа"
//      fn f() -> impl Trait
//
// impl Trait vs dyn Trait:
//   impl Trait — compile time, static dispatch, zero cost
//   impl Trait — время компиляции, статическая диспетчеризация, без затрат
//   dyn Trait  — runtime, dynamic dispatch, heap allocation, vtable
//   dyn Trait  — runtime, динамическая диспетчеризация, куча, vtable

fn impl_trait_argument_pozitsiya() {

    // impl Trait — argument sifatida
    // impl Trait — в качестве аргумента
    fn chiqarish(val: impl fmt::Display) {
        println!("{}", val);
    }

    chiqarish(42);
    chiqarish("salom");
    chiqarish(3.14);
    // 42
    // salom
    // 3.14

    // Generic bilan ekvivalent
    // Эквивалент с generic
    fn chiqarish_generic<T: fmt::Display>(val: T) {
        println!("{}", val);
    }
    // Ikkalasi bir xil — chiqarish va chiqarish_generic
    // Оба одинаковы — chiqarish и chiqarish_generic

    // Bir nechta trait bound
    // Несколько ограничений трейта
    fn clone_va_chiqarish(val: impl fmt::Display + Clone) -> String {
        let nusxa = val.clone();
        format!("{}", nusxa)
    }

    println!("{}", clone_va_chiqarish(42));
    println!("{}", clone_va_chiqarish("salom"));
    // 42
    // salom

    // impl Fn — closure argument
    // impl Fn — аргумент замыкания
    fn qo_lla(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }

    println!("{}", qo_lla(|x| x * 2, 5));
    println!("{}", qo_lla(|x| x + 10, 5));
    // 10
    // 15

    // impl Iterator — iterator argument
    // impl Iterator — аргумент итератора
    fn yig_indi(iter: impl Iterator<Item = i32>) -> i32 {
        iter.sum()
    }

    println!("{}", yig_indi(vec![1, 2, 3, 4, 5].into_iter()));
    println!("{}", yig_indi(1..=10));
    // 15
    // 55

    // impl IntoIterator — yanada keng
    // impl IntoIterator — ещё шире
    fn barcha_yig_indi(iter: impl IntoIterator<Item = i32>) -> i32 {
        iter.into_iter().sum()
    }

    println!("{}", barcha_yig_indi(vec![1, 2, 3]));
    println!("{}", barcha_yig_indi([10, 20, 30]));
    // 6
    // 60
}

fn impl_trait_return_pozitsiya() {

    // impl Trait — qaytarish turi sifatida
    // impl Trait — в качестве возвращаемого типа
    fn kvadrat_iterator() -> impl Iterator<Item = i32> {
        (1..=5).map(|x| x * x)
    }

    let v: Vec<i32> = kvadrat_iterator().collect();
    println!("{:?}", v);
    // [1, 4, 9, 16, 25]

    // Closure qaytarish — impl Fn
    // Возврат замыкания — impl Fn
    fn ko_paytuvchi(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x * n
    }

    let ikki_baravar = ko_paytuvchi(2);
    let uch_baravar = ko_paytuvchi(3);
    println!("{}", ikki_baravar(5));
    println!("{}", uch_baravar(5));
    // 10
    // 15

    // impl Display qaytarish
    // Возврат impl Display
    fn xush_xabar(muvaffaqiyatlimi: bool) -> impl fmt::Display {
        if muvaffaqiyatlimi {
            "Muvaffaqiyat!"
        } else {
            "Xato yuz berdi"
        }
        // Muhim: ikkalasi bir xil tur bo'lishi shart!
        // Важно: оба должны быть одного типа!
    }

    println!("{}", xush_xabar(true));
    println!("{}", xush_xabar(false));
    // Muvaffaqiyat!
    // Xato yuz berdi

    // Murakkab iterator zanjiri — impl Iterator
    // Сложная цепочка итераторов — impl Iterator
    fn juft_kvadratlar(limit: i32) -> impl Iterator<Item = i32> {
        (1..=limit)
            .filter(|x| x % 2 == 0)
            .map(|x| x * x)
    }

    let v2: Vec<i32> = juft_kvadratlar(10).collect();
    println!("{:?}", v2);
    // [4, 16, 36, 64, 100]

    // impl Fn qaytarish — adder
    // Возврат impl Fn — adder
    fn qo_shuvchi(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }

    let beshga_qo_sh = qo_shuvchi(5);
    let o_nga_qo_sh = qo_shuvchi(10);
    println!("{}", beshga_qo_sh(3));
    println!("{}", o_nga_qo_sh(3));
    // 8
    // 13
}

fn impl_trait_vs_dyn_trait() {

    trait Hisobla {
        fn hisob(&self) -> i32;
    }

    struct Qoshish(i32, i32);
    struct Kopaytirish(i32, i32);

    impl Hisobla for Qoshish {
        fn hisob(&self) -> i32 { self.0 + self.1 }
    }
    impl Hisobla for Kopaytirish {
        fn hisob(&self) -> i32 { self.0 * self.1 }
    }

    // impl Trait — static dispatch, compile time, zero cost
    // impl Trait — статическая диспетчеризация, compile time, без затрат
    fn static_hisob(h: impl Hisobla) -> i32 {
        h.hisob()
    }

    // dyn Trait — dynamic dispatch, runtime, vtable
    // dyn Trait — динамическая диспетчеризация, runtime, vtable
    fn dynamic_hisob(h: &dyn Hisobla) -> i32 {
        h.hisob()
    }

    let q = Qoshish(3, 4);
    let k = Kopaytirish(3, 4);

    println!("{}", static_hisob(Qoshish(3, 4)));   // 7
    println!("{}", static_hisob(Kopaytirish(3, 4)));// 12
    println!("{}", dynamic_hisob(&q));               // 7
    println!("{}", dynamic_hisob(&k));               // 12

    // impl Trait return — faqat bitta tur qaytarish mumkin
    // impl Trait return — можно вернуть только один тип
    // Bu kod KOMPILATSIYA BO'LMAYDI:
    // Этот код НЕ КОМПИЛИРУЕТСЯ:
    // fn noto_g_ri(musbat: bool) -> impl Hisobla {
    //     if musbat { Qoshish(1,1) } else { Kopaytirish(1,1) } // ← XATO!
    // }

    // Bu yaxshi — dyn Trait bilan:
    // Правильно — с dyn Trait:
    fn to_g_ri(musbat: bool) -> Box<dyn Hisobla> {
        if musbat { Box::new(Qoshish(1, 1)) }
        else       { Box::new(Kopaytirish(2, 3)) }
    }

    println!("{}", to_g_ri(true).hisob());   // 2
    println!("{}", to_g_ri(false).hisob()); // 6

    // O'lcham farqi
    // Разница в размере
    println!("impl Hisobla (Qoshish):    {} bayt", std::mem::size_of::<Qoshish>());
    println!("Box<dyn Hisobla>:           {} bayt", std::mem::size_of::<Box<dyn Hisobla>>());
    // impl Hisobla (Qoshish):    8 bayt (ikki i32)
    // Box<dyn Hisobla>:           16 bayt (ptr + vtable ptr)
}

fn impl_trait_murakkab_misollari() {

    // impl Trait — bir nechta qaytarish
    // impl Trait — несколько возвращаемых значений
    fn juft_va_toq(v: &[i32]) -> (impl Iterator<Item = &i32>, impl Iterator<Item = &i32>) {
        let juftlar = v.iter().filter(|&&x| x % 2 == 0);
        let toqlar  = v.iter().filter(|&&x| x % 2 != 0);
        (juftlar, toqlar)
    }

    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let (juftlar, toqlar) = juft_va_toq(&v);
    let j: Vec<&i32> = juftlar.collect();
    let t: Vec<&i32> = toqlar.collect();
    println!("Juftlar: {:?}", j);
    println!("Toqlar: {:?}", t);
    // Juftlar: [2, 4, 6, 8]
    // Toqlar: [1, 3, 5, 7]

    // impl Trait + lifetime
    // impl Trait + lifetime
    fn eng_uzun_soz<'a>(matn: &'a str) -> impl Iterator<Item = &'a str> {
        let max_uzunlik = matn.split_whitespace()
            .map(|s| s.len())
            .max()
            .unwrap_or(0);
        matn.split_whitespace().filter(move |s| s.len() == max_uzunlik)
    }

    let matn: &str = "salom ajoyib rust dunyo";
    let uzunlar: Vec<&str> = eng_uzun_soz(matn).collect();
    println!("{:?}", uzunlar);
    // ["ajoyib"]

    // Builder pattern — impl Trait qaytarish
    // Builder pattern — возврат impl Trait
    struct QueryBuilder {
        jadval: String,
        shart: Option<String>,
        limit: Option<usize>,
    }

    impl QueryBuilder {
        fn new(jadval: &str) -> Self {
            QueryBuilder { jadval: jadval.to_string(), shart: None, limit: None }
        }

        fn qayerda(mut self, shart: &str) -> Self {
            self.shart = Some(shart.to_string());
            self
        }

        fn limit(mut self, n: usize) -> Self {
            self.limit = Some(n);
            self
        }

        fn qur(&self) -> String {
            let mut so_rov = format!("SELECT * FROM {}", self.jadval);
            if let Some(ref s) = self.shart {
                so_rov.push_str(&format!(" WHERE {}", s));
            }
            if let Some(l) = self.limit {
                so_rov.push_str(&format!(" LIMIT {}", l));
            }
            so_rov
        }
    }

    let so_rov: String = QueryBuilder::new("foydalanuvchilar")
        .qayerda("yosh > 18")
        .limit(10)
        .qur();
    println!("{}", so_rov);
    // SELECT * FROM foydalanuvchilar WHERE yosh > 18 LIMIT 10
}

fn real_hayot_misollari() {

    // 1. Middleware pattern — impl Fn bilan
    // 1. Паттерн Middleware — с impl Fn
    fn middleware_qo_sh(
        handler: impl Fn(&str) -> String,
        middleware: impl Fn(String) -> String,
    ) -> impl Fn(&str) -> String {
        move |req| middleware(handler(req))
    }

    let asosiy = |req: &str| format!("Javob: {}", req);
    let log_mw = |resp: String| { println!("[LOG] {}", resp); resp };
    let katta_mw = |resp: String| resp.to_uppercase();

    let zanjir = middleware_qo_sh(asosiy, log_mw);
    let zanjir2 = middleware_qo_sh(
        |req: &str| format!("Javob: {}", req),
        katta_mw,
    );

    zanjir("GET /api");
    println!("{}", zanjir2("POST /data"));
    // [LOG] Javob: GET /api
    // JAVOB: POST /DATA

    // 2. Parser combinators — impl Fn bilan
    // 2. Комбинаторы парсеров — с impl Fn
    type ParseResult<'a, T> = Option<(T, &'a str)>;

    fn char_parser(c: char) -> impl Fn(&str) -> ParseResult<char> {
        move |input| {
            if input.starts_with(c) {
                Some((c, &input[c.len_utf8()..]))
            } else {
                None
            }
        }
    }

    fn digit_parser() -> impl Fn(&str) -> ParseResult<u32> {
        move |input| {
            let raqam = input.chars().next()?;
            if raqam.is_ascii_digit() {
                Some((raqam.to_digit(10)?, &input[1..]))
            } else {
                None
            }
        }
    }

    let parse_a = char_parser('a');
    let parse_digit = digit_parser();

    println!("{:?}", parse_a("abc"));
    println!("{:?}", parse_a("xyz"));
    println!("{:?}", parse_digit("42"));
    println!("{:?}", parse_digit("abc"));
    // Some(('a', "bc"))
    // None
    // Some((4, "2"))
    // None

    // 3. Iterator adapter — impl Iterator qaytarish
    // 3. Адаптер итератора — возврат impl Iterator
    fn oralik_qiymatlar(from: f64, to: f64, qadam: usize) -> impl Iterator<Item = f64> {
        let bosqich = (to - from) / qadam as f64;
        (0..=qadam).map(move |i| from + i as f64 * bosqich)
    }

    let qiymatlar: Vec<f64> = oralik_qiymatlar(0.0, 1.0, 4)
        .map(|x| (x * 100.0).round() / 100.0)
        .collect();
    println!("{:?}", qiymatlar);
    // [0.0, 0.25, 0.5, 0.75, 1.0]
}

fn main() {

    println!("=== ARGUMENT POZITSIYA ===");
    impl_trait_argument_pozitsiya();

    println!("\n=== RETURN POZITSIYA ===");
    impl_trait_return_pozitsiya();

    println!("\n=== IMPL vs DYN ===");
    impl_trait_vs_dyn_trait();

    println!("\n=== MURAKKAB MISOLLAR ===");
    impl_trait_murakkab_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                  | Tavsif (UZ)                               | Описание (RU)                                              |
// #================================================================================================================================================#
// # |   1 | fn f(x: impl Trait)           | Argument — istalgan T: Trait              | Аргумент — любой T: Trait                                  |
// # |   2 | fn f() -> impl Trait          | Return — aniq tur aytmasdan               | Возврат — без указания конкретного типа                    |
// # |   3 | fn f<T:Trait>(x:T) ≡ impl T   | Generic va impl Trait ekvivalent          | Generic и impl Trait эквивалентны                          |
// # |   4 | impl Fn(T) -> U               | Closure argument yoki return              | Аргумент или возврат замыкания                             |
// # |   5 | impl Iterator<Item=T>         | Iterator argument yoki return             | Аргумент или возврат итератора                             |
// # |   6 | impl Display + Clone          | Bir nechta bound                          | Несколько ограничений                                      |
// # |   7 | Static dispatch               | Compile time — zero cost                  | Compile time — без затрат                                  |
// # |   8 | Bitta tur qaytarish           | Return da faqat bitta tur                 | Только один тип в возврате                                 |
// # |   9 | impl vs dyn                   | impl — static, dyn — dynamic (vtable)     | impl — статический, dyn — динамический                     |
// # |  10 | dyn — turli tur qaytarish     | Box<dyn Trait> — turli turlar             | Box<dyn Trait> — разные типы                               |
// #================================================================================================================================================#