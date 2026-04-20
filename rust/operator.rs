// #================================================================================================================================================#
// #                                                                ? OPERATORI                                                                     #
// #                            ? — XATONI YUQORIGA UZATISH. MATCH YOZMASDAN QISQA VA CHIROYLI KOD. RESULT VA OPTION BILAN ISHLAYDI.                #
// #                            ? — РАСПРОСТРАНЕНИЕ ОШИБКИ ВВЕРХ. КОРОТКИЙ КОД БЕЗ MATCH. РАБОТАЕТ С RESULT И OPTION.                               #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::num::ParseIntError;
use std::fmt;

// ? operatori aslida nima qiladi:
// Что на самом деле делает оператор ?:
//
//   expr?
//
//   ≡  match expr {
//          Ok(val)  => val,
//          Err(e)   => return Err(From::from(e)),
//      }
//
//   Option uchun:
//   Для Option:
//   ≡  match expr {
//          Some(val) => val,
//          None      => return None,
//      }
//
// Muhim: From::from(e) — xato turini aylantiradi
// Важно: From::from(e) — конвертирует тип ошибки
// Shuning uchun: impl From<E1> for E2 bo'lsa — ? ishlaydi
// Поэтому: если impl From<E1> for E2 — ? работает

#[derive(Debug)]
enum XatoTur {
    Parse(ParseIntError),
    Manfiy(i32),
    JudaKatta(i32),
    BoshQator,
}

impl std::error::Error for XatoTur {}

impl fmt::Display for XatoTur {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            XatoTur::Parse(e)       => write!(f, "Parse xato: {}", e),
            XatoTur::Manfiy(n)      => write!(f, "Manfiy son: {}", n),
            XatoTur::JudaKatta(n)   => write!(f, "Juda katta: {}", n),
            XatoTur::BoshQator      => write!(f, "Bo'sh qator"),
        }
    }
}

impl From<ParseIntError> for XatoTur {
    fn from(e: ParseIntError) -> Self {
        XatoTur::Parse(e)
    }
}

// ? BILAN — qisqa va chiroyli
// С ? — коротко и красиво
fn son_parse_qil(s: &str) -> Result<i32, XatoTur> {
    if s.is_empty() {
        return Err(XatoTur::BoshQator);
    }
    let n: i32 = s.parse::<i32>()?;
    // ↑ ParseIntError → XatoTur::Parse (From orqali avtomatik)
    // ↑ ParseIntError → XatoTur::Parse (автоматически через From)
    if n < 0 {
        return Err(XatoTur::Manfiy(n));
    }
    if n > 1000 {
        return Err(XatoTur::JudaKatta(n));
    }
    Ok(n)
}

// ? BILAN — zanjir
// С ? — цепочка
fn ikki_sonni_parse_qil(a: &str, b: &str) -> Result<i32, XatoTur> {
    let x: i32 = son_parse_qil(a)?;
    // ↑ XatoTur → XatoTur (bir xil tur — From<T> for T bor)
    let y: i32 = son_parse_qil(b)?;
    Ok(x + y)
}

// ? BILAN — ko'p amaliyot
// С ? — много операций
fn konfiguratsiya_yuklash(
    port_str: &str,
    timeout_str: &str,
    max_str: &str,
) -> Result<(i32, i32, i32), XatoTur> {
    let port: i32    = son_parse_qil(port_str)?;
    let timeout: i32 = son_parse_qil(timeout_str)?;
    let max: i32     = son_parse_qil(max_str)?;
    Ok((port, timeout, max))
}

// ? BILAN match — taqqoslash
// Сравнение ? с match
fn match_bilan(s: &str) -> Result<i32, XatoTur> {
    let n: i32 = match s.parse::<i32>() {
        Ok(val)  => val,
        Err(e)   => return Err(XatoTur::Parse(e)),
    };
    if n < 0 {
        return Err(XatoTur::Manfiy(n));
    }
    Ok(n)
}
// ↑ va ↓ bir xil natija!
// ↑ и ↓ одинаковый результат!
fn question_bilan(s: &str) -> Result<i32, XatoTur> {
    let n: i32 = s.parse::<i32>()?;
    if n < 0 {
        return Err(XatoTur::Manfiy(n));
    }
    Ok(n)
}

fn result_question_misollari() {

    // Muvaffaqiyatli
    // Успешно
    println!("{:?}", son_parse_qil("42"));
    println!("{:?}", son_parse_qil("0"));
    // Ok(42)
    // Ok(0)

    // Xatolar
    // Ошибки
    println!("{:?}", son_parse_qil("abc"));
    println!("{:?}", son_parse_qil("-5"));
    println!("{:?}", son_parse_qil("9999"));
    println!("{:?}", son_parse_qil(""));
    // Err(Parse(invalid digit found in string))
    // Err(Manfiy(-5))
    // Err(JudaKatta(9999))
    // Err(BoshQator)

    // Zanjir
    // Цепочка
    println!("{:?}", ikki_sonni_parse_qil("10", "20"));
    println!("{:?}", ikki_sonni_parse_qil("10", "abc"));
    // Ok(30)
    // Err(Parse(invalid digit found in string))

    // Ko'p amaliyot
    // Много операций
    println!("{:?}", konfiguratsiya_yuklash("8080", "30", "100"));
    println!("{:?}", konfiguratsiya_yuklash("8080", "abc", "100"));
    // Ok((8080, 30, 100))
    // Err(Parse(invalid digit found in string))
}

fn option_question_misollari() {

    // Option bilan ? — None bo'lsa funksiyadan chiqadi
    // ? с Option — при None выходит из функции
    fn birinchi_juft(v: &[i32]) -> Option<i32> {
        let birinchi: &i32 = v.first()?;
        // ↑ None bo'lsa → return None
        // ↑ если None → return None
        if birinchi % 2 == 0 {
            Some(*birinchi)
        } else {
            None
        }
    }

    println!("{:?}", birinchi_juft(&[2, 3, 4]));
    println!("{:?}", birinchi_juft(&[3, 4, 5]));
    println!("{:?}", birinchi_juft(&[]));
    // Some(2)
    // None
    // None

    // Option zanjiri
    // Цепочка Option
    fn ism_uzunligi(sozlar: &[&str], indeks: usize) -> Option<usize> {
        let soz: &&str = sozlar.get(indeks)?;
        // ↑ indeks chegaradan oshsa → return None
        // ↑ если индекс за пределами → return None
        Some(soz.len())
    }

    println!("{:?}", ism_uzunligi(&["salom", "dunyo"], 0));
    println!("{:?}", ism_uzunligi(&["salom", "dunyo"], 5));
    // Some(5)
    // None

    // HashMap dan qidirish zanjiri
    // Цепочка поиска в HashMap
    use std::collections::HashMap;
    fn shahar_mamlakat(
        foydalanuvchi: &HashMap<&str, &str>,
        shaharlar: &HashMap<&str, &str>,
        ism: &str,
    ) -> Option<String> {
        let shahar: &&str = foydalanuvchi.get(ism)?;
        // ↑ foydalanuvchi topilmasa → None
        let mamlakat: &&str = shaharlar.get(*shahar)?;
        // ↑ shahar topilmasa → None
        Some(format!("{} → {}", shahar, mamlakat))
    }

    let mut foydalanuvchilar: HashMap<&str, &str> = HashMap::new();
    foydalanuvchilar.insert("Dilshod", "Toshkent");
    foydalanuvchilar.insert("Ali", "Samarqand");

    let mut shaharlar: HashMap<&str, &str> = HashMap::new();
    shaharlar.insert("Toshkent", "O'zbekiston");
    shaharlar.insert("Samarqand", "O'zbekiston");

    println!("{:?}", shahar_mamlakat(&foydalanuvchilar, &shaharlar, "Dilshod"));
    println!("{:?}", shahar_mamlakat(&foydalanuvchilar, &shaharlar, "Vali"));
    // Some("Toshkent → O'zbekiston")
    // None
}

// From implement qilinmagan — Box<dyn Error> ishlatiladi
// From не реализован — используется Box<dyn Error>
fn turli_xatolar_boxed(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let n: i32 = s.parse::<i32>()?;
    // ParseIntError → Box<dyn Error>  (avtomatik)
    // ParseIntError → Box<dyn Error>  (автоматически)
    Ok(n * 2)
}

// anyhow bilan — har qanday xato
// с anyhow — любая ошибка
fn turli_xatolar_anyhow(s: &str) -> anyhow::Result<i32> {
    let n: i32 = s.parse::<i32>()?;
    // ParseIntError → anyhow::Error (avtomatik)
    // ParseIntError → anyhow::Error (автоматически)
    let m: i32 = son_parse_qil(s)?;
    // XatoTur → anyhow::Error (avtomatik)
    // XatoTur → anyhow::Error (автоматически)
    Ok(n + m)
}

fn turli_xatolar_misollari() {

    // Box<dyn Error>
    // Box<dyn Error>
    println!("{:?}", turli_xatolar_boxed("21"));
    println!("{}", turli_xatolar_boxed("abc").unwrap_err());
    // Ok(42)
    // invalid digit found in string

    // anyhow
    // anyhow
    println!("{:?}", turli_xatolar_anyhow("21"));
    println!("{}", turli_xatolar_anyhow("abc").unwrap_err());
    // Ok(42)
    // invalid digit found in string
}

fn iterator_question_misollari() {

    // ? iterator da ishlaydi — collect bilan
    // ? работает в итераторе — с collect
    let stringlar: Vec<&str> = vec!["1", "2", "3", "4", "5"];
    let natija: Result<Vec<i32>, _> = stringlar.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{:?}", natija);
    // Ok([1, 2, 3, 4, 5])

    // Bitta xato — butun Err
    // Одна ошибка — весь Err
    let aralash: Vec<&str> = vec!["1", "ikki", "3"];
    let xatoli: Result<Vec<i32>, _> = aralash.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{}", xatoli.is_err());
    // true

    // filter_map — xatolarni o'tkazib yuborish
    // filter_map — пропуск ошибок
    let aralash2: Vec<&str> = vec!["1", "ikki", "3", "to'rt", "5"];
    let faqat_sonlar: Vec<i32> = aralash2.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("{:?}", faqat_sonlar);
    // [1, 3, 5]

    // ? funksiya ichida — collect bilan
    // ? внутри функции — с collect
    fn barcha_parse(qatorlar: &[&str]) -> Result<Vec<i32>, ParseIntError> {
        qatorlar.iter()
            .map(|s| s.parse::<i32>())
            .collect()
    }

    println!("{:?}", barcha_parse(&["10", "20", "30"]));
    println!("{:?}", barcha_parse(&["10", "abc", "30"]));
    // Ok([10, 20, 30])
    // Err(ParseIntError { kind: InvalidDigit })
}

#[derive(Debug)]
struct ServerKonfig {
    host: String,
    port: u16,
    timeout: u32,
}

#[derive(Debug)]
enum KonfigXato {
    MajburiyYoq(&'static str),
    NotogriiPort(String),
    NotogriiTimeout(String),
}

impl fmt::Display for KonfigXato {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KonfigXato::MajburiyYoq(k)      => write!(f, "'{k}' kalit yo'q"),
            KonfigXato::NotogriiPort(v)      => write!(f, "Noto'g'ri port: {v}"),
            KonfigXato::NotogriiTimeout(v)   => write!(f, "Noto'g'ri timeout: {v}"),
        }
    }
}

fn konfig_yuklash(
    ma_lumotlar: &std::collections::HashMap<&str, &str>
) -> Result<ServerKonfig, KonfigXato> {

    let host: String = ma_lumotlar
        .get("host")
        .ok_or(KonfigXato::MajburiyYoq("host"))?
        // ↑ None → Err → return
        .to_string();

    let port: u16 = ma_lumotlar
        .get("port")
        .ok_or(KonfigXato::MajburiyYoq("port"))?
        .parse::<u16>()
        .map_err(|_| KonfigXato::NotogriiPort(
            ma_lumotlar["port"].to_string()
        ))?;

    let timeout: u32 = ma_lumotlar
        .get("timeout")
        .ok_or(KonfigXato::MajburiyYoq("timeout"))?
        .parse::<u32>()
        .map_err(|_| KonfigXato::NotogriiTimeout(
            ma_lumotlar["timeout"].to_string()
        ))?;

    Ok(ServerKonfig { host, port, timeout })
}

fn main() {

    println!("=== RESULT BILAN ? ===");
    result_question_misollari();

    println!("\n=== OPTION BILAN ? ===");
    option_question_misollari();

    println!("\n=== TURLI XATO TURLARI ===");
    turli_xatolar_misollari();

    println!("\n=== ITERATOR DA ? ===");
    iterator_question_misollari();

    println!("\n=== REAL HAYOT ===");
    use std::collections::HashMap;

    let mut to_g_ri: HashMap<&str, &str> = HashMap::new();
    to_g_ri.insert("host", "localhost");
    to_g_ri.insert("port", "8080");
    to_g_ri.insert("timeout", "30");
    println!("{:#?}", konfig_yuklash(&to_g_ri));
    // Ok(ServerKonfig { host: "localhost", port: 8080, timeout: 30 })

    let mut host_yoq: HashMap<&str, &str> = HashMap::new();
    host_yoq.insert("port", "8080");
    host_yoq.insert("timeout", "30");
    println!("{}", konfig_yuklash(&host_yoq).unwrap_err());
    // 'host' kalit yo'q

    let mut port_xato: HashMap<&str, &str> = HashMap::new();
    port_xato.insert("host", "localhost");
    port_xato.insert("port", "notogri");
    port_xato.insert("timeout", "30");
    println!("{}", konfig_yuklash(&port_xato).unwrap_err());
    // Noto'g'ri port: notogri
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                   | Описание (RU)                                              |
// #================================================================================================================================================#
// # |   1 | expr?                     | Ok(v) → v, Err(e) → return Err(From::from(e)) | Ok(v) → v, Err(e) → return Err(From::from(e))              |
// # |   2 | Option da ?               | Some(v) → v, None → return None               | Some(v) → v, None → return None                            |
// # |   3 | From::from(e)             | Xato turini avtomatik aylantirish             | Автоматическая конвертация типа ошибки                     |
// # |   4 | ? zanjiri                 | Bir nechta ? ketma-ket                        | Несколько ? подряд                                         |
// # |   5 | Box<dyn Error> + ?        | Har qanday xato bilan ishlash                 | Работа с любой ошибкой                                     |
// # |   6 | anyhow + ?                | Eng qulay variant                             | Наиболее удобный вариант                                   |
// # |   7 | .ok_or(err)?              | Option → Result → ?                           | Option → Result → ?                                        |
// # |   8 | .map_err(|e| ...)?        | Xato turini o'zgartirish + ?                  | Изменение типа ошибки + ?                                  |
// # |   9 | collect::<Result<Vec,E>>  | Iterator + ? effekti                          | Итератор + эффект ?                                        |
// # |  10 | impl From<E1> for E2      | ? ishlashi uchun shart                        | Условие для работы ?                                       |
// #================================================================================================================================================#