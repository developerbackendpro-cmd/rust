// #================================================================================================================================================#
// #                                                                    DSL PATTERN                                                                 #
// #                    DSL — DOMAIN SPECIFIC LANGUAGE. MACRO_RULES!, BUILDER, OPERATOR OVERLOADING BILAN TIL YARATISH.                             #
// #                    DSL — ПРЕДМЕТНО-ОРИЕНТИРОВАННЫЙ ЯЗЫК. MACRO_RULES!, BUILDER, ПЕРЕГРУЗКА ОПЕРАТОРОВ ДЛЯ СОЗДАНИЯ ЯЗЫКА.                      #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::collections::HashMap;
use std::ops::{Add, Sub, Mul, BitAnd, BitOr, Not};

// DSL nima:
// Что такое DSL:
//
//   Domain Specific Language — muayyan soha uchun til
//   Предметно-ориентированный язык — для конкретной области
//
//   Rust da DSL usullari:
//   Способы DSL в Rust:
//   1. macro_rules! — token asosida sintaksis
//      macro_rules! — синтаксис на основе токенов
//   2. Builder pattern — fluent API
//      Builder pattern — fluent API
//   3. Operator overloading — matematik notatsiya
//      Перегрузка операторов — математическая нотация
//   4. Closure API — konfiguratsiya DSL
//      Closure API — конфигурационный DSL
//
//   Misollar:
//   Примеры:
//   - SQL query builder
//   - HTML generator
//   - Test framework
//   - Konfiguratsiya DSL
//   - Matematik ifodalar

#[derive(Debug, Clone)]
enum SorovShart {
    Teng(String, String),
    KattaRoq(String, String),
    KichikRoq(String, String),
    Orasida(String, String, String),
    Qidirish(String, String),
    Va(Box<SorovShart>, Box<SorovShart>),
    Yoki(Box<SorovShart>, Box<SorovShart>),
    Emas(Box<SorovShart>),
}

impl fmt::Display for SorovShart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Teng(ustun, val)         => write!(f, "{} = {}", ustun, val),
            Self::KattaRoq(ustun, val)     => write!(f, "{} > {}", ustun, val),
            Self::KichikRoq(ustun, val)    => write!(f, "{} < {}", ustun, val),
            Self::Orasida(ustun, a, b)     => write!(f, "{} BETWEEN {} AND {}", ustun, a, b),
            Self::Qidirish(ustun, namuna)  => write!(f, "{} LIKE '{}'", ustun, namuna),
            Self::Va(a, b)                 => write!(f, "({} AND {})", a, b),
            Self::Yoki(a, b)               => write!(f, "({} OR {})", a, b),
            Self::Emas(s)                  => write!(f, "NOT ({})", s),
        }
    }
}

impl BitAnd for SorovShart {
    type Output = Self;
    fn bitand(self, b: Self) -> Self {
        SorovShart::Va(Box::new(self), Box::new(b))
    }
}

impl BitOr for SorovShart {
    type Output = Self;
    fn bitor(self, b: Self) -> Self {
        SorovShart::Yoki(Box::new(self), Box::new(b))
    }
}

impl Not for SorovShart {
    type Output = Self;
    fn not(self) -> Self {
        SorovShart::Emas(Box::new(self))
    }
}

// Ustun — DSL qurilish bloki
// Столбец — строительный блок DSL
struct Ustun(String);

impl Ustun {
    fn teng(&self, val: &str) -> SorovShart {
        SorovShart::Teng(self.0.clone(), val.to_string())
    }
    fn katta(&self, val: &str) -> SorovShart {
        SorovShart::KattaRoq(self.0.clone(), val.to_string())
    }
    fn kichik(&self, val: &str) -> SorovShart {
        SorovShart::KichikRoq(self.0.clone(), val.to_string())
    }
    fn orasida(&self, a: &str, b: &str) -> SorovShart {
        SorovShart::Orasida(self.0.clone(), a.to_string(), b.to_string())
    }
    fn qidirish(&self, namuna: &str) -> SorovShart {
        SorovShart::Qidirish(self.0.clone(), namuna.to_string())
    }
}

fn col(nomi: &str) -> Ustun { Ustun(nomi.to_string()) }

#[derive(Debug)]
struct SelectSorov {
    ustunlar: Vec<String>,
    jadval: String,
    shart: Option<SorovShart>,
    tartib: Option<(String, bool)>,
    chegara: Option<usize>,
    siljish: Option<usize>,
}

impl SelectSorov {
    fn new(jadval: &str) -> Self {
        SelectSorov {
            ustunlar: vec!["*".to_string()],
            jadval: jadval.to_string(),
            shart: None,
            tartib: None,
            chegara: None,
            siljish: None,
        }
    }

    fn select(mut self, ustunlar: &[&str]) -> Self {
        self.ustunlar = ustunlar.iter().map(|s| s.to_string()).collect();
        self
    }

    fn qayerda(mut self, shart: SorovShart) -> Self {
        self.shart = Some(shart); self
    }

    fn tartiblash(mut self, ustun: &str, o_sish: bool) -> Self {
        self.tartib = Some((ustun.to_string(), o_sish)); self
    }

    fn chegara(mut self, n: usize) -> Self { self.chegara = Some(n); self }
    fn siljish(mut self, n: usize) -> Self { self.siljish = Some(n); self }

    fn qur(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}",
                              self.ustunlar.join(", "), self.jadval);
        if let Some(shart) = &self.shart {
            sql.push_str(&format!(" WHERE {}", shart));
        }
        if let Some((ustun, o_sish)) = &self.tartib {
            sql.push_str(&format!(" ORDER BY {} {}", ustun, if *o_sish { "ASC" } else { "DESC" }));
        }
        if let Some(n) = self.chegara {
            sql.push_str(&format!(" LIMIT {}", n));
        }
        if let Some(n) = self.siljish {
            sql.push_str(&format!(" OFFSET {}", n));
        }
        sql
    }
}

fn from(jadval: &str) -> SelectSorov { SelectSorov::new(jadval) }

fn sql_dsl_misoli() {

    println!("--- SQL DSL ---");

    let sorov1 = from("foydalanuvchilar")
        .select(&["id", "ism", "email"])
        .qayerda(col("yosh").katta("18") & col("faol").teng("true"))
        .tartiblash("ism", true)
        .chegara(10)
        .qur();
    println!("{}", sorov1);
    // SELECT id, ism, email FROM foydalanuvchilar WHERE (yosh > 18 AND faol = true) ORDER BY ism ASC LIMIT 10

    let sorov2 = from("mahsulotlar")
        .qayerda(
            col("narx").orasida("1000", "5000") |
                col("nomi").qidirish("%rust%")
        )
        .tartiblash("narx", false)
        .chegara(20)
        .siljish(40)
        .qur();
    println!("{}", sorov2);
    // SELECT * FROM mahsulotlar WHERE ((narx BETWEEN 1000 AND 5000) OR nomi LIKE '%rust%') ORDER BY narx DESC LIMIT 20 OFFSET 40

    let sorov3 = from("buyurtmalar")
        .select(&["id", "sana", "jami"])
        .qayerda(!col("bekor_qilingan").teng("true"))
        .qur();
    println!("{}", sorov3);
    // SELECT id, sana, jami FROM buyurtmalar WHERE NOT (bekor_qilingan = true)
}

#[derive(Debug)]
enum HtmlElement {
    Tag { nomi: String, atributlar: Vec<(String, String)>, bolalar: Vec<HtmlElement> },
    Matn(String),
    Izoh(String),
}

impl HtmlElement {
    fn chiqar(&self, chuqurlik: usize) -> String {
        let bosh = "  ".repeat(chuqurlik);
        match self {
            HtmlElement::Matn(s) => format!("{}{}", bosh, s),
            HtmlElement::Izoh(s) => format!("{}<!-- {} -->", bosh, s),
            HtmlElement::Tag { nomi, atributlar, bolalar } => {
                let atrs = if atributlar.is_empty() {
                    String::new()
                } else {
                    format!(" {}", atributlar.iter()
                        .map(|(k, v)| format!("{}=\"{}\"", k, v))
                        .collect::<Vec<_>>()
                        .join(" "))
                };
                if bolalar.is_empty() {
                    format!("{}<{}{} />", bosh, nomi, atrs)
                } else {
                    let ichki: Vec<String> = bolalar.iter()
                        .map(|b| b.chiqar(chuqurlik + 1))
                        .collect();
                    format!("{}<{}{}>\n{}\n{}</{}>",
                            bosh, nomi, atrs, ichki.join("\n"), bosh, nomi)
                }
            }
        }
    }
}

struct HtmlBuilder {
    nomi: String,
    atrutlar: Vec<(String, String)>,
    bolalar: Vec<HtmlElement>,
}

impl HtmlBuilder {
    fn new(nomi: &str) -> Self {
        HtmlBuilder { nomi: nomi.to_string(), atrutlar: Vec::new(), bolalar: Vec::new() }
    }

    fn atr(mut self, kalit: &str, qiymat: &str) -> Self {
        self.atrutlar.push((kalit.to_string(), qiymat.to_string())); self
    }

    fn class(self, cls: &str) -> Self { self.atr("class", cls) }
    fn id(self, id: &str) -> Self { self.atr("id", id) }
    fn href(self, url: &str) -> Self { self.atr("href", url) }
    fn src(self, url: &str) -> Self { self.atr("src", url) }
    fn style(self, s: &str) -> Self { self.atr("style", s) }

    fn matn(mut self, s: &str) -> Self {
        self.bolalar.push(HtmlElement::Matn(s.to_string())); self
    }

    fn izoh(mut self, s: &str) -> Self {
        self.bolalar.push(HtmlElement::Izoh(s.to_string())); self
    }

    fn bola(mut self, b: HtmlBuilder) -> Self {
        self.bolalar.push(b.qur()); self
    }

    fn qur(self) -> HtmlElement {
        HtmlElement::Tag {
            nomi: self.nomi,
            atributlar: self.atrutlar,
            bolalar: self.bolalar,
        }
    }
}

fn el(nomi: &str) -> HtmlBuilder { HtmlBuilder::new(nomi) }

macro_rules! html_doc {
    ($($element:expr),* $(,)?) => {
        {
            let mut bolalar: Vec<HtmlElement> = Vec::new();
            $( bolalar.push($element.qur()); )*
            bolalar
        }
    };
}

fn html_dsl_misoli() {

    println!("--- HTML DSL ---");

    let sahifa = el("html")
        .atr("lang", "uz")
        .bola(
            el("head").bola(
                el("title").matn("DSL Misol")
            ).bola(
                el("meta").atr("charset", "UTF-8")
            )
        )
        .bola(
            el("body").class("container").bola(
                el("h1").id("sarlavha").class("title").matn("Salom Dunyo!")
            ).bola(
                el("p").class("tavsif").matn("Rust DSL bilan HTML yaratish.")
            ).bola(
                el("ul").class("royxat")
                    .bola(el("li").matn("Birinchi element"))
                    .bola(el("li").matn("Ikkinchi element"))
                    .bola(el("li").matn("Uchinchi element"))
            ).bola(
                el("a").href("https://rust-lang.org").class("havola")
                    .matn("Rust sahifasi")
            )
        );

    println!("{}", sahifa.qur().chiqar(0));
}

struct TestMijoz {
    ism: String,
    sinovlar: Vec<(String, bool, Option<String>)>,
}

impl TestMijoz {
    fn new(ism: &str) -> Self {
        println!("📋 Test: {}", ism);
        TestMijoz { ism: ism.to_string(), sinovlar: Vec::new() }
    }

    fn tekshir(mut self, tavsif: &str, shart: bool) -> Self {
        let ok = shart;
        self.sinovlar.push((tavsif.to_string(), ok, None));
        if ok { println!("  ✅ {}", tavsif); }
        else  { println!("  ❌ {}", tavsif); }
        self
    }

    fn teng<T: PartialEq + fmt::Debug>(mut self, tavsif: &str, haqiqiy: T, kutilgan: T) -> Self {
        let ok = haqiqiy == kutilgan;
        let xabar = if !ok {
            Some(format!("kutilgan: {:?}, haqiqiy: {:?}", kutilgan, haqiqiy))
        } else { None };
        self.sinovlar.push((tavsif.to_string(), ok, xabar.clone()));
        if ok { println!("  ✅ {}", tavsif); }
        else  { println!("  ❌ {} — {}", tavsif, xabar.unwrap_or_default()); }
        self
    }

    fn hisobot(self) -> bool {
        let otdi = self.sinovlar.iter().filter(|(_, ok, _)| *ok).count();
        let jami = self.sinovlar.len();
        let hammasi_otdi = otdi == jami;
        println!("📊 Natija: {}/{} o'tdi {}", otdi, jami,
                 if hammasi_otdi { "✅" } else { "❌" });
        hammasi_otdi
    }
}

macro_rules! tekshir_teng {
    ($mijoz:expr, $tavsif:literal, $haqiqiy:expr, $kutilgan:expr) => {
        $mijoz = $mijoz.teng($tavsif, $haqiqiy, $kutilgan)
    };
}

fn test_dsl_misoli() {

    println!("--- Test Framework DSL ---");

    let o_tdi = TestMijoz::new("Matematik funksiyalar")
        .teng("2 + 2 = 4", 2 + 2, 4)
        .teng("3 * 4 = 12", 3 * 4, 12)
        .teng("10 / 2 = 5", 10 / 2, 5)
        .teng("faktorial(5) = 120", (1u64..=5).product::<u64>(), 120)
        .tekshir("100 > 50", 100 > 50)
        .tekshir("Vec bo'sh emas", !vec![1, 2, 3].is_empty())
        .hisobot();
    println!();

    let muvaffaqiyatsiz = TestMijoz::new("Xato tekshiruv (qasddan)")
        .teng("2 + 2 = 5?", 2 + 2, 5)    // xato!
        .teng("3 * 3 = 9", 3 * 3, 9)      // to'g'ri
        .hisobot();
    // 📋 Test: Matematik funksiyalar
    //   ✅ 2 + 2 = 4
    //   ✅ 3 * 4 = 12
    //   ...
    // 📊 Natija: 6/6 o'tdi ✅
}

#[derive(Debug, Clone)]
enum Ifoda {
    Son(f64),
    Qoshish(Box<Ifoda>, Box<Ifoda>),
    Ayirish(Box<Ifoda>, Box<Ifoda>),
    Kopaytirish(Box<Ifoda>, Box<Ifoda>),
    Bolish(Box<Ifoda>, Box<Ifoda>),
    Daraja(Box<Ifoda>, Box<Ifoda>),
    Sin(Box<Ifoda>),
    Cos(Box<Ifoda>),
    Sqrt(Box<Ifoda>),
    Abs(Box<Ifoda>),
}

impl Ifoda {
    fn hisoblash(&self) -> f64 {
        match self {
            Self::Son(n)         => *n,
            Self::Qoshish(a,b) => a.hisoblash() + b.hisoblash(),
            Self::Ayirish(a,b)  => a.hisoblash() - b.hisoblash(),
            Self::Kopaytirish(a,b) => a.hisoblash() * b.hisoblash(),
            Self::Bolish(a,b)   => a.hisoblash() / b.hisoblash(),
            Self::Daraja(a,b)   => a.hisoblash().powf(b.hisoblash()),
            Self::Sin(a)        => a.hisoblash().sin(),
            Self::Cos(a)        => a.hisoblash().cos(),
            Self::Sqrt(a)       => a.hisoblash().sqrt(),
            Self::Abs(a)        => a.hisoblash().abs(),
        }
    }

    fn formula(&self) -> String {
        match self {
            Self::Son(n)         => format!("{}", n),
            Self::Qoshish(a,b) => format!("({} + {})", a.formula(), b.formula()),
            Self::Ayirish(a,b)  => format!("({} - {})", a.formula(), b.formula()),
            Self::Kopaytirish(a,b) => format!("({} * {})", a.formula(), b.formula()),
            Self::Bolish(a,b)   => format!("({} / {})", a.formula(), b.formula()),
            Self::Daraja(a,b)   => format!("({}^{})", a.formula(), b.formula()),
            Self::Sin(a)        => format!("sin({})", a.formula()),
            Self::Cos(a)        => format!("cos({})", a.formula()),
            Self::Sqrt(a)       => format!("√({})", a.formula()),
            Self::Abs(a)        => format!("|{}|", a.formula()),
        }
    }
}

impl Add for Ifoda {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Ifoda::Qoshish(Box::new(self), Box::new(b))
    }
}

impl Sub for Ifoda {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Ifoda::Ayirish(Box::new(self), Box::new(b))
    }
}

impl Mul for Ifoda {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        Ifoda::Kopaytirish(Box::new(self), Box::new(b))
    }
}

fn n(val: f64) -> Ifoda { Ifoda::Son(val) }
fn sin(a: Ifoda) -> Ifoda { Ifoda::Sin(Box::new(a)) }
fn cos(a: Ifoda) -> Ifoda { Ifoda::Cos(Box::new(a)) }
fn sqrt(a: Ifoda) -> Ifoda { Ifoda::Sqrt(Box::new(a)) }
fn abs(a: Ifoda) -> Ifoda { Ifoda::Abs(Box::new(a)) }
fn pow(a: Ifoda, b: Ifoda) -> Ifoda { Ifoda::Daraja(Box::new(a), Box::new(b)) }

fn matematik_dsl_misoli() {

    println!("--- Matematik Ifoda DSL ---");

    // Pifagor teoremasi: c = sqrt(a² + b²)
    let a = n(3.0);
    let b = n(4.0);
    let c = sqrt(pow(a.clone(), n(2.0)) + pow(b.clone(), n(2.0)));
    println!("{} = {:.4}", c.formula(), c.hisoblash());
    // √((3^2) + (4^2)) = 5.0000

    // Trigonometrik ifoda: sin²(x) + cos²(x) = 1
    let x = n(0.5);
    let ifoda = pow(sin(x.clone()), n(2.0)) + pow(cos(x.clone()), n(2.0));
    println!("{} = {:.4}", ifoda.formula(), ifoda.hisoblash());
    // (sin(0.5)^2) + (cos(0.5)^2) = 1.0000

    // Murakkab ifoda
    let expr = (n(5.0) + n(3.0)) * (n(10.0) - n(4.0));
    println!("{} = {:.4}", expr.formula(), expr.hisoblash());
    // ((5 + 3) * (10 - 4)) = 48.0000

    // Abs va boshqalar
    let expr2 = abs(n(-7.0)) + sqrt(n(25.0));
    println!("{} = {:.4}", expr2.formula(), expr2.hisoblash());
    // (|-7| + √(25)) = 12.0000
}

#[derive(Debug, Default)]
struct AppKonfig {
    server: ServerSozlama,
    db: DbSozlama,
    log: LogSozlama,
}

#[derive(Debug)]
struct ServerSozlama {
    host: String,
    port: u16,
    workers: u32,
    tls: bool,
}

#[derive(Debug)]
struct DbSozlama {
    url: String,
    max_pool: u32,
    timeout: u64,
}

#[derive(Debug)]
struct LogSozlama {
    daraja: String,
    fayl: Option<String>,
    konsole: bool,
}

impl Default for ServerSozlama {
    fn default() -> Self {
        ServerSozlama { host: "0.0.0.0".into(), port: 8080, workers: 4, tls: false }
    }
}

impl Default for DbSozlama {
    fn default() -> Self {
        DbSozlama { url: "sqlite://app.db".into(), max_pool: 5, timeout: 30 }
    }
}

impl Default for LogSozlama {
    fn default() -> Self {
        LogSozlama { daraja: "info".into(), fayl: None, konsole: true }
    }
}

impl ServerSozlama {
    fn host(&mut self, h: &str) -> &mut Self { self.host = h.into(); self }
    fn port(&mut self, p: u16) -> &mut Self { self.port = p; self }
    fn workers(&mut self, n: u32) -> &mut Self { self.workers = n; self }
    fn tls(&mut self, t: bool) -> &mut Self { self.tls = t; self }
}

impl DbSozlama {
    fn url(&mut self, u: &str) -> &mut Self { self.url = u.into(); self }
    fn max_pool(&mut self, n: u32) -> &mut Self { self.max_pool = n; self }
    fn timeout(&mut self, t: u64) -> &mut Self { self.timeout = t; self }
}

impl LogSozlama {
    fn daraja(&mut self, d: &str) -> &mut Self { self.daraja = d.into(); self }
    fn fayl(&mut self, f: &str) -> &mut Self { self.fayl = Some(f.into()); self }
    fn konsole(&mut self, k: bool) -> &mut Self { self.konsole = k; self }
}

impl AppKonfig {
    fn new() -> Self { Self::default() }

    fn server<F: FnOnce(&mut ServerSozlama)>(&mut self, f: F) -> &mut Self {
        f(&mut self.server); self
    }

    fn db<F: FnOnce(&mut DbSozlama)>(&mut self, f: F) -> &mut Self {
        f(&mut self.db); self
    }

    fn log<F: FnOnce(&mut LogSozlama)>(&mut self, f: F) -> &mut Self {
        f(&mut self.log); self
    }
}

fn konfig_dsl_misoli() {

    println!("--- Konfiguratsiya DSL ---");

    let mut konfig = AppKonfig::new();
    konfig
        .server(|s| {
            s.host("api.example.com")
                .port(443)
                .workers(16)
                .tls(true);
        })
        .db(|d| {
            d.url("postgresql://localhost:5432/myapp")
                .max_pool(20)
                .timeout(10);
        })
        .log(|l| {
            l.daraja("debug")
                .fayl("/var/log/app.log")
                .konsole(false);
        });

    println!("Server: {}:{} (tls={}, workers={})",
             konfig.server.host, konfig.server.port,
             konfig.server.tls, konfig.server.workers);
    println!("DB: {} (pool={})", konfig.db.url, konfig.db.max_pool);
    println!("Log: {} → {:?}", konfig.log.daraja, konfig.log.fayl);
    // Server: api.example.com:443 (tls=true, workers=16)
    // DB: postgresql://localhost:5432/myapp (pool=20)
    // Log: debug → Some("/var/log/app.log")
}

fn main() {

    println!("=== SQL DSL ===");
    sql_dsl_misoli();

    println!("\n=== HTML DSL ===");
    html_dsl_misoli();

    println!("\n=== TEST FRAMEWORK DSL ===");
    test_dsl_misoli();

    println!("\n=== MATEMATIK DSL ===");
    matematik_dsl_misoli();

    println!("\n=== KONFIGURATSIYA DSL ===");
    konfig_dsl_misoli();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        DSL TEXNIKALARI                                                                                       |
// #================================================================================================================================================#
// # |   1 | macro_rules!                    | Token asosida sintaksis                    | Синтаксис на основе токенов                             |
// # |   2 | Builder + fluent API            | Metod zanjiri                              | Цепочка методов                                         |
// # |   3 | Operator overloading            | +, -, *, &, |, ! matematik notatsiya       | +, -, *, &, |, ! математическая нотация                 |
// # |   4 | Closure API                     | |konfig| { konfig.x = y }                  | |konfig| { konfig.x = y }                               |
// # |   5 | Enum AST                        | Ifoda daraxti                              | Дерево выражений                                        |
// #================================================================================================================================================#
// # |                                        MISOLLAR                                                                                              |
// #================================================================================================================================================#
// # |   6 | SQL Builder                     | & (AND), | (OR), ! (NOT) operatorlari      | Операторы & (AND), | (OR), ! (NOT)                      |
// # |   7 | HTML Builder                    | el().class().id().bola().qur()             | el().class().id().bola().qur()                          |
// # |   8 | Test Framework                  | .teng().tekshir().hisobot()                | .teng().tekshir().hisobot()                             |
// # |   9 | Matematik ifoda                 | n(3.0) + n(4.0), sin(), sqrt()             | n(3.0) + n(4.0), sin(), sqrt()                          |
// # |  10 | Konfig DSL                      | .server(|s| { s.port(443) })               | .server(|s| { s.port(443) })                            |
// #================================================================================================================================================#