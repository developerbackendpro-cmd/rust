// #================================================================================================================================================#
// #                                                            BUILDER PATTERN                                                                     #
// #                    BUILDER — MURAKKAB OBYEKT QURISH. TYPESTATE BUILDER. FLUENT API. VALIDATSIYA. GENERIC BUILDER.                              #
// #                    BUILDER — ПОСТРОЕНИЕ СЛОЖНОГО ОБЪЕКТА. TYPESTATE BUILDER. FLUENT API. ВАЛИДАЦИЯ. GENERIC BUILDER.                           #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::collections::HashMap;
use std::time::Duration;
use std::net::SocketAddr;

// Builder Pattern nima:
// Что такое Builder Pattern:
//
//   Murakkab obyektni bosqichma-bosqich qurish
//   Пошаговое построение сложного объекта
//
//   Qachon kerak:
//   Когда нужен:
//   - Ko'p ixtiyoriy parametrlar
//   - Много необязательных параметров
//   - Validatsiya zarur
//   - Нужна валидация
//   - Fluent API (metod zanjiri)
//   - Fluent API (цепочка методов)
//
//   Rust da variatsiyalar:
//   Варианты в Rust:
//   1. Oddiy Builder — pub struct + impl Builder
//   2. TypeState Builder — compile-time validatsiya
//   3. Derive Builder — proc macro bilan
//   4. Generic Builder — turli tur uchun

#[derive(Debug, Clone)]
struct ServerKonfig {
    host: String,
    port: u16,
    max_ulanishlar: u32,
    timeout: Duration,
    tls_yoqilgan: bool,
    log_daraja: String,
    ishchilar: u32,
}

#[derive(Debug, Default)]
struct ServerKonfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    max_ulanishlar: Option<u32>,
    timeout: Option<Duration>,
    tls_yoqilgan: Option<bool>,
    log_daraja: Option<String>,
    ishchilar: Option<u32>,
}

impl ServerKonfigBuilder {
    fn new() -> Self { Self::default() }

    fn host(mut self, host: &str) -> Self {
        self.host = Some(host.to_string()); self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = Some(port); self
    }

    fn max_ulanishlar(mut self, n: u32) -> Self {
        self.max_ulanishlar = Some(n); self
    }

    fn timeout(mut self, secs: u64) -> Self {
        self.timeout = Some(Duration::from_secs(secs)); self
    }

    fn tls(mut self, yoqilgan: bool) -> Self {
        self.tls_yoqilgan = Some(yoqilgan); self
    }

    fn log_daraja(mut self, daraja: &str) -> Self {
        self.log_daraja = Some(daraja.to_string()); self
    }

    fn ishchilar(mut self, n: u32) -> Self {
        self.ishchilar = Some(n); self
    }

    fn qur(self) -> Result<ServerKonfig, Vec<String>> {
        let mut xatolar = Vec::new();

        let host = self.host.unwrap_or_else(|| "0.0.0.0".to_string());
        let port = self.port.unwrap_or(8080);

        if port == 0 {
            xatolar.push("Port 0 bo'lishi mumkin emas".to_string());
        }
        if host.is_empty() {
            xatolar.push("Host bo'sh bo'lishi mumkin emas".to_string());
        }

        let ishchilar = self.ishchilar.unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(|n| n.get() as u32)
                .unwrap_or(4)
        });

        if ishchilar == 0 {
            xatolar.push("Ishchilar soni kamida 1 bo'lishi kerak".to_string());
        }

        if !xatolar.is_empty() {
            return Err(xatolar);
        }

        Ok(ServerKonfig {
            host,
            port,
            max_ulanishlar: self.max_ulanishlar.unwrap_or(1000),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
            tls_yoqilgan: self.tls_yoqilgan.unwrap_or(false),
            log_daraja: self.log_daraja.unwrap_or_else(|| "info".to_string()),
            ishchilar,
        })
    }
}

impl fmt::Display for ServerKonfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Server[{}:{}, workers={}, max_conn={}, tls={}, log={}]",
               self.host, self.port, self.ishchilar,
               self.max_ulanishlar, self.tls_yoqilgan, self.log_daraja
        )
    }
}

fn oddiy_builder_misoli() {

    // To'liq konfiguratsiya
    let konfig = ServerKonfigBuilder::new()
        .host("localhost")
        .port(8080)
        .max_ulanishlar(500)
        .timeout(60)
        .tls(true)
        .log_daraja("debug")
        .ishchilar(8)
        .qur();

    match konfig {
        Ok(k)   => println!("{}", k),
        Err(xatolar) => {
            for x in xatolar { println!("❌ {}", x); }
        }
    }
    // Server[localhost:8080, workers=8, max_conn=500, tls=true, log=debug]

    // Minimal konfiguratsiya — standart qiymatlar
    let minimal = ServerKonfigBuilder::new()
        .port(3000)
        .qur()
        .unwrap();
    println!("{}", minimal);
    // Server[0.0.0.0:3000, workers=N, max_conn=1000, tls=false, log=info]

    // Xatoli konfiguratsiya
    let xatoli = ServerKonfigBuilder::new()
        .port(0)
        .host("")
        .ishchilar(0)
        .qur();

    match xatoli {
        Ok(_)  => println!("OK"),
        Err(v) => {
            println!("Xatolar:");
            for x in v { println!("  ❌ {}", x); }
        }
    }
    // Xatolar:
    //   ❌ Port 0 bo'lishi mumkin emas
    //   ❌ Host bo'sh bo'lishi mumkin emas
    //   ❌ Ishchilar soni kamida 1 bo'lishi kerak
}

// TypeState — qaysi metodlar chaqirilishi kerakligini compile time da tekshirish
// TypeState — проверка во время компиляции какие методы должны быть вызваны

// Holat marker turlar
struct HostBerilmagan;
struct HostBerilgan;
struct PortBerilmagan;
struct PortBerilgan;

struct UlanishBuilder<H, P> {
    host: Option<String>,
    port: Option<u16>,
    protocol: String,
    _host_holat: std::marker::PhantomData<H>,
    _port_holat: std::marker::PhantomData<P>,
}

impl UlanishBuilder<HostBerilmagan, PortBerilmagan> {
    fn new() -> Self {
        UlanishBuilder {
            host: None,
            port: None,
            protocol: "http".to_string(),
            _host_holat: std::marker::PhantomData,
            _port_holat: std::marker::PhantomData,
        }
    }
}

// Host berilgandan keyin — HostBerilgan holatiga o'tish
impl<P> UlanishBuilder<HostBerilmagan, P> {
    fn host(self, host: &str) -> UlanishBuilder<HostBerilgan, P> {
        UlanishBuilder {
            host: Some(host.to_string()),
            port: self.port,
            protocol: self.protocol,
            _host_holat: std::marker::PhantomData,
            _port_holat: std::marker::PhantomData,
        }
    }
}

// Port berilgandan keyin — PortBerilgan holatiga o'tish
impl<H> UlanishBuilder<H, PortBerilmagan> {
    fn port(self, port: u16) -> UlanishBuilder<H, PortBerilgan> {
        UlanishBuilder {
            host: self.host,
            port: Some(port),
            protocol: self.protocol,
            _host_holat: std::marker::PhantomData,
            _port_holat: std::marker::PhantomData,
        }
    }
}

// Protocol — istalgan holatda o'rnatish mumkin
impl<H, P> UlanishBuilder<H, P> {
    fn protocol(mut self, prot: &str) -> Self {
        self.protocol = prot.to_string();
        self
    }
}

// Faqat IKKALASI berilganda qurish mumkin
impl UlanishBuilder<HostBerilgan, PortBerilgan> {
    fn qur(self) -> String {
        format!("{}://{}:{}",
                self.protocol,
                self.host.unwrap(),
                self.port.unwrap()
        )
    }
}

fn typestate_builder_misoli() {

    // To'g'ri tartib
    let url = UlanishBuilder::new()
        .host("api.example.com")
        .port(443)
        .protocol("https")
        .qur();
    println!("{}", url);
    // https://api.example.com:443

    // Tartib farqli ham mumkin
    let url2 = UlanishBuilder::new()
        .port(8080)
        .host("localhost")
        .qur();
    println!("{}", url2);
    // http://localhost:8080

    // Bu KOMPILE BO'LMAYDI — host berilmagan!
    // Это НЕ СКОМПИЛИРУЕТСЯ — host не задан!
    // UlanishBuilder::new().port(80).qur(); // ← XATO!

    // Bu KOMPILE BO'LMAYDI — port berilmagan!
    // Это НЕ СКОМПИЛИРУЕТСЯ — port не задан!
    // UlanishBuilder::new().host("test").qur(); // ← XATO!

    println!("TypeState Builder — compile time kafolat ✅");
}

#[derive(Debug)]
struct SorovBuilder {
    url: String,
    metod: String,
    sarlavhalar: HashMap<String, String>,
    parametrlar: HashMap<String, String>,
    matn: Option<String>,
    timeout: Duration,
    urinishlar: u32,
}

impl SorovBuilder {
    fn get(url: &str) -> Self {
        SorovBuilder {
            url: url.to_string(),
            metod: "GET".to_string(),
            sarlavhalar: HashMap::new(),
            parametrlar: HashMap::new(),
            matn: None,
            timeout: Duration::from_secs(30),
            urinishlar: 1,
        }
    }

    fn post(url: &str) -> Self {
        SorovBuilder {
            url: url.to_string(),
            metod: "POST".to_string(),
            sarlavhalar: HashMap::new(),
            parametrlar: HashMap::new(),
            matn: None,
            timeout: Duration::from_secs(30),
            urinishlar: 1,
        }
    }

    fn sarlavha(mut self, kalit: &str, qiymat: &str) -> Self {
        self.sarlavhalar.insert(kalit.to_string(), qiymat.to_string());
        self
    }

    fn parametr(mut self, kalit: &str, qiymat: &str) -> Self {
        self.parametrlar.insert(kalit.to_string(), qiymat.to_string());
        self
    }

    fn json_matn(mut self, json: &str) -> Self {
        self.matn = Some(json.to_string());
        self.sarlavhalar.insert(
            "Content-Type".to_string(),
            "application/json".to_string()
        );
        self
    }

    fn bearer_token(self, token: &str) -> Self {
        self.sarlavha("Authorization", &format!("Bearer {}", token))
    }

    fn timeout(mut self, secs: u64) -> Self {
        self.timeout = Duration::from_secs(secs);
        self
    }

    fn urinishlar(mut self, n: u32) -> Self {
        self.urinishlar = n;
        self
    }

    fn bajar(&self) -> String {
        // HTTP so'rov simulyatsiyasi
        let mut qismlar = vec![
            format!("{} {}", self.metod, self.url),
        ];

        let mut sarlavhalar: Vec<_> = self.sarlavhalar.iter().collect();
        sarlavhalar.sort_by_key(|(k, _)| k.as_str());
        for (k, v) in sarlavhalar {
            qismlar.push(format!("  {}: {}", k, v));
        }

        if !self.parametrlar.is_empty() {
            let mut params: Vec<_> = self.parametrlar.iter().collect();
            params.sort_by_key(|(k, _)| k.as_str());
            let p: Vec<String> = params.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
            qismlar.push(format!("  Parametrlar: {}", p.join("&")));
        }

        if let Some(matn) = &self.matn {
            qismlar.push(format!("  Matn: {}", matn));
        }

        qismlar.push(format!("  Timeout: {:?}, Urinishlar: {}", self.timeout, self.urinishlar));
        qismlar.join("\n")
    }
}

fn fluent_api_misoli() {

    println!("--- GET so'rov ---");
    let natija = SorovBuilder::get("https://api.example.com/users")
        .bearer_token("my_secret_token")
        .sarlavha("Accept", "application/json")
        .parametr("page", "1")
        .parametr("limit", "20")
        .timeout(10)
        .bajar();
    println!("{}", natija);

    println!("\n--- POST so'rov ---");
    let natija2 = SorovBuilder::post("https://api.example.com/users")
        .bearer_token("my_secret_token")
        .json_matn(r#"{"ism":"Dilshod","yosh":22}"#)
        .urinishlar(3)
        .timeout(30)
        .bajar();
    println!("{}", natija2);
}

// Ma'lumotlar bazasi ulanish builder
#[derive(Debug)]
struct DbUlanish {
    host: String,
    port: u16,
    bazanomi: String,
    foydalanuvchi: String,
    parol: String,
    max_pool: u32,
    min_pool: u32,
    ulanish_timeout: Duration,
    ssl: bool,
}

impl fmt::Display for DbUlanish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "postgresql://{}:***@{}:{}/{}?pool={}-{}&ssl={}",
               self.foydalanuvchi, self.host, self.port,
               self.bazanomi, self.min_pool, self.max_pool, self.ssl
        )
    }
}

struct DbBuilder {
    host: String,
    port: u16,
    bazanomi: Option<String>,
    foydalanuvchi: Option<String>,
    parol: Option<String>,
    max_pool: u32,
    min_pool: u32,
    ulanish_timeout: Duration,
    ssl: bool,
}

impl DbBuilder {
    fn new() -> Self {
        DbBuilder {
            host: "localhost".to_string(),
            port: 5432,
            bazanomi: None,
            foydalanuvchi: None,
            parol: None,
            max_pool: 10,
            min_pool: 2,
            ulanish_timeout: Duration::from_secs(30),
            ssl: false,
        }
    }

    fn host(mut self, h: &str) -> Self { self.host = h.to_string(); self }
    fn port(mut self, p: u16) -> Self { self.port = p; self }
    fn baza(mut self, b: &str) -> Self { self.bazanomi = Some(b.to_string()); self }
    fn foydalanuvchi(mut self, f: &str) -> Self { self.foydalanuvchi = Some(f.to_string()); self }
    fn parol(mut self, p: &str) -> Self { self.parol = Some(p.to_string()); self }
    fn max_pool(mut self, n: u32) -> Self { self.max_pool = n; self }
    fn min_pool(mut self, n: u32) -> Self { self.min_pool = n; self }
    fn timeout(mut self, secs: u64) -> Self { self.ulanish_timeout = Duration::from_secs(secs); self }
    fn ssl(mut self, s: bool) -> Self { self.ssl = s; self }

    fn qur(self) -> Result<DbUlanish, Vec<String>> {
        let mut xatolar = Vec::new();

        let bazanomi = self.bazanomi.ok_or_else(|| {
            xatolar.push("Baza nomi ko'rsatilmagan".to_string());
            ()
        });
        let foydalanuvchi = self.foydalanuvchi.ok_or_else(|| {
            xatolar.push("Foydalanuvchi ko'rsatilmagan".to_string());
            ()
        });
        let parol = self.parol.ok_or_else(|| {
            xatolar.push("Parol ko'rsatilmagan".to_string());
            ()
        });

        if self.min_pool > self.max_pool {
            xatolar.push(format!("min_pool ({}) > max_pool ({})", self.min_pool, self.max_pool));
        }

        if !xatolar.is_empty() { return Err(xatolar); }

        Ok(DbUlanish {
            host: self.host,
            port: self.port,
            bazanomi: bazanomi.unwrap(),
            foydalanuvchi: foydalanuvchi.unwrap(),
            parol: parol.unwrap(),
            max_pool: self.max_pool,
            min_pool: self.min_pool,
            ulanish_timeout: self.ulanish_timeout,
            ssl: self.ssl,
        })
    }
}

fn real_hayot_misollari() {

    println!("--- Server Konfig ---");
    oddiy_builder_misoli();

    println!("\n--- TypeState Builder ---");
    typestate_builder_misoli();

    println!("\n--- Fluent API ---");
    fluent_api_misoli();

    println!("\n--- DB Ulanish Builder ---");
    let db = DbBuilder::new()
        .host("db.production.com")
        .port(5432)
        .baza("myapp_db")
        .foydalanuvchi("app_user")
        .parol("secret_password")
        .max_pool(20)
        .min_pool(5)
        .timeout(10)
        .ssl(true)
        .qur();

    match db {
        Ok(ulanish) => println!("✅ {}", ulanish),
        Err(xatolar) => {
            println!("❌ Xatolar:");
            for x in xatolar { println!("  - {}", x); }
        }
    }
    // ✅ postgresql://app_user:***@db.production.com:5432/myapp_db?pool=5-20&ssl=true

    // Xatoli holat
    let xatoli_db = DbBuilder::new()
        .min_pool(20)
        .max_pool(5) // min > max — xato!
        .qur();

    match xatoli_db {
        Ok(_) => println!("OK"),
        Err(xatolar) => {
            println!("\n❌ Xatolar:");
            for x in xatolar { println!("  - {}", x); }
        }
    }
    // ❌ Xatolar:
    //   - Baza nomi ko'rsatilmagan
    //   - Foydalanuvchi ko'rsatilmagan
    //   - Parol ko'rsatilmagan
    //   - min_pool (20) > max_pool (5)
}

fn main() {

    println!("=== ODDIY BUILDER ===");
    oddiy_builder_misoli();

    println!("\n=== TYPESTATE BUILDER ===");
    typestate_builder_misoli();

    println!("\n=== FLUENT API ===");
    fluent_api_misoli();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                        | Tavsif (UZ)                           | Описание (RU)                                            |
// #================================================================================================================================================#
// # |                                        ODDIY BUILDER                                                                                         |
// #================================================================================================================================================#
// # |   1 | struct XBuilder { fields: Option }  | Option maydonlar                      | Поля Option                                              |
// # |   2 | fn field(mut self, v) -> Self       | Fluent metod — self qaytarish         | Fluent метод — возврат self                              |
// # |   3 | fn qur(self) -> Result<X, E>        | Validatsiya + obyekt qurish           | Валидация + построение объекта                           |
// # |   4 | #[derive(Default)]                  | Default qiymatlar                     | Значения по умолчанию                                    |
// # |   5 | unwrap_or_else(|| standart)         | Majburiy bo'lmagan maydonlar          | Необязательные поля                                      |
// #================================================================================================================================================#
// # |                                        TYPESTATE BUILDER                                                                                     |
// #================================================================================================================================================#
// # |   6 | struct Builder<H, P> { ... }        | Generic holat parametrlari            | Обобщённые параметры состояния                           |
// # |   7 | struct HolatA; struct HolatB;       | Marker tur holatlari                  | Маркерные типы состояний                                 |
// # |   8 | impl Builder<HolatA, _>             | Faqat ma'lum holatda metod            | Метод только в определённом состоянии                    |
// # |   9 | fn qur(self: Builder<A,B>)          | Faqat to'liq holatda qurish           | Построение только в полном состоянии                     |
// # |  10 | Compile-time kafolat                | Runtime xato yo'q                     | Нет ошибок в runtime                                     |
// #================================================================================================================================================#
// # |                                        FLUENT API                                                                                            |
// #================================================================================================================================================#
// # |  11 | Metod zanjiri                       | .a().b().c().qur()                    | Цепочка методов                                          |
// # |  12 | Builder::get(url) / post(url)       | Turli boshlang'ich holatlar           | Различные начальные состояния                            |
// # |  13 | Qurilish va bajarish ajratish       | Builder va Executor                   | Разделение построения и выполнения                       |
// #================================================================================================================================================#