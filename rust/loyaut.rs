// #================================================================================================================================================#
// #                                                                MEMORY LAYOUT + #[REPR]                                                         #
// #                        MEMORY LAYOUT — STRUCT PADDING, ALIGNMENT. REPR(C, PACKED, TRANSPARENT, ALIGN). SIZE_OF, OFFSET_OF.                     #
// #                        MEMORY LAYOUT — ВЫРАВНИВАНИЕ STRUCT, ЗАПОЛНЕНИЕ. REPR(C, PACKED, TRANSPARENT, ALIGN). SIZE_OF, OFFSET_OF.               #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::mem;
use std::fmt;

// Memory Layout nima:
// Что такое Memory Layout:
//
//   Struct xotirada qanday joylashadi
//   Как структура располагается в памяти
//
//   Asosiy tushunchalar:
//   Основные понятия:
//   - size     — o'lcham (baytlar soni)
//   - align    — hizalanish (2^n bayt)
//   - padding  — bo'sh joy (hizalanish uchun)
//   - offset   — maydon boshlanish joyi
//
//   #[repr] variantlari:
//   Варианты #[repr]:
//   - repr(Rust)       — default, kompilyator optimizatsiya qiladi
//   - repr(C)          — C bilan mos, tartib kafolatlangan
//   - repr(packed)     — padding yo'q, kichik o'lcham
//   - repr(align(N))   — minimal N bayt hizalanish
//   - repr(transparent)— bitta maydon, wrapper
//   - repr(u8/u16/..)  — enum diskriminant o'lchami

fn default_rust_layout() {

    println!("=== DEFAULT RUST LAYOUT ===\n");

    // Rust — maydonlarni qayta tartiblaishi mumkin (optimizatsiya)
    // Rust — может переупорядочивать поля (оптимизация)
    struct A { a: u8, b: u32, c: u8 }
    struct B { b: u32, a: u8, c: u8 }
    struct C { a: u8, c: u8, b: u32 }

    println!("struct A {{ a: u8, b: u32, c: u8 }}");
    println!("  size:  {} bayt", mem::size_of::<A>());
    println!("  align: {} bayt", mem::align_of::<A>());
    println!();
    println!("struct B {{ b: u32, a: u8, c: u8 }}");
    println!("  size:  {} bayt", mem::size_of::<B>());
    println!("  align: {} bayt", mem::align_of::<B>());
    println!();
    println!("struct C {{ a: u8, c: u8, b: u32 }}");
    println!("  size:  {} bayt", mem::size_of::<C>());
    println!("  align: {} bayt", mem::align_of::<C>());
    // Rust da A,B,C barchasi 8 yoki 6 bo'lishi mumkin
    // Kompilyator optimizatsiya qiladi

    println!();
    // Primitiv turlar
    println!("Primitiv turlar:");
    println!("  bool:    size={}, align={}", mem::size_of::<bool>(),  mem::align_of::<bool>());
    println!("  u8:      size={}, align={}", mem::size_of::<u8>(),    mem::align_of::<u8>());
    println!("  u16:     size={}, align={}", mem::size_of::<u16>(),   mem::align_of::<u16>());
    println!("  u32:     size={}, align={}", mem::size_of::<u32>(),   mem::align_of::<u32>());
    println!("  u64:     size={}, align={}", mem::size_of::<u64>(),   mem::align_of::<u64>());
    println!("  u128:    size={}, align={}", mem::size_of::<u128>(),  mem::align_of::<u128>());
    println!("  usize:   size={}, align={}", mem::size_of::<usize>(), mem::align_of::<usize>());
    println!("  f32:     size={}, align={}", mem::size_of::<f32>(),   mem::align_of::<f32>());
    println!("  f64:     size={}, align={}", mem::size_of::<f64>(),   mem::align_of::<f64>());
    println!("  char:    size={}, align={}", mem::size_of::<char>(),  mem::align_of::<char>());
    println!();
    // Pointer va reference
    println!("Pointer/Reference:");
    println!("  *const u8:   size={}", mem::size_of::<*const u8>());
    println!("  &u8:         size={}", mem::size_of::<&u8>());
    println!("  &str:        size={} (fat ptr)", mem::size_of::<&str>());
    println!("  &[u8]:       size={} (fat ptr)", mem::size_of::<&[u8]>());
    println!("  Box<u8>:     size={}", mem::size_of::<Box<u8>>());
    println!("  Option<&u8>: size={} (NPO!)", mem::size_of::<Option<&u8>>());
}

fn repr_c_misoli() {

    println!("\n=== REPR(C) ===\n");

    // repr(C) — C bilan bir xil tartib va padding
    // repr(C) — тот же порядок и заполнение как в C

    // C da:
    // struct S { char a; int b; char c; }
    // Hajm: 12 bayt (padding bilan)
    //   [a][pad,pad,pad][b,b,b,b][c][pad,pad,pad]

    #[repr(C)]
    struct Sx { a: u8, b: u32, c: u8 }

    // repr(Rust) da Rust optimizatsiya qilishi mumkin
    struct Sy { a: u8, b: u32, c: u8 }

    println!("repr(C)    {{ a: u8, b: u32, c: u8 }}: {} bayt", mem::size_of::<Sx>());
    println!("repr(Rust) {{ a: u8, b: u32, c: u8 }}: {} bayt", mem::size_of::<Sy>());
    // repr(C):    12 bayt
    // repr(Rust): 8 bayt (optimizatsiya)

    println!("\noffset_of! (repr(C)):");
    println!("  a offset: {}", mem::offset_of!(Sx, a)); // 0
    println!("  b offset: {}", mem::offset_of!(Sx, b)); // 4
    println!("  c offset: {}", mem::offset_of!(Sx, c)); // 8
    // a offset: 0
    // b offset: 4
    // c offset: 8

    // Nested repr(C)
    #[repr(C)]
    struct Inner { x: f32, y: f32 }

    #[repr(C)]
    struct Outer { id: u32, pos: Inner, active: bool }

    println!("\nrepr(C) nested:");
    println!("  Inner: {} bayt", mem::size_of::<Inner>());
    println!("  Outer: {} bayt", mem::size_of::<Outer>());
    println!("  id offset:     {}", mem::offset_of!(Outer, id));
    println!("  pos offset:    {}", mem::offset_of!(Outer, pos));
    println!("  active offset: {}", mem::offset_of!(Outer, active));
    // Inner: 8 bayt
    // Outer: 16 bayt (12 + 1 + 3 padding)

    // FFI uchun repr(C) zarur
    // repr(C) необходим для FFI
    #[repr(C)]
    struct CApiRequest {
        version: u32,
        flags: u16,
        payload_len: u16,
        payload: *const u8,
    }

    println!("\nFFI struct CApiRequest: {} bayt", mem::size_of::<CApiRequest>());
    // 16 bayt (u32 + u16 + u16 + ptr(8))
}

fn repr_packed_misoli() {

    println!("\n=== REPR(PACKED) ===\n");

    // repr(packed) — padding yo'q, lekin unaligned access mumkin
    // repr(packed) — нет заполнения, но возможен unaligned access

    #[repr(C)]
    struct Normal { a: u8, b: u32, c: u16 }

    #[repr(C, packed)]
    struct Packed { a: u8, b: u32, c: u16 }

    #[repr(C, packed(2))]
    struct Packed2 { a: u8, b: u32, c: u16 } // 2 bayt hizalanish

    println!("Normal (C):     {} bayt", mem::size_of::<Normal>());  // 8
    println!("Packed:         {} bayt", mem::size_of::<Packed>());  // 7
    println!("Packed(2):      {} bayt", mem::size_of::<Packed2>()); // 8

    // Xavfli: packed struct maydoniga reference olish
    // Опасно: получение ссылки на поле packed struct
    let p = Packed { a: 1, b: 0x01020304, c: 0xABCD };

    // ptr::addr_of! — xavfsiz yo'l
    let b_ptr = std::ptr::addr_of!(p.b);
    let b_val = unsafe { std::ptr::read_unaligned(b_ptr) };
    println!("Packed.b = {:#010X}", b_val); // 0x01020304
    // Packed.b = 0x01020304

    // Tarmoq paketi — packed ideal
    // Сетевой пакет — packed идеален
    #[repr(C, packed)]
    struct TarmoqPaketi {
        versiya: u8,    // 0
        tur: u8,        // 1
        uzunlik: u16,   // 2-3
        src_port: u16,  // 4-5
        dst_port: u16,  // 6-7
        checksum: u32,  // 8-11
        ma_lumot: [u8; 4], // 12-15
    }

    println!("\nTarmoqPaketi: {} bayt (padding yo'q!)", mem::size_of::<TarmoqPaketi>());
    // 16 bayt — aniq 16 (padding yo'q)

    let paket = TarmoqPaketi {
        versiya: 1, tur: 2, uzunlik: 16,
        src_port: 8080, dst_port: 443,
        checksum: 0xDEADBEEF,
        ma_lumot: [0xAA, 0xBB, 0xCC, 0xDD],
    };

    let bytes: &[u8] = unsafe {
        std::slice::from_raw_parts(
            &paket as *const _ as *const u8,
            mem::size_of::<TarmoqPaketi>()
        )
    };
    println!("Paket bytes: {:02X?}", bytes);
    // [01, 02, 10, 00, 90, 1F, BB, 01, EF, BE, AD, DE, AA, BB, CC, DD]
}

fn repr_align_misoli() {

    println!("\n=== REPR(ALIGN(N)) ===\n");

    // False sharing oldini olish — cache line (64 bayt) hizalanish
    // Предотвращение false sharing — выравнивание по cache line (64 байта)

    // Oddiy AtomicUsize — cache line larda overlap bo'lishi mumkin
    use std::sync::atomic::AtomicUsize;
    struct OddiyHisob { qiymat: AtomicUsize }

    // Cache-padded — har biri o'z cache line da
    #[repr(align(64))]
    struct CachePadded { qiymat: AtomicUsize }

    println!("OddiyHisob size:    {} bayt", mem::size_of::<OddiyHisob>());
    println!("OddiyHisob align:   {} bayt", mem::align_of::<OddiyHisob>());
    println!("CachePadded size:   {} bayt", mem::size_of::<CachePadded>());
    println!("CachePadded align:  {} bayt", mem::align_of::<CachePadded>());
    // OddiyHisob size:    8 bayt
    // OddiyHisob align:   8 bayt
    // CachePadded size:   64 bayt
    // CachePadded align:  64 bayt

    // SIMD uchun hizalanish
    #[repr(align(32))]
    struct Avx2Vektor { ma_lumot: [f32; 8] } // 256-bit AVX2

    println!("\nAvx2Vektor size:  {} bayt", mem::size_of::<Avx2Vektor>());
    println!("Avx2Vektor align: {} bayt", mem::align_of::<Avx2Vektor>());
    // 32 bayt, align 32

    // Massiv uchun hizalanish
    let cache1 = CachePadded { qiymat: AtomicUsize::new(0) };
    let cache2 = CachePadded { qiymat: AtomicUsize::new(0) };

    let ptr1 = &cache1 as *const _ as usize;
    let ptr2 = &cache2 as *const _ as usize;
    println!("\ncache1 manzil: {:#x} (64ga bo'linadi: {})", ptr1, ptr1 % 64 == 0);
    println!("cache2 manzil: {:#x} (64ga bo'linadi: {})", ptr2, ptr2 % 64 == 0);

    // repr(align) + repr(packed) birgalikda
    #[repr(C, align(16))]
    struct Aligned16 { a: u8, b: u8 }

    println!("\nAligned16: size={}, align={}",
             mem::size_of::<Aligned16>(), mem::align_of::<Aligned16>());
    // size=16, align=16
}

fn repr_transparent_misoli() {

    println!("\n=== REPR(TRANSPARENT) ===\n");

    // repr(transparent) — bitta maydon, ABI mos
    // repr(transparent) — одно поле, совместимость ABI

    // Newtype pattern + FFI
    #[repr(transparent)]
    struct Metr(f64);

    #[repr(transparent)]
    struct Kilogramm(f64);

    // Bir xil ABI — C da f64 sifatida ko'rinadi
    println!("Metr:      size={}, align={}", mem::size_of::<Metr>(), mem::align_of::<Metr>());
    println!("Kilogramm: size={}, align={}", mem::size_of::<Kilogramm>(), mem::align_of::<Kilogramm>());
    println!("f64:       size={}, align={}", mem::size_of::<f64>(), mem::align_of::<f64>());
    // Barchasi 8 bayt, align 8 — bir xil!

    // NonNull<T> — repr(transparent) bilan
    // Pointer wrapper — C ga f64* sifatida ko'rinadi
    #[repr(transparent)]
    struct SafePtr<T>(std::ptr::NonNull<T>);

    println!("SafePtr<i32>: size={}", mem::size_of::<SafePtr<i32>>());
    println!("*mut i32:     size={}", mem::size_of::<*mut i32>());
    // Bir xil!

    // repr(transparent) bilan transmute xavfsizroq
    // transmute безопаснее с repr(transparent)
    #[repr(transparent)]
    struct MyBool(u8);

    let t = MyBool(1u8);
    let raw: u8 = unsafe { mem::transmute(t) };
    println!("MyBool(1) → u8: {}", raw); // 1
    // 1

    // Enum uchun transparent (ZST)
    // transparent для enum (ZST)
    // #[repr(transparent)]
    // enum Wrapper<T> { Value(T) }
    // — faqat bitta non-ZST variant bo'lsa ishlaydi

    println!("\nrepr(transparent) qoidalari:");
    println!("  - Faqat bitta non-ZST maydon bo'lishi kerak");
    println!("  - Qolgan maydonlar ZST (PhantomData kabi)");
    println!("  - ABI wrapper uchun ideal (FFI + newtype)");
}

fn enum_repr_misoli() {

    println!("\n=== ENUM REPR ===\n");

    // Default enum — kompilyator tanlaydi
    enum E0 { A, B, C }

    // repr(u8) — diskriminant u8
    #[repr(u8)]
    enum E8 { A = 0, B = 1, C = 2 }

    // repr(u16)
    #[repr(u16)]
    enum E16 { A = 0, B = 1000, C = 65535 }

    // repr(C) — C enum kabi
    #[repr(C)]
    enum EC { A, B, C }

    println!("enum E0 (default): {} bayt", mem::size_of::<E0>());
    println!("enum E8  (u8):     {} bayt", mem::size_of::<E8>());
    println!("enum E16 (u16):    {} bayt", mem::size_of::<E16>());
    println!("enum EC  (C):      {} bayt", mem::size_of::<EC>());
    // E0: 1 bayt (Rust optimizatsiya)
    // E8: 1 bayt
    // E16: 2 bayt
    // EC: 4 bayt (C int kabi)

    // Data bilan enum
    #[repr(u8)]
    #[derive(Debug)]
    enum Xabar {
        Salom = 1,
        Nomer(u32) = 2,
        Matn([u8; 8]) = 3,
    }

    println!("\nXabar: {} bayt", mem::size_of::<Xabar>());
    // 9 bayt (1 tag + 8 ma'lumot)

    // Option optimizatsiya — NPO (Null Pointer Optimization)
    println!("\nNPO (Null Pointer Optimization):");
    println!("Option<&u8>:       {} bayt", mem::size_of::<Option<&u8>>());
    println!("&u8:               {} bayt", mem::size_of::<&u8>());
    // Option<&u8> == &u8 (8 bayt) — NPO!

    println!("Option<Box<u8>>:   {} bayt", mem::size_of::<Option<Box<u8>>>());
    println!("Box<u8>:           {} bayt", mem::size_of::<Box<u8>>());
    // Bir xil!

    println!("Option<NonZeroU32>:{} bayt", mem::size_of::<Option<std::num::NonZeroU32>>());
    println!("NonZeroU32:        {} bayt", mem::size_of::<std::num::NonZeroU32>());
    // Bir xil!

    // repr(C) bilan union-style enum
    #[repr(C, u32)]
    #[derive(Debug)]
    enum CTaggedUnion {
        Int(u32),
        Float(f32),
        Bool(bool),
    }

    println!("\nCTaggedUnion: {} bayt", mem::size_of::<CTaggedUnion>());
}

fn padding_vizualizatsiya() {

    println!("\n=== PADDING VIZUALIZATSIYA ===\n");

    fn vizualizatsiya<T>(nomi: &str) where T: Sized {
        let size = mem::size_of::<T>();
        let align = mem::align_of::<T>();
        println!("{}: size={}, align={}", nomi, size, align);
        println!("  [{}]", "█".repeat(size));
    }

    #[repr(C)] struct S1 { a: u8, b: u64, c: u8 }
    #[repr(C)] struct S2 { b: u64, a: u8, c: u8 }
    #[repr(C, packed)] struct S3 { a: u8, b: u64, c: u8 }

    vizualizatsiya::<S1>("repr(C) { u8, u64, u8 }");
    // 24 bayt: [a][ppp ppp p][bbbbbbbb][c][ppp ppp p]

    vizualizatsiya::<S2>("repr(C) { u64, u8, u8 }");
    // 16 bayt: [bbbbbbbb][a][c][pppppp]

    vizualizatsiya::<S3>("repr(C,packed) { u8, u64, u8 }");
    // 10 bayt: [a][bbbbbbbb][c]

    // Optimal tartib — katta maydon avval
    println!("\nOptimal struct tartibi (katta → kichik):");
    #[repr(C)] struct Optimal { d: u64, c: u32, b: u16, a: u8, e: u8 }
    vizualizatsiya::<Optimal>("Optimal { u64, u32, u16, u8, u8 }");
    // 16 bayt — minimal padding

    println!("\nOffset jadval:");
    println!("  d offset: {}", mem::offset_of!(Optimal, d));
    println!("  c offset: {}", mem::offset_of!(Optimal, c));
    println!("  b offset: {}", mem::offset_of!(Optimal, b));
    println!("  a offset: {}", mem::offset_of!(Optimal, a));
    println!("  e offset: {}", mem::offset_of!(Optimal, e));
    // 0, 8, 12, 14, 15
}

fn real_hayot_misollari() {

    println!("\n=== REAL HAYOT ===\n");

    // 1. ECS (Entity Component System) — cache-friendly layout
    // 1. ECS — компоновка дружественная к кэшу
    println!("--- AoS vs SoA ---");

    // Array of Structs (AoS) — eski usul
    #[repr(C)]
    struct AoSJismi { x: f32, y: f32, z: f32, massa: f32 }
    // [x,y,z,m][x,y,z,m][x,y,z,m]...

    // Structure of Arrays (SoA) — cache friendly
    struct SoAJismlar {
        x: Vec<f32>, y: Vec<f32>, z: Vec<f32>, massa: Vec<f32>
    }
    // [x,x,x,...][y,y,y,...][z,z,z,...][m,m,m,...]

    let n = 1_000_000;
    let mut soa = SoAJismlar {
        x: vec![0.0f32; n], y: vec![0.0f32; n],
        z: vec![0.0f32; n], massa: vec![1.0f32; n],
    };

    let t = std::time::Instant::now();
    let jami_massa: f32 = soa.massa.iter().sum();
    println!("SoA jami_massa ({} element): {:?}", n, t.elapsed());

    // 2. Xotira tejash — enum size optimallashtirish
    println!("\n--- Enum Size Optimallashtirish ---");

    #[derive(Debug)]
    enum Katta { A(u64), B(u64), C(u64), D(u64) }

    #[derive(Debug)]
    enum Kichik { A(u32), B(u32), C(u32), D(u32) }

    #[repr(u8)]
    #[derive(Debug)]
    enum Minimal { A(u32), B(u32), C(u32), D(u32) }

    println!("Katta  enum: {} bayt", mem::size_of::<Katta>());
    println!("Kichik enum: {} bayt", mem::size_of::<Kichik>());
    println!("Minimal enum: {} bayt", mem::size_of::<Minimal>());

    // 3. Inline assembly uchun hizalanish
    println!("\n--- SIMD hizalanish ---");
    #[repr(align(16))]
    struct Sse128 { ma_lumot: [f32; 4] }

    #[repr(align(32))]
    struct Avx256 { ma_lumot: [f32; 8] }

    println!("SSE  128-bit: size={}, align={}", mem::size_of::<Sse128>(), mem::align_of::<Sse128>());
    println!("AVX  256-bit: size={}, align={}", mem::size_of::<Avx256>(), mem::align_of::<Avx256>());

    let sse = Sse128 { ma_lumot: [1.0, 2.0, 3.0, 4.0] };
    let addr = &sse as *const _ as usize;
    println!("SSE manzil 16ga bo'linadi: {}", addr % 16 == 0);
}

fn main() {

    default_rust_layout();
    repr_c_misoli();
    repr_packed_misoli();
    repr_align_misoli();
    repr_transparent_misoli();
    enum_repr_misoli();
    padding_vizualizatsiya();
    real_hayot_misollari();

    println!("\n=== XULOSA ===");
    println!("#[repr] variantlari:");
    println!("  repr(Rust)        — default, optimal o'lcham");
    println!("  repr(C)           — FFI, C bilan mos");
    println!("  repr(packed)      — minimal o'lcham, unaligned");
    println!("  repr(align(N))    — katta hizalanish, cache/SIMD");
    println!("  repr(transparent) — wrapper, bitta maydon");
    println!("  repr(u8/u16/..)   — enum diskriminant o'lchami");
    println!();
    println!("Qoidalar:");
    println!("  - FFI → repr(C) ZARUR");
    println!("  - Tarmoq protokoli → repr(C, packed)");
    println!("  - False sharing → repr(align(64))");
    println!("  - SIMD → repr(align(16/32/64))");
    println!("  - Newtype FFI → repr(transparent)");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        REPR VARIANTLARI                                                                                      |
// #================================================================================================================================================#
// # |   1 | #[repr(Rust)]                   | Default, kompilyator optimizatsiya         | По умолчанию, оптимизация компилятора                   |
// # |   2 | #[repr(C)]                      | C bilan mos, tartib kafolatlangan          | Совместимость с C, порядок гарантирован                 |
// # |   3 | #[repr(packed)]                 | Padding yo'q, kichik o'lcham               | Без заполнения, меньший размер                          |
// # |   4 | #[repr(C, packed(N))]           | N bayt hizalanish bilan packed             | packed с выравниванием N байт                           |
// # |   5 | #[repr(align(N))]               | Minimal N bayt hizalanish                  | Минимальное выравнивание N байт                         |
// # |   6 | #[repr(transparent)]            | Bitta maydon, wrapper, ABI mos             | Одно поле, обёртка, совместимость ABI                   |
// # |   7 | #[repr(u8/u16/u32/u64)]         | Enum diskriminant o'lchami                 | Размер дискриминанта enum                               |
// # |   8 | #[repr(C, u32)]                 | C-like tagged union enum                   | Enum как tagged union в C                               |
// #================================================================================================================================================#
// # |                                        YORDAMCHI FUNKSIYALAR                                                                                 |
// #================================================================================================================================================#
// # |   9 | mem::size_of::<T>()             | T o'lchami baytlarda                       | Размер T в байтах                                       |
// # |  10 | mem::align_of::<T>()            | T hizalanish talabi                        | Требование выравнивания T                               |
// # |  11 | mem::offset_of!(T, field)       | Field offseti (1.77+)                      | Смещение поля (1.77+)                                   |
// # |  12 | ptr::addr_of!(val.field)        | Packed field manzili (xavfsiz)             | Адрес поля packed (безопасно)                           |
// # |  13 | ptr::read_unaligned(ptr)        | Unaligned o'qish                           | Чтение без выравнивания                                 |
// #================================================================================================================================================#
// # |                                        OPTIMIZATSIYA                                                                                         |
// #================================================================================================================================================#
// # |  14 | NPO                             | Option<&T> == &T (null pointer opt)        | Option<&T> == &T (оптимизация нулевого ptr)             |
// # |  15 | SoA vs AoS                      | Cache-friendly layout                      | Компоновка дружественная к кэшу                         |
// # |  16 | Katta maydon avval              | Minimal padding                            | Минимальное заполнение                                  |
// #================================================================================================================================================#