// #================================================================================================================================================#
// #                                                                 FUNCTIONS                                                                      #
// #                                             fn — FUNKSIYA E'LON QILISH  / fn — ОБЪЯВЛЕНИЕ ФУНКЦИИ                                              #
// #================================================================================================================================================#

fn main() {

    // oddiy funksiya — hech narsa qaytarmaydi
    // простая функция — ничего не возвращает
    fn salom() {
        println!("Salom Rust!");
    }
    salom();
    // Salom Rust!

    // parametrli funksiya
    // функция с параметрами
    fn qo_sh(a: i32, b: i32) {
        println!("{}", a + b);
    }
    qo_sh(3, 5);
    // 8

    // qiymat qaytaruvchi funksiya — -> type
    // функция с возвратом значения
    fn ko_payt(a: i32, b: i32) -> i32 {
        a * b // oxirgi expression avtomatik qaytadi — return shart emas!
    }
    let natija = ko_payt(4, 5);
    println!("{}", natija);
    // 20

    // return — erta chiqish
    // ранний выход из функции
    fn musbatmi(x: i32) -> bool {
        if x < 0 {
            return false; // erta chiqish
        }
        true // oxirgi expression
    }
    println!("{}", musbatmi(5));
    println!("{}", musbatmi(-3));
    // true
    // false

    // tuple qaytarish — bir nechta qiymat
    // возврат нескольких значений через кортеж
    fn min_max(a: i32, b: i32) -> (i32, i32) {
        if a < b { (a, b) } else { (b, a) }
    }
    let (kichik, katta) = min_max(10, 3);
    println!("min={} max={}", kichik, katta);
    // min=3 max=10

    // () — hech narsa qaytarmaydi (unit type)
    // возврат пустого кортежа
    fn hech_narsa() -> () {
        println!("hech narsa qaytarmayman");
    }
    hech_narsa();
    // hech narsa qaytarmayman

    // expression vs statement
    // выражение vs оператор
    fn kvadrat(x: i32) -> i32 {
        x * x  // ; yo'q → expression → qaytaradi
        // x * x; → statement → qaytarmaydi → xato!
    }
    println!("{}", kvadrat(5));
    // 25

    // rekursiya — o'zini chaqirish
    // рекурсия — вызов самой себя
    // → 5 * faktorial(4)
          // → 4 * faktorial(3)
                // → 3 * faktorial(2)
                      // → 2 * faktorial(1)
                            // → 1 * faktorial(0)
                                  // → 0 == 0 → 1 qaytdi

    fn faktorial(n: u64) -> u64 {
        if n == 0 { 1 } else { n * faktorial(n - 1) }
    }
    println!("{}", faktorial(5));
    // 120

    // funksiya ichida funksiya
    // функция внутри функции
    fn tashqi(x: i32) -> i32 {
        fn ichki(y: i32) -> i32 {
            y * 2
        }
        ichki(x) + 1
    }
    println!("{}", tashqi(5));
    // 11

    // generik funksiya — har xil type bilan ishlaydi
    // обобщённая функция — работает с разными типами
    fn birinchi<T>(arr: &[T]) -> &T {
        &arr[0]
    }
    println!("{}", birinchi(&[1, 2, 3]));
    println!("{}", birinchi(&["a", "b", "c"]));
    // 1
    // a

    // closure — o'zgaruvchiga yozilgan funksiya (Python lambda)
    // замыкание — функция в переменной
    let qo_sh = |a: i32, b: i32| a + b;
    println!("{}", qo_sh(3, 5));
    // 8

    // higher-order function — funksiyani argument sifatida berish
    // функция принимающая другую функцию
    fn ikki_marta(f: fn(i32) -> i32, x: i32) -> i32 {
        f(f(x))
    }
    fn ikki_kat(x: i32) -> i32 { x * 2 }
    println!("{}", ikki_marta(ikki_kat, 3));
    // 12  → ikki_kat(3)=6 → ikki_kat(6)=12
}
// #================================================================================================================================================#
// # |  №  | Funksiya turi                  | Tavsif (UZ)                                          | Описание (RU)                                  |
// #================================================================================================================================================#
// # |   1 | fn nom() {}                    | Oddiy funksiya (hech narsa qaytarmaydi)              | Простая функция (ничего не возвращает)         |
// # |   2 | fn nom(a: i32, b: i32) {}      | Parametrli funksiya                                  | Функция с параметрами                          |
// # |   3 | fn nom() -> i32 { value }      | Qiymat qaytaruvchi (expression)                      | Функция с возвратом (выражение)                |
// # |   4 | return value;                  | Erta chiqish (early return)                          | Ранний возврат                                 |
// # |   5 | fn nom() -> (i32, i32)         | Tuple qaytarish (bir nechta qiymat)                  | Возврат кортежа (несколько значений)           |
// # |   6 | fn nom() -> () { }             | Unit type — hech narsa qaytarmaydi                   | Пустой кортеж — ничего не возвращает           |
// # |   7 | expression vs statement        | ; bilan farqi (expression qaytaradi)                 | Разница: выражение возвращает, оператор — нет  |
// # |   8 | fn nom() { fn ichki() {} }     | Funksiya ichida funksiya                             | Функция внутри функции                         |
// # |   9 | fn nom<T>(arr: &[T]) -> &T     | Generik funksiya                                     | Обобщённая функция                             |
// # |  10 | let f = \|a, b\| a + b         | Closure (lambda)                                     | Замыкание (лямбда)                             |
// # |  11 | fn hof(f: fn(i32)->i32, x)     | Higher-order function (funksiya qabul qiladi)        | Функция высшего порядка (принимает функцию)    |
// # |  12 | fn rekursiya() { rekursiya() } | Rekursiya (o'zini chaqirish)                         | Рекурсия (вызов самой себя)                    |
// #================================================================================================================================================#