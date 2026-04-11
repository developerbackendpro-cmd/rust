// #================================================================================================================================================#
// #                                                                METHODS                                                                         #
// #                                 METHODS — STRUCT YOKI ENUM GA BOG'LIQ FUNKSIYALAR. IMPL BLOKI ICHIDA YOZILADI.                                 #
// #                                 METHODS — ФУНКЦИИ, ПРИВЯЗАННЫЕ К СТРУКТУРЕ ИЛИ ENUM. ПИШУТСЯ ВНУТРИ IMPL БЛОКА.                                #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

struct Turtburchak {
    eni: f64,
    boyi: f64,
}

impl Turtburchak {

    // ::new() — associated function (constructor)
    // ассоциированная функция (конструктор)
    fn new(eni: f64, boyi: f64) -> Turtburchak {
        Turtburchak { eni, boyi }
    }

    // &self — o'qish (immutable borrow)
    // чтение (иммутабельное заимствование)
    fn yuza(&self) -> f64 {
        self.eni * self.boyi
    }

    // &self — perimetr hisoblash
    // вычисление периметра
    fn perimetr(&self) -> f64 {
        2.0 * (self.eni + self.boyi)
    }

    // &self — kvadratmi?
    // является ли квадратом?
    fn kvadratmi(&self) -> bool {
        self.eni == self.boyi
    }

    // &mut self — o'zgartirish (mutable borrow)
    // изменение (мутабельное заимствование)
    fn ikki_baravar(&mut self) {
        self.eni *= 2.0;
        self.boyi *= 2.0;
    }

    // &mut self — eni o'zgartirish
    // изменение ширины
    fn enini_ozgartir(&mut self, yangi: f64) {
        self.eni = yangi;
    }

    // self — egalikni olish (consuming)
    // потребление (передача владения)
    fn matn_qilib_yut(self) -> String {
        format!("{}x{}", self.eni, self.boyi)
    }

    // method chaining — &mut self qaytarish
    // цепочка вызовов — возврат &mut self
    fn enini_set(&mut self, eni: f64) -> &mut Self {
        self.eni = eni;
        self
    }

    fn boyini_set(&mut self, boyi: f64) -> &mut Self {
        self.boyi = boyi;
        self
    }

    // associated function — self yo'q (static method)
    // ассоциированная функция — без self (статический метод)
    fn kvadrat(tomon: f64) -> Turtburchak {
        Turtburchak { eni: tomon, boyi: tomon }
    }

    // associated const — impl ichida constant
    // константа внутри impl
    const MIN_OLCHAM: f64 = 0.1;
}

// ─────────────────────────────────────────────────────────────────────────────
// bir nechta impl blok — ruxsat etilgan
// несколько impl блоков — разрешено
// ─────────────────────────────────────────────────────────────────────────────

impl Turtburchak {

    // nisbat — eni / boyi
    // соотношение — ширина / высота
    fn nisbat(&self) -> f64 {
        self.eni / self.boyi
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// enum uchun impl
// impl для enum
// ─────────────────────────────────────────────────────────────────────────────

enum Yonalish { Shimol, Janub, Sharq, Garb }

impl Yonalish {

    // &self — teskari yonalish
    // противоположное направление
    fn teskari(&self) -> Yonalish {
        match self {
            Yonalish::Shimol => Yonalish::Janub,
            Yonalish::Janub  => Yonalish::Shimol,
            Yonalish::Sharq  => Yonalish::Garb,
            Yonalish::Garb   => Yonalish::Sharq,
        }
    }

    // &self — shimolmi?
    // это север?
    fn shimolmi(&self) -> bool {
        matches!(self, Yonalish::Shimol)
    }
}

fn main() {

    // ::new() — constructor chaqirish
    // вызов конструктора
    let mut t = Turtburchak::new(4.0, 3.0);
    println!("{}", t.yuza());
    println!("{}", t.perimetr());
    // 12
    // 14

    // &self — o'qish
    // чтение
    println!("{}", t.kvadratmi());
    // false

    // &mut self — o'zgartirish
    // изменение
    t.ikki_baravar();
    println!("{} {}", t.eni, t.boyi);
    // 8 6

    // method chaining
    t.enini_set(10.0).boyini_set(5.0);
    println!("{} {}", t.eni, t.boyi);
    // 10 5

    // self — consuming method
    // потребляющий метод
    let t2 = Turtburchak::new(3.0, 4.0);
    let s = t2.matn_qilib_yut();
    println!("{}", s);
    // 3x4

    // associated function — :: bilan chaqirish
    // вызов через ::
    let kv = Turtburchak::kvadrat(5.0);
    println!("{}", kv.yuza());
    // 25

    // associated const
    println!("{}", Turtburchak::MIN_OLCHAM);
    // 0.1

    // ikkinchi impl blok
    // второй impl блок
    let t3 = Turtburchak::new(6.0, 3.0);
    println!("{}", t3.nisbat());
    // 2

    // enum impl
    let y = Yonalish::Shimol;
    println!("{}", y.shimolmi());
    // true
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |   1 | fn metod(&self)          | O'qish — structni o'zgartirmaydi                     | Чтение — не изменяет структуру                       |
// # |   2 | fn metod(&mut self)      | O'zgartirish — structni o'zgartiradi                 | Изменение — изменяет структуру                       |
// # |   3 | fn metod(self)           | Consuming — egalikni oladi, keyin ishlatib bo'lmaydi | Потребление — забирает владение, нельзя использовать |
// # |   4 | fn new(...) -> Self      | Associated function (constructor), self yo'q         | Ассоциированная функция (конструктор), без self      |
// # |   5 | fn f() -> Self           | Static method — :: bilan chaqiriladi                 | Статический метод — вызывается через ::              |
// # |   6 | &mut self qaytarish      | Method chaining imkonini beradi                      | Позволяет цепочку вызовов                            |
// # |   7 | bir nechta impl blok     | Bir struct uchun ko'p impl yozish mumkin             | Для одной структуры можно несколько impl блоков      |
// # |   8 | enum uchun impl          | Enum variantlari uchun ham method yoziladi           | Методы можно писать и для вариантов enum             |
// # |   9 | const NOMI: T = ...      | impl ichida associated constant                      | Ассоциированная константа внутри impl                |
// # |  10 | Self kalit so'zi         | Joriy tur nomi o'rnida ishlatiladi                   | Используется вместо имени текущего типа              |
// #================================================================================================================================================#