// #================================================================================================================================================#
// #                                                            STD::FMT  (CHUQUR)                                                                  #
// #                                    STD::FMT — FORMATLASH MODULI. BARCHA {} {} FORMAT TRAITLARI SHU YERDA.                                      #
// #                                    STD::FMT — МОДУЛЬ ФОРМАТИРОВАНИЯ. ВСЕ ТРЕЙТЫ ФОРМАТА {} НАХОДЯТСЯ ЗДЕСЬ.                                    #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

struct Son(i64);

impl fmt::Display for Son {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Son {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Son({})", self.0)
    }
}

impl fmt::Binary for Son {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:b}", self.0)
    }
}

impl fmt::Octal for Son {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:o}", self.0)
    }
}

impl fmt::LowerHex for Son {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl fmt::UpperHex for Son {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl fmt::LowerExp for Son {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:e}", self.0)
    }
}

impl fmt::UpperExp for Son {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:E}", self.0)
    }
}

struct Jadval {
    sarlavha: String,
    qiymat: f64,
}

impl fmt::Display for Jadval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // f.width() — berilgan kenglikni olish
        // получение заданной ширины
        let kenglik = f.width().unwrap_or(0);

        // f.precision() — berilgan aniqlikni olish
        // получение заданной точности
        let aniqlik = f.precision().unwrap_or(2);

        // f.align() — hizalashni olish
        // получение выравнивания
        let hizalash = match f.align() {
            Some(fmt::Alignment::Left)   => "chap",
            Some(fmt::Alignment::Right)  => "ong",
            Some(fmt::Alignment::Center) => "markaz",
            None                         => "yo'q",
        };

        write!(f, "[{}]: {:.prec$} (hizalash: {})",
            self.sarlavha,
            self.qiymat,
            hizalash,
            prec = aniqlik
        )
    }
}

struct Mashina {
    marka: String,
    yil: u32,
    narx: f64,
}

impl fmt::Debug for Mashina {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // debug_struct — struct uchun
        // для структуры
        f.debug_struct("Mashina")
            .field("marka", &self.marka)
            .field("yil", &self.yil)
            .field("narx", &format_args!("${:.2}", self.narx))
            .finish()
    }
}

struct Ranglar(Vec<String>);

impl fmt::Debug for Ranglar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // debug_list — list uchun
        // для списка
        f.debug_list()
            .entries(self.0.iter())
            .finish()
    }
}

struct Sozlamalar {
    til: String,
    daraja: u32,
}

impl fmt::Debug for Sozlamalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // debug_map — map uchun
        // для map
        f.debug_map()
            .entry(&"til", &self.til)
            .entry(&"daraja", &self.daraja)
            .finish()
    }
}

struct Koordinatalar(f64, f64, f64);

impl fmt::Debug for Koordinatalar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // debug_tuple — tuple uchun
        // для кортежа
        f.debug_tuple("Koordinatalar")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

struct Xabar {
    daraja: &'static str,
    matn: String,
}

impl fmt::Display for Xabar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // format_args! — xotira ajratmasdan formatlash
        // форматирование без выделения памяти
        write!(f, "[{}] {}", self.daraja, self.matn)
    }
}

struct HtmlBuilder {
    ichki: String,
}

impl HtmlBuilder {
    fn new() -> Self {
        HtmlBuilder { ichki: String::new() }
    }
}

impl fmt::Write for HtmlBuilder {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.ichki.push_str(s);
        Ok(())
    }
}

struct Wrapper<T: fmt::Display>(T);

impl<T: fmt::Display> fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<<{}>>", self.0)
    }
}

impl<T: fmt::Display + fmt::Debug> fmt::Debug for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Wrapper({:?})", self.0)
    }
}

fn main() {

    // barcha formatlar bitta struct da
    // все форматы в одной структуре
    let son = Son(255);
    println!("Display  : {}", son);
    println!("Debug    : {:?}", son);
    println!("Binary   : {:b}", son);
    println!("Octal    : {:o}", son);
    println!("LowerHex : {:x}", son);
    println!("UpperHex : {:X}", son);
    println!("LowerExp : {:e}", son);
    println!("UpperExp : {:E}", son);
    // Display  : 255
    // Debug    : Son(255)
    // Binary   : 11111111
    // Octal    : 377
    // LowerHex : ff
    // UpperHex : FF
    // LowerExp : 2.55e2
    // UpperExp : 2.55E2

    // width(), precision(), align() ishlash
    // работа с width(), precision(), align()
    let jadval = Jadval {
        sarlavha: String::from("narx"),
        qiymat: 3.14159,
    };
    println!("{}", jadval);
    println!("{:.4}", jadval);
    println!("{:<20.3}", jadval);
    // [narx]: 3.14 (hizalash: yo'q)
    // [narx]: 3.1416 (hizalash: yo'q)
    // [narx]: 3.142 (hizalash: chap)

    // debug_struct
    let mashina = Mashina {
        marka: String::from("Tesla"),
        yil: 2023,
        narx: 45000.0,
    };
    println!("{:#?}", mashina);
    // Mashina {
    //     marka: "Tesla",
    //     yil: 2023,
    //     narx: $45000.00,
    // }

    // debug_list
    let ranglar = Ranglar(vec![
        String::from("qizil"),
        String::from("yashil"),
        String::from("moviy"),
    ]);
    println!("{:?}", ranglar);
    // ["qizil", "yashil", "moviy"]

    // debug_map
    let sozlamalar = Sozlamalar {
        til: String::from("uz"),
        daraja: 5,
    };
    println!("{:?}", sozlamalar);
    // {"til": "uz", "daraja": 5}

    // debug_tuple
    let koordinatalar = Koordinatalar(1.0, 2.5, -3.7);
    println!("{:?}", koordinatalar);
    // Koordinatalar(1.0, 2.5, -3.7)

    // format_args! — xotira ajratmasdan
    // без выделения памяти
    let xabar = Xabar {
        daraja: "INFO",
        matn: String::from("dastur boshlandi"),
    };
    println!("{}", xabar);
    // [INFO] dastur boshlandi

    // format_args! — to'g'ridan-to'g'ri
    // напрямую
    let args = format_args!("son: {}, matn: {}", 42, "salom");
    let natija: String = args.to_string();
    println!("{}", natija);
    // son: 42, matn: salom

    // fmt::Write — custom bufferga yozish
    // запись в пользовательский буфер
    use std::fmt::Write as FmtWrite;
    let mut html = HtmlBuilder::new();
    write!(html, "<h1>{}</h1>", "Salom dunyo").unwrap();
    write!(html, "<p>{}</p>", "Bu paragraf").unwrap();
    println!("{}", html.ichki);
    // <h1>Salom dunyo</h1><p>Bu paragraf</p>

    // generic wrapper
    // обёртка с обобщением
    let son_wrapper: Wrapper<i32> = Wrapper(42);
    let matn_wrapper: Wrapper<&str> = Wrapper("salom");
    println!("{}", son_wrapper);
    println!("{}", matn_wrapper);
    println!("{:?}", son_wrapper);
    // <<42>>
    // <<salom>>
    // Wrapper(42)

    // nomlangan parametrlar
    // именованные параметры
    let ism: &str = "Dilshod";
    let yosh: u32 = 22;
    let gap: String = format!("{ism} {yosh} yoshda");
    println!("{}", gap);
    // Dilshod 22 yoshda

    // nomlangan + format belgi
    // именованный + спецификатор формата
    let pi: f64 = 3.14159;
    let chiqish: String = format!("pi = {pi:.3}");
    println!("{}", chiqish);
    // pi = 3.142

    // pozitsion parametrlar
    // позиционные параметры
    let birinchi: String = format!("{0} {1} {0}", "takror", "bir marta");
    println!("{}", birinchi);
    // takror bir marta takror

    // pozitsion + format
    // позиционный + формат
    let ikkinchi: String = format!("{0:.2} va {0:.4}", 3.14159);
    println!("{}", ikkinchi);
    // 3.14 va 3.1416

    // dinamik kenglik
    // динамическая ширина
    let kenglik: usize = 10;
    let dinamik_kenglik: String = format!("{:>width$}", "salom", width = kenglik);
    println!("|{}|", dinamik_kenglik);
    // |     salom|

    // dinamik aniqlik
    // динамическая точность
    let aniqlik: usize = 4;
    let dinamik_aniqlik: String = format!("{:.prec$}", 3.14159265, prec = aniqlik);
    println!("{}", dinamik_aniqlik);
    // 3.1416

    // dinamik kenglik va aniqlik
    // динамическая ширина и точность
    let kenglik2: usize = 12;
    let aniqlik2: usize = 3;
    let ikkalasi: String = format!("{:>width$.prec$}",
        3.14159,
        width = kenglik2,
        prec = aniqlik2
    );
    println!("|{}|", ikkalasi);
    // |       3.142|

    // {{ va }} — figurali qavslarni chiqarish
    // вывод фигурных скобок
    let ochiq_qavs: String = format!("{{");
    let yopiq_qavs: String = format!("}}");
    let json_like: String = format!("{{ \"ism\": \"{}\" }}", "Dilshod");
    println!("{}", ochiq_qavs);
    println!("{}", yopiq_qavs);
    println!("{}", json_like);
    // {
    // }
    // { "ism": "Dilshod" }
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                        FORMAT TRAITLARI                                                                                      |
// #================================================================================================================================================#
// # |   1 | impl fmt::Display        | {} — foydalanuvchi uchun                             | {} — для пользователя                                |
// # |   2 | impl fmt::Debug          | {:?} — dasturchi uchun                               | {:?} — для разработчика                              |
// # |   3 | impl fmt::Binary         | {:b} — ikkilik                                       | {:b} — двоичный                                      |
// # |   4 | impl fmt::Octal          | {:o} — sakkizlik                                     | {:o} — восьмеричный                                  |
// # |   5 | impl fmt::LowerHex       | {:x} — kichik hex                                    | {:x} — строчный hex                                  |
// # |   6 | impl fmt::UpperHex       | {:X} — katta hex                                     | {:X} — заглавный hex                                 |
// # |   7 | impl fmt::LowerExp       | {:e} — ilmiy kichik                                  | {:e} — научный строчный                              |
// # |   8 | impl fmt::UpperExp       | {:E} — ilmiy katta                                   | {:E} — научный заглавный                             |
// #================================================================================================================================================#
// # |                                        FORMATTER METODLARI                                                                                   |
// #================================================================================================================================================#
// # |   9 | f.width()                | Berilgan kenglikni olish                             | Получение заданной ширины                            |
// # |  10 | f.precision()            | Berilgan aniqlikni olish                             | Получение заданной точности                          |
// # |  11 | f.align()                | Hizalash turini olish                                | Получение типа выравнивания                          |
// # |  12 | f.fill()                 | To'ldirish belgisini olish                           | Получение символа заполнения                         |
// #================================================================================================================================================#
// # |                                       DEBUG BUILDER METODLARI                                                                                |
// #================================================================================================================================================#
// # |  13 | f.debug_struct(name)     | Struct debug builder                                 | Построитель debug для структуры                      |
// # |  14 | f.debug_list()           | List debug builder                                   | Построитель debug для списка                         |
// # |  15 | f.debug_map()            | Map debug builder                                    | Построитель debug для map                            |
// # |  16 | f.debug_tuple(name)      | Tuple debug builder                                  | Построитель debug для кортежа                        |
// #================================================================================================================================================#
// # |                                        FORMAT PARAMETRLARI                                                                                   |
// #================================================================================================================================================#
// # |  17 | {ism} {yosh}             | Nomlangan parametrlar                                | Именованные параметры                                |
// # |  18 | {0} {1} {0}              | Pozitsion parametrlar                                | Позиционные параметры                                |
// # |  19 | {:>width$}               | Dinamik kenglik                                      | Динамическая ширина                                  |
// # |  20 | {:.prec$}                | Dinamik aniqlik                                      | Динамическая точность                                |
// # |  21 | {{ va }}                 | Figurali qavslarni chiqarish                         | Вывод фигурных скобок                                |
// #================================================================================================================================================#
// # |                                       BOSHQA MUHIMLAR                                                                                        |
// #================================================================================================================================================#
// # |  22 | format_args!(...)        | Xotira ajratmasdan formatlash                        | Форматирование без выделения памяти                  |
// # |  23 | impl fmt::Write          | Custom bufferga yozish imkoniyati                    | Возможность записи в пользовательский буфер          |
// # |  24 | Wrapper<T: Display>      | Generic Display implementatsiya                      | Обобщённая реализация Display                        |
// #================================================================================================================================================#