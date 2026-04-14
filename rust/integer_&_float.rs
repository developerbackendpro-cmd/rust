// #================================================================================================================================================#
// #                                              INTEGER TURLAR  / ЦЕЛОЧИСЛЕННЫЕ ТИПЫ ДАННЫХ                                                       #
// #                                                      KASR SONLAR / ДРОБНЫЕ ЧИСЛА                                                               #
// #================================================================================================================================================#

fn main() {
    // i8 — ishorali 8-bitli butun son
    // i8 — знаковое 8-битное целое число
    let i8_son: i8 = 100;
    println!("i8 → -128 dan 127 gacha → {}", i8_son);
    // i8 → -128 dan 127 gacha → 100

    // i16 — ishorali 16-bitli butun son
    // i16 — знаковое 16-битное целое число
    let i16_son: i16 = 1_000;
    println!("i16 → -32_768 dan 32_767 gacha → {}", i16_son);
    // i16 → -32_768 dan 32_767 gacha → 1000

    // i32 — ishorali 32-bitli butun son (eng ko'p ishlatiladigan)
    // i32 — знаковое 32-битное целое число (самый используемый)
    let i32_son: i32 = 100_000;
    println!("i32 → -2_147_483_648 dan 2_147_483_647 gacha → {}", i32_son);
    // i32 → -2_147_483_648 dan 2_147_483_647 gacha → 100000

    // i64 — ishorali 64-bitli butun son
    // i64 — знаковое 64-битное целое число
    let i64_son: i64 = 1_000_000_000;
    println!("i64 -9_223_372_036_854_775_808 dan 9_223_372_036_854_775_807 gacha → {}", i64_son);
    // i64 → -9_223_372_036_854_775_808 dan 9_223_372_036_854_775_807 gacha  → 1000000000

    // i128 — ishorali 128-bitli butun son (eng katta ishorali)
    // i128 — знаковое 128-битное целое число (самый большой знаковый)
    let i128_son: i128 = 1_000_000_000_000;
    println!("i128 — -170_141_183_460_469_231_731_687_303_715_884_105_728 dan 170_141_183_460_469_231_731_687_303_715_884_105_727 gacha → {}", i128_son);
    // i128 → -170_141_183_460_469_231_731_687_303_715_884_105_728 dan 170_141_183_460_469_231_731_687_303_715_884_105_727 gacha → 1000000000000

    // isize — platformaga bog'liq ishorali son (pointer o'lchami)
    // isize — знаковое число, зависящее от платформы (размер указателя)
    let isize_son: isize = 500;
    println!("isize — 64-bit tizimda i64 bilan teng  → {}", isize_son);
    // isize → 64-bit tizimda i64 bilan teng  → 500

    // u8 — ishorsiz 8-bitli butun son
    // u8 — беззнаковое 8-битное целое число
    let u8_son: u8 = 200;
    println!("u8 → 0 dan 255 gacha  → {}", u8_son);
    // u8 → 0 dan 255 gacha → 200

    // u16 — ishorsiz 16-bitli butun son
    // u16 — беззнаковое 16-битное целое число
    let u16_son: u16 = 60_000;
    println!("u16 → 0 dan 65_535 gacha → {}", u16_son);
    // u16 → 0 dan 65_535 gacha → 60000

    // u32 — ishorsiz 32-bitli butun son
    // u32 — беззнаковое 32-битное целое число
    let u32_son: u32 = 4_000_000;
    println!("u32 → 0 dan 4_294_967_295 gacha → {}", u32_son);
    // u32 → 0 dan 4_294_967_295 gacha → 4000000

    // u64 — ishorsiz 64-bitli butun son
    // u64 — беззнаковое 64-битное целое число
    let u64_son: u64 = 10_000_000_000;
    println!("u64 → 0 dan 18_446_744_073_709_551_615 gacha → {}", u64_son);
    // u64 → 0 dan 18_446_744_073_709_551_615 gacha → 10000000000

    // u128 — ishorsiz 128-bitli butun son (eng katta ishorsiz)
    // u128 — беззнаковое 128-битное целое число (самый большой беззнаковый)
    let u128_son: u128 = 100_000_000_000_000;
    println!("u128 → 0 dan 340_282_366_920_938_463_463_374_607_431_768_211_455 gacha → {}", u128_son);
    // u128 → 0 dan 340_282_366_920_938_463_463_374_607_431_768_211_455 gacha → 100000000000000

    // usize — platformaga bog'liq ishorsiz son (indeks uchun ishlatiladi)
    // usize — беззнаковое число, зависящее от платформы (используется для индексов)
    let usize_son: usize = 1024;
    println!("usize — 64-bit tizimda u64 bilan teng  → {}", usize_son);
    // usize → 64-bit tizimda u64 bilan teng → 1024

    // f32 — 32-bitli suzuvchi nuqtali son (oddiy aniqlik)
    // f32 — 32-битное число с плавающей точкой (одинарная точность)
    let f32_son: f32 = 3.14;
    println!("f32 1.401298464324817e-45 dan 3.4028234663852886e38 gacha → {}", f32_son);
    // f32 → 1.401298464324817e-45 dan 3.4028234663852886e38 gacha  → 3.14

    // f64 — 64-bitli suzuvchi nuqtali son (ikki baravar aniqlik)
    // f64 — 64-битное число с плавающей точкой (двойная точность)
    let f64_son: f64 = 3.141592653589793;
    println!("f64  — 4.9406564584124654e-324 dan 1.7976931348623157e308 gacha → {}", f64_son);
    // f64 → 4.9406564584124654e-324 dan 1.7976931348623157e308 gacha → 3.141592653589793

    // abs() — mutlaq qiymat qaytaradi (faqat signed turlar)
    // abs() — возвращает абсолютное значение (только signed типы)
    let x: i32 = -42;
    let natija = x.abs();
    println!("1) abs() -► {}", natija);
    // 1) abs() -► 42

    // signum() — sonning ishorasini qaytaradi: -1, 0, yoki 1
    // signum() — возвращает знак числа: -1, 0 или 1
    let a: i32 = -99;
    let b: i32 = 0;
    let c: i32 = 55;
    println!("2) signum() manfiy ► {}", a.signum());
    println!("2) signum() nol    ► {}", b.signum());
    println!("2) signum() musbat ► {}", c.signum());
    // 2) signum() manfiy  ► -1
    // 2) signum() nol     ► 0
    // 2) signum() musbat  ► 1

    // pow() — sonni darajaga ko'taradi
    // pow() — возводит число в степень
    let asos: i32 = 2;
    let natija = asos.pow(10);
    println!("3) pow() ► {}", natija);
    // 3) pow() ► 1024

    // isqrt() — butun son kvadrat ildizi (Rust 1.84+)
    // isqrt() — целочисленный квадратный корень (Rust 1.84+)
    let son: u32 = 144;
    let natija = son.isqrt();
    println!("4) isqrt() ► {}", natija);
    // 4) isqrt() ► 12

    // abs_diff() — ikki son orasidagi farq (har doim musbat)
    // abs_diff() — разница между двумя числами (всегда положительная)
    let narx1: u32 = 500;
    let narx2: u32 = 320;
    let natija = narx1.abs_diff(narx2);
    println!("5) abs_diff() ► {}", natija);
    // 5) abs_diff() ► 180

    // min() — ikkita sondan kichigini qaytaradi
    // min() — возвращает меньшее из двух чисел
    let ball: i32 = 15;
    let natija = ball.min(10);
    println!("6) min() ► {}", natija);
    // 6) min() ► 10

    // max() — ikkita sondan kattasini qaytaradi
    // max() — возвращает большее из двух чисел
    let ball: i32 = 15;
    let natija = ball.max(20);
    println!("7) max() ► {}", natija);
    // 7) max() ► 20

    // clamp() — sonni berilgan oraliqda ushlab turadi
    // clamp() — удерживает число в заданном диапазоне
    let ovoz: i32 = 150;
    let natija = ovoz.clamp(0, 100);
    println!("8) clamp() (150 → 0..100) ► {}", natija);
    // 8) clamp() (150 → 0..100) ► 100

    // checked_add() — overflow bo'lsa None qaytaradi
    // checked_add() — возвращает None при переполнении
    let limit: i32 = i32::MAX;
    let natija = limit.checked_add(1);
    println!("9) checked_add() overflow ► {:?}", natija);
    // 9) checked_add() overflow -► None

    let son: i32 = 10;
    let natija = son.checked_add(5);
    println!("9) checked_add() ok ► {:?}", natija);
    // 9) checked_add() ok ► Some(15)

    // checked_sub() — overflow bo'lsa None qaytaradi
    // checked_sub() — возвращает None при переполнении
    let limit: i32 = i32::MIN;
    let natija = limit.checked_sub(1);
    println!("10) checked_sub() overflow ► {:?}", natija);
    // 10) checked_sub() overflow  ► None

    // checked_mul() — overflow bo'lsa None qaytaradi
    // checked_mul() — возвращает None при переполнении
    let limit: i32 = i32::MAX;
    let natija = limit.checked_mul(2);
    println!("11) checked_mul() overflow ► {:?}", natija);
    // 11) checked_mul() overflow ► None

    // checked_div() — nolga bo'lishda None qaytaradi
    // checked_div() — возвращает None при делении на ноль
    let son: i32 = 10;
    let natija = son.checked_div(0);
    println!("12) checked_div() nol ► {:?}", natija);
    // 12) checked_div() nol ► None

    let natija = son.checked_div(2);
    println!("12) checked_div() ok ► {:?}", natija);
    // 12) checked_div() ok ► Some(5)

    // checked_rem() — nolga bo'lishda None qaytaradi
    // checked_rem() — возвращает None при делении на ноль
    let son: i32 = 10;
    let natija = son.checked_rem(3);
    println!("13) checked_rem() ► {:?}", natija);
    // 13) checked_rem() ► Some(1)

    // saturating_add() — overflow bo'lsa MAX/MIN da to'xtaydi
    // saturating_add() — останавливается на MAX/MIN при переполнении
    let limit: i32 = i32::MAX;
    let natija = limit.saturating_add(999);
    println!("14) saturating_add() ► {}", natija);
    // 14) saturating_add() ► 2147483647

    // saturating_sub() — overflow bo'lsa MAX/MIN da to'xtaydi
    // saturating_sub() — останавливается на MAX/MIN при переполнении
    let limit: i32 = i32::MIN;
    let natija = limit.saturating_sub(1);
    println!("15) saturating_sub() ► {}", natija);
    // 15) saturating_sub() ► -2147483648

    // saturating_mul() — overflow bo'lsa MAX/MIN da to'xtaydi
    // saturating_mul() — останавливается на MAX/MIN при переполнении
    let limit: i32 = i32::MAX;
    let natija = limit.saturating_mul(2);
    println!("16) saturating_mul() ► {}", natija);
    // 16) saturating_mul() ► 2147483647

    // wrapping_add() — overflow bo'lsa aylanib ketadi (C kabi)
    // wrapping_add() — при переполнении оборачивается (как в C)
    let limit: i32 = i32::MAX;
    let natija = limit.wrapping_add(1);
    println!("17) wrapping_add() ► {}", natija);
    // 17) wrapping_add() ► -2147483648

    // wrapping_sub() — overflow bo'lsa aylanib ketadi
    // wrapping_sub() — при переполнении оборачивается
    let limit: i32 = i32::MIN;
    let natija = limit.wrapping_sub(1);
    println!("18) wrapping_sub() ► {}", natija);
    // 18) wrapping_sub() ► 2147483647

    // wrapping_mul() — overflow bo'lsa aylanib ketadi
    // wrapping_mul() — при переполнении оборачивается
    let limit: i32 = i32::MAX;
    let natija = limit.wrapping_mul(2);
    println!("19) wrapping_mul() ► {}", natija);
    // 19) wrapping_mul() ► -2

    // overflowing_add() — (natija, overflow bo'ldimi?) qaytaradi
    // overflowing_add() — возвращает (результат, было_ли_переполнение?)
    let limit: i32 = i32::MAX;
    let (natija, overflow) = limit.overflowing_add(1);
    println!("20) overflowing_add() ► {}, {}", natija, overflow);
    // 20) overflowing_add() ► -2147483648, true

    let son: i32 = 10;
    let (natija, overflow) = son.overflowing_add(5);
    println!("20) overflowing_add() ok ► {}, {}", natija, overflow);
    // 20) overflowing_add() ok ► 15, false

    // count_ones() — nechta 1-bit borligini qaytaradi
    // count_ones() — возвращает количество единичных битов
    let bits: u32 = 0b_1010_1100;
    let natija = bits.count_ones();
    println!("21) count_ones() ► {}", natija);
    // 21) count_ones() ► 4

    // count_zeros() — nechta 0-bit borligini qaytaradi
    // count_zeros() — возвращает количество нулевых битов
    let bits: u32 = 0b_1010_1100;
    let natija = bits.count_zeros();
    println!("22) count_zeros() ► {}", natija);
    // 22) count_zeros() ► 28

    // leading_zeros() — chapdan nechta 0-bit borligini qaytaradi
    // leading_zeros() — возвращает количество ведущих нулевых битов
    let bits: u32 = 0b_0000_1010;
    let natija = bits.leading_zeros();
    println!("23) leading_zeros() ► {}", natija);
    // 23) leading_zeros() ► 28

    // trailing_zeros() — o'ngdan nechta 0-bit borligini qaytaradi
    // trailing_zeros() — возвращает количество замыкающих нулевых битов
    let bits: u32 = 0b_1010_1000;
    let natija = bits.trailing_zeros();
    println!("24) trailing_zeros() ► {}", natija);
    // 24) trailing_zeros() ► 3

    // reverse_bits() — bitlarni teskari tartibda qaytaradi
    // reverse_bits() — возвращает биты в обратном порядке
    let bits: u8 = 0b_1010_1100;
    let natija = bits.reverse_bits();
    println!("25) reverse_bits() ► {:08b}", natija);
    // 25) reverse_bits() ► 00110101

    // rotate_left() — bitlarni chapga aylantiradi
    // rotate_left() — вращает биты влево
    let son: u32 = 1;
    let natija = son.rotate_left(3);
    println!("26) rotate_left() ► {}", natija);
    // 26) rotate_left() ► 8

    // rotate_right() — bitlarni o'ngga aylantiradi
    // rotate_right() — вращает биты вправо
    let son: u32 = 8;
    let natija = son.rotate_right(3);
    println!("27) rotate_right() ► {}", natija);
    // 27) rotate_right() ► 1

    // is_positive() — son musbatmi?
    // is_positive() — является ли число положительным?
    let son: i32 = 5;
    println!("28) is_positive() ► {}", son.is_positive());
    // 28) is_positive() ► true

    // is_negative() — son manfiyimi?
    // is_negative() — является ли число отрицательным?
    let son: i32 = -5;
    println!("29) is_negative() ► {}", son.is_negative());
    // 29) is_negative() ► true

    // is_power_of_two() — son 2 ning darajasimi?
    // is_power_of_two() — является ли число степенью двойки?
    let son: u32 = 16;
    println!("30) is_power_of_two() (16) ► {}", son.is_power_of_two());
    // 30) is_power_of_two() (16) ► true

    let son: u32 = 15;
    println!("30) is_power_of_two() (15) ► {}", son.is_power_of_two());
    // 30) is_power_of_two() (15)  ► false

    // next_power_of_two() — keyingi 2 ning darajasini qaytaradi
    // next_power_of_two() — возвращает следующую степень двойки
    let son: u32 = 5;
    let natija = son.next_power_of_two();
    println!("31) next_power_of_two() (5) ► {}", natija);
    // 31) next_power_of_two() (5) ► 8

    let son: u32 = 8;
    let natija = son.next_power_of_two();
    println!("31) next_power_of_two() (8) ► {}", natija);
    // 31) next_power_of_two() (8) ► 8

    // to_be_bytes() — sonni big-endian baytlarga aylantiradi
    // to_be_bytes() — преобразует число в байты big-endian
    let son: i32 = 255;
    let natija = son.to_be_bytes();
    println!("32) to_be_bytes() ► {:?}", natija);
    // 32) to_be_bytes() ► [0, 0, 0, 255]

    // to_le_bytes() — sonni little-endian baytlarga aylantiradi
    // to_le_bytes() — преобразует число в байты little-endian
    let son: i32 = 255;
    let natija = son.to_le_bytes();
    println!("33) to_le_bytes() ► {:?}", natija);
    // 33) to_le_bytes() ► [255, 0, 0, 0]

    // from_be_bytes() — big-endian baytlardan son yasaydi
    // from_be_bytes() — создаёт число из байтов big-endian
    let baytlar: [u8; 4] = [0, 0, 0, 255];
    let natija = i32::from_be_bytes(baytlar);
    println!("34) from_be_bytes() ► {}", natija);
    // 34) from_be_bytes() ► 255

    // from_le_bytes() — little-endian baytlardan son yasaydi
    // from_le_bytes() — создаёт число из байтов little-endian
    let baytlar: [u8; 4] = [255, 0, 0, 0];
    let natija = i32::from_le_bytes(baytlar);
    println!("35) from_le_bytes() ► {}", natija);
    // 35) from_le_bytes() ► 255

    // to_string() — sonni String ga aylantiradi
    // to_string() — преобразует число в String
    let son: i32 = 42;
    let matn = son.to_string();
    println!("36) to_string() ► {}", matn);
    // 36) to_string() ► 42

    // parse() — String ni songa aylantiradi
    // parse() — преобразует String в число
    let matn = "123";
    let son: i32 = matn.parse().unwrap();
    println!("37) parse() ► {}", son + 1);
    // 37) parse() ► 124

    let xato = "abc";
    let natija: Result<i32, _> = xato.parse();
    println!("37) parse() xato  ► {}", natija.is_err());
    // 37) parse() xato ► true

    // abs() — musbat qiymatni qaytaradi
    // abs() — возвращает абсолютное значение
    let x: f64 = -3.14;
    let natija = x.abs();
    println!("1) abs() ► {}", natija);
    // 1) abs() ► 3.14

    // sqrt() — kvadrat ildiz
    // sqrt() — квадратный корень
    let son: f64 = 16.0;
    let natija = son.sqrt();
    println!("2) sqrt() ► {}", natija);
    // 2) sqrt() ► 4

    // cbrt() — kub ildiz
    // cbrt() — кубический корень
    let son: f64 = 27.0;
    let natija = son.cbrt();
    println!("3) cbrt() ► {}", natija);
    // 3) cbrt() ► 3

    // powf() — suzuvchi nuqtali darajaga ko'taradi
    // powf() — возводит в степень с плавающей точкой
    let asos: f64 = 2.0;
    let natija = asos.powf(10.0);
    println!("4) powf() ► {}", natija);
    // 4) powf() ► 1024

    // powi() — butun son darajaga ko'taradi (tezroq)
    // powi() — возводит в целочисленную степень (быстрее)
    let asos: f64 = 2.0;
    let natija = asos.powi(10);
    println!("5) powi() ► {}", natija);
    // 5) powi() ► 1024

    // floor() — pastga yaxlitlaydi
    // floor() — округляет вниз
    let son: f64 = 3.9;
    let natija = son.floor();
    println!("6) floor() ► {}", natija);
    // 6) floor() ► 3

    // ceil() — tepaga yaxlitlaydi
    // ceil() — округляет вверх
    let son: f64 = 3.1;
    let natija = son.ceil();
    println!("7) ceil() ► {}", natija);
    // 7) ceil() ► 4

    // round() — yaqiniga yaxlitlaydi
    // round() — округляет до ближайшего
    let son: f64 = 3.5;
    let natija = son.round();
    println!("8) round() ► {}", natija);
    // 8) round() ► 4

    let son: f64 = 3.4;
    let natija = son.round();
    println!("8) round() ► {}", natija);
    // 8) round() ► 3

    // trunc() — kasr qismini olib tashlaydi
    // trunc() — отбрасывает дробную часть
    let son: f64 = 3.99;
    let natija = son.trunc();
    println!("9) trunc() ► {}", natija);
    // 9) trunc() ► 3

    // fract() — faqat kasr qismini qaytaradi
    // fract() — возвращает только дробную часть
    let son: f64 = 3.75;
    let natija = son.fract();
    println!("10) fract() ► {}", natija);
    // 10) fract() ► 0.75

    // min() — kichigini qaytaradi (NaN xavfsiz)
    // min() — возвращает меньшее (безопасно для NaN)
    let a: f64 = 3.5;
    let b: f64 = 7.2;
    println!("11) min() ► {}", a.min(b));
    // 11) min() ► 3.5

    // max() — kattasini qaytaradi (NaN xavfsiz)
    // max() — возвращает большее (безопасно для NaN)
    println!("12) max() ► {}", a.max(b));
    // 12) max() ► 7.2

    // clamp() — sonni oraliqda ushlab turadi
    // clamp() — удерживает число в диапазоне
    let son: f64 = 1.5;
    let natija = son.clamp(0.0, 1.0);
    println!("13) clamp() ► {}", natija);
    // 13) clamp() ► 1

    // ln() — natural logarifm (e asosida)
    // ln() — натуральный логарифм (по основанию e)
    let son: f64 = std::f64::consts::E;
    let natija = son.ln();
    println!("14) ln() ► {}", natija);
    // 14) ln() ► 1

    // log2() — 2 asosidagi logarifm
    // log2() — логарифм по основанию 2
    let son: f64 = 8.0;
    let natija = son.log2();
    println!("15) log2() ► {}", natija);
    // 15) log2() ► 3

    // log10() — 10 asosidagi logarifm
    // log10() — логарифм по основанию 10
    let son: f64 = 1000.0;
    let natija = son.log10();
    println!("16) log10() ► {}", natija);
    // 16) log10() ► 3

    // log() — ixtiyoriy asosidagi logarifm
    // log() — логарифм по произвольному основанию
    let son: f64 = 27.0;
    let natija = son.log(3.0);
    println!("17) log() ► {}", natija);
    // 17) log() ► 3

    // exp() — e^x ni hisoblaydi
    // exp() — вычисляет e^x
    let son: f64 = 1.0;
    let natija = son.exp();
    println!("18) exp() ► {:.5}", natija);
    // 18) exp() ► 2.71828

    // sin() — sinusni hisoblaydi (radian)
    // sin() — вычисляет синус (в радианах)
    let burchak: f64 = std::f64::consts::PI / 2.0;
    let natija = burchak.sin();
    println!("19) sin() ► {}", natija);
    // 19) sin() ► 1

    // cos() — kosinusni hisoblaydi (radian)
    // cos() — вычисляет косинус (в радианах)
    let burchak: f64 = 0.0;
    let natija = burchak.cos();
    println!("20) cos() ► {}", natija);
    // 20) cos() ► 1

    // tan() — tangensni hisoblaydi (radian)
    // tan() — вычисляет тангенс (в радианах)
    let burchak: f64 = std::f64::consts::PI / 4.0;
    let natija = burchak.tan();
    println!("21) tan() ► {:.1}", natija);
    // 21) tan() ► 1.0

    // to_radians() — darajani radianga aylantiradi
    // to_radians() — переводит градусы в радианы
    let daraja: f64 = 180.0;
    let natija = daraja.to_radians();
    println!("22) to_radians() ► {:.5}", natija);
    // 22) to_radians() ► 3.14159

    // to_degrees() — radianni darajaga aylantiradi
    // to_degrees() — переводит радианы в градусы
    let radian: f64 = std::f64::consts::PI;
    let natija = radian.to_degrees();
    println!("23) to_degrees() ► {}", natija);
    // 23) to_degrees() ► 180

    // signum() — sonning ishorasini qaytaradi
    // signum() — возвращает знак числа
    let son: f64 = -7.5;
    let natija = son.signum();
    println!("24) signum() ► {}", natija);
    // 24) signum() ► -1

    // hypot() — gipotenuzani hisoblaydi: sqrt(x²+y²)
    // hypot() — вычисляет гипотенузу: sqrt(x²+y²)
    let a: f64 = 3.0;
    let b: f64 = 4.0;
    let natija = a.hypot(b);
    println!("25) hypot() ► {}", natija);
    // 25) hypot() ► 5

    // is_nan() — son NaN (Not a Number) ekanligini tekshiradi
    // is_nan() — проверяет, является ли число NaN
    let son: f64 = f64::NAN;
    println!("26) is_nan() ► {}", son.is_nan());
    // 26) is_nan() ► true

    // is_infinite() — son cheksizmi?
    // is_infinite() — является ли число бесконечным?
    let son: f64 = f64::INFINITY;
    println!("27) is_infinite() ► {}", son.is_infinite());
    // 27) is_infinite() ► true

    // is_finite() — son chekliimi?
    // is_finite() — является ли число конечным?
    let son: f64 = 3.14;
    println!("28) is_finite() ► {}", son.is_finite());
    // 28) is_finite() ► true

    // is_sign_positive() — son musbat ishorali?
    // is_sign_positive() — имеет ли число положительный знак?
    let son: f64 = 3.14;
    println!("29) is_sign_positive() ► {}", son.is_sign_positive());
    // 29) is_sign_positive() ► true

    // is_sign_negative() — son manfiy ishorali?
    // is_sign_negative() — имеет ли число отрицательный знак?
    let son: f64 = -3.14;
    println!("30) is_sign_negative() ► {}", son.is_sign_negative());
    // 30) is_sign_negative() ► true

    // recip() — sonning teskarisini qaytaradi: 1/x
    // recip() — возвращает обратное значение: 1/x
    let son: f64 = 4.0;
    let natija = son.recip();
    println!("31) recip() ► {}", natija);
    // 31) recip() ► 0.25

    // to_string() — sonni String ga aylantiradi
    // to_string() — преобразует число в String
    let son: f64 = 3.14;
    let matn = son.to_string();
    println!("32) to_string() ► {}", matn);
    // 32) to_string() ► 3.14

    // parse() — String ni f64 ga aylantiradi
    // parse() — преобразует String в f64
    let matn = "3.14";
    let son: f64 = matn.parse().unwrap();
    println!("33) parse() ► {}", son);
    // 33) parse() ► 3.14

    // to_bits() — f64 ni u64 bitlar ko'rinishida qaytaradi
    // to_bits() — возвращает f64 в виде битов u64
    let son: f64 = 1.0;
    let natija = son.to_bits();
    println!("34) to_bits() ► {}", natija);
    // 34) to_bits() ► 4607182418800017408

    // from_bits() — u64 bitlardan f64 yasaydi
    // from_bits() — создаёт f64 из битов u64
    let bits: u64 = 4607182418800017408;
    let natija = f64::from_bits(bits);
    println!("35) from_bits() ► {}", natija);
    // 35) from_bits() ► 1
}
// #================================================================================================================================================#
// # |  №  | Metod                    | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # == INTEGER ====================================================================================================================================#
// # |   1 | abs()                    | Mutlaq qiymat (faqat signed)                         | Абсолютное значение (только signed)                  |
// # |   2 | signum()                 | Ishora: -1, 0, 1                                     | Знак числа: -1, 0, 1                                 |
// # |   3 | pow()                    | Darajaga ko'taradi                                   | Возводит в степень                                   |
// # |   4 | isqrt()                  | Butun son kvadrat ildizi                             | Целочисленный квадратный корень                      |
// # |   5 | abs_diff()               | Ikki son farqi (har doim musbat)                     | Разность двух чисел (всегда положительная)           |
// # |   6 | min()                    | Kichik sonni qaytaradi                               | Возвращает меньшее число                             |
// # |   7 | max()                    | Katta sonni qaytaradi                                | Возвращает большее число                             |
// # |   8 | clamp()                  | Sonni oraliqda ushlab turadi                         | Удерживает число в диапазоне                         |
// # |   9 | checked_add()            | Overflow bo'lsa None                                 | None при переполнении                                |
// # |  10 | checked_sub()            | Overflow bo'lsa None                                 | None при переполнении                                |
// # |  11 | checked_mul()            | Overflow bo'lsa None                                 | None при переполнении                                |
// # |  12 | checked_div()            | Nolga bo'lishda None                                 | None при делении на ноль                             |
// # |  13 | checked_rem()            | Nolga bo'lishda None                                 | None при делении на ноль                             |
// # |  14 | saturating_add()         | MAX/MIN da to'xtaydi                                 | Останавливается на MAX/MIN                           |
// # |  15 | saturating_sub()         | MAX/MIN da to'xtaydi                                 | Останавливается на MAX/MIN                           |
// # |  16 | saturating_mul()         | MAX/MIN da to'xtaydi                                 | Останавливается на MAX/MIN                           |
// # |  17 | wrapping_add()           | Aylanib ketadi (C kabi)                              | Оборачивается при переполнении (как в C)             |
// # |  18 | wrapping_sub()           | Aylanib ketadi                                       | Оборачивается при переполнении                       |
// # |  19 | wrapping_mul()           | Aylanib ketadi                                       | Оборачивается при переполнении                       |
// # |  20 | overflowing_add()        | (natija, overflow_bo'ldimi?) qaytaradi               | Возвращает (результат, было_переполнение?)           |
// # |  21 | count_ones()             | Nechta 1-bit bor                                     | Количество единичных битов                           |
// # |  22 | count_zeros()            | Nechta 0-bit bor                                     | Количество нулевых битов                             |
// # |  23 | leading_zeros()          | Chapdan nechta 0-bit                                 | Количество ведущих нулей                             |
// # |  24 | trailing_zeros()         | O'ngdan nechta 0-bit                                 | Количество замыкающих нулей                          |
// # |  25 | reverse_bits()           | Bitlarni teskari tartibda                            | Биты в обратном порядке                              |
// # |  26 | rotate_left()            | Bitlarni chapga aylantiradi                          | Вращает биты влево                                   |
// # |  27 | rotate_right()           | Bitlarni o'ngga aylantiradi                          | Вращает биты вправо                                  |
// # |  28 | is_positive()            | Son musbatmi?                                        | Является ли число положительным?                     |
// # |  29 | is_negative()            | Son manfiyimi?                                       | Является ли число отрицательным?                     |
// # |  30 | is_power_of_two()        | Son 2 ning darajasimi?                               | Является ли число степенью двойки?                   |
// # |  31 | next_power_of_two()      | Keyingi 2 ning darajasi                              | Следующая степень двойки                             |
// # |  32 | to_be_bytes()            | Big-endian baytlarga aylantiradi                     | Преобразует в байты big-endian                       |
// # |  33 | to_le_bytes()            | Little-endian baytlarga aylantiradi                  | Преобразует в байты little-endian                    |
// # |  34 | from_be_bytes()          | Big-endian baytlardan son yasaydi                    | Создаёт число из байтов big-endian                   |
// # |  35 | from_le_bytes()          | Little-endian baytlardan son yasaydi                 | Создаёт число из байтов little-endian                |
// # |  36 | to_string()              | Songa String yasaydi                                 | Преобразует число в String                           |
// # |  37 | parse()                  | String ni songa aylantiradi                          | Преобразует String в число                           |
// # == FLOAT ======================================================================================================================================#
// # |   1 | abs()                    | Mutlaq qiymat                                        | Абсолютное значение                                  |
// # |   2 | sqrt()                   | Kvadrat ildiz                                        | Квадратный корень                                    |
// # |   3 | cbrt()                   | Kub ildiz                                            | Кубический корень                                    |
// # |   4 | powf()                   | Float darajaga ko'taradi                             | Возводит в степень (float)                           |
// # |   5 | powi()                   | Butun son darajaga ko'taradi (tezroq)                | Возводит в целочисленную степень (быстрее)           |
// # |   6 | floor()                  | Pastga yaxlitlaydi                                   | Округляет вниз                                       |
// # |   7 | ceil()                   | Tepaga yaxlitlaydi                                   | Округляет вверх                                      |
// # |   8 | round()                  | Yaqiniga yaxlitlaydi                                 | Округляет до ближайшего                              |
// # |   9 | trunc()                  | Kasr qismini olib tashlaydi                          | Отбрасывает дробную часть                            |
// # |  10 | fract()                  | Faqat kasr qismini qaytaradi                         | Возвращает только дробную часть                      |
// # |  11 | min()                    | Kichigini qaytaradi (NaN xavfsiz)                    | Возвращает меньшее (безопасно для NaN)               |
// # |  12 | max()                    | Kattasini qaytaradi (NaN xavfsiz)                    | Возвращает большее (безопасно для NaN)               |
// # |  13 | clamp()                  | Sonni oraliqda ushlab turadi                         | Удерживает число в диапазоне                         |
// # |  14 | ln()                     | Natural logarifm (e asosida)                         | Натуральный логарифм (по основанию e)                |
// # |  15 | log2()                   | 2 asosidagi logarifm                                 | Логарифм по основанию 2                              |
// # |  16 | log10()                  | 10 asosidagi logarifm                                | Логарифм по основанию 10                             |
// # |  17 | log()                    | Ixtiyoriy asosidagi logarifm                         | Логарифм по произвольному основанию                  |
// # |  18 | exp()                    | e^x ni hisoblaydi                                    | Вычисляет e^x                                        |
// # |  19 | sin()                    | Sinusni hisoblaydi (radian)                          | Вычисляет синус (в радианах)                         |
// # |  20 | cos()                    | Kosinusni hisoblaydi (radian)                        | Вычисляет косинус (в радианах)                       |
// # |  21 | tan()                    | Tangensni hisoblaydi (radian)                        | Вычисляет тангенс (в радианах)                       |
// # |  22 | to_radians()             | Darajadan radianga                                   | Из градусов в радианы                                |
// # |  23 | to_degrees()             | Radianadan darajaga                                  | Из радиан в градусы                                  |
// # |  24 | signum()                 | Ishora: -1.0, 0.0, 1.0                               | Знак: -1.0, 0.0, 1.0                                 |
// # |  25 | hypot()                  | Gipotenuz: sqrt(x²+y²)                               | Гипотенуза: sqrt(x²+y²)                              |
// # |  26 | is_nan()                 | NaN ekanligini tekshiradi                            | Проверяет, является ли NaN                           |
// # |  27 | is_infinite()            | Cheksizligini tekshiradi                             | Проверяет, является ли бесконечным                   |
// # |  28 | is_finite()              | Chekliligini tekshiradi                              | Проверяет, является ли конечным                      |
// # |  29 | is_sign_positive()       | Musbat ishorali?                                     | Имеет положительный знак?                            |
// # |  30 | is_sign_negative()       | Manfiy ishorali?                                     | Имеет отрицательный знак?                            |
// # |  31 | recip()                  | Teskarisi: 1/x                                       | Обратное значение: 1/x                               |
// # |  32 | to_string()              | Sonni String ga aylantiradi                          | Преобразует число в String                           |
// # |  33 | parse()                  | String ni f64 ga aylantiradi                         | Преобразует String в f64                             |
// # |  34 | to_bits()                | f64 ni u64 bitlar ko'rinishida                       | f64 в виде битов u64                                 |
// # |  35 | from_bits()              | u64 bitlardan f64 yasaydi                            | Создаёт f64 из битов u64                             |
// #================================================================================================================================================#