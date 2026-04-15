// #================================================================================================================================================#
// #                                                               ARRAY va VEC FARQI                                                               #
// #                   Array — o'lchami qat'iy, stack'da turadi, tez ishlaydi / Array — фиксированный размер, хранится в стеке, быстрый.            #
// #                   Vec   — o'lchami o'zgaruvchan, heap'da turadi, moslashuvchan / Vec — динамический размер, хранится в куче, гибкий.           #
// #================================================================================================================================================#

fn main() {

    // [0; 4] — 0 qiymatini 4 marta takrorla
    // повтори значение 0 четыре раза
    let arr = [0; 4];
    println!("{:?}", arr);
    // [0, 0, 0, 0]

    // arr[2] — 2-indeksdagi elementni ol
    // получи элемент по индексу 2
    let arr = [10, 20, 30, 40, 50];
    println!("{}", arr[2]);
    // 30

    // &arr[1..3] — 1-indeksdan 3-gacha kesma ol (3 kirmaydi)
    // срез от индекса 1 до 3 (3 не включается)
    let arr = [1, 2, 3];
    let s = &arr[1..3];
    println!("{:?}", s);
    // [2, 3]

    // a == b — ikkala arrayni element-by-element solishtir
    // сравнивает массивы поэлементно
    let a: [i32; 3] = [1, 2, 3];
    let b: [i32; 3] = [1, 2, 3];
    println!("{}", a == b);
    // true

    // .len() — nechta element borligini qaytaradi
    // .len() возвращает количество элементов
    let arr = [10, 20, 30];
    let a = arr.len();
    println!("{}", a);
    // 3

    // .contains(&x) — x element arrayda bormi?
    // есть ли элемент x в массиве?
    let arr = [1, 2, 3, 4, 5];
    let a = arr.contains(&3);
    println!("{}", a);
    // true

    // .map(|x| ...) — har bir elementni o'zgartiradi
    // преобразует каждый элемент
    let arr = [1, 2, 3];
    let doubled = arr.map(|x| x * 2);
    println!("{:?}", doubled);
    // [2, 4, 6]

    // .iter().sum() — barcha elementlarni qo'shib chiqaradi
    // .iter().sum() — суммирует все элементы
    let arr = [1, 2, 3, 4, 5];
    let total: i32 = arr.iter().sum();
    println!("{}", total);
    // 15

    // .iter().max() — eng katta elementni qaytaradi (Some ichida)
    // возвращает максимальный элемент (внутри Some)
    let arr = [3, 1, 7, 2];
    let a = arr.iter().max();
    println!("{:?}", a);
    // Some(7)

    // .iter().min() — eng kichik elementni qaytaradi (Some ichida)
    // возвращает минимальный элемент (внутри Some)
    let arr = [3, 1, 7, 2];
    let a = arr.iter().min();
    println!("{:?}", a);
    // Some(1)

    // .iter().filter() — shartga mos elementlarni oladi
    // фильтрует элементы по условию
    let arr = [1, 2, 3, 4, 5];
    let katta: Vec<_> = arr.iter().filter(|&&x| x > 3).collect();
    println!("{:?}", katta);
    // [4, 5]

    // .iter().enumerate() — index va qiymatni birga beradi
    // даёт индекс и значение вместе
    let arr = ["a", "b", "c"];
    for (i, val) in arr.iter().enumerate() {
        println!("{}: {}", i, val);
    }
    // 0: a
    // 1: b
    // 2: c

    // .iter().position() — elementning indeksini topadi
    // находит индекс элемента
    let arr = [10, 20, 30];
    let pos = arr.iter().position(|&x| x == 20);
    println!("{:?}", pos);
    // Some(1)

    // .iter().any() — birorta element shartga to'g'ri keladimi?
    // хотя бы один элемент удовлетворяет условию?
    let arr = [1, 2, 3, 4, 5];
    let a = arr.iter().any(|&x| x > 4);
    println!("{}", a);
    // true

    // .iter().all() — BARCHA elementlar shartga to'g'ri keladimi?
    // ВСЕ элементы удовлетворяют условию?
    let arr = [2, 4, 6, 8];
    let a = arr.iter().all(|&x| x % 2 == 0);
    println!("{}", a);
    // true

    // .sort() — kichikdan kattaga tartiblaydi
    // массив сортируется от меньшего к большему
    let mut arr = [3, 1, 4, 1, 5];
    arr.sort();
    println!("{:?}", arr);
    // [1, 1, 3, 4, 5]

    // .sort_by() — o'zing tartib belgilaysan
    // сортируешь сам по своему условию
    let mut arr = [3, 1, 4, 1, 5];
    arr.sort_by(|a, b| b.cmp(a));
    println!("{:?}", arr);
    // [5, 4, 3, 1, 1]

    // .binary_search() — tartiblangan arrayda tez qidirish
    // быстрый поиск в отсортированном массиве
    let arr = [1, 2, 3, 4, 5];
    let a = arr.binary_search(&3);
    println!("{:?}", a);
    // Ok(2)

    // .windows(n) — n o'lchamli oynalar bilan aylanish
    // скользящее окно размером n
    let arr = [1, 2, 3, 4, 5];
    for w in arr.windows(3) {
        println!("{:?}", w);
    }
    // [1, 2, 3]
    // [2, 3, 4]
    // [3, 4, 5]

    // .chunks(n) — n o'lchamli bo'laklarga ajratish
    // разбивает на части по n элементов
    let arr = [1, 2, 3, 4, 5];
    for c in arr.chunks(2) {
        println!("{:?}", c);
    }
    // [1, 2]
    // [3, 4]
    // [5]

    // .fill(x) — hammasini x qiymat bilan to'ldiradi
    // заполняет весь массив значением x
    let mut arr = [0; 5];
    arr.fill(7);
    println!("{:?}", arr);
    // [7, 7, 7, 7, 7]

    // .reverse() — teskari qiladi
    // переворачивает массив
    let mut arr = [1, 2, 3, 4, 5];
    arr.reverse();
    println!("{:?}", arr);
    // [5, 4, 3, 2, 1]

    // .split_at(n) — n indeksdan ikki qismga bo'ladi
    // разбивает массив на две части по индексу n
    let arr = [1, 2, 3, 4, 5];
    let (left, right) = arr.split_at(3);
    println!("{:?}", left);   // [1, 2, 3]
    println!("{:?}", right);  // [4, 5]

    // .iter().map().collect() — har elementni o'zgartirib yangi vec yaratadi
    // преобразует каждый элемент и собирает в новый вектор
    let arr = [1, 2, 3, 4, 5];
    let doubled: Vec<i32> = arr.iter().map(|&x| x * 2).collect();
    println!("{:?}", doubled);
    // [2, 4, 6, 8, 10]

    // .iter().zip() — ikki arrayni juftlab birlashtiradi
    // объединяет два массива попарно
    let a = [1, 2, 3];
    let b = ["bir", "ikki", "uch"];
    let juft: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("{:?}", juft);
    // [(1, "bir"), (2, "ikki"), (3, "uch")]

    // .iter().count() — elementlar sonini qaytaradi
    // возвращает количество элементов
    let arr = [1, 2, 3, 4, 5];
    let a = arr.iter().count();
    println!("{}", a);
    // 5

    // .iter().fold() — barcha elementlarni bitta qiymatga yig'adi
    // сворачивает все элементы в одно значение
    let arr = [1, 2, 3, 4, 5];
    let total = arr.iter().fold(0, |sum, &x| sum + x);
    println!("{}", total);
    // 15

    // .first() — birinchi elementni xavfsiz qaytaradi
    // Безопасно возвращает первый элемент
    let arr = [10, 20, 30, 40, 50];
    let a = arr.first();
    println!("first() ► {:?}", a);
    // first() ► Some(10)

    // .last() — oxirgi elementni xavfsiz qaytaradi
    // Безопасно возвращает последний элемент
    let arr = [10, 20, 30, 40, 50];
    let a = arr.last();
    println!("last() ► {:?}", a);
    // last() ► Some(50)

    // .is_empty() — array bo'shligini tekshiradi
    // Проверяет, пуст ли массив
    let arr = [1, 2, 3];
    let a = arr.is_empty();
    println!("is_empty() ► {}", a);
    // is_empty() ► false

    // .as_slice() — arraydan slice yasaydi
    // Создаёт срез из массива
    let arr = [1, 2, 3, 4, 5];
    let slice = arr.as_slice();
    println!("as_slice() ► {:?}", slice);
    // as_slice() ► [1, 2, 3, 4, 5]

    // .iter_mut() — o'zgartiruvchi iterator
    // Изменяемый итератор
    let mut arr = [1, 2, 3, 4, 5];
    for x in arr.iter_mut() {
        *x *= 2;
    }
    println!("iter_mut() ► {:?}", arr);
    // iter_mut() ► [2, 4, 6, 8, 10]

    // .starts_with() — kesma bilan boshlanishini tekshiradi
    // Проверяет начало
    let arr = [1, 2, 3, 4, 5];
    let a = arr.starts_with(&[1, 2]);
    println!("starts_with() ► {}", a);
    // starts_with() ► true

    // .ends_with() — kesma bilan tugashini tekshiradi
    // Проверяет окончание
    let arr = [1, 2, 3, 4, 5];
    let a = arr.ends_with(&[4, 5]);
    println!("ends_with() ► {}", a);
    // ends_with() ► true
}
// #================================================================================================================================================#
// # |  №  | Metod / Operator         | Tavsif (UZ)                                               | Описание (RU)                                   |
// #================================================================================================================================================#
// # |   1 | [0; 4]                   | Qiymatni n marta takrorlab array yaratadi                 | Создаёт массив, повторяя значение n раз         |
// # |   2 | arr[2]                   | Indeks bo'yicha elementga murojaat                        | Доступ к элементу по индексу                    |
// # |   3 | &arr[1..3]               | Arraydan kesma oladi (start dan end-1 gacha)              | Создаёт срез массива (от start до end-1)        |
// # |   4 | a == b                   | Ikkala arrayni element-by-element solishtiradi            | Сравнивает массивы поэлементно                  |
// # |   5 | len()                    | Arraydagi elementlar sonini qaytaradi                     | Возвращает количество элементов в массиве       |
// # |   6 | contains()               | Element arrayda borligini tekshiradi                      | Проверяет наличие элемента в массиве            |
// # |   7 | map()                    | Har bir elementni o'zgartiradi                            | Преобразует каждый элемент                      |
// # |   8 | iter().sum()             | Barcha elementlarni qo'shib chiqaradi                     | Суммирует все элементы                          |
// # |   9 | iter().max()             | Eng katta elementni qaytaradi (Some ichida)               | Возвращает максимальный элемент (в Some)        |
// # |  10 | iter().min()             | Eng kichik elementni qaytaradi (Some ichida)              | Возвращает минимальный элемент (в Some)         |
// # |  11 | iter().filter()          | Shartga mos elementlarni oladi                            | Фильтрует элементы по условию                   |
// # |  12 | iter().enumerate()       | Index va qiymatni birga beradi                            | Даёт индекс и значение вместе                   |
// # |  13 | iter().position()        | Elementning indeksini topadi                              | Находит индекс элемента                         |
// # |  14 | iter().any()             | Birorta element shartga to'g'ri keladimi?                 | Хотя бы один элемент удовлетворяет условию?     |
// # |  15 | iter().all()             | Barcha elementlar shartga to'g'ri keladimi?               | Все элементы удовлетворяют условию?             |
// # |  16 | sort()                   | Arrayni kichikdan kattaga tartiblaydi                     | Сортирует массив от меньшего к большему         |
// # |  17 | sort_by()                | O'zing belgilagan tartib bo'yicha sortlaydi               | Сортирует по заданному условию                  |
// # |  18 | binary_search()          | Tartiblangan arrayda tez qidirish                         | Быстрый поиск в отсортированном массиве         |
// # |  19 | windows()                | Berilgan o'lchamdagi oynalar bilan aylantiradi            | Скользящее окно заданного размера               |
// # |  20 | chunks()                 | Arrayni berilgan o'lchamdagi bo'laklarga ajratadi         | Разбивает массив на части указанного размера    |
// # |  21 | fill()                   | Arraydagi barcha elementlarni bir qiymat bilan to'ldiradi | Заполняет массив одним значением                |
// # |  22 | reverse()                | Arrayni teskari tartibga keltiradi                        | Переворачивает массив                           |
// # |  23 | split_at()               | Berilgan indeksdan ikki qismga bo'ladi                    | Разбивает массив на две части по индексу        |
// # |  24 | iter().map().collect()   | Elementlarni o'zgartirib yangi vector yaratadi            | Преобразует элементы и собирает в новый вектор  |
// # |  25 | iter().zip()             | Ikki arrayni juftlab birlashtiradi                        | Объединяет два массива попарно                  |
// # |  26 | iter().count()           | Elementlar sonini qaytaradi                               | Возвращает количество элементов                 |
// # |  27 | iter().fold()            | Barcha elementlarni bitta qiymatga yig'adi                | Сворачивает все элементы в одно значение        |
// # |  28 | first()                  | Birinchi elementni xavfsiz qaytaradi                      | Безопасно возвращает первый элемент             |
// # |  29 | last()                   | Oxirgi elementni xavfsiz qaytaradi                        | Безопасно возвращает последний элемент          |
// # |  30 | is_empty()               | Array bo'shligini tekshiradi                              | Проверяет, пуст ли массив                       |
// # |  31 | as_slice()               | Arraydan slice yasaydi                                    | Создаёт срез из массива                         |
// # |  32 | iter_mut()               | O'zgartiruvchi iterator qaytaradi                         | Возвращает изменяемый итератор                  |
// # |  33 | starts_with()            | Kesma bilan boshlanishini tekshiradi                      | Проверяет начало с указанной подстрокой         |
// # |  34 | ends_with()              | Kesma bilan tugashini tekshiradi                          | Проверяет окончание указанной подстрокой        |
// #================================================================================================================================================#