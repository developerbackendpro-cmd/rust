// #================================================================================================================================================#
// #                                                                 TYPE ALIASES                                                                   #
// #                                                    TYPE — MAVJUD TYPEGA YANGI NOM BERADI                                                       #
// #                                                    TYPE — ПСЕВДОНИМ ДЛЯ СУЩЕСТВУЮЩЕГО ТИПА                                                     #
// #================================================================================================================================================#

fn main() {

    // type Km = i32 — yangi nom, lekin bir xil type
    // новое имя, но тот же тип
    type Km = i32;
    type Metr = i32;
    let masofa: Km = 100;
    let balandlik: Metr = 50;
    println!("{} {}", masofa, balandlik);
    // 100 50

    // type — murakkab typeni qisqartirish
    // сокращение сложного типа
    type Natija = Result<i32, String>;
    fn hisobla(x: i32) -> Natija {
        if x > 0 { Ok(x * 2) } else { Err("Manfiy!".to_string()) }
    }
    println!("{:?}", hisobla(5));
    println!("{:?}", hisobla(-1));
    // Ok(10)
    // Err("Manfiy!")

    // type — funksiya typeini qisqartirish
    // сокращение типа функции
    type Fn2 = fn(i32, i32) -> i32;
    let qo_sh: Fn2 = |a, b| a + b;
    println!("{}", qo_sh(3, 4));
    // 7
}
// #================================================================================================================================================#
// # |  №  | Sintaksis / Pattern             | Tavsif (UZ)                                          | Описание (RU)                                 |
// #================================================================================================================================================#
// # |   1 | type YangiNomi = MavjudType     | Mavjud typega yangi nom berish                       | Создание псевдонима для существующего типа    |
// # |   2 | type Km = i32                   | Sodda typega alias                                   | Псевдоним для простого типа                   |
// # |   3 | type Natija = Result<i32, S>    | Murakkab typeni qisqartirish                         | Сокращение сложного типа                      |
// # |   4 | type Fn2 = fn(i32, i32) -> i32  | Funksiya typeiga alias                               | Псевдоним для типа функции                    |
// # |   5 | type Table = HashMap<String, V> | Generic bilan ishlatish                              | Использование с дженериками                   |
// #================================================================================================================================================#