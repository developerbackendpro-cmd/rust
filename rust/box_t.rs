// #================================================================================================================================================#
// #                                                              BOX<T>                                                                            #
// #                            BOX — HEAP DA SAQLASH. OWNERSHIP BILAN. REKURSIV STRUKTURA VA DST UCHUN ZARUR.                                      #
// #                            BOX — ХРАНЕНИЕ В КУЧЕ. С ВЛАДЕНИЕМ. НЕОБХОДИМ ДЛЯ РЕКУРСИВНЫХ СТРУКТУР И DST.                                       #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// Box<T> nima:
// Что такое Box<T>:
//
//   - Heap da T ni saqlash uchun smart pointer
//   - Smart pointer для хранения T в куче
//   - Ownership semantikasi — Drop bo'lganda xotira ozod qilinadi
//   - Семантика владения — память освобождается при Drop
//   - Deref implement qilgan — *box → T
//   - Реализует Deref — *box → T
//   - 8 bayt (pointer o'lchami)
//   - 8 байт (размер указателя)
//
// Qachon kerak:
// Когда нужен:
//   1. Rekursiv ma'lumot strukturalari (LinkedList, Tree)
//      Рекурсивные структуры данных
//   2. DST — dyn Trait, [T] — stack da sig'maydi
//      DST — dyn Trait, [T] — не помещаются на стек
//   3. Katta ob'ektni heap da saqlash
//      Хранение большого объекта в куче
//   4. Ownership transferi uchun pointer
//      Указатель для передачи владения

fn box_asosiy_misollari() {

    // Box::new — heap da yaratish
    // Box::new — создание в куче
    let b: Box<i32> = Box::new(42);
    println!("{}", b);
    println!("{}", *b);
    // 42
    // 42

    // Deref — *b orqali ichki qiymat
    // Deref — внутреннее значение через *b
    let b2: Box<String> = Box::new(String::from("salom"));
    println!("{}", b2);
    println!("{}", b2.len()); // Deref coercion
    // salom
    // 5

    // Box — stack vs heap o'lchami
    // Box — размер на стеке vs куче
    println!("Box<i32>: {} bayt (stack)", std::mem::size_of::<Box<i32>>());
    println!("i32:      {} bayt (stack)", std::mem::size_of::<i32>());
    // Box<i32>: 8 bayt (stack) — faqat pointer!
    // i32:      4 bayt (stack)

    // Box — funksiyaga o'tkazish
    // Box — передача в функцию
    fn qiymat_ol(b: Box<i32>) -> i32 {
        *b
    }
    let b3: Box<i32> = Box::new(99);
    println!("{}", qiymat_ol(b3));
    // 99

    // Box — mut bilan
    // Box — с mut
    let mut b4: Box<i32> = Box::new(10);
    *b4 += 5;
    println!("{}", b4);
    // 15

    // Box<Vec<T>> — katta ma'lumot heap da
    // Box<Vec<T>> — большие данные в куче
    let katta: Box<Vec<i32>> = Box::new((0..1000).collect());
    println!("Uzunlik: {}", katta.len()); // Deref orqali
    // Uzunlik: 1000

    // Box::into_raw va Box::from_raw
    // Box::into_raw и Box::from_raw
    let b5: Box<i32> = Box::new(42);
    let raw: *mut i32 = Box::into_raw(b5);
    let b6: Box<i32> = unsafe { Box::from_raw(raw) };
    println!("{}", b6);
    // 42
}

fn box_rekursiv_struktura() {

    // Rekursiv struct — Box olmasa compile bo'lmaydi
    // Рекурсивная структура — без Box не компилируется
    // enum Ro'yxat { Bor(i32, Ro'yxat), Yoq }  ← XATO: o'lcham noaniq
    // enum Ro'yxat { Bor(i32, Box<Ro'yxat>), Yoq }  ← OK: pointer o'lchami aniq

    #[derive(Debug)]
    enum Royxat {
        Bor(i32, Box<Royxat>),
        Yoq,
    }

    impl Royxat {
        fn yig_indi(&self) -> i32 {
            match self {
                Royxat::Bor(n, keyingi) => n + keyingi.yig_indi(),
                Royxat::Yoq             => 0,
            }
        }
    }

    impl fmt::Display for Royxat {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Royxat::Bor(n, keyingi) => write!(f, "{} → {}", n, keyingi),
                Royxat::Yoq             => write!(f, "Yoq"),
            }
        }
    }

    let royxat = Royxat::Bor(1,
                             Box::new(Royxat::Bor(2,
                                                  Box::new(Royxat::Bor(3,
                                                                       Box::new(Royxat::Yoq))))));

    println!("{}", royxat);
    println!("Yig'indi: {}", royxat.yig_indi());
    // 1 → 2 → 3 → Yoq
    // Yig'indi: 6

    // Ikkilik daraxt — Box bilan
    // Двоичное дерево — с Box
    #[derive(Debug)]
    enum Daraxt {
        Tugun {
            qiymat: i32,
            chap: Box<Daraxt>,
            ong: Box<Daraxt>,
        },
        Barg,
    }

    impl Daraxt {
        fn barg() -> Box<Self> { Box::new(Daraxt::Barg) }
        fn tugun(qiymat: i32, chap: Box<Daraxt>, ong: Box<Daraxt>) -> Box<Self> {
            Box::new(Daraxt::Tugun { qiymat, chap, ong })
        }
        fn yig_indi(&self) -> i32 {
            match self {
                Daraxt::Tugun { qiymat, chap, ong } => qiymat + chap.yig_indi() + ong.yig_indi(),
                Daraxt::Barg => 0,
            }
        }
        fn balandlik(&self) -> usize {
            match self {
                Daraxt::Barg => 0,
                Daraxt::Tugun { chap, ong, .. } => 1 + chap.balandlik().max(ong.balandlik()),
            }
        }
    }

    //        4
    //       / \
    //      2   6
    //     / \
    //    1   3
    let daraxt = Daraxt::tugun(4,
                               Daraxt::tugun(2,
                                             Daraxt::tugun(1, Daraxt::barg(), Daraxt::barg()),
                                             Daraxt::tugun(3, Daraxt::barg(), Daraxt::barg()),
                               ),
                               Daraxt::tugun(6, Daraxt::barg(), Daraxt::barg()),
    );

    println!("Yig'indi: {}", daraxt.yig_indi());
    println!("Balandlik: {}", daraxt.balandlik());
    // Yig'indi: 16
    // Balandlik: 3
}

fn box_dyn_trait() {

    // Box<dyn Trait> — runtime dispatch
    // Box<dyn Trait> — диспетчеризация во время выполнения
    trait Shakl {
        fn yuza(&self) -> f64;
        fn nomi(&self) -> &str;
    }

    struct Doira { radius: f64 }
    struct Turtburchak { eni: f64, boyi: f64 }
    struct Uchburchak { asos: f64, balandlik: f64 }

    impl Shakl for Doira {
        fn yuza(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
        fn nomi(&self) -> &str { "Doira" }
    }
    impl Shakl for Turtburchak {
        fn yuza(&self) -> f64 { self.eni * self.boyi }
        fn nomi(&self) -> &str { "Turtburchak" }
    }
    impl Shakl for Uchburchak {
        fn yuza(&self) -> f64 { 0.5 * self.asos * self.balandlik }
        fn nomi(&self) -> &str { "Uchburchak" }
    }

    // Vec<Box<dyn Trait>> — turli xil ob'ektlar
    // Vec<Box<dyn Trait>> — объекты разных типов
    let shakllar: Vec<Box<dyn Shakl>> = vec![
        Box::new(Doira { radius: 5.0 }),
        Box::new(Turtburchak { eni: 4.0, boyi: 3.0 }),
        Box::new(Uchburchak { asos: 6.0, balandlik: 4.0 }),
    ];

    let mut jami_yuza: f64 = 0.0;
    for shakl in &shakllar {
        println!("{}: {:.2}", shakl.nomi(), shakl.yuza());
        jami_yuza += shakl.yuza();
    }
    println!("Jami: {:.2}", jami_yuza);
    // Doira: 78.54
    // Turtburchak: 12.00
    // Uchburchak: 12.00
    // Jami: 102.54

    // Box<dyn Fn> — closure saqlash
    // Box<dyn Fn> — хранение замыкания
    let operatsiyalar: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x * x),
    ];

    let mut qiymat: i32 = 3;
    for op in &operatsiyalar {
        qiymat = op(qiymat);
        print!("{} ", qiymat);
    }
    println!();
    // 4 8 64
}

fn box_dst_misollari() {

    // Box<str> — &str ga o'xshash lekin owned
    // Box<str> — похож на &str, но с владением
    let s: Box<str> = Box::from("salom dunyo");
    println!("{}", s);
    println!("{}", s.len());
    // salom dunyo
    // 11

    // Box<[T]> — owned slice
    // Box<[T]> — owned срез
    let arr: Box<[i32]> = Box::from([1, 2, 3, 4, 5].as_ref());
    println!("{:?}", arr);
    println!("{}", arr.len());
    // [1, 2, 3, 4, 5]
    // 5

    // Vec<T> → Box<[T]>
    // Vec<T> → Box<[T]>
    let v: Vec<i32> = vec![10, 20, 30];
    let boxed: Box<[i32]> = v.into_boxed_slice();
    println!("{:?}", boxed);
    // [10, 20, 30]

    // Box<dyn std::error::Error>
    // Box<dyn std::error::Error>
    fn xato_qaytaradi(muvaffaqiyatlimi: bool) -> Result<String, Box<dyn std::error::Error>> {
        if muvaffaqiyatlimi {
            Ok(String::from("Muvaffaqiyat"))
        } else {
            Err(Box::new("xato xabari".parse::<i32>().unwrap_err()))
        }
    }

    println!("{:?}", xato_qaytaradi(true));
    println!("{}", xato_qaytaradi(false).unwrap_err());
    // Ok("Muvaffaqiyat")
    // invalid digit found in string
}

fn real_hayot_misollari() {

    // 1. Plugin tizimi — Box<dyn Trait>
    // 1. Система плагинов — Box<dyn Trait>
    trait Plugin: fmt::Debug {
        fn nomi(&self) -> &str;
        fn ishla(&self, kiritish: &str) -> String;
    }

    #[derive(Debug)]
    struct KattaHarfPlugin;
    #[derive(Debug)]
    struct TeskariPlugin;
    #[derive(Debug)]
    struct TrimPlugin;

    impl Plugin for KattaHarfPlugin {
        fn nomi(&self) -> &str { "KattaHarf" }
        fn ishla(&self, k: &str) -> String { k.to_uppercase() }
    }
    impl Plugin for TeskariPlugin {
        fn nomi(&self) -> &str { "Teskari" }
        fn ishla(&self, k: &str) -> String { k.chars().rev().collect() }
    }
    impl Plugin for TrimPlugin {
        fn nomi(&self) -> &str { "Trim" }
        fn ishla(&self, k: &str) -> String { k.trim().to_string() }
    }

    let pluginlar: Vec<Box<dyn Plugin>> = vec![
        Box::new(TrimPlugin),
        Box::new(KattaHarfPlugin),
        Box::new(TeskariPlugin),
    ];

    let matn: &str = "  salom dunyo  ";
    let natija: String = pluginlar.iter().fold(matn.to_string(), |acc, p| {
        let chiqish = p.ishla(&acc);
        println!("[{}]: '{}'", p.nomi(), chiqish);
        chiqish
    });
    println!("Yakuniy: '{}'", natija);
    // [Trim]: 'salom dunyo'
    // [KattaHarf]: 'SALOM DUNYO'
    // [Teskari]: 'OYND MOLAS'
    // Yakuniy: 'OYND MOLAS'

    // 2. State pattern — Box<dyn State>
    // 2. Паттерн State — Box<dyn State>
    trait Holat: fmt::Debug {
        fn nomi(&self) -> &str;
        fn keyingi(self: Box<Self>) -> Box<dyn Holat>;
    }

    #[derive(Debug)] struct Yangi;
    #[derive(Debug)] struct Faol;
    #[derive(Debug)] struct Tugatilgan;

    impl Holat for Yangi {
        fn nomi(&self) -> &str { "Yangi" }
        fn keyingi(self: Box<Self>) -> Box<dyn Holat> { Box::new(Faol) }
    }
    impl Holat for Faol {
        fn nomi(&self) -> &str { "Faol" }
        fn keyingi(self: Box<Self>) -> Box<dyn Holat> { Box::new(Tugatilgan) }
    }
    impl Holat for Tugatilgan {
        fn nomi(&self) -> &str { "Tugatilgan" }
        fn keyingi(self: Box<Self>) -> Box<dyn Holat> { self }
    }

    let mut holat: Box<dyn Holat> = Box::new(Yangi);
    for _ in 0..4 {
        println!("Holat: {}", holat.nomi());
        holat = holat.keyingi();
    }
    // Holat: Yangi
    // Holat: Faol
    // Holat: Tugatilgan
    // Holat: Tugatilgan
}

fn main() {

    println!("=== BOX ASOSIY ===");
    box_asosiy_misollari();

    println!("\n=== REKURSIV STRUKTURA ===");
    box_rekursiv_struktura();

    println!("\n=== DYN TRAIT ===");
    box_dyn_trait();

    println!("\n=== DST ===");
    box_dst_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                  | Описание (RU)                                               |
// #================================================================================================================================================#
// # |   1 | Box::new(val)             | Heap da qiymat yaratish                       | Создание значения в куче                                   |
// # |   2 | *b                        | Deref — ichki qiymatga kirish                 | Deref — доступ к внутреннему значению                      |
// # |   3 | Box<dyn Trait>            | Runtime dispatch, turli turlar                | Диспетчеризация времени выполнения                         |
// # |   4 | Box<dyn Fn(T) -> U>       | Closure saqlash                               | Хранение замыкания                                         |
// # |   5 | Rekursiv enum/struct      | Box olmasa compile bo'lmaydi                  | Без Box не компилируется                                   |
// # |   6 | Box<str>                  | Owned &str                                    | Owned &str                                                 |
// # |   7 | Box<[T]>                  | Owned slice                                   | Owned срез                                                 |
// # |   8 | Vec::into_boxed_slice()   | Vec<T> → Box<[T]>                             | Vec<T> → Box<[T]>                                          |
// # |   9 | Box::into_raw()           | Raw pointer olish (unsafe)                    | Получение сырого указателя (unsafe)                        |
// # |  10 | Box::from_raw()           | Raw pointerdan Box (unsafe)                   | Box из сырого указателя (unsafe)                           |
// # |  11 | 8 bayt                    | Stack da faqat pointer                        | На стеке только указатель                                  |
// # |  12 | Drop                      | Scope tugaganda xotira avtomatik              | Память освобождается автоматически                         |
// #================================================================================================================================================#