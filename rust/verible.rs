// #================================================================================================================================================#
// #                                          LET  |  LET MUT  |  CONST  |  STATIC  |  STATIC MUT                                                   |
// #                                 O'ZGARUVCHILAR VA KONSTANTALAR — RUST DA XOTIRA BOSHQARUVINING ASOSI.                                          |
// #                                    ПЕРЕМЕННЫЕ И КОНСТАНТЫ — ОСНОВА УПРАВЛЕНИЯ ПАМЯТЬЮ В RUST.                                                  |
// #================================================================================================================================================#
// #                                                        TAQQOSLASH JADVALI                                                                      |
// #                                                        ТАБЛИЦА СРАВНЕНИЯ                                                                       |
// #================================================================================================================================================#
// # Konstruksiya  | O'zgaradi | Tip  | vaqt  | Lifetime | Xotira  | safe | Unsafe |     Qayerda     |           Real ishlatish                     |
// #================================================================================================================================================#
// # let           |    🔴     |  🔴  |   🔴  |  scope   |  stack  |  🟢  |   🔴   | funksiya ichida | oddiy o'zgaruvchi                            |
// # let mut       |    🟢     |  🔴  |   🔴  |  scope   |  stack  |  🟢  |   🔴   | funksiya ichida | o'zgaruvchan qiymat                          |
// # const         |    🔴     |  🟢  |   🟢  | 'static  |  inline |  🟢  |   🔴   | istalgan joyda  | matematik konstantalar, konfiguratsiya       |
// # static        |    🔴     |  🟢  |   🟢  | 'static  |  global |  🟢  |   🔴   | global          | string literallar, global config, OnceLock   |
// # static mut    |    🟢     |  🟢  |   🟢  | 'static  |  global |  🔴  |   🟢   | global (xavfli!)| ISHLATMA — AtomicT yoki Mutex ishlat         |
// #================================================================================================================================================#

#![allow(dead_code, unused)]

fn main() {

    // let — immutable o'zgaruvchi, o'zgartirib bo'lmaydi
    // иммутабельная переменная, нельзя изменить
    let x = 5;
    println!("{}", x);
    // 5

    // let — tip annotatsiya bilan
    // с аннотацией типа
    let y: i32 = 10;
    println!("{}", y);
    // 10

    // let — bir nechta qiymat (destructuring)
    // несколько значений (деструктуризация)
    let (a, b) = (1, 2);
    println!("{} {}", a, b);
    // 1 2

    // let — array destructuring
    // деструктуризация массива
    let [p, q, r] = [10, 20, 30];
    println!("{} {} {}", p, q, r);
    // 10 20 30

    // let _ — qiymatni ignore qilish
    // игнорирование значения
    let _ = 999;

    // let — shadowing (qayta e'lon qilish)
    // затенение (повторное объявление)
    let z = 5;
    let z = z + 1;
    let z = z * 2;
    println!("{}", z);
    // 12

    // let — shadowing bilan tip o'zgartirish
    // изменение типа через затенение
    let s = "salom";
    let s = s.len();
    println!("{}", s);
    // 5

    // let — blok ichida scope
    // область видимости внутри блока
    let n = {
        let a = 3;
        let b = 4;
        a + b
    };
    println!("{}", n);
    // 7

    // let mut — qiymatni o'zgartirish mumkin
    // можно изменить значение
    let mut count = 0;
    count += 1;
    count += 1;
    println!("{}", count);
    // 2

    // let mut — string o'zgartirish
    // изменение строки
    let mut s = String::from("salom");
    s.push_str(", dunyo!");
    println!("{}", s);
    // salom, dunyo!

    // let mut — swap
    // обмен значениями
    let mut a = 1;
    let mut b = 2;
    std::mem::swap(&mut a, &mut b);
    println!("{} {}", a, b);
    // 2 1

    // let mut — mutable reference olish
    // получение мутабельной ссылки
    let mut v = vec![1, 2, 3];
    let first = &mut v[0];
    *first = 99;
    println!("{:?}", v);
    // [99, 2, 3]

    // let mut — loop ichida o'zgartirish
    // изменение внутри цикла
    let mut sum = 0;
    for i in 1..=5 {
        sum += i;
    }
    println!("{}", sum);
    // 15

    // const — tip annotatsiya SHART, global yoki lokal bo'lishi mumkin
    // аннотация типа ОБЯЗАТЕЛЬНА, может быть глобальной или локальной
    const MAX_SON: u32 = 100_000;
    println!("{}", MAX_SON);
    // 100000

    // const — matematik ifoda
    // математическое выражение
    const SONIYALAR: u32 = 60 * 60 * 24;
    println!("{}", SONIYALAR);
    // 86400

    // const — string
    // строковая константа
    const VERSIYA: &str = "1.0.0";
    println!("{}", VERSIYA);
    // 1.0.0

    // const — massiv
    // константный массив
    const RANGLAR: [&str; 3] = ["qizil", "yashil", "ko'k"];
    println!("{:?}", RANGLAR);
    // ["qizil", "yashil", "ko'k"]

    // const fn — kompile vaqtida hisoblash
    // вычисление во время компиляции
    const fn kvadrat(x: u32) -> u32 {
        x * x
    }
    const NATIJA: u32 = kvadrat(5);
    println!("{}", NATIJA);
    // 25

    // static — global o'zgarmas, 'static lifetime
    // глобальная неизменяемая, 'static lifetime
    static DASTUR_NOMI: &str = "RustLearner";
    println!("{}", DASTUR_NOMI);
    // RustLearner

    // static — butun son
    // целочисленная статическая
    static MAX_ULANISH: u32 = 1000;
    println!("{}", MAX_ULANISH);
    // 1000

    // static — 'static reference qaytarish
    // возврат 'static ссылки
    fn versiya() -> &'static str {
        static V: &str = "2.0.0";
        V
    }
    println!("{}", versiya());
    // 2.0.0

    // static — thread safe global (OnceLock bilan)
    // потокобезопасная глобальная (с OnceLock)
    use std::sync::OnceLock;
    static KONFIGURATSIYA: OnceLock<String> = OnceLock::new();
    KONFIGURATSIYA.get_or_init(|| String::from("default"));
    println!("{:?}", KONFIGURATSIYA.get());
    // Some("default")

    // static mut — faqat unsafe ichida o'qish/yozish mumkin
    // чтение/запись только внутри unsafe
    static mut HISOBLAGICH: u32 = 0;

    unsafe {
        HISOBLAGICH += 1;
        let val = std::ptr::read(&raw const HISOBLAGICH);
        println!("{}", val);
    }
    // 1

    // static mut — real hayotda ISHLATMA
    // в реальном коде НЕ ИСПОЛЬЗУЙ
    // buning o'rniga: Mutex<T>, AtomicU32, OnceLock ishlatiladi
    // вместо этого: Mutex<T>, AtomicU32, OnceLock

    // static mut — to'g'ri alternativa: AtomicU32
    // правильная альтернатива: AtomicU32
    use std::sync::atomic::{AtomicU32, Ordering};
    static XAVFSIZ_HISOBLAGICH: AtomicU32 = AtomicU32::new(0);
    XAVFSIZ_HISOBLAGICH.fetch_add(1, Ordering::SeqCst);
    println!("{}", XAVFSIZ_HISOBLAGICH.load(Ordering::SeqCst));
    // 1
}


// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                               LET                                                                                            |
// #================================================================================================================================================#
// # |   1 | let x = 5                | Immutable o'zgaruvchi                                | Иммутабельная переменная                             |
// # |   2 | let x: i32 = 5           | Tip annotatsiya bilan                                | С аннотацией типа                                    |
// # |   3 | let (a, b) = (1, 2)      | Tuple destructuring                                  | Деструктуризация кортежа                             |
// # |   4 | let [a, b] = [1, 2]      | Array destructuring                                  | Деструктуризация массива                             |
// # |   5 | let _ = x                | Qiymatni ignore qilish                               | Игнорирование значения                               |
// # |   6 | let x = { ... }          | Blok qiymati                                         | Значение блока                                       |
// # |   7 | let x = x + 1 (shadowing)| Qayta e'lon — tip ham o'zgarishi mumkin              | Переобъявление — тип тоже может измениться           |
// #================================================================================================================================================#
// # |                                             LET MUT                                                                                          |
// #================================================================================================================================================#
// # |   8 | let mut x = 5            | Mutable o'zgaruvchi                                  | Мутабельная переменная                               |
// # |   9 | x += 1                   | Qiymatni o'zgartirish                                | Изменение значения                                   |
// # |  10 | &mut x                   | Mutable reference olish                              | Получение мутабельной ссылки                         |
// # |  11 | mem::swap(&mut a, &mut b)| Ikki qiymatni almashtirish                           | Обмен двух значений                                  |
// #================================================================================================================================================#
// # |                                              CONST                                                                                           |
// #================================================================================================================================================#
// # |  12 | const X: u32 = 5         | Kompile vaqtida ma'lum, tip SHART                    | Известна на этапе компиляции, тип ОБЯЗАТЕЛЕН         |
// # |  13 | const fn f(x: T) -> T    | Kompile vaqtida bajariladigan funksiya               | Функция, выполняемая во время компиляции             |
// # |  14 | 100_000 (underscore)      | Sonlarni o'qilishi uchun ajratish                    | Разделение числа для читаемости                     |
// #================================================================================================================================================#
// # |                                             STATIC                                                                                           |
// #================================================================================================================================================#
// # |  15 | static X: &str = "..."   | Global o'zgarmas, 'static lifetime                   | Глобальная неизменяемая, 'static lifetime            |
// # |  16 | fn f() -> &'static str   | 'static reference qaytarish                          | Возврат 'static ссылки                               |
// # |  17 | OnceLock<T>              | Thread-safe global initsializatsiya                  | Потокобезопасная глобальная инициализация            |
// #================================================================================================================================================#
// # |                                           STATIC MUT                                                                                         |
// #================================================================================================================================================#
// # |  18 | static mut X: u32 = 0    | Global mutable — XAVFLI, unsafe kerak                | Глобальная мутабельная — ОПАСНО, нужен unsafe        |
// # |  19 | unsafe { X += 1 }        | Faqat unsafe ichida o'qish/yozish                    | Чтение/запись только внутри unsafe                   |
// # |  20 | AtomicU32 (alternativa)  | static mut o'rniga xavfsiz variant                   | Безопасная альтернатива static mut                   |
// #================================================================================================================================================#