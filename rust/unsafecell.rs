// #================================================================================================================================================#
// #                                                                UNSAFECELL                                                                      #
// #                UNSAFECELL — INTERIOR MUTABILITY ASOSI. CELL, REFCELL, MUTEX ICHIDA. RAW POINTER ORQALI O'ZGARTIRISH.                           #
// #                UNSAFECELL — ОСНОВА INTERIOR MUTABILITY. ВНУТРИ CELL, REFCELL, MUTEX. ИЗМЕНЕНИЕ ЧЕРЕЗ СЫРОЙ УКАЗАТЕЛЬ.                          #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::cell::{UnsafeCell, Cell, RefCell};
use std::sync::{Mutex, RwLock};
use std::ptr;
use std::fmt;

// UnsafeCell nima:
// Что такое UnsafeCell:
//
//   - Rustning yagona "qonuniy" interior mutability mexanizmi
//   - Единственный "легальный" механизм interior mutability в Rust
//   - &T orqali T ni o'zgartirish mumkin qiladi
//   - Позволяет изменять T через &T
//   - Barcha interior mutability UnsafeCell asosida qurilgan:
//   - Все interior mutability построены на UnsafeCell:
//       Cell<T>    — UnsafeCell<T> wrapper
//       RefCell<T> — UnsafeCell<T> + borrow tracking
//       Mutex<T>   — UnsafeCell<T> + OS lock
//       RwLock<T>  — UnsafeCell<T> + read-write lock
//       AtomicT    — UnsafeCell<T> + atomic ops
//
//   UnsafeCell<T>: !Sync — thread safe emas!
//   UnsafeCell<T>: !Sync — не потокобезопасен!
//
//   get() — *mut T raw pointer qaytaradi
//   get() — возвращает *mut T сырой указатель
//   Dereference — unsafe blok ichida

fn unsafecell_asosiy_misollari() {

    // UnsafeCell::new — yaratish
    // UnsafeCell::new — создание
    let cell: UnsafeCell<i32> = UnsafeCell::new(42);

    // get() — *mut T raw pointer
    // get() — *mut T сырой указатель
    let ptr: *mut i32 = cell.get();

    // O'qish
    // Чтение
    unsafe {
        println!("{}", *ptr); // 42
    }

    // &cell orqali o'zgartirish — interior mutability!
    // Изменение через &cell — interior mutability!
    unsafe {
        *ptr = 100;
    }
    println!("{}", unsafe { *cell.get() }); // 100
    // 42
    // 100

    // into_inner() — qiymatni consume qilib olish
    // into_inner() — взять значение с consume
    let cell2: UnsafeCell<String> = UnsafeCell::new(String::from("salom"));
    let s: String = cell2.into_inner();
    println!("{}", s); // salom
    // salom

    // get_mut() — &mut T (faqat bitta owner bo'lsa)
    // get_mut() — &mut T (только если один владелец)
    let mut cell3: UnsafeCell<Vec<i32>> = UnsafeCell::new(vec![1, 2, 3]);
    cell3.get_mut().push(4);
    println!("{:?}", cell3.into_inner()); // [1, 2, 3, 4]
    // [1, 2, 3, 4]

    // raw_get — pointer dan pointer (const self)
    // raw_get — из указателя в указатель (const self)
    let cell4: UnsafeCell<f64> = UnsafeCell::new(3.14);
    let ptr4: *mut f64 = UnsafeCell::raw_get(&cell4 as *const _ as *const UnsafeCell<f64>);
    unsafe {
        println!("{}", *ptr4); // 3.14
    }
    // 3.14

    // O'lcham — UnsafeCell<T> == T
    // Размер — UnsafeCell<T> == T
    println!("i32:             {} bayt", std::mem::size_of::<i32>());
    println!("UnsafeCell<i32>: {} bayt", std::mem::size_of::<UnsafeCell<i32>>());
    // i32:             4 bayt
    // UnsafeCell<i32>: 4 bayt
}

// Cell<T> ichida UnsafeCell<T> bor
// Внутри Cell<T> находится UnsafeCell<T>
// struct Cell<T> { value: UnsafeCell<T> }

fn cell_ichki_ishlash() {

    // Cell — Copy turlar uchun interior mutability
    // Cell — interior mutability для Copy типов
    let cell: Cell<i32> = Cell::new(0);

    // get() — qiymatni copy qilib olish
    // get() — получить значение через copy
    println!("{}", cell.get()); // 0

    // set() — yangi qiymat o'rnatish
    // set() — установить новое значение
    cell.set(42);
    println!("{}", cell.get()); // 42

    // replace() — eski qiymatni qaytarib yangi o'rnatish
    // replace() — вернуть старое и установить новое
    let eski = cell.replace(100);
    println!("Eski: {}, Yangi: {}", eski, cell.get()); // 42, 100

    // update() — eski qiymat bilan yangilash
    // update() — обновление на основе старого значения
    cell.update(|v| v * 2);
    println!("{}", cell.get()); // 200
    // 0
    // 42
    // Eski: 42, Yangi: 100
    // 200

    // Cell — immutable reference orqali o'zgartirish
    // Cell — изменение через неизменяемую ссылку
    fn cell_orqali_o_zgart(cell: &Cell<i32>) {
        cell.set(cell.get() + 1);
    }

    let c = Cell::new(0);
    for _ in 0..5 {
        cell_orqali_o_zgart(&c);
    }
    println!("{}", c.get()); // 5
    // 5

    // Cell — struct ichida
    // Cell — в структуре
    struct Hisoblagich {
        qiymat: Cell<u32>,
        nomi: String,
    }

    impl Hisoblagich {
        fn new(nomi: &str) -> Self {
            Hisoblagich { qiymat: Cell::new(0), nomi: nomi.to_string() }
        }
        fn oshir(&self) { self.qiymat.set(self.qiymat.get() + 1); }
        fn qiymat(&self) -> u32 { self.qiymat.get() }
    }

    let h = Hisoblagich::new("test");
    h.oshir(); h.oshir(); h.oshir();
    println!("{}: {}", h.nomi, h.qiymat()); // test: 3
    // test: 3
}

// RefCell<T> ichida:
// Внутри RefCell<T>:
// struct RefCell<T> {
//     borrow: Cell<BorrowState>, // borrow hisob
//     value: UnsafeCell<T>,
// }

fn refcell_ichki_ishlash() {

    let cell: RefCell<Vec<i32>> = RefCell::new(vec![1, 2, 3]);

    // borrow() — &T (runtime da borrow tekshiruvi)
    // borrow() — &T (проверка borrow в runtime)
    {
        let r1 = cell.borrow();
        let r2 = cell.borrow(); // ko'p immutable borrow — OK
        println!("{:?} {:?}", *r1, *r2);
        // Scope tugaguncha r1, r2 yashaydi
    }
    // [1, 2, 3] [1, 2, 3]

    // borrow_mut() — &mut T
    // borrow_mut() — &mut T
    {
        let mut w = cell.borrow_mut();
        w.push(4);
        w.push(5);
    }
    println!("{:?}", cell.borrow()); // [1, 2, 3, 4, 5]
    // [1, 2, 3, 4, 5]

    // try_borrow() — panic bo'lmasdan
    // try_borrow() — без panic
    let r = cell.borrow();
    match cell.try_borrow_mut() {
        Ok(_)  => println!("Mut borrow olindi"),
        Err(e) => println!("Xato: {}", e), // Borrow tekshiruvi muvaffaqiyatsiz
    }
    drop(r);
    // Xato: already borrowed: BorrowMutError

    // RefCell — panicking holat
    // RefCell — паникующий случай
    // Bu panic qiladi:
    // Это вызовет панику:
    /*
    let r = cell.borrow();
    let w = cell.borrow_mut(); // ← PANIC! allaqachon borrowed
    */

    // into_inner() — qiymatni olish
    // into_inner() — взять значение
    let cell2: RefCell<String> = RefCell::new(String::from("salom"));
    let s = cell2.into_inner();
    println!("{}", s); // salom
    // salom

    // borrow_count — qancha borrow bor
    // borrow_count — сколько borrow активно
    let cell3: RefCell<i32> = RefCell::new(0);
    println!("Borrow soni: {}", 0); // oddiy tekshirish
    let _r1 = cell3.borrow();
    let _r2 = cell3.borrow();
    // drop(_r1) — keyin borrow_mut mumkin
}

// SyncCell — thread-safe Cell (Mutex wrapper, UnsafeCell asosida tushuntirish)
// SyncCell — потокобезопасный Cell (обёртка Mutex, объяснение на основе UnsafeCell)

// Cell<T> ni o'zimiz implement qilamiz — UnsafeCell asosida
// Реализуем Cell<T> сами — на основе UnsafeCell
struct MyCell<T: Copy> {
    ichki: UnsafeCell<T>,
}

impl<T: Copy> MyCell<T> {
    fn new(qiymat: T) -> Self {
        MyCell { ichki: UnsafeCell::new(qiymat) }
    }

    fn get(&self) -> T {
        unsafe { *self.ichki.get() }
    }

    fn set(&self, qiymat: T) {
        unsafe { *self.ichki.get() = qiymat; }
    }

    fn update<F: FnOnce(T) -> T>(&self, f: F) {
        self.set(f(self.get()));
    }

    fn replace(&self, yangi: T) -> T {
        let eski = self.get();
        self.set(yangi);
        eski
    }
}

impl<T: Copy + fmt::Debug> fmt::Debug for MyCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyCell({:?})", self.get())
    }
}

// MyRefCell — runtime borrow tekshiruvi bilan
// MyRefCell — с проверкой borrow в runtime
struct MyRefCell<T> {
    ichki: UnsafeCell<T>,
    borrow_soni: Cell<i32>, // musbat: immutable, -1: mutable
}

impl<T> MyRefCell<T> {
    fn new(qiymat: T) -> Self {
        MyRefCell {
            ichki: UnsafeCell::new(qiymat),
            borrow_soni: Cell::new(0),
        }
    }

    fn borrow(&self) -> MyRef<'_, T> {
        let hozir = self.borrow_soni.get();
        if hozir < 0 {
            panic!("allaqachon mut borrowed!");
        }
        self.borrow_soni.set(hozir + 1);
        MyRef { cell: self }
    }

    fn borrow_mut(&self) -> MyRefMut<'_, T> {
        let hozir = self.borrow_soni.get();
        if hozir != 0 {
            panic!("allaqachon borrowed!");
        }
        self.borrow_soni.set(-1);
        MyRefMut { cell: self }
    }
}

struct MyRef<'a, T> {
    cell: &'a MyRefCell<T>,
}

impl<'a, T> std::ops::Deref for MyRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.cell.ichki.get() }
    }
}

impl<'a, T> Drop for MyRef<'a, T> {
    fn drop(&mut self) {
        self.cell.borrow_soni.set(self.cell.borrow_soni.get() - 1);
    }
}

struct MyRefMut<'a, T> {
    cell: &'a MyRefCell<T>,
}

impl<'a, T> std::ops::Deref for MyRefMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.cell.ichki.get() }
    }
}

impl<'a, T> std::ops::DerefMut for MyRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.cell.ichki.get() }
    }
}

impl<'a, T> Drop for MyRefMut<'a, T> {
    fn drop(&mut self) {
        self.cell.borrow_soni.set(0);
    }
}

fn custom_cell_misollari() {

    // MyCell
    let c: MyCell<i32> = MyCell::new(10);
    println!("{:?}", c);         // MyCell(10)
    c.set(20);
    println!("{:?}", c);         // MyCell(20)
    c.update(|v| v * 3);
    println!("{:?}", c);         // MyCell(60)
    println!("{}", c.replace(0)); // 60
    println!("{:?}", c);          // MyCell(0)
    // MyCell(10)
    // MyCell(20)
    // MyCell(60)
    // 60
    // MyCell(0)

    // MyRefCell
    let rc: MyRefCell<Vec<i32>> = MyRefCell::new(vec![1, 2, 3]);

    {
        let r1 = rc.borrow();
        let r2 = rc.borrow(); // ko'p immutable OK
        println!("{:?}", *r1);
        println!("Borrow soni: {}", rc.borrow_soni.get()); // 2
    } // r1, r2 drop

    {
        let mut w = rc.borrow_mut();
        w.push(4);
        println!("{:?}", *w);
    } // w drop
    // [1, 2, 3]
    // Borrow soni: 2
    // [1, 2, 3, 4]
}

// Lazy initialization — UnsafeCell bilan
// Ленивая инициализация — с UnsafeCell
struct LazyField<T> {
    ichki: UnsafeCell<Option<T>>,
}

impl<T> LazyField<T> {
    fn new() -> Self {
        LazyField { ichki: UnsafeCell::new(None) }
    }

    fn get_or_init<F: FnOnce() -> T>(&self, f: F) -> &T {
        unsafe {
            if (*self.ichki.get()).is_none() {
                *self.ichki.get() = Some(f());
            }
            (*self.ichki.get()).as_ref().unwrap()
        }
    }

    fn bajarildi(&self) -> bool {
        unsafe { (*self.ichki.get()).is_some() }
    }
}

// Memoization — UnsafeCell bilan
// Мемоизация — с UnsafeCell
struct Memoize<T: Clone> {
    funksiya: fn() -> T,
    kesh: UnsafeCell<Option<T>>,
}

impl<T: Clone + fmt::Debug> Memoize<T> {
    fn new(f: fn() -> T) -> Self {
        Memoize { funksiya: f, kesh: UnsafeCell::new(None) }
    }

    fn qo_ng_ir_ol(&self) -> T {
        unsafe {
            if let Some(ref v) = *self.kesh.get() {
                println!("[Keshdan] {:?}", v);
                return v.clone();
            }
            let natija = (self.funksiya)();
            println!("[Hisoblandi] {:?}", natija);
            *self.kesh.get() = Some(natija.clone());
            natija
        }
    }
}

fn interior_mutability_patternlari() {

    // LazyField
    let lazy: LazyField<String> = LazyField::new();
    println!("Bajarildi: {}", lazy.bajarildi()); // false

    let v = lazy.get_or_init(|| {
        println!("Initsializatsiya...");
        String::from("lazy qiymat")
    });
    println!("{}", v);
    println!("Bajarildi: {}", lazy.bajarildi()); // true

    let v2 = lazy.get_or_init(|| {
        println!("Bu chiqmaydi!");
        String::from("boshqa")
    });
    println!("{}", v2);
    // Bajarildi: false
    // Initsializatsiya...
    // lazy qiymat
    // Bajarildi: true
    // lazy qiymat

    // Memoize
    fn qimmat_hisoblash() -> Vec<u64> {
        (1u64..=5).map(|n| n * n * n).collect()
    }

    let memo: Memoize<Vec<u64>> = Memoize::new(qimmat_hisoblash);
    memo.qo_ng_ir_ol(); // birinchi — hisoblaydi
    memo.qo_ng_ir_ol(); // ikkinchi — keshdan
    memo.qo_ng_ir_ol(); // uchinchi — keshdan
    // [Hisoblandi] [1, 8, 27, 64, 125]
    // [Keshdan] [1, 8, 27, 64, 125]
    // [Keshdan] [1, 8, 27, 64, 125]
}

// Statistika yig'uvchi — Cell bilan
// Сборщик статистики — с Cell
struct Statistika {
    so_rovlar: Cell<u64>,
    muvaffaqiyatlar: Cell<u64>,
    xatolar: Cell<u64>,
    jami_ms: Cell<u64>,
}

impl Statistika {
    fn new() -> Self {
        Statistika {
            so_rovlar: Cell::new(0),
            muvaffaqiyatlar: Cell::new(0),
            xatolar: Cell::new(0),
            jami_ms: Cell::new(0),
        }
    }

    fn so_rov_qo_sh(&self, muvaffaqiyatli: bool, ms: u64) {
        self.so_rovlar.update(|v| v + 1);
        self.jami_ms.update(|v| v + ms);
        if muvaffaqiyatli {
            self.muvaffaqiyatlar.update(|v| v + 1);
        } else {
            self.xatolar.update(|v| v + 1);
        }
    }

    fn o_rtacha_ms(&self) -> f64 {
        let s = self.so_rovlar.get();
        if s == 0 { return 0.0; }
        self.jami_ms.get() as f64 / s as f64
    }

    fn chiqar(&self) {
        println!("So'rovlar: {}", self.so_rovlar.get());
        println!("Muvaffaqiyat: {}", self.muvaffaqiyatlar.get());
        println!("Xatolar: {}", self.xatolar.get());
        println!("O'rtacha: {:.1}ms", self.o_rtacha_ms());
    }
}

// Graf — RefCell bilan (aylana havolalar)
// Граф — с RefCell (циклические ссылки)
use std::rc::Rc;

struct GrafTugun {
    id: u32,
    qo_shnilar: RefCell<Vec<Rc<GrafTugun>>>,
}

impl GrafTugun {
    fn new(id: u32) -> Rc<Self> {
        Rc::new(GrafTugun {
            id,
            qo_shnilar: RefCell::new(vec![]),
        })
    }

    fn qo_shnini_qo_sh(tugun: &Rc<Self>, qo_shni: Rc<GrafTugun>) {
        tugun.qo_shnilar.borrow_mut().push(qo_shni);
    }

    fn qo_shnilar_id(&self) -> Vec<u32> {
        self.qo_shnilar.borrow().iter().map(|t| t.id).collect()
    }
}

fn real_hayot_misollari() {

    println!("--- Statistika ---");
    let stat = Statistika::new();

    let so_rovlar = [
        (true, 45u64), (true, 32), (false, 120), (true, 28),
        (false, 85), (true, 55), (true, 41), (true, 38),
    ];

    for &(muvaffaqiyatli, ms) in &so_rovlar {
        stat.so_rov_qo_sh(muvaffaqiyatli, ms);
    }
    stat.chiqar();
    // So'rovlar: 8
    // Muvaffaqiyat: 6
    // Xatolar: 2
    // O'rtacha: 55.5ms

    println!("\n--- Graf (RefCell) ---");
    let t1 = GrafTugun::new(1);
    let t2 = GrafTugun::new(2);
    let t3 = GrafTugun::new(3);
    let t4 = GrafTugun::new(4);

    GrafTugun::qo_shnini_qo_sh(&t1, Rc::clone(&t2));
    GrafTugun::qo_shnini_qo_sh(&t1, Rc::clone(&t3));
    GrafTugun::qo_shnini_qo_sh(&t2, Rc::clone(&t3));
    GrafTugun::qo_shnini_qo_sh(&t2, Rc::clone(&t4));
    GrafTugun::qo_shnini_qo_sh(&t3, Rc::clone(&t4));

    println!("T1 qo'shnilari: {:?}", t1.qo_shnilar_id());
    println!("T2 qo'shnilari: {:?}", t2.qo_shnilar_id());
    println!("T3 qo'shnilari: {:?}", t3.qo_shnilar_id());
    // T1 qo'shnilari: [2, 3]
    // T2 qo'shnilari: [3, 4]
    // T3 qo'shnilari: [4]

    println!("\n--- Interior Mutability Patternlari ---");
    interior_mutability_patternlari();
}

fn main() {

    println!("=== UNSAFECELL ASOSIY ===");
    unsafecell_asosiy_misollari();

    println!("\n=== CELL<T> ICHKI ISHLASH ===");
    cell_ichki_ishlash();

    println!("\n=== REFCELL<T> ICHKI ISHLASH ===");
    refcell_ichki_ishlash();

    println!("\n=== CUSTOM CELL IMPLEMENTATSIYA ===");
    custom_cell_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        UNSAFECELL                                                                                            |
// #================================================================================================================================================#
// # |   1 | UnsafeCell::new(val)            | Yangi UnsafeCell yaratish                  | Создание нового UnsafeCell                              |
// # |   2 | cell.get()                      | *mut T raw pointer olish                   | Получить *mut T сырой указатель                         |
// # |   3 | cell.get_mut()                  | &mut T (bitta owner bo'lsa)                | &mut T (если один владелец)                             |
// # |   4 | cell.into_inner()               | Qiymatni consume qilib olish               | Взять значение с consume                                |
// # |   5 | UnsafeCell::raw_get(ptr)        | *const Self → *mut T                       | *const Self → *mut T                                    |
// # |   6 | !Sync                           | Thread-safe emas                           | Не потокобезопасен                                      |
// #================================================================================================================================================#
// # |                                        CELL<T> — UNSAFECELL WRAPPER                                                                          |
// #================================================================================================================================================#
// # |   7 | Cell::new(val)                  | Copy turlar uchun interior mutability      | Interior mutability для Copy типов                      |
// # |   8 | cell.get()                      | Qiymatni copy qilib olish                  | Получить значение через copy                            |
// # |   9 | cell.set(val)                   | Yangi qiymat o'rnatish                     | Установить новое значение                               |
// # |  10 | cell.update(|v| ...)            | Eski asosida yangilash                     | Обновить на основе старого                              |
// # |  11 | cell.replace(val)               | Eski qaytarib yangi o'rnatish              | Вернуть старое, установить новое                        |
// #================================================================================================================================================#
// # |                                        REFCELL<T>                                                                                            |
// #================================================================================================================================================#
// # |  12 | RefCell::new(val)               | Runtime borrow tekshiruvi                  | Проверка borrow в runtime                               |
// # |  13 | cell.borrow()                   | Ref<T> — immutable borrow                  | Ref<T> — неизменяемый borrow                            |
// # |  14 | cell.borrow_mut()               | RefMut<T> — mutable borrow                 | RefMut<T> — изменяемый borrow                           |
// # |  15 | cell.try_borrow()               | Panic bo'lmasdan                           | Без panic                                               |
// # |  16 | Runtime panic                   | Bir vaqtda mut+imm borrow → panic          | Одновременный mut+imm borrow → panic                    |
// #================================================================================================================================================#
// # |                                        QACHON NIMA ISHLATISH                                                                                 |
// #================================================================================================================================================#
// # |  17 | Cell<T: Copy>                   | Sodda Copy turlar, single-thread           | Простые Copy типы, один поток                           |
// # |  18 | RefCell<T>                      | Non-Copy, runtime borrow, single-thread    | Non-Copy, runtime borrow, один поток                    |
// # |  19 | Mutex<T>                        | Multi-thread, blocking lock                | Много потоков, блокирующий lock                         |
// # |  20 | RwLock<T>                       | Multi-thread, ko'p o'quvchi                | Много потоков, много читателей                          |
// # |  21 | Atomic<T>                       | Multi-thread, lock-free primitiv           | Много потоков, lock-free примитив                       |
// # |  22 | UnsafeCell<T>                   | Custom interior mutability qurishda        | При построении custom interior mutability               |
// #================================================================================================================================================#