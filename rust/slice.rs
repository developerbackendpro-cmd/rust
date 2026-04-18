// #================================================================================================================================================#
// #                                                              SLICES                                                                            #
// #                                    Slice — collection ning bir qismiga reference (ownership o'tmaydi)                                          #
// #                                    Slice — ссылка на часть коллекции (без передачи владения)                                                   #
// #================================================================================================================================================#
// #                                    Slice — asosiy tushunchalar / Основные концепции Slice                                                      #
// #                                    &[T]   — array/vec slice (ixtiyoriy type)                                                                   #
// #                                    &str   — string slice (doim reference)                                                                      #
// #                                    [start..end]   — start dan end-1 gacha (end kirmaydi)                                                       #
// #                                    [start..=end]  — start dan end gacha (end kiradi)                                                           #
// #                                    [..end]        — boshidan end-1 gacha                                                                       #
// #                                    [start..]      — start dan oxirigacha                                                                       #
// #                                    [..]           — hammasi                                                                                    #
// #================================================================================================================================================#

fn main() {

    // String slice — &str
    // Срез строки — &str
    let s = String::from("salom dunyo");
    let salom = &s[0..5];
    let dunyo = &s[6..11];
    println!("{} {}", salom, dunyo);
    // salom dunyo

    // ..= inclusive slice
    // включительный срез
    let s = String::from("salom");
    let sl = &s[0..=2]; // 0,1,2 — 3 ta harf
    println!("{}", sl);
    // sal

    // boshidan kesish
    // срез с начала
    let s = String::from("salom dunyo");
    let sl = &s[..5]; // 0..5 bilan bir xil
    println!("{}", sl);
    // salom

    // oxirigacha kesish
    // срез до конца
    let s = String::from("salom dunyo");
    let sl = &s[6..]; // 6 dan oxirigacha
    println!("{}", sl);
    // dunyo

    // hammasi
    // весь срез
    let s = String::from("salom");
    let sl = &s[..]; // hammasini olish
    println!("{}", sl);
    // salom

    // Array slice — &[T]
    // Срез массива
    let arr = [1, 2, 3, 4, 5];
    let sl = &arr[1..4];
    println!("{:?}", sl);
    // [2, 3, 4]

    // Vec slice
    // Срез вектора
    let v = vec![10, 20, 30, 40, 50];
    let sl = &v[2..];
    println!("{:?}", sl);
    // [30, 40, 50]

    // Funksiyaga slice berish — &str kuchli usul
    // Передача среза в функцию — &str мощный способ
    fn uzunlik(s: &str) -> usize {
        s.len()
    }
    let s = String::from("salom");
    println!("{}", uzunlik(&s));    // String dan
    println!("{}", uzunlik("salom")); // &str dan — ikkalasi ishlaydi!
    // 5
    // 5

    // Funksiyaga &[i32] berish
    // Передача &[i32] в функцию
    fn yig_indi(sl: &[i32]) -> i32 {
        sl.iter().sum()
    }
    let arr = [1, 2, 3, 4, 5];
    let v = vec![1, 2, 3, 4, 5];
    println!("{}", yig_indi(&arr));    // array dan
    println!("{}", yig_indi(&v));      // vec dan — ikkalasi ishlaydi!
    // 15
    // 15

    // Slice dan birinchi so'z topish
    // Поиск первого слова в срезе
    fn birinchi_soz(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &b) in bytes.iter().enumerate() {
            if b == b' ' { return &s[0..i]; }
        }
        &s[..]
    }
    let s = String::from("salom dunyo");
    println!("{}", birinchi_soz(&s));
    // salom

    // Slice len va is_empty
    // Длина и пустота среза
    let arr = [1, 2, 3];
    let sl = &arr[1..];
    println!("{}", sl.len());      // 2
    println!("{}", sl.is_empty()); // false

    // Slice contains
    // Содержит ли срез элемент
    let arr = [1, 2, 3, 4, 5];
    let sl = &arr[0..3];
    println!("{}", sl.contains(&2)); // true
    println!("{}", sl.contains(&5)); // false

    // Slice iter
    // Итерация по срезу
    let arr = [10, 20, 30, 40, 50];
    let sl = &arr[1..4];
    for x in sl.iter() {
        print!("{} ", x);
    }
    println!();
    // 20 30 40

    // Mutable slice — &mut [T]
    // Изменяемый срез
    let mut arr = [1, 2, 3, 4, 5];
    let sl = &mut arr[0..3];
    sl[0] = 100;
    println!("{:?}", arr);
    // [100, 2, 3, 4, 5]

    // Slice sort
    // Сортировка среза
    let mut arr = [3, 1, 4, 1, 5, 9];
    let sl = &mut arr[0..4];
    sl.sort();
    println!("{:?}", arr);
    // [1, 1, 3, 4, 5, 9]
}
// #================================================================================================================================================#
// # |  №  | Mavzu                    | Tavsif (UZ)                                              | Описание (RU)                                    |
// #================================================================================================================================================#
// # |   1 | &str                     | String ning bir qismiga reference                        | Ссылка на часть строки                           |
// # |   2 | &[T]                     | Array/Vec ning bir qismiga reference                     | Ссылка на часть массива/вектора                  |
// # |   3 | [start..end]             | start dan end-1 gacha (end kirmaydi)                     | От start до end-1 (end не включается)            |
// # |   4 | [start..=end]            | start dan end gacha (end kiradi)                         | От start до end включительно                     |
// # |   5 | [..end]                  | Boshidan end-1 gacha                                     | От начала до end-1                               |
// # |   6 | [start..]                | start dan oxirigacha                                     | От start до конца                                |
// # |   7 | [..]                     | Butun collection                                         | Весь массив/строка                               |
// # |   8 | fn f(s: &str)            | &str ham String ham qabul qiladi                         | Принимает как &str, так и String                 |
// # |   9 | fn f(s: &[i32])          | Array ham Vec ham qabul qiladi                           | Принимает как массив, так и Vec                  |
// # |  10 | &mut [T]                 | Mutable slice — o'zgartirish mumkin                      | Изменяемый срез                                  |
// # |  11 | .len() / .is_empty()     | Uzunlik va bo'shligini tekshirish                        | Длина и проверка на пустоту                      |
// # |  12 | .contains(&x)            | Element bormi?                                           | Содержит ли элемент?                             |
// # |  13 | .iter()                  | Elementlar bo'ylab aylanish                              | Итерация по элементам                            |
// # |  14 | .sort()                  | Mutable slice ni tartiblash                              | Сортировка изменяемого среза                     |
// #================================================================================================================================================#