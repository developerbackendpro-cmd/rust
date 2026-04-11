// #================================================================================================================================================#
// #                                                    LET ELSE  |  MATCHES!  |  TODO!                                                             #
// #                                    LET ELSE — QIYMAT OLOLMASANG CHIQ. MATCHES! — PATTERN TEKSHIR. TODO! — KEYINROQ.                            #
// #                            LET ELSE — НЕ ПОЛУЧИЛ ЗНАЧЕНИЕ — ВЫЙДИ. MATCHES! — ПРОВЕРЬ ПАТТЕРН. TODO! — СДЕЛАЮ ПОЗЖЕ.                           #
// #================================================================================================================================================#
#![allow(dead_code, unused)]

fn main() {

    // let else — pattern mos kelmasa else bloki ishlaydi (return / break / panic)
    // если паттерн не совпадает — выполняется else блок (return / break / panic)
    let s = Some(42);
    let Some(val) = s else {
        println!("qiymat yo'q");
        return;
    };
    println!("{}", val);
    // 42

    // let else — None kelsa chiqib ketish
    // выход из функции если None
    fn juft_son(x: Option<i32>) -> i32 {
        let Some(n) = x else { return -1 };
        n * 2
    }
    println!("{}", juft_son(Some(5)));
    println!("{}", juft_son(None));
    // 10
    // -1

    // let else — Result bilan ishlatish
    // использование с Result
    fn parse_son(s: &str) -> i32 {
        let Ok(n) = s.parse::<i32>() else { return 0 };
        n
    }
    println!("{}", parse_son("99"));
    println!("{}", parse_son("abc"));
    // 99
    // 0

    // let else — enum bilan ishlatish
    // использование с enum
    enum Holat { Yoqilgan, Ochirilgan }
    let h = Holat::Yoqilgan;
    let Holat::Yoqilgan = h else {
        println!("ochirilgan");
        return;
    };
    println!("yoqilgan");
    // yoqilgan

    // matches!(qiymat, pattern) — true/false qaytaradi
    // возвращает true/false — совпадает ли паттерн
    let x = Some(5);
    println!("{}", matches!(x, Some(_)));
    // true

    // matches! — None tekshirish
    // проверка None
    let x: Option<i32> = None;
    println!("{}", matches!(x, None));
    // true

    // matches! — enum variant tekshirish
    // проверка варианта enum
    enum Rang { Qizil, Yashil, Kok }
    let r = Rang::Yashil;
    println!("{}", matches!(r, Rang::Yashil));
    // true

    // matches! — shart bilan birga (guard)
    // с условием (guard)
    let x = Some(10);
    println!("{}", matches!(x, Some(n) if n > 5));
    // true

    // matches! — bir nechta pattern
    // несколько паттернов
    let x = 3;
    println!("{}", matches!(x, 1 | 2 | 3));
    // true

    // matches! — filter() bilan ishlatish
    // использование с filter()
    let v = vec![Some(1), None, Some(3), None, Some(5)];
    let count = v.iter().filter(|x| matches!(x, Some(_))).count();
    println!("{}", count);
    // 3

    // todo!() — hali yozilmagan kod (panic chiqaradi)
    // код ещё не написан (вызывает панику)
    fn hisobla(_x: i32) -> i32 {
        todo!()
    }
    // hisobla(5); // panics: not yet implemented

    // todo!("xabar") — o'z xabar bilan
    // с собственным сообщением
    fn yuklash() {
        todo!("ma'lumotlar bazasi ulanmagan")
    }
    // yuklash(); // panics: ma'lumotlar bazasi ulanmagan

    // unimplemented!() — intentional: bu qo'llab quvvatlanmaydi
    // намеренно: это не поддерживается
    fn eski_api() {
        unimplemented!("bu metod endi ishlatilmaydi")
    }
    // eski_api(); // panics: not implemented: bu metod endi ishlatilmaydi

    // unreachable!() — bu yerga hech qachon kelmasligi kerak
    // сюда никогда не должны доходить
    let x = 2;
    let _natija = match x {
        1 => "bir",
        2 => "ikki",
        _ => unreachable!("bu holat mumkin emas"),
    };
    println!("{}", _natija);
    // ikki

    // todo! — compile bo'ladi, ishga tushmaydi (placeholder)
    // компилируется, но не запускается (заглушка)
    fn trait_metod() -> String {
        todo!()
    }
    let _ = std::panic::catch_unwind(trait_metod);
    println!("kod kompilyatsiya bo'ldi");
    // kod kompilyatsiya bo'ldi
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |   1 | let Pat = x else { }     | Pattern mos kelmasa else ishlaydi                    | Если паттерн не совпал — выполняется else            |
// # |   2 | let Some(n) = opt else{} | Option dan xavfsiz olish                             | Безопасное извлечение из Option                      |
// # |   3 | let Ok(n) = res else {}  | Result dan xavfsiz olish                             | Безопасное извлечение из Result                      |
// # |   4 | else { return }          | Else blok diverge qilishi shart (return/break/panic) | Else блок должен расходиться (return/break/panic)    |
// #================================================================================================================================================#
// # |   5 | matches!(x, Pat)         | Pattern mos kelishini tekshirish                     | Проверка совпадения паттерна                         |
// # |   6 | matches!(x, Pat if cond) | Pattern + qo'shimcha shart                           | Паттерн + дополнительное условие                     |
// # |   7 | matches!(x, A | B | C)   | Bir nechta patterndan biri                           | Один из нескольких паттернов                         |
// # |   8 | filter(|x| matches!(...))| Iterator bilan birga ishlatish                       | Использование вместе с итератором                    |
// #================================================================================================================================================#
// # |   9 | todo!()                  | Hali yozilmagan kod, panic chiqaradi                 | Код ещё не написан, вызывает панику                  |
// # |  10 | todo!("xabar")           | O'z xabar bilan todo                                 | todo с собственным сообщением                        |
// # |  11 | unimplemented!()         | Bu funksiya qo'llab quvvatlanmaydi                   | Эта функция не поддерживается                        |
// # |  12 | unreachable!()           | Bu yerga kelish mumkin emas                          | Сюда не должны доходить                              |
// #================================================================================================================================================#