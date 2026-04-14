// #================================================================================================================================================#
// #                                                           BORROWING & REFERENCES                                                               #
// #                                      Borrowing — qiymatni egasidan vaqtincha olish, ownership o'tmasdan                                        #
// #                                      Borrowing — временное использование значения без передачи владения                                        #
// #================================================================================================================================================#
// #                                    Borrowing — 3 ta temir qoida / Borrowing — 3 железных правила                                               #
// #            1. Istalgancha &T (immutable) reference olsa bo'ladi, lekin bir vaqtda &mut T bo'lsa — boshqa reference yo'q                        #
// #                Можно иметь сколько угодно &T ссылок, но если есть &mut T — других ссылок нет                                                   #
// #            2. Bir vaqtda faqat BITTA &mut T reference bo'lishi mumkin                                                                          #
// #                В одно время может существовать только ОДНА изменяемая ссылка &mut T                                                            #
// #            3. Reference doimo yaroqli bo'lishi kerak (dangling reference yo'q)                                                                 #
// #                Ссылка всегда должна быть действительной (висячих ссылок нет)                                                                   #
// #================================================================================================================================================#

fn main() {

    // &T — immutable reference (faqat o'qish)
    // &T — неизменяемая ссылка (только чтение)
    let s = String::from("salom");
    let r = &s; // s dan reference oldik — s hali tirik!
    println!("{} {}", s, r);
    // salom salom

    // Funksiyaga reference berish — ownership o'tmaydi
    // Передача ссылки в функцию — владение не переходит
    fn uzunlik(s: &String) -> usize {
        s.len()
    }
    let s = String::from("salom");
    let len = uzunlik(&s); // &s — reference beramiz
    println!("{} = {} harf", s, len); // s hali tirik!
    // salom = 5 harf

    // Bir nechta immutable reference — mumkin!
    // Несколько неизменяемых ссылок — можно!
    let s = String::from("salom");
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("{} {} {}", r1, r2, r3);
    // salom salom salom

    // &mut T — mutable reference (o'zgartirish)
    // &mut T — изменяемая ссылка (изменение)
    let mut s = String::from("salom");
    let r = &mut s;
    r.push_str(" dunyo");
    println!("{}", s);
    // salom dunyo

    // Bir vaqtda faqat 1 ta &mut reference!
    // Только одна &mut ссылка в одно время!
    let mut s = String::from("salom");
    let r1 = &mut s;
    // let r2 = &mut s; // XATO! ikkinchi &mut bo'lmaydi
    println!("{}", r1);
    // salom

    // &T va &mut T bir vaqtda bo'lmaydi!
    // &T и &mut T не могут существовать одновременно!
    let s = String::from("salom");
    let r1 = &s;     // immutable
    let r2 = &s;     // immutable — OK
    // let r3 = &mut s; // XATO! &T va &mut T birga bo'lmaydi
    println!("{} {}", r1, r2);
    // salom salom

    // Dangling reference — Rust qo'ymaydi!
    // Висячая ссылка — Rust не допускает!
    // fn yaratma() -> &String { // XATO!
    //     let s = String::from("salom");
    //     &s // s scope dan chiqadi — reference yaroqsiz!
    // }

    // To'g'ri: ownership qaytarish
    // Правильно: возврат владения
    fn yarat() -> String {
        String::from("salom") // ownership qaytaradi
    }
    let s = yarat();
    println!("{}", s);
    // salom

    // Slice — reference ning maxsus turi
    // Slice — особый вид ссылки
    let s = String::from("salom dunyo");
    let salom = &s[0..5];   // "salom"
    let dunyo = &s[6..11];  // "dunyo"
    println!("{} {}", salom, dunyo);
    // salom dunyo

    // &str — string slice (doim reference)
    // &str — срез строки (всегда ссылка)
    let s = String::from("salom");
    let slice: &str = &s[0..3];
    println!("{}", slice);
    // sal

    // Array slice
    // Срез массива
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("{:?}", slice);
    // [2, 3, 4]

    // Funksiyada &str ishlatish — kuchli usul
    // Использование &str в функции — мощный способ
    fn birinchi_soz(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }
    let s = String::from("salom dunyo");
    let soz = birinchi_soz(&s);
    println!("{}", soz);
    // salom
}
// #=============================================================================================================================================================#
// # |  №  | Mavzu / Qoida              | Tavsif (UZ)                                                  | Описание (RU)                                           |
// #=============================================================================================================================================================#
// # |   1 | &T (immutable reference)   | Faqat o'qish, ownership o'tmaydi                             | Только чтение, владение не переходит                    |
// # |   2 | &mut T (mutable reference) | O'zgartirish mumkin, bir vaqtda faqat bitta                  | Можно изменять, только одна в одно время                |
// # |   3 | Bir nechta &T              | Istalgancha immutable reference olsa bo'ladi                 | Можно иметь сколько угодно неизменяемых ссылок          |
// # |   4 | &T + &mut T birga yo'q     | Bir vaqtda &T va &mut T bo'lmaydi                            | &T и &mut T не могут существовать одновременно          |
// # |   5 | Dangling reference yo'q    | Rust yaroqsiz reference yaratishga yo'l qo'ymaydi            | Rust не допускает висячих ссылок                        |
// # |   6 | Slice (&[T])               | Collection ning bir qismiga reference                        | Ссылка на часть коллекции                               |
// # |   7 | &str                       | String slice — doim reference, stack da                      | Срез строки — всегда ссылка, на стеке                   |
// # |   8 | Funksiyaga &T berish       | ownership o'tmaydi, faqat vaqtincha ishlatadi                | Владение не переходит, только временное использование   |
// # |   9 | Aliasing XOR Mutability    | O'qish ko'p bo'lsa — yozish yo'q, yozish bo'lsa — o'qish yo'q| Много читателей ИЛИ один писатель — не оба вместе       |
// #=============================================================================================================================================================#