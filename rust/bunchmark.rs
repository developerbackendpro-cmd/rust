// #================================================================================================================================================#
// #                                                            BENCHMARK (CRITERION)                                                               #
// #                        CRITERION — STATISTIK BENCHMARK. THROUGHPUT, COMPARISON, PLOT. CARGO-FLAMEGRAPH. PERF.                                  #
// #                        CRITERION — СТАТИСТИЧЕСКИЙ БЕНЧМАРК. THROUGHPUT, COMPARISON, PLOT. CARGO-FLAMEGRAPH. PERF.                              #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::time::{Duration, Instant};
use std::hint::black_box;

// Criterion nima:
// Что такое Criterion:
//
//   Statistik benchmark kutubxonasi
//   Статистическая библиотека бенчмарков
//
//   Cargo.toml:
//   [dev-dependencies]
//   criterion = { version = "0.5", features = ["html_reports"] }
//
//   [[bench]]
//   name = "mening_bench"
//   harness = false
//
//   Afzalliklari:
//   Преимущества:
//   ✅ Statistik tahlil (o'rtacha, standart og'ish)
//   ✅ Outlier aniqlash
//   ✅ Taqqoslama benchmark
//   ✅ HTML grafik hisobot
//   ✅ Throughput o'lchash
//   ✅ Regression aniqlash

fn criterion_haqiqiy_kod() {

    println!("=== CRITERION HAQIQIY KOD ===\n");

    println!(r#"// benches/mening_bench.rs

use criterion::{{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput}};

// 1. ODDIY BENCHMARK
pub fn faktorial_bench(c: &mut Criterion) {{
    c.bench_function("faktorial 10", |b| {{
        b.iter(|| faktorial(black_box(10)))
    }});
}}

// 2. PARAMETRLI BENCHMARK
pub fn fibonacci_bench(c: &mut Criterion) {{
    let mut group = c.benchmark_group("fibonacci");

    for n in [10u32, 15, 20, 25].iter() {{
        group.bench_with_input(BenchmarkId::new("rekursiv", n), n, |b, &n| {{
            b.iter(|| fib_rekursiv(black_box(n)))
        }});
        group.bench_with_input(BenchmarkId::new("iterativ", n), n, |b, &n| {{
            b.iter(|| fib_iterativ(black_box(n)))
        }});
        group.bench_with_input(BenchmarkId::new("memoized", n), n, |b, &n| {{
            b.iter(|| fib_memoized(black_box(n)))
        }});
    }}
    group.finish();
}}

// 3. THROUGHPUT BENCHMARK (MB/s)
pub fn sort_throughput_bench(c: &mut Criterion) {{
    let mut group = c.benchmark_group("sort");
    let ma_lumot: Vec<u64> = (0..10000).rev().collect();

    group.throughput(Throughput::Elements(ma_lumot.len() as u64));

    group.bench_function("sort_stable", |b| {{
        b.iter(|| {{
            let mut v = ma_lumot.clone();
            v.sort();
            black_box(v)
        }})
    }});

    group.bench_function("sort_unstable", |b| {{
        b.iter(|| {{
            let mut v = ma_lumot.clone();
            v.sort_unstable();
            black_box(v)
        }})
    }});

    group.finish();
}}

// 4. ITER_BATCHED — Setup bilan benchmark
pub fn hashmap_bench(c: &mut Criterion) {{
    use std::collections::HashMap;

    c.bench_function("hashmap_insert_1000", |b| {{
        b.iter_batched(
            // Setup (har iteratsiyada yangi map)
            || HashMap::new(),
            // Bench
            |mut map| {{
                for i in 0..1000u32 {{
                    map.insert(i, i * 2);
                }}
                black_box(map)
            }},
            criterion::BatchSize::SmallInput,
        )
    }});
}}

// 5. COMPARISON: Vec vs ArrayVec
pub fn vec_comparison_bench(c: &mut Criterion) {{
    let mut group = c.benchmark_group("vec_vs_arrayvec");
    group.plot_config(criterion::PlotConfiguration::default()
        .summary_scale(criterion::AxisScale::Logarithmic));

    for size in [10, 100, 1000].iter() {{
        group.bench_with_input(BenchmarkId::new("Vec", size), size, |b, &n| {{
            b.iter(|| {{
                let mut v: Vec<i32> = Vec::with_capacity(n);
                for i in 0..n {{ v.push(black_box(i as i32)); }}
                black_box(v)
            }})
        }});
    }}
    group.finish();
}}

criterion_group!(
    benches,
    faktorial_bench,
    fibonacci_bench,
    sort_throughput_bench,
    hashmap_bench,
    vec_comparison_bench,
);
criterion_main!(benches);

// ISHLATISH:
// cargo bench                          -- barcha benchmarklar
// cargo bench -- fibonacci             -- faqat fibonacci
// cargo bench -- fibonacci/iterativ    -- faqat iterativ
// cargo bench --baseline main          -- baseline saqlash
// cargo bench -- --save-baseline main  -- baseline yangilash"#);
}

#[derive(Debug)]
struct BenchNatija {
    nomi: String,
    o_rtacha_ns: f64,
    min_ns: f64,
    max_ns: f64,
    std_og_ish: f64,
    iteratsiyalar: usize,
}

impl BenchNatija {
    fn chiqar(&self) {
        println!("  {:<35} {:>10.2}ns  ±{:.2}ns  [min:{:.2} max:{:.2}]  ({} iter)",
                 self.nomi,
                 self.o_rtacha_ns,
                 self.std_og_ish,
                 self.min_ns,
                 self.max_ns,
                 self.iteratsiyalar,
        );
    }
}

fn bench<F: Fn() -> ()>(nomi: &str, iteratsiyalar: usize, f: F) -> BenchNatija {
    // Isitish (warmup)
    for _ in 0..100 { f(); }

    let mut o_lchashlar: Vec<f64> = Vec::with_capacity(iteratsiyalar);

    for _ in 0..iteratsiyalar {
        let t = Instant::now();
        f();
        o_lchashlar.push(t.elapsed().as_nanos() as f64);
    }

    let o_rtacha = o_lchashlar.iter().sum::<f64>() / iteratsiyalar as f64;
    let min = o_lchashlar.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = o_lchashlar.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let variance = o_lchashlar.iter().map(|&x| (x - o_rtacha).powi(2)).sum::<f64>() / iteratsiyalar as f64;
    let std = variance.sqrt();

    BenchNatija {
        nomi: nomi.to_string(),
        o_rtacha_ns: o_rtacha,
        min_ns: min,
        max_ns: max,
        std_og_ish: std,
        iteratsiyalar,
    }
}

fn bench_batch<F: Fn() -> ()>(nomi: &str, iteratsiyalar: usize, batch: usize, f: F) -> BenchNatija {
    for _ in 0..50 { f(); }
    let mut o_lchashlar: Vec<f64> = Vec::with_capacity(iteratsiyalar);
    for _ in 0..iteratsiyalar {
        let t = Instant::now();
        for _ in 0..batch { f(); }
        let ns = t.elapsed().as_nanos() as f64 / batch as f64;
        o_lchashlar.push(ns);
    }
    let o_rtacha = o_lchashlar.iter().sum::<f64>() / iteratsiyalar as f64;
    let min = o_lchashlar.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = o_lchashlar.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let variance = o_lchashlar.iter().map(|&x| (x - o_rtacha).powi(2)).sum::<f64>() / iteratsiyalar as f64;
    BenchNatija {
        nomi: nomi.to_string(),
        o_rtacha_ns: o_rtacha,
        min_ns: min,
        max_ns: max,
        std_og_ish: variance.sqrt(),
        iteratsiyalar,
    }
}

fn fib_rekursiv(n: u32) -> u64 {
    match n { 0 => 0, 1 => 1, _ => fib_rekursiv(n-1) + fib_rekursiv(n-2) }
}

fn fib_iterativ(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n { (a, b) = (b, a + b); }
    a
}

fn fib_memoized(n: u32) -> u64 {
    let mut memo = vec![0u64; (n + 1) as usize];
    memo[0] = 0; if n > 0 { memo[1] = 1; }
    for i in 2..=(n as usize) { memo[i] = memo[i-1] + memo[i-2]; }
    memo[n as usize]
}

fn fibonacci_bench_misoli() {

    println!("\n=== FIBONACCI ALGORITM TAQQOSLASH ===");
    let iteratsiyalar = 200;
    let n = 25u32;

    let r1 = bench_batch(&format!("Rekursiv fib({})", n), iteratsiyalar, 50,
                         || { black_box(fib_rekursiv(black_box(n))); });
    let r2 = bench_batch(&format!("Iterativ  fib({})", n), iteratsiyalar, 10000,
                         || { black_box(fib_iterativ(black_box(n))); });
    let r3 = bench_batch(&format!("Memoized  fib({})", n), iteratsiyalar, 10000,
                         || { black_box(fib_memoized(black_box(n))); });

    r1.chiqar(); r2.chiqar(); r3.chiqar();

    if r2.o_rtacha_ns > 0.0 {
        println!("  Tezlashuv (rekursiv/iterativ): {:.0}x",
                 r1.o_rtacha_ns / r2.o_rtacha_ns);
    }
}

fn sort_bench_misoli() {

    println!("\n=== SORT ALGORITM TAQQOSLASH ===");
    let n = 10_000usize;
    let iteratsiyalar = 100;

    // sort — stable
    let r1 = bench("Vec::sort (stable)", iteratsiyalar, || {
        let mut v: Vec<i32> = (0..n as i32).rev().collect();
        v.sort();
        black_box(v);
    });

    // sort_unstable — unstable, lekin tezroq
    let r2 = bench("Vec::sort_unstable", iteratsiyalar, || {
        let mut v: Vec<i32> = (0..n as i32).rev().collect();
        v.sort_unstable();
        black_box(v);
    });

    // sort_by — comparator bilan
    let r3 = bench("Vec::sort_by", iteratsiyalar, || {
        let mut v: Vec<i32> = (0..n as i32).rev().collect();
        v.sort_by(|a, b| a.cmp(b));
        black_box(v);
    });

    r1.chiqar(); r2.chiqar(); r3.chiqar();
}

fn iterator_vs_for_bench() {

    println!("\n=== ITERATOR VS FOR LOOP ===");
    let n = 1_000_000usize;
    let v: Vec<i32> = (0..n as i32).collect();
    let iteratsiyalar = 50;

    let r1 = bench("Iterator .sum()", iteratsiyalar, || {
        let s: i32 = v.iter().sum();
        black_box(s);
    });

    let r2 = bench("Iterator .filter().map().sum()", iteratsiyalar, || {
        let s: i32 = v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum();
        black_box(s);
    });

    let r3 = bench("For loop manualimpl", iteratsiyalar, || {
        let mut s = 0i32;
        for &x in &v { if x % 2 == 0 { s += x * x; } }
        black_box(s);
    });

    let r4 = bench("chunks(1024) parallel", iteratsiyalar, || {
        let s: i32 = v.chunks(1024)
            .map(|chunk| chunk.iter().filter(|&&x| x % 2 == 0).map(|&x| x * x).sum::<i32>())
            .sum();
        black_box(s);
    });

    r1.chiqar(); r2.chiqar(); r3.chiqar(); r4.chiqar();
}

fn map_bench_misoli() {

    println!("\n=== HASHMAP VS BTREEMAP ===");
    let n = 10_000usize;
    let iteratsiyalar = 50;
    use std::collections::{HashMap, BTreeMap};

    // Insert
    let r1 = bench("HashMap insert 10k", iteratsiyalar, || {
        let mut m: HashMap<u32, u32> = HashMap::with_capacity(n);
        for i in 0..n as u32 { m.insert(i, i * 2); }
        black_box(m);
    });

    let r2 = bench("BTreeMap insert 10k", iteratsiyalar, || {
        let mut m: BTreeMap<u32, u32> = BTreeMap::new();
        for i in 0..n as u32 { m.insert(i, i * 2); }
        black_box(m);
    });

    // Lookup
    let hmap: HashMap<u32, u32> = (0..n as u32).map(|i| (i, i*2)).collect();
    let bmap: BTreeMap<u32, u32> = (0..n as u32).map(|i| (i, i*2)).collect();

    let r3 = bench_batch("HashMap lookup", iteratsiyalar, 10000, || {
        black_box(hmap.get(&black_box(5000)));
    });

    let r4 = bench_batch("BTreeMap lookup", iteratsiyalar, 10000, || {
        black_box(bmap.get(&black_box(5000)));
    });

    r1.chiqar(); r2.chiqar(); r3.chiqar(); r4.chiqar();
    println!("  HashMap tezroq lookup, BTreeMap tartiblangan");
}

fn string_bench_misoli() {

    println!("\n=== STRING OPERATSIYALAR ===");
    let iteratsiyalar = 200;
    let n = 1000usize;

    let r1 = bench_batch("String::push_str (loop)", iteratsiyalar, 100, || {
        let mut s = String::new();
        for _ in 0..n { s.push_str("salom"); }
        black_box(s);
    });

    let r2 = bench_batch("String::with_capacity + push_str", iteratsiyalar, 100, || {
        let mut s = String::with_capacity(n * 5);
        for _ in 0..n { s.push_str("salom"); }
        black_box(s);
    });

    let r3 = bench_batch("format! concat", iteratsiyalar, 100, || {
        let parts: Vec<&str> = vec!["salom"; n];
        let s = parts.join("");
        black_box(s);
    });

    let r4 = bench_batch("String::from_utf8 (bytes)", iteratsiyalar, 100, || {
        let bytes: Vec<u8> = b"salom".repeat(n);
        let s = String::from_utf8(bytes).unwrap();
        black_box(s);
    });

    r1.chiqar(); r2.chiqar(); r3.chiqar(); r4.chiqar();
}

fn profiling_tushuntirish() {

    println!("\n=== FLAMEGRAPH VA PROFILING ===\n");

    println!("1. cargo-flamegraph:");
    println!("   cargo install flamegraph");
    println!("   cargo flamegraph --bench mening_bench -- fibonacci");
    println!("   # flamegraph.svg yaratiladi");
    println!();

    println!("2. perf (Linux):");
    println!("   perf record --call-graph dwarf cargo bench");
    println!("   perf report");
    println!();

    println!("3. Criterion HTML hisobot:");
    println!("   cargo bench");
    println!("   # target/criterion/report/index.html");
    println!();

    println!("4. iai-callgrind (instruction count based):");
    println!("   [dev-dependencies]");
    println!("   iai-callgrind = \"0.12\"");
    println!("   # Deterministik, CI uchun ideal");
    println!();

    println!("5. pprof (Go-like profiler):");
    println!("   [dev-dependencies]");
    println!("   criterion = {{ features = [\"html_reports\"] }}");
    println!("   pprof = {{ features = [\"flamegraph\"] }}");
    println!();

    println!("6. Samply / cargo-samply:");
    println!("   cargo install samply");
    println!("   cargo samply bench");
    println!("   # Firefox Profiler da ko'rish");
}

fn black_box_misoli() {

    println!("\n=== BLACK_BOX ===");

    // black_box — kompilyator optimizatsiyasini bloklash
    // Критично для benchmarking!

    // Muammo: kompilyator natijani optimizatsiya qilib yuborishi mumkin
    let t1 = Instant::now();
    let mut sum = 0i64;
    for i in 0..1_000_000i64 { sum += i; }
    // Kompilyator: "sum foydalanilmayapti → loop o'chir"
    let _vaqt_bad = t1.elapsed();

    // Yechim: black_box
    let t2 = Instant::now();
    let mut sum2 = 0i64;
    for i in 0..1_000_000i64 {
        sum2 += black_box(i); // black_box optimizatsiyani bloklaydi
    }
    black_box(sum2); // natijani ham bloklash
    let vaqt_good = t2.elapsed();

    println!("black_box bilan haqiqiy o'lchash: {:?}", vaqt_good);
    println!("black_box ni criterion da har doim ishlating!");
    println!();
    println!("std::hint::black_box(val) — nightly da ham ishlaydi");
    println!("criterion::black_box(val) — criterion ning o'zida");
}

fn main() {

    criterion_haqiqiy_kod();
    fibonacci_bench_misoli();
    sort_bench_misoli();
    iterator_vs_for_bench();
    map_bench_misoli();
    string_bench_misoli();
    black_box_misoli();
    profiling_tushuntirish();

    println!("\n=== XULOSA ===");
    println!("Benchmark qoidalari:");
    println!("  1. black_box — optimallashtirish oldini ol");
    println!("  2. Warmup — cache isitish uchun");
    println!("  3. Ko'p iteratsiya — statistik ishonchlilik");
    println!("  4. Release mode — cargo bench (avtomatik)");
    println!("  5. Realistic input — haqiqiy ma'lumot ishlat");
    println!("  6. Baseline saqlash — regression tracking");
    println!("  7. Throughput o'lchash — MB/s, ops/s");
    println!("  8. Flamegraph — hotspot topish");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        CRITERION                                                                                             |
// #================================================================================================================================================#
// # |   1 | c.bench_function(name, |b| ..)  | Oddiy benchmark                            | Простой бенчмарк                                        |
// # |   2 | c.benchmark_group(name)         | Guruhli benchmark                          | Групповой бенчмарк                                      |
// # |   3 | BenchmarkId::new(name, param)   | Parametrli ID                              | Параметризованный ID                                    |
// # |   4 | b.iter(|| ...)                  | Iteratsiya                                 | Итерация                                                |
// # |   5 | b.iter_batched(setup, bench, ..)| Setup bilan batch                          | Batch с настройкой                                      |
// # |   6 | Throughput::Elements(n)         | Throughput o'lchash                        | Измерение пропускной способности                        |
// # |   7 | html_reports feature            | HTML grafik hisobot                        | Графический HTML отчёт                                  |
// #================================================================================================================================================#
// # |                                        MUHIM TEXNIKALAR                                                                                      |
// #================================================================================================================================================#
// # |   8 | black_box(val)                  | Optimizatsiya oldini olish                 | Предотвращение оптимизации                              |
// # |   9 | criterion_group! + main!        | Benchmark guruhlarini ro'yxatlash          | Регистрация групп бенчмарков                            |
// # |  10 | --baseline / --save-baseline    | Regression tracking                        | Отслеживание регрессий                                  |
// # |  11 | cargo bench -- fibonacci        | Faqat ma'lum bench ishga tushirish         | Запуск конкретного бенчмарка                            |
// # |  12 | cargo flamegraph                | Flamegraph yaratish                        | Создание flamegraph                                     |
// #================================================================================================================================================#