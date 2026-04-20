// #================================================================================================================================================#
// #                                                            TYPESTATE PATTERN                                                                   #
// #                        TYPESTATE — COMPILE-TIME HOLAT MASHINASI. PHANTOM TYPE. NOTO'G'RI HOLAT IMKONSIZ.                                       #
// #                        TYPESTATE — МАШИНА СОСТОЯНИЙ ВО ВРЕМЯ КОМПИЛЯЦИИ. PHANTOM TYPE. НЕВЕРНОЕ СОСТОЯНИЕ НЕВОЗМОЖНО.                          #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::marker::PhantomData;
use std::fmt;

// TypeState Pattern nima:
// Что такое TypeState Pattern:
//
//   Ob'ekt holati tur parametri sifatida kodlanadi
//   Состояние объекта кодируется как параметр типа
//
//   Afzalliklari:
//   Преимущества:
//   ✅ Noto'g'ri holat imkonsiz (compile-time)
//      Неверное состояние невозможно (во время компиляции)
//   ✅ Metodlar faqat to'g'ri holatda mavjud
//      Методы доступны только в правильном состоянии
//   ✅ Zero-cost (runtime overhead yo'q)
//      Нулевые затраты (нет накладных расходов)
//   ✅ Xato xabarlari aniq
//      Понятные сообщения об ошибках
//
//   Qachon ishlatish:
//   Когда использовать:
//   - Protokol holatlar (TCP: Closed → Listen → Established)
//   - Tranzaksiya (Begin → Active → Commit/Rollback)
//   - Builder majburiy qadamlar
//   - File (Yopiq → Ochiq → O'qish/Yozish)
//   - Lock (Erkin → Qulflangan)

// Holat marker turlari
struct Yopilgan;
struct Ochilgan;
struct OqishUchun;
struct YozishUchun;

struct Fayl<Holat> {
    nomi: String,
    tarkib: Vec<u8>,
    kursor: usize,
    _holat: PhantomData<Holat>,
}

// ─── Yopilgan holat ───
impl Fayl<Yopilgan> {
    fn yangi(nomi: &str) -> Self {
        println!("[Fayl] '{}' yaratildi (yopilgan)", nomi);
        Fayl { nomi: nomi.to_string(), tarkib: Vec::new(), kursor: 0, _holat: PhantomData }
    }

    fn o_qish_uchun_och(self) -> Fayl<OqishUchun> {
        println!("[Fayl] '{}' o'qish uchun ochildi", self.nomi);
        Fayl { nomi: self.nomi, tarkib: self.tarkib, kursor: 0, _holat: PhantomData }
    }

    fn yozish_uchun_och(self) -> Fayl<YozishUchun> {
        println!("[Fayl] '{}' yozish uchun ochildi", self.nomi);
        Fayl { nomi: self.nomi, tarkib: self.tarkib, kursor: 0, _holat: PhantomData }
    }
}

// ─── O'qish holati ───
impl Fayl<OqishUchun> {
    fn o_qi(&mut self, n: usize) -> &[u8] {
        let boshi = self.kursor;
        let oxiri = (self.kursor + n).min(self.tarkib.len());
        self.kursor = oxiri;
        &self.tarkib[boshi..oxiri]
    }

    fn hammasini_o_qi(&self) -> &[u8] { &self.tarkib }

    fn satr_o_qi(&self) -> Vec<String> {
        String::from_utf8_lossy(&self.tarkib)
            .lines()
            .map(|s| s.to_string())
            .collect()
    }

    fn yop(self) -> Fayl<Yopilgan> {
        println!("[Fayl] '{}' yopildi", self.nomi);
        Fayl { nomi: self.nomi, tarkib: self.tarkib, kursor: 0, _holat: PhantomData }
    }
}

// ─── Yozish holati ───
impl Fayl<YozishUchun> {
    fn yoz(&mut self, ma_lumot: &[u8]) -> usize {
        self.tarkib.extend_from_slice(ma_lumot);
        ma_lumot.len()
    }

    fn yoz_satr(&mut self, satr: &str) {
        self.tarkib.extend_from_slice(satr.as_bytes());
        self.tarkib.push(b'\n');
    }

    fn tozala(&mut self) { self.tarkib.clear(); self.kursor = 0; }

    fn yop(self) -> Fayl<Yopilgan> {
        println!("[Fayl] '{}' yopildi ({} bayt)", self.nomi, self.tarkib.len());
        Fayl { nomi: self.nomi, tarkib: self.tarkib, kursor: 0, _holat: PhantomData }
    }
}

fn fayl_typestate_misoli() {

    println!("=== FAYL TYPESTATE ===\n");

    // Yozish
    let fayl = Fayl::<Yopilgan>::yangi("config.toml");
    let mut yoziladigan = fayl.yozish_uchun_och();
    yoziladigan.yoz_satr("host = \"localhost\"");
    yoziladigan.yoz_satr("port = 8080");
    yoziladigan.yoz_satr("debug = true");
    let yopilgan = yoziladigan.yop();

    // O'qish
    let mut o_qiladigan = yopilgan.o_qish_uchun_och();
    let satrlar = o_qiladigan.satr_o_qi();
    println!("Satrlar:");
    for s in &satrlar { println!("  {}", s); }
    let yopilgan2 = o_qiladigan.yop();

    // Bu KOMPILE BO'LMAYDI:
    // Это НЕ СКОМПИЛИРУЕТСЯ:
    // yopilgan2.yoz_satr("...");   // Yopilgan holat!
    // yoziladigan.o_qi(5);         // moved!
    println!("Fayl TypeState kafolati ✅");
    // [Fayl] 'config.toml' yaratildi (yopilgan)
    // [Fayl] 'config.toml' yozish uchun ochildi
    // [Fayl] 'config.toml' yopildi (38 bayt)
    // [Fayl] 'config.toml' o'qish uchun ochildi
    // host = "localhost"
    // port = 8080
    // debug = true
    // [Fayl] 'config.toml' yopildi
}

struct Bgoshlash;
struct Tinglash;
struct UlanishQabul;
struct Olingan;
struct Yopish;

struct TcpUlanish<Holat> {
    manzil: String,
    port: u16,
    bufer: Vec<u8>,
    _holat: PhantomData<Holat>,
}

impl TcpUlanish<Bgoshlash> {
    fn new(manzil: &str, port: u16) -> Self {
        println!("[TCP] Yaratildi: {}:{}", manzil, port);
        TcpUlanish {
            manzil: manzil.to_string(), port,
            bufer: Vec::new(), _holat: PhantomData,
        }
    }

    fn bog_lash(self) -> TcpUlanish<Tinglash> {
        println!("[TCP] Socket bog'landi: {}:{}", self.manzil, self.port);
        TcpUlanish { manzil: self.manzil, port: self.port, bufer: self.bufer, _holat: PhantomData }
    }
}

impl TcpUlanish<Tinglash> {
    fn qabul_qilish(self) -> TcpUlanish<UlanishQabul> {
        println!("[TCP] Ulanish qabul qilindi");
        TcpUlanish { manzil: self.manzil, port: self.port, bufer: self.bufer, _holat: PhantomData }
    }

    fn yopish(self) -> TcpUlanish<Yopish> {
        println!("[TCP] Tinglash to'xtatildi");
        TcpUlanish { manzil: self.manzil, port: self.port, bufer: self.bufer, _holat: PhantomData }
    }
}

impl TcpUlanish<UlanishQabul> {
    fn jo_natish(mut self, ma_lumot: &[u8]) -> (Self, usize) {
        println!("[TCP] Jo'natildi: {} bayt", ma_lumot.len());
        self.bufer.extend_from_slice(ma_lumot);
        let n = ma_lumot.len();
        (self, n)
    }

    fn qabul(mut self, ma_lumot: &[u8]) -> (Self, &'static str) {
        println!("[TCP] Qabul qilindi: {} bayt", ma_lumot.len());
        self.bufer.extend_from_slice(ma_lumot);
        (self, "OK")
    }

    fn yop_ulanish(self) -> TcpUlanish<Yopish> {
        println!("[TCP] Ulanish yopildi (FIN jo'natildi)");
        TcpUlanish { manzil: self.manzil, port: self.port, bufer: self.bufer, _holat: PhantomData }
    }
}

impl TcpUlanish<Yopish> {
    fn resurslarni_ozod_qil(self) {
        println!("[TCP] Resurslar ozod qilindi. Jami bufer: {} bayt", self.bufer.len());
    }
}

fn tcp_typestate_misoli() {

    println!("\n=== TCP PROTOKOL TYPESTATE ===\n");

    let ulanish = TcpUlanish::<Bgoshlash>::new("0.0.0.0", 8080)
        .bog_lash()
        .qabul_qilish();

    let (ulanish, n) = ulanish.jo_natish(b"HTTP/1.1 200 OK\r\n");
    println!("Jo'natildi: {} bayt", n);

    let (ulanish, _) = ulanish.qabul(b"GET / HTTP/1.1\r\n\r\n");

    ulanish
        .yop_ulanish()
        .resurslarni_ozod_qil();

    // Bu KOMPILE BO'LMAYDI:
    // TcpUlanish::new("x", 80).qabul_qilish(); // Bog'lanmasdan qabul!
    // TcpUlanish::new("x", 80).bog_lash().jo_natish(b"..."); // Qabul qilinmasdan!
    println!("TCP holat kafolati ✅");
}

struct BoshlangichTx;
struct FaolTx;
struct MuvaffaqiyatliTx;
struct BekorTx;

struct Tranzaksiya<Holat> {
    id: u64,
    operatsiyalar: Vec<String>,
    _holat: PhantomData<Holat>,
}

impl Tranzaksiya<BoshlangichTx> {
    fn yangi(id: u64) -> Self {
        println!("[TX #{}] BEGIN", id);
        Tranzaksiya { id, operatsiyalar: Vec::new(), _holat: PhantomData }
    }

    fn boshlash(self) -> Tranzaksiya<FaolTx> {
        Tranzaksiya { id: self.id, operatsiyalar: self.operatsiyalar, _holat: PhantomData }
    }
}

impl Tranzaksiya<FaolTx> {
    fn bajar(&mut self, sql: &str) -> &mut Self {
        println!("[TX #{}] SQL: {}", self.id, sql);
        self.operatsiyalar.push(sql.to_string());
        self
    }

    fn commit(self) -> Tranzaksiya<MuvaffaqiyatliTx> {
        println!("[TX #{}] COMMIT ({} operatsiya)", self.id, self.operatsiyalar.len());
        Tranzaksiya { id: self.id, operatsiyalar: self.operatsiyalar, _holat: PhantomData }
    }

    fn rollback(self) -> Tranzaksiya<BekorTx> {
        println!("[TX #{}] ROLLBACK ({} operatsiya bekor qilindi)", self.id, self.operatsiyalar.len());
        Tranzaksiya { id: self.id, operatsiyalar: self.operatsiyalar, _holat: PhantomData }
    }
}

impl Tranzaksiya<MuvaffaqiyatliTx> {
    fn natija(&self) -> String {
        format!("TX #{} muvaffaqiyatli ({} operatsiya)", self.id, self.operatsiyalar.len())
    }
}

impl Tranzaksiya<BekorTx> {
    fn sabab(&self) -> String {
        format!("TX #{} bekor qilindi ({} operatsiya)", self.id, self.operatsiyalar.len())
    }
}

fn tranzaksiya_typestate_misoli() {

    println!("\n=== TRANZAKSIYA TYPESTATE ===\n");

    // Muvaffaqiyatli
    let mut tx = Tranzaksiya::yangi(1).boshlash();
    tx.bajar("INSERT INTO users VALUES (1, 'Dilshod')")
        .bajar("UPDATE balances SET amount = 1000 WHERE id = 1")
        .bajar("INSERT INTO logs VALUES (NOW(), 'user_created')");
    let ok = tx.commit();
    println!("{}", ok.natija());
    println!();

    // Bekor
    let mut tx2 = Tranzaksiya::yangi(2).boshlash();
    tx2.bajar("DELETE FROM orders WHERE id = 99")
        .bajar("UPDATE inventory SET count = count - 1");
    let err = tx2.rollback();
    println!("{}", err.sabab());

    // Bu KOMPILE BO'LMAYDI:
    // ok.rollback();    // Muvaffaqiyatli tx rollback qilib bo'lmaydi!
    // err.commit();     // Bekor tx commit qilib bo'lmaydi!
    println!("\nTranzaksiya holat kafolati ✅");
}

struct Qizil;
struct Sariq;
struct Yashil;

struct Svetofor<Rang> {
    _rang: PhantomData<Rang>,
}

impl Svetofor<Qizil> {
    fn new() -> Self {
        println!("[Svetofor] QIZIL 🔴");
        Svetofor { _rang: PhantomData }
    }
    fn sariq_ga_otish(self) -> Svetofor<Sariq> {
        println!("[Svetofor] SARIQ 🟡 (tayyorlaning...)");
        Svetofor { _rang: PhantomData }
    }
}

impl Svetofor<Sariq> {
    fn yashil_ga_otish(self) -> Svetofor<Yashil> {
        println!("[Svetofor] YASHIL 🟢 (yuring!)");
        Svetofor { _rang: PhantomData }
    }
}

impl Svetofor<Yashil> {
    fn qizil_ga_otish(self) -> Svetofor<Qizil> {
        println!("[Svetofor] QIZIL 🔴 (to'xtang!)");
        Svetofor { _rang: PhantomData }
    }

    fn o_ting(&self) {
        println!("[Svetofor] Yo'ldan o'tyapmiz...");
    }
}

fn svetofor_misoli() {

    println!("\n=== SVETOFOR TYPESTATE ===\n");

    let qizil = Svetofor::<Qizil>::new();
    let sariq = qizil.sariq_ga_otish();
    let yashil = sariq.yashil_ga_otish();

    yashil.o_ting();
    yashil.o_ting();

    let qizil2 = yashil.qizil_ga_otish();
    let sariq2 = qizil2.sariq_ga_otish();
    let _yashil2 = sariq2.yashil_ga_otish();

    // Bu KOMPILE BO'LMAYDI:
    // qizil2.yashil_ga_otish(); // Qizildan to'g'ridan yashilga yo'q!
    // sariq2.qizil_ga_otish();  // Sariqdan qizilga yo'q!
    println!("Svetofor tartib kafolati ✅");
}

struct Yaratilgan;
struct SarlavhalarQoshilgan;
struct TanaQoshilgan;
struct Yuborilgan;
struct HttpOlingan;

struct HttpSorov<Holat> {
    metod: String,
    url: String,
    sarlavhalar: Vec<(String, String)>,
    tana: Option<Vec<u8>>,
    javob: Option<(u16, Vec<u8>)>,
    _holat: PhantomData<Holat>,
}

impl HttpSorov<Yaratilgan> {
    fn get(url: &str) -> Self {
        println!("[HTTP] GET {} yaratildi", url);
        HttpSorov { metod: "GET".into(), url: url.to_string(), sarlavhalar: vec![], tana: None, javob: None, _holat: PhantomData }
    }

    fn post(url: &str) -> Self {
        println!("[HTTP] POST {} yaratildi", url);
        HttpSorov { metod: "POST".into(), url: url.to_string(), sarlavhalar: vec![], tana: None, javob: None, _holat: PhantomData }
    }

    fn sarlavha_qosh(mut self, kalit: &str, qiymat: &str) -> HttpSorov<SarlavhalarQoshilgan> {
        self.sarlavhalar.push((kalit.to_string(), qiymat.to_string()));
        println!("[HTTP] Sarlavha: {}: {}", kalit, qiymat);
        HttpSorov { metod: self.metod, url: self.url, sarlavhalar: self.sarlavhalar, tana: None, javob: None, _holat: PhantomData }
    }
}

impl HttpSorov<SarlavhalarQoshilgan> {
    fn sarlavha(mut self, kalit: &str, qiymat: &str) -> Self {
        self.sarlavhalar.push((kalit.to_string(), qiymat.to_string()));
        println!("[HTTP] Sarlavha: {}: {}", kalit, qiymat);
        self
    }

    fn tana(self, tana: &[u8]) -> HttpSorov<TanaQoshilgan> {
        println!("[HTTP] Tana: {} bayt", tana.len());
        HttpSorov { metod: self.metod, url: self.url, sarlavhalar: self.sarlavhalar, tana: Some(tana.to_vec()), javob: None, _holat: PhantomData }
    }

    fn yuborish(self) -> HttpSorov<Yuborilgan> {
        println!("[HTTP] {} {} yuborildi ({} sarlavha)", self.metod, self.url, self.sarlavhalar.len());
        HttpSorov { metod: self.metod, url: self.url, sarlavhalar: self.sarlavhalar, tana: self.tana, javob: None, _holat: PhantomData }
    }
}

impl HttpSorov<TanaQoshilgan> {
    fn yuborish(self) -> HttpSorov<Yuborilgan> {
        let tana_uzunlik = self.tana.as_ref().map(|t| t.len()).unwrap_or(0);
        println!("[HTTP] {} {} yuborildi (tana={} bayt)", self.metod, self.url, tana_uzunlik);
        HttpSorov { metod: self.metod, url: self.url, sarlavhalar: self.sarlavhalar, tana: self.tana, javob: None, _holat: PhantomData }
    }
}

impl HttpSorov<Yuborilgan> {
    fn javob_kutish(self) -> HttpSorov<HttpOlingan> {
        let javob_tana = b"{ \"status\": \"ok\" }".to_vec();
        println!("[HTTP] Javob: 200 OK ({} bayt)", javob_tana.len());
        HttpSorov { metod: self.metod, url: self.url, sarlavhalar: self.sarlavhalar, tana: self.tana, javob: Some((200, javob_tana)), _holat: PhantomData }
    }
}

impl HttpSorov<HttpOlingan> {
    fn status_kodi(&self) -> u16 { self.javob.as_ref().map(|(s, _)| *s).unwrap_or(0) }
    fn tana_matn(&self) -> String {
        self.javob.as_ref()
            .map(|(_, t)| String::from_utf8_lossy(t).to_string())
            .unwrap_or_default()
    }
    fn muvaffaqiyatli(&self) -> bool { self.status_kodi() < 400 }
}

fn http_sorov_misoli() {

    println!("\n=== HTTP SO'ROV TYPESTATE ===\n");

    // GET so'rov
    let javob = HttpSorov::get("https://api.example.com/users")
        .sarlavha_qosh("Authorization", "Bearer token123")
        .sarlavha("Accept", "application/json")
        .yuborish()
        .javob_kutish();

    println!("Status: {}, Muvaffaqiyat: {}", javob.status_kodi(), javob.muvaffaqiyatli());
    println!("Tana: {}", javob.tana_matn());
    println!();

    // POST so'rov
    let javob2 = HttpSorov::post("https://api.example.com/users")
        .sarlavha_qosh("Content-Type", "application/json")
        .tana(b"{\"ism\":\"Dilshod\",\"yosh\":22}")
        .yuborish()
        .javob_kutish();

    println!("POST natija: {}", javob2.tana_matn());
    println!("HTTP so'rov holat kafolati ✅");
}

fn main() {

    fayl_typestate_misoli();
    tcp_typestate_misoli();
    tranzaksiya_typestate_misoli();
    svetofor_misoli();
    http_sorov_misoli();

    println!("\n=== XULOSA ===");
    println!("TypeState Pattern:");
    println!("  struct S<Holat> {{ _: PhantomData<Holat> }}");
    println!("  impl S<A> {{ fn o_tish(self) -> S<B> }}");
    println!();
    println!("Misollar:");
    println!("  Fayl:    Yopilgan → OqishUchun / YozishUchun");
    println!("  TCP:     Bgoshlash → Tinglash → UlanishQabul");
    println!("  TX:      Boshlangich → Faol → Commit/Rollback");
    println!("  Svetofor: Qizil → Sariq → Yashil → Qizil");
    println!("  HTTP:    Yaratilgan → Sarlavha → Tana → Yuborilgan");
    println!();
    println!("Zero-cost: PhantomData size = 0 bayt ✅");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                     | Tavsif (UZ)                                | Описание (RU)                                          |
// #================================================================================================================================================#
// # |                                        TYPESTATE ASOSLARI                                                                                    |
// #================================================================================================================================================#
// # |   1 | struct A; struct B;              | Holat marker turlari                       | Маркерные типы состояний                               |
// # |   2 | struct S<H> { _: PhantomData<H> }| TypeState struct                           | TypeState структура                                    |
// # |   3 | impl S<A> { fn metod }           | Faqat A holatda metod                      | Метод только в состоянии A                             |
// # |   4 | fn o_tish(self) -> S<B>          | A holatdan B holatga o'tish                | Переход из состояния A в B                             |
// # |   5 | PhantomData size = 0             | Zero-cost — runtime overhead yo'q          | Zero-cost — нет накладных расходов                     |
// #================================================================================================================================================#
// # |                                        PATTERNLAR                                                                                            |
// #================================================================================================================================================#
// # |   6 | Fayl: Yopilgan → Ochilgan       | Fayl hayot davri                           | Жизненный цикл файла                                    |
// # |   7 | TCP: Closed→Listen→Established  | Protokol holatlari                         | Состояния протокола                                     |
// # |   8 | TX: Begin→Active→Commit         | Tranzaksiya holatlari                      | Состояния транзакции                                    |
// # |   9 | Svetofor: Q→S→Y→Q               | Aylanma holat                              | Циклическое состояние                                   |
// # |  10 | HTTP: Yaratilgan→Yuborilgan     | So'rov hayot davri                         | Жизненный цикл запроса                                  |
// #================================================================================================================================================#
// # |                                        AFZALLIKLAR                                                                                           |
// #================================================================================================================================================#
// # |  11 | Noto'g'ri holat imkonsiz        | Compile-time kafolat                       | Гарантия во время компиляции                            |
// # |  12 | Faqat to'g'ri metodlar          | API xavfsizligi                            | Безопасность API                                        |
// # |  13 | Runtime overhead yo'q           | Zero-cost abstraktsiya                     | Zero-cost абстракция                                    |
// # |  14 | Aniq xato xabarlari             | Debug oson                                 | Лёгкая отладка                                          |
// #================================================================================================================================================#