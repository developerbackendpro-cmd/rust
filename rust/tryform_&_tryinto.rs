// #================================================================================================================================================#
// #                                                                TRYFROM  |  TRYINTO                                                             #
// #                            TRYFROM — XATO QAYTARISHI MUMKIN BO'LGAN KONVERSIYA. FROM DAN FARQI: MUVAFFAQIYATSIZ BO'LISHI MUMKIN.               #
// #                            TRYFROM — КОНВЕРТАЦИЯ КОТОРАЯ МОЖЕТ ВЕРНУТЬ ОШИБКУ. ОТЛИЧИЕ ОТ FROM: МОЖЕТ ЗАВЕРШИТЬСЯ НЕУДАЧЕЙ.                    #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::convert::TryFrom;
use std::convert::TryInto;

// From  — har doim muvaffaqiyatli, T qaytaradi
// From  — всегда успешна, возвращает T
// TryFrom — muvaffaqiyatsiz bo'lishi mumkin, Result<T, E> qaytaradi
// TryFrom — может завершиться неудачей, возвращает Result<T, E>

// oddiy TryFrom misoli
// простой пример TryFrom
#[derive(Debug, PartialEq)]
struct JuftSon(i32);

#[derive(Debug)]
struct ToqSonXato {
    qiymat: i32,
}

impl fmt::Display for ToqSonXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} toq son, juft son kerak", self.qiymat)
    }
}

impl TryFrom<i32> for JuftSon {
    type Error = ToqSonXato;

    fn try_from(qiymat: i32) -> Result<Self, Self::Error> {
        if qiymat % 2 == 0 {
            Ok(JuftSon(qiymat))
        } else {
            Err(ToqSonXato { qiymat })
        }
    }
}

// katta → kichik: xavfli — TryFrom ishlatiladi
// большой → маленький: опасно — используется TryFrom
fn integer_tryfrom_misollari() {
    // i32 → u8 (0..=255 oralig'ida bo'lsa Ok)
    // i32 → u8 (Ok если в диапазоне 0..=255)
    let kichik: i32 = 100;
    let katta: i32 = 1000;
    let manfiy: i32 = -1;

    let kichik_natija: Result<u8, _> = u8::try_from(kichik);
    let katta_natija: Result<u8, _> = u8::try_from(katta);
    let manfiy_natija: Result<u8, _> = u8::try_from(manfiy);

    println!("{:?}", kichik_natija);
    println!("{:?}", katta_natija);
    println!("{:?}", manfiy_natija);
    // Ok(100)
    // Err(TryFromIntError(()))
    // Err(TryFromIntError(()))

    // i64 → i32
    // i64 → i32
    let i64_kichik: i64 = 2_147_483_647;
    let i64_katta: i64 = 2_147_483_648;

    let i32_ok: Result<i32, _> = i32::try_from(i64_kichik);
    let i32_err: Result<i32, _> = i32::try_from(i64_katta);

    println!("{:?}", i32_ok);
    println!("{:?}", i32_err);
    // Ok(2147483647)
    // Err(TryFromIntError(()))

    // usize → u32
    // usize → u32
    let usize_son: usize = 42;
    let u32_natija: Result<u32, _> = u32::try_from(usize_son);
    println!("{:?}", u32_natija);
    // Ok(42)

    // u32 → i32 (katta qiymatlar manfiy bo'lishi mumkin)
    // u32 → i32 (большие значения могут стать отрицательными)
    let u32_kichik: u32 = 100;
    let u32_katta: u32 = 3_000_000_000;

    let i32_ok2: Result<i32, _> = i32::try_from(u32_kichik);
    let i32_err2: Result<i32, _> = i32::try_from(u32_katta);

    println!("{:?}", i32_ok2);
    println!("{:?}", i32_err2);
    // Ok(100)
    // Err(TryFromIntError(()))
}

// custom xato — std::error::Error implement qilgan
// пользовательская ошибка — реализует std::error::Error
#[derive(Debug)]
enum YoshXato {
    ManfiyYosh(i32),
    JudaKattaYosh(i32),
}

impl fmt::Display for YoshXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            YoshXato::ManfiyYosh(y)    => write!(f, "Yosh manfiy bo'lishi mumkin emas: {}", y),
            YoshXato::JudaKattaYosh(y) => write!(f, "Yosh juda katta: {}", y),
        }
    }
}

impl std::error::Error for YoshXato {}

#[derive(Debug)]
struct Yosh(u32);

impl TryFrom<i32> for Yosh {
    type Error = YoshXato;

    fn try_from(qiymat: i32) -> Result<Self, Self::Error> {
        if qiymat < 0 {
            Err(YoshXato::ManfiyYosh(qiymat))
        } else if qiymat > 150 {
            Err(YoshXato::JudaKattaYosh(qiymat))
        } else {
            Ok(Yosh(qiymat as u32))
        }
    }
}

// Email validatsiya
// Валидация Email
#[derive(Debug)]
struct Email(String);

#[derive(Debug)]
enum EmailXato {
    BoshqaQavsBelgisi,
    DomainYoq,
    BoshqaUzunlik,
}

impl fmt::Display for EmailXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EmailXato::BoshqaQavsBelgisi => write!(f, "@ belgisi yo'q"),
            EmailXato::DomainYoq         => write!(f, "Domain qismi yo'q"),
            EmailXato::BoshqaUzunlik     => write!(f, "Email juda qisqa yoki uzun"),
        }
    }
}

impl TryFrom<&str> for Email {
    type Error = EmailXato;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.len() < 5 || s.len() > 254 {
            return Err(EmailXato::BoshqaUzunlik);
        }
        if !s.contains('@') {
            return Err(EmailXato::BoshqaQavsBelgisi);
        }
        let qismlar: Vec<&str> = s.split('@').collect();
        if qismlar.len() != 2 || qismlar[1].is_empty() {
            return Err(EmailXato::DomainYoq);
        }
        Ok(Email(s.to_string()))
    }
}

// Port raqami validatsiya
// Валидация номера порта
#[derive(Debug, Clone, Copy)]
struct Port(u16);

#[derive(Debug)]
enum PortXato {
    TizimPorti(u16),
    NolPort,
}

impl fmt::Display for PortXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PortXato::TizimPorti(p) => write!(f, "{} tizim porti, 1024 dan katta bo'lishi kerak", p),
            PortXato::NolPort       => write!(f, "Port 0 bo'lishi mumkin emas"),
        }
    }
}

impl TryFrom<u16> for Port {
    type Error = PortXato;

    fn try_from(qiymat: u16) -> Result<Self, Self::Error> {
        match qiymat {
            0          => Err(PortXato::NolPort),
            1..=1023   => Err(PortXato::TizimPorti(qiymat)),
            _          => Ok(Port(qiymat)),
        }
    }
}

impl TryFrom<i32> for Port {
    type Error = PortXato;

    fn try_from(qiymat: i32) -> Result<Self, Self::Error> {
        let u16_qiymat = u16::try_from(qiymat)
            .map_err(|_| PortXato::NolPort)?;
        Port::try_from(u16_qiymat)
    }
}

// &[T] → [T; N] — eng ko'p ishlatiladigan TryFrom
// &[T] → [T; N] — наиболее часто используемый TryFrom
fn slice_dan_array_misollari() {
    // Vec → [T; 3]
    // Vec → [T; 3]
    let vektor: Vec<i32> = vec![1, 2, 3];
    let array_natija: Result<[i32; 3], _> = vektor.as_slice().try_into();
    println!("{:?}", array_natija);
    // Ok([1, 2, 3])

    // uzunlik mos kelmasa — Err
    // если длина не совпадает — Err
    let uzun_vektor: Vec<i32> = vec![1, 2, 3, 4, 5];
    let qisqa_array: Result<[i32; 3], _> = uzun_vektor.as_slice().try_into();
    println!("{:?}", qisqa_array);
    // Err(TryFromSliceError(()))

    // &[u8] → [u8; 4] — IP manzil uchun
    // &[u8] → [u8; 4] — для IP адреса
    let ip_baytlari: &[u8] = &[192, 168, 1, 1];
    let ip_array: Result<[u8; 4], _> = ip_baytlari.try_into();
    println!("{:?}", ip_array);
    // Ok([192, 168, 1, 1])

    // bytes → i32
    // байты → i32
    let baytlar: &[u8] = &[0, 0, 0, 42];
    let son_baytlari: Result<[u8; 4], _> = baytlar.try_into();
    if let Ok(arr) = son_baytlari {
        let son = i32::from_be_bytes(arr);
        println!("Son: {}", son);
    }
    // Son: 42
}

// TryFrom implement qilsak TryInto bepul keladi
// реализуя TryFrom, TryInto достаётся бесплатно
fn tryinto_misollari() {
    // JuftSon::try_from(4)  ≡  4i32.try_into()
    let juft: Result<JuftSon, _> = 4i32.try_into();
    let toq: Result<JuftSon, _> = 5i32.try_into();
    println!("{:?}", juft);
    println!("{:?}", toq);
    // Ok(JuftSon(4))
    // Err(ToqSonXato { qiymat: 5 })

    // Yosh TryInto
    // TryInto для Yosh
    let yosh1: Result<Yosh, _> = 25i32.try_into();
    let yosh2: Result<Yosh, _> = (-5i32).try_into();
    let yosh3: Result<Yosh, _> = 200i32.try_into();
    println!("{:?}", yosh1);
    println!("{:?}", yosh2);
    println!("{:?}", yosh3);
    // Ok(Yosh(25))
    // Err(ManfiyYosh(-5))
    // Err(JudaKattaYosh(200))

    // Port TryInto
    // TryInto для Port
    let port1: Result<Port, _> = 8080u16.try_into();
    let port2: Result<Port, _> = 80u16.try_into();
    let port3: Result<Port, _> = 0u16.try_into();
    println!("{:?}", port1);
    println!("{:?}", port2);
    println!("{:?}", port3);
    // Ok(Port(8080))
    // Err(TizimPorti(80))
    // Err(NolPort)
}

// ? operatori bilan TryFrom ishlatish
// использование TryFrom с оператором ?
#[derive(Debug)]
struct ServerKonfiguratsiya {
    port: Port,
    email: Email,
    admin_yoshi: Yosh,
}

#[derive(Debug)]
enum KonfiguratsiyaXato {
    PortXato(PortXato),
    EmailXato(EmailXato),
    YoshXato(YoshXato),
}

impl fmt::Display for KonfiguratsiyaXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KonfiguratsiyaXato::PortXato(e)  => write!(f, "Port xatosi: {}", e),
            KonfiguratsiyaXato::EmailXato(e) => write!(f, "Email xatosi: {}", e),
            KonfiguratsiyaXato::YoshXato(e)  => write!(f, "Yosh xatosi: {}", e),
        }
    }
}

impl From<PortXato> for KonfiguratsiyaXato {
    fn from(e: PortXato) -> Self { KonfiguratsiyaXato::PortXato(e) }
}

impl From<EmailXato> for KonfiguratsiyaXato {
    fn from(e: EmailXato) -> Self { KonfiguratsiyaXato::EmailXato(e) }
}

impl From<YoshXato> for KonfiguratsiyaXato {
    fn from(e: YoshXato) -> Self { KonfiguratsiyaXato::YoshXato(e) }
}

fn server_konfiguratsiya_yarat(
    port: u16,
    email: &str,
    yosh: i32,
) -> Result<ServerKonfiguratsiya, KonfiguratsiyaXato> {
    // ? operatori — TryFrom + From xato konversiyasi
    // оператор ? — TryFrom + From конвертация ошибок
    let port: Port = Port::try_from(port)?;
    let email: Email = Email::try_from(email)?;
    let admin_yoshi: Yosh = Yosh::try_from(yosh)?;

    Ok(ServerKonfiguratsiya { port, email, admin_yoshi })
}

// T: TryFrom<U> — generic bound
// T: TryFrom<U> — generic ограничение
fn xavfsiz_aylantir<T, U, E>(qiymat: U) -> Result<T, E>
where
    T: TryFrom<U, Error = E>,
{
    T::try_from(qiymat)
}

// TryInto generic bound
// generic ограничение TryInto
fn tekshirib_saqlash<T>(qiymat: T) -> Option<u8>
where
    u8: TryFrom<T>,
{
    u8::try_from(qiymat).ok()
}

// bir nechta TryFrom zanjiri
// цепочка нескольких TryFrom
#[derive(Debug)]
struct Foiz(u8);

#[derive(Debug)]
struct FoizXato(String);

impl fmt::Display for FoizXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<f64> for Foiz {
    type Error = FoizXato;

    fn try_from(qiymat: f64) -> Result<Self, Self::Error> {
        if qiymat < 0.0 || qiymat > 100.0 {
            Err(FoizXato(format!("{} foiz 0-100 oralig'ida emas", qiymat)))
        } else {
            Ok(Foiz(qiymat as u8))
        }
    }
}

impl TryFrom<i32> for Foiz {
    type Error = FoizXato;

    fn try_from(qiymat: i32) -> Result<Self, Self::Error> {
        Foiz::try_from(qiymat as f64)
    }
}

impl TryFrom<&str> for Foiz {
    type Error = FoizXato;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.trim_end_matches('%');
        let qiymat: f64 = s.parse()
            .map_err(|_| FoizXato(format!("'{}' raqam emas", s)))?;
        Foiz::try_from(qiymat)
    }
}

fn result_bilan_ishlash() {
    // unwrap — ishonchli bo'lganda
    // unwrap — когда уверены
    let port: Port = Port::try_from(8080u16).unwrap();
    println!("{:?}", port);
    // Port(8080)

    // unwrap_or — xato bo'lsa default
    // unwrap_or — default при ошибке
    let port2: Port = Port::try_from(80u16)
        .unwrap_or(Port(8080));
    println!("{:?}", port2);
    // Port(8080)

    // unwrap_or_else — xato bo'lsa closure
    // unwrap_or_else — closure при ошибке
    let port3: Port = Port::try_from(0u16)
        .unwrap_or_else(|_| Port(3000));
    println!("{:?}", port3);
    // Port(3000)

    // map — Ok bo'lsa qiymatni o'zgartirish
    // map — изменение значения при Ok
    let port_raqami: Option<u16> = Port::try_from(8080u16)
        .map(|p| p.0)
        .ok();
    println!("{:?}", port_raqami);
    // Some(8080)

    // match — to'liq nazorat
    // match — полный контроль
    match Port::try_from(443u16) {
        Ok(port)  => println!("Port: {:?}", port),
        Err(xato) => println!("Xato: {}", xato),
    }
    // Xato: 443 tizim porti, 1024 dan katta bo'lishi kerak

    // if let — faqat Ok
    // if let — только Ok
    if let Ok(port) = Port::try_from(9000u16) {
        println!("Muvaffaqiyatli port: {:?}", port);
    }
    // Muvaffaqiyatli port: Port(9000)

    // ok() — Result → Option
    // ok() — Result → Option
    let yosh_option: Option<Yosh> = Yosh::try_from(25).ok();
    println!("{:?}", yosh_option);
    // Some(Yosh(25))
}

fn main() {

    // JuftSon — TryFrom<i32>
    // JuftSon — TryFrom<i32>
    let juft1: Result<JuftSon, _> = JuftSon::try_from(4);
    let juft2: Result<JuftSon, _> = JuftSon::try_from(5);
    println!("{:?}", juft1);
    println!("{:?}", juft2);
    // Ok(JuftSon(4))
    // Err(ToqSonXato { qiymat: 5 })

    integer_tryfrom_misollari();

    // Yosh validatsiya
    // Валидация Yosh
    let yosh1: Result<Yosh, _> = Yosh::try_from(25);
    let yosh2: Result<Yosh, _> = Yosh::try_from(-5);
    let yosh3: Result<Yosh, _> = Yosh::try_from(200);
    println!("{:?}", yosh1);
    println!("Xato: {}", yosh2.unwrap_err());
    println!("Xato: {}", yosh3.unwrap_err());
    // Ok(Yosh(25))
    // Xato: Yosh manfiy bo'lishi mumkin emas: -5
    // Xato: Yosh juda katta: 200

    // Email validatsiya
    // Валидация Email
    let email1: Result<Email, _> = Email::try_from("dilshod@example.com");
    let email2: Result<Email, _> = Email::try_from("noto_g_ri");
    let email3: Result<Email, _> = Email::try_from("ab");
    println!("{:?}", email1);
    println!("Xato: {}", email2.unwrap_err());
    println!("Xato: {}", email3.unwrap_err());
    // Ok(Email("dilshod@example.com"))
    // Xato: @ belgisi yo'q
    // Xato: Email juda qisqa yoki uzun

    // Port validatsiya
    // Валидация Port
    let port1: Result<Port, _> = Port::try_from(8080u16);
    let port2: Result<Port, _> = Port::try_from(80u16);
    let port3: Result<Port, _> = Port::try_from(0u16);
    println!("{:?}", port1);
    println!("Xato: {}", port2.unwrap_err());
    println!("Xato: {}", port3.unwrap_err());
    // Ok(Port(8080))
    // Xato: 80 tizim porti, 1024 dan katta bo'lishi kerak
    // Xato: Port 0 bo'lishi mumkin emas

    slice_dan_array_misollari();

    tryinto_misollari();

    // muvaffaqiyatli konfiguratsiya
    // успешная конфигурация
    let muvaffaqiyatli = server_konfiguratsiya_yarat(
        8080,
        "admin@example.com",
        30,
    );
    println!("{:#?}", muvaffaqiyatli);
    // Ok(ServerKonfiguratsiya { port: Port(8080), email: Email("admin@..."), admin_yoshi: Yosh(30) })

    // xatoli konfiguratsiya — port
    // ошибочная конфигурация — порт
    let xatoli_port = server_konfiguratsiya_yarat(80, "admin@example.com", 30);
    println!("{:?}", xatoli_port);
    // Err(PortXato(TizimPorti(80)))

    // xatoli konfiguratsiya — email
    // ошибочная конфигурация — email
    let xatoli_email = server_konfiguratsiya_yarat(8080, "noto_g_ri", 30);
    println!("{:?}", xatoli_email);
    // Err(EmailXato(BoshqaQavsBelgisi))

    // xavfsiz_aylantir — generic TryFrom
    // xavfsiz_aylantir — generic TryFrom
    let u8_natija: Result<u8, _> = xavfsiz_aylantir::<u8, i32, _>(42);
    let u8_xato: Result<u8, _> = xavfsiz_aylantir::<u8, i32, _>(1000);
    println!("{:?}", u8_natija);
    println!("{:?}", u8_xato);
    // Ok(42)
    // Err(TryFromIntError(()))

    // tekshirib_saqlash — Option qaytaradi
    // tekshirib_saqlash — возвращает Option
    let opt1: Option<u8> = tekshirib_saqlash(42i32);
    let opt2: Option<u8> = tekshirib_saqlash(1000i32);
    println!("{:?}", opt1);
    println!("{:?}", opt2);
    // Some(42)
    // None

    // Foiz — f64, i32, &str dan
    // Foiz — из f64, i32, &str
    let f1: Result<Foiz, _> = Foiz::try_from(75.5f64);
    let f2: Result<Foiz, _> = Foiz::try_from(150.0f64);
    let f3: Result<Foiz, _> = Foiz::try_from(50i32);
    let f4: Result<Foiz, _> = Foiz::try_from("85%");
    let f5: Result<Foiz, _> = Foiz::try_from("abc");

    println!("{:?}", f1);
    println!("Xato: {}", f2.unwrap_err());
    println!("{:?}", f3);
    println!("{:?}", f4);
    println!("Xato: {}", f5.unwrap_err());
    // Ok(Foiz(75))
    // Xato: 150 foiz 0-100 oralig'ida emas
    // Ok(Foiz(50))
    // Ok(Foiz(85))
    // Xato: 'abc' raqam emas

    result_bilan_ishlash();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                       TRYFROM TRAIT                                                                                          |
// #================================================================================================================================================#
// # |   1 | impl TryFrom<U> for T    | U dan T ga muvaffaqiyatsiz konversiya                | Неуспешная конвертация из U в T                      |
// # |   2 | type Error = E;          | Xato turi (majburiy)                                 | Тип ошибки (обязательно)                             |
// # |   3 | T::try_from(u)           | TryFrom ni chaqirish → Result<T, E>                  | Вызов TryFrom → Result<T, E>                         |
// # |   4 | TryFrom<i32>, TryFrom<f64>| Bir nechta TryFrom implement qilish                 | Реализация нескольких TryFrom                        |
// #================================================================================================================================================#
// # |                                       TRYINTO TRAIT                                                                                          |
// #================================================================================================================================================#
// # |   5 | TryFrom → TryInto bepul  | TryFrom implement qilsak TryInto bepul keladi        | Реализуя TryFrom, TryInto достаётся бесплатно        |
// # |   6 | u.try_into()             | TryInto ni chaqirish → Result<T, E>                  | Вызов TryInto → Result<T, E>                         |
// # |   7 | let x: T = u.try_into()? | ? bilan TryInto                                      | TryInto с оператором ?                               |
// #================================================================================================================================================#
// # |                                    INTEGER KONVERSIYALAR                                                                                     |
// #================================================================================================================================================#
// # |   8 | u8::try_from(i32)        | i32 → u8 (0..=255 emas bo'lsa Err)                  | i32 → u8 (Err если не в 0..=255)                      |
// # |   9 | i32::try_from(i64)       | i64 → i32 (range xatosi bo'lsa Err)                 | i64 → i32 (Err при выходе за диапазон)                |
// # |  10 | i32::try_from(u32)       | u32 → i32 (katta qiymatlar uchun Err)               | u32 → i32 (Err для больших значений)                  |
// #================================================================================================================================================#
// # |                                    SLICE → ARRAY                                                                                             |
// #================================================================================================================================================#
// # |  11 | slice.try_into::<[T;N]>()| Slice → Array (uzunlik mos kelmasa Err)             | Slice → Array (Err если длина не совпадает)           |
// # |  12 | &[u8] → [u8; 4]          | IP manzil, kalit baytlari uchun qulay               | Удобно для IP адресов, ключевых байтов                |
// #================================================================================================================================================#
// # |                                    ? OPERATOR BILAN                                                                                          |
// #================================================================================================================================================#
// # |  13 | val.try_into()?          | Muvaffaqiyatsiz bo'lsa funksiyadan chiqish           | Выход из функции при неудаче                         |
// # |  14 | From + TryFrom + ?       | Xato zanjiri — turli xatolarni birlashtirish         | Цепочка ошибок — объединение разных ошибок           |
// #================================================================================================================================================#
// # |                                    GENERIC BILAN                                                                                             |
// #================================================================================================================================================#
// # |  15 | T: TryFrom<U, Error=E>   | Generic TryFrom bound                                | Generic ограничение TryFrom                          |
// # |  16 | u8: TryFrom<T>           | Generic TryInto pattern                              | Generic паттерн TryInto                              |
// #================================================================================================================================================#
// # |                                    RESULT BILAN ISHLASH                                                                                      |
// #================================================================================================================================================#
// # |  17 | .unwrap()                | Ishonchli bo'lganda (panic bo'lishi mumkin)          | Когда уверены (может паниковать)                     |
// # |  18 | .unwrap_or(default)      | Xato bo'lsa default qiymat                          | Значение по умолчанию при ошибке                      |
// # |  19 | .unwrap_or_else(||...)   | Xato bo'lsa closure                                 | Замыкание при ошибке                                  |
// # |  20 | .map(|x| ...)            | Ok qiymatini o'zgartirish                           | Изменение значения Ok                                 |
// # |  21 | .ok()                    | Result → Option                                      | Result → Option                                      |
// # |  22 | match / if let           | To'liq nazorat                                      | Полный контроль                                       |
// #================================================================================================================================================#
// # |                                    FROM VS TRYFROM                                                                                           |
// #================================================================================================================================================#
// # |  23 | From                     | Har doim muvaffaqiyatli, T qaytaradi                 | Всегда успешна, возвращает T                         |
// # |  24 | TryFrom                  | Muvaffaqiyatsiz bo'lishi mumkin, Result<T,E>         | Может завершиться неудачей, Result<T,E>              |
// # |  25 | Qachon TryFrom           | Oraliq tekshirish, validatsiya, yo'qolishi bor       | Проверка диапазона, валидация, потери данных         |
// #================================================================================================================================================#