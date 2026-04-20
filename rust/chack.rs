// #================================================================================================================================================#
// #                                                             COMPILE-TIME CHECKS                                                                #
// #                    COMPILE-TIME — CONST FN, CONST GENERICS, STATIC ASSERT, TYPE SYSTEM KAFOLATLARI. ZERO RUNTIME XARAJAT.                      #
// #                    COMPILE-TIME — CONST FN, CONST GENERICS, STATIC ASSERT, ГАРАНТИИ СИСТЕМЫ ТИПОВ. НОЛЬ RUNTIME ЗАТРАТ.                        #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::marker::PhantomData;
use std::fmt;

// Compile-time checks nima:
// Что такое Compile-time checks:
//
//   Runtime da tekshirish o'rniga — kompilyatsiya vaqtida
//   Вместо проверки в runtime — во время компиляции
//
//   Usullari:
//   Способы:
//   1. const fn / const eval — compile-time hisoblash
//      const fn / const eval — вычисление во время компиляции
//   2. const generics — o'lcham va qiymat parametrlari
//      const generics — параметры размера и значения
//   3. assert! / static_assert — kafolatlash
//      assert! / static_assert — утверждения
//   4. Type system — tur tizimi kafolatlari
//      Система типов — гарантии системы типов
//   5. Trait bounds — cheklovlar orqali xavfsizlik
//      Trait bounds — безопасность через ограничения
//   6. PhantomData — tur darajasida invariantlar
//      PhantomData — инварианты на уровне типов

const fn faktorial(n: u64) -> u64 {
    match n { 0 | 1 => 1, _ => n * faktorial(n - 1) }
}

const fn fibonacci(n: u32) -> u64 {
    match n { 0 => 0, 1 => 1, _ => fibonacci(n - 1) + fibonacci(n - 2) }
}

const fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3u64;
    while i * i <= n {
        if n % i == 0 { return false; }
        i += 2;
    }
    true
}

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

const fn pow_const(base: u64, exp: u32) -> u64 {
    match exp { 0 => 1, _ => base * pow_const(base, exp - 1) }
}

// Compile-time konstantalar
const FAKT_10:  u64 = faktorial(10);   // 3628800
const FAKT_15:  u64 = faktorial(15);   // 1307674368000
const FIB_20:   u64 = fibonacci(20);   // 6765
const FIB_25:   u64 = fibonacci(25);   // 75025
const POW_2_16: u64 = pow_const(2, 16); // 65536
const GCD_48_18: u64 = gcd(48, 18);    // 6

// Const assert — compile time kafolat
// Const assert — гарантия во время компиляции
const _: () = assert!(faktorial(5) == 120, "5! 120 bo'lishi kerak");
const _: () = assert!(fibonacci(10) == 55, "fib(10) 55 bo'lishi kerak");
const _: () = assert!(is_prime(17), "17 tub son bo'lishi kerak");
const _: () = assert!(!is_prime(15), "15 tub son emas");
const _: () = assert!(gcd(12, 8) == 4, "gcd(12,8) = 4");

fn const_fn_misoli() {

    println!("--- Const Fn ---");
    println!("10! = {} (compile time)", FAKT_10);
    println!("15! = {} (compile time)", FAKT_15);
    println!("fib(20) = {} (compile time)", FIB_20);
    println!("fib(25) = {} (compile time)", FIB_25);
    println!("2^16 = {} (compile time)", POW_2_16);
    println!("gcd(48,18) = {} (compile time)", GCD_48_18);
    // 10! = 3628800
    // 15! = 1307674368000
    // fib(20) = 6765
    // fib(30) = 832040
    // 2^16 = 65536
    // gcd(48,18) = 6

    // Tub sonlar jadvali — compile time
    const TUBS: [u64; 10] = {
        let mut arr = [0u64; 10];
        let mut hisob = 0;
        let mut n = 2u64;
        while hisob < 10 {
            if is_prime(n) { arr[hisob] = n; hisob += 1; }
            n += 1;
        }
        arr
    };
    println!("Birinchi 10 tub son: {:?}", TUBS);
    // [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]

    // const blok — runtime da ham ishlaydi
    let n = 7u64;
    let fakt_n = faktorial(n); // runtime da
    println!("{}! = {}", n, fakt_n); // 5040
    // 7! = 5040

    // Lookup table — compile time da hisoblangan
    const KVADRATLAR: [u32; 16] = {
        let mut arr = [0u32; 16];
        let mut i = 0;
        while i < 16 { arr[i] = (i * i) as u32; i += 1; }
        arr
    };
    println!("Kvadratlar[0..8]: {:?}", &KVADRATLAR[..8]);
    // [0, 1, 4, 9, 16, 25, 36, 49]
}

// Massiv o'lchami — compile time da kafolat
// Размер массива — гарантия во время компиляции
struct MatriksNxM<const N: usize, const M: usize> {
    ma_lumot: [[f64; M]; N],
}

impl<const N: usize, const M: usize> MatriksNxM<N, M> {
    fn new() -> Self {
        MatriksNxM { ma_lumot: [[0.0; M]; N] }
    }

    fn qiymat_o_rnat(&mut self, i: usize, j: usize, val: f64) {
        assert!(i < N && j < M, "Indeks chegaradan tashqari!");
        self.ma_lumot[i][j] = val;
    }

    fn ol(&self, i: usize, j: usize) -> f64 { self.ma_lumot[i][j] }

    fn satrlar(&self) -> usize { N }
    fn ustunlar(&self) -> usize { M }

    fn transpozitsiya(&self) -> MatriksNxM<M, N> {
        let mut natija = MatriksNxM::<M, N>::new();
        for i in 0..N {
            for j in 0..M {
                natija.ma_lumot[j][i] = self.ma_lumot[i][j];
            }
        }
        natija
    }
}

// Ko'paytirish — faqat mos o'lchamlar
// Умножение — только совместимые размеры
impl<const N: usize, const M: usize> MatriksNxM<N, M> {
    fn ko_paytir<const K: usize>(&self, b: &MatriksNxM<M, K>) -> MatriksNxM<N, K> {
        let mut natija = MatriksNxM::<N, K>::new();
        for i in 0..N {
            for j in 0..K {
                let mut yig = 0.0;
                for k in 0..M { yig += self.ma_lumot[i][k] * b.ma_lumot[k][j]; }
                natija.ma_lumot[i][j] = yig;
            }
        }
        natija
    }
}

impl<const N: usize, const M: usize> fmt::Display for MatriksNxM<N, M> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..N {
            write!(f, "[")?;
            for j in 0..M {
                if j > 0 { write!(f, ", ")?; }
                write!(f, "{:6.2}", self.ma_lumot[i][j])?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

// FixedStack — compile time kapasitet
// FixedStack — ёмкость во время компиляции
struct FixedStack<T: Copy, const N: usize> {
    ma_lumot: [Option<T>; N],
    uch: usize,
}

impl<T: Copy + fmt::Debug, const N: usize> FixedStack<T, N> {
    const fn new() -> Self {
        FixedStack { ma_lumot: [None; N], uch: 0 }
    }

    fn push(&mut self, val: T) -> Result<(), &'static str> {
        if self.uch >= N { return Err("Stack to'la!"); }
        self.ma_lumot[self.uch] = Some(val);
        self.uch += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.uch == 0 { return None; }
        self.uch -= 1;
        self.ma_lumot[self.uch].take()
    }

    fn peek(&self) -> Option<T> {
        if self.uch == 0 { None } else { self.ma_lumot[self.uch - 1] }
    }

    fn uzunlik(&self) -> usize { self.uch }
    fn sig_im(&self) -> usize { N }
    fn boshmi(&self) -> bool { self.uch == 0 }
    fn to_la(&self) -> bool { self.uch >= N }
}

fn const_generics_misoli() {

    println!("\n--- Const Generics ---");

    // Matriks
    let mut a: MatriksNxM<2, 3> = MatriksNxM::new();
    a.qiymat_o_rnat(0, 0, 1.0); a.qiymat_o_rnat(0, 1, 2.0); a.qiymat_o_rnat(0, 2, 3.0);
    a.qiymat_o_rnat(1, 0, 4.0); a.qiymat_o_rnat(1, 1, 5.0); a.qiymat_o_rnat(1, 2, 6.0);

    println!("A (2x3):\n{}", a);
    println!("Transpozitsiya (3x2):\n{}", a.transpozitsiya());

    // Ko'paytirish: (2x3) * (3x2) = (2x2)
    let mut b: MatriksNxM<3, 2> = MatriksNxM::new();
    b.qiymat_o_rnat(0, 0, 7.0); b.qiymat_o_rnat(0, 1, 8.0);
    b.qiymat_o_rnat(1, 0, 9.0); b.qiymat_o_rnat(1, 1, 10.0);
    b.qiymat_o_rnat(2, 0, 11.0); b.qiymat_o_rnat(2, 1, 12.0);

    let c = a.ko_paytir(&b); // (2x3) * (3x2) = (2x2)
    println!("A*B (2x2):\n{}", c);

    // Bu KOMPILE BO'LMAYDI — o'lcham mos emas:
    // Это НЕ СКОМПИЛИРУЕТСЯ — размеры не совпадают:
    // let xato = b.ko_paytir(&a); // (3x2) * (2x3) ← XATO!

    // FixedStack
    println!("--- FixedStack<i32, 4> ---");
    let mut stack: FixedStack<i32, 4> = FixedStack::new();
    println!("Sig'im: {}", stack.sig_im()); // 4

    stack.push(10).unwrap();
    stack.push(20).unwrap();
    stack.push(30).unwrap();
    stack.push(40).unwrap();
    println!("Push 50: {:?}", stack.push(50)); // Err("Stack to'la!")
    println!("Peek: {:?}", stack.peek()); // Some(40)
    println!("Pop: {:?}", stack.pop());   // Some(40)
    println!("Pop: {:?}", stack.pop());   // Some(30)
    // Sig'im: 4
    // Push 50: Err("Stack to'la!")
    // Peek: Some(40)
    // Pop: Some(40)
    // Pop: Some(30)
}

// O'lcham kafolatlari
const _: () = assert!(std::mem::size_of::<u8>() == 1);
const _: () = assert!(std::mem::size_of::<u32>() == 4);
const _: () = assert!(std::mem::size_of::<u64>() == 8);
const _: () = assert!(std::mem::size_of::<usize>() >= 4);

// Alignment kafolatlari
const _: () = assert!(std::mem::align_of::<u32>() == 4);
const _: () = assert!(std::mem::align_of::<u64>() == 8);

// Konstantalar kafolati
const MAX_BUFSIZE: usize = 1024;
const MIN_BUFSIZE: usize = 16;
const _: () = assert!(MAX_BUFSIZE > MIN_BUFSIZE, "Max > Min bo'lishi kerak");
const _: () = assert!(MAX_BUFSIZE.is_power_of_two(), "2 darajasi bo'lishi kerak");

// Macro — static assert
macro_rules! static_assert {
    ($shart:expr) => {
        const _: () = assert!($shart, concat!("Static assert muvaffaqiyatsiz: ", stringify!($shart)));
    };
    ($shart:expr, $xabar:literal) => {
        const _: () = assert!($shart, $xabar);
    };
}

static_assert!(4 + 4 == 8);
static_assert!(std::mem::size_of::<i32>() == 4, "i32 4 bayt bo'lishi kerak");
static_assert!(u8::MAX == 255);

// Struct o'lcham kafolati
macro_rules! assert_size {
    ($tur:ty, $hajm:expr) => {
        const _: [(); $hajm] = [(); std::mem::size_of::<$tur>()];
    };
}

#[repr(C)]
struct PacketHeader { version: u8, flags: u8, length: u16, checksum: u32 }
assert_size!(PacketHeader, 8); // Aniq 8 bayt bo'lishi kerak

fn static_assert_misoli() {

    println!("\n--- Static Assert ---");
    println!("Barcha static assertlar kompilyatsiya vaqtida tekshirildi ✅");
    println!("u8={}b, u32={}b, u64={}b",
             std::mem::size_of::<u8>(),
             std::mem::size_of::<u32>(),
             std::mem::size_of::<u64>());
    println!("PacketHeader: {}b", std::mem::size_of::<PacketHeader>());
    println!("MAX_BUFSIZE={}, MIN_BUFSIZE={}", MAX_BUFSIZE, MIN_BUFSIZE);
}

// NonZero — nol bo'lmasligi compile-time kafolat
// NonZero — гарантия ненулевого значения во время компиляции
use std::num::{NonZeroU32, NonZeroUsize};

fn bo_lish(a: u32, b: NonZeroU32) -> u32 {
    a / b.get() // 0 ga bo'linish IMKONSIZ!
}

// Kafolatlangan diapazon — PhantomData bilan
// Гарантированный диапазон — с PhantomData
#[derive(Debug)]
struct Foiz(f64);

impl Foiz {
    fn new(val: f64) -> Result<Self, String> {
        if (0.0..=100.0).contains(&val) {
            Ok(Foiz(val))
        } else {
            Err(format!("{} — yaroqli foiz emas (0-100)", val))
        }
    }
    fn qiymat(&self) -> f64 { self.0 }
}

// Holat mashinasi — compile-time
// Машина состояний — во время компиляции
struct Boshlangich;
struct Ishlamoqda;
struct Tugagan;

struct Vazifa<Holat> {
    id: u32,
    nomi: String,
    _h: PhantomData<Holat>,
}

impl Vazifa<Boshlangich> {
    fn new(id: u32, nomi: &str) -> Self {
        Vazifa { id, nomi: nomi.to_string(), _h: PhantomData }
    }
    fn boshlash(self) -> Vazifa<Ishlamoqda> {
        println!("[Vazifa #{}] '{}' boshlandi", self.id, self.nomi);
        Vazifa { id: self.id, nomi: self.nomi, _h: PhantomData }
    }
}

impl Vazifa<Ishlamoqda> {
    fn tugatish(self) -> Vazifa<Tugagan> {
        println!("[Vazifa #{}] '{}' tugadi", self.id, self.nomi);
        Vazifa { id: self.id, nomi: self.nomi, _h: PhantomData }
    }
    fn hisoblash(&self, val: i32) -> i32 {
        println!("[Vazifa #{}] hisoblash: {}", self.id, val);
        val * val
    }
}

impl Vazifa<Tugagan> {
    fn natija(&self) -> String {
        format!("#{} '{}' muvaffaqiyatli tugadi", self.id, self.nomi)
    }
}

fn tur_tizimi_misoli() {

    println!("\n--- Tur Tizimi Kafolatlari ---");

    // NonZero
    let a = 100u32;
    let b = NonZeroU32::new(7).unwrap();
    println!("100 / 7 = {}", bo_lish(a, b));
    // NonZeroU32::new(0) → None, bo'linish imkonsiz!

    // Foiz
    println!("\n{:?}", Foiz::new(75.0).map(|f| f.qiymat())); // Ok(75.0)
    println!("{:?}", Foiz::new(101.0));    // Err(...)
    println!("{:?}", Foiz::new(-5.0));     // Err(...)

    // Holat mashinasi
    println!("\n--- Holat Mashinasi ---");
    let v = Vazifa::<Boshlangich>::new(1, "DB migratsiya");
    // v.hisoblash(5); // ← KOMPILE XATO! Boshlangich da yo'q
    let v = v.boshlash();
    let natija = v.hisoblash(9);
    println!("Natija: {}", natija);
    let v = v.tugatish();
    // v.hisoblash(5); // ← KOMPILE XATO! Tugagan da yo'q
    println!("{}", v.natija());

    // [Vazifa #1] 'DB migratsiya' boshlandi
    // [Vazifa #1] hisoblash: 9
    // Natija: 81
    // [Vazifa #1] 'DB migratsiya' tugadi
    // #1 'DB migratsiya' muvaffaqiyatli tugadi
}

// Faqat ma'lum shartlarda funksiya ishlaydi
// Функция работает только при определённых условиях

// Faqat Send + Sync turlar uchun
fn thread_xavfsiz_ishlov<T: Send + Sync + fmt::Debug>(val: T) -> String {
    format!("{:?}", val)
}

// Faqat Clone + PartialOrd turlar uchun
fn saralangan_juftlik<T: Clone + PartialOrd>(a: T, b: T) -> (T, T) {
    if a <= b { (a, b) } else { (b, a) }
}

// Faqat Default + Clone turlar uchun
fn n_ta_default<T: Default + Clone>(n: usize) -> Vec<T> {
    vec![T::default(); n]
}

// Compile-time turni tekshirish
// Проверка типа во время компиляции
fn faqat_primitiv<T: Copy + 'static>(_: T) {}

// Where clause bilan murakkab cheklov
fn serialize_qil<T>(val: &T) -> String
where
    T: fmt::Debug + fmt::Display + Clone + PartialEq,
{
    format!("debug={:?} display={}", val, val)
}

fn trait_bounds_misoli() {

    println!("\n--- Trait Bounds ---");

    // Send + Sync
    let s = thread_xavfsiz_ishlov(vec![1, 2, 3]);
    println!("{}", s); // [1, 2, 3]

    // Clone + PartialOrd
    let (kichik, katta) = saralangan_juftlik(30, 10);
    println!("({}, {})", kichik, katta); // (10, 30)

    let (a, b) = saralangan_juftlik("z", "a");
    println!("({}, {})", a, b); // (a, z)

    // Default + Clone
    let v: Vec<i32> = n_ta_default(5);
    println!("{:?}", v); // [0, 0, 0, 0, 0]

    let v2: Vec<String> = n_ta_default(3);
    println!("{:?}", v2); // ["", "", ""]

    // Serialize
    println!("{}", serialize_qil(&42i32));
    println!("{}", serialize_qil(&"salom"));
    // debug=42 display=42
    // debug="salom" display=salom
}

// Xavfsiz matematik — compile-time kafolat
fn xavfsiz_matematik_misoli() {

    println!("\n--- Xavfsiz Matematik ---");

    // Overflow tekshiruvi — compile time
    const MAX: u32 = u32::MAX;
    let n = MAX.checked_add(1);
    println!("u32::MAX + 1 = {:?}", n); // None

    let a: u32 = 1_000_000;
    let b: u32 = 1_000_000;
    match a.checked_mul(b) {
        Some(v) => println!("{} * {} = {}", a, b, v),
        None    => println!("Overflow!"),
    }
    // Overflow!

    // Saturating
    println!("Saturating: {}", u8::MAX.saturating_add(1)); // 255
    println!("Wrapping:   {}", u8::MAX.wrapping_add(1));   // 0
    // Saturating: 255
    // Wrapping:   0

    // Const generics bilan xavfsiz bufer
    fn xavfsiz_nusxa<const N: usize>(src: &[u8; N], dst: &mut [u8; N]) {
        dst.copy_from_slice(src);
    }

    let src: [u8; 4] = [1, 2, 3, 4];
    let mut dst: [u8; 4] = [0; 4];
    xavfsiz_nusxa(&src, &mut dst);
    println!("Nusxa: {:?}", dst); // [1, 2, 3, 4]
    // Turli o'lcham — KOMPILE XATO!
    // let src5: [u8; 5] = [1,2,3,4,5];
    // xavfsiz_nusxa(&src5, &mut dst); // ← XATO!
    // [1, 2, 3, 4]
}

// Lookup table + compile-time hash
fn lookup_table_misoli() {

    println!("\n--- Lookup Table ---");

    // Sinüs jadval — compile time hisoblangan
    const TRIG_TABLE_SIZE: usize = 8;
    const SIN_TABLE: [f64; TRIG_TABLE_SIZE] = {
        let mut arr = [0.0f64; TRIG_TABLE_SIZE];
        // const context da float ops limited, manualimpl
        arr[0] = 0.0;
        arr[1] = 0.382683;
        arr[2] = 0.707107;
        arr[3] = 0.923880;
        arr[4] = 1.0;
        arr[5] = 0.923880;
        arr[6] = 0.707107;
        arr[7] = 0.382683;
        arr
    };

    println!("SIN jadvali (0..π):");
    for (i, &v) in SIN_TABLE.iter().enumerate() {
        println!("  sin({}/8 * π) ≈ {:.6}", i, v);
    }

    // CRC jadval — compile time
    const fn crc8_jadval() -> [u8; 256] {
        let mut tbl = [0u8; 256];
        let mut i = 0usize;
        while i < 256 {
            let mut crc = i as u8;
            let mut j = 0;
            while j < 8 {
                if crc & 0x80 != 0 { crc = (crc << 1) ^ 0x07; }
                else { crc <<= 1; }
                j += 1;
            }
            tbl[i] = crc;
            i += 1;
        }
        tbl
    }

    const CRC8_TABLE: [u8; 256] = crc8_jadval();

    fn crc8_hisoblash(data: &[u8]) -> u8 {
        data.iter().fold(0u8, |crc, &b| CRC8_TABLE[(crc ^ b) as usize])
    }

    let ma_lumot = b"Salom Rust!";
    println!("\nCRC8 ('Salom Rust!'): 0x{:02X}", crc8_hisoblash(ma_lumot));
    let ma_lumot2 = b"hello";
    println!("CRC8 ('hello'):       0x{:02X}", crc8_hisoblash(ma_lumot2));
}

fn main() {

    println!("=== CONST FN ===");
    const_fn_misoli();

    println!("\n=== CONST GENERICS ===");
    const_generics_misoli();

    println!("\n=== STATIC ASSERT ===");
    static_assert_misoli();

    println!("\n=== TUR TIZIMI ===");
    tur_tizimi_misoli();

    println!("\n=== TRAIT BOUNDS ===");
    trait_bounds_misoli();

    println!("\n=== XAVFSIZ MATEMATIK ===");
    xavfsiz_matematik_misoli();

    println!("\n=== LOOKUP TABLE ===");
    lookup_table_misoli();

    println!("\n=== XULOSA ===");
    println!("Compile-time checks:");
    println!("  const fn    → hisoblash compile vaqtida");
    println!("  const {}  → o'lcham parametri", "{N}");
    println!("  assert!     → compile-time kafolat");
    println!("  Tur tizimi  → holat mashinasi, NonZero");
    println!("  Trait bounds→ faqat ma'lum turlar");
    println!("  Lookup table→ runtime hisoblash yo'q");
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                               | Описание (RU)                                            |
// #================================================================================================================================================#
// # |                                        CONST FN                                                                                              |
// #================================================================================================================================================#
// # |   1 | const fn f(n: u64) -> u64       | Compile-time bajariluvchi funksiya         | Функция выполняемая во время компиляции                 |
// # |   2 | const X: T = expr               | Compile-time konstanta                     | Константа времени компиляции                            |
// # |   3 | const {}: () = assert!(...)     | Compile-time kafolat                       | Гарантия времени компиляции                             |
// # |   4 | const TABLO: [T; N] = { ... }   | Compile-time jadval yaratish               | Создание таблицы во время компиляции                    |
// #================================================================================================================================================#
// # |                                        CONST GENERICS                                                                                        |
// #================================================================================================================================================#
// # |   5 | struct S<const N: usize>           | O'lcham parametri                          | Параметр размера                                     |
// # |   6 | fn f<const N: usize>(arr: &[T; N]) | Massiv o'lchami — compile time             | Размер массива — во время компиляции                 |
// # |   7 | MatriksNxM<N,M>                    | Ko'paytirish o'lchami tekshiruvi           | Проверка размеров умножения                          |
// #================================================================================================================================================#
// # |                                        KAFOLATLAR                                                                                            |
// #================================================================================================================================================#
// # |   8 | NonZeroU32                      | Nol bo'lmaslik kafolati                    | Гарантия ненулевого значения                            |
// # |   9 | TypeState                       | Holat mashinasi — compile time             | Машина состояний — во время компиляции                  |
// # |  10 | assert_size!(T, N)              | Struct o'lcham kafolati                    | Гарантия размера структуры                              |
// # |  11 | Trait bounds                    | Faqat ma'lum turlar uchun funksiya         | Функция только для определённых типов                   |
// # |  12 | checked_add/mul                 | Runtime overflow xavfsizligi               | Безопасность от overflow в runtime                      |
// #================================================================================================================================================#