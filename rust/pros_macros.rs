// #================================================================================================================================================#
// #                                                            PROC MACROS                                                                         #
// #            PROTSESSUAL MAKROLAR — TOKEN STREAM USTIDA ISHLASH. DERIVE, ATTRIBUTE, FUNCTION-LIKE. REAL KUTUBXONA MISOLLARI.                     #
// #            ПРОЦЕДУРНЫЕ МАКРОСЫ — РАБОТА С TOKEN STREAM. DERIVE, ATTRIBUTE, FUNCTION-LIKE. ПРИМЕРЫ РЕАЛЬНЫХ БИБЛИОТЕК.                          #
// #================================================================================================================================================#

#![allow(dead_code, unused)]
use std::fmt;
use std::collections::HashMap;

// Serde simulyatsiyasi (haqiqiy serde o'rniga)
// Симуляция Serde (вместо настоящего serde)
trait Serializatsiya {
    fn serializatsiya(&self) -> String;
}

trait Deserializatsiya: Sized {
    fn deserializatsiya(s: &str) -> Result<Self, String>;
}

// macro_rules! bilan derive simulyatsiyasi
// Симуляция derive с macro_rules!
macro_rules! derive_serialize {
    (
        $nomi:ident {
            $($field:ident: $tur:ty),+ $(,)?
        }
    ) => {
        impl Serializatsiya for $nomi {
            fn serializatsiya(&self) -> String {
                let mut juftlar = Vec::new();
                $(
                    juftlar.push(format!("\"{}\":{:?}", stringify!($field), self.$field));
                )+
                format!("{{{}}}", juftlar.join(","))
            }
        }

        impl fmt::Debug for $nomi {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} {{ ", stringify!($nomi))?;
                $(write!(f, "{}: {:?} ", stringify!($field), self.$field)?;)+
                write!(f, "}}")
            }
        }
    };
}

#[derive(Clone)]
struct Foydalanuvchi {
    id: u64,
    ism: String,
    email: String,
    yosh: u32,
}

derive_serialize!(Foydalanuvchi {
    id: u64,
    ism: String,
    email: String,
    yosh: u32,
});

#[derive(Clone)]
struct Mahsulot {
    id: u64,
    nomi: String,
    narx: f64,
    mavjud: bool,
}

derive_serialize!(Mahsulot {
    id: u64,
    nomi: String,
    narx: f64,
    mavjud: bool,
});

fn derive_macro_misollari() {

    let f = Foydalanuvchi {
        id: 1,
        ism: "Dilshod".to_string(),
        email: "dilshod@rust.uz".to_string(),
        yosh: 22,
    };

    println!("{}", f.serializatsiya());
    println!("{:?}", f);
    // {"id":1,"ism":"Dilshod","email":"dilshod@rust.uz","yosh":22}
    // Foydalanuvchi { id: 1 ism: "Dilshod" email: "dilshod@rust.uz" yosh: 22 }

    let m = Mahsulot {
        id: 42,
        nomi: "Olma".to_string(),
        narx: 1500.0,
        mavjud: true,
    };

    println!("{}", m.serializatsiya());
    // {"id":42,"nomi":"Olma","narx":1500.0,"mavjud":true}
}

macro_rules! route {
    ($metod:literal, $yo_l:literal, fn $fn_nomi:ident($($param:ident: $tur:ty),*) $blok:block) => {
        fn $fn_nomi($($param: $tur),*) -> String $blok

        // Routelarni ro'yxatlash
        // Регистрация маршрутов
        fn concat_route() -> (&'static str, &'static str, &'static str) {
            ($metod, $yo_l, stringify!($fn_nomi))
        }
    };
}

macro_rules! timed {
    (fn $fn_nomi:ident($($param:ident: $tur:ty),*) -> $ret:ty $blok:block) => {
        fn $fn_nomi($($param: $tur),*) -> $ret {
            let _boshlanish = std::time::Instant::now();
            let natija = (|| $blok)();
            println!("[VAQT] {} — {:.3?}", stringify!($fn_nomi), _boshlanish.elapsed());
            natija
        }
    };
}

macro_rules! retry {
    ($urinishlar:literal, fn $fn_nomi:ident($($param:ident: $tur:ty),*) -> Result<$ok:ty, $err:ty> $blok:block) => {
        fn $fn_nomi($($param: $tur),*) -> Result<$ok, $err> {
            let mut oxirgi_xato = None;
            for urinish in 0..$urinishlar {
                match (|| -> Result<$ok, $err> $blok)() {
                    Ok(v) => return Ok(v),
                    Err(e) => {
                        println!("[RETRY] Urinish {}/{}", urinish + 1, $urinishlar);
                        oxirgi_xato = Some(e);
                    }
                }
            }
            Err(oxirgi_xato.unwrap())
        }
    };
}

// timed attribute simulyatsiyasi
// Симуляция attribute timed
timed! {
    fn sekin_hisoblash(n: u64) -> u64 {
        (1..=n).sum()
    }
}

fn attribute_macro_misollari() {

    // timed — vaqt o'lchash
    println!("{}", sekin_hisoblash(1_000_000));
    // [VAQT] sekin_hisoblash — 0.XYZms
    // 500000500000

    // retry simulyatsiyasi
    let mut urinish_soni = 0;
    let natija: Result<String, &str> = {
        let mut oxirgi = None;
        for i in 0..3 {
            urinish_soni += 1;
            if urinish_soni < 3 {
                println!("[RETRY] Urinish {}/3", i + 1);
                oxirgi = Some(Err("vaqtinchalik xato"));
            } else {
                oxirgi = Some(Ok(format!("Muvaffaqiyat: {} urinish", urinish_soni)));
                break;
            }
        }
        oxirgi.unwrap()
    };
    println!("{:?}", natija);
    // [RETRY] Urinish 1/3
    // [RETRY] Urinish 2/3
    // Ok("Muvaffaqiyat: 3 urinish")
}

macro_rules! html {
    (tag: $tag:ident, content: $mazmun:literal) => {
        format!("<{}>{}</{}>", stringify!($tag), $mazmun, stringify!($tag))
    };
    (tag: $tag:ident) => {
        format!("<{}>", stringify!($tag))
    };
}

macro_rules! json {
    (null) => { "null".to_string() };
    (true) => { "true".to_string() };
    (false) => { "false".to_string() };
    ($n:literal) => { format!("{}", $n) };
    ([$($element:tt),* $(,)?]) => {
        {
            let elementlar: Vec<String> = vec![$(json!($element)),*];
            format!("[{}]", elementlar.join(","))
        }
    };
    ({$($kalit:literal: $qiymat:tt),* $(,)?}) => {
        {
            let juftlar: Vec<String> = vec![
                $(format!("\"{}\":{}", $kalit, json!($qiymat))),*
            ];
            format!("{{{}}}", juftlar.join(","))
        }
    };
}

fn function_like_macro_misollari() {

    // HTML generator
    let h1 = html!(tag: h1, content: "Salom Dunyo");
    let div = html!(tag: div, content: "Rust tili");
    println!("{}", h1);
    println!("{}", div);
    // <h1>Salom Dunyo</h1>
    // <div>Rust tili</div>

    // JSON generator
    let j1 = json!(null);
    let j2 = json!(42);
    let j3 = json!(true);
    let j4 = json!([1, 2, 3]);
    let j5 = json!({
        "ism": "Dilshod",
        "yosh": 22,
        "faol": true
    });
    println!("{}", j1);
    println!("{}", j2);
    println!("{}", j3);
    println!("{}", j4);
    println!("{}", j5);
    // null
    // 22
    // true
    // [1,2,3]
    // {"ism":"Dilshod","yosh":22,"faol":true}
}

// Serde simulyatsiyasi
// Симуляция Serde
struct JsonSerializer {
    natija: String,
}

impl JsonSerializer {
    fn new() -> Self { JsonSerializer { natija: String::new() } }

    fn yoz_kalit(&mut self, kalit: &str, qiymat: &str) {
        if !self.natija.is_empty() { self.natija.push(','); }
        self.natija.push_str(&format!("\"{}\":{}", kalit, qiymat));
    }

    fn tugatish(self) -> String {
        format!("{{{}}}", self.natija)
    }
}

macro_rules! serde_like {
    (
        $(#[$meta:meta])*
        struct $nomi:ident {
            $(
                $(#[field($field_attr:ident = $field_val:literal)])?
                $field:ident: $tur:ty
            ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        struct $nomi {
            $($field: $tur,)+
        }

        impl $nomi {
            fn to_json(&self) -> String {
                let mut s = JsonSerializer::new();
                $(
                    let kalit = {
                        $(
                            if stringify!($field_attr) == "rename" {
                                $field_val
                            } else {
                                stringify!($field)
                            }
                        )?
                        stringify!($field)
                    };
                    s.yoz_kalit(kalit, &format!("{:?}", self.$field));
                )+
                s.tugatish()
            }
        }
    };
}

serde_like! {
    #[allow(dead_code)]
    struct ServerKonfig {
        host: String,
        port: u16,
        debug: bool,
        timeout: u32
    }
}

fn serde_simulyatsiya_misollari() {

    let konfig = ServerKonfig {
        host: "localhost".to_string(),
        port: 8080,
        debug: false,
        timeout: 30,
    };

    println!("{}", konfig.to_json());
    // {"host":"localhost","port":8080,"debug":false,"timeout":30}
}

// Clap simulyatsiyasi
// Симуляция Clap
macro_rules! cli_parser {
    (
        struct $nomi:ident {
            $($field:ident: $tur:ty = $standart:expr),+ $(,)?
        }
    ) => {
        struct $nomi {
            $($field: $tur,)+
        }

        impl $nomi {
            fn standart() -> Self {
                $nomi {
                    $($field: $standart,)+
                }
            }

            fn parse_args(args: &[&str]) -> Self {
                let mut konfig = Self::standart();
                let mut i = 0;
                while i < args.len() {
                    match args[i] {
                        $(
                            x if x == concat!("--", stringify!($field)) => {
                                if i + 1 < args.len() {
                                    i += 1;
                                    // Sodda parse
                                    let val_str = args[i];
                                    konfig.$field = val_str.parse()
                                        .unwrap_or(konfig.$field);
                                }
                            }
                        )+
                        "--help" => {
                            println!("Yordam:");
                            $(println!("  --{}: (standart: {:?})", stringify!($field), $standart);)+
                        }
                        _ => {}
                    }
                    i += 1;
                }
                konfig
            }
        }
    };
}

cli_parser! {
    struct AppCli {
        port: u16 = 8080u16,
        workers: u32 = 4u32,
        debug: bool = false
    }
}

fn clap_simulyatsiya_misollari() {

    let standart = AppCli::standart();
    println!("Port: {}, Workers: {}, Debug: {}", standart.port, standart.workers, standart.debug);
    // Port: 8080, Workers: 4, Debug: false

    let args = ["--port", "3000", "--debug", "true", "--workers", "8"];
    let konfig = AppCli::parse_args(&args);
    println!("Port: {}, Workers: {}, Debug: {}", konfig.port, konfig.workers, konfig.debug);
    // Port: 3000, Workers: 8, Debug: true
}

// Oddiy builder — to'liq ishlaydi
// Простой builder — полностью работает
macro_rules! simple_builder {
    (struct $nomi:ident { $($field:ident: $tur:ty),+ $(,)? }) => {

        #[derive(Debug, Clone)]
        pub struct $nomi {
            $(pub $field: $tur,)+
        }

        #[derive(Default)]
        pub struct Builder {
            $($field: Option<$tur>,)+
        }

        impl Builder {
            pub fn new() -> Self { Builder::default() }

            $(
                pub fn $field(mut self, val: $tur) -> Self {
                    self.$field = Some(val);
                    self
                }
            )+

            pub fn qur(self) -> Result<$nomi, String> {
                Ok($nomi {
                    $(
                        $field: self.$field.ok_or_else(||
                            format!("'{}' maydoni ko'rsatilmagan", stringify!($field))
                        )?,
                    )+
                })
            }
        }

        impl $nomi {
            pub fn builder() -> Builder { Builder::new() }
        }
    };
}

simple_builder! {
    struct HttpSorov {
        metod: String,
        url: String,
        timeout_ms: u64,
        takrorlashlar: u32
    }
}

fn real_hayot_misollari() {

    // Derive macro misollari
    derive_macro_misollari();

    // Serde simulyatsiya
    serde_simulyatsiya_misollari();

    // Clap simulyatsiya
    clap_simulyatsiya_misollari();

    // Builder pattern
    let so_rov = HttpSorov::builder()
        .metod("GET".to_string())
        .url("https://api.example.com/users".to_string())
        .timeout_ms(5000)
        .takrorlashlar(3)
        .qur();

    match so_rov {
        Ok(s) => println!("{} {} (timeout={}ms)", s.metod, s.url, s.timeout_ms),
        Err(e) => println!("Xato: {}", e),
    }
    // GET https://api.example.com/users (timeout=5000ms)

    // Xato holati
    let xato_so_rov = HttpSorov::builder()
        .metod("POST".to_string())
        .qur();

    println!("{:?}", xato_so_rov.map_err(|e| e));
    // Err("'url' maydoni ko'rsatilmagan")

    // JSON generatsiya
    function_like_macro_misollari();

    // Attribute macro
    attribute_macro_misollari();
}

fn main() {

    println!("=== DERIVE PROC MACRO ===");
    derive_macro_misollari();

    println!("\n=== ATTRIBUTE PROC MACRO ===");
    attribute_macro_misollari();

    println!("\n=== FUNCTION-LIKE PROC MACRO ===");
    function_like_macro_misollari();

    println!("\n=== SERDE SIMULYATSIYA ===");
    serde_simulyatsiya_misollari();

    println!("\n=== CLAP SIMULYATSIYA ===");
    clap_simulyatsiya_misollari();

    println!("\n=== REAL HAYOT ===");
    let so_rov = HttpSorov::builder()
        .metod("GET".to_string())
        .url("https://api.example.com".to_string())
        .timeout_ms(3000)
        .takrorlashlar(3)
        .qur();
    println!("{:?}", so_rov.map(|s| format!("{} {}", s.metod, s.url)));
    // Ok("GET https://api.example.com")
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        PROC MACRO TURLARI                                                                                    |
// #================================================================================================================================================#
// # |   1 | #[proc_macro_derive(Name)]      | Derive macro — trait implement             | Derive macro — реализация трейта                        |
// # |   2 | #[proc_macro_attribute]         | Attribute macro — elementni o'zgartirish   | Attribute macro — изменение элемента                    |
// # |   3 | #[proc_macro]                   | Function-like macro                        | Функционально-подобный макрос                           |
// # |   4 | proc-macro = true               | Cargo.toml da yoqish                       | Включение в Cargo.toml                                  |
// #================================================================================================================================================#
// # |                                        SYN + QUOTE                                                                                           |
// #================================================================================================================================================#
// # |   5 | parse_macro_input!(i as T)      | TokenStream → syn AST                      | TokenStream → syn AST                                   |
// # |   6 | quote! { ... }                  | syn AST → TokenStream                      | syn AST → TokenStream                                   |
// # |   7 | #nomi                           | quote! interpolatsiya                      | Интерполяция в quote!                                   |
// # |   8 | #(#field_nomi: None,)*          | quote! takrorlash                          | Повторение в quote!                                     |
// # |   9 | DeriveInput                     | Struct/enum parse                          | Парсинг struct/enum                                     |
// # |  10 | format_ident!("{}Builder", n)   | Identifikator yaratish                     | Создание идентификатора                                 |
// #================================================================================================================================================#
// # |                                        REAL KUTUBXONALAR                                                                                     |
// #================================================================================================================================================#
// # |  11 | serde::Serialize                | JSON/TOML/YAML serialize                   | Сериализация JSON/TOML/YAML                             |
// # |  12 | serde::Deserialize              | JSON/TOML/YAML deserialize                 | Десериализация JSON/TOML/YAML                           |
// # |  13 | clap::Parser                    | CLI argumentlarni parse                    | Парсинг аргументов CLI                                  |
// # |  14 | tokio::main                     | Async main yaratish                        | Создание async main                                     |
// # |  15 | thiserror::Error                | Error Display avtomatik                    | Автоматический Display для ошибок                       |
// # |  16 | derive_more                     | Ko'p traitlarni derive qilish              | Derive многих трейтов                                   |
// #================================================================================================================================================#