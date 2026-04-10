// #================================================================================================================================================#
// #                                                        loop / while / for SIKLLAR                                                              #
// #                    loop  — cheksiz sikl, break bilan to'xtatiladi / loop  — бесконечный цикл, останавливается через break                      #
// #                                    while — shart bajarilgunga qadar / while — пока условие выполняется                                         #
// #                                    for   — collection bo'ylab aylanish / for   — итерация по коллекции                                         #
// #================================================================================================================================================#

fn main() {

    // loop — cheksiz sikl
    // бесконечный цикл
    let mut i = 0;
    loop {
        i += 1;
        if i == 3 { break; }
        println!("{}", i);
    }
    // 1
    // 2

    // loop — qiymat qaytaradi (break bilan)
    // возврат значения из loop
    let mut i = 0;
    let natija = loop {
        i += 1;
        if i == 5 { break i * 2; }
    };
    println!("{}", natija);
    // 10

    // while — shart bajarilgunga qadar
    // пока условие истинно
    let mut i = 0;
    while i < 3 {
        println!("{}", i);
        i += 1;
    }
    // 0
    // 1
    // 2

    // while let — Option tugaguncha
    // пока есть значение
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    // 3
    // 2
    // 1

    // for — range bo'ylab
    // по диапазону
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();
    // 0 1 2 3 4

    // for — range inclusive
    // включительный диапазон
    for i in 0..=5 {
        print!("{} ", i);
    }
    println!();
    // 0 1 2 3 4 5

    // for — Vec bo'ylab
    // по вектору
    let v = vec![10, 20, 30];
    for x in &v {
        println!("{}", x);
    }
    // 10
    // 20
    // 30

    // for — Vec ni o'zgartirish (&mut)
    // изменение элементов вектора
    let mut v = vec![1, 2, 3];
    for x in &mut v {
        *x *= 2;
    }
    println!("{:?}", v);
    // [2, 4, 6]

    // for — Vec ownership (o'zi)
    // владение вектором
    let v = vec![1, 2, 3];
    for x in v {
        println!("{}", x);
    }
    // v endi yo'q — ownership o'tdi!

    // for — enumerate (index + qiymat)
    // индекс и значение вместе
    let v = vec!["a", "b", "c"];
    for (i, x) in v.iter().enumerate() {
        println!("{}: {}", i, x);
    }
    // 0: a
    // 1: b
    // 2: c

    // for — Array bo'ylab
    // по массиву
    let arr = [1, 2, 3, 4, 5];
    for x in arr.iter() {
        print!("{} ", x);
    }
    println!();
    // 1 2 3 4 5

    // for — String harflari bo'ylab
    // по символам строки
    let s = String::from("salom");
    for c in s.chars() {
        print!("{} ", c);
    }
    println!();
    // s a l o m

    // for — HashMap bo'ylab
    // по HashMap
    use std::collections::HashMap;
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }
    // a: 1
    // b: 2

    // for — zip (ikki collection birga)
    // два итератора вместе
    let a = vec![1, 2, 3];
    let b = vec!["bir", "ikki", "uch"];
    for (x, y) in a.iter().zip(b.iter()) {
        println!("{} = {}", x, y);
    }
    // 1 = bir
    // 2 = ikki
    // 3 = uch

    // for — filter bilan
    // с фильтрацией
    let v = vec![1, 2, 3, 4, 5, 6];
    for x in v.iter().filter(|&&x| x % 2 == 0) {
        print!("{} ", x);
    }
    println!();
    // 2 4 6

    // for — map bilan
    // с преобразованием
    let v = vec![1, 2, 3];
    for x in v.iter().map(|&x| x * 10) {
        print!("{} ", x);
    }
    println!();
    // 10 20 30

    // for — step_by (qadam bilan)
    // с шагом
    for i in (0..10).step_by(3) {
        print!("{} ", i);
    }
    println!();
    // 0 3 6 9

    // for — rev (teskari)
    // в обратном порядке
    for i in (0..5).rev() {
        print!("{} ", i);
    }
    println!();
    // 4 3 2 1 0

    // for — take (N ta olish)
    // взять N элементов
    let v = vec![1, 2, 3, 4, 5];
    for x in v.iter().take(3) {
        print!("{} ", x);
    }
    println!();
    // 1 2 3

    // for — skip (N ta o'tkazib yuborish)
    // пропустить N элементов
    let v = vec![1, 2, 3, 4, 5];
    for x in v.iter().skip(2) {
        print!("{} ", x);
    }
    println!();
    // 3 4 5

    // break — sikldan chiqish
    // выход из цикла
    for i in 0..10 {
        if i == 5 { break; }
        print!("{} ", i);
    }
    println!();
    // 0 1 2 3 4

    // continue — keyingi iteratsiyaga o'tish
    // переход к следующей итерации
    for i in 0..5 {
        if i == 2 { continue; }
        print!("{} ", i);
    }
    println!();
    // 0 1 3 4

    // label — ichki sikldan tashqi siklni to'xtatish
    // метка — выход из внешнего цикла
    'tashqi: for i in 0..3 {
        for j in 0..3 {
            if j == 1 { break 'tashqi; }
            println!("{} {}", i, j);
        }
    }
    // 0 0
}
// #================================================================================================================================================#
// # |  №  | Sikl / Metod               | Tavsif (UZ)                                               | Описание (RU)                                 |
// #================================================================================================================================================#
// # |   1 | loop                       | Cheksiz sikl, break bilan to'xtatiladi                    | Бесконечный цикл, останавливается через break |
// # |   2 | loop + break qiymat        | Sikldan qiymat qaytaradi                                  | Возвращает значение из цикла                  |
// # |   3 | while                      | Shart bajarilgunga qadar takrorlanadi                     | Выполняется пока условие истинно              |
// # |   4 | while let                  | Option/Result tugaguncha takrorlanadi                     | Выполняется пока есть значение                |
// # |   5 | for .. in range            | Range bo'ylab aylanadi (0..5)                             | Итерация по диапазону (0..5)                  |
// # |   6 | for .. in range=           | Inclusive range (0..=5)                                   | Включительный диапазон (0..=5)                |
// # |   7 | for .. in &Vec             | Vektor bo'ylab reference bilan aylanadi                   | Итерация по вектору через ссылку              |
// # |   8 | for .. in &mut Vec         | Vektor elementlarini o'zgartirib aylanadi                 | Итерация с изменением элементов вектора       |
// # |   9 | for .. in Vec              | Vektor ownership ini olib aylanadi                        | Итерация с забиранием владения вектором       |
// # |  10 | enumerate()                | Index va qiymatni birga beradi                            | Даёт индекс и значение вместе                 |
// # |  11 | for .. in Array            | Array bo'ylab aylanadi                                    | Итерация по массиву                           |
// # |  12 | chars()                    | String harflari bo'ylab aylanadi                          | Итерация по символам строки                   |
// # |  13 | for .. in HashMap          | HashMap kalit-qiymat juftlari bo'ylab aylanadi            | Итерация по HashMap                           |
// # |  14 | zip()                      | Ikki collection ni bir vaqtda aylanadi                    | Итерация по двум коллекциям одновременно      |
// # |  15 | filter()                   | Shartga mos elementlarni filterlaydi                      | Фильтрует элементы по условию                 |
// # |  16 | map()                      | Elementlarni o'zgartirib beradi                           | Преобразует элементы                          |
// # |  17 | step_by()                  | Qadam bilan aylanadi (0, 3, 6, 9...)                      | Итерация с шагом (0, 3, 6, 9...)              |
// # |  18 | rev()                      | Teskari tartibda aylanadi                                 | Итерация в обратном порядке                   |
// # |  19 | take()                     | Faqat birinchi N ta elementni oladi                       | Берёт только первые N элементов               |
// # |  20 | skip()                     | Birinchi N ta elementni tashlab qolganini oladi           | Пропускает N элементов, берёт остальные       |
// # |  21 | break                      | Sikldan chiqish                                           | Выход из цикла                                |
// # |  22 | continue                   | Keyingi iteratsiyaga o'tish                               | Переход к следующей итерации                  |
// # |  23 | 'label (break 'label)      | Tashqi sikldan chiqish (nested loop)                      | Выход из внешнего цикла (вложенные циклы)     |
// #================================================================================================================================================#