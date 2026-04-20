// #================================================================================================================================================#
// #                                                                RAII PATTERN                                                                    #
// #                        RAII — RESOURCE ACQUISITION IS INITIALIZATION. DROP ORQALI AVTOMATIK TOZALASH. GUARD PATTERN.                           #
// #                        RAII — ИНИЦИАЛИЗАЦИЯ КАК ПОЛУЧЕНИЕ РЕСУРСА. АВТОМАТИЧЕСКАЯ ОЧИСТКА ЧЕРЕЗ DROP. ПАТТЕРН GUARD.                           #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;
use std::collections::HashMap;

// RAII nima:
// Что такое RAII:
//
//   Resource Acquisition Is Initialization
//   Resurs olish = Initsializatsiya
//
//   Asosiy g'oya:
//   Основная идея:
//   - Resurs olish: constructor/new()
//   - Получение ресурса: constructor/new()
//   - Resurs bo'shatish: Drop trait avtomatik
//   - Освобождение ресурса: Drop trait автоматически
//
//   Rustdagi misollar:
//   Примеры в Rust:
//   - Box<T>         — heap xotira
//   - File           — fayl deskriptori
//   - MutexGuard     — qulflash
//   - TempDir        — vaqtinchalik katalog
//   - Connection     — tarmoq ulanishi
//   - Transaction    — DB tranzaksiya


struct Fayl {
    nomi: String,
    ochildi: bool,
}

impl Fayl {
    fn ochish(nomi: &str) -> Result<Self, String> {
        println!("[Fayl] '{}' ochildi", nomi);
        Ok(Fayl { nomi: nomi.to_string(), ochildi: true })
    }

    fn yozish(&self, matn: &str) -> Result<(), String> {
        if !self.ochildi {
            return Err("Fayl yopilgan!".to_string());
        }
        println!("[Fayl] '{}' ga yozildi: '{}'", self.nomi, matn);
        Ok(())
    }

    fn o_qish(&self) -> Result<String, String> {
        if !self.ochildi {
            return Err("Fayl yopilgan!".to_string());
        }
        Ok(format!("[{}] mazmun", self.nomi))
    }
}

impl Drop for Fayl {
    fn drop(&mut self) {
        if self.ochildi {
            println!("[Fayl] '{}' yopildi (Drop avtomatik)", self.nomi);
            self.ochildi = false;
        }
    }
}

fn asosiy_raii_misoli() {

    println!("--- Asosiy RAII ---");
    {
        let f = Fayl::ochish("config.toml").unwrap();
        f.yozish("key = value").unwrap();
        println!("{}", f.o_qish().unwrap());
        println!("Scope tugamoqda...");
    } // ← bu yerda Drop avtomatik chaqiriladi!
    println!("Scope tugadi — fayl yopildi");
    // [Fayl] 'config.toml' ochildi
    // [Fayl] 'config.toml' ga yozildi: 'key = value'
    // [config.toml] mazmun
    // Scope tugamoqda...
    // [Fayl] 'config.toml' yopildi (Drop avtomatik)
    // Scope tugadi — fayl yopildi

    // Drop tartibi — teskari
    // Порядок Drop — обратный
    println!("\n--- Drop tartibi ---");
    {
        let _a = Fayl::ochish("a.txt").unwrap();
        let _b = Fayl::ochish("b.txt").unwrap();
        let _c = Fayl::ochish("c.txt").unwrap();
        println!("Uchala fayl ochiq...");
    }
    // c.txt, b.txt, a.txt — teskari tartibda yopiladi
    // [Fayl] 'a.txt' ochildi
    // [Fayl] 'b.txt' ochildi
    // [Fayl] 'c.txt' ochildi
    // Uchala fayl ochiq...
    // [Fayl] 'c.txt' yopildi
    // [Fayl] 'b.txt' yopildi
    // [Fayl] 'a.txt' yopildi
}

// MutexGuard ga o'xshash — qulf tutuvchi
// Похоже на MutexGuard — держатель блокировки
struct QulfGuard<'a, T> {
    qiymat: &'a mut T,
    qulf_nomi: &'static str,
}

impl<'a, T> QulfGuard<'a, T> {
    fn qulflash(qiymat: &'a mut T, nomi: &'static str) -> Self {
        println!("[Guard] '{}' qulflandi", nomi);
        QulfGuard { qiymat, qulf_nomi: nomi }
    }
}

impl<'a, T> std::ops::Deref for QulfGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T { self.qiymat }
}

impl<'a, T> std::ops::DerefMut for QulfGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T { self.qiymat }
}

impl<'a, T> Drop for QulfGuard<'a, T> {
    fn drop(&mut self) {
        println!("[Guard] '{}' qulf ochildi (Drop)", self.qulf_nomi);
    }
}

fn guard_pattern_misoli() {

    println!("--- Guard Pattern ---");
    let mut ma_lumot: Vec<i32> = vec![1, 2, 3];

    {
        let mut guard = QulfGuard::qulflash(&mut ma_lumot, "ma_lumot_qulfi");
        guard.push(4);
        guard.push(5);
        println!("Guard orqali: {:?}", *guard);
        println!("Scope tugamoqda...");
    } // ← qulf avtomatik ochildi!
    println!("Ma'lumot: {:?}", ma_lumot);
    // [Guard] 'ma_lumot_qulfi' qulflandi
    // Guard orqali: [1, 2, 3, 4, 5]
    // Scope tugamoqda...
    // [Guard] 'ma_lumot_qulfi' qulf ochildi (Drop)
    // Ma'lumot: [1, 2, 3, 4, 5]

    // Panic bo'lsa ham guard ochildi
    println!("\n--- Panic da Guard ---");
    let mut n: i32 = 0;
    let natija = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut g = QulfGuard::qulflash(&mut n, "panic_testi");
        *g = 42;
        panic!("Test panic!");
    }));
    println!("Panic ushlanidi: {}", natija.is_err());
    // Guard ochildi — panic bo'lsa ham!
    // [Guard] 'panic_testi' qulflandi
    // [Guard] 'panic_testi' qulf ochildi (Drop)
    // Panic ushlanidi: true
}

struct VaqtOlchash {
    nomi: String,
    boshlanish: Instant,
}

impl VaqtOlchash {
    fn boshlash(nomi: &str) -> Self {
        println!("[Timer] '{}' boshlandi", nomi);
        VaqtOlchash {
            nomi: nomi.to_string(),
            boshlanish: Instant::now(),
        }
    }
}

impl Drop for VaqtOlchash {
    fn drop(&mut self) {
        let o_tgan = self.boshlanish.elapsed();
        println!("[Timer] '{}' tugadi: {:?}", self.nomi, o_tgan);
    }
}

// Sodda benchmark macro
macro_rules! vaqt_olch {
    ($nomi:literal, $blok:block) => {
        {
            let _timer = VaqtOlchash::boshlash($nomi);
            $blok
        }
    };
}

fn timer_raii_misoli() {

    println!("--- Timer RAII ---");

    vaqt_olch!("Hisoblash", {
        let mut yig: u64 = 0;
        for i in 0..1_000_000u64 { yig += i; }
        println!("Yig'indi: {}", yig);
    });

    // Ichma-ich timerlar
    {
        let _t1 = VaqtOlchash::boshlash("Tashqi");
        {
            let _t2 = VaqtOlchash::boshlash("Ichki");
            std::thread::sleep(std::time::Duration::from_millis(5));
        } // Ichki tugadi
        std::thread::sleep(std::time::Duration::from_millis(5));
    } // Tashqi tugadi
    // [Timer] 'Tashqi' boshlandi
    // [Timer] 'Ichki' boshlandi
    // [Timer] 'Ichki' tugadi: ~5ms
    // [Timer] 'Tashqi' tugadi: ~10ms
}

struct DbTranzaksiya {
    id: u32,
    operatsiyalar: Vec<String>,
    amalga_oshirildi: bool,
}

impl DbTranzaksiya {
    fn boshlash(id: u32) -> Self {
        println!("[DB-TX #{}] Tranzaksiya boshlandi", id);
        DbTranzaksiya {
            id,
            operatsiyalar: Vec::new(),
            amalga_oshirildi: false,
        }
    }

    fn bajar(&mut self, sql: &str) {
        self.operatsiyalar.push(sql.to_string());
        println!("[DB-TX #{}] SQL: {}", self.id, sql);
    }

    fn commit(mut self) {
        println!("[DB-TX #{}] COMMIT — {} operatsiya", self.id, self.operatsiyalar.len());
        self.amalga_oshirildi = true;
        // self drop bo'lganda — amalga_oshirildi = true, rollback yo'q
    }
}

impl Drop for DbTranzaksiya {
    fn drop(&mut self) {
        if !self.amalga_oshirildi {
            println!("[DB-TX #{}] ROLLBACK — commit qilinmagan ({} operatsiya)",
                     self.id, self.operatsiyalar.len());
        } else {
            println!("[DB-TX #{}] Tranzaksiya yakunlandi", self.id);
        }
    }
}

fn tranzaksiya_raii_misoli() {

    println!("--- Tranzaksiya RAII ---");

    // Muvaffaqiyatli tranzaksiya
    {
        let mut tx = DbTranzaksiya::boshlash(1);
        tx.bajar("INSERT INTO users VALUES (1, 'Dilshod')");
        tx.bajar("UPDATE balances SET amount = 1000 WHERE id = 1");
        tx.commit(); // muvaffaqiyat
    }
    // [DB-TX #1] Tranzaksiya boshlandi
    // [DB-TX #1] SQL: INSERT INTO users...
    // [DB-TX #1] SQL: UPDATE balances...
    // [DB-TX #1] COMMIT — 2 operatsiya
    // [DB-TX #1] Tranzaksiya yakunlandi

    println!();

    // Muvaffaqiyatsiz — rollback avtomatik
    {
        let mut tx = DbTranzaksiya::boshlash(2);
        tx.bajar("DELETE FROM orders WHERE id = 5");
        tx.bajar("UPDATE inventory SET count = count - 1");
        println!("[Main] Xato yuz berdi — commit chaqirilmaydi");
        // commit() chaqirilmaydi → Drop → ROLLBACK!
    }
    // [DB-TX #2] Tranzaksiya boshlandi
    // [DB-TX #2] SQL: DELETE...
    // [DB-TX #2] SQL: UPDATE...
    // [Main] Xato yuz berdi — commit chaqirilmaydi
    // [DB-TX #2] ROLLBACK — commit qilinmagan (2 operatsiya)
}

struct Resurs {
    id: u32,
}

impl fmt::Debug for Resurs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Resurs#{}", self.id)
    }
}

struct ResursPool {
    mavjud: Arc<Mutex<Vec<Resurs>>>,
    hajm: u32,
}

impl ResursPool {
    fn new(hajm: u32) -> Self {
        let resurslar: Vec<Resurs> = (1..=hajm)
            .map(|id| {
                println!("[Pool] Resurs#{} yaratildi", id);
                Resurs { id }
            })
            .collect();
        ResursPool {
            mavjud: Arc::new(Mutex::new(resurslar)),
            hajm,
        }
    }

    fn ol(&self) -> Option<PoolGuard> {
        let mut mavjud = self.mavjud.lock().unwrap();
        mavjud.pop().map(|r| {
            println!("[Pool] {:?} berildi", r);
            PoolGuard {
                resurs: Some(r),
                pool: Arc::clone(&self.mavjud),
            }
        })
    }

    fn mavjud_soni(&self) -> usize {
        self.mavjud.lock().unwrap().len()
    }
}

struct PoolGuard {
    resurs: Option<Resurs>,
    pool: Arc<Mutex<Vec<Resurs>>>,
}

impl PoolGuard {
    fn id(&self) -> u32 {
        self.resurs.as_ref().map(|r| r.id).unwrap_or(0)
    }
}

impl Drop for PoolGuard {
    fn drop(&mut self) {
        if let Some(r) = self.resurs.take() {
            println!("[Pool] {:?} qaytarildi", r);
            self.pool.lock().unwrap().push(r);
        }
    }
}

fn resurs_pool_misoli() {

    println!("--- Resurs Pool ---");
    let pool = ResursPool::new(3);
    println!("Mavjud: {}", pool.mavjud_soni()); // 3

    {
        let g1 = pool.ol().unwrap();
        let g2 = pool.ol().unwrap();
        println!("Mavjud: {} (2 ta berilgan)", pool.mavjud_soni()); // 1
        println!("g1 id={}, g2 id={}", g1.id(), g2.id());
        // Scope tugaganda g1, g2 qaytariladi
    }

    println!("Mavjud: {} (qaytarildi)", pool.mavjud_soni()); // 3

    // Pool to'la bo'lsa — None
    let mut guardlar = vec![];
    for _ in 0..3 {
        if let Some(g) = pool.ol() {
            guardlar.push(g);
        }
    }
    println!("Mavjud: {} (hammasi olingan)", pool.mavjud_soni()); // 0
    println!("Qo'shimcha: {:?}", pool.ol().map(|g| g.id())); // None
    drop(guardlar); // hammasi qaytarildi
    println!("Mavjud: {} (hammasi qaytarildi)", pool.mavjud_soni()); // 3
    // [Pool] Resurs#1,2,3 yaratildi
    // Mavjud: 3
    // [Pool] Resurs#3 berildi
    // [Pool] Resurs#2 berildi
    // Mavjud: 1
    // [Pool] Resurs#3 qaytarildi
    // [Pool] Resurs#2 qaytarildi
    // Mavjud: 3
}

// Go/Swift da defer kalit so'z bor — Rust da Drop bilan
// В Go/Swift есть ключевое слово defer — в Rust через Drop
struct Defer<F: FnOnce()> {
    f: Option<F>,
    nomi: &'static str,
}

impl<F: FnOnce()> Defer<F> {
    fn new(nomi: &'static str, f: F) -> Self {
        Defer { f: Some(f), nomi }
    }

    fn bekor_qilish(mut self) {
        self.f = None;
        println!("[Defer] '{}' bekor qilindi", self.nomi);
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        if let Some(f) = self.f.take() {
            println!("[Defer] '{}' bajarilmoqda...", self.nomi);
            f();
        }
    }
}

macro_rules! defer {
    ($nomi:literal, $blok:block) => {
        let _defer = Defer::new($nomi, || $blok);
    };
}

fn defer_misoli() {

    println!("--- Defer Pattern ---");

    {
        defer!("tozalash", {
            println!("Tozalanmoqda...");
        });
        defer!("log", {
            println!("Log yozilmoqda...");
        });

        println!("Asosiy ish bajarildi");
    }
    // Asosiy ish bajarildi
    // [Defer] 'log' bajarilmoqda...     ← teskari tartib
    // Log yozilmoqda...
    // [Defer] 'tozalash' bajarilmoqda...
    // Tozalanmoqda...

    // Defer bekor qilish — rollback pattern
    println!("\n--- Defer bekor qilish ---");
    let mut amalga_oshirildi = false;
    {
        let tozalash = Defer::new("rollback", || {
            println!("ROLLBACK bajarildi!");
        });

        println!("Ish bajarilmoqda...");
        amalga_oshirildi = true;

        if amalga_oshirildi {
            tozalash.bekor_qilish(); // rollback kerak emas
        }
    }
    // Ish bajarilmoqda...
    // [Defer] 'rollback' bekor qilindi
}

fn real_hayot_misollari() {

    println!("=== Tranzaksiya ===");
    tranzaksiya_raii_misoli();

    println!("\n=== Timer ===");
    timer_raii_misoli();

    println!("\n=== Resurs Pool ===");
    resurs_pool_misoli();

    println!("\n=== Defer ===");
    defer_misoli();

    // Hamma birlashtirib
    println!("\n=== Barcha RAII birgalikda ===");
    {
        let _t = VaqtOlchash::boshlash("barcha_operatsiyalar");

        let pool = ResursPool::new(2);
        let g = pool.ol().unwrap();

        {
            let mut tx = DbTranzaksiya::boshlash(99);
            tx.bajar(&format!("UPDATE resource SET used=true WHERE id={}", g.id()));

            defer!("log_yozish", {
                println!("[Log] Operatsiya tugadi");
            });

            tx.commit();
        }
    }
    // Hamma RAII avtomatik tozalanadi!
}

fn main() {

    println!("=== ASOSIY RAII ===");
    asosiy_raii_misoli();

    println!("\n=== GUARD PATTERN ===");
    guard_pattern_misoli();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        RAII ASOSLARI                                                                                         |
// #================================================================================================================================================#
// # |   1 | impl Drop for T { fn drop }     | Resurs avtomatik bo'shatish                | Автоматическое освобождение ресурса                     |
// # |   2 | Scope tugaganda Drop            | RAII asosi                                 | Основа RAII                                             |
// # |   3 | Teskari tartib                  | Son oxirgi → Drop birinchi                 | Последний созданный → первым Drop                       |
// # |   4 | Panic da ham Drop               | Xavfsizlik kafolati                        | Гарантия безопасности даже при panic                    |
// #================================================================================================================================================#
// # |                                        PATTERNLAR                                                                                            |
// #================================================================================================================================================#
// # |   5 | Guard Pattern                   | Qulflash RAII bilan                        | Блокировка через RAII                                   |
// # |   6 | Timer Guard                     | Vaqt o'lchash RAII bilan                   | Измерение времени через RAII                            |
// # |   7 | Transaction Guard               | Commit/Rollback RAII bilan                 | Commit/Rollback через RAII                              |
// # |   8 | Pool Guard                      | Resurs Pool RAII bilan                     | Пул ресурсов через RAII                                 |
// # |   9 | Defer Pattern                   | Keyinga qoldirish RAII bilan               | Отсрочка выполнения через RAII                          |
// # |  10 | MutexGuard                      | std Mutex RAII guard                       | std Mutex RAII guard                                    |
// #================================================================================================================================================#
// # |                                        AFZALLIKLARI                                                                                          |
// #================================================================================================================================================#
// # |  11 | Xotira sizmasligi               | Drop kafolati                              | Гарантия отсутствия утечек                              |
// # |  12 | Exception xavfsizligi           | Panic da ham tozalanadi                    | Очистка даже при panic                                  |
// # |  13 | Avtomatik boshqaruv             | Qo'lda close/free kerak emas               | Не нужно вручную close/free                             |
// # |  14 | Zero-cost                       | Abstraktsiya xarajatsiz                    | Абстракция без затрат                                   |
// #================================================================================================================================================#