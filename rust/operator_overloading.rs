// #================================================================================================================================================#
// #                                                      OPERATOR OVERLOADING                                                                      #
// #                     OPERATOR OVERLOADING — +, -, *, /, ==, <, [] VA BOSHQA OPERATORLARNI CUSTOM TURLARGA MOSLASH.                              #
// #                     OPERATOR OVERLOADING — НАСТРОЙКА +, -, *, /, ==, <, [] И ДРУГИХ ОПЕРАТОРОВ ДЛЯ CUSTOM ТИПОВ.                               #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::ops::{
    Add, Sub, Mul, Div, Rem, Neg,
    AddAssign, SubAssign, MulAssign, DivAssign,
    BitAnd, BitOr, BitXor, Not, Shl, Shr,
    Index, IndexMut,
};
use std::fmt;
use std::cmp::{PartialEq, PartialOrd, Ordering};

// Operator ↔ Trait jadvali:
// Таблица оператор ↔ Трейт:
//   +    →  Add         -    →  Sub
//   *    →  Mul         /    →  Div
//   %    →  Rem         -x   →  Neg
//   +=   →  AddAssign   -=   →  SubAssign
//   *=   →  MulAssign   /=   →  DivAssign
//   &    →  BitAnd       |    →  BitOr
//   ^    →  BitXor       !    →  Not
//   <<   →  Shl          >>   →  Shr
//   ==   →  PartialEq    !=   →  PartialEq
//   <    →  PartialOrd   >    →  PartialOrd
//   []   →  Index        []=  →  IndexMut
//   *x   →  Deref        *x=  →  DerefMut

#[derive(Debug, Clone, Copy)]
struct Vektor2D {
    x: f64,
    y: f64,
}

impl Vektor2D {
    fn new(x: f64, y: f64) -> Self { Vektor2D { x, y } }
    fn uzunlik(&self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    fn birlik(&self) -> Self {
        let u = self.uzunlik();
        if u == 0.0 { *self } else { Vektor2D::new(self.x / u, self.y / u) }
    }
    fn nuqta_ko_paytma(&self, b: &Vektor2D) -> f64 { self.x * b.x + self.y * b.y }
}

impl fmt::Display for Vektor2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

impl Add for Vektor2D {
    type Output = Vektor2D;
    fn add(self, b: Vektor2D) -> Vektor2D {
        Vektor2D::new(self.x + b.x, self.y + b.y)
    }
}

impl Sub for Vektor2D {
    type Output = Vektor2D;
    fn sub(self, b: Vektor2D) -> Vektor2D {
        Vektor2D::new(self.x - b.x, self.y - b.y)
    }
}

impl Mul<f64> for Vektor2D {
    type Output = Vektor2D;
    fn mul(self, s: f64) -> Vektor2D {
        Vektor2D::new(self.x * s, self.y * s)
    }
}

impl Mul<Vektor2D> for f64 {
    type Output = Vektor2D;
    fn mul(self, v: Vektor2D) -> Vektor2D {
        Vektor2D::new(v.x * self, v.y * self)
    }
}

impl Div<f64> for Vektor2D {
    type Output = Vektor2D;
    fn div(self, s: f64) -> Vektor2D {
        Vektor2D::new(self.x / s, self.y / s)
    }
}

impl Neg for Vektor2D {
    type Output = Vektor2D;
    fn neg(self) -> Vektor2D {
        Vektor2D::new(-self.x, -self.y)
    }
}

impl AddAssign for Vektor2D {
    fn add_assign(&mut self, b: Vektor2D) {
        self.x += b.x;
        self.y += b.y;
    }
}

impl SubAssign for Vektor2D {
    fn sub_assign(&mut self, b: Vektor2D) {
        self.x -= b.x;
        self.y -= b.y;
    }
}

impl PartialEq for Vektor2D {
    fn eq(&self, b: &Vektor2D) -> bool {
        (self.x - b.x).abs() < 1e-10 && (self.y - b.y).abs() < 1e-10
    }
}

fn vektor_misollari() {

    let v1: Vektor2D = Vektor2D::new(1.0, 2.0);
    let v2: Vektor2D = Vektor2D::new(3.0, 4.0);

    // + operatori
    let qo_shma: Vektor2D = v1 + v2;
    println!("v1 + v2 = {}", qo_shma);
    // v1 + v2 = (4.00, 6.00)

    // - operatori
    let ayirma: Vektor2D = v2 - v1;
    println!("v2 - v1 = {}", ayirma);
    // v2 - v1 = (2.00, 2.00)

    // * operatori (skalar bilan)
    let kattaytirilgan: Vektor2D = v1 * 3.0;
    println!("v1 * 3 = {}", kattaytirilgan);
    // v1 * 3 = (3.00, 6.00)

    // f64 * Vektor2D
    let kattaytirilgan2: Vektor2D = 2.0 * v2;
    println!("2 * v2 = {}", kattaytirilgan2);
    // 2 * v2 = (6.00, 8.00)

    // / operatori
    let bo_lingan: Vektor2D = v2 / 2.0;
    println!("v2 / 2 = {}", bo_lingan);
    // v2 / 2 = (1.50, 2.00)

    // - (unary negation)
    let inkor: Vektor2D = -v1;
    println!("-v1 = {}", inkor);
    // -v1 = (-1.00, -2.00)

    // += operatori
    let mut v3: Vektor2D = Vektor2D::new(0.0, 0.0);
    v3 += v1;
    v3 += v2;
    println!("v3 += v1 + v2 = {}", v3);
    // v3 += v1 + v2 = (4.00, 6.00)

    // == operatori
    let v4: Vektor2D = Vektor2D::new(1.0, 2.0);
    println!("v1 == v4: {}", v1 == v4);
    println!("v1 == v2: {}", v1 == v2);
    // v1 == v4: true
    // v1 == v2: false

    // Murakkab ifoda
    let natija: Vektor2D = v1 * 2.0 + v2 - Vektor2D::new(1.0, 1.0);
    println!("v1*2 + v2 - (1,1) = {}", natija);
    // v1*2 + v2 - (1,1) = (4.00, 7.00)

    // Uzunlik va birlik vektori
    println!("|v2| = {:.2}", v2.uzunlik());
    println!("birlik(v2) = {}", v2.birlik());
    println!("v1 · v2 = {}", v1.nuqta_ko_paytma(&v2));
    // |v2| = 5.00
    // birlik(v2) = (0.60, 0.80)
    // v1 · v2 = 11.00
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Kompleks {
    real: f64,
    xayoliy: f64,
}

impl Kompleks {
    fn new(real: f64, xayoliy: f64) -> Self { Kompleks { real, xayoliy } }
    fn modul(&self) -> f64 { (self.real * self.real + self.xayoliy * self.xayoliy).sqrt() }
    fn konjugat(&self) -> Self { Kompleks::new(self.real, -self.xayoliy) }
}

impl fmt::Display for Kompleks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.xayoliy >= 0.0 { write!(f, "{:.2}+{:.2}i", self.real, self.xayoliy) }
        else                    { write!(f, "{:.2}{:.2}i", self.real, self.xayoliy) }
    }
}

impl Add for Kompleks {
    type Output = Self;
    fn add(self, b: Self) -> Self {
        Kompleks::new(self.real + b.real, self.xayoliy + b.xayoliy)
    }
}

impl Sub for Kompleks {
    type Output = Self;
    fn sub(self, b: Self) -> Self {
        Kompleks::new(self.real - b.real, self.xayoliy - b.xayoliy)
    }
}

impl Mul for Kompleks {
    type Output = Self;
    fn mul(self, b: Self) -> Self {
        Kompleks::new(
            self.real * b.real - self.xayoliy * b.xayoliy,
            self.real * b.xayoliy + self.xayoliy * b.real,
        )
    }
}

impl Div for Kompleks {
    type Output = Self;
    fn div(self, b: Self) -> Self {
        let boshich: f64 = b.real * b.real + b.xayoliy * b.xayoliy;
        Kompleks::new(
            (self.real * b.real + self.xayoliy * b.xayoliy) / boshich,
            (self.xayoliy * b.real - self.real * b.xayoliy) / boshich,
        )
    }
}

impl Neg for Kompleks {
    type Output = Self;
    fn neg(self) -> Self { Kompleks::new(-self.real, -self.xayoliy) }
}

fn kompleks_misollari() {

    let z1: Kompleks = Kompleks::new(3.0, 4.0);
    let z2: Kompleks = Kompleks::new(1.0, -2.0);

    println!("z1 = {}", z1);
    println!("z2 = {}", z2);
    println!("z1 + z2 = {}", z1 + z2);
    println!("z1 - z2 = {}", z1 - z2);
    println!("z1 * z2 = {}", z1 * z2);
    println!("z1 / z2 = {}", z1 / z2);
    println!("-z1 = {}", -z1);
    println!("|z1| = {:.2}", z1.modul());
    println!("z1* = {}", z1.konjugat());
    // z1 = 3.00+4.00i
    // z2 = 1.00-2.00i
    // z1 + z2 = 4.00+2.00i
    // z1 - z2 = 2.00+6.00i
    // z1 * z2 = 11.00-2.00i
    // z1 / z2 = -1.00+2.00i
    // -z1 = -3.00-4.00i
    // |z1| = 5.00
    // z1* = 3.00-4.00i
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Temperatura {
    kelvin: f64,
}

impl Temperatura {
    fn kelvin(k: f64) -> Self { Temperatura { kelvin: k } }
    fn celsius(c: f64) -> Self { Temperatura { kelvin: c + 273.15 } }
    fn farengeyt(f: f64) -> Self { Temperatura { kelvin: (f - 32.0) * 5.0 / 9.0 + 273.15 } }
    fn as_celsius(&self) -> f64 { self.kelvin - 273.15 }
}

impl fmt::Display for Temperatura {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}°C", self.as_celsius())
    }
}

impl PartialOrd for Temperatura {
    fn partial_cmp(&self, b: &Self) -> Option<Ordering> {
        self.kelvin.partial_cmp(&b.kelvin)
    }
}

fn taqqoslash_misollari() {

    let t1: Temperatura = Temperatura::celsius(100.0);
    let t2: Temperatura = Temperatura::celsius(0.0);
    let t3: Temperatura = Temperatura::farengeyt(212.0); // = 100°C

    println!("{} > {}: {}", t1, t2, t1 > t2);
    println!("{} < {}: {}", t1, t2, t1 < t2);
    println!("{} == {}: {}", t1, t3, t1 == t3);
    // 100.00°C > 0.00°C: true
    // 100.00°C < 0.00°C: false
    // 100.00°C == 100.00°C: true

    let mut temperaturalar: Vec<Temperatura> = vec![
        Temperatura::celsius(37.0),
        Temperatura::celsius(0.0),
        Temperatura::celsius(100.0),
        Temperatura::celsius(-10.0),
    ];
    temperaturalar.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for t in &temperaturalar {
        print!("{} ", t);
    }
    println!();
    // -10.00°C 0.00°C 37.00°C 100.00°C
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct BitSet(u32);

impl BitSet {
    fn new() -> Self { BitSet(0) }
    fn dan(n: u32) -> Self { BitSet(n) }
    fn bit_qo_y(&mut self, pos: u32) { self.0 |= 1 << pos; }
    fn bit_ochir(&mut self, pos: u32) { self.0 &= !(1 << pos); }
    fn bit_bor(&self, pos: u32) -> bool { (self.0 >> pos) & 1 == 1 }
}

impl fmt::Display for BitSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BitSet({:08b})", self.0)
    }
}

impl BitAnd for BitSet {
    type Output = Self;
    fn bitand(self, b: Self) -> Self { BitSet(self.0 & b.0) }
}

impl BitOr for BitSet {
    type Output = Self;
    fn bitor(self, b: Self) -> Self { BitSet(self.0 | b.0) }
}

impl BitXor for BitSet {
    type Output = Self;
    fn bitxor(self, b: Self) -> Self { BitSet(self.0 ^ b.0) }
}

impl Not for BitSet {
    type Output = Self;
    fn not(self) -> Self { BitSet(!self.0) }
}

impl Shl<u32> for BitSet {
    type Output = Self;
    fn shl(self, n: u32) -> Self { BitSet(self.0 << n) }
}

impl Shr<u32> for BitSet {
    type Output = Self;
    fn shr(self, n: u32) -> Self { BitSet(self.0 >> n) }
}

fn bit_operator_misollari() {

    let a: BitSet = BitSet::dan(0b1010_1010);
    let b: BitSet = BitSet::dan(0b1100_1100);

    println!("a     = {}", a);
    println!("b     = {}", b);
    println!("a & b = {}", a & b);
    println!("a | b = {}", a | b);
    println!("a ^ b = {}", a ^ b);
    println!("a << 2 = {}", a << 2);
    println!("a >> 2 = {}", a >> 2);
    // a     = BitSet(10101010)
    // b     = BitSet(11001100)
    // a & b = BitSet(10001000)
    // a | b = BitSet(11101110)
    // a ^ b = BitSet(01100110)
    // a << 2 = BitSet(10101000)
    // a >> 2 = BitSet(00101010)

    // Bit o'rnatish
    let mut bs: BitSet = BitSet::new();
    bs.bit_qo_y(0);
    bs.bit_qo_y(3);
    bs.bit_qo_y(7);
    println!("{}", bs);
    println!("bit[3] = {}", bs.bit_bor(3));
    // BitSet(10001001)
    // bit[3] = true
}

#[derive(Debug)]
struct Matritsa {
    qatorlar: usize,
    ustunlar: usize,
    ichki: Vec<f64>,
}

impl Matritsa {
    fn new(q: usize, u: usize) -> Self {
        Matritsa { qatorlar: q, ustunlar: u, ichki: vec![0.0; q * u] }
    }
    fn to_ldirish(mut self, v: Vec<f64>) -> Self { self.ichki = v; self }
}

impl fmt::Display for Matritsa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for q in 0..self.qatorlar {
            write!(f, "[")?;
            for u in 0..self.ustunlar {
                if u > 0 { write!(f, ", ")?; }
                write!(f, "{:.1}", self.ichki[q * self.ustunlar + u])?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl Index<(usize, usize)> for Matritsa {
    type Output = f64;
    fn index(&self, (q, u): (usize, usize)) -> &f64 {
        &self.ichki[q * self.ustunlar + u]
    }
}

impl IndexMut<(usize, usize)> for Matritsa {
    fn index_mut(&mut self, (q, u): (usize, usize)) -> &mut f64 {
        &mut self.ichki[q * self.ustunlar + u]
    }
}

fn index_misollari() {

    let mut m: Matritsa = Matritsa::new(3, 3).to_ldirish(vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ]);

    println!("{}", m);
    // [1.0, 2.0, 3.0]
    // [4.0, 5.0, 6.0]
    // [7.0, 8.0, 9.0]

    println!("m[1,1] = {}", m[(1, 1)]);
    println!("m[2,0] = {}", m[(2, 0)]);
    // m[1,1] = 5
    // m[2,0] = 7

    m[(0, 0)] = 99.0;
    m[(1, 1)] = 55.0;
    println!("{}", m);
    // [99.0, 2.0, 3.0]
    // [4.0, 55.0, 6.0]
    // [7.0, 8.0, 9.0]
}

fn main() {

    println!("=== VEKTOR2D ===");
    vektor_misollari();

    println!("\n=== KOMPLEKS SON ===");
    kompleks_misollari();

    println!("\n=== TAQQOSLASH ===");
    taqqoslash_misollari();

    println!("\n=== BIT OPERATORLARI ===");
    bit_operator_misollari();

    println!("\n=== INDEX/INDEXMUT ===");
    index_misollari();
}
// #================================================================================================================================================#
// # |  №  | Operator | Trait          | Tavsif (UZ)                       | Описание (RU)                                                          |
// #================================================================================================================================================#
// # |   1 | a + b    | Add            | Qo'shish                          | Сложение                                                               |
// # |   2 | a - b    | Sub            | Ayirish                           | Вычитание                                                              |
// # |   3 | a * b    | Mul            | Ko'paytirish                      | Умножение                                                              |
// # |   4 | a / b    | Div            | Bo'lish                           | Деление                                                                |
// # |   5 | a % b    | Rem            | Qoldiq                            | Остаток                                                                |
// # |   6 | -a       | Neg            | Manfiy                            | Отрицание                                                              |
// # |   7 | a += b   | AddAssign      | Qo'shib o'zlashtirish             | Сложение с присвоением                                                 |
// # |   8 | a -= b   | SubAssign      | Ayirib o'zlashtirish              | Вычитание с присвоением                                                |
// # |   9 | a == b   | PartialEq      | Tenglik                           | Равенство                                                              |
// # |  10 | a < b    | PartialOrd     | Kichiklik                         | Меньше                                                                 |
// # |  11 | a & b    | BitAnd         | Bit VA                            | Побитовое И                                                            |
// # |  12 | a \| b   | BitOr          | Bit YOKI                          | Побитовое ИЛИ                                                          |
// # |  13 | a ^ b    | BitXor         | Bit XOR                           | Побитовый XOR                                                          |
// # |  14 | !a       | Not            | Bit INKOR                         | Побитовое НЕ                                                           |
// # |  15 | a << n   | Shl            | Chapga siljish                    | Сдвиг влево                                                            |
// # |  16 | a >> n   | Shr            | O'ngga siljish                    | Сдвиг вправо                                                           |
// # |  17 | a[i]     | Index          | Indekslash                        | Индексация                                                             |
// # |  18 | a[i] = v | IndexMut       | Indeks bilan o'zlashtirish        | Присвоение по индексу                                                  |
// #================================================================================================================================================#