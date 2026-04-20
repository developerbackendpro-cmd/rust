// #================================================================================================================================================#
// #                                                                ASM! + SIMD                                                                     #
// #                        ASM! — INLINE ASSEMBLY. SIMD — PARALLEL DATA PROCESSING. STD::ARCH. PORTABLE_SIMD.                                      #
// #                        ASM! — ВСТРОЕННЫЙ АССЕМБЛЕР. SIMD — ПАРАЛЛЕЛЬНАЯ ОБРАБОТКА ДАННЫХ. STD::ARCH. PORTABLE_SIMD.                            #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::time::Instant;

// ASM! nima:
// Что такое ASM!:
//
//   Rust ichida to'g'ridan assembly kodi yozish
//   Написание ассемблерного кода непосредственно в Rust
//   std::arch::asm! makrosi (Rust 1.59+ stable)
//
// SIMD nima:
// Что такое SIMD:
//
//   Single Instruction Multiple Data
//   Bir ko'rsatma bilan bir nechta ma'lumot
//   128/256/512 bit registrlar:
//   SSE  — 128 bit (4x f32 yoki 2x f64)
//   AVX  — 256 bit (8x f32 yoki 4x f64)
//   AVX-512 — 512 bit (16x f32 yoki 8x f64)
//   NEON — ARM da (4x f32 yoki 2x f64)

fn asm_asosiy_misoli() {

    println!("=== ASM! ASOSIY ===\n");

    // 1. Oddiy asm — register bilan
    // 1. Простой asm — с регистром
    let natija: u64;
    unsafe {
        std::arch::asm!(
        "mov rax, 42",   // registr to'g'ridan yoziladi
        out("rax") natija,
        );
    }
    println!("mov 42: {}", natija); // 42

    // 2. Qo'shish
    // 2. Сложение
    let a: u64 = 10;
    let b: u64 = 32;
    let yig: u64;
    unsafe {
        std::arch::asm!(
        "mov {yig}, {a}",
        "add {yig}, {b}",
        a = in(reg) a,
        b = in(reg) b,
        yig = out(reg) yig,
        );
    }
    println!("10 + 32 = {}", yig); // 42

    // 3. Ko'paytirish (mul)
    // 3. Умножение (mul)
    let x: u64 = 6;
    let y: u64 = 7;
    let natija2: u64;
    unsafe {
        std::arch::asm!(
        "mul {y}",           // rax * y → rdx:rax
        y = in(reg) y,
        inout("rax") x => natija2,
        out("rdx") _,        // rdx ni o'qimaymiz
        );
    }
    println!("6 * 7 = {}", natija2); // 42

    // 4. NOP va CPUID
    // 4. NOP и CPUID
    unsafe {
        std::arch::asm!("nop"); // Hech narsa qilmaydi — timing uchun
    }
    println!("nop bajarildi");

    // 5. Stack operatsiyalari
    // 5. Операции со стеком
    let val: u64 = 100;
    let res: u64;
    unsafe {
        std::arch::asm!(
        "push {v}",
        "pop {r}",
        v = in(reg) val,
        r = out(reg) res,
        );
    }
    println!("push/pop: {}", res); // 100
    // mov 42: 42
    // 10 + 32 = 42
    // 6 * 7 = 42
    // push/pop: 100
}

// CRC32 — hardware akseleratsiya simulyatsiya
// CRC32 — симуляция аппаратного ускорения
fn asm_crc32_simulyatsiya(ma_lumot: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;
    for &bayt in ma_lumot {
        crc ^= bayt as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
        }
    }
    !crc
}

// Bit count — popcount
fn asm_popcount_rust(n: u64) -> u32 {
    n.count_ones() // Rust builtin — compiler POPCNT ishlatadi
}

// Leading zeros
fn asm_lzcnt_rust(n: u64) -> u32 {
    n.leading_zeros()
}

// Swap bytes — bswap
fn asm_bswap(n: u32) -> u32 {
    n.swap_bytes() // Compiler bswap ishlatadi
}

fn asm_murakkab_misoli() {

    println!("\n=== ASM! MURAKKAB ===");

    // CRC32
    let ma_lumot = b"Salom Rust SIMD!";
    let crc = asm_crc32_simulyatsiya(ma_lumot);
    println!("CRC32: {:#010X}", crc);

    // Popcount — bit sanaydigan
    for n in [0u64, 1, 0xFF, 0xFFFF, u64::MAX] {
        println!("popcount({:#018X}) = {}", n, asm_popcount_rust(n));
    }
    // 0 → 0
    // 1 → 1
    // 0xFF → 8
    // 0xFFFF → 16
    // u64::MAX → 64

    // Leading zeros
    println!("\nlzcnt:");
    for n in [1u64, 2, 256, 1 << 32, 1 << 63] {
        println!("  lzcnt({:#018X}) = {}", n, asm_lzcnt_rust(n));
    }

    // Byte swap
    let original: u32 = 0x01020304;
    let swapped = asm_bswap(original);
    println!("\nbswap({:#010X}) = {:#010X}", original, swapped);
    // 0x01020304 → 0x04030201
}

fn simd_tushuntirish() {

    println!("\n=== SIMD TUSHUNTIRISH ===\n");

    println!(r#"// std::arch — Platform-specific SIMD intrinsics

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// SSE2 — 4 ta f32 parallel qo'shish
#[cfg(target_arch = "x86_64")]
unsafe fn sse2_qoshish(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {{
    let va = _mm_loadu_ps(a.as_ptr());  // 4x f32 yuklash
    let vb = _mm_loadu_ps(b.as_ptr());
    let vc = _mm_add_ps(va, vb);        // 4 ta qo'shish BITTA ko'rsatmada
    let mut result = [0.0f32; 4];
    _mm_storeu_ps(result.as_mut_ptr(), vc);
    result
}}

// AVX2 — 8 ta i32 parallel ko'paytirish
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn avx2_kopaytirish(a: &[i32; 8], b: &[i32; 8]) -> [i32; 8] {{
    let va = _mm256_loadu_si256(a.as_ptr() as *const __m256i);
    let vb = _mm256_loadu_si256(b.as_ptr() as *const __m256i);
    let vc = _mm256_mullo_epi32(va, vb); // 8 ta ko'paytirish BITTA
    let mut result = [0i32; 8];
    _mm256_storeu_si256(result.as_mut_ptr() as *mut __m256i, vc);
    result
}}

// Qanday tekshirish:
// is_x86_feature_detected!("sse2")   → bool
// is_x86_feature_detected!("avx2")   → bool
// is_x86_feature_detected!("avx512f")→ bool"#);
}

fn portable_simd_tushuntirish() {

    println!("\n=== PORTABLE SIMD (Nightly) ===\n");

    println!(r#"// Nightly: #![feature(portable_simd)]
use std::simd::{{f32x4, f32x8, i32x8, Simd}};

fn simd_yigindi(a: &[f32], b: &[f32]) -> Vec<f32> {{
    assert_eq!(a.len(), b.len());
    let mut natija = vec![0.0f32; a.len()];

    // SIMD bloklarda
    let blok = a.len() / 4;
    for i in 0..blok {{
        let va = f32x4::from_slice(&a[i*4..]);
        let vb = f32x4::from_slice(&b[i*4..]);
        let vc = va + vb;  // 4 ta qo'shish bitta!
        vc.copy_to_slice(&mut natija[i*4..]);
    }}

    // Qolgan elementlar
    for i in blok*4..a.len() {{
        natija[i] = a[i] + b[i];
    }}
    natija
}}

// Dot product — SIMD bilan
fn dot_product_simd(a: &[f32], b: &[f32]) -> f32 {{
    let mut yig = f32x8::splat(0.0);
    let chunks = a.len() / 8;

    for i in 0..chunks {{
        let va = f32x8::from_slice(&a[i*8..]);
        let vb = f32x8::from_slice(&b[i*8..]);
        yig += va * vb;  // fused multiply-add
    }}

    // SIMD dan skalyar
    yig.reduce_sum()
    + a[chunks*8..].iter().zip(&b[chunks*8..]).map(|(x,y)| x*y).sum::<f32>()
}}"#);
}

// SIMD ning scalar versiyasi — tushunish uchun
// Скалярная версия SIMD — для понимания

#[derive(Debug, Clone, Copy)]
struct F32x4(f32, f32, f32, f32);

impl F32x4 {
    fn new(a: f32, b: f32, c: f32, d: f32) -> Self { F32x4(a, b, c, d) }
    fn splat(v: f32) -> Self { F32x4(v, v, v, v) }
    fn from_slice(s: &[f32]) -> Self {
        F32x4(s[0], s[1], s[2], s[3])
    }

    fn to_array(self) -> [f32; 4] { [self.0, self.1, self.2, self.3] }

    fn reduce_sum(self) -> f32 { self.0 + self.1 + self.2 + self.3 }
    fn reduce_max(self) -> f32 { self.0.max(self.1).max(self.2).max(self.3) }
    fn reduce_min(self) -> f32 { self.0.min(self.1).min(self.2).min(self.3) }
}

impl std::ops::Add for F32x4 {
    type Output = Self;
    fn add(self, b: Self) -> Self { F32x4(self.0+b.0, self.1+b.1, self.2+b.2, self.3+b.3) }
}

impl std::ops::Mul for F32x4 {
    type Output = Self;
    fn mul(self, b: Self) -> Self { F32x4(self.0*b.0, self.1*b.1, self.2*b.2, self.3*b.3) }
}

impl std::ops::Sub for F32x4 {
    type Output = Self;
    fn sub(self, b: Self) -> Self { F32x4(self.0-b.0, self.1-b.1, self.2-b.2, self.3-b.3) }
}

// SIMD bilan tezlashtirilgan dot product simulyatsiya
fn dot_product_simd_sim(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    let n = a.len();
    let chunks = n / 4;

    let mut yig = F32x4::splat(0.0);
    for i in 0..chunks {
        let va = F32x4::from_slice(&a[i*4..]);
        let vb = F32x4::from_slice(&b[i*4..]);
        yig = yig + va * vb;
    }

    let mut scalar_yig = yig.reduce_sum();
    for i in chunks*4..n {
        scalar_yig += a[i] * b[i];
    }
    scalar_yig
}

// Oddiy dot product — solishtirish uchun
fn dot_product_oddiy(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

// Massiv qo'shish SIMD
fn massiv_qoshish_simd(a: &[f32], b: &[f32]) -> Vec<f32> {
    let n = a.len();
    let mut natija = vec![0.0f32; n];
    let chunks = n / 4;

    for i in 0..chunks {
        let va = F32x4::from_slice(&a[i*4..]);
        let vb = F32x4::from_slice(&b[i*4..]);
        let vc = va + vb;
        natija[i*4..i*4+4].copy_from_slice(&vc.to_array());
    }
    for i in chunks*4..n {
        natija[i] = a[i] + b[i];
    }
    natija
}

// Maksimal qiymat SIMD
fn maksimal_simd(v: &[f32]) -> f32 {
    if v.is_empty() { return f32::NEG_INFINITY; }
    let chunks = v.len() / 4;
    let mut maks = F32x4::splat(f32::NEG_INFINITY);

    for i in 0..chunks {
        let va = F32x4::from_slice(&v[i*4..]);
        maks = F32x4(
            maks.0.max(va.0), maks.1.max(va.1),
            maks.2.max(va.2), maks.3.max(va.3),
        );
    }

    let mut skalyar_maks = maks.reduce_max();
    for i in chunks*4..v.len() {
        skalyar_maks = skalyar_maks.max(v[i]);
    }
    skalyar_maks
}

fn simd_sim_misoli() {

    println!("\n=== SIMD SIMULYATSIYA (F32x4) ===");

    // F32x4 arifmetika
    let a = F32x4::new(1.0, 2.0, 3.0, 4.0);
    let b = F32x4::new(5.0, 6.0, 7.0, 8.0);

    println!("\nF32x4 arifmetika:");
    println!("  a = {:?}", a);
    println!("  b = {:?}", b);
    println!("  a+b = {:?}", (a + b).to_array());
    println!("  a*b = {:?}", (a * b).to_array());
    println!("  sum(a) = {}", a.reduce_sum());
    println!("  max(b) = {}", b.reduce_max());
    // a+b = [6.0, 8.0, 10.0, 12.0]
    // a*b = [5.0, 12.0, 21.0, 32.0]

    // Dot product taqqoslash
    let n = 1_000_000;
    let va: Vec<f32> = (0..n).map(|i| i as f32 * 0.001).collect();
    let vb: Vec<f32> = (0..n).map(|i| (n - i) as f32 * 0.001).collect();

    let t1 = Instant::now();
    let r1 = dot_product_oddiy(&va, &vb);
    let vaqt1 = t1.elapsed();

    let t2 = Instant::now();
    let r2 = dot_product_simd_sim(&va, &vb);
    let vaqt2 = t2.elapsed();

    println!("\nDot product ({} element):", n);
    println!("  Oddiy:  {:.3} → {:?}", r1, vaqt1);
    println!("  SIMD:   {:.3} → {:?}", r2, vaqt2);
    println!("  Bir xilmi: {}", (r1 - r2).abs() < 1.0);

    // Massiv qo'shish
    let a_kichik: Vec<f32> = (0..20).map(|i| i as f32).collect();
    let b_kichik: Vec<f32> = (0..20).map(|i| (i * 2) as f32).collect();
    let qoshilgan = massiv_qoshish_simd(&a_kichik, &b_kichik);
    println!("\nMassiv qo'shish (ilk 8): {:?}", &qoshilgan[..8]);
    // [0.0, 3.0, 6.0, 9.0, 12.0, 15.0, 18.0, 21.0]

    // Maksimal
    let v: Vec<f32> = vec![3.0, 7.0, 1.0, 9.0, 2.0, 5.0, 8.0, 4.0, 6.0];
    println!("Maksimal(simd): {}", maksimal_simd(&v)); // 9.0
}

fn auto_vektorization_misoli() {

    println!("\n=== AUTO-VEKTORIZATION ===\n");

    println!("Kompilyator SIMD optimallashtirish:");
    println!("  cargo build --release (O3 + LLVM vectorization)");
    println!("  RUSTFLAGS=\"-C target-cpu=native\" cargo build --release");
    println!();

    // Iterator bilan — kompilyator vectorize qiladi
    let n = 1_000_000usize;
    let a: Vec<f32> = (0..n).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..n).map(|i| (n - i) as f32).collect();

    // Bu kod kompilyator tomonidan SIMD ga o'giriladi
    // Этот код превращается в SIMD компилятором
    let t = Instant::now();
    let yig: f32 = a.iter().zip(b.iter()).map(|(x, y)| x + y).sum();
    println!("Auto-vectorized sum ({} el): {:?} → {:.0}", n, t.elapsed(), yig);

    // Chunks bilan — vectorization yaxshiroq
    let t2 = Instant::now();
    let yig2: f32 = a.chunks(4).zip(b.chunks(4))
        .map(|(ac, bc)| ac.iter().zip(bc.iter()).map(|(x, y)| x + y).sum::<f32>())
        .sum();
    println!("Chunks vectorized  ({} el): {:?} → {:.0}", n, t2.elapsed(), yig2);

    println!();
    println!("Target CPU xususiyatlari:");
    println!("  SSE2:   {}", is_x86_feature_detected!("sse2"));
    println!("  SSE4.1: {}", is_x86_feature_detected!("sse4.1"));
    println!("  AVX:    {}", is_x86_feature_detected!("avx"));
    println!("  AVX2:   {}", is_x86_feature_detected!("avx2"));
    println!("  POPCNT: {}", is_x86_feature_detected!("popcnt"));
}

fn asm_qoidalari() {

    println!("\n=== ASM! QOIDALARI ===\n");

    println!(r#"// 1. Kiritish/chiqarish operandlar:
//   in(reg) val    — kirish
//   out(reg) var   — chiqish
//   inout(reg) var — kiritish + chiqish

// 2. Clobbered registrlar (o'zgaradigan):
//   out("rax") _   — rax o'zgaradi, lekin o'qimaymiz

// 3. Options:
//   options(nostack) — stack ishlatmaymiz
//   options(pure)    — side effect yo'q
//   options(nomem)   — memory o'qimaymiz
//   options(readonly)— faqat o'qish

// 4. Labels:
unsafe {{
    std::arch::asm!(
        "xor eax, eax",
        "2:",                   // Label
        "inc eax",
        "cmp eax, 5",
        "jne 2b",              // 2-labelga qayt (b=backward)
        out("eax") _,
    );
}}

// 5. Memory operand:
let mut val = 42u32;
unsafe {{
    std::arch::asm!(
        "add dword ptr [rax], 8",
        in("rax") &mut val,
    );
}}
// val = 50

// 6. Platform tekshiruvi:
#[cfg(target_arch = "x86_64")]
fn faqat_x86() {{ /* x86_64 kod */ }}
#[cfg(target_arch = "aarch64")]
fn faqat_arm() {{ /* ARM kod */ }}"#);
}

fn main() {

    asm_asosiy_misoli();
    asm_murakkab_misoli();
    simd_tushuntirish();
    portable_simd_tushuntirish();
    simd_sim_misoli();
    auto_vektorization_misoli();
    asm_qoidalari();

    println!("\n=== XULOSA ===");
    println!("ASM! (Rust 1.59+ stable):");
    println!("  std::arch::asm! — inline assembly");
    println!("  in/out/inout — operandlar");
    println!("  options() — xulq-atvor");
    println!();
    println!("SIMD:");
    println!("  std::arch::x86_64::* — SSE/AVX intrinsics");
    println!("  is_x86_feature_detected! — runtime tekshiruv");
    println!("  Auto-vectorization — kompilyator yordami");
    println!("  portable_simd (nightly) — platformadan mustaqil");
    println!();
    println!("Qachon ishlatish:");
    println!("  Critical path — hotspot optimizatsiya");
    println!("  Crypto, ML, DSP — matematik operatsiyalar");
    println!("  Hardware feature — maxsus instruksiyalar");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        ASM!                                                                                                  |
// #================================================================================================================================================#
// # |   1 | std::arch::asm!("...")          | Inline assembly (1.59+ stable)             | Встроенный ассемблер (1.59+ stable)                     |
// # |   2 | in(reg) val                     | Kirish operand                             | Входной операнд                                         |
// # |   3 | out(reg) var                    | Chiqish operand                            | Выходной операнд                                        |
// # |   4 | inout(reg) var                  | Kirish+chiqish operand                     | Входной+выходной операнд                                |
// # |   5 | options(nostack, pure, nomem)   | Optimizatsiya ma'lumotlari                 | Информация для оптимизации                              |
// #================================================================================================================================================#
// # |                                        SIMD                                                                                                  |
// #================================================================================================================================================#
// # |   6 | std::arch::x86_64::*            | x86 SSE/AVX intrinsics                     | Интринсики x86 SSE/AVX                                  |
// # |   7 | #[target_feature(enable="avx2")]| Feature yoqish                             | Включение фичи                                          |
// # |   8 | is_x86_feature_detected!("avx") | Runtime feature tekshiruvi                 | Проверка фичи в runtime                                 |
// # |   9 | _mm_add_ps / _mm256_add_ps      | SSE/AVX qo'shish                           | Сложение SSE/AVX                                        |
// # |  10 | F32x4 simulyatsiya              | SIMD mantiqini tushunish                   | Понимание логики SIMD                                   |
// #================================================================================================================================================#
// # |                                        OPTIMALLASHTIRISH                                                                                     |
// #================================================================================================================================================#
// # |  11 | -C target-cpu=native            | CPU xususiyatlarini yoqish                 | Включение возможностей CPU                              |
// # |  12 | Auto-vectorization              | Kompilyator SIMD qiladi                    | Компилятор применяет SIMD                               |
// # |  13 | portable_simd (nightly)         | Platformadan mustaqil SIMD                 | SIMD не зависящий от платформы                          |
// # |  14 | chunks(N) + iter                | Auto-vect uchun qulay shakl                | Удобная форма для авто-векторизации                     |
// #================================================================================================================================================#