// #================================================================================================================================================#
// #                                                                   CLOSURES                                                                     #
// #                                CLOSURES — ATROF-MUHITNI QAMRAB OLUVCHI FUNKSIYALAR. FN, FNMUT, FNONCE TRAITLARI.                               #
// #                                CLOSURES — ФУНКЦИИ ЗАХВАТЫВАЮЩИЕ ОКРУЖЕНИЕ. ТРЕЙТЫ FN, FNMUT, FNONCE.                                           #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// Closure — funksiya + atrof-muhitdan o'zgaruvchilarni qamrab olish
// Closure — функция + захват переменных из окружения
//
// 3 xil capture:
// 3 вида захвата:
//   &T    — immutable borrow (FnOnce, FnMut, Fn)
//   &mut T — mutable borrow (FnOnce, FnMut)
//   T      — move/ownership (FnOnce)
//
// 3 xil trait:
// 3 вида трейта:
//   Fn     — ko'p marta, immutable capture
//   FnMut  — ko'p marta, mutable capture
//   FnOnce — bir marta, ownership capture

fn sintaksis_misollari() {

    // eng oddiy closure — parametrsiz
    // простейшее замыкание — без параметров
    let salom = || println!("Salom!");
    salom();
    // Salom!

    // bir parametrli closure
    // замыкание с одним параметром
    let kvadrat = |x: i32| x * x;
    let natija: i32 = kvadrat(5);
    println!("{}", natija);
    // 25

    // tur annotatsiyasiz — kompilyator aniqlaydi
    // без аннотации типа — компилятор определяет
    let ikki_baravar = |x| x * 2;
    let natija2: i32 = ikki_baravar(10);
    println!("{}", natija2);
    // 20

    // ko'p qatorli closure — blok bilan
    // многострочное замыкание — с блоком
    let abs_qiymat = |x: i32| {
        if x < 0 { -x } else { x }
    };
    println!("{}", abs_qiymat(-5));
    println!("{}", abs_qiymat(3));
    // 5
    // 3

    // closure va funksiya — farq
    // замыкание и функция — разница
    fn fn_kvadrat(x: i32) -> i32 { x * x }
    let cl_kvadrat = |x: i32| x * x;
    println!("{} {}", fn_kvadrat(4), cl_kvadrat(4));
    // 16 16

    // closure tur annotatsiyasi — to'liq yozilgan
    // полная аннотация типа замыкания
    let to_liq: fn(i32) -> i32 = |x: i32| -> i32 { x + 1 };
    println!("{}", to_liq(9));
    // 10
}

fn capture_misollari() {

    // &T capture — immutable borrow
    // захват &T — иммутабельное заимствование
    let x: i32 = 10;
    let qo_shish = |y| x + y;  // x ni borrow qildi
    println!("{}", qo_shish(5));
    println!("{}", x);  // x hali ishlatiladi
    // 15
    // 10

    // &mut T capture — mutable borrow
    // захват &mut T — мутабельное заимствование
    let mut hisob: i32 = 0;
    let mut oshirish = || { hisob += 1; hisob };
    println!("{}", oshirish());
    println!("{}", oshirish());
    println!("{}", oshirish());
    // 1
    // 2
    // 3

    // move — ownership capture
    // move — захват владения
    let matn: String = String::from("salom");
    let chiqaruvchi = move || println!("{}", matn);
    chiqaruvchi();
    // println!("{}", matn);  // xato! matn moved
    // salom

    // move — thread uchun zarur
    // move — необходимо для потоков
    let son: i32 = 42;
    let handle = std::thread::spawn(move || {
        println!("Thread: {}", son);
    });
    handle.join().unwrap();
    // Thread: 42

    // Copy turlar — avtomatik copy
    // Copy типы — автоматически копируются
    let n: i32 = 100;
    let f = move || println!("{}", n);
    f();
    println!("{}", n);  // n hali ishlatiladi (copy bo'ldi)
    // 100
    // 100
}

// Fn — ko'p marta chaqiriladi, &self (immutable capture)
// Fn — вызывается многократно, &self (иммутабельный захват)
fn fn_trait_misollari() {

    // Fn — oddiy closure
    // Fn — простое замыкание
    let x: i32 = 10;
    let qo_sh = |y| x + y;
    println!("{}", qo_sh(5));
    println!("{}", qo_sh(15));
    println!("{}", qo_sh(25));
    // 15
    // 25
    // 35

    // Fn — funksiya parametrida
    // Fn — в параметре функции
    fn ko_p_marta_chaqir<F: Fn(i32) -> i32>(f: F, n: i32) -> Vec<i32> {
        (0..n).map(|i| f(i)).collect()
    }
    let natijalar: Vec<i32> = ko_p_marta_chaqir(|x| x * x, 5);
    println!("{:?}", natijalar);
    // [0, 1, 4, 9, 16]

    // Fn — Box<dyn Fn>
    // Fn — Box<dyn Fn>
    let f: Box<dyn Fn(i32) -> i32> = Box::new(|x| x * 3);
    println!("{}", f(7));
    // 21
}

// FnMut — ko'p marta, &mut self (mutable capture)
// FnMut — многократно, &mut self (мутабельный захват)
fn fnmut_trait_misollari() {

    // FnMut — hisob yurituvchi
    // FnMut — счётчик
    let mut hisob: i32 = 0;
    let mut hisoblagich = || {
        hisob += 1;
        hisob
    };
    println!("{}", hisoblagich());
    println!("{}", hisoblagich());
    println!("{}", hisoblagich());
    // 1
    // 2
    // 3

    // FnMut — funksiya parametrida
    // FnMut — в параметре функции
    fn fnmut_qo_llash<F: FnMut(i32)>(mut f: F, qiymatlar: &[i32]) {
        for &q in qiymatlar {
            f(q);
        }
    }
    let mut yig_indi: i32 = 0;
    fnmut_qo_llash(|x| yig_indi += x, &[1, 2, 3, 4, 5]);
    println!("{}", yig_indi);
    // 15

    // FnMut — Vec ga qo'shish
    // FnMut — добавление в Vec
    let mut log: Vec<String> = Vec::new();
    let mut yozuvchi = |xabar: &str| {
        log.push(xabar.to_string());
    };
    yozuvchi("birinchi");
    yozuvchi("ikkinchi");
    yozuvchi("uchinchi");
    println!("{:?}", log);
    // ["birinchi", "ikkinchi", "uchinchi"]
}

// FnOnce — bir marta, self (ownership capture)
// FnOnce — один раз, self (захват владения)
fn fnonce_trait_misollari() {

    // FnOnce — owned qiymatni consume qiladi
    // FnOnce — потребляет owned значение
    let matn: String = String::from("salom");
    let consume = move || {
        println!("{}", matn);
        matn  // matn bu yerda consume bo'ladi
    };
    let qaytarilgan: String = consume();
    // consume();  // xato! ikkinchi marta chaqirib bo'lmaydi
    println!("{}", qaytarilgan);
    // salom
    // salom

    // FnOnce — funksiya parametrida
    // FnOnce — в параметре функции
    fn bir_marta_chaqir<F: FnOnce() -> String>(f: F) -> String {
        f()
    }
    let matn2: String = String::from("dunyo");
    let natija: String = bir_marta_chaqir(move || matn2);
    println!("{}", natija);
    // dunyo

    // FnOnce — Drop bilan
    // FnOnce — с Drop
    struct Resurs {
        nomi: String,
    }
    impl Drop for Resurs {
        fn drop(&mut self) {
            println!("{} o'chirildi", self.nomi);
        }
    }
    let r = Resurs { nomi: String::from("test_resurs") };
    let ishlatuvchi = move || {
        println!("{} ishlatildi", r.nomi);
    };
    ishlatuvchi();
    // test_resurs ishlatildi
    // test_resurs o'chirildi
}

// FnOnce ⊇ FnMut ⊇ Fn
// FnOnce — barcha closurelar (eng keng)
// FnMut  — mutable + immutable
// Fn     — faqat immutable (eng tor)
//
// Fn: FnMut
// FnMut: FnOnce
//
// Ya'ni:
// То есть:
//   Fn qabul qiluvchi joyga Fn, FnMut, FnOnce berilishi mumkin
//   FnMut qabul qiluvchi joyga FnMut, FnOnce berilishi mumkin
//   FnOnce qabul qiluvchi joyga istalgan closure berilishi mumkin

fn ierarxiya_misollari() {

    // Fn qabul qiluvchi — Fn berish
    // Принимает Fn — даём Fn
    fn fn_qabul<F: Fn()>(f: F) { f(); f(); }
    fn_qabul(|| println!("Fn"));
    // Fn
    // Fn

    // FnMut qabul qiluvchi — FnMut berish
    // Принимает FnMut — даём FnMut
    fn fnmut_qabul<F: FnMut()>(mut f: F) { f(); f(); }
    let mut n: i32 = 0;
    fnmut_qabul(|| { n += 1; println!("FnMut: {}", n); });
    // FnMut: 1
    // FnMut: 2

    // FnOnce qabul qiluvchi — istalgan closure
    // Принимает FnOnce — любое замыкание
    fn fnonce_qabul<F: FnOnce()>(f: F) { f(); }
    fnonce_qabul(|| println!("FnOnce"));
    // FnOnce
}

// impl Fn — closure qaytarish
// impl Fn — возврат замыкания
fn qo_shuvchi_yasash(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn ko_paytuvchi_yasash(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x * y
}

// impl FnMut — mutable closure qaytarish
// impl FnMut — возврат мутабельного замыкания
fn hisoblagich_yasash() -> impl FnMut() -> i32 {
    let mut hisob: i32 = 0;
    move || {
        hisob += 1;
        hisob
    }
}

// Box<dyn Fn> — dynamic closure qaytarish
// Box<dyn Fn> — возврат динамического замыкания
fn operatsiya_yasash(tur: &str) -> Box<dyn Fn(i32, i32) -> i32> {
    match tur {
        "qo'sh"    => Box::new(|a, b| a + b),
        "ayir"     => Box::new(|a, b| a - b),
        "ko'payt"  => Box::new(|a, b| a * b),
        _          => Box::new(|a, _| a),
    }
}

fn iterator_closure_misollari() {

    let sonlar: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map — har elementni o'zgartirish
    // map — преобразование каждого элемента
    let kvadratlar: Vec<i32> = sonlar.iter()
        .map(|&x| x * x)
        .collect();
    println!("{:?}", kvadratlar);
    // [1, 4, 9, 16, 25, 36, 49, 64, 81, 100]

    // filter — shartga mos elementlar
    // filter — элементы по условию
    let juftlar: Vec<&i32> = sonlar.iter()
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("{:?}", juftlar);
    // [2, 4, 6, 8, 10]

    // filter_map — filter + map birga
    // filter_map — filter + map вместе
    let natijalar: Vec<i32> = sonlar.iter()
        .filter_map(|&x| if x % 3 == 0 { Some(x * x) } else { None })
        .collect();
    println!("{:?}", natijalar);
    // [9, 36, 81]

    // fold — akkumulator bilan yig'ish
    // fold — свёртка с аккумулятором
    let yig_indi: i32 = sonlar.iter().fold(0, |acc, &x| acc + x);
    println!("{}", yig_indi);
    // 55

    // for_each — yon ta'sir uchun
    // for_each — для побочных эффектов
    let mut yig_indi2: i32 = 0;
    sonlar.iter().for_each(|&x| yig_indi2 += x);
    println!("{}", yig_indi2);
    // 55

    // take_while — shart bajarilguncha
    // take_while — пока выполняется условие
    let kichiklar: Vec<&i32> = sonlar.iter()
        .take_while(|&&x| x < 5)
        .collect();
    println!("{:?}", kichiklar);
    // [1, 2, 3, 4]

    // flat_map — yassilash bilan map
    // flat_map — map со сплющиванием
    let sozlar: Vec<&str> = vec!["salom dunyo", "rust tili"];
    let harflar: Vec<&str> = sozlar.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("{:?}", harflar);
    // ["salom", "dunyo", "rust", "tili"]

    // any va all
    // any и all
    println!("{}", sonlar.iter().any(|&x| x > 9));
    println!("{}", sonlar.iter().all(|&x| x > 0));
    // true
    // true

    // position va find
    // position и find
    let pos = sonlar.iter().position(|&x| x == 5);
    let topilgan = sonlar.iter().find(|&&x| x > 7);
    println!("{:?}", pos);
    println!("{:?}", topilgan);
    // Some(4)
    // Some(8)

    // chain — ikki iteratorni birlashtirish
    // chain — объединение двух итераторов
    let a: Vec<i32> = vec![1, 2, 3];
    let b: Vec<i32> = vec![4, 5, 6];
    let birga: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("{:?}", birga);
    // [1, 2, 3, 4, 5, 6]

    // sort_by — closure bilan tartiblash
    // sort_by — сортировка с замыканием
    let mut sozlar2: Vec<&str> = vec!["banan", "olma", "anor", "uzum"];
    sozlar2.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("{:?}", sozlar2);
    // ["olma", "anor", "uzum", "banan"]
}

// 1. Event handler — closure bilan
// 1. Обработчик событий — с замыканием
struct Tugma {
    bosildi_handler: Option<Box<dyn Fn()>>,
}

impl Tugma {
    fn new() -> Self {
        Tugma { bosildi_handler: None }
    }

    fn bosish_hodisasi<F: Fn() + 'static>(&mut self, handler: F) {
        self.bosildi_handler = Some(Box::new(handler));
    }

    fn bosish(&self) {
        if let Some(handler) = &self.bosildi_handler {
            handler();
        }
    }
}

// 2. Pipeline builder
// 2. Построитель pipeline
struct Pipeline<T> {
    qiymat: T,
}

impl<T> Pipeline<T> {
    fn new(qiymat: T) -> Self {
        Pipeline { qiymat }
    }

    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Pipeline<U> {
        Pipeline { qiymat: f(self.qiymat) }
    }

    fn tap<F: FnOnce(&T)>(self, f: F) -> Self {
        f(&self.qiymat);
        self
    }

    fn natija(self) -> T {
        self.qiymat
    }
}

// 3. Memoization — closure bilan
// 3. Мемоизация — с замыканием
struct Memoize<T, U> {
    funksiya: Box<dyn Fn(T) -> U>,
    kesh: std::collections::HashMap<String, U>,
}

impl<T: fmt::Debug, U: Clone> Memoize<T, U> {
    fn new<F: Fn(T) -> U + 'static>(f: F) -> Self {
        Memoize {
            funksiya: Box::new(f),
            kesh: std::collections::HashMap::new(),
        }
    }

    fn hisob(&mut self, kiritish: T) -> U
    where
        T: fmt::Debug,
    {
        let kalit: String = format!("{:?}", kiritish);
        if let Some(natija) = self.kesh.get(&kalit) {
            return natija.clone();
        }
        let natija: U = (self.funksiya)(kiritish);
        self.kesh.insert(kalit, natija.clone());
        natija
    }
}

// 4. Retry mexanizmi
// 4. Механизм повтора
fn retry<T, E, F>(mut f: F, urinishlar: u32) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut oxirgi_xato: Option<E> = None;
    for i in 0..urinishlar {
        match f() {
            Ok(natija) => return Ok(natija),
            Err(e)     => {
                println!("Urinish {} muvaffaqiyatsiz", i + 1);
                oxirgi_xato = Some(e);
            }
        }
    }
    Err(oxirgi_xato.unwrap())
}

fn main() {

    sintaksis_misollari();

    capture_misollari();

    fn_trait_misollari();

    fnmut_trait_misollari();

    fnonce_trait_misollari();

    ierarxiya_misollari();

    // impl Fn qaytarish
    // возврат impl Fn
    let beshga_qo_sh = qo_shuvchi_yasash(5);
    let uchga_ko_payt = ko_paytuvchi_yasash(3);
    println!("{}", beshga_qo_sh(10));
    println!("{}", uchga_ko_payt(7));
    // 15
    // 21

    // impl FnMut qaytarish
    // возврат impl FnMut
    let mut hisoblagich = hisoblagich_yasash();
    println!("{}", hisoblagich());
    println!("{}", hisoblagich());
    println!("{}", hisoblagich());
    // 1
    // 2
    // 3

    // Box<dyn Fn> qaytarish
    // возврат Box<dyn Fn>
    let qo_shish = operatsiya_yasash("qo'sh");
    let ko_paytirish = operatsiya_yasash("ko'payt");
    println!("{}", qo_shish(10, 5));
    println!("{}", ko_paytirish(10, 5));
    // 15
    // 50

    iterator_closure_misollari();

    // 1. Tugma handler
    // 1. Обработчик кнопки
    let mut tugma = Tugma::new();
    let hisoblagich_ref = std::sync::Arc::new(std::sync::Mutex::new(0));
    let hisoblagich_clone = hisoblagich_ref.clone();
    tugma.bosish_hodisasi(move || {
        let mut h = hisoblagich_clone.lock().unwrap();
        *h += 1;
        println!("Tugma bosildi! Hisob: {}", h);
    });
    tugma.bosish();
    tugma.bosish();
    // Tugma bosildi! Hisob: 1
    // Tugma bosildi! Hisob: 2

    // 2. Pipeline
    // 2. Pipeline
    let natija: String = Pipeline::new(vec![1, 2, 3, 4, 5])
        .tap(|v| println!("Boshlang'ich: {:?}", v))
        .map(|v| v.iter().map(|&x| x * 2).collect::<Vec<_>>())
        .tap(|v| println!("Ikki baravar: {:?}", v))
        .map(|v| v.iter().sum::<i32>())
        .tap(|&s| println!("Yig'indi: {}", s))
        .map(|s| format!("Natija: {}", s))
        .natija();
    println!("{}", natija);
    // Boshlang'ich: [1, 2, 3, 4, 5]
    // Ikki baravar: [2, 4, 6, 8, 10]
    // Yig'indi: 30
    // Natija: 30

    // 3. Retry
    // 3. Retry
    let mut urinish: i32 = 0;
    let natija2 = retry(|| {
        urinish += 1;
        if urinish < 3 {
            Err(format!("Urinish {} muvaffaqiyatsiz", urinish))
        } else {
            Ok(format!("Muvaffaqiyat! {} urinishda", urinish))
        }
    }, 5);
    println!("{:?}", natija2);
    // Urinish 1 muvaffaqiyatsiz
    // Urinish 2 muvaffaqiyatsiz
    // Ok("Muvaffaqiyat! 3 urinishda")
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                       CLOSURE SINTAKSISI                                                                                     |
// #================================================================================================================================================#
// # |   1 | || expr                  | Parametrsiz closure                                  | Замыкание без параметров                             |
// # |   2 | |x| expr                 | Bir parametrli closure                               | Замыкание с одним параметром                         |
// # |   3 | |x: T| -> U { ... }      | To'liq tur annotatsiyali closure                     | Замыкание с полной аннотацией типа                   |
// # |   4 | move || { ... }          | Ownership capture — qiymatlarni o'zlashtirish        | Захват владения — присвоение значений                |
// #================================================================================================================================================#
// # |                                       CAPTURE TURLARI                                                                                        |
// #================================================================================================================================================#
// # |   5 | &T (immutable borrow)    | Fn — o'zgarmaydi, ko'p marta                         | Fn — неизменяемо, многократно                        |
// # |   6 | &mut T (mutable borrow)  | FnMut — o'zgartiriladi, ko'p marta                   | FnMut — изменяемо, многократно                       |
// # |   7 | T (move/ownership)       | FnOnce — bir marta, consume qiladi                   | FnOnce — один раз, потребляет                        |
// # |   8 | Copy turlar              | move bilan ham copy bo'ladi                          | Копируются даже с move                               |
// #================================================================================================================================================#
// # |                                       FN TRAITLAR                                                                                            |
// #================================================================================================================================================#
// # |   9 | Fn                       | &self — ko'p marta, immutable capture                | &self — многократно, иммутабельный захват            |
// # |  10 | FnMut                    | &mut self — ko'p marta, mutable capture              | &mut self — многократно, мутабельный захват          |
// # |  11 | FnOnce                   | self — bir marta, ownership capture                  | self — один раз, захват владения                     |
// # |  12 | FnOnce ⊇ FnMut ⊇ Fn      | Ierarxiya — Fn eng tor, FnOnce eng keng              | Иерархия — Fn уже всех, FnOnce шире всех             |
// #================================================================================================================================================#
// # |                                       CLOSURE QAYTARISH                                                                                      |
// #================================================================================================================================================#
// # |  13 | -> impl Fn(T) -> U       | Closure qaytarish — static dispatch                  | Возврат замыкания — статическая диспетчеризация      |
// # |  14 | -> Box<dyn Fn(T) -> U>   | Closure qaytarish — dynamic dispatch                 | Возврат замыкания — динамическая диспетчеризация     |
// # |  15 | move || { ... }          | Qaytariluvchi closure — move shart                   | Возвращаемое замыкание — нужен move                  |
// #================================================================================================================================================#
// # |                                       ITERATOR METODLARI                                                                                     |
// #================================================================================================================================================#
// # |  16 | .map(|x| ...)            | Har elementni o'zgartirish                           | Преобразование каждого элемента                      |
// # |  17 | .filter(|x| ...)         | Shartga mos elementlar                               | Элементы по условию                                  |
// # |  18 | .filter_map(|x| ...)     | Filter + Map birga                                   | Filter + Map вместе                                  |
// # |  19 | .fold(acc, |acc, x| ...) | Akkumulator bilan yig'ish                            | Свёртка с аккумулятором                              |
// # |  20 | .for_each(|x| ...)       | Yon ta'sir uchun iteratsiya                          | Итерация для побочных эффектов                       |
// # |  21 | .sort_by(|a, b| ...)     | Closure bilan tartiblash                             | Сортировка с замыканием                              |
// #================================================================================================================================================#