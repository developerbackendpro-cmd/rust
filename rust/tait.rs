// #================================================================================================================================================#
// #                                                                    TAIT                                                                        #
// #                            TAIT — TYPE ALIAS IMPL TRAIT. EXISTENTIAL TYPE. IMPL TRAIT IN TYPE ALIAS. OPAQUE TYPE.                              #
// #                            TAIT — ПСЕВДОНИМ ТИПА IMPL TRAIT. ЭКЗИСТЕНЦИАЛЬНЫЙ ТИП. OPAQUE TYPE.                                                #
// #================================================================================================================================================#

// MUHIM: TAIT (type alias impl Trait) hali NIGHTLY faqat!
// Stable Rust da: impl Trait funksiya qaytarishida ishlatiladi.
// Bu fayl stable Rust bilan ishlovchi kod + TAIT tushuntirishni beradi.

#![allow(dead_code, unused)]

use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// TAIT nima:
// Что такое TAIT:
//
//   type MyIter = impl Iterator<Item = i32>;  ← NIGHTLY
//   fn f() -> impl Trait;                      ← STABLE (RPIT)
//
//   Stable da mavjud:
//   В stable доступно:
//   ✅ fn f() -> impl Trait           (RPIT - Return Position Impl Trait)
//   ✅ fn f(x: impl Trait)            (APIT - Argument Position Impl Trait)
//   ✅ type Iter = std::iter::Filter<...> (konkret tur alias)
//   ❌ type T = impl Trait            (TAIT - nightly only!)
//
//   Stabilizatsiya yo'li:
//   Путь к стабилизации:
//   RFC 2071 → Rust nightly → stable (yaqinda)
//   Tracking issue: https://github.com/rust-lang/rust/issues/63063

// RPIT — fn qaytarishida impl Trait (stable, Rust 1.26+)
// RPIT — impl Trait в позиции возврата (stable, Rust 1.26+)

fn juftlar_yaratish(max: i32) -> impl Iterator<Item = i32> {
    (0..max).filter(|x| x % 2 == 0)
}

fn muqobil_yaratish(n: i32) -> impl Iterator<Item = i32> + Clone {
    (0..n).map(|x| if x % 2 == 0 { x } else { -x })
}

fn saralash_va_filter<'a>(matnlar: &'a [&'a str], kalit: &'a str)
                          -> impl Iterator<Item = &'a str> + 'a
{
    matnlar.iter().copied().filter(move |s| s.contains(kalit))
}

fn transform_qil(v: Vec<i32>) -> impl Iterator<Item = String> {
    v.into_iter().filter(|&x| x > 0).map(|x| format!("#{:04}", x))
}

fn rpit_misoli() {

    println!("=== RPIT (Stable impl Trait) ===\n");

    // Juft sonlar
    let v: Vec<i32> = juftlar_yaratish(20).collect();
    println!("Juftlar: {:?}", v);
    // [0, 2, 4, 6, 8, 10, 12, 14, 16, 18]

    // Clone bilan
    let iter = muqobil_yaratish(8);
    let klon = iter.clone();
    println!("Muqobil: {:?}", iter.collect::<Vec<_>>());
    println!("Klon:    {:?}", klon.collect::<Vec<_>>());
    // [0, -1, 2, -3, 4, -5, 6, -7]

    // Lifetime bilan
    let matnlar = ["rust", "python", "rustacean", "golang", "rusted"];
    let v2: Vec<&str> = saralash_va_filter(&matnlar, "rust").collect();
    println!("'rust' bor: {:?}", v2);
    // ["rust", "rustacean", "rusted"]

    // Transform
    let v3: Vec<String> = transform_qil(vec![-5, 3, 0, 7, -2, 11, 4]).collect();
    println!("Transform: {:?}", v3);
    // ["#0003", "#0007", "#0011", "#0004"]
}

fn tait_nightly_sintaksis() {

    println!("\n=== TAIT SINTAKSIS (Nightly) ===\n");

    println!(r#"// Nightly da:
// #![feature(type_alias_impl_trait)]

type JuftlarIter = impl Iterator<Item = i32>;
type MuqobilIter = impl Iterator<Item = i32> + Clone;

#[define_opaque(JuftlarIter)]
fn juftlar_yaratish(max: i32) -> JuftlarIter {{
    (0..max).filter(|x| x % 2 == 0)
}}

#[define_opaque(MuqobilIter)]
fn muqobil_yaratish(n: i32) -> MuqobilIter {{
    (0..n).map(|x| if x % 2 == 0 {{ x }} else {{ -x }})
}}

// Afzalligi: bir xil nom bilan qayta ishlatish
fn ikki_iter_birlashtir(a: JuftlarIter, b: JuftlarIter) -> impl Iterator<Item = i32> {{
    a.chain(b)  // Ikkisi ham bir xil tur — JuftlarIter
}}"#);
}

// Stable: trait da associated type + Box<dyn> yoki konkret tur
trait Hisoblash {
    type Natija: fmt::Debug + fmt::Display;

    fn hisoblash(&self) -> Self::Natija;
    fn barcha_vec(&self, n: usize) -> Vec<Self::Natija>;

    // RPIT — stable (Rust 1.75+ RPITIT: Return Position impl Trait in Trait)
    fn barcha_iter(&self, n: usize) -> impl Iterator<Item = Self::Natija> {
        self.barcha_vec(n).into_iter()
    }
}

struct FibHisob { boshlanish: u64 }
struct KvadratHisob { boshlanish: u32 }

impl Hisoblash for FibHisob {
    type Natija = u64;

    fn hisoblash(&self) -> u64 {
        let (mut a, mut b) = (0u64, 1u64);
        for _ in 0..self.boshlanish { (a, b) = (b, a + b); }
        a
    }

    fn barcha_vec(&self, n: usize) -> Vec<u64> {
        let mut a = 0u64;
        let mut b = 1u64;
        (0..n).map(|_| { let v = a; (a, b) = (b, a + b); v }).collect()
    }
}

impl Hisoblash for KvadratHisob {
    type Natija = u32;

    fn hisoblash(&self) -> u32 {
        self.boshlanish * self.boshlanish
    }

    fn barcha_vec(&self, n: usize) -> Vec<u32> {
        let s = self.boshlanish;
        (s..s + n as u32).map(|x| x * x).collect()
    }
}

fn trait_impl_misoli() {

    println!("\n=== TRAIT + IMPL TRAIT (Stable) ===");

    let fib = FibHisob { boshlanish: 10 };
    println!("fib(10) = {}", fib.hisoblash());
    let fib_v: Vec<u64> = fib.barcha_iter(8).collect();
    println!("fib barcha(8): {:?}", fib_v);
    // fib(10) = 55
    // fib barcha(8): [0, 1, 1, 2, 3, 5, 8, 13]

    let kv = KvadratHisob { boshlanish: 5 };
    println!("kv(5) = {}", kv.hisoblash());
    let kv_v: Vec<u32> = kv.barcha_iter(5).collect();
    println!("kv barcha(5): {:?}", kv_v);
    // kv(5) = 25
    // kv barcha(5): [25, 36, 49, 64, 81]
}

async fn async_bajarish(n: u32) -> Result<String, String> {
    if n == 0 {
        return Err("Nol emas bo'lishi kerak!".to_string());
    }
    let natija: u64 = (1..=n as u64).product();
    Ok(format!("{}! = {}", n, natija))
}

async fn data_yuklash(bufer: &mut Vec<u8>, ma_lumot: &[u8]) -> Vec<u8> {
    bufer.clear();
    bufer.extend_from_slice(ma_lumot);
    bufer.clone()
}

fn noop_waker() -> std::task::Waker {
    use std::task::{RawWaker, RawWakerVTable, Waker};
    unsafe fn clone(_: *const ()) -> RawWaker { RawWaker::new(std::ptr::null(), &VT) }
    unsafe fn noop(_: *const ()) {}
    static VT: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VT)) }
}

fn poll_fut<F: Future>(mut f: Pin<Box<F>>) -> F::Output {
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    loop {
        match f.as_mut().poll(&mut cx) {
            Poll::Ready(v)  => return v,
            Poll::Pending   => std::hint::spin_loop(),
        }
    }
}

fn async_misoli() {

    println!("\n=== ASYNC FN + impl Future (Stable) ===");

    let r1 = poll_fut(Box::pin(async_bajarish(5)));
    let r2 = poll_fut(Box::pin(async_bajarish(0)));
    println!("{:?}", r1); // Ok("5! = 120")
    println!("{:?}", r2); // Err("Nol emas bo'lishi kerak!")

    let mut bufer = Vec::new();
    let data = poll_fut(Box::pin(data_yuklash(&mut bufer, b"Hello impl Trait!")));
    println!("Yuklandi: {}", std::str::from_utf8(&data).unwrap());
    // Ok("5! = 120")
    // Err("Nol emas bo'lishi kerak!")
    // Yuklandi: Hello impl Trait!
}

fn impl_vs_dyn_taqqoslash() {

    println!("\n=== IMPL TRAIT vs BOX<DYN> ===\n");

    // impl Trait — static dispatch, zero-cost
    fn impl_iter(v: Vec<i32>) -> impl Iterator<Item = i32> {
        v.into_iter().filter(|x| x % 2 == 0)
    }

    // Box<dyn> — dynamic dispatch, heap alloc
    fn dyn_iter(v: Vec<i32>) -> Box<dyn Iterator<Item = i32>> {
        Box::new(v.into_iter().filter(|x| x % 2 == 0))
    }

    let v1 = vec![1,2,3,4,5,6];
    let v2 = v1.clone();
    let iv: Vec<i32> = impl_iter(v1).collect();
    let dv: Vec<i32> = dyn_iter(v2).collect();
    println!("impl: {:?}", iv);
    println!("dyn:  {:?}", dv);

    println!();
    println!("┌─────────────────┬───────────┬────────────────┐");
    println!("│ Xususiyat       │ impl Trait│  Box<dyn Trait>│");
    println!("├─────────────────┼───────────┼────────────────┤");
    println!("│ Dispatch        │ Static    │  Dynamic       │");
    println!("│ Heap alloc      │ Yo'q      │  Ha            │");
    println!("│ Nom berish      │ Cheklangan│  Ha            │");
    println!("│ Qayta ishlatish │ Qiyin     │  Oson          │");
    println!("│ Turli funcsiya  │ Har biri  │  Bir tur       │");
    println!("│ dyn safe        │ Shart emas│  Kerak         │");
    println!("└─────────────────┴───────────┴────────────────┘");
    // impl: [2, 4, 6]
    // dyn:  [2, 4, 6]
}

// Middleware — impl Trait bilan
trait Middleware {
    type Chiqish: fmt::Debug;
    fn bajar(&self, kirish: String) -> Self::Chiqish;
}

struct LogMiddleware;
struct TransformMiddleware;
struct AuthMiddleware { token: String }

impl Middleware for LogMiddleware {
    type Chiqish = String;
    fn bajar(&self, kirish: String) -> String {
        println!("[LOG] {}", kirish);
        kirish
    }
}

impl Middleware for TransformMiddleware {
    type Chiqish = Vec<String>;
    fn bajar(&self, kirish: String) -> Vec<String> {
        kirish.split_whitespace().map(|s| s.to_uppercase()).collect()
    }
}

impl Middleware for AuthMiddleware {
    type Chiqish = Result<String, String>;
    fn bajar(&self, kirish: String) -> Result<String, String> {
        if kirish.contains(&self.token) {
            Ok(kirish.replace(&self.token, "[AUTH]"))
        } else {
            Err(format!("Token yo'q: '{}'", kirish))
        }
    }
}

// Pipeline — Box<dyn Fn> bilan (stable, TAIT alternativ)
fn pipeline_qur(prefiks: &str, sufiks: &str) -> Box<dyn Fn(String) -> String> {
    let p = prefiks.to_string();
    let s = sufiks.to_string();
    Box::new(move |kirish: String| {
        format!("{} | {} | {}", p, kirish.to_uppercase(), s)
    })
}

// Lazy fibonacci — impl Iterator bilan (stable)
fn fibonacci_lazy() -> impl Iterator<Item = u64> + Clone {
    let mut a = 0u64;
    let mut b = 1u64;
    std::iter::from_fn(move || {
        let v = a;
        (a, b) = (b, a.saturating_add(b));
        Some(v)
    })
}

fn real_hayot_misollari() {

    println!("\n=== REAL HAYOT ===\n");

    // Middleware
    let log = LogMiddleware;
    let auth = AuthMiddleware { token: "secret".to_string() };
    let transform = TransformMiddleware;

    let kirish = "GET /api secret token".to_string();
    let log_natija = log.bajar(kirish.clone());
    let auth_natija = auth.bajar(kirish.clone());
    let transform_natija = transform.bajar(kirish.clone());

    println!("Log: {}", log_natija);
    println!("Auth: {:?}", auth_natija);
    println!("Transform: {:?}", transform_natija);
    // [LOG] GET /api secret token
    // Auth: Ok("GET /api [AUTH] token")
    // Transform: ["GET", "/API", "SECRET", "TOKEN"]

    // Pipeline
    println!();
    let pipeline = pipeline_qur(">>", "<<");
    println!("Pipeline 1: {}", pipeline("salom".to_string()));
    println!("Pipeline 2: {}", pipeline("rust tili".to_string()));
    // Pipeline 1: >> | SALOM | <<
    // Pipeline 2: >> | RUST TILI | <<

    // Lazy fibonacci
    println!();
    let fib = fibonacci_lazy();
    let fib2 = fib.clone();
    let birinchi_10: Vec<u64> = fib.take(10).collect();
    let keyingi_5: Vec<u64> = fib2.skip(10).take(5).collect();
    println!("Birinchi 10: {:?}", birinchi_10);
    println!("Keyingi 5:   {:?}", keyingi_5);
    // Birinchi 10: [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
    // Keyingi 5:   [55, 89, 144, 233, 377]
}

fn tait_kelajak() {

    println!("\n=== TAIT KELAJAK ===\n");

    println!("Hozir stable da (Rust 1.75+):");
    println!("  ✅ fn f() -> impl Trait         (RPIT)");
    println!("  ✅ fn f(x: impl Trait)           (APIT)");
    println!("  ✅ async fn f() -> T             (AFIT)");
    println!("  ✅ fn f() -> impl Trait in trait (RPITIT)");
    println!();
    println!("Nightly (yaqinda stable bo'ladi):");
    println!("  🔄 type T = impl Trait           (TAIT)");
    println!("  🔄 #[define_opaque(T)]           (TAIT attribute)");
    println!("  🔄 type T<'a> = impl Trait + 'a  (TAIT + lifetime)");
    println!();
    println!("TAIT stable bo'lganda afzalliklari:");
    println!("  → Murakkab iter zanjiri type alias");
    println!("  → Module darajasida public opaque type");
    println!("  → Trait da type = impl ... (ATPIT)");
    println!("  → Bir tur sifatida qayta ishlatish");
    println!();
    println!("Kuzatish: https://github.com/rust-lang/rust/issues/63063");
}

fn main() {

    rpit_misoli();
    tait_nightly_sintaksis();
    trait_impl_misoli();
    async_misoli();
    impl_vs_dyn_taqqoslash();
    real_hayot_misollari();
    tait_kelajak();

    println!("\n=== XULOSA ===");
    println!("Stable da impl Trait:");
    println!("  fn f() -> impl Iterator  — RPIT ✅");
    println!("  fn f() -> impl Fn(...)   — closure ✅");
    println!("  async fn f() -> T        — async ✅");
    println!("  fn f(x: impl Trait)      — argument ✅");
    println!();
    println!("Nightly da:");
    println!("  type T = impl Trait      — TAIT 🔄");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        STABLE (RPIT)                                                                                         |
// #================================================================================================================================================#
// # |   1 | fn f() -> impl Trait            | Return position impl trait (1.26+)         | Impl trait в позиции возврата (1.26+)                   |
// # |   2 | fn f(x: impl Trait)             | Argument position impl trait               | Impl trait в позиции аргумента                          |
// # |   3 | -> impl Trait + Clone           | Ko'p bound bilan                           | С несколькими bounds                                    |
// # |   4 | -> impl Trait + 'a              | Lifetime bilan                             | С lifetime                                              |
// # |   5 | async fn f() -> T               | Async funksiya (1.39+)                     | Async функция (1.39+)                                   |
// # |   6 | fn f() -> impl Trait in trait   | RPITIT (1.75+)                             | RPITIT (1.75+)                                          |
// #================================================================================================================================================#
// # |                                        NIGHTLY (TAIT)                                                                                        |
// #================================================================================================================================================#
// # |   7 | type T = impl Trait             | Type alias impl trait (nightly)            | Псевдоним типа impl trait (nightly)                     |
// # |   8 | #[define_opaque(T)]             | TAIT ni belgilash (nightly)                | Определение TAIT (nightly)                              |
// # |   9 | type T<'a> = impl Trait + 'a    | Lifetime bilan TAIT (nightly)              | TAIT с lifetime (nightly)                               |
// #================================================================================================================================================#
// # |                                        ALTERNATIVLAR                                                                                         |
// #================================================================================================================================================#
// # |  10 | Box<dyn Fn(T) -> U>             | Pipeline: stable alternativ                | Pipeline: стабильная альтернатива                       |
// # |  11 | type Alias = ConcreteType       | Konkret tur alias (stable)                 | Псевдоним конкретного типа (stable)                     |
// # |  12 | struct Wrapper(impl Trait)      | Newtype wrapper                            | Обёртка newtype                                         |
// #================================================================================================================================================#