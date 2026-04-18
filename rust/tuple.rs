// #================================================================================================================================================#
// #                                                                 TUPLE                                                                          #
// #                                       TUPLE — HAR XIL TYPEDAGI QIYMATLARNI SAQLAYDI, QAT'IY O'LCHAM.                                           #
// #                                           TUPLE — ХРАНИТ РАЗНЫЕ ТИПЫ, ФИКСИРОВАННЫЙ РАЗМЕР                                                     #
// #================================================================================================================================================#

fn main() {

    // (T1, T2, T3) — har xil typedagi tuple yaratish
    // создание кортежа с разными типами
    let t = (1, "salom", true, 3.14);
    println!("{:?}", t);
    // (1, "salom", true, 3.14)

    // t.0, t.1 — indeks bilan elementni olish
    // получение элемента по индексу
    let t = (10, "dunyo", false);
    println!("{}", t.0);
    println!("{}", t.1);
    println!("{}", t.2);
    // 10
    // dunyo
    // false

    // destructuring — tuple ni o'zgaruvchilarga ajratish
    // деструктуризация кортежа в переменные
    let t = (1, "salom", true);
    let (a, b, c) = t;
    println!("{} {} {}", a, b, c);
    // 1 salom true

    // _ — kerak bo'lmagan qiymatni o'tkazib yuborish
    // пропуск ненужного значения
    let t = (1, "salom", true);
    let (a, _, c) = t;
    println!("{} {}", a, c);
    // 1 true

    // mut tuple — o'zgaruvchan tuple
    // изменяемый кортеж
    let mut t = (1, 2, 3);
    t.0 = 10;
    println!("{:?}", t);
    // (10, 2, 3)

    // tuple ichida tuple — nested tuple
    // вложенный кортеж
    let t = (1, (2, 3), "salom");
    println!("{}", t.1.0);
    println!("{}", t.1.1);
    // 2
    // 3


    // funksiyadan tuple qaytarish
    // возврат кортежа из функции
    fn min_max(arr: &[i32]) -> (i32, i32) {
        (*arr.iter().min().unwrap(), *arr.iter().max().unwrap())
    }
    let (min, max) = min_max(&[3, 1, 4, 1, 5]);
    println!("min={} max={}", min, max);
    // min=1 max=5

    // unit type () — bo'sh tuple, hech narsa qaytarmaydi
    // пустой кортеж, ничего не возвращает
    let t: () = ();
    println!("{:?}", t);
    // ()

    // tuple type annotation — type belgilash
    // явное указание типов кортежа
    let t: (i32, &str, bool) = (42, "rust", true);
    println!("{:?}", t);
    // (42, "rust", true)

    // == solishtirish — bir xil typedagi tuplelar
    // сравнение кортежей одного типа
    let a = (1, 2, 3);
    let b = (1, 2, 3);
    println!("{}", a == b);
    // true
}
// #================================================================================================================================================#
// # |  №  | Sintaksis / Usul               | Tavsif (UZ)                                          | Описание (RU)                                  |
// #================================================================================================================================================#
// # |   1 | (T1, T2, T3, ...)              | Har xil typedagi tuple yaratish                      | Создание кортежа с разными типами              |
// # |   2 | tuple.0, tuple.1, ...          | Indeks bilan elementni olish                         | Получение элемента по индексу                  |
// # |   3 | let (a, b, c) = tuple          | Destructuring — o'zgaruvchilarga ajratish            | Деструктуризация в переменные                  |
// # |   4 | let (a, _, c) = tuple          | _ bilan keraksiz qiymatni o'tkazib yuborish          | Пропуск ненужного значения через _             |
// # |   5 | mut tuple                      | O'zgaruvchan tuple                                   | Изменяемый кортеж                              |
// # |   6 | (1, (2, 3), "salom")           | Nested tuple (tuple ichida tuple)                    | Вложенный кортеж                               |
// # |   7 | fn foo() -> (i32, i32)         | Funksiyadan tuple qaytarish                          | Возврат кортежа из функции                     |
// # |   8 | ()                             | Unit type — bo'sh tuple, hech narsa qaytarmaydi      | Пустой кортеж, ничего не возвращает            |
// # |   9 | let t: (i32, &str, bool)       | Type annotation — turlarni aniq belgilash            | Явное указание типов кортежа                   |
// # |  10 | a == b                         | Solishtirish (bir xil typedagi tuple'lar uchun)      | Сравнение кортежей одного типа                 |
// #================================================================================================================================================#