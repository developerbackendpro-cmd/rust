// #================================================================================================================================================#
// #                                                                 HASHSET                                                                        #
// #                                                    HASHSET — TAKRORSIZ QIYMATLAR TO'PLAMI           .                                          #
// #                                                    HASHSET — МНОЖЕСТВО УНИКАЛЬНЫХ ЗНАЧЕНИЙ                                                     #
// #================================================================================================================================================#

use std::collections::HashSet;
fn main() {

    // HashSet::new() — bo'sh set yaratish
    // создание пустого множества
    let set: HashSet<i32> = HashSet::new();
    println!("{:?}", set);
    // {}

    // .insert() — element qo'shish (takror bo'lsa qo'shilmaydi)
    // добавление элемента (дубликаты игнорируются)
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(2); // takror — qo'shilmaydi
    println!("{:?}", set);
    // {1, 2}

    // .contains() — element bormi?
    // есть ли элемент?
    let mut set = HashSet::new();
    set.insert(1);
    println!("{}", set.contains(&1));
    println!("{}", set.contains(&5));
    // true
    // false

    // .remove() — elementni o'chirish
    // удаление элемента
    let mut set = HashSet::new();
    set.insert(1);
    set.remove(&1);
    println!("{:?}", set);
    // {}

    // .len() — nechta element
    // количество элементов
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    println!("{}", set.len());
    // 2

    // .is_empty() — bo'shmi?
    // пустое ли множество?
    let set: HashSet<i32> = HashSet::new();
    println!("{}", set.is_empty());
    // true

    // takrorlarni olib tashlash — eng ko'p ishlatiladi!
    // удаление дубликатов из вектора
    let v = vec![1, 2, 2, 3, 3, 3];
    let set: HashSet<i32> = v.into_iter().collect();
    println!("{:?}", set);
    // {1, 2, 3}

    // .intersection() — ikkala setda ham bor elementlar
    // элементы, присутствующие в обоих множествах
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
    let inter: HashSet<_> = a.intersection(&b).collect();
    println!("{:?}", inter);
    // {2, 3}

    // .union() — ikkala setdagi barcha elementlar
    // все элементы обоих множеств
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
    let union: HashSet<_> = a.union(&b).collect();
    println!("{:?}", union);
    // {3, 1, 2, 4}

    // .difference() — a da bor, b da yo'q
    // элементы только в a, но не в b
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
    let diff: HashSet<_> = a.difference(&b).collect();
    println!("{:?}", diff);
    // {1}

    // .is_subset() — a, b ning to'liq qismimidir?
    // является ли a подмножеством b?
    let a: HashSet<i32> = [1, 2].iter().cloned().collect();
    let b: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    println!("{}", a.is_subset(&b));
    // true

    // .iter() — elementlar bo'ylab aylanish
    // итерация по элементам
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    for x in set.iter() {
        println!("{}", x);
    }
    // 2
    // 1

    // .drain() — hammani olib chiqish, set bo'shaydi
    // извлечение всех элементов, множество очищается
    let mut set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    for x in set.drain() {
        println!("{}", x);
    }
    println!("{:?}", set);
    // {}

    // .retain() — shartga mos elementlarni qoldirish
    // оставить только подходящие элементы
    let mut set: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    set.retain(|&x| x % 2 == 0);
    println!("{:?}", set);
    // {2, 4}

    // .is_superset() — a, b ni o'z ichiga oladimi?
    // является ли a надмножеством b?
    let a: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<i32> = [1, 2].iter().cloned().collect();
    println!("{}", a.is_superset(&b));
    // true
}
// #================================================================================================================================================#
// # |  №  | Metod                          | Tavsif (UZ)                                          | Описание (RU)                                  |
// #================================================================================================================================================#
// # |   1 |  HashSet::new()                | Bo'sh set yaratish                                   | Создание пустого множества                     |
// # |   2 |  insert(v)                     | Element qo'shish (takrorlar avtomatik o'chiriladi)   | Добавление элемента (дубликаты игнорируются)   |
// # |   3 |  contains(v)                   | Element borligini tekshiradi                         | Проверяет наличие элемента                     |
// # |   4 |  remove(v)                     | Elementni o'chirish                                  | Удаление элемента                              |
// # |   5 |  len()                         | Elementlar sonini qaytaradi                          | Возвращает количество элементов                |
// # |   6 |  is_empty()                    | Bo'shligini tekshiradi                               | Проверяет, пустое ли множество                 |
// # |   7 |  iter()                        | Elementlar bo'ylab aylanadi                          | Итерация по элементам                          |
// # |   8 |  drain()                       | Hammani olib chiqib, setni bo'shatadi                | Извлекает все элементы, очищает множество      |
// # |   9 |  retain(f)                     | Shartga mos elementlarni qoldiradi                   | Оставляет только подходящие элементы           |
// # |  10 |  intersection(&other)          | Ikkala setda ham bor elementlar                      | Элементы, присутствующие в обоих множествах    |
// # |  11 |  union(&other)                 | Ikkala setdagi barcha elementlar                     | Все элементы обоих множеств                    |
// # |  12 |  difference(&other)            | A da bor, B da yo'q elementlar                       | Элементы только в A, но не в B                 |
// # |  13 |  is_subset(&other)             | A, B ning to'liq qismi ekanligini tekshiradi         | Проверяет, является ли A подмножеством B       |
// # |  14 |  is_superset(&other)           | A, B ni o'z ichiga oladimi?                          | Проверяет, является ли A надмножеством B       |
// # |  15 |  collect() patterni            | Vectordagi takrorlarni olib tashlash                 | Удаление дубликатов из вектора                 |
// #================================================================================================================================================#