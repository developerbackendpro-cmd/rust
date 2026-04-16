// #================================================================================================================================================#
// #                                                        CONST FN  |  CONST GENERICS                                                             #
// #                        CONST FN — KOMPILYATSIYA VAQTIDA HISOBLASH. CONST GENERICS — TURDA KONSTANT PARAMETR.                                   #
// #                        CONST FN — ВЫЧИСЛЕНИЕ ВО ВРЕМЯ КОМПИЛЯЦИИ. CONST GENERICS — КОНСТАНТНЫЙ ПАРАМЕТР В ТИ                                   #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// const fn nima:
// Что такое const fn:
//
//   - Kompilyatsiya vaqtida chaqirilishi mumkin bo'lgan funksiya
//   - Функция которую можно вызвать во время компиляции
//   - Runtime da ham ishlaydi — cheklov yo'q
//   - Работает и в runtime — нет ограничений
//   - const blok, const item, array uzunligi uchun ishlatiladi
//   - Используется в const block, const item, длина массива
//
// const generics nima:
// Что такое const generics:
//
//   - Tur parametri sifatida konstant ishlatish
//   - Использование константы как параметра типа
//   - [T; N] — N const generic
//   - Compile-time ma'lum o'lchamdagi strukturalar
//   - Структуры с размером известным во время компиляции

// const fn — kompilyatsiya vaqtida ham ishlaydi
// const fn — работает и во время компиляции
const fn qo_shish(a: i32, b: i32) -> i32 {
    a + b
}

const fn faktorial(n: u64) -> u64 {
    if n == 0 { 1 } else { n * faktorial(n - 1) }
}

const fn maksimum(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

const fn min_qiymat(v: &[i32]) -> i32 {
    let mut min: i32 = i32::MAX;
    let mut i: usize = 0;
    while i < v.len() {
        if v[i] < min { min = v[i]; }
        i += 1;
    }
    min
}

// const — kompilyatsiya vaqtida hisoblangan
// const — вычислено во время компиляции
const BESH_FAKTORIAL: u64 = faktorial(5);       // 120
const O_N_FAKTORIAL: u64 = faktorial(10);        // 3628800
const MAX_QIYMAT: i32 = maksimum(42, 100);       // 100
const QO_SHMA: i32 = qo_shish(1000, 2000);      // 3000

// Array o'lchami — const fn bilan
// Размер массива — с const fn
const fn buffer_hajmi(kattalashtiruvchi: usize) -> usize {
    1024 * kattalashtiruvchi
}
const BUFFER: [u8; buffer_hajmi(4)] = [0; buffer_hajmi(4)];

fn const_fn_misollari() {

    println!("5! = {}", BESH_FAKTORIAL);
    println!("10! = {}", O_N_FAKTORIAL);
    println!("max(42,100) = {}", MAX_QIYMAT);
    println!("1000+2000 = {}", QO_SHMA);
    println!("Buffer: {} bayt", BUFFER.len());
    // 5! = 120
    // 10! = 3628800
    // max(42,100) = 100
    // 1000+2000 = 3000
    // Buffer: 4096 bayt

    // const fn — runtime da ham ishlaydi
    // const fn — работает и в runtime
    let n: u64 = 7;
    println!("{}! = {}", n, faktorial(n));
    // 7! = 5040

    // const blok
    // const блок
    let x: i32 = const { qo_shish(10, 20) };
    println!("{}", x);
    // 30

    // const fn — murakkab hisoblash
    // const fn — сложное вычисление
    const FIB_20: u64 = {
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        let mut i: u64 = 0;
        while i < 20 {
            let c = a + b;
            a = b;
            b = c;
            i += 1;
        }
        a
    };
    println!("fib(20) = {}", FIB_20);
    // fib(20) = 6765
}

// [T; N] — eng mashhur const generic
// [T; N] — наиболее известный const generic
// N — kompile vaqtida ma'lum

// Custom const generic struct
// Пользовательская структура с const generic
#[derive(Debug, Clone, Copy)]
struct Matritsa<T, const SATR: usize, const USTUN: usize> {
    ma_lumot: [[T; USTUN]; SATR],
}

impl<T, const S: usize, const U: usize> Matritsa<T, S, U>
where
    T: Default + Copy,
{
    fn yangi() -> Self {
        Matritsa { ma_lumot: [[T::default(); U]; S] }
    }

    fn o_lcham(&self) -> (usize, usize) { (S, U) }
    fn satr_soni(&self) -> usize { S }
    fn ustun_soni(&self) -> usize { U }
}

impl<T, const S: usize, const U: usize> fmt::Display for Matritsa<T, S, U>
where
    T: fmt::Display + Default + Copy,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for satr in &self.ma_lumot {
            for (i, val) in satr.iter().enumerate() {
                if i > 0 { write!(f, " ")?; }
                write!(f, "{:4}", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// Const generic — Stack
// Стек с const generic
struct ConstStek<T, const N: usize> {
    ma_lumot: [Option<T>; N],
    uzunlik: usize,
}

impl<T: Copy + Default, const N: usize> ConstStek<T, N> {
    fn new() -> Self {
        ConstStek {
            ma_lumot: [None; N],
            uzunlik: 0,
        }
    }

    fn push(&mut self, val: T) -> bool {
        if self.uzunlik >= N { return false; }
        self.ma_lumot[self.uzunlik] = Some(val);
        self.uzunlik += 1;
        true
    }

    fn pop(&mut self) -> Option<T> {
        if self.uzunlik == 0 { return None; }
        self.uzunlik -= 1;
        self.ma_lumot[self.uzunlik].take()
    }

    fn peek(&self) -> Option<&T> {
        if self.uzunlik == 0 { return None; }
        self.ma_lumot[self.uzunlik - 1].as_ref()
    }

    fn to_lib(&self) -> bool { self.uzunlik == N }
    fn boshmi(&self) -> bool { self.uzunlik == 0 }
    fn uzunlik(&self) -> usize { self.uzunlik }
    fn sig_im(&self) -> usize { N }
}

// Const generic — funksiyada
// Const generic — в функции
fn birinchi_n<T: Copy, const N: usize>(slice: &[T]) -> [T; N]
where
    T: Default,
{
    let mut natija: [T; N] = [T::default(); N];
    let ko_chirish: usize = N.min(slice.len());
    natija[..ko_chirish].copy_from_slice(&slice[..ko_chirish]);
    natija
}

fn const_generics_misollari() {

    // Matritsa — const generics bilan
    // Матрица — с const generics
    let mut m: Matritsa<i32, 3, 3> = Matritsa::yangi();
    m.ma_lumot[0] = [1, 2, 3];
    m.ma_lumot[1] = [4, 5, 6];
    m.ma_lumot[2] = [7, 8, 9];

    println!("{}", m);
    println!("O'lcham: {:?}", m.o_lcham());
    // 1  2  3
    // 4  5  6
    // 7  8  9
    // O'lcham: (3, 3)

    // Tur tekshiruvi — kompilyatsiya vaqtida
    // Проверка типов — во время компиляции
    let m2: Matritsa<f64, 2, 4> = Matritsa::yangi();
    println!("{}x{}", m2.satr_soni(), m2.ustun_soni());
    // 2x4

    // ConstStek — compile-time sig'im
    // ConstStek — ёмкость во время компиляции
    let mut stek: ConstStek<i32, 5> = ConstStek::new();
    stek.push(1);
    stek.push(2);
    stek.push(3);

    println!("Uzunlik: {}", stek.uzunlik());
    println!("Sig'im: {}", stek.sig_im());
    println!("Tepada: {:?}", stek.peek());
    // Uzunlik: 3
    // Sig'im: 5
    // Tepada: Some(3)

    while let Some(v) = stek.pop() {
        print!("{} ", v);
    }
    println!();
    // 3 2 1

    // birinchi_n — const generic funksiya
    // birinchi_n — функция с const generic
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let uch: [i32; 3] = birinchi_n(&v);
    let besh: [i32; 5] = birinchi_n(&v);
    println!("{:?}", uch);
    println!("{:?}", besh);
    // [1, 2, 3]
    // [1, 2, 3, 4, 5]

    // Tur farqi — kompilyatsiya vaqtida
    // Разница типов — во время компиляции
    println!("ConstStek<i32,3>: {} bayt", std::mem::size_of::<ConstStek<i32, 3>>());
    println!("ConstStek<i32,8>: {} bayt", std::mem::size_of::<ConstStek<i32, 8>>());
    // ConstStek<i32,3>: XX bayt
    // ConstStek<i32,8>: XX bayt (kattaroq)
}

// Hash — const generic bilan
// Хэш — с const generic
struct FixedHash<const N: usize> {
    baytlar: [u8; N],
}

impl<const N: usize> FixedHash<N> {
    fn yangi(ma_lumot: &[u8]) -> Self {
        let mut baytlar: [u8; N] = [0; N];
        // Oddiy hash simulyatsiyasi
        // Простая симуляция хэша
        for (i, &b) in ma_lumot.iter().enumerate() {
            baytlar[i % N] ^= b.wrapping_add((i as u8).wrapping_mul(31));
        }
        FixedHash { baytlar }
    }
}

impl<const N: usize> fmt::Display for FixedHash<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for b in &self.baytlar {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

// Vektor — const generic bilan fizik hisoblash
// Вектор — физические вычисления с const generic
#[derive(Debug, Clone, Copy)]
struct Vektor<const N: usize> {
    komponentlar: [f64; N],
}

impl<const N: usize> Vektor<N> {
    fn yangi(komponentlar: [f64; N]) -> Self {
        Vektor { komponentlar }
    }

    fn uzunlik(&self) -> f64 {
        self.komponentlar.iter().map(|&x| x * x).sum::<f64>().sqrt()
    }

    fn skalyar_ko_paytma(&self, b: &Vektor<N>) -> f64 {
        self.komponentlar.iter().zip(b.komponentlar.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }
}

impl<const N: usize> std::ops::Add for Vektor<N> {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        let mut natija: [f64; N] = [0.0; N];
        for i in 0..N {
            natija[i] = self.komponentlar[i] + b.komponentlar[i];
        }
        Vektor::yangi(natija)
    }
}

fn real_hayot_misollari() {

    // FixedHash
    let h1: FixedHash<16> = FixedHash::yangi(b"salom dunyo");
    let h2: FixedHash<32> = FixedHash::yangi(b"rust generics");
    println!("Hash-128: {}", h1);
    println!("Hash-256: {}", h2);

    // Vektor fizik hisoblash
    let v1: Vektor<3> = Vektor::yangi([1.0, 2.0, 3.0]);
    let v2: Vektor<3> = Vektor::yangi([4.0, 5.0, 6.0]);
    let yig: Vektor<3> = v1 + v2;

    println!("v1 uzunlik: {:.2}", v1.uzunlik());
    println!("v1·v2: {:.2}", v1.skalyar_ko_paytma(&v2));
    println!("v1+v2: {:?}", yig.komponentlar);
    // v1 uzunlik: 3.74
    // v1·v2: 32.00
    // v1+v2: [5.0, 7.0, 9.0]

    // 2D va 3D vektorlar — tur farqi kompilyatsiya vaqtida
    // 2D и 3D векторы — разница типов во время компиляции
    let v2d: Vektor<2> = Vektor::yangi([3.0, 4.0]);
    let v3d: Vektor<3> = Vektor::yangi([1.0, 0.0, 0.0]);
    println!("2D uzunlik: {:.1}", v2d.uzunlik()); // 5.0
    println!("3D uzunlik: {:.1}", v3d.uzunlik()); // 1.0
    // v2d + v3d — kompile bo'lmaydi! Turli N!
    // v2d + v3d — не скомпилируется! Разные N!
}

fn main() {

    println!("=== CONST FN ===");
    const_fn_misollari();

    println!("\n=== CONST GENERICS ===");
    const_generics_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        CONST FN                                                                                              |
// #================================================================================================================================================#
// # |   1 | const fn f(x: T) -> U           | Kompilyatsiya vaqtida chaqirilishi mumkin  | Можно вызвать во время компиляции                       |
// # |   2 | const X: T = f(args)            | Kompilyatsiya vaqtida hisoblangan          | Вычислено во время компиляции                           |
// # |   3 | const { expr }                  | Const blok                                 | Const блок                                              |
// # |   4 | [T; const_fn()]                 | Array uzunligi const fn bilan              | Длина массива через const fn                            |
// # |   5 | if, while, match                | const fn da ruxsat etilgan                 | Разрешено в const fn                                    |
// #================================================================================================================================================#
// # |                                        CONST GENERICS                                                                                        |
// #================================================================================================================================================#
// # |   6 | struct S<const N: usize>        | Const generic parametr                     | Параметр const generic                                  |
// # |   7 | fn f<const N: usize>(...)       | Funksiyada const generic                   | Const generic в функции                                 |
// # |   8 | [T; N]                          | Eng ko'p ishlatiladigan const generic      | Наиболее часто используемый                             |
// # |   9 | Compile-time o'lcham            | Heap allocation yo'q                       | Нет выделения кучи                                      |
// # |  10 | Tur xavfsizligi                 | Turli N — turli tur, mos kelmaydi          | Разные N — разные типы, несовместимы                    |
// #================================================================================================================================================#