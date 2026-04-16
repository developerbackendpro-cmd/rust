// #================================================================================================================================================#
// #                                                                PRODUCT  |  SUM                                                                 #
// #                                SUM — ITERATOR ELEMENTLARI YIG'INDISI. PRODUCT — ITERATOR ELEMENTLARI KO'PAYTMASI.                              #
// #                                SUM — СУММА ЭЛЕМЕНТОВ ИТЕРАТОРА. PRODUCT — ПРОИЗВЕДЕНИЕ ЭЛЕМЕНТОВ ИТЕРАТОРА.                                    #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::iter::{Sum, Product};
use std::ops::{Add, Mul};

// Sum trait:
//   trait Sum<A = Self>: Sized {
//       fn sum<I: Iterator<Item = A>>(iter: I) -> Self;
//   }
//
// Product trait:
//   trait Product<A = Self>: Sized {
//       fn product<I: Iterator<Item = A>>(iter: I) -> Self;
//   }
//
// Ikkalasi fold() ning maxsus holati:
// Оба являются частным случаем fold():
//   .sum()     ≡ .fold(0, |acc, x| acc + x)
//   .product() ≡ .fold(1, |acc, x| acc * x)

fn builtin_sum_misollari() {

    // i32 yig'indisi
    // сумма i32
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let yig_indi: i32 = v.iter().sum();
    println!("{}", yig_indi);
    // 15

    // into_iter() bilan — &i32 emas, i32
    // с into_iter() — не &i32, а i32
    let v2: Vec<i32> = vec![10, 20, 30, 40, 50];
    let yig_indi2: i32 = v2.into_iter().sum();
    println!("{}", yig_indi2);
    // 150

    // f64 yig'indisi
    // сумма f64
    let floatlar: Vec<f64> = vec![1.1, 2.2, 3.3, 4.4];
    let float_yig: f64 = floatlar.iter().sum();
    println!("{:.1}", float_yig);
    // 11.0

    // Range yig'indisi
    // сумма Range
    let range_yig: i32 = (1..=100).sum();
    println!("{}", range_yig);
    // 5050

    // map + sum — transformatsiya va yig'ish
    // map + sum — преобразование и суммирование
    let kvadratlar_yig: i32 = (1..=5).map(|x| x * x).sum();
    println!("{}", kvadratlar_yig);
    // 55  (1 + 4 + 9 + 16 + 25)

    // filter + sum — shartli yig'ish
    // filter + sum — условное суммирование
    let juft_yig: i32 = (1..=10).filter(|x| x % 2 == 0).sum();
    println!("{}", juft_yig);
    // 30  (2 + 4 + 6 + 8 + 10)

    // u64 — katta sonlar uchun
    // u64 — для больших чисел
    let katta_yig: u64 = (1u64..=1000).sum();
    println!("{}", katta_yig);
    // 500500

    // usize — indekslar yig'indisi
    // usize — сумма индексов
    let usize_yig: usize = vec![1usize, 2, 3, 4, 5].iter().sum();
    println!("{}", usize_yig);
    // 15
}

fn builtin_product_misollari() {

    // i32 ko'paytmasi
    // произведение i32
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let ko_paytma: i32 = v.iter().product();
    println!("{}", ko_paytma);
    // 120

    // faktorial — product bilan
    // факториал — через product
    let faktorial_5: u64 = (1u64..=5).product();
    let faktorial_10: u64 = (1u64..=10).product();
    println!("5! = {}", faktorial_5);
    println!("10! = {}", faktorial_10);
    // 5! = 120
    // 10! = 3628800

    // f64 ko'paytmasi
    // произведение f64
    let floatlar: Vec<f64> = vec![1.5, 2.0, 3.0];
    let float_ko_p: f64 = floatlar.iter().product();
    println!("{}", float_ko_p);
    // 9.0

    // filter + product
    // filter + product
    let toq_ko_p: i32 = (1..=7).filter(|x| x % 2 != 0).product();
    println!("{}", toq_ko_p);
    // 105  (1 * 3 * 5 * 7)

    // map + product — har elementni o'zgartirib ko'paytirish
    // map + product — преобразование и умножение
    let ikki_darajasi: i32 = (0..=4).map(|x: i32| 2i32.pow(x as u32)).product();
    println!("{}", ikki_darajasi);
    // 1024  (1 * 2 * 4 * 8 * 16)

    // into_iter() bilan product
    // product с into_iter()
    let v2: Vec<i32> = vec![2, 3, 4, 5];
    let natija: i32 = v2.into_iter().product();
    println!("{}", natija);
    // 120

    // u64 — katta ko'paytma
    // u64 — большое произведение
    let u64_ko_p: u64 = vec![2u64, 3, 5, 7, 11].into_iter().product();
    println!("{}", u64_ko_p);
    // 2310
}

// Custom struct — Sum implement qilish
// Custom структура — реализация Sum
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vektor2D {
    x: f64,
    y: f64,
}

impl Vektor2D {
    fn new(x: f64, y: f64) -> Self {
        Vektor2D { x, y }
    }

    fn uzunlik(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl std::fmt::Display for Vektor2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

// Add trait — qo'shish uchun
// Add trait — для сложения
impl Add for Vektor2D {
    type Output = Vektor2D;
    fn add(self, other: Vektor2D) -> Vektor2D {
        Vektor2D::new(self.x + other.x, self.y + other.y)
    }
}

// Sum — &Vektor2D uchun
// Sum — для &Vektor2D
impl<'a> Sum<&'a Vektor2D> for Vektor2D {
    fn sum<I: Iterator<Item = &'a Vektor2D>>(iter: I) -> Vektor2D {
        iter.fold(Vektor2D::new(0.0, 0.0), |acc, v| acc + *v)
    }
}

// Sum — Vektor2D (owned) uchun
// Sum — для Vektor2D (owned)
impl Sum for Vektor2D {
    fn sum<I: Iterator<Item = Vektor2D>>(iter: I) -> Vektor2D {
        iter.fold(Vektor2D::new(0.0, 0.0), |acc, v| acc + v)
    }
}

fn custom_sum_misollari() {

    // Vektor2D yig'indisi — reference orqali
    // Сумма Vektor2D — через ссылку
    let vektorlar: Vec<Vektor2D> = vec![
        Vektor2D::new(1.0, 2.0),
        Vektor2D::new(3.0, 4.0),
        Vektor2D::new(5.0, 6.0),
    ];
    let yig_indi: Vektor2D = vektorlar.iter().sum();
    println!("{}", yig_indi);
    // (9.0, 12.0)

    // Vektor2D yig'indisi — ownership orqali
    // Сумма Vektor2D — через владение
    let yig_indi2: Vektor2D = vektorlar.into_iter().sum();
    println!("{}", yig_indi2);
    // (9.0, 12.0)

    // map + sum — vektorlarni o'zgartirib yig'ish
    // map + sum — преобразование и суммирование векторов
    let skalar: f64 = 2.0;
    let vektorlar2: Vec<Vektor2D> = vec![
        Vektor2D::new(1.0, 0.0),
        Vektor2D::new(0.0, 1.0),
        Vektor2D::new(1.0, 1.0),
    ];
    let kattaytirilgan: Vektor2D = vektorlar2.iter()
        .map(|v| Vektor2D::new(v.x * skalar, v.y * skalar))
        .sum();
    println!("{}", kattaytirilgan);
    // (4.0, 4.0)
}

// Matritsa — Product implement qilish
// Матрица — реализация Product
#[derive(Debug, Clone, Copy)]
struct Matritsa2x2 {
    a: f64, b: f64,
    c: f64, d: f64,
}

impl Matritsa2x2 {
    fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Matritsa2x2 { a, b, c, d }
    }

    // Birlik matritsa — ko'paytma uchun neytral element
    // Единичная матрица — нейтральный элемент для умножения
    fn birlik() -> Self {
        Matritsa2x2::new(1.0, 0.0, 0.0, 1.0)
    }

    fn determinant(&self) -> f64 {
        self.a * self.d - self.b * self.c
    }
}

impl std::fmt::Display for Matritsa2x2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{:.1} {:.1} | {:.1} {:.1}]",
               self.a, self.b, self.c, self.d)
    }
}

impl Mul for Matritsa2x2 {
    type Output = Matritsa2x2;
    fn mul(self, other: Matritsa2x2) -> Matritsa2x2 {
        Matritsa2x2::new(
            self.a * other.a + self.b * other.c,
            self.a * other.b + self.b * other.d,
            self.c * other.a + self.d * other.c,
            self.c * other.b + self.d * other.d,
        )
    }
}

// Product — Matritsa2x2 uchun
// Product — для Matritsa2x2
impl Product for Matritsa2x2 {
    fn product<I: Iterator<Item = Matritsa2x2>>(iter: I) -> Matritsa2x2 {
        iter.fold(Matritsa2x2::birlik(), |acc, m| acc * m)
    }
}

impl<'a> Product<&'a Matritsa2x2> for Matritsa2x2 {
    fn product<I: Iterator<Item = &'a Matritsa2x2>>(iter: I) -> Matritsa2x2 {
        iter.fold(Matritsa2x2::birlik(), |acc, m| acc * *m)
    }
}

fn custom_product_misollari() {

    // Matritsalar ko'paytmasi zanjiri
    // Цепочка произведения матриц
    let m1 = Matritsa2x2::new(1.0, 2.0, 3.0, 4.0);
    let m2 = Matritsa2x2::new(5.0, 6.0, 7.0, 8.0);
    let m3 = Matritsa2x2::new(1.0, 0.0, 0.0, 1.0); // birlik

    let matritsalar: Vec<Matritsa2x2> = vec![m1, m2, m3];
    let ko_paytma: Matritsa2x2 = matritsalar.iter().product();
    println!("{}", ko_paytma);
    // [19.0 22.0 | 43.0 50.0]

    // Determinantlar ko'paytmasi
    // Произведение детерминантов
    let det_ko_p: f64 = matritsalar.iter()
        .map(|m| m.determinant())
        .product();
    println!("{:.1}", det_ko_p);
    // -2.0
}

#[derive(Debug, Clone)]
struct Buyurtma {
    mahsulot: String,
    narx: f64,
    soni: u32,
}

impl Buyurtma {
    fn new(mahsulot: &str, narx: f64, soni: u32) -> Self {
        Buyurtma { mahsulot: mahsulot.to_string(), narx, soni }
    }

    fn jami(&self) -> f64 {
        self.narx * self.soni as f64
    }
}

fn real_hayot_misollari() {

    let buyurtmalar: Vec<Buyurtma> = vec![
        Buyurtma::new("Olma",    1500.0, 5),
        Buyurtma::new("Non",      800.0, 3),
        Buyurtma::new("Sut",     2000.0, 2),
        Buyurtma::new("Banan",   3000.0, 4),
        Buyurtma::new("Pishloq", 8000.0, 1),
    ];

    // 1. Jami summa — map + sum
    // 1. Общая сумма — map + sum
    let jami_summa: f64 = buyurtmalar.iter()
        .map(|b| b.jami())
        .sum();
    println!("Jami summa: {:.0} so'm", jami_summa);
    // Jami summa: 37900 so'm

    // 2. Jami mahsulot soni — sum
    // 2. Общее количество — sum
    let jami_soni: u32 = buyurtmalar.iter()
        .map(|b| b.soni)
        .sum();
    println!("Jami soni: {}", jami_soni);
    // Jami soni: 15

    // 3. O'rtacha narx — sum / count
    // 3. Средняя цена — sum / count
    let narxlar_yig: f64 = buyurtmalar.iter().map(|b| b.narx).sum();
    let o_rtacha_narx: f64 = narxlar_yig / buyurtmalar.len() as f64;
    println!("O'rtacha narx: {:.0} so'm", o_rtacha_narx);
    // O'rtacha narx: 3060 so'm

    // 4. Eng qimmat mahsulotlar yig'indisi — filter + sum
    // 4. Сумма дорогих товаров — filter + sum
    let qimmat_yig: f64 = buyurtmalar.iter()
        .filter(|b| b.narx >= 2000.0)
        .map(|b| b.jami())
        .sum();
    println!("Qimmat mahsulotlar: {:.0} so'm", qimmat_yig);
    // Qimmat mahsulotlar: 28000 so'm

    // 5. Kombinatsiya ehtimoli — product
    // 5. Вероятность комбинации — product
    let ehtimollar: Vec<f64> = vec![0.9, 0.8, 0.95, 0.85];
    let umumiy_ehtimol: f64 = ehtimollar.iter().product();
    println!("Umumiy ehtimol: {:.4}", umumiy_ehtimol);
    // Umumiy ehtimol: 0.5814

    // 6. N! / K! hisoblash — product
    // 6. Вычисление N! / K! — product
    fn kombinatsiya(n: u64, k: u64) -> u64 {
        let numerator: u64 = ((k + 1)..=n).product();
        let denominator: u64 = (1..=(n - k)).product();
        numerator / denominator
    }
    println!("C(10, 3) = {}", kombinatsiya(10, 3));
    println!("C(6, 2) = {}", kombinatsiya(6, 2));
    // C(10, 3) = 120
    // C(6, 2) = 15

    // 7. Geometrik o'rtacha — product + pow
    // 7. Геометрическое среднее — product + pow
    let sonlar: Vec<f64> = vec![2.0, 8.0, 32.0, 128.0];
    let ko_paytma: f64 = sonlar.iter().product();
    let geometrik_ortacha: f64 = ko_paytma.powf(1.0 / sonlar.len() as f64);
    println!("Geometrik o'rtacha: {:.1}", geometrik_ortacha);
    // Geometrik o'rtacha: 16.0

    // 8. Matritsa tizimida sum va product
    // 8. Sum и product в системе матриц
    let o_lchov_birliklari: Vec<f64> = vec![100.0, 0.01, 1000.0];
    let aylantirish_koeffitsienti: f64 = o_lchov_birliklari.iter().product();
    println!("Aylantirish: {:.0}", aylantirish_koeffitsienti);
    // Aylantirish: 1000

    // 9. Weighted sum — og'irlikli yig'indi
    // 9. Взвешенная сумма — weighted sum
    let baholar: Vec<f64> = vec![85.0, 90.0, 78.0, 92.0, 88.0];
    let og_irliklar: Vec<f64> = vec![0.2, 0.25, 0.15, 0.25, 0.15];
    let og_irlikli_yig: f64 = baholar.iter()
        .zip(og_irliklar.iter())
        .map(|(b, o)| b * o)
        .sum();
    println!("Og'irlikli o'rtacha: {:.1}", og_irlikli_yig);
    // Og'irlikli o'rtacha: 87.3
}

fn main() {

    println!("=== BUILT-IN SUM ===");
    builtin_sum_misollari();

    println!("\n=== BUILT-IN PRODUCT ===");
    builtin_product_misollari();

    println!("\n=== CUSTOM SUM ===");
    custom_sum_misollari();

    println!("\n=== CUSTOM PRODUCT ===");
    custom_product_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                    | Описание (RU)                                             |
// #================================================================================================================================================#
// # |                                       SUM TRAIT                                                                                              |
// #================================================================================================================================================#
// # |   1 | .sum::<T>()               | Iterator elementlari yig'indisi                | Сумма элементов итератора                                 |
// # |   2 | impl Sum for T            | Custom sum — neytral element 0                 | Custom sum — нейтральный элемент 0                        |
// # |   3 | impl Sum<&'a T> for T     | Reference orqali sum                           | Сумма через ссылку                                        |
// # |   4 | map + sum                 | Transformatsiya + yig'ish                      | Преобразование + суммирование                             |
// # |   5 | filter + sum              | Shartli yig'ish                                | Условное суммирование                                     |
// # |   6 | fold(0, |acc, x| acc + x) | sum() ekvivalenti                              | Эквивалент sum()                                          |
// #================================================================================================================================================#
// # |                                       PRODUCT TRAIT                                                                                          |
// #================================================================================================================================================#
// # |   7 | .product::<T>()           | Iterator elementlari ko'paytmasi               | Произведение элементов итератора                          |
// # |   8 | impl Product for T        | Custom product — neytral element 1             | Custom product — нейтральный элемент 1                    |
// # |   9 | impl Product<&'a T> for T | Reference orqali product                       | Произведение через ссылку                                 |
// # |  10 | (1..=n).product()         | Faktorial hisoblash                            | Вычисление факториала                                     |
// # |  11 | fold(1, |acc, x| acc * x) | product() ekvivalenti                          | Эквивалент product()                                      |
// #================================================================================================================================================#
// # |                                       REAL HAYOT                                                                                             |
// #================================================================================================================================================#
// # |  12 | map + sum                 | Jami narx hisoblash                            | Вычисление итоговой суммы                                 |
// # |  13 | zip + map + sum           | Weighted sum (og'irlikli)                      | Взвешенная сумма                                          |
// # |  14 | (k+1..=n).product()       | Kombinatsiya va permutatsiya                   | Комбинации и перестановки                                 |
// # |  15 | iter().product()          | Ehtimollar ko'paytmasi                         | Произведение вероятностей                                 |
// # |  16 | powf(1.0/n)               | Geometrik o'rtacha — product bilan             | Геометрическое среднее — через product                    |
// #================================================================================================================================================#