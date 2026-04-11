// #================================================================================================================================================#
// #                                                                 VEC<T                                                                          #
// #                                        VEC — O'LCHAMI O'ZGARUVCHAN, HEAP'DA TURADI, MOSLASHUVCHAN.                                             #
// #                                             VEC — ДИНАМИЧЕСКИЙ РАЗМЕР, ХРАНИТСЯ В КУЧЕ, ГИБКИЙ                                                 #
// #================================================================================================================================================#

fn main() {

    // Vec::new() — bo'sh vec yaratish
    // создание пустого вектора
    let mut v: Vec<i32> = Vec::new();
    println!("{:?}", v);
    // []

    // vec![] — qiymatlar bilan yaratish (macro)
    // создание вектора с начальными значениями
    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    // [1, 2, 3, 4, 5]

    // .push() — oxiriga element qo'shish
    // добавление элемента в конец
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);
    // [1, 2, 3, 4]

    // .pop() — oxirgi elementni olish va o'chirish
    // удаление и возврат последнего элемента
    let mut v = vec![1, 2, 3];
    let a = v.pop();
    println!("{:?}", a);
    println!("{:?}", v);
    // Some(3)
    // [1, 2]

    // .len() — elementlar soni
    // количество элементов
    let v = vec![1, 2, 3];
    println!("{}", v.len());
    // 3

    // .is_empty() — bo'shmi?
    // пустой ли вектор?
    let v: Vec<i32> = vec![];
    println!("{}", v.is_empty());
    // true

    // .insert(i, x) — i indeksga x qo'shish
    // вставка x по индексу i
    let mut v = vec![1, 2, 3];
    v.insert(1, 10);
    println!("{:?}", v);
    // [1, 10, 2, 3]

    // .remove(i) — i indeksdagi elementni o'chirish
    // удаление элемента по индексу i
    let mut v = vec![1, 2, 3, 4];
    let a = v.remove(1);
    println!("{}", a);
    println!("{:?}", v);
    // 2
    // [1, 3, 4]

    // .contains(&x) — x element bormi?
    // есть ли элемент x?
    let v = vec![1, 2, 3];
    println!("{}", v.contains(&2));
    // true

    // .sort() — kichikdan kattaga tartiblash
    // сортировка по возрастанию
    let mut v = vec![3, 1, 4, 1, 5];
    v.sort();
    println!("{:?}", v);
    // [1, 1, 3, 4, 5]

    // .reverse() — teskari qilish
    // переворачивание вектора
    let mut v = vec![1, 2, 3];
    v.reverse();
    println!("{:?}", v);
    // [3, 2, 1]

    // .dedup() — ketma-ket takrorlarni o'chirish
    // удаление последовательных дубликатов
    let mut v = vec![1, 1, 2, 3, 3, 3];
    v.dedup();
    println!("{:?}", v);
    // [1, 2, 3]

    // .retain() — shartga mos elementlarni qoldirish
    // оставить только элементы, удовлетворяющие условию
    let mut v = vec![1, 2, 3, 4, 5];
    v.retain(|&x| x % 2 == 0);
    println!("{:?}", v);
    // [2, 4]

    // .clear() — hammasini o'chirish
    // очистка вектора
    let mut v = vec![1, 2, 3];
    v.clear();
    println!("{:?}", v);
    // []

    // .extend() — boshqa collectiondan elementlar qo'shish
    // добавление элементов из другой коллекции
    let mut v = vec![1, 2, 3];
    v.extend([4, 5, 6]);
    println!("{:?}", v);
    // [1, 2, 3, 4, 5, 6]

    // .truncate(n) — n tadan keyin qirqish
    // обрезать до n элементов
    let mut v = vec![1, 2, 3, 4, 5];
    v.truncate(3);
    println!("{:?}", v);
    // [1, 2, 3]

    // .capacity() — ajratilgan xotira hajmi
    // выделенная ёмкость памяти
    let mut v = Vec::with_capacity(10);
    v.push(1);
    println!("{}", v.capacity());
    println!("{}", v.len());
    // 10
    // 1

    // .iter() — elementlar bo'ylab aylanish
    // итерация по элементам
    let v = vec![1, 2, 3];
    for x in v.iter() {
        println!("{}", x);
    }
    // 1
    // 2
    // 3

    // .iter().map() — har elementni o'zgartirish
    // преобразование каждого элемента
    let v = vec![1, 2, 3];
    let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
    println!("{:?}", doubled);
    // [2, 4, 6]

    // .iter().filter() — shartga mos elementlarni olish
    // фильтрация элементов по условию
    let v = vec![1, 2, 3, 4, 5];
    let katta: Vec<&i32> = v.iter().filter(|&&x| x > 3).collect();
    println!("{:?}", katta);
    // [4, 5]

    // .iter().sum() — barcha elementlarni qo'shish
    // сумма всех элементов
    let v = vec![1, 2, 3, 4, 5];
    let total: i32 = v.iter().sum();
    println!("{}", total);
    // 15

    // .iter().max() / .min() — eng katta / kichik element
    // максимальный / минимальный элемент
    let v = vec![3, 1, 7, 2];
    println!("{:?}", v.iter().max());
    println!("{:?}", v.iter().min());
    // Some(7)
    // Some(1)

    // .iter().enumerate() — index va qiymat birga
    // индекс и значение вместе
    let v = vec!["a", "b", "c"];
    for (i, val) in v.iter().enumerate() {
        println!("{}: {}", i, val);
    }
    // 0: a  1: b  2: c

    // .windows(n) — n o'lchamli oyna
    // скользящее окно размером n
    let v = vec![1, 2, 3, 4, 5];
    for w in v.windows(3) {
        println!("{:?}", w);
    }
    // [1,2,3] [2,3,4] [3,4,5]

    // .chunks(n) — n o'lchamli bo'laklar
    // разбивка на части по n элементов
    let v = vec![1, 2, 3, 4, 5];
    for c in v.chunks(2) {
        println!("{:?}", c);
    }
    // [1,2] [3,4] [5]
}
// #================================================================================================================================================#
// # |  №  | Metod                    | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |   1 | Vec::new()               | Bo'sh vector yaratish                                | Создание пустого вектора                             |
// # |   2 | vec![]                   | Makro bilan vector yaratish (ENG KO'P ISHLATILADI)   | Создание вектора с помощью макроса (САМЫЙ ЧАСТЫЙ)    |
// # |   3 | push()                   | Oxiriga element qo'shish                             | Добавление элемента в конец                          |
// # |   4 | pop()                    | Oxirgi elementni olish va o'chirish                  | Получение и удаление последнего элемента             |
// # |   5 | len()                    | Elementlar soni                                      | Количество элементов                                 |
// # |   6 | is_empty()               | Bo'shligini tekshirish                               | Проверка на пустоту                                  |
// # |   7 | insert()                 | Indeksga element qo'shish                            | Вставка элемента по индексу                          |
// # |   8 | remove()                 | Indeksdagi elementni o'chirish                       | Удаление элемента по индексу                         |
// # |   9 | contains()               | Element mavjudligini tekshirish                      | Проверка наличия элемента                            |
// # |  10 | sort()                   | Kichikdan kattaga tartiblash                         | Сортировка по возрастанию                            |
// # |  11 | reverse()                | Teskari qilish                                       | Переворачивание вектора                              |
// # |  12 | dedup()                  | Ketma-ket takrorlarni o'chirish                      | Удаление последовательных дубликатов                 |
// # |  13 | retain()                 | Shartga mos elementlarni qoldirish                   | Оставить только элементы по условию                  |
// # |  14 | clear()                  | Hammasini o'chirish                                  | Очистка вектора                                      |
// # |  15 | extend()                 | Boshqa collectiondan elementlar qo'shish             | Добавление элементов из другой коллекции             |
// # |  16 | truncate()               | n tadan keyin qirqish                                | Обрезать до n элементов                              |
// # |  17 | capacity()               | Ajratilgan xotira hajmi                              | Выделенная ёмкость памяти                            |
// # |  18 | with_capacity()          | Oldindan joy ajratib yaratish                        | С предварительным выделением памяти                  |
// # |  19 | iter()                   | Elementlar bo'ylab aylanish                          | Итерация по элементам                                |
// # |  20 | map()                    | Har bir elementni o'zgartirish                       | Преобразование каждого элемента                      |
// # |  21 | filter()                 | Shartga mos elementlarni olish                       | Фильтрация элементов по условию                      |
// # |  22 | sum()                    | Barcha elementlarni qo'shish                         | Сумма всех элементов                                 |
// # |  23 | max()                    | Eng katta element                                    | Максимальный элемент                                 |
// # |  24 | min()                    | Eng kichik element                                   | Минимальный элемент                                  |
// # |  25 | enumerate()              | Index va qiymat birga                                | Индекс и значение вместе                             |
// # |  26 | windows()                | n o'lchamli oyna (skolzyachi)                        | Скользящее окно размером n                           |
// # |  27 | chunks()                 | n o'lchamli bo'laklar                                | Разбивка на части по n элементов                     |
// #================================================================================================================================================#