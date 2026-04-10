// #================================================================================================================================================#
// #                                                        STRING TURLARI / ТИПЫ СТРОК                                                             #
// #                            &str  — o'zgarmas, stack/binary da, tez  /  неизменяемая, в стеке/бинарнике, быстрая                                #
// #                            String — o'zgaruvchan, heap da, moslashuvchan  / String — изменяемая, в куче, гибкая                                #
// #================================================================================================================================================#

fn main() {

    // String::new() — Yangi bo'sh string yaratadi
    // Создаёт новую пустую строку
    let s1 = String::new();
    println!("1) String::new() ► '{}'", s1);
    // 1) String::new() ► ''

    // String::from() — String literaldan string yaratadi
    // Создаёт строку из строкового литерала
    let s2 = String::from("Dilshod");
    println!("2) String::from() ► {}", s2);
    // 2) String::from() ► Dilshod

    // to_string() — Qiymatni stringga aylantiradi
    // Преобразует значение в строку
    let son = 42;
    let s3 = son.to_string();
    println!("3) to_string() ► {}", s3);
    // 3) to_string() ► 42

    // to_string() — Qiymatni stringga aylantiradi
    // Преобразует значение в строку
    let pi = 3.14;
    let s4 = pi.to_string();
    println!("3) to_string() ► {}", s4);
    // 3) to_string() ► 3.14

    // len() — String uzunligini (baytda) qaytaradi
    // Возвращает длину строки (в байтах)
    let s = String::from("salom");
    let a = s.len();
    println!("4) len() ► {}", a);
    // 4) len() ► 5

    // len() — String uzunligini (baytda) qaytaradi
    // Возвращает длину строки (в байтах)
    let s = String::from("😊😊😊");
    let a = s.len();
    println!("4) len() (emojilar) ► {}", a);
    // 4) len() (emojilar) ► 12

    // is_empty() — String bo'shligini tekshiradi
    // Проверяет, пуста ли строка
    let s = String::from("salom");
    let a = s.is_empty();
    println!("5) is_empty() 'salom' ► {}", a);
    // 5) is_empty() 'salom' ► false

    // is_empty() — String bo'shligini tekshiradi
    // Проверяет, пуста ли строка
    let s = String::new();
    let a = s.is_empty();
    println!("5) is_empty() '' ► {}", a);
    // 5) is_empty() '' ► true

    // push() — Bir belgi qo'shadi
    // Добавляет один символ
    let mut s = String::from("salom");
    s.push('!');
    println!("6) push() ► {}", s);
    // 6) push() ► salom!

    // push_str() — String qo'shadi
    // Добавляет строку
    let mut s = String::from("salom");
    s.push_str(" dunyo");
    println!("7) push_str() ► {}", s);
    // 7) push_str() ► salom dunyo

    // pop() — Oxirgi belgini olib tashlaydi
    // Удаляет последний символ
    let mut s = String::from("salom!");
    let a = s.pop();
    println!("8) pop() ► {:?}, qolgani: {}", a, s);
    // 8) pop() ► Some('!'), qolgani: salom

    // clear() — Stringni tozalaydi / Очищает строку
    let mut s = String::from("salom dunyo");
    s.clear();
    println!("9) clear() ► '{}'", s);
    // 9) clear() ► ''

    // truncate() — Stringni berilgan uzunlikka kesadi
    // Обрезает строку до указанной длины
    let mut s = String::from("salom dunyo");
    s.truncate(5);
    println!("10) truncate() ► '{}'", s);
    // 10) truncate() ► 'salom'

    // retain() — Shart bo'yicha belgilarni saqlaydi
    // Сохраняет символы по условию
    let mut s = String::from("salom dunyo rust123");
    s.retain(|c| c != ' ');
    println!("11) retain() (probelsiz) ► {}", s);
    // 11) retain() (probelsiz) ► salomdunyorust123

    // retain() — Shart bo'yicha belgilarni saqlaydi
    // retain() — Сохраняет символы по условию
    let mut s = String::from("salom123dunyo");
    s.retain(|c| c.is_alphabetic());
    println!("11) retain() (faqat harflar) ► {}", s);
    // 11) retain() (faqat harflar) ► salomdunyo

    // contains() — Qism string borligini tekshiradi
    // Проверяет наличие подстроки
    let s = String::from("salom dunyo");
    let a = s.contains("dunyo");
    println!("12) contains() 'dunyo' ► {}", a);
    // 12) contains() 'dunyo' ► true

    // contains() — Qism string borligini tekshiradi
    // Проверяет наличие подстроки
    let a = s.contains("rust");
    println!("12) contains() 'rust' ► {}", a);
    // 12) contains() 'rust' ► false

    // replace() — Qism stringni almashtiradi
    // Заменяет подстроку
    let s = String::from("salom dunyo");
    let b = s.replace("dunyo", "rust");
    println!("13) replace() ► {}", b);
    // 13) replace() ► salom rust

    // find() — Qism string indeksini qaytaradi
    // Возвращает индекс подстроки
    let s = String::from("salom dunyo");
    let a = s.find("dunyo");
    println!("23) find() ► {:?}", a);
    // 23) find() ► Some(6)

    // find() — Qism string indeksini qaytaradi
    // Возвращает индекс подстроки
    let a = s.find("rust");
    println!("23) find() (topilmadi) ► {:?}", a);
    // 23) find() (topilmadi) ► None

    // split() — Berilgan belgi bo'yicha ajratadi
    // Разделяет по указанному символу
    let s = String::from("salom dunyo rust");
    let a: Vec<&str> = s.split(" ").collect();
    println!("14) split() ► {:?}", a);
    // 14) split() ► ["salom", "dunyo", "rust"]

    // split_once() — Bir marta ajratadi
    // Разделяет один раз
    let s = String::from("ism:familiya:yosh");
    let a = s.split_once(":");
    println!("15) split_once() ► {:?}", a);
    // 15) split_once() ► Some(("ism", "familiya:yosh"))

    // split_once() — Bir marta ajratadi
    // Разделяет один раз
    let s = String::from("salom dunyo");
    let a = s.split_once("xyz");
    println!("15) split_once() (topilmadi) ► {:?}", a);
    // 15) split_once() (topilmadi) ► None

    // lines() — Qator bo'yicha iterator qaytaradi
    // Возвращает итератор по строкам
    let s = String::from("salom\ndunyo\nrust");
    println!("26) lines() ►");
    for qator in s.lines() {
        println!("   {}", qator);
    }
    // 26) lines() ►
    //    salom
    //    dunyo
    //    rust

    // trim() — Ikki tomondan bo'sh joylarni olib tashlaydi
    // Удаляет пробелы с обоих концов
    let s = String::from("  salom dunyo  ");
    let a = s.trim();
    println!("16) trim() ► '{}'", a);
    // 16) trim() ► 'salom dunyo'

    // trim_start() — Chap tomondan bo'sh joylarni olib tashlaydi
    // Удаляет пробелы слева
    let s = String::from(" salom1 ");
    let a = s.trim_start();
    println!("17) trim_start() ► '{}'", a);
    // 17) trim_start() ► 'salom1 '

    // trim_end() — O'ng tomondan bo'sh joylarni olib tashlaydi
    // Удаляет пробелы справа
    let s = String::from(" salom2 ");
    let a = s.trim_end();
    println!("18) trim_end() ► '{}'", a);
    // 18) trim_end() ► ' salom2'

    // to_lowercase() — Kichik harflarga o'tkazadi
    // Преобразует в нижний регистр
    let s = String::from("SALOM Dunyo");
    let a = s.to_lowercase();
    println!("19) to_lowercase() ► {}", a);
    // 19) to_lowercase() ► salom dunyo

    // to_uppercase() — Katta harflarga o'tkazadi
    // Преобразует в верхний регистр
    let s = String::from("Salom Dunyo");
    let a = s.to_uppercase();
    println!("20) to_uppercase() ► {}", a);
    // 20) to_uppercase() ► SALOM DUNYO

    // starts_with() — Qanday boshlanishini tekshiradi
    // Проверяет начало строки
    let s = String::from("salom");
    let a = s.starts_with("sal");
    println!("21) starts_with() 'sal' ► {}", a);
    // 21) starts_with() 'sal' ► true

    // starts_with() — Qanday boshlanishini tekshiradi
    // Проверяет начало строки
    let s = String::from("salom");
    let a = s.starts_with("xyz");
    println!("21) starts_with() 'xyz' ► {}", a);
    // 21) starts_with() 'xyz' ► false

    // ends_with() — Qanday tugashini tekshiradi
    // Проверяет конец строки
    let s = String::from("salom");
    let a = s.ends_with("lom");
    println!("22) ends_with() 'lom' ► {}", a);
    // 22) ends_with() 'lom' ► true

    // ends_with() — Qanday tugashini tekshiradi
    // Проверяет конец строки
    let s = String::from("salom");
    let a = s.ends_with("xyz");
    println!("22) ends_with() 'xyz' ► {}", a);
    // 22) ends_with() 'xyz' ► false

    // repeat() — Stringni takrorlaydi
    // Повторяет строку
    let s = String::from("ha ");
    let a = s.repeat(3);
    println!("24) repeat() ► {}", a);
    // 24) repeat() ► ha ha ha

    // chars() — Belgi bo'yicha iterator qaytaradi
    // Возвращает итератор по символам
    let s = String::from("salom");
    println!("25) chars() ►");
    for harf in s.chars() {
        println!("   {}", harf);
    }
    // 25) chars() ►
    //    s
    //    a
    //    l
    //    o
    //    m

    // chars() — Belgi bo'yicha iterator qaytaradi
    // Возвращает итератор по символам
    let a = s.chars().count();
    println!("25) chars().count() ► {}", a);
    // 25) chars().count() ► 5

    // parse() — Stringni songa o'tkazadi
    // Преобразует строку в число
    let s = "42";
    let a: i32 = s.parse().unwrap();
    println!("27) parse() i32 ► {}", a + 1);
    // 27) parse() i32 ► 43

    // parse() — Stringni songa o'tkazadi
    // Преобразует строку в число
    let s = "3.14";
    let a: f64 = s.parse().unwrap();
    println!("27) parse() f64 ► {}", a);
    // 27) parse() f64 ► 3.14

    // parse() — Stringni songa o'tkazadi
    // Преобразует строку в число
    let s = "true";
    let a: bool = s.parse().unwrap();
    println!("27) parse() bool ► {}", a);
    // 27) parse() bool ► true

    // as_bytes() — Baytlar sratini qaytaradi (ownership saqlanadi)
    // Возвращает срез байтов (ownership сохраняется)
    let s = String::from("salom");
    let bytes = s.as_bytes();
    println!("28) as_bytes() ► {:?}", bytes);
    // 28) as_bytes() ► [115, 97, 108, 111, 109]
    println!("28) as_bytes() dan keyin s hali bor: '{}'", s);
    // 28) as_bytes() dan keyin s hali bor: 'salom'

    // into_bytes() — Baytlar vektoriga o'tkazadi (ownership o'tadi)
    // Преобразует в вектор байтов (ownership переходит)
    let s = String::from("salom");
    let bytes = s.into_bytes();
    println!("29) into_bytes() ► {:?}", bytes);
    // 29) into_bytes() ► [115, 97, 108, 111, 109]
    // println!("{}", s); ❌ XATO! s endi yo'q

    // format!() — Makros yordamida string yaratadi
    // Создаёт строку с помощью макроса
    let ism = "Dilshod";
    let yosh = 25;
    let matn = format!("Ism: {}, Yosh: {}", ism, yosh);
    println!("30) format!() ► {}", matn);
    // 30) format!() ► Ism: Dilshod, Yosh: 25

    // join() — Vektordagi elementlarni birlashtiradi
    // Объединяет элементы вектора
    let sozlar = vec!["rust", "dasturlash", "tili"];
    let gap = sozlar.join(" ");
    println!("31) join() ► {}", gap);
    // 31) join() ► rust dasturlash tili

    // join() — Vektordagi elementlarni birlashtiradi
    // Объединяет элементы вектора
    let raqamlar = vec!["1", "2", "3"];
    let natija = raqamlar.join("-");
    println!("31) join() ► {}", natija);
    // 31) join() ► 1-2-3

    // insert() — Belgini o'rtaga qo'shadi
    // Вставляет символ в середину
    let mut s = String::from("hello");
    s.insert(2, 'x');
    println!("32) insert() ► {}", s);
    // 32) insert() ► hexllo

    // insert() — Belgini o'rtaga qo'shadi
    // Вставляет символ в середину
    let mut s = String::from("salom");
    s.insert(0, 'A');
    println!("32) insert() ► {}", s);
    // 32) insert() ► Asalom

    // insert_str() — Stringni o'rtaga qo'shadi
    // Вставляет строку в середину
    let mut s = String::from("salom dunyo");
    s.insert_str(6, "katta ");
    println!("33) insert_str() ► {}", s);
    // 33) insert_str() ► salom katta dunyo

    // insert_str() — Stringni o'rtaga qo'shadi
    // Вставляет строку в середину
    let mut s = String::from("rust");
    s.insert_str(0, "learn ");
    println!("33) insert_str() ► {}", s);
    // 33) insert_str() ► learn rust

    // remove() — Indeks bo'yicha belgini o'chiradi
    // Удаляет символ по индексу
    let mut s = String::from("salom");
    let a = s.remove(2);
    println!("34) remove() ► {} , qolgani: {}", a, s);
    // 34) remove() ► l , qolgani: saom

    // remove() — Indeks bo'yicha belgini o'chiradi
    // Удаляет символ по индексу
    let mut s = String::from("hello");
    let a = s.remove(0);
    println!("34) remove() ► {} , qolgani: {}", a, s);
    // 34) remove() ► h , qolgani: ello

    // get() — Xavfsiz indekslash (byte index)
    // Безопасная индексация (байтовый индекс)
    let s = String::from("salom");
    let a = s.get(0..2);
    println!("35) get() ► {:?}", a);
    // 35) get() ► Some("sa")

    // get() — Xavfsiz indekslash (byte index)
    // Безопасная индексация (байтовый индекс)
    let s = String::from("salom");
    let a = s.get(0..1);
    println!("35) get() ► {:?}", a);
    // 35) get() ► Some("s")

    // get() — Xavfsiz indekslash (byte index)
    // Безопасная индексация (байтовый индекс)
    let s = String::from("salom");
    let a = s.get(10..20);
    println!("35) get() (xato) ► {:?}", a);
    // 35) get() (xato) ► None

    // is_ascii() — ASCII ekanligini tekshiradi
    // Проверяет, является ли ASCII
    let s = "hello";
    let a = s.is_ascii();
    println!("36) is_ascii() 'hello' ► {}", a);
    // 36) is_ascii() 'hello' ► true

    // is_ascii() — ASCII ekanligini tekshiradi
    // Проверяет, является ли ASCII
    let s = "Привет";
    let a = s.is_ascii();
    println!("36) is_ascii() 'Привет' ► {}", a);
    // 36) is_ascii() 'Привет' ► false

    // is_ascii() — ASCII ekanligini tekshiradi
    // Проверяет, является ли ASCII
    let s = "hello123!";
    let a = s.is_ascii();
    println!("36) is_ascii() 'hello123!' ► {}", a);
    // 36) is_ascii() 'hello123!' ► true

    // escape_unicode() — Unicode belgilarini escape qiladi
    // Экранирует Unicode символы
    let s = "😊";
    let a: String = s.escape_unicode().collect();
    println!("37) escape_unicode() ► {}", a);
    // 37) escape_unicode() ► \u{1f60a}

    // escape_unicode() — Unicode belgilarini escape qiladi
    // Экранирует Unicode символы
    let s = "Привет";
    let a: String = s.escape_unicode().collect();
    println!("37) escape_unicode() ► {}", a);
    // 37) escape_unicode() ► \u{41f}\u{440}\u{438}\u{432}\u{435}\u{442}

    // escape_unicode() — Unicode belgilarini escape qiladi
    // Экранирует Unicode символы
    let s = "a😊b";
    let a: String = s.escape_unicode().collect();
    println!("37) escape_unicode() ► {}", a);
    // 37) escape_unicode() ► a\u{1f60a}b
}
// #================================================================================================================================================#
// # |  №  | Metod                    | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |   1 | String::new()            | Yangi bo'sh string yaratadi                          | Создаёт новую пустую строку                          |
// # |   2 | String::from()           | String literaldan string yaratadi                    | Создаёт строку из строкового литерала                |
// # |   3 | to_string()              | Qiymatni stringga aylantiradi                        | Преобразует значение в строку                        |
// # |   4 | len()                    | String uzunligini (baytda) qaytaradi                 | Возвращает длину строки (в байтах)                   |
// # |   5 | is_empty()               | String bo'shligini tekshiradi                        | Проверяет, пуста ли строка                           |
// # |   6 | push()                   | Bir belgi qo'shadi                                   | Добавляет один символ                                |
// # |   7 | push_str()               | String qo'shadi                                      | Добавляет строку                                     |
// # |   8 | pop()                    | Oxirgi belgini olib tashlaydi                        | Удаляет последний символ                             |
// # |   9 | clear()                  | Stringni tozalaydi                                   | Очищает строку                                       |
// # |  10 | truncate()               | Stringni berilgan uzunlikka kesadi                   | Обрезает строку до указанной длины                   |
// # |  11 | retain()                 | Shart bo'yicha belgilarni saqlaydi                   | Сохраняет символы по условию                         |
// # |  12 | contains()               | Qism string borligini tekshiradi                     | Проверяет наличие подстроки                          |
// # |  13 | replace()                | Qism stringni almashtiradi                           | Заменяет подстроку                                   |
// # |  14 | split()                  | Berilgan belgi bo'yicha ajratadi                     | Разделяет по указанному символу                      |
// # |  15 | split_once()             | Bir marta ajratadi                                   | Разделяет один раз                                   |
// # |  16 | trim()                   | Ikki tomondan bo'sh joylarni olib tashlaydi          | Удаляет пробелы с обоих концов                       |
// # |  17 | trim_start()             | Chap tomondan bo'sh joylarni olib tashlaydi          | Удаляет пробелы слева                                |
// # |  18 | trim_end()               | O'ng tomondan bo'sh joylarni olib tashlaydi          | Удаляет пробелы справа                               |
// # |  19 | to_lowercase()           | Kichik harflarga o'tkazadi                           | Преобразует в нижний регистр                         |
// # |  20 | to_uppercase()           | Katta harflarga o'tkazadi                            | Преобразует в верхний регистр                        |
// # |  21 | starts_with()            | Qanday boshlanishini tekshiradi                      | Проверяет начало строки                              |
// # |  22 | ends_with()              | Qanday tugashini tekshiradi                          | Проверяет конец строки                               |
// # |  23 | find()                   | Qism string indeksini qaytaradi                      | Возвращает индекс подстроки                          |
// # |  24 | repeat()                 | Stringni takrorlaydi                                 | Повторяет строку                                     |
// # |  25 | chars()                  | Belgi bo'yicha iterator qaytaradi                    | Возвращает итератор по символам                      |
// # |  26 | lines()                  | Qator bo'yicha iterator qaytaradi                    | Возвращает итератор по строкам                       |
// # |  27 | parse()                  | Stringni songa o'tkazadi                             | Преобразует строку в число                           |
// # |  28 | as_bytes()               | Baytlar sratini qaytaradi (ownership saqlanadi)      | Возвращает срез байтов (ownership сохраняется)       |
// # |  29 | into_bytes()             | Baytlar vektoriga o'tkazadi (ownership o'tadi)       | Преобразует в вектор байтов (ownership переходит)    |
// # |  30 | format!()                | Makros yordamida string yaratadi                     | Создаёт строку с помощью макроса                     |
// # |  31 | join()                   | Vektordagi elementlarni birlashtiradi                | Объединяет элементы вектора                          |
// # |  32 | insert()                 | Belgini o'rtaga qo'shadi                             | Вставляет символ в середину                          |
// # |  33 | insert_str()             | Stringni o'rtaga qo'shadi                            | Вставляет строку в середину                          |
// # |  34 | remove()                 | Indeks bo'yicha belgini o'chiradi                    | Удаляет символ по индексу                            |
// # |  35 | get()                    | Xavfsiz indekslash (byte index)                      | Безопасная индексация (байтовый индекс)              |
// # |  36 | is_ascii()               | ASCII ekanligini tekshiradi                          | Проверяет, является ли ASCII                         |
// # |  37 | escape_unicode()         | Unicode belgilarini escape qiladi                    | Экранирует Unicode символы                           |
// #================================================================================================================================================#