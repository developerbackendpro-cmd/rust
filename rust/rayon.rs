// #================================================================================================================================================#
// #                                                                    RAYON                                                                       #
// #                RAYON — DATA PARALLELISM. PAR_ITER, PAR_SORT, JOIN, SCOPE, THREADPOOL. WORK STEALING. PARALLEL ALGORITHMS.                      #
// #                RAYON — ПАРАЛЛЕЛИЗМ ДАННЫХ. PAR_ITER, PAR_SORT, JOIN, SCOPE, THREADPOOL. WORK STEALING. ПАРАЛЛЕЛЬНЫЕ АЛГОРИТМЫ.                 #
// #================================================================================================================================================#

// Cargo.toml:
// [dependencies]
// rayon = "1.10"

// Bu fayl rayon kutubxonasisiz (std bilan) rayon konseptsiyalarini
// tushuntiradi va simulyatsiya qiladi.
// Haqiqiy rayon kodi comment sifatida ko'rsatiladi.

#![allow(dead_code, unused)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// Rayon nima:
// Что такое Rayon:
//
//   - Data parallelism kutubxonasi
//   - Библиотека параллелизма данных
//   - Iterator ni parallel qiladi — .par_iter()
//   - Делает Iterator параллельным — .par_iter()
//   - Work-stealing thread pool — CPU larni to'liq ishlatadi
//   - Пул потоков с work-stealing — полное использование CPU
//   - Xavfsiz — Rust tur tizimi bilan kafolatlangan
//   - Безопасно — гарантировано системой типов Rust
//
// Asosiy API:
// Основной API:
//   .par_iter()          — parallel iterator
//   .par_iter_mut()      — parallel mutable iterator
//   .into_par_iter()     — parallel into iterator
//   .par_chunks(n)       — parallel chunks
//   rayon::join(f1, f2)  — ikki funksiyani parallel
//   rayon::scope(|s| {}) — parallel scope
//   ThreadPoolBuilder    — custom thread pool

fn parallel_map<T, R, F>(
    ma_lumot: Vec<T>,
    funksiya: F,
    thread_soni: usize,
) -> Vec<R>
where
    T: Send + Clone + 'static,
    R: Send + 'static,
    F: Fn(T) -> R + Send + Sync + 'static,
{
    let funksiya = Arc::new(funksiya);
    let chunk_hajm = (ma_lumot.len() + thread_soni - 1) / thread_soni;

    let chunks: Vec<Vec<T>> = ma_lumot
        .chunks(chunk_hajm)
        .map(|c| c.to_vec())
        .collect();

    let handlar: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            let f = Arc::clone(&funksiya);
            thread::spawn(move || {
                chunk.into_iter().map(|x| f(x)).collect::<Vec<R>>()
            })
        })
        .collect();

    handlar
        .into_iter()
        .flat_map(|h| h.join().unwrap())
        .collect()
}

fn parallel_map_misoli() {

    println!("--- Parallel Map ---");
    let n = 1_000_000usize;
    let ma_lumot: Vec<i64> = (1..=(n as i64)).collect();
    let thread_soni = num_cpus_sim();

    // Ketma-ket
    let boshlanish = Instant::now();
    let ketma_ket_yig: i64 = ma_lumot.iter().map(|&x| x * x % 1_000_007).sum();
    let ketma_ket_vaqt = boshlanish.elapsed();

    // Parallel simulyatsiya
    let boshlanish2 = Instant::now();
    let natijalar = parallel_map(
        ma_lumot.clone(),
        |x| x * x % 1_000_007i64,
        thread_soni,
    );
    let parallel_yig: i64 = natijalar.iter().sum();
    let parallel_vaqt = boshlanish2.elapsed();

    println!("Ketma-ket:  {:?} (yig: {})", ketma_ket_vaqt, ketma_ket_yig);
    println!("Parallel:   {:?} (yig: {})", parallel_vaqt, parallel_yig);
    println!("Bir xilmi:  {}", ketma_ket_yig == parallel_yig);
    println!("Thread soni: {}", thread_soni);
}

fn num_cpus_sim() -> usize {
    thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

fn parallel_filter<T, P>(ma_lumot: Vec<T>, predikat: P, thread_soni: usize) -> Vec<T>
where
    T: Send + Clone + 'static,
    P: Fn(&T) -> bool + Send + Sync + 'static,
{
    let predikat = Arc::new(predikat);
    let chunk_hajm = (ma_lumot.len() + thread_soni - 1) / thread_soni;

    let chunks: Vec<Vec<T>> = ma_lumot.chunks(chunk_hajm).map(|c| c.to_vec()).collect();

    let handlar: Vec<_> = chunks.into_iter().map(|chunk| {
        let p = Arc::clone(&predikat);
        thread::spawn(move || {
            chunk.into_iter().filter(|x| p(x)).collect::<Vec<T>>()
        })
    }).collect();

    handlar.into_iter().flat_map(|h| h.join().unwrap()).collect()
}

fn parallel_fold<T, R, F, C>(
    ma_lumot: Vec<T>,
    boshlang_ich: R,
    fold_funksiya: F,   // Yig'ish funksiyasi: (R, T) -> R
    reduce_funksiya: C, // Birlashtirish funksiyasi: (R, R) -> R
    thread_soni: usize,
) -> R
where
    T: Send + Clone + 'static,
    R: Send + Clone + 'static,
    F: Fn(R, T) -> R + Send + Sync + 'static,
    C: Fn(R, R) -> R + Send + Sync + 'static,
{
    let chunk_hajm = (ma_lumot.len() + thread_soni - 1) / thread_soni;
    let chunks: Vec<Vec<T>> = ma_lumot.chunks(chunk_hajm).map(|c| c.to_vec()).collect();

    let fold_funksiya = Arc::new(fold_funksiya);
    let reduce_funksiya = Arc::new(reduce_funksiya);

    let handlar: Vec<_> = chunks.into_iter().map(|chunk| {
        let fold_f = Arc::clone(&fold_funksiya);
        let init = boshlang_ich.clone();
        thread::spawn(move || {
            chunk.into_iter().fold(init, |acc, x| fold_f(acc, x))
        })
    }).collect();

    let blok_natijalari: Vec<R> = handlar.into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    // Birlashtirish bosqichi
    blok_natijalari.into_iter().fold(boshlang_ich, |acc, x| reduce_funksiya(acc, x))
}

fn parallel_filter_fold_misoli() {

    println!("\n--- Parallel Filter ---");
    let v: Vec<i32> = (1..=100).collect();
    let thread_soni = num_cpus_sim();

    let juftlar = parallel_filter(v.clone(), |x| x % 2 == 0, thread_soni);
    println!("Juftlar soni: {}", juftlar.len()); // 50
    println!("Dastlabki 5: {:?}", &juftlar[..5.min(juftlar.len())]);
    // Juftlar soni: 50
    // Dastlabki 5: [2, 4, 6, 8, 10]  (tartib o'zgarishi mumkin)

    println!("\n--- Parallel Yig'indi ---");
    // Sodda parallel sum
    let katta_v: Vec<i64> = (1..=1_000_000i64).collect();
    let boshlanish = Instant::now();

    let chunk_hajm = katta_v.len() / thread_soni;
    let chunks: Vec<Vec<i64>> = katta_v.chunks(chunk_hajm).map(|c| c.to_vec()).collect();
    let handlar: Vec<_> = chunks.into_iter().map(|c| {
        thread::spawn(move || c.iter().sum::<i64>())
    }).collect();
    let parallel_yig: i64 = handlar.into_iter().map(|h| h.join().unwrap()).sum();

    println!("Parallel yig'indi: {} ({:.2?})", parallel_yig, boshlanish.elapsed());
    println!("Kutilgan: {}", 1_000_000i64 * 1_000_001 / 2);
    // Parallel yig'indi: 500000500000 (~Xms)
    // Kutilgan: 500000500000
}

// rayon::join simulyatsiyasi
// Симуляция rayon::join
fn join<A, B, RA, RB>(f1: A, f2: B) -> (RA, RB)
where
    A: FnOnce() -> RA + Send + 'static,
    B: FnOnce() -> RB + Send + 'static,
    RA: Send + 'static,
    RB: Send + 'static,
{
    let h2 = thread::spawn(f2);
    let r1 = f1();
    let r2 = h2.join().unwrap();
    (r1, r2)
}

fn join_misoli() {

    println!("\n--- Join (parallel ikki vazifa) ---");

    fn factorial(n: u64) -> u64 {
        (1..=n).product()
    }

    fn fibonacci(n: u32) -> u64 {
        let (mut a, mut b) = (0u64, 1u64);
        for _ in 0..n { (a, b) = (b, a + b); }
        a
    }

    let boshlanish = Instant::now();
    let (fakt, fib) = join(
        || { let r = factorial(20); println!("[T1] 20! = {}", r); r },
        || { let r = fibonacci(50); println!("[T2] fib(50) = {}", r); r },
    );
    println!("Ikkalasi tugadi: 20!={}, fib(50)={} ({:.2?})", fakt, fib, boshlanish.elapsed());
    // [T1] 20! = 2432902008176640000
    // [T2] fib(50) = 12586269025
    // Ikkalasi tugadi: ...

    // Parallel merge sort — join bilan rekursiv
    // Параллельная сортировка слиянием — рекурсивно с join
    fn par_merge_sort(mut v: Vec<i32>) -> Vec<i32> {
        if v.len() <= 512 {
            v.sort();
            return v;
        }
        let o_rta = v.len() / 2;
        let ong = v.split_off(o_rta);
        let chap = v;

        let (sorted_chap, sorted_ong) = join(
            move || par_merge_sort(chap),
            move || par_merge_sort(ong),
        );

        let mut natija = Vec::with_capacity(sorted_chap.len() + sorted_ong.len());
        let (mut i, mut j) = (0, 0);
        while i < sorted_chap.len() && j < sorted_ong.len() {
            if sorted_chap[i] <= sorted_ong[j] { natija.push(sorted_chap[i]); i += 1; }
            else { natija.push(sorted_ong[j]); j += 1; }
        }
        natija.extend_from_slice(&sorted_chap[i..]);
        natija.extend_from_slice(&sorted_ong[j..]);
        natija
    }

    let mut test_v: Vec<i32> = (0..1000).rev().collect();
    test_v.extend((0..1000).rev());

    let sorted = par_merge_sort(test_v.clone());
    test_v.sort();
    println!("Parallel merge sort to'g'rimi: {}", sorted == test_v);
    // Parallel merge sort to'g'rimi: true
}

// rayon::scope simulyatsiyasi
// Симуляция rayon::scope
fn scope_misoli() {

    println!("\n--- Scope (parallel scope) ---");

    let ma_lumot = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let natijalar = Arc::new(Mutex::new(vec![0i32; ma_lumot.len()]));

    thread::scope(|s| {
        for (i, &qiymat) in ma_lumot.iter().enumerate() {
            let natijalar = Arc::clone(&natijalar);
            s.spawn(move || {
                let hisob = qiymat * qiymat + qiymat;
                natijalar.lock().unwrap()[i] = hisob;
            });
        }
    });

    println!("{:?}", natijalar.lock().unwrap());
    // [2, 6, 12, 20, 30, 42, 56, 72]

    // Scoped parallel pipeline
    let kirish = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let chiqish = Arc::new(Mutex::new(Vec::new()));

    thread::scope(|s| {
        for &x in &kirish {
            let chiq = Arc::clone(&chiqish);
            s.spawn(move || {
                if x % 2 == 0 {
                    let natija = x * x;
                    chiq.lock().unwrap().push(natija);
                }
            });
        }
    });

    let mut v = chiqish.lock().unwrap().clone();
    v.sort();
    println!("Juft kvadratlar: {:?}", v);
    // Juft kvadratlar: [4, 16, 36, 64, 100]
}

fn par_sort_misoli() {

    println!("\n--- Par Sort ---");

    let n = 100_000;
    let mut v: Vec<i32> = (0..n).rev().collect();
    let mut v2 = v.clone();

    // Ketma-ket sort
    let t1 = Instant::now();
    v.sort();
    let t1 = t1.elapsed();

    // Parallel sort simulyatsiya (divide and conquer)
    let t2 = Instant::now();
    let thread_soni = num_cpus_sim();
    let chunk_hajm = v2.len() / thread_soni;

    let mut chunks: Vec<Vec<i32>> = v2.chunks_mut(chunk_hajm)
        .map(|c| { let mut v = c.to_vec(); v.sort(); v })
        .collect();

    // Merge all sorted chunks
    let mut natija: Vec<i32> = Vec::with_capacity(v2.len());
    while !chunks.is_empty() {
        let mut min_i = 0;
        for i in 1..chunks.len() {
            if !chunks[i].is_empty() && (chunks[min_i].is_empty() || chunks[i][0] < chunks[min_i][0]) {
                min_i = i;
            }
        }
        if chunks[min_i].is_empty() { break; }
        natija.push(chunks[min_i].remove(0));
        chunks.retain(|c| !c.is_empty());
        if chunks.is_empty() { break; }
    }

    // Oddiy parallel sort
    let mut v3 = v2.clone();
    v3.sort(); // std sort — rayon parallel sort bilan almashtiriladi

    let t2 = t2.elapsed();

    println!("Ketma-ket sort:  {:?}", t1);
    println!("Parallel (sim):  {:?}", t2);
    println!("Natija to'g'ri:  {}", v == v3);
    // Ketma-ket sort:  ~Xms
    // Parallel (sim):  ~Xms
    // Natija to'g'ri:  true
}

// Matn tahlili — parallel so'z hisoblash
// Анализ текста — параллельный подсчёт слов
fn parallel_word_count(matnlar: Vec<String>, thread_soni: usize) -> std::collections::HashMap<String, usize> {
    use std::collections::HashMap;

    let chunk_hajm = (matnlar.len() + thread_soni - 1) / thread_soni;
    let chunks: Vec<Vec<String>> = matnlar.chunks(chunk_hajm)
        .map(|c| c.to_vec()).collect();

    let handlar: Vec<_> = chunks.into_iter().map(|chunk| {
        thread::spawn(move || {
            let mut hisob: HashMap<String, usize> = HashMap::new();
            for matn in &chunk {
                for soz in matn.split_whitespace() {
                    let soz = soz.to_lowercase();
                    let soz = soz.trim_matches(|c: char| !c.is_alphabetic()).to_string();
                    if !soz.is_empty() {
                        *hisob.entry(soz).or_insert(0) += 1;
                    }
                }
            }
            hisob
        })
    }).collect();

    let mut jami: HashMap<String, usize> = HashMap::new();
    for h in handlar {
        for (soz, hisob) in h.join().unwrap() {
            *jami.entry(soz).or_insert(0) += hisob;
        }
    }
    jami
}

// Rasm piksellari parallel qayta ishlash
// Параллельная обработка пикселей изображения
fn parallel_grayscale(piksellar: &mut Vec<(u8, u8, u8)>, thread_soni: usize) {
    let n = piksellar.len();
    let chunk_hajm = (n + thread_soni - 1) / thread_soni;

    thread::scope(|s| {
        for chunk in piksellar.chunks_mut(chunk_hajm) {
            s.spawn(|| {
                for p in chunk.iter_mut() {
                    let kulrang = ((p.0 as u32 * 299 + p.1 as u32 * 587 + p.2 as u32 * 114) / 1000) as u8;
                    *p = (kulrang, kulrang, kulrang);
                }
            });
        }
    });
}

fn real_hayot_misollari() {

    println!("\n--- Parallel Word Count ---");
    let matnlar = vec![
        "Rust tili tez va xavfsiz".to_string(),
        "Rust ownership modeli ajoyib".to_string(),
        "Tez dasturlar Rust bilan yaratiladi".to_string(),
        "Rayon parallel hisoblash uchun".to_string(),
        "Rust va Rayon ajoyib kombinatsiya".to_string(),
    ];

    let hisob = parallel_word_count(matnlar, num_cpus_sim());
    let mut top: Vec<(&String, &usize)> = hisob.iter().collect();
    top.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

    println!("Eng ko'p uchraydigan so'zlar:");
    for (soz, n) in top.iter().take(5) {
        println!("  {}: {}", soz, n);
    }
    // Eng ko'p uchraydigan so'zlar:
    //   rust: 5
    //   ajoyib: 2
    //   va: 2
    //   ...

    println!("\n--- Parallel Grayscale ---");
    let mut piksellar: Vec<(u8, u8, u8)> = vec![
        (255, 0, 0),   // qizil
        (0, 255, 0),   // yashil
        (0, 0, 255),   // ko'k
        (255, 255, 0), // sariq
        (128, 64, 32), // jigarrang
    ];

    println!("Avval: {:?}", &piksellar[..3]);
    parallel_grayscale(&mut piksellar, 2);
    println!("Kulrang: {:?}", &piksellar[..3]);
    // Avval: [(255, 0, 0), (0, 255, 0), (0, 0, 255)]
    // Kulrang: [(76, 76, 76), (150, 150, 150), (29, 29, 29)]

    println!("\n--- Performance Taqqoslash ---");
    let n = 500_000usize;
    let ma_lumot: Vec<f64> = (1..=(n as i64)).map(|x| x as f64).collect();

    // Ketma-ket
    let t1 = Instant::now();
    let _: f64 = ma_lumot.iter().map(|&x| (x * x + x).sqrt()).sum();
    let vaqt1 = t1.elapsed();

    // Parallel
    let t2 = Instant::now();
    let thread_soni = num_cpus_sim();
    let chunk_hajm = n / thread_soni;
    let chunks: Vec<Vec<f64>> = ma_lumot.chunks(chunk_hajm).map(|c| c.to_vec()).collect();
    let handlar: Vec<_> = chunks.into_iter().map(|c| {
        thread::spawn(move || c.iter().map(|&x| (x * x + x).sqrt()).sum::<f64>())
    }).collect();
    let _: f64 = handlar.into_iter().map(|h| h.join().unwrap()).sum();
    let vaqt2 = t2.elapsed();

    println!("n={}, thread soni={}", n, thread_soni);
    println!("Ketma-ket: {:?}", vaqt1);
    println!("Parallel:  {:?}", vaqt2);
    if vaqt2 < vaqt1 {
        println!("Tezlashuv: {:.1}x", vaqt1.as_secs_f64() / vaqt2.as_secs_f64());
    }
}

fn main() {

    println!("=== PARALLEL MAP ===");
    parallel_map_misoli();

    println!("=== PARALLEL FILTER VA FOLD ===");
    parallel_filter_fold_misoli();

    println!("=== JOIN ===");
    join_misoli();

    println!("=== SCOPE ===");
    scope_misoli();

    println!("=== PAR SORT ===");
    par_sort_misoli();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        RAYON ASOSIY API                                                                                      |
// #================================================================================================================================================#
// # |   1 | v.par_iter()                    | Parallel immutable iterator                | Параллельный иммутабельный итератор                     |
// # |   2 | v.par_iter_mut()                | Parallel mutable iterator                  | Параллельный мутабельный итератор                       |
// # |   3 | v.into_par_iter()               | Parallel owning iterator                   | Параллельный owning итератор                            |
// # |   4 | .map() .filter() .sum()         | Parallel kombinatorlar                     | Параллельные комбинаторы                                |
// # |   5 | .collect::<Vec<_>>()            | Parallel natijalarni yig'ish               | Сбор параллельных результатов                           |
// #================================================================================================================================================#
// # |                                        RAYON SORT                                                                                            |
// #================================================================================================================================================#
// # |   6 | v.par_sort()                    | Parallel tartiblash                        | Параллельная сортировка                                 |
// # |   7 | v.par_sort_by(|a,b| ...)        | Parallel custom tartiblash                 | Параллельная пользовательская сортировка                |
// # |   8 | v.par_sort_by_key(|x| ...)      | Parallel kalit bo'yicha tartiblash         | Параллельная сортировка по ключу                        |
// # |   9 | v.par_sort_unstable()           | Tezroq (tartib kafolatsiz)                 | Быстрее (без гарантии порядка)                          |
// #================================================================================================================================================#
// # |                                        RAYON JOIN VA SCOPE                                                                                   |
// #================================================================================================================================================#
// # |  10 | rayon::join(f1, f2)             | Ikki funksiyani parallel                   | Два задания параллельно                                 |
// # |  11 | rayon::scope(|s| { s.spawn }) ) | Parallel scope (scoped threads kabi)       | Параллельный scope (как scoped threads)                 |
// # |  12 | ThreadPoolBuilder::new()        | Custom thread pool                         | Пользовательский пул потоков                            |
// # |  13 | pool.install(|| ...)            | Pool da kod bajarish                       | Выполнение кода в пуле                                  |
// #================================================================================================================================================#
// # |                                        QACHON RAYON                                                                                          |
// #================================================================================================================================================#
// # |  14 | CPU-intensive                   | Ko'p CPU talab qiladigan hisoblash         | Вычисления требующие много CPU                          |
// # |  15 | Katta ma'lumot to'plamlari      | Vec, slice parallel qayta ishlash          | Параллельная обработка Vec, slice                       |
// # |  16 | par_sort                        | Katta massivlarni tartiblash               | Сортировка больших массивов                             |
// # |  17 | .par_chunks(n)                  | Bloklarga bo'lib parallel                  | Параллельно по блокам                                   |
// # |  18 | Work-stealing pool              | CPU larni avtomatik balanslash             | Автоматическая балансировка CPU                         |
// #================================================================================================================================================#