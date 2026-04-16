// #================================================================================================================================================#
// #                                                                LIFETIMES                                                                       #
// #                            LIFETIMES — REFERENCE QANCHA VAQT YASHASHINI KOMPILYATOR TEKSHIRADI. DANGLING REFERENCE YO'Q.                       #
// #                            LIFETIMES — КОМПИЛЯТОР ПРОВЕРЯЕТ ВРЕМЯ ЖИЗНИ ССЫЛОК. НЕТ ВИСЯЧИХ ССЫЛОК.                                            #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// Lifetime — reference qancha vaqt amal qilishini belgilaydi
// Lifetime — определяет как долго ссылка остаётся действительной
//
// Muammo (Rust da kompilyatsiya bo'lmaydi):
// Проблема (не компилируется в Rust):
//   let r;
//   {
//       let x = 5;
//       r = &x;      // x bu blokda o'ladi
//   }
//   println!("{}", r); // r endi dangalama reference!
//
// Rust borrow checker bunday kodni rad etadi
// Borrow checker Rust отвергает такой код

// Lifetime elision — kompilyator o'zi aniqlaydi
// Lifetime elision — компилятор определяет сам
//
// Qoidalar (elision rules):
// Правила (elision rules):
//   1. Har input reference o'z lifetime oladi
//   2. Bitta input reference bo'lsa — output shu lifetime
//   3. &self bo'lsa — output self ning lifetime ini oladi

// lifetime annotatsiya yo'q — elision
// без аннотации lifetime — элизия
fn birinchi_soz(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None    => s,
    }
}

// lifetime annotatsiya bilan — 'a
// с аннотацией lifetime — 'a
fn eng_uzun<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// NIMA UCHUN KERAK?
// ЗАЧЕМ НУЖНО?
// Kompilyator bilmaydi — natija x danmi yoki y danmi?
// Компилятор не знает — результат из x или из y?
// 'a deydi: ikkalasi ham kamida 'a yashaydi, natija ham 'a

// ikki xil lifetime — x va y turli umrda
// два разных lifetime — x и y разного времени жизни
fn birinchisini_ol<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x  // faqat x qaytariladi — 'a lifetime
}

// uchta lifetime
// три lifetime
fn birlashtir<'a>(x: &'a str, y: &'a str, z: &'a str) -> String {
    format!("{} {} {}", x, y, z)
}

// struct reference saqlaydi — lifetime kerak
// структура хранит ссылку — нужен lifetime
#[derive(Debug)]
struct Kesim<'a> {
    matn: &'a str,
}

impl<'a> Kesim<'a> {
    fn new(matn: &'a str) -> Self {
        Kesim { matn }
    }

    fn uzunlik(&self) -> usize {
        self.matn.len()
    }

    // &self lifetime — elision qoidasi 3
    // lifetime &self — правило элизии 3
    fn qiymat(&self) -> &str {
        self.matn
    }
}

impl<'a> fmt::Display for Kesim<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.matn)
    }
}

// bir nechta reference field
// несколько полей-ссылок
#[derive(Debug)]
struct Foydalanuvchi<'a> {
    ism: &'a str,
    email: &'a str,
}

impl<'a> Foydalanuvchi<'a> {
    fn new(ism: &'a str, email: &'a str) -> Self {
        Foydalanuvchi { ism, email }
    }

    fn tavsif(&self) -> String {
        format!("{} <{}>", self.ism, self.email)
    }
}

// struct + owned + reference
// структура + owned + ссылка
#[derive(Debug)]
struct MalumotnomaBandi<'a> {
    sarlavha: String,       // owned — lifetime kerak emas
    muallif: &'a str,       // reference — lifetime kerak
    sahifalar: u32,
}

impl<'a> MalumotnomaBandi<'a> {
    fn new(sarlavha: &str, muallif: &'a str, sahifalar: u32) -> Self {
        MalumotnomaBandi {
            sarlavha: sarlavha.to_string(),
            muallif,
            sahifalar,
        }
    }
}

impl<'a> fmt::Display for MalumotnomaBandi<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\" — {} ({} sahifa)", self.sarlavha, self.muallif, self.sahifalar)
    }
}

#[derive(Debug)]
struct Parser<'a> {
    kiritish: &'a str,
    pozitsiya: usize,
}

impl<'a> Parser<'a> {
    fn new(kiritish: &'a str) -> Self {
        Parser { kiritish, pozitsiya: 0 }
    }

    // &self dan reference qaytarish — struct ning lifetime
    // возврат ссылки из &self — lifetime структуры
    fn joriy_qism(&self) -> &str {
        &self.kiritish[self.pozitsiya..]
    }

    // n ta belgi o'tkazish
    // пропуск n символов
    fn oldinga_sil(&mut self, n: usize) {
        self.pozitsiya = (self.pozitsiya + n).min(self.kiritish.len());
    }

    // keyingi so'z
    // следующее слово
    fn keyingi_soz(&mut self) -> Option<&str> {
        let qism = &self.kiritish[self.pozitsiya..];
        let qism = qism.trim_start();
        if qism.is_empty() {
            return None;
        }
        let oxiri = qism.find(|c: char| c.is_whitespace()).unwrap_or(qism.len());
        self.pozitsiya += self.kiritish.len() - qism.len() + oxiri;
        Some(&qism[..oxiri])
    }
}

// Qoida 1: Har input reference o'z lifetime oladi
// Правило 1: Каждая входная ссылка получает свой lifetime
// fn f(x: &str) → fn f<'a>(x: &'a str)
// fn f(x: &str, y: &str) → fn f<'a, 'b>(x: &'a str, y: &'b str)

// Qoida 2: Bitta input reference — output shu lifetime
// Правило 2: Одна входная ссылка — выход получает её lifetime
// fn birinchi(s: &str) -> &str
// →  fn birinchi<'a>(s: &'a str) -> &'a str

// Qoida 3: &self yoki &mut self — output self ning lifetime
// Правило 3: &self или &mut self — выход получает lifetime self
// fn qiymat(&self) -> &str
// →  fn qiymat<'a>(&'a self) -> &'a str

// Elision — aniq yozish (bir xil)
// Элизия — явная запись (одинаково)
fn elision_misoli_1(s: &str) -> &str {
    s  // → fn<'a>(s: &'a str) -> &'a str
}

fn elision_misoli_2<'a>(s: &'a str) -> &'a str {
    s  // yuqori bilan bir xil
}

// generic T + lifetime 'a
// generic T + lifetime 'a
#[derive(Debug)]
struct RefQuti<'a, T> {
    qiymat: &'a T,
}

impl<'a, T: fmt::Display> RefQuti<'a, T> {
    fn new(qiymat: &'a T) -> Self {
        RefQuti { qiymat }
    }

    fn chiqar(&self) {
        println!("{}", self.qiymat);
    }
}

// generic funksiya + lifetime
// generic функция + lifetime
fn ref_eng_katta<'a, T: PartialOrd>(x: &'a T, y: &'a T) -> &'a T {
    if x > y { x } else { y }
}

// ikki turli lifetime — 'a va 'b
// два разных lifetime — 'a и 'b
struct Cache<'a, 'b> {
    kalit: &'a str,
    qiymat: &'b str,
}

impl<'a, 'b> Cache<'a, 'b> {
    fn new(kalit: &'a str, qiymat: &'b str) -> Self {
        Cache { kalit, qiymat }
    }

    fn kalit(&self) -> &'a str {
        self.kalit
    }

    fn qiymat(&self) -> &'b str {
        self.qiymat
    }
}

// lifetime bilan trait implement qilish
// реализация трейта с lifetime
trait Tavsif {
    fn tavsif(&self) -> String;
}

impl<'a> Tavsif for Kesim<'a> {
    fn tavsif(&self) -> String {
        format!("Kesim[{}]: '{}'", self.matn.len(), self.matn)
    }
}

fn scope_misollari() {
    // Sodda scope — lifetime aniq
    // Простой scope — lifetime очевиден
    let x: i32 = 5;
    let r: &i32 = &x;
    println!("{}", r);
    // 5  ← x va r bir xil scopeda

    // Ichki scope — to'g'ri foydalanish
    // Внутренний scope — правильное использование
    let s1: String = String::from("uzun qator");
    let natija: &str;
    {
        let s2: String = String::from("qisqa");
        natija = eng_uzun(s1.as_str(), s2.as_str());
        // s2 bu yerda hali tirik!
        // s2 ещё жива здесь!
        println!("Eng uzun: {}", natija);
    }
    // natija endi ishlatilmaydi — s2 o'lgan
    // natija больше не используется — s2 умерла

    // Vec ichida reference — bir xil lifetime
    // ссылки внутри Vec — одинаковый lifetime
    let s1: String = String::from("bir");
    let s2: String = String::from("ikki");
    let s3: String = String::from("uch");
    let referenslar: Vec<&str> = vec![&s1, &s2, &s3];
    for r in &referenslar {
        print!("{} ", r);
    }
    println!();
    // bir ikki uch
}

// 1. Konfiguratsiya o'quvchi — reference bilan
// 1. Читатель конфигурации — со ссылками
struct KonfiguratsiyaOquvchi<'a> {
    matn: &'a str,
}

impl<'a> KonfiguratsiyaOquvchi<'a> {
    fn new(matn: &'a str) -> Self {
        KonfiguratsiyaOquvchi { matn }
    }

    fn qiymatni_ol(&self, kalit: &str) -> Option<&str> {
        for qator in self.matn.lines() {
            let qismlar: Vec<&str> = qator.splitn(2, '=').collect();
            if qismlar.len() == 2 && qismlar[0].trim() == kalit {
                return Some(qismlar[1].trim());
            }
        }
        None
    }
}

// 2. Tokenizer — reference bilan
// 2. Токенайзер — со ссылками
#[derive(Debug)]
struct Token<'a> {
    tur: &'static str,
    qiymat: &'a str,
}

fn tokenizatsiya<'a>(kiritish: &'a str) -> Vec<Token<'a>> {
    let mut tokenlar: Vec<Token<'a>> = Vec::new();
    let mut pozitsiya: usize = 0;

    while pozitsiya < kiritish.len() {
        let qism: &str = &kiritish[pozitsiya..];

        if qism.starts_with(|c: char| c.is_alphabetic()) {
            let oxiri: usize = qism.find(|c: char| !c.is_alphabetic())
                .unwrap_or(qism.len());
            tokenlar.push(Token {
                tur: "SOZLIK",
                qiymat: &kiritish[pozitsiya..pozitsiya + oxiri],
            });
            pozitsiya += oxiri;
        } else if qism.starts_with(|c: char| c.is_numeric()) {
            let oxiri: usize = qism.find(|c: char| !c.is_numeric())
                .unwrap_or(qism.len());
            tokenlar.push(Token {
                tur: "SON",
                qiymat: &kiritish[pozitsiya..pozitsiya + oxiri],
            });
            pozitsiya += oxiri;
        } else {
            pozitsiya += 1;
        }
    }
    tokenlar
}

// 3. Eng uzun umumiy prefiks
// 3. Наибольший общий префикс
fn eng_uzun_prefiks<'a>(sozlar: &[&'a str]) -> &'a str {
    if sozlar.is_empty() {
        return "";
    }
    let birinchi: &str = sozlar[0];
    let mut oxiri: usize = birinchi.len();

    for soz in &sozlar[1..] {
        oxiri = birinchi
            .chars()
            .zip(soz.chars())
            .take_while(|(a, b)| a == b)
            .count();
    }
    &birinchi[..oxiri]
}

fn main() {

    // birinchi_soz — elision
    // birinchi_soz — элизия
    let gap: &str = "salom dunyo rust";
    let soz: &str = birinchi_soz(gap);
    println!("{}", soz);
    // salom

    // eng_uzun — 'a lifetime
    // eng_uzun — lifetime 'a
    let s1: String = String::from("uzun qator matni");
    let natija: &str;
    {
        let s2: String = String::from("qisqa");
        natija = eng_uzun(s1.as_str(), s2.as_str());
        println!("{}", natija);
    }
    // uzun qator matni

    // birinchisini_ol — ikki xil lifetime
    // birinchisini_ol — два разных lifetime
    let s1: &str = "salom";
    let natija: &str;
    {
        let s2: String = String::from("vaqtinchalik");
        natija = birinchisini_ol(s1, s2.as_str());
        // natija = s1 — s1 ning lifetime ini oladi
        // natija = s1 — получает lifetime s1
    }
    println!("{}", natija);
    // salom

    // birlashtir — uchta lifetime
    // birlashtir — три lifetime
    let a: &str = "bir";
    let b: &str = "ikki";
    let c: &str = "uch";
    let birlashgan: String = birlashtir(a, b, c);
    println!("{}", birlashgan);
    // bir ikki uch

    // Kesim — reference struct
    // Kesim — структура со ссылкой
    let gap2: String = String::from("salom dunyo");
    let kesim: Kesim = Kesim::new(&gap2[0..5]);
    println!("{}", kesim);
    println!("Uzunlik: {}", kesim.uzunlik());
    println!("Qiymat: {}", kesim.qiymat());
    println!("{}", kesim.tavsif());
    // "salom"
    // Uzunlik: 5
    // Qiymat: salom
    // Kesim[5]: 'salom'

    // Foydalanuvchi — ikki reference
    // Foydalanuvchi — две ссылки
    let ism: String = String::from("Dilshod");
    let email: String = String::from("dilshod@example.com");
    let f: Foydalanuvchi = Foydalanuvchi::new(&ism, &email);
    println!("{}", f.tavsif());
    // Dilshod <dilshod@example.com>

    // MalumotnomaBandi — owned + reference
    // MalumotnomaBandi — owned + ссылка
    let muallif: &str = "Donald Knuth";
    let kitob = MalumotnomaBandi::new("The Art of Computer Programming", muallif, 672);
    println!("{}", kitob);
    // "The Art of Computer Programming" — Donald Knuth (672 sahifa)

    // Parser — lifetime bilan
    // Parser — с lifetime
    let kiritish: String = String::from("salom dunyo rust tili");
    let mut parser: Parser = Parser::new(&kiritish);
    println!("{:?}", parser.keyingi_soz());
    println!("{:?}", parser.keyingi_soz());
    println!("{:?}", parser.keyingi_soz());
    // Some("salom")
    // Some("dunyo")
    // Some("rust")

    // RefQuti — generic + lifetime
    // RefQuti — generic + lifetime
    let son: i32 = 42;
    let matn: String = String::from("rust");
    let q1: RefQuti<i32> = RefQuti::new(&son);
    let q2: RefQuti<String> = RefQuti::new(&matn);
    q1.chiqar();
    q2.chiqar();
    println!("{:?}", q1);
    // 42
    // rust
    // RefQuti { qiymat: 42 }

    // ref_eng_katta — generic + lifetime
    // ref_eng_katta — generic + lifetime
    let a: i32 = 10;
    let b: i32 = 20;
    let katta: &i32 = ref_eng_katta(&a, &b);
    println!("{}", katta);
    // 20

    scope_misollari();

    // KonfiguratsiyaOquvchi
    // KonfiguratsiyaOquvchi
    let konfiguratsiya: &str = "
port = 8080
host = localhost
debug = true
timeout = 30
";
    let oquvchi: KonfiguratsiyaOquvchi = KonfiguratsiyaOquvchi::new(konfiguratsiya);
    println!("{:?}", oquvchi.qiymatni_ol("port"));
    println!("{:?}", oquvchi.qiymatni_ol("host"));
    println!("{:?}", oquvchi.qiymatni_ol("yo_q"));
    // Some("8080")
    // Some("localhost")
    // None

    // Tokenizatsiya
    // Токенизация
    let kod: &str = "salom42dunyo100rust";
    let tokenlar: Vec<Token> = tokenizatsiya(kod);
    for token in &tokenlar {
        println!("{}: {}", token.tur, token.qiymat);
    }
    // SOZLIK: salom
    // SON: 42
    // SOZLIK: dunyo
    // SON: 100
    // SOZLIK: rust

    // Eng uzun prefiks
    // Наибольший общий префикс
    let sozlar: Vec<&str> = vec!["salomlash", "salom", "salomchi"];
    let prefiks: &str = eng_uzun_prefiks(&sozlar);
    println!("{}", prefiks);
    // salom
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                       LIFETIME ASOSLARI                                                                                      |
// #================================================================================================================================================#
// # |   1 | &'a T                    | T ga reference — 'a davomida yashaydi                | Ссылка на T — живёт в течение 'a                     |
// # |   2 | fn f<'a>(x: &'a T)       | Funksiyada lifetime annotatsiya                      | Аннотация lifetime в функции                         |
// # |   3 | struct S<'a> { x: &'a T} | Structda lifetime — reference field                  | Lifetime в структуре — поле-ссылка                   |
// # |   4 | impl<'a> S<'a>           | impl blokda lifetime                                 | Lifetime в блоке impl                                |
// #================================================================================================================================================#
// # |                                       ELISION QOIDALARI                                                                                      |
// #================================================================================================================================================#
// # |   5 | Qoida 1                  | Har input reference o'z lifetime oladi               | Каждая входная ссылка получает свой lifetime         |
// # |   6 | Qoida 2                  | 1 ta input → output shu lifetime oladi               | 1 входная → выход получает её lifetime               |
// # |   7 | Qoida 3                  | &self bo'lsa → output self lifetime oladi            | Если &self → выход получает lifetime self            |
// #================================================================================================================================================#
// # |                                       LIFETIME KOMBINATSIYALARI                                                                              |
// #================================================================================================================================================#
// # |   8 | <'a, 'b>                 | Ikki xil lifetime                                    | Два разных lifetime                                  |
// # |   9 | <'a, T>                  | Generic + lifetime birga                             | Generic + lifetime вместе                            |
// # |  10 | <'a: 'b>                 | 'a kamida 'b qadар yashaydi (subtyping)              | 'a живёт не меньше 'b (подтипирование)               |
// # |  11 | fn f<'a>(x: &'a str, _y: &str) -> &'a str | Faqat x qaytarilganda               | Когда возвращается только x                          |
// #================================================================================================================================================#
// # |                                       QACHON KERAK                                                                                           |
// #================================================================================================================================================#
// # |  12 | Funksiya reference qaytarsa | Input reference bilan bog'liqlik kerak             | Нужна связь с входной ссылкой                       |
// # |  13 | Struct reference saqlasa  | Struct fieldda reference bo'lsa                      | Если в поле структуры есть ссылка                   |
// # |  14 | Ikki+ reference kiritish  | Qaysi biri qaytishini aniqlashtirish                 | Уточнение какая из них возвращается                 |
// #================================================================================================================================================#
// # |                                       REAL HAYOT                                                                                             |
// #================================================================================================================================================#
// # |  15 | Tokenizer/Parser         | Kiritish matnidan reference olish                    | Получение ссылок из входного текста                  |
// # |  16 | Konfiguratsiya o'quvchi  | Konfiguratsiya satridan qiymat olish                 | Получение значений из строки конфигурации            |
// # |  17 | RefQuti<'a, T>           | Generic reference container                          | Generic контейнер ссылок                             |
// #================================================================================================================================================#