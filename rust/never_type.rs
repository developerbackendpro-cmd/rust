// #================================================================================================================================================#
// #                                                           NEVER TYPE  !                                                                        #
// #                     NEVER TYPE — HECH QACHON QIYMAT QAYTARMAYDIGAN TUR. DIVERGING FUNCTION. BARCHA TURGA AYLANA OLADI.                         #
// #                     NEVER TYPE — ТИП НИКОГДА НЕ ВОЗВРАЩАЮЩИЙ ЗНАЧЕНИЯ. ДИВЕРГИРУЮЩАЯ ФУНКЦИЯ. МОЖЕТ СТАТЬ ЛЮБЫМ ТИПОМ.                         #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

// ! — never type:
//   - Hech qachon qiymat qaytarmaydigan turlar uchun
//   - Для типов никогда не возвращающих значение
//   - Barcha turning kichik turi (bottom type)
//   - Является подтипом любого типа (bottom type)
//
// Qachon ishlatiladi:
// Когда используется:
//   - panic!() → !
//   - loop { } (break yo'q) → !
//   - process::exit() → !
//   - continue, break — expression kontekstida → !
//   - return — expression kontekstida → !
//
// ! → T ga aylana oladi (barcha T uchun)
// ! → может стать T (для любого T)
// Shuning uchun match da ishlaydi:
// Поэтому работает в match:
//   let x: i32 = match option {
//       Some(v) => v,
//       None => panic!("xato"),  // ! → i32
//   };

// -> ! — funksiya hech qachon qaytmaydi
// -> ! — функция никогда не возвращает управление
fn chiqish_qil(kod: i32) -> ! {
    std::process::exit(kod);
}

fn panic_chaqir(xabar: &str) -> ! {
    panic!("{}", xabar);
}

fn cheksiz_loop() -> ! {
    loop {
        // Hech qachon tugamaydi
        // Никогда не заканчивается
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

// Xato chiqaruvchi funksiya — ! qaytaradi
// Функция выдающая ошибку — возвращает !
fn xato_chiqar(xabar: &str) -> ! {
    eprintln!("XATO: {}", xabar);
    std::process::exit(1);
}

fn diverging_misollari() {

    // panic! → ! → istalgan tur
    // panic! → ! → любой тип
    let x: i32 = if true {
        42
    } else {
        panic!("Bu hech qachon ishlamaydi")
        // panic! → ! → i32 ga aylanadi
        // panic! → ! → преобразуется в i32
    };
    println!("{}", x);
    // 42

    // match da ! ishlatish
    // использование ! в match
    let option: Option<i32> = Some(10);
    let qiymat: i32 = match option {
        Some(v) => v,
        None    => panic!("Qiymat yo'q!"),
        // None  → ! → i32  (type check o'tadi!)
        // None  → ! → i32  (проверка типов проходит!)
    };
    println!("{}", qiymat);
    // 10

    // loop — break yo'q bo'lsa ! qaytaradi
    // loop — без break возвращает !
    let mut n: i32 = 0;
    let _natija: i32 = loop {
        n += 1;
        if n == 5 {
            break n * 2;  // break expression → i32
        }
        // break bo'lguncha ! bo'lib turadi
        // до break остаётся !
    };
    println!("{}", _natija);
    // 10
}

fn match_control_flow_misollari() {

    // continue → ! → tur tekshiruvi o'tadi
    // continue → ! → проверка типов проходит
    let sonlar: Vec<i32> = vec![1, -2, 3, -4, 5];
    let musbatlar: Vec<i32> = sonlar.iter()
        .map(|&x| {
            if x > 0 { x } else { return 0; }
            // return → ! → i32
            // return → ! → i32
        })
        .filter(|&x| x > 0)
        .collect();
    println!("{:?}", musbatlar);
    // [1, 3, 5]

    // unwrap_or_else — panic bilan
    // unwrap_or_else — с panic
    let s: &str = "42";
    let son: i32 = s.parse::<i32>().unwrap_or_else(|_| {
        panic!("Parse xatosi")
        // panic! → ! → i32
    });
    println!("{}", son);
    // 42

    // Result bilan — ? operatori
    // С Result — оператор ?
    // ? operatori aslida: match result { Ok(v) => v, Err(e) => return Err(e) }
    // return → ! → T (istalgan tur)
    // оператор ? это: match result { Ok(v) => v, Err(e) => return Err(e) }
    // return → ! → T (любой тип)

    // unwrap() — panic qaytaradi (!)
    // unwrap() — возвращает panic (!)
    let v: Vec<i32> = vec![1, 2, 3];
    let birinchi: &i32 = v.first().unwrap();
    println!("{}", birinchi);
    // 1

    // expect() — xabar bilan panic (!)
    // expect() — panic с сообщением (!)
    let ikkinchi: &i32 = v.get(1).expect("Ikkinchi element yo'q");
    println!("{}", ikkinchi);
    // 2

    // todo!() → ! — hali implement qilinmagan
    // todo!() → ! — ещё не реализовано
    fn hali_yozilmagan() -> i32 {
        if false {
            todo!("Bu funksiya hali yozilmagan")
            // todo! → ! → i32
        }
        42
    }
    println!("{}", hali_yozilmagan());
    // 42

    // unimplemented!() → !
    // unimplemented!() → !
    fn turi_bilan(_x: i32) -> i32 {
        if false {
            unimplemented!()
            // unimplemented! → ! → i32
        }
        0
    }
    println!("{}", turi_bilan(5));
    // 0

    // unreachable!() → !
    // unreachable!() → !
    let n: i32 = 3;
    let _tavsif: &str = match n {
        1 => "bir",
        2 => "ikki",
        3 => "uch",
        _ => unreachable!("Bu yerga kelmasligi kerak"),
        // unreachable! → ! → &str
    };
    println!("{}", _tavsif);
    // uch
}

// ! — Result<T, !> — xato bo'lmasligi kafolati
// ! — Result<T, !> — гарантия отсутствия ошибки
fn xatosiz_parse(s: &str) -> Result<String, std::convert::Infallible> {
    Ok(s.to_uppercase())
}

// Infallible — ! ning stable versiyasi
// Infallible — стабильная версия !
// std::convert::Infallible — hech qachon sodir bo'lmaydigan xato
// std::convert::Infallible — ошибка которая никогда не происходит
fn infallible_misoli() {
    let natija: Result<String, std::convert::Infallible> = xatosiz_parse("salom");
    // .unwrap() xavfsiz — chunki Err hech qachon bo'lmaydi
    // .unwrap() безопасен — потому что Err никогда не бывает
    let qiymat: String = natija.unwrap();
    println!("{}", qiymat);
    // SALOM

    // From<Infallible> — istalgan xato turiga aylantirish
    // From<Infallible> — преобразование в любой тип ошибки
    use std::convert::Infallible;
    let r: Result<i32, Infallible> = Ok(42);
    let _: Result<i32, String> = r.map_err(|e| match e {});
    // match e {} — Infallible bo'sh enum, hech qachon match bo'lmaydi
    // match e {} — Infallible пустой enum, никогда не совпадает
}

// Server — cheksiz loop → !
// Сервер — бесконечный loop → !
fn server_ishlashi_kerak_edi() {
    // Haqiqiy server:
    // Настоящий сервер:
    // fn server_run() -> ! {
    //     loop {
    //         let request = accept_connection();
    //         handle_request(request);
    //     }
    // }
    println!("Server misoli (! qaytaradi)");
}

// CLI — xato bo'lsa chiqib ketish
// CLI — выход при ошибке
fn cli_xato_boshqaruvi(natija: Result<i32, &str>) -> i32 {
    match natija {
        Ok(v)    => v,
        Err(msg) => {
            eprintln!("Xato: {}", msg);
            // Real dasturda:
            // В реальной программе:
            // std::process::exit(1);  → !
            0
        }
    }
}

// Assertion — test uchun
// Assertion — для тестов
fn assert_misoli(shart: bool, xabar: &str) {
    if !shart {
        panic!("{}", xabar);
        // panic! → ! — funksiya bu yerda tugaydi
        // panic! → ! — функция завершается здесь
    }
}

// Builder pattern — ! bilan xavfsizlik
// Builder pattern — безопасность с !
struct ServerBuilder {
    port: Option<u16>,
    host: Option<String>,
}

impl ServerBuilder {
    fn new() -> Self {
        ServerBuilder { port: None, host: None }
    }

    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    fn host(mut self, host: &str) -> Self {
        self.host = Some(host.to_string());
        self
    }

    fn qur(self) -> (u16, String) {
        let port: u16 = self.port.unwrap_or_else(|| {
            panic!("Port ko'rsatilmagan!")
            // panic! → ! → u16
        });
        let host: String = self.host.unwrap_or_else(|| {
            panic!("Host ko'rsatilmagan!")
            // panic! → ! → String
        });
        (port, host)
    }
}

fn main() {

    println!("=== DIVERGING FUNKSIYALAR ===");
    diverging_misollari();

    println!("\n=== MATCH VA CONTROL FLOW ===");
    match_control_flow_misollari();

    println!("\n=== INFALLIBLE ===");
    infallible_misoli();

    println!("\n=== REAL HAYOT ===");

    // CLI xato boshqaruvi
    // Управление ошибками CLI
    let ok_natija: i32 = cli_xato_boshqaruvi(Ok(42));
    let err_natija: i32 = cli_xato_boshqaruvi(Err("fayl topilmadi"));
    println!("Ok: {}", ok_natija);
    println!("Err: {}", err_natija);
    // Ok: 42
    // Xato: fayl topilmadi
    // Err: 0

    // assert_misoli
    // assert_misoli
    assert_misoli(true, "Bu o'tishi kerak");
    println!("Assert o'tdi");
    // Assert o'tdi

    // ServerBuilder — panic! bilan xavfsizlik
    // ServerBuilder — безопасность с panic!
    let (port, host) = ServerBuilder::new()
        .port(8080)
        .host("localhost")
        .qur();
    println!("{}:{}", host, port);
    // localhost:8080

    // server_ishlashi_kerak_edi
    server_ishlashi_kerak_edi();

    // ! — barcha built-in makrolar
    // ! — все встроенные макросы
    println!("\n=== BUILT-IN ! MAKROLAR ===");

    // panic! — dasturni to'xtatadi
    // panic! — останавливает программу
    let _: i32 = if false { panic!("test") } else { 1 };
    println!("panic! → !");
    // panic! → !

    // todo! — implement qilinmagan
    // todo! — не реализовано
    let _: i32 = if false { todo!() } else { 2 };
    println!("todo! → !");
    // todo! → !

    // unimplemented! — implement qilinmagan
    // unimplemented! — не реализовано
    let _: i32 = if false { unimplemented!() } else { 3 };
    println!("unimplemented! → !");
    // unimplemented! → !

    // unreachable! — yetib bo'lmaydigan kod
    // unreachable! — недостижимый код
    let _: i32 = if false { unreachable!() } else { 4 };
    println!("unreachable! → !");
    // unreachable! → !

    // abort! — process::abort (signal)
    // abort! — process::abort (сигнал)
    // std::process::abort() → !  (ishlatilmadi — dasturni o'ldiradi)
    println!("abort → ! (ishlatilmadi)");
    // abort → ! (не использовали)
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                  | Описание (RU)                                               |
// #================================================================================================================================================#
// # |                                       NEVER TYPE ASOSLARI                                                                                     |
// #================================================================================================================================================#
// # |   1 | !                         | Hech qachon qiymat qaytarmaydigan tur         | Тип никогда не возвращающий значение                       |
// # |   2 | fn f() -> !               | Diverging funksiya                            | Дивергирующая функция                                      |
// # |   3 | ! → T                     | Barcha turga aylanishi mumkin                 | Может стать любым типом                                    |
// # |   4 | bottom type               | Barcha turning kichik turi                    | Подтип любого типа                                         |
// #================================================================================================================================================#
// # |                                       NEVER TYPE MANBALAR                                                                                     |
// #================================================================================================================================================#
// # |   5 | panic!(...)               | ! qaytaradi — dasturni to'xtatadi             | Возвращает ! — останавливает программу                     |
// # |   6 | todo!()                   | ! qaytaradi — implement qilinmagan            | Возвращает ! — не реализовано                              |
// # |   7 | unimplemented!()          | ! qaytaradi — implement qilinmagan            | Возвращает ! — не реализовано                              |
// # |   8 | unreachable!()            | ! qaytaradi — yetib bo'lmaydigan kod          | Возвращает ! — недостижимый код                            |
// # |   9 | loop { }  (break yo'q)    | ! qaytaradi — cheksiz loop                    | Возвращает ! — бесконечный цикл                            |
// # |  10 | process::exit(code)       | ! qaytaradi — dasturni tugatadi               | Возвращает ! — завершает программу                         |
// # |  11 | continue (expr kontekst)  | ! qaytaradi — iteratsiyani o'tkazadi          | Возвращает ! — пропускает итерацию                         |
// # |  12 | return (expr kontekst)    | ! qaytaradi — funksiyadan qaytadi             | Возвращает ! — выходит из функции                          |
// # |  13 | ?  operatori (Err holat)  | return Err → ! → T                            | return Err → ! → T                                         |
// #================================================================================================================================================#
// # |                                       INFALLIBLE                                                                                             |
// #================================================================================================================================================#
// # |  14 | std::convert::Infallible  | ! ning stable versiyasi                       | Стабильная версия !                                        |
// # |  15 | Result<T, Infallible>     | Xato hech qachon bo'lmasligi kafolati         | Гарантия отсутствия ошибки                                 |
// # |  16 | .unwrap() xavfsiz         | Infallible bilan unwrap panic qilmaydi        | unwrap не паникует с Infallible                            |
// #================================================================================================================================================#
// # |                                       QACHON ISHLATISH                                                                                        |
// #================================================================================================================================================#
// # |  17 | match arm                 | Bir arm ! bo'lsa — tur tekshiruvi o'tadi      | Если arm ! — проверка типов проходит                       |
// # |  18 | if-else                   | Bir tarm ! bo'lsa — ikkinchi tur ishlatiladi  | Если ветка ! — используется тип другой ветки               |
// # |  19 | unwrap_or_else + panic!   | Xavfsiz unwrap alternativasi                  | Безопасная альтернатива unwrap                             |
// # |  20 | Server loop               | Cheksiz server loop → !                       | Бесконечный серверный цикл → !                             |
// #================================================================================================================================================#