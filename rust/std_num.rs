// #================================================================================================================================================#
// #                                                                   STD::NUM                                                                     #
// #                                STD::NUM — SONLAR BILAN ISHLASH. NONZERO, WRAPPING, SATURATING, CHECKED OPERATSIYALAR.                          #
// #                                STD::NUM — РАБОТА С ЧИСЛАМИ. NONZERO, WRAPPING, SATURATING, CHECKED ОПЕРАЦИИ.                                   #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::num::{
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    Wrapping, Saturating,
};

// std::num ichida nima bor:
// Что внутри std::num:
//
//   NonZero*   — nol bo'lmasligi kafolatlangan son
//   NonZero*   — число гарантированно не равное нулю
//   Wrapping   — to'lib ketganda aylanib ketuvchi son
//   Wrapping   — число с оберточной арифметикой при переполнении
//   Saturating — to'lib ketganda maksimum/minimumga yopishib qoluvchi
//   Saturating — число насыщающееся при переполнении
//   Checked    — xato qaytaruvchi arifmetika (metod sifatida)
//   Checked    — арифметика возвращающая ошибку (в виде методов)
//   Overflowing— to'lib ketishni xabar beruvchi arifmetika
//   Overflowing— арифметика сообщающая о переполнении
//   Wrapping   — xuddi to'lib ketgandek hisoblash
//   Wrapping   — вычисления как при переполнении

fn checked_arifmetika_misollari() {

    // checked_add — to'lib ketsa None
    // checked_add — None при переполнении
    let x: u8 = 250;
    println!("{:?}", x.checked_add(5));   // Some(255)
    println!("{:?}", x.checked_add(10));  // None — overflow!
    // Some(255)
    // None

    // checked_sub — manfiy bo'lsa None (unsigned uchun)
    // checked_sub — None если отрицательный (для unsigned)
    let y: u8 = 5;
    println!("{:?}", y.checked_sub(3));  // Some(2)
    println!("{:?}", y.checked_sub(10)); // None — underflow!
    // Some(2)
    // None

    // checked_mul — to'lib ketsa None
    // checked_mul — None при переполнении
    let z: u8 = 100;
    println!("{:?}", z.checked_mul(2));   // Some(200)
    println!("{:?}", z.checked_mul(3));   // None — overflow!
    // Some(200)
    // None

    // checked_div — nolga bo'lsa None
    // checked_div — None при делении на ноль
    let a: i32 = 100;
    println!("{:?}", a.checked_div(5));  // Some(20)
    println!("{:?}", a.checked_div(0));  // None — divide by zero!
    // Some(20)
    // None

    // checked_pow — daraja hisoblash
    // checked_pow — возведение в степень
    let b: u32 = 2;
    println!("{:?}", b.checked_pow(10)); // Some(1024)
    println!("{:?}", b.checked_pow(32)); // None — overflow!
    // Some(1024)
    // None

    // checked_neg — manfiy qilish
    // checked_neg — отрицание
    let c: i8 = -128;
    println!("{:?}", c.checked_neg()); // None — i8::MIN ni manfiy qilib bo'lmaydi
    let d: i8 = 5;
    println!("{:?}", d.checked_neg()); // Some(-5)
    // None
    // Some(-5)

    // checked_rem — qoldiq
    // checked_rem — остаток
    let e: i32 = 10;
    println!("{:?}", e.checked_rem(3));  // Some(1)
    println!("{:?}", e.checked_rem(0));  // None
    // Some(1)
    // None

    // Zanjirli checked operatsiya
    // Цепочка checked операций
    let natija: Option<i32> = 100i32
        .checked_add(50)
        .and_then(|n| n.checked_mul(2))
        .and_then(|n| n.checked_sub(100));
    println!("{:?}", natija);
    // Some(200)
}

fn saturating_arifmetika_misollari() {

    // saturating_add — maksimumda to'xtaydi
    // saturating_add — останавливается на максимуме
    let x: u8 = 250;
    println!("{}", x.saturating_add(5));   // 255 (MAX)
    println!("{}", x.saturating_add(100)); // 255 (MAX — overflow bo'lmaydi)
    // 255
    // 255

    // saturating_sub — minimumda to'xtaydi
    // saturating_sub — останавливается на минимуме
    let y: u8 = 5;
    println!("{}", y.saturating_sub(3));   // 2
    println!("{}", y.saturating_sub(100)); // 0 (MIN — underflow bo'lmaydi)
    // 2
    // 0

    // saturating_mul
    // saturating_mul
    let z: u8 = 100;
    println!("{}", z.saturating_mul(2)); // 200
    println!("{}", z.saturating_mul(3)); // 255 (MAX)
    // 200
    // 255

    // i32 bilan — manfiy va musbat chegaralar
    // с i32 — отрицательные и положительные границы
    println!("{}", i32::MAX.saturating_add(1));   // 2147483647 (MAX)
    println!("{}", i32::MIN.saturating_sub(1));   // -2147483648 (MIN)
    println!("{}", i32::MAX.saturating_mul(2));   // 2147483647 (MAX)
    // 2147483647
    // -2147483648
    // 2147483647

    // Saturating struct — operator overloading bilan
    // Структура Saturating — с перегрузкой операторов
    let a: Saturating<u8> = Saturating(250u8);
    let b: Saturating<u8> = Saturating(20u8);
    println!("{}", (a + b).0);  // 255 (MAX)
    println!("{}", (a - b).0);  // 230
    println!("{}", (a * b).0);  // 255 (MAX)
    // 255
    // 230
    // 255

    // Saturating — sensor ma'lumotlari uchun foydali
    // Saturating — полезно для данных датчиков
    let sensor_qiymati: u8 = 200;
    let ko_paytuvchi: u8 = 2;
    let to_ydirilgan: u8 = sensor_qiymati.saturating_mul(ko_paytuvchi);
    println!("Sensor (saturating): {}", to_ydirilgan); // 255 emas, 255
    // Sensor (saturating): 255
}

fn wrapping_arifmetika_misollari() {

    // wrapping_add — aylanib ketadi
    // wrapping_add — оборачивается
    let x: u8 = 250;
    println!("{}", x.wrapping_add(5));   // 255
    println!("{}", x.wrapping_add(10));  // 4  (255+1=0, 255+5=4)
    println!("{}", x.wrapping_add(100)); // 94
    // 255
    // 4
    // 94

    // wrapping_sub
    // wrapping_sub
    let y: u8 = 5;
    println!("{}", y.wrapping_sub(3));  // 2
    println!("{}", y.wrapping_sub(10)); // 251 (0-5 = 251 u8 da)
    // 2
    // 251

    // wrapping_mul
    // wrapping_mul
    let z: u8 = 200;
    println!("{}", z.wrapping_mul(2));  // 144 ((200*2) % 256)
    // 144

    // Wrapping struct — operator overloading bilan
    // Структура Wrapping — с перегрузкой операторов
    let a: Wrapping<u8> = Wrapping(250u8);
    let b: Wrapping<u8> = Wrapping(10u8);
    println!("{}", (a + b).0);  // 4
    println!("{}", (a - b).0);  // 240
    println!("{}", (a * b).0);  // ((250*10) % 256 = 2500 % 256 = 196)
    // 4
    // 240
    // 196

    // wrapping_neg — wrapping negation
    // wrapping_neg — отрицание с оборачиванием
    println!("{}", 0i8.wrapping_neg());    // 0
    println!("{}", 1i8.wrapping_neg());    // -1
    println!("{}", i8::MIN.wrapping_neg()); // -128 (MIN o'zi, chunki MIN = -128, -(-128) = 128 sig'maydi)
    // 0
    // -1
    // -128

    // wrapping_shl, wrapping_shr
    // wrapping_shl, wrapping_shr
    println!("{}", 1u8.wrapping_shl(7));  // 128
    println!("{}", 1u8.wrapping_shl(8));  // 1 (8 bit aylanib ketdi)
    println!("{}", 128u8.wrapping_shr(7)); // 1
    // 128
    // 1
    // 1

    // Wrapping — kriptografiya uchun foydali
    // Wrapping — полезно для криптографии
    let hash: Wrapping<u32> = Wrapping(0u32);
    let ma_lumot: &[u8] = b"salom";
    let mut h: Wrapping<u32> = Wrapping(2166136261u32);
    for &byte in ma_lumot {
        h ^= Wrapping(byte as u32);
        h *= Wrapping(16777619u32);
    }
    println!("FNV-1a hash: {}", h.0);
}

fn overflowing_arifmetika_misollari() {

    // overflowing_add — (natija, to'lib_ketdimi)
    // overflowing_add — (результат, было_ли_переполнение)
    let x: u8 = 250;
    println!("{:?}", x.overflowing_add(5));   // (255, false)
    println!("{:?}", x.overflowing_add(10));  // (4, true)
    // (255, false)
    // (4, true)

    // overflowing_sub
    // overflowing_sub
    let y: u8 = 5;
    println!("{:?}", y.overflowing_sub(3));   // (2, false)
    println!("{:?}", y.overflowing_sub(10));  // (251, true)
    // (2, false)
    // (251, true)

    // overflowing_mul
    // overflowing_mul
    let z: u8 = 200;
    println!("{:?}", z.overflowing_mul(2));   // (144, true)
    // (144, true)

    // overflowing_pow
    // overflowing_pow
    let a: u32 = 2;
    println!("{:?}", a.overflowing_pow(31)); // (2147483648, false)
    println!("{:?}", a.overflowing_pow(32)); // (0, true) — overflow
    // (2147483648, false)
    // (0, true)

    // Overflow tekshirish mantiqida
    // В логике проверки переполнения
    fn xavfsiz_qo_sh(a: u32, b: u32) -> Result<u32, String> {
        let (natija, to_lib_ketdi) = a.overflowing_add(b);
        if to_lib_ketdi {
            Err(format!("{} + {} overflow!", a, b))
        } else {
            Ok(natija)
        }
    }
    println!("{:?}", xavfsiz_qo_sh(100, 200));
    println!("{:?}", xavfsiz_qo_sh(u32::MAX, 1));
    // Ok(300)
    // Err("4294967295 + 1 overflow!")
}

fn nonzero_misollari() {

    // NonZeroU32 — nol bo'lmasligi kafolat
    // NonZeroU32 — гарантия ненулевого значения
    let n: NonZeroU32 = NonZeroU32::new(42).unwrap();
    println!("{}", n);
    println!("{}", n.get());
    // 42
    // 42

    // new() — None qaytaradi nol uchun
    // new() — возвращает None для нуля
    let nol: Option<NonZeroU32> = NonZeroU32::new(0);
    let musbat: Option<NonZeroU32> = NonZeroU32::new(100);
    println!("{:?}", nol);
    println!("{:?}", musbat);
    // None
    // Some(100)

    // Barcha NonZero turlari
    // Все NonZero типы
    let _a: NonZeroU8   = NonZeroU8::new(1).unwrap();
    let _b: NonZeroU16  = NonZeroU16::new(1000).unwrap();
    let _c: NonZeroU64  = NonZeroU64::new(u64::MAX).unwrap();
    let _d: NonZeroI32  = NonZeroI32::new(-42).unwrap();
    let _e: NonZeroIsize = NonZeroIsize::new(1).unwrap();

    // NonZero — xotira optimizatsiyasi
    // NonZero — оптимизация памяти
    println!("Option<u32>:        {} bayt", std::mem::size_of::<Option<u32>>());
    println!("Option<NonZeroU32>: {} bayt", std::mem::size_of::<Option<NonZeroU32>>());
    // Option<u32>:        8 bayt
    // Option<NonZeroU32>: 4 bayt  ← null pointer optimization!

    // NonZero — bo'luvchi sifatida (nolga bo'lish xavfi yo'q)
    // NonZero — в качестве делителя (нет риска деления на ноль)
    fn bo_l(a: u32, b: NonZeroU32) -> u32 {
        a / b.get()  // xavfsiz — b hech qachon 0 bo'lmaydi
        // безопасно — b никогда не равен 0
    }
    let bo_luvchi: NonZeroU32 = NonZeroU32::new(5).unwrap();
    println!("{}", bo_l(100, bo_luvchi));
    // 20

    // NonZeroU32::MIN va MAX
    // NonZeroU32::MIN и MAX
    println!("NonZeroU32::MIN = {}", NonZeroU32::MIN);
    println!("NonZeroU32::MAX = {}", NonZeroU32::MAX);
    // NonZeroU32::MIN = 1
    // NonZeroU32::MAX = 4294967295

    // NonZero — checked operatsiyalar
    // NonZero — checked операции
    let x: NonZeroU32 = NonZeroU32::new(5).unwrap();
    let y: NonZeroU32 = NonZeroU32::new(3).unwrap();
    println!("{:?}", x.checked_add(y.get()));
    println!("{:?}", NonZeroU32::MAX.checked_add(1));
    // Some(8)
    // None
}

fn son_konstantlari_misollari() {

    // Integer konstanta va metodlar
    // Целочисленные константы и методы
    println!("i8::MIN  = {}", i8::MIN);
    println!("i8::MAX  = {}", i8::MAX);
    println!("u8::MIN  = {}", u8::MIN);
    println!("u8::MAX  = {}", u8::MAX);
    println!("i32::MIN = {}", i32::MIN);
    println!("i32::MAX = {}", i32::MAX);
    println!("u64::MAX = {}", u64::MAX);
    // i8::MIN  = -128
    // i8::MAX  = 127
    // u8::MIN  = 0
    // u8::MAX  = 255
    // i32::MIN = -2147483648
    // i32::MAX = 2147483647
    // u64::MAX = 18446744073709551615

    // Float konstantalari
    // Константы с плавающей точкой
    println!("f32::MIN       = {}", f32::MIN);
    println!("f32::MAX       = {}", f32::MAX);
    println!("f32::INFINITY  = {}", f32::INFINITY);
    println!("f32::NEG_INFINITY = {}", f32::NEG_INFINITY);
    println!("f32::NAN       = {}", f32::NAN);
    println!("f64::EPSILON   = {}", f64::EPSILON);
    // f32::MIN       = -3.4028235e38
    // f32::MAX       = 3.4028235e38
    // f32::INFINITY  = inf
    // f32::NEG_INFINITY = -inf
    // f32::NAN       = NaN
    // f64::EPSILON   = 0.00000000000000022204460492503131

    // Float metodlari
    // Методы с плавающей точкой
    let f: f64 = -3.7;
    println!("{}", f.abs());    // 3.7
    println!("{}", f.ceil());   // -3.0
    println!("{}", f.floor());  // -4.0
    println!("{}", f.round());  // -4.0
    println!("{}", f.trunc());  // -3.0
    println!("{}", f.sqrt().is_nan()); // true (manfiy sonning ildizi)
    println!("{}", (4.0f64).sqrt()); // 2.0
    println!("{}", (2.0f64).powi(10)); // 1024.0
    println!("{}", (2.0f64).powf(0.5)); // 1.4142...
    println!("{}", (std::f64::consts::E).ln()); // 1.0
    println!("{}", (10.0f64).log10()); // 1.0
    println!("{}", (8.0f64).log2());   // 3.0

    // NaN tekshirish
    // Проверка NaN
    let nan: f64 = f64::NAN;
    println!("{}", nan.is_nan());      // true
    println!("{}", nan == nan);        // false! NaN != NaN
    println!("{}", nan.is_finite());   // false
    println!("{}", nan.is_infinite()); // false

    // Infinity
    // Бесконечность
    let inf: f64 = f64::INFINITY;
    println!("{}", inf.is_infinite()); // true
    println!("{}", inf.is_finite());   // false
    println!("{}", inf > 1000000.0);   // true

    // abs_diff — mutloq farq (overflow xavfsiz)
    // abs_diff — абсолютная разность (без риска переполнения)
    let a: u32 = 5;
    let b: u32 = 10;
    println!("{}", a.abs_diff(b));  // 5
    println!("{}", b.abs_diff(a));  // 5
    // 5
    // 5

    // leading_zeros, trailing_zeros, count_ones
    // leading_zeros, trailing_zeros, count_ones
    let n: u8 = 0b0001_1010;
    println!("{}", n.leading_zeros());   // 3
    println!("{}", n.trailing_zeros());  // 1
    println!("{}", n.count_ones());      // 3
    println!("{}", n.count_zeros());     // 5
    println!("{}", n.reverse_bits());    // 0b0101_1000 = 88
    // 3
    // 1
    // 3
    // 5
    // 88

    // ilog2, ilog10 — integer logarifm
    // ilog2, ilog10 — целочисленный логарифм
    println!("{}", 8u32.ilog2());    // 3
    println!("{}", 1000u32.ilog10()); // 2
    // 3
    // 2
}

fn real_hayot_misollari() {

    // 1. Xavfsiz o'rtacha hisoblash — overflow bo'lmasdan
    // 1. Безопасное вычисление среднего — без переполнения
    fn o_rtacha_xavfsiz(a: u32, b: u32) -> u32 {
        // (a + b) / 2 — overflow bo'lishi mumkin!
        // (a + b) / 2 — может переполниться!
        a / 2 + b / 2 + (a % 2 + b % 2) / 2
    }
    println!("{}", o_rtacha_xavfsiz(u32::MAX, u32::MAX));
    // 4294967295

    // 2. Saturating — LED yorqinligi
    // 2. Saturating — яркость LED
    struct Led { yorqinlik: u8 }
    impl Led {
        fn oshirish(&mut self, miqdor: u8) {
            self.yorqinlik = self.yorqinlik.saturating_add(miqdor);
        }
        fn kamaytirish(&mut self, miqdor: u8) {
            self.yorqinlik = self.yorqinlik.saturating_sub(miqdor);
        }
    }
    let mut led = Led { yorqinlik: 200 };
    led.oshirish(100); // 255 ga yetib to'xtaydi
    println!("LED: {}", led.yorqinlik);
    led.kamaytirish(255); // 0 ga yetib to'xtaydi (255 ≥ 200, shuning uchun 0)
    println!("LED: {}", led.yorqinlik);
    // LED: 255
    // LED: 0

    // 3. NonZero — xotira samarali Option
    // 3. NonZero — оптимизированный по памяти Option
    let sahifalar_soni: Option<NonZeroU32> = NonZeroU32::new(100);
    if let Some(n) = sahifalar_soni {
        println!("{} sahifa", n.get());
    }
    // 100 sahifa

    // 4. Wrapping — hashcode hisoblash
    // 4. Wrapping — вычисление hashcode
    fn string_hash(s: &str) -> u32 {
        s.bytes().fold(Wrapping(0u32), |acc, b| {
            acc * Wrapping(31) + Wrapping(b as u32)
        }).0
    }
    println!("hash('salom') = {}", string_hash("salom"));
    println!("hash('dunyo') = {}", string_hash("dunyo"));
    // hash('salom') = XXXX
    // hash('dunyo') = YYYY

    // 5. Checked — bank operatsiyasi
    // 5. Checked — банковская операция
    fn pul_o_tkaz(balans: u64, miqdor: u64) -> Result<u64, &'static str> {
        if miqdor == 0 {
            return Err("Miqdor nol bo'lishi mumkin emas");
        }
        balans.checked_sub(miqdor).ok_or("Balans yetarli emas")
    }
    println!("{:?}", pul_o_tkaz(1000, 500));
    println!("{:?}", pul_o_tkaz(1000, 1500));
    println!("{:?}", pul_o_tkaz(1000, 0));
    // Ok(500)
    // Err("Balans yetarli emas")
    // Err("Miqdor nol bo'lishi mumkin emas")
}

fn main() {

    println!("=== CHECKED ARIFMETIKA ===");
    checked_arifmetika_misollari();

    println!("\n=== SATURATING ARIFMETIKA ===");
    saturating_arifmetika_misollari();

    println!("\n=== WRAPPING ARIFMETIKA ===");
    wrapping_arifmetika_misollari();

    println!("\n=== OVERFLOWING ARIFMETIKA ===");
    overflowing_arifmetika_misollari();

    println!("\n=== NONZERO ===");
    nonzero_misollari();

    println!("\n=== SON KONSTANTLARI ===");
    son_konstantlari_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                | Tavsif (UZ)                               | Описание (RU)                                                |
// #================================================================================================================================================#
// # |                                        CHECKED ARIFMETIKA                                                                                    |
// #================================================================================================================================================#
// # |   1 | .checked_add(b)             | To'lsa None, aks holda Some               | None при переполнении, иначе Some                            |
// # |   2 | .checked_sub(b)             | Kamaysa None (unsigned), aks holda Some   | None при потере (unsigned), иначе Some                       |
// # |   3 | .checked_mul(b)             | To'lsa None, aks holda Some               | None при переполнении, иначе Some                            |
// # |   4 | .checked_div(b)             | Nolga bo'lsa None, aks holda Some         | None при делении на 0, иначе Some                            |
// # |   5 | .checked_pow(n)             | To'lsa None, aks holda Some               | None при переполнении, иначе Some                            |
// #================================================================================================================================================#
// # |                                        SATURATING ARIFMETIKA                                                                                 |
// #================================================================================================================================================#
// # |   6 | .saturating_add(b)          | MAX/MIN da to'xtaydi                       | Останавливается на MAX/MIN                                  |
// # |   7 | .saturating_sub(b)          | MIN da to'xtaydi                           | Останавливается на MIN                                      |
// # |   8 | .saturating_mul(b)          | MAX da to'xtaydi                           | Останавливается на MAX                                      |
// # |   9 | Saturating<T>               | Operator overloading bilan                 | С перегрузкой операторов                                    |
// #================================================================================================================================================#
// # |                                        WRAPPING ARIFMETIKA                                                                                   |
// #================================================================================================================================================#
// # |  10 | .wrapping_add(b)            | Aylanib ketadi                             | Оборачивается                                               |
// # |  11 | .wrapping_sub(b)            | Aylanib ketadi                             | Оборачивается                                               |
// # |  12 | .wrapping_mul(b)            | Aylanib ketadi                             | Оборачивается                                               |
// # |  13 | Wrapping<T>                 | Operator overloading bilan                 | С перегрузкой операторов                                    |
// #================================================================================================================================================#
// # |                                        OVERFLOWING ARIFMETIKA                                                                                |
// #================================================================================================================================================#
// # |  14 | .overflowing_add(b)         | (natija, to'lib_ketdimi) → (T, bool)       | (результат, было_переполнение) → (T, bool)                  |
// # |  15 | .overflowing_sub(b)         | (natija, kamayib_ketdimi) → (T, bool)      | (результат, было_потеря) → (T, bool)                        |
// #================================================================================================================================================#
// # |                                        NONZERO                                                                                               |
// #================================================================================================================================================#
// # |  16 | NonZeroU32::new(n)          | None nol uchun, Some(n) boshqalar uchun    | None для нуля, Some(n) для остальных                        |
// # |  17 | .get()                      | Ichki qiymatni olish                       | Получение внутреннего значения                              |
// # |  18 | Option<NonZeroU32>          | 4 bayt (NULL pointer optimization)         | 4 байта (оптимизация нулевого указателя)                    |
// # |  19 | Bo'luvchi sifatida          | Nolga bo'lish xavfi yo'q                   | Нет риска деления на ноль                                   |
// #================================================================================================================================================#