// #================================================================================================================================================#
// #                                                                 HASHMAP                                                                        #
// #                                           HASHMAP — KALIT-QIYMAT JUFTLIKLARI, HEAP'DA TURADI        .                                          #
// #                                              HASHMAP — ПАРЫ КЛЮЧ-ЗНАЧЕНИЕ, ХРАНИТСЯ В КУЧЕ                                                     #
// #================================================================================================================================================#

use std::collections::HashMap;
fn main() {

    // HashMap::new() — bo'sh hashmap yaratish
    // создание пустой хэш-карты
    let mut map: HashMap<&str, i32> = HashMap::new();
    println!("{:?}", map);
    // {}

    // HashMap::with_capacity(n) — oldindan xotira ajratish
    // предварительное выделение памяти
    let mut map: HashMap<&str, i32> = HashMap::with_capacity(10);
    map.insert("a", 1);
    println!("{}", map.capacity());
    // 14

    // .insert(k, v) — kalit-qiymat qo'shish, eski qiymatni qaytaradi
    // добавление пары, возвращает старое значение
    let mut map = HashMap::new();
    map.insert("ball", 10);
    let old = map.insert("ball", 20); // qayta yozadi
    println!("{:?}", old);
    println!("{:?}", map.get("ball"));
    // Some(10)
    // Some(20)

    // .get(k) — immutable reference qaytaradi
    // возвращает неизменяемую ссылку
    let mut map = HashMap::new();
    map.insert("yosh", 25);
    let a = map.get("yosh");
    println!("{:?}", a);
    // Some(25)

    // .get_mut(k) — mutable reference qaytaradi
    // возвращает изменяемую ссылку
    let mut map = HashMap::new();
    map.insert("ball", 10);
    if let Some(v) = map.get_mut("ball") {
        *v += 5;
    }
    println!("{:?}", map.get("ball"));
    // Some(15)

    // .contains_key(k) — kalit bormi?
    // есть ли такой ключ?
    let mut map = HashMap::new();
    map.insert("til", "Rust");
    println!("{}", map.contains_key("til"));
    println!("{}", map.contains_key("xyz"));
    // true
    // false

    // .remove(k) — kalitni o'chirib qiymatini qaytaradi
    // удаление ключа с возвратом значения
    let mut map = HashMap::new();
    map.insert("a", 1);
    let val = map.remove("a");
    println!("{:?}", val);
    println!("{:?}", map);
    // Some(1)
    // {}

    // .entry().or_insert() — kalit yo'q bo'lsa qo'sh
    // вставить значение если ключ отсутствует
    let mut map = HashMap::new();
    map.entry("ball").or_insert(0);
    map.entry("ball").or_insert(100);
    println!("{:?}", map.get("ball"));
    // ishlamaydi — bor
    // Some(0)

    // .entry().or_insert() — counter pattern (eng ko'p ishlatiladi!)
    // подсчёт вхождений — самый частый паттерн!
    let mut map = HashMap::new();
    let sozlar = vec!["a", "b", "a", "c", "a"];
    for soz in sozlar {
        let count = map.entry(soz).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    // {"a": 3, "b": 1, "c": 1}

    // .entry().or_default() — default qiymat bilan
    // вставка со значением по умолчанию
    let mut map: HashMap<&str, Vec<i32>> = HashMap::new();
    map.entry("guruh").or_default().push(1);
    map.entry("guruh").or_default().push(2);
    println!("{:?}", map);
    // {"guruh": [1, 2]}

    // .entry().or_insert_with() — lazy qiymat yaratish
    // ленивое создание значения
    let mut map = HashMap::new();
    map.entry("kalit").or_insert_with(|| vec![1, 2, 3]);
    println!("{:?}", map);
    // {"kalit": [1, 2, 3]}

    // .len() — nechta juft bor
    // количество пар
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    println!("{}", map.len());
    // 2

    // .is_empty() — bo'shmi?
    // пустая ли карта?
    let map: HashMap<&str, i32> = HashMap::new();
    println!("{}", map.is_empty());
    // true

    // .keys() — barcha kalitlar
    // все ключи
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    for k in map.keys() {
        println!("{}", k);
    }
    // a
    // b

    // .values() — barcha qiymatlar
    // все значения
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    for v in map.values() {
        println!("{}", v);
    }
    // 1
    // 2

    // .values_mut() — qiymatlarni o'zgartirish
    // изменение значений
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    for v in map.values_mut() {
        *v *= 10;
    }
    println!("{:?}", map);
    // {"a": 10, "b": 20}

    // .iter() — (kalit, qiymat) juftlari
    // итерация по парам
    for (k, v) in map.iter() {
        println!("{}: {}", k, v);
    }
    // a: 10
    // b: 20

    // .iter_mut() — qiymatlarni o'zgartirib aylanish
    // итерация с изменением значений
    let mut map = HashMap::new();
    map.insert("a", 1);
    for (_, v) in map.iter_mut() {
        *v += 100;
    }
    println!("{:?}", map);
    // {"a": 101}

    // .retain() — shartga mos juftlarni qoldirish
    // оставить только подходящие пары
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map.retain(|_, v| *v > 1);
    println!("{:?}", map);
    // {"b": 2, "c": 3}

    // .extend() — boshqa collectiondan qo'shish
    // добавление из другой коллекции
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.extend([("b", 2), ("c", 3)]);
    println!("{:?}", map);
    // {"b": 2, "c": 3, "a": 1}

    // .drain() — hammani olib chiqish, map bo'shaydi
    // извлечение всех элементов, карта очищается
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    for (k, v) in map.drain() {
        println!("{}: {}", k, v);
    }
    println!("{:?}", map);
    // {}

    // .clear() — hammasini o'chirish
    // очистка карты
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.clear();
    println!("{:?}", map);
    // {}

    // .capacity() — ajratilgan xotira
    // выделенная ёмкость
    let map: HashMap<&str, i32> = HashMap::with_capacity(10);
    println!("{}", map.capacity());
    // 14
}
// #================================================================================================================================================#
// # |  №  | Metod                          | Tavsif (UZ)                                          | Описание (RU)                                  |
// #================================================================================================================================================#
// # |   1 |  HashMap::new()                | Bo'sh hashmap yaratish                               | Создание пустой хэш-карты                      |
// # |   2 |  HashMap::with_capacity(n)     | Oldindan xotira ajratish                             | Предварительное выделение памяти               |
// # |   3 |  insert(k, v)                  | Kalit-qiymat qo'shish, eski qiymatni qaytaradi       | Добавление пары, возвращает старое значение    |
// # |   4 |  get(k)                        | Immutable reference qaytaradi                        | Возвращает неизменяемую ссылку                 |
// # |   5 |  get_mut(k)                    | Mutable reference qaytaradi                          | Возвращает изменяемую ссылку                   |
// # |   6 |  contains_key(k)               | Kalit borligini tekshiradi                           | Проверяет наличие ключа                        |
// # |   7 |  remove(k)                     | Kalitni o'chirib qiymatini qaytaradi                 | Удаляет ключ и возвращает значение             |
// # |   8 |  entry().or_insert(v)          | Kalit yo'q bo'lsa qo'shadi                           | Вставляет значение, если ключ отсутствует      |
// # |   9 |  entry().or_default()          | Default qiymat bilan qo'shadi                        | Вставляет значение по умолчанию                |
// # |  10 |  entry().or_insert_with(f)     | Lazy qiymat yaratadi                                 | Ленивое создание значения                      |
// # |  11 |  len()                         | Juftlar sonini qaytaradi                             | Возвращает количество пар                      |
// # |  12 |  is_empty()                    | Bo'shligini tekshiradi                               | Проверяет, пустая ли карта                     |
// # |  13 |  keys()                        | Barcha kalitlarni qaytaradi                          | Возвращает все ключи                           |
// # |  14 |  values()                      | Barcha qiymatlarni qaytaradi                         | Возвращает все значения                        |
// # |  15 |  values_mut()                  | Qiymatlarni o'zgartirish imkoni                      | Возможность изменения значений                 |
// # |  16 |  iter()                        | (k, v) juftlari bo'ylab aylanadi                     | Итерация по парам (k, v)                       |
// # |  17 |  iter_mut()                    | Qiymatlarni o'zgartirib aylanadi                     | Итерация с изменением значений                 |
// # |  18 |  retain(f)                     | Shartga mos juftlarni qoldiradi                      | Оставляет только подходящие пары               |
// # |  19 |  extend(iter)                  | Boshqa collectiondan qo'shadi                        | Добавляет из другой коллекции                  |
// # |  20 |  drain()                       | Hammani olib chiqib, mapni bo'shatadi                | Извлекает все элементы, очищает карту          |
// # |  21 |  clear()                       | Hammasini o'chiradi                                  | Очищает карту                                  |
// # |  22 |  capacity()                    | Ajratilgan xotira hajmini qaytaradi                  | Возвращает выделенную ёмкость                  |
// #================================================================================================================================================#