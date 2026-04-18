// #================================================================================================================================================#
// #                                                               DROP                                                                             #
// #                     DROP — QIYMAT SCOPE DAN CHIQGANDA AVTOMATIK CHAQIRILADI. XOTIRANI VA RESURSLARNI TOZALASH.                                 #
// #                     DROP — АВТОМАТИЧЕСКИ ВЫЗЫВАЕТСЯ КОГДА ЗНАЧЕНИЕ ВЫХОДИТ ИЗ ОБЛАСТИ. ОЧИСТКА ПАМЯТИ И РЕСУРСОВ.                              #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// Drop trait:
//   trait Drop {
//       fn drop(&mut self);
//   }
//
// Qoidalar:
// Правила:
//   1. drop() avtomatik chaqiriladi — scope tugaganda
//      drop() вызывается автоматически — когда область заканчивается
//   2. drop() qo'lda chaqirib bo'lmaydi — faqat std::mem::drop()
//      drop() нельзя вызвать вручную — только std::mem::drop()
//   3. Drop tartibi: teskari — oxirgi yaratilgan birinchi tushiriladi
//      Порядок Drop: обратный — последний созданный удаляется первым
//   4. Copy + Drop — birga bo'la olmaydi
//      Copy + Drop — не могут быть вместе
//   5. Struct tushirilganda — fieldlar ham avtomatik tushiriladi
//      При удалении структуры — поля тоже удаляются автоматически
//   6. struct drop() → field drop() — avval struct, keyin fieldlar
//      struct drop() → field drop() — сначала структура, потом поля

#[derive(Debug)]
struct Resurs {
    nomi: String,
    qiymat: i32,
}

impl Resurs {
    fn new(nomi: &str, qiymat: i32) -> Self {
        println!("  [YARATILDI] {}", nomi);
        Resurs { nomi: nomi.to_string(), qiymat }
    }
}

impl Drop for Resurs {
    fn drop(&mut self) {
        println!("  [TUSHIRILDI] {} (qiymat: {})", self.nomi, self.qiymat);
        // Resursni tozalash kodi bu yerda
        // Код очистки ресурса здесь
    }
}

fn drop_asosiy_misoli() {

    println!("-- Scope boshlanadi --");
    {
        let r1: Resurs = Resurs::new("A", 1);
        let r2: Resurs = Resurs::new("B", 2);
        let r3: Resurs = Resurs::new("C", 3);
        println!("  Ishlamoqda...");
        // scope tugaydi — teskari tartibda tushiriladi
        // область заканчивается — удаляются в обратном порядке
    }
    println!("-- Scope tugadi --");
    // -- Scope boshlanadi --
    //   [YARATILDI] A
    //   [YARATILDI] B
    //   [YARATILDI] C
    //   Ishlamoqda...
    //   [TUSHIRILDI] C (qiymat: 3)  ← oxirgi yaratilgan birinchi tushiriladi
    //   [TUSHIRILDI] B (qiymat: 2)
    //   [TUSHIRILDI] A (qiymat: 1)
    // -- Scope tugadi --
}

fn erta_tushirish_misoli() {

    println!("-- Erta tushirish --");
    let r1: Resurs = Resurs::new("X", 10);
    let r2: Resurs = Resurs::new("Y", 20);

    println!("  r1 erta tushirilmoqda...");
    drop(r1);  // std::mem::drop() — erta tushirish
    // std::mem::drop() — раннее удаление
    // r1.drop() qo'lda chaqirib bo'lmaydi!
    // r1.drop() нельзя вызвать вручную!

    println!("  r2 hali bor...");
    println!("-- Scope tugaydi --");
    // r2 scope oxirida tushiriladi
    // r2 удаляется в конце области

    // -- Erta tushirish --
    //   [YARATILDI] X
    //   [YARATILDI] Y
    //   r1 erta tushirilmoqda...
    //   [TUSHIRILDI] X (qiymat: 10)
    //   r2 hali bor...
    // -- Scope tugaydi --
    //   [TUSHIRILDI] Y (qiymat: 20)
}

#[derive(Debug)]
struct FieldA;
#[derive(Debug)]
struct FieldB;
#[derive(Debug)]
struct FieldC;

impl Drop for FieldA {
    fn drop(&mut self) { println!("    [drop] FieldA"); }
}
impl Drop for FieldB {
    fn drop(&mut self) { println!("    [drop] FieldB"); }
}
impl Drop for FieldC {
    fn drop(&mut self) { println!("    [drop] FieldC"); }
}

#[derive(Debug)]
struct MurakkabStruct {
    a: FieldA,
    b: FieldB,
    c: FieldC,
}

impl Drop for MurakkabStruct {
    fn drop(&mut self) {
        println!("  [drop] MurakkabStruct (avval struct, keyin fieldlar)");
        // Структура drop'и → keyin fieldlar drop'i
        // Drop структуры → потом drop полей
    }
}

fn drop_tartibi_misoli() {

    println!("-- Drop tartibi --");
    let _m = MurakkabStruct {
        a: FieldA,
        b: FieldB,
        c: FieldC,
    };
    println!("  Scope tugaydi...");
    // -- Drop tartibi --
    //   Scope tugaydi...
    //   [drop] MurakkabStruct
    //   [drop] FieldA   ← e'lon qilish tartibida
    //   [drop] FieldB
    //   [drop] FieldC
}

// Fayl simulyatsiyasi — RAII pattern
// Симуляция файла — паттерн RAII
#[derive(Debug)]
struct FaylBoshqaruvchi {
    yo_l: String,
    ochiqmi: bool,
}

impl FaylBoshqaruvchi {
    fn och(yo_l: &str) -> Self {
        println!("  [OCHILDI] {}", yo_l);
        FaylBoshqaruvchi {
            yo_l: yo_l.to_string(),
            ochiqmi: true,
        }
    }

    fn yoz(&self, mazmun: &str) {
        if self.ochiqmi {
            println!("  [YOZILDI] {} → '{}'", self.yo_l, mazmun);
        }
    }

    fn o_qi(&self) -> String {
        if self.ochiqmi {
            format!("{} mazmuni", self.yo_l)
        } else {
            String::new()
        }
    }
}

impl Drop for FaylBoshqaruvchi {
    fn drop(&mut self) {
        if self.ochiqmi {
            self.ochiqmi = false;
            println!("  [YOPILDI] {} (avtomatik)", self.yo_l);
        }
    }
}

// Tarmoq ulanishi simulyatsiyasi — RAII
// Симуляция сетевого соединения — RAII
#[derive(Debug)]
struct TarmoqUlanish {
    manzil: String,
    port: u16,
}

impl TarmoqUlanish {
    fn ulash(manzil: &str, port: u16) -> Result<Self, String> {
        println!("  [ULANDI] {}:{}", manzil, port);
        Ok(TarmoqUlanish {
            manzil: manzil.to_string(),
            port,
        })
    }

    fn so_rov_yuborish(&self, so_rov: &str) {
        println!("  [SO'ROV] {}:{} → {}", self.manzil, self.port, so_rov);
    }
}

impl Drop for TarmoqUlanish {
    fn drop(&mut self) {
        println!("  [UZILDI] {}:{} (avtomatik)", self.manzil, self.port);
    }
}

fn raii_misollari() {

    println!("-- RAII Fayl --");
    {
        let fayl: FaylBoshqaruvchi = FaylBoshqaruvchi::och("ma_lumot.txt");
        fayl.yoz("Salom dunyo!");
        println!("  O'qildi: {}", fayl.o_qi());
        // scope tugaydi — fayl avtomatik yopiladi
        // область заканчивается — файл закрывается автоматически
    }
    println!("-- Fayl yopildi --");

    println!("\n-- RAII Tarmoq --");
    {
        let ulanish: TarmoqUlanish = TarmoqUlanish::ulash("api.example.com", 443)
            .expect("Ulanib bo'lmadi");
        ulanish.so_rov_yuborish("GET /users");
        ulanish.so_rov_yuborish("POST /data");
        // scope tugaydi — ulanish avtomatik uziladi
        // область заканчивается — соединение закрывается автоматически
    }
    println!("-- Ulanish uzildi --");

    // -- RAII Fayl --
    //   [OCHILDI] ma_lumot.txt
    //   [YOZILDI] ma_lumot.txt → 'Salom dunyo!'
    //   O'qildi: ma_lumot.txt mazmuni
    //   [YOPILDI] ma_lumot.txt (avtomatik)
    // -- Fayl yopildi --
    //
    // -- RAII Tarmoq --
    //   [ULANDI] api.example.com:443
    //   [SO'ROV] api.example.com:443 → GET /users
    //   [SO'ROV] api.example.com:443 → POST /data
    //   [UZILDI] api.example.com:443 (avtomatik)
    // -- Ulanish uzildi --
}

// Lock Guard — Drop bilan avtomatik unlock
// Lock Guard — автоматический unlock через Drop
#[derive(Debug)]
struct LockGuard<'a> {
    qulf_nomi: &'a str,
}

impl<'a> LockGuard<'a> {
    fn yangi(qulf_nomi: &'a str) -> Self {
        println!("  [QULFLANDI] {}", qulf_nomi);
        LockGuard { qulf_nomi }
    }

    fn amaliyot(&self) {
        println!("  [AMALIYOT] {} qulflangan holda", self.qulf_nomi);
    }
}

impl<'a> Drop for LockGuard<'a> {
    fn drop(&mut self) {
        println!("  [QULF OCHILDI] {} (avtomatik)", self.qulf_nomi);
    }
}

fn lock_guard_misoli() {

    println!("-- Lock Guard --");
    {
        let _guard: LockGuard = LockGuard::yangi("ma_lumotlar_bazasi");
        _guard.amaliyot();
        // scope tugaydi — qulf avtomatik ochiladi
        // область заканчивается — блокировка снимается автоматически
    }
    println!("-- Qulf ochildi --");

    // -- Lock Guard --
    //   [QULFLANDI] ma_lumotlar_bazasi
    //   [AMALIYOT] ma_lumotlar_bazasi qulflangan holda
    //   [QULF OCHILDI] ma_lumotlar_bazasi (avtomatik)
    // -- Qulf ochildi --
}

fn drop_va_panic_misoli() {

    // Panic bo'lganda ham drop() chaqiriladi
    // drop() вызывается даже при панике
    println!("-- Drop + Panic --");

    let natija = std::panic::catch_unwind(|| {
        let r: Resurs = Resurs::new("Panic_test", 99);
        println!("  Panic oldidan...");
        panic!("Test panic!");
        // r.drop() chaqiriladi panic bo'lsa ham!
        // r.drop() вызывается даже при панике!
    });

    println!("  Panic ushlandi: {}", natija.is_err());

    // -- Drop + Panic --
    //   [YARATILDI] Panic_test
    //   Panic oldidan...
    //   [TUSHIRILDI] Panic_test (qiymat: 99)  ← drop chaqirildi!
    //   Panic ushlandi: true
}

fn mem_forget_misoli() {

    // std::mem::forget — drop() chaqirmasdan ega bo'lishni uzatish
    // std::mem::forget — передача владения без вызова drop()
    // OGOHLANTIRISH: Memory leak bo'lishi mumkin!
    // ПРЕДУПРЕЖДЕНИЕ: Возможна утечка памяти!

    println!("-- mem::forget --");
    let r: Resurs = Resurs::new("Forget_test", 777);
    println!("  forget() chaqirilmoqda...");
    std::mem::forget(r);
    // drop() CHAQIRILMAYDI!
    // drop() НЕ ВЫЗЫВАЕТСЯ!
    println!("  forget() tugadi — drop bo'lmadi");

    // -- mem::forget --
    //   [YARATILDI] Forget_test
    //   forget() chaqirilmoqda...
    //   forget() tugadi — drop bo'lmadi
    // Eslatma: [TUSHIRILDI] chiqmadi!
}

fn main() {

    println!("=== DROP ASOSIY ===");
    drop_asosiy_misoli();

    println!("\n=== ERTA TUSHIRISH ===");
    erta_tushirish_misoli();

    println!("\n=== DROP TARTIBI ===");
    drop_tartibi_misoli();

    println!("\n=== RAII ===");
    raii_misollari();

    println!("\n=== LOCK GUARD ===");
    lock_guard_misoli();

    println!("\n=== DROP + PANIC ===");
    drop_va_panic_misoli();

    println!("\n=== MEM::FORGET ===");
    mem_forget_misoli();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya              | Tavsif (UZ)                                   | Описание (RU)                                              |
// #================================================================================================================================================#
// # |                                       DROP TRAIT                                                                                             |
// #================================================================================================================================================#
// # |   1 | impl Drop for T           | Tozalash kodi — scope tugaganda               | Код очистки — при выходе из области                        |
// # |   2 | fn drop(&mut self)        | Yagona majburiy metod                         | Единственный обязательный метод                            |
// # |   3 | drop() avtomatik          | Qo'lda chaqirib bo'lmaydi                     | Нельзя вызвать вручную                                     |
// # |   4 | std::mem::drop(val)       | Erta tushirish — scope tugamasdan             | Раннее удаление — до конца области                         |
// # |   5 | Copy + Drop               | Birga bo'la olmaydi                           | Не могут быть вместе                                       |
// #================================================================================================================================================#
// # |                                       DROP TARTIBI                                                                                           |
// #================================================================================================================================================#
// # |   6 | Teskari tartib            | Oxirgi yaratilgan birinchi tushiriladi        | Последний созданный удаляется первым                       |
// # |   7 | Struct → Field            | Avval struct, keyin fieldlar tushiriladi      | Сначала структура, потом поля                              |
// # |   8 | Panic + drop()            | Panic bo'lganda ham drop() chaqiriladi        | drop() вызывается даже при панике                          |
// # |   9 | std::mem::forget()        | drop() chaqirmasdan — memory leak!            | Без вызова drop() — утечка памяти!                         |
// #================================================================================================================================================#
// # |                                       RAII PATTERN                                                                                           |
// #================================================================================================================================================#
// # |  10 | RAII                      | Resource Acquisition Is Initialization        | Получение ресурса при инициализации                        |
// # |  11 | Fayl → Drop               | Fayl scope tugaganda avtomatik yopiladi       | Файл закрывается автоматически                             |
// # |  12 | Tarmoq → Drop             | Ulanish scope tugaganda avtomatik uziladi     | Соединение закрывается автоматически                       |
// # |  13 | MutexGuard → Drop         | Qulf scope tugaganda avtomatik ochiladi       | Блокировка снимается автоматически                         |
// # |  14 | Xotira → Drop             | Heap xotira avtomatik ozod qilinadi           | Куча освобождается автоматически                           |
// #================================================================================================================================================#