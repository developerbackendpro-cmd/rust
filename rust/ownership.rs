// #================================================================================================================================================#
// #                                                              OWNERSHIP                                                                         #
// #                               Har bir qiymatning bitta egasi bor Ego scope dan chiqsa — qiymat o'chadi                                         #
// #                    У каждого значения есть один владелец Когда владелец выходит из области видимости — значение удаляется                      #
// #================================================================================================================================================#
// #                                    Ownership — 3 ta temir qoida / Владение — 3 железных правила                                                #
// #         1. Har qiymatning bitta egasi bor / У каждого значения есть один владелец                                                              #
// #         2. Ego scope dan chiqsa → qiymat o'chadi (Drop) / Когда владелец выходит из области видимости (scope) → значение удаляется (Drop)      #
// #         3. Move → eski o'zgaruvchi yo'qoladi / При перемещении (Move) → старая переменная исчезает                                             #
// #================================================================================================================================================#
// #                                                            Stack vs Heap                                                                       #
// #                                            i32, f64, bool → stack → Copy  (ko'chiriladi)                                                       #
// #                                            String, Vec    → heap  → Move  (ownership o'tadi)                                                   #
// #================================================================================================================================================#
// #                                      Nima uchun Ownership kerak / Зачем нужно владение (Ownership)                                             #
// #                                                 Python → Garbage Collector (sekin)                                                             #
// #                                                 C/C++  → qo'lda xotira (xavfli)                                                                #
// #                                                 Rust   → Ownership (tez + xavfsiz!)                                                            #
// #================================================================================================================================================#

fn main() {

    // QOIDA 1: Har bir qiymatning bitta egasi bor
    // ПРАВИЛО 1: У каждого значения один владелец
    let s = String::from("salom"); // s — egasi
    println!("{}", s);
    // salom

    // QOIDA 2: Move — ownership boshqa o'zgaruvchiga o'tadi
    // ПРАВИЛО 2: Move — владение переходит к другой переменной
    let s1 = String::from("salom");
    let s2 = s1; // s1 dan s2 ga o'tdi — s1 endi yo'q!
    // println!("{}", s1); // XATO! s1 moved
    println!("{}", s2);
    // salom

    // QOIDA 3: Scope tugagach qiymat o'chadi (Drop)
    // ПРАВИЛО 3: Значение удаляется при выходе из области видимости
    {
        let s = String::from("vaqtinchalik");
        println!("{}", s);
    }
    // s bu yerda o'chadi — Drop chaqiriladi
    // println!("{}", s); // XATO! s yo'q
    // vaqtinchalik

    // Copy — stack da saqlanadigan typlar ko'chiriladi
    // Copy — типы на стеке копируются, а не перемещаются
    let x = 5;
    let y = x; // x ko'chirildi — x hali tirik!
    println!("{} {}", x, y);
    // 5 5

    // Copy bo'luvchi typlar
    // Типы с трейтом Copy
    let a: i32 = 10;
    let b = a;      // copy
    let c: f64 = 3.14;
    let d = c;      // copy
    let e: bool = true;
    let f = e;      // copy
    println!("{} {} {} {} {} {}", a, b, c, d, e, f);
    // 10 10 3.14 3.14 true true

    // String — Copy emas, Move bo'ladi
    // String — не Copy, происходит Move
    let s1 = String::from("salom");
    let s2 = s1;    // move — s1 yo'q!
    println!("{}", s2);
    // salom

    // Clone — chuqur nusxa olish
    // Clone — глубокое копирование
    let s1 = String::from("salom");
    let s2 = s1.clone(); // s1 ham, s2 ham tirik!
    println!("{} {}", s1, s2);
    // salom salom

    // Funksiyaga berish — ownership o'tadi
    // Передача в функцию — владение переходит
    fn iste_mol_qil(s: String) {
        println!("{}", s);
    }
    // s bu yerda o'chadi
    // salom

    let s = String::from("salom");
    iste_mol_qil(s); // s ownership o'tdi
    // println!("{}", s); // XATO! s moved

    // Funksiyaga berish — Copy type
    // Передача Copy-типа в функцию
    fn ikki_kat(x: i32) -> i32 {
        x * 2
    }
    let x = 5;
    let y = ikki_kat(x);
    println!("{} {}", x, y);
    // 5 10 — x hali tirik!

    // Funksiyadan qaytarish — ownership qaytadi
    // Возврат из функции — владение возвращается
    fn yarat() -> String {
        String::from("yangi string") // ownership tashqariga chiqadi
    }
    let s = yarat();
    println!("{}", s);
    // yangi string

    // Ownership qaytarib olish — noqulay usul
    // Неудобный способ вернуть владение
    fn uzunlik(s: String) -> (String, usize) {
        let len = s.len();
        (s, len) // s ni qaytaramiz
    }
    let s = String::from("salom");
    let (s, len) = uzunlik(s);
    println!("{} = {} harf", s, len);
    // salom = 5 harf
    // Bu noqulay — shuning uchun Borrowing bor!
}
// #=============================================================================================================================================================#
// # |  №  | Mavzu / Qoida             | Tavsif (UZ)                                               | Описание (RU)                                               |
// #=============================================================================================================================================================#
// # |   1 | Qoida 1                   | Har qiymatning bitta egasi bor                            | У каждого значения есть один владелец                       |
// # |   2 | Qoida 2                   | Ego scope dan chiqsa → qiymat o'chadi (Drop)              | Когда владелец выходит из scope → значение удаляется (Drop) |
// # |   3 | Qoida 3                   | Move → eski o'zgaruvchi yo'qoladi                         | При перемещении (Move) → старая переменная исчезает         |
// # |   4 | Stack vs Heap (Copy)      | i32, f64, bool → stack → Copy (ko'chiriladi)              | i32, f64, bool → стек → Copy (копируется)                   |
// # |   5 | Stack vs Heap (Move)      | String, Vec → heap → Move (ownership o'tadi)              | String, Vec → куча → Move (владение переходит)              |
// # |   6 | Move                      | s1 dan s2 ga o'tdi — s1 endi yo'q!                        | Из s1 в s2 — s1 больше не существует!                       |
// # |   7 | Copy                      | x = 5; y = x → ikkalasi ham tirik!                        | x = 5; y = x → оба живы!                                    |
// # |   8 | Clone                     | s1.clone() — chuqur nusxa, ikkalasi ham tirik             | s1.clone() — глубокая копия, оба живы                       |
// # |   9 | Funksiyaga berish (Move)  | iste_mol_qil(s) → s ownership o'tdi, endi yo'q            | iste_mol_qil(s) → владение ушло, s больше нет               |
// # |  10 | Funksiyaga berish (Copy)  | ikki_kat(x) → x hali tirik (Copy trait)                   | ikki_kat(x) → x все еще жив (Copy trait)                    |
// # |  11 | Funksiyadan qaytarish     | yarat() → String qaytaradi, ownership tashqariga chiqadi  | yarat() → возвращает String, владение выходит наружу        |
// # |  12 | Ownership qaytarib olish  | (s, len) usuli — noqulay! Borrowing kerak                 | Способ (s, len) — неудобно! Нужен Borrowing                 |
// #=============================================================================================================================================================#