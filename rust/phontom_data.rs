// #================================================================================================================================================#
// #                                                            PHANTOMDATA                                                                         #
// #                        PHANTOMDATA — ZERO-SIZE MARKER. VARIANCE. OWNERSHIP. DROP CHECK. TURLAR BILAN ISHLASH.                                  #
// #                        PHANTOMDATA — МАРКЕР НУЛЕВОГО РАЗМЕРА. VARIANCE. OWNERSHIP. DROP CHECK. РАБОТА С ТИПАМИ.                                #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::marker::PhantomData;
use std::fmt;
use std::ptr::NonNull;

// PhantomData nima:
// Что такое PhantomData:
//
//   PhantomData<T> — zero-size type, runtime qiymati yo'q
//   PhantomData<T> — тип нулевого размера, нет значения в runtime
//
//   Nima uchun kerak:
//   Зачем нужен:
//   1. Variance — tur parametrining variance ni boshqarish
//      Variance — управление вариантностью параметра типа
//   2. Ownership — kompilatorga T ni own qilishni bildirish
//      Ownership — указать компилятору что владеем T
//   3. Drop check — T drop bo'lishi mumkinligini bildirish
//      Drop check — указать что T может быть dropped
//   4. !Send / !Sync — marker qo'shish
//      !Send / !Sync — добавление маркера
//   5. Lifetime — struct da lifetime saqlash
//      Lifetime — хранение lifetime в структуре
//
//   O'lcham: mem::size_of::<PhantomData<T>>() == 0
//   Размер: mem::size_of::<PhantomData<T>>() == 0

fn phantomdata_asosiy() {

    // PhantomData — zero-size
    // PhantomData — нулевой размер
    println!("PhantomData<i32>:    {} bayt", std::mem::size_of::<PhantomData<i32>>());
    println!("PhantomData<String>: {} bayt", std::mem::size_of::<PhantomData<String>>());
    println!("PhantomData<Vec<u8>>: {} bayt", std::mem::size_of::<PhantomData<Vec<u8>>>());
    // PhantomData<i32>:    0 bayt
    // PhantomData<String>: 0 bayt
    // PhantomData<Vec<u8>>: 0 bayt

    // Struct bilan — hech qanday xotira qo'shilmaydi
    // В структуре — не добавляет памяти
    struct A { n: i32 }
    struct B { n: i32, _m: PhantomData<String> }
    println!("A o'lcham: {} bayt", std::mem::size_of::<A>());
    println!("B o'lcham: {} bayt", std::mem::size_of::<B>());
    // A o'lcham: 4 bayt
    // B o'lcham: 4 bayt

    // Turli PhantomData variantlari
    // Различные варианты PhantomData
    struct Covariant<T>     { _m: PhantomData<T> }        // covariant over T
    struct Contravariant<T> { _m: PhantomData<fn(T)> }    // contravariant over T
    struct Invariant<T>     { _m: PhantomData<fn(T) -> T> } // invariant over T
    struct OwnerOf<T>       { _m: PhantomData<T> }        // T ni own qiladi
    struct BorrowOf<'a, T>  { _m: PhantomData<&'a T> }    // &'a T borrow

    println!("Turli PhantomData varianta ishladi");
}

// COVARIANT — 'uzun 'qisqa o'rnida ishlatilishi mumkin
// COVARIANT — 'long можно использовать вместо 'short
// PhantomData<T> — T bo'yicha covariant

struct CovariantWrapper<'a> {
    _m: PhantomData<&'a str>,
}

impl<'a> CovariantWrapper<'a> {
    fn new() -> Self { CovariantWrapper { _m: PhantomData } }
}

// CONTRAVARIANT — aksincha
// CONTRAVARIANT — наоборот
// PhantomData<fn(T)> — T bo'yicha contravariant
struct ContravariantWrapper<T> {
    _m: PhantomData<fn(T)>,
}

impl<T> ContravariantWrapper<T> {
    fn new() -> Self { ContravariantWrapper { _m: PhantomData } }
}

// INVARIANT — na katta, na kichik
// INVARIANT — ни больше, ни меньше
// PhantomData<fn(T) -> T> — T bo'yicha invariant
struct InvariantWrapper<T> {
    _m: PhantomData<fn(T) -> T>,
}

impl<T> InvariantWrapper<T> {
    fn new() -> Self { InvariantWrapper { _m: PhantomData } }
}

fn variance_misollari() {

    // Covariance — uzun lifetime qisqa o'rnida
    // Covariance — длинный lifetime вместо короткого
    let uzun_str = String::from("uzun yashaydi");
    let wrapper: CovariantWrapper = CovariantWrapper::new();
    // 'uzun 'qisqa o'rnida — covariant

    println!("Covariant PhantomData<&'a T>: ✅");
    println!("Contravariant PhantomData<fn(T)>: ✅");
    println!("Invariant PhantomData<fn(T)->T>: ✅");

    // Variance jadvali
    println!("\nVariance jadvali:");
    println!("  PhantomData<T>         → T bo'yicha COVARIANT");
    println!("  PhantomData<fn(T)>     → T bo'yicha CONTRAVARIANT");
    println!("  PhantomData<fn(T)->T>  → T bo'yicha INVARIANT");
    println!("  PhantomData<*mut T>    → T bo'yicha INVARIANT");
    println!("  PhantomData<*const T>  → T bo'yicha COVARIANT");
    // Variance jadvali:
    //   PhantomData<T>         → T bo'yicha COVARIANT
    //   ...
}

// Raw pointer ishlatganda — kompilyator ownership bilmaydi
// При использовании raw pointer — компилятор не знает о владении
// PhantomData<T> — "Men T ni own qilaman" deb bildirish
// PhantomData<T> — сообщить "Я владею T"

struct RawVec<T> {
    ptr: NonNull<T>,
    uzunlik: usize,
    sig_im: usize,
    _marker: PhantomData<T>, // "Men T elementlarni own qilaman"
    // PhantomData<T> bo'lmasa — T drop bo'lmaydi!
}

impl<T> RawVec<T> {
    fn new() -> Self {
        RawVec {
            ptr: NonNull::dangling(),
            uzunlik: 0,
            sig_im: 0,
            _marker: PhantomData,
        }
    }

    fn push(&mut self, val: T) {
        if self.uzunlik == self.sig_im {
            let yangi_sig_im = if self.sig_im == 0 { 4 } else { self.sig_im * 2 };
            let layout = std::alloc::Layout::array::<T>(yangi_sig_im).unwrap();

            let yangi_ptr = if self.sig_im == 0 {
                unsafe { std::alloc::alloc(layout) as *mut T }
            } else {
                let eski_layout = std::alloc::Layout::array::<T>(self.sig_im).unwrap();
                unsafe { std::alloc::realloc(self.ptr.as_ptr() as *mut u8, eski_layout, layout.size()) as *mut T }
            };

            self.ptr = NonNull::new(yangi_ptr).expect("Xotira ajratish xatosi!");
            self.sig_im = yangi_sig_im;
        }

        unsafe { self.ptr.as_ptr().add(self.uzunlik).write(val); }
        self.uzunlik += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.uzunlik == 0 { return None; }
        self.uzunlik -= 1;
        Some(unsafe { self.ptr.as_ptr().add(self.uzunlik).read() })
    }

    fn ol(&self, i: usize) -> Option<&T> {
        if i >= self.uzunlik { return None; }
        Some(unsafe { &*self.ptr.as_ptr().add(i) })
    }

    fn uzunlik(&self) -> usize { self.uzunlik }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.sig_im > 0 {
            unsafe {
                // T elementlarni drop qilish — PhantomData<T> tufayli drop check ishlaydi
                // Drop T элементов — работает из-за PhantomData<T>
                for i in 0..self.uzunlik {
                    std::ptr::drop_in_place(self.ptr.as_ptr().add(i));
                }
                let layout = std::alloc::Layout::array::<T>(self.sig_im).unwrap();
                std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for RawVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.uzunlik {
            if i > 0 { write!(f, ", ")?; }
            unsafe { write!(f, "{:?}", &*self.ptr.as_ptr().add(i))?; }
        }
        write!(f, "]")
    }
}

fn raw_vec_misoli() {

    let mut v: RawVec<String> = RawVec::new();
    v.push(String::from("salom"));
    v.push(String::from("dunyo"));
    v.push(String::from("rust"));

    println!("{:?}", v);
    println!("Uzunlik: {}", v.uzunlik());
    println!("{:?}", v.ol(1));
    println!("{:?}", v.pop());
    println!("{:?}", v);
    // ["salom", "dunyo", "rust"]
    // Uzunlik: 3
    // Some("dunyo")
    // Some("rust")
    // ["salom", "dunyo"]
}

// Lifetime saqlash — PhantomData<&'a T>
// Хранение lifetime — PhantomData<&'a T>
struct Iter<'a, T: 'a> {
    ptr: *const T,
    end: *const T,
    _marker: PhantomData<&'a T>, // 'a lifetime saqlanadi
}

impl<'a, T> Iter<'a, T> {
    fn new(slice: &'a [T]) -> Self {
        let ptr = slice.as_ptr();
        let end = unsafe { ptr.add(slice.len()) };
        Iter { ptr, end, _marker: PhantomData }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if self.ptr == self.end {
            None
        } else {
            let item = unsafe { &*self.ptr };
            self.ptr = unsafe { self.ptr.add(1) };
            Some(item)
        }
    }
}

// IterMut — &'a mut T bilan
// IterMut — с &'a mut T
struct IterMut<'a, T: 'a> {
    ptr: *mut T,
    end: *mut T,
    _marker: PhantomData<&'a mut T>, // invariant over T (mut reference)
}

impl<'a, T> IterMut<'a, T> {
    fn new(slice: &'a mut [T]) -> Self {
        let ptr = slice.as_mut_ptr();
        let end = unsafe { ptr.add(slice.len()) };
        IterMut { ptr, end, _marker: PhantomData }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        if self.ptr == self.end {
            None
        } else {
            let item = unsafe { &mut *self.ptr };
            self.ptr = unsafe { self.ptr.add(1) };
            Some(item)
        }
    }
}

fn iter_misollari() {

    let v = vec![10, 20, 30, 40, 50];
    let mut iter = Iter::new(&v);

    while let Some(x) = iter.next() {
        print!("{} ", x);
    }
    println!();
    // 10 20 30 40 50

    let mut v2 = vec![1, 2, 3, 4, 5];
    let iter_mut = IterMut::new(&mut v2);
    for x in iter_mut {
        *x *= 2;
    }
    println!("{:?}", v2);
    // [2, 4, 6, 8, 10]
}

// Type State — compile time holat mashinasi
// Type State — машина состояний во время компиляции
struct Yopilgan;
struct Ochilgan;
struct Yuborilmoqda;

struct Ulanish<Holat> {
    manzil: String,
    _holat: PhantomData<Holat>,
}

impl Ulanish<Yopilgan> {
    fn yangi(manzil: &str) -> Self {
        println!("[Ulanish] Yaratildi: {}", manzil);
        Ulanish { manzil: manzil.to_string(), _holat: PhantomData }
    }

    fn ochish(self) -> Ulanish<Ochilgan> {
        println!("[Ulanish] Ochildi: {}", self.manzil);
        Ulanish { manzil: self.manzil, _holat: PhantomData }
    }
}

impl Ulanish<Ochilgan> {
    fn so_rov_boshlash(self) -> Ulanish<Yuborilmoqda> {
        println!("[Ulanish] So'rov yuborilmoqda...");
        Ulanish { manzil: self.manzil, _holat: PhantomData }
    }

    fn yopish(self) -> Ulanish<Yopilgan> {
        println!("[Ulanish] Yopildi: {}", self.manzil);
        Ulanish { manzil: self.manzil, _holat: PhantomData }
    }
}

impl Ulanish<Yuborilmoqda> {
    fn natija_olish(self) -> (Ulanish<Ochilgan>, String) {
        println!("[Ulanish] Natija olindi");
        let javob = format!("Javob: {}", self.manzil);
        (Ulanish { manzil: self.manzil, _holat: PhantomData }, javob)
    }
}

fn type_state_misoli() {

    // Compile time da holat tekshiriladi
    // Состояние проверяется во время компиляции
    let ulanish = Ulanish::yangi("https://api.example.com");
    // ulanish.so_rov_boshlash(); // ← KOMPILE XATO! Yopilgan holat!

    let ochiq = ulanish.ochish();
    let yuborilmoqda = ochiq.so_rov_boshlash();
    let (ochiq2, javob) = yuborilmoqda.natija_olish();
    println!("Javob: {}", javob);
    let _yopildi = ochiq2.yopish();
    // [Ulanish] Yaratildi: https://api.example.com
    // [Ulanish] Ochildi: https://api.example.com
    // [Ulanish] So'rov yuborilmoqda...
    // [Ulanish] Natija olindi
    // Javob: Javob: https://api.example.com
    // [Ulanish] Yopildi: https://api.example.com
}

// PhantomData<*mut ()> — !Send + !Sync qilish
// PhantomData<*mut ()> — сделать !Send + !Sync
struct FaqatAsosiyThread {
    qiymat: i32,
    _unsend: PhantomData<*mut ()>, // *mut (): !Send + !Sync
}

impl FaqatAsosiyThread {
    fn new(q: i32) -> Self {
        FaqatAsosiyThread { qiymat: q, _unsend: PhantomData }
    }
    fn qiymat(&self) -> i32 { self.qiymat }
}

// PhantomData<Rc<()>> — !Send qilish (Sync saqlanadi)
// PhantomData<Rc<()>> — сделать !Send (Sync сохраняется)
use std::rc::Rc;
struct NotSend {
    _m: PhantomData<Rc<()>>,
}

// PhantomData<UnsafeCell<()>> — !Sync qilish (Send saqlanadi)
// PhantomData<UnsafeCell<()>> — сделать !Sync (Send сохраняется)
use std::cell::UnsafeCell;
struct NotSync {
    _m: PhantomData<UnsafeCell<()>>,
}

fn marker_misollari() {

    let obj = FaqatAsosiyThread::new(42);
    println!("Qiymat: {}", obj.qiymat());

    // obj ni thread ga uzatish mumkin emas — *mut () !Send
    // Нельзя передать obj в поток — *mut () !Send
    // thread::spawn(move || println!("{}", obj.qiymat())); // ← KOMPILE XATO

    println!("PhantomData<*mut ()>: !Send + !Sync ✅");
    println!("PhantomData<Rc<()>>: !Send ✅");
    println!("PhantomData<UnsafeCell<()>>: !Sync ✅");
    // Qiymat: 42
    // PhantomData<*mut ()>: !Send + !Sync ✅
}

// Birlik tizimi — turli fizik birliklarni aralashmaslik
// Система единиц — не перепутать разные физические единицы
struct Metr;
struct Sekund;
struct Kilogramm;

struct Olchov<Birlik> {
    qiymat: f64,
    _birlik: PhantomData<Birlik>,
}

impl<Birlik> Olchov<Birlik> {
    fn new(qiymat: f64) -> Self {
        Olchov { qiymat, _birlik: PhantomData }
    }
    fn qiymat(&self) -> f64 { self.qiymat }
}

impl<Birlik> std::ops::Add for Olchov<Birlik> {
    type Output = Self;
    fn add(self, b: Self) -> Self { Olchov::new(self.qiymat + b.qiymat) }
}

impl<Birlik> std::ops::Mul<f64> for Olchov<Birlik> {
    type Output = Self;
    fn mul(self, k: f64) -> Self { Olchov::new(self.qiymat * k) }
}

impl fmt::Display for Olchov<Metr> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} m", self.qiymat)
    }
}

impl fmt::Display for Olchov<Sekund> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} s", self.qiymat)
    }
}

fn birlik_tizimi_misoli() {

    let a: Olchov<Metr>   = Olchov::new(5.0);
    let b: Olchov<Metr>   = Olchov::new(3.0);
    let c: Olchov<Metr>   = a + b;
    println!("{}", c); // 8.00 m

    let t: Olchov<Sekund> = Olchov::new(2.5);
    let t2: Olchov<Sekund> = t * 2.0;
    println!("{}", t2); // 5.00 s

    // Bu KOMPILE BO'LMAYDI — birlik noto'g'ri:
    // Это НЕ СКОМПИЛИРУЕТСЯ — неправильная единица:
    // let d: Olchov<Metr> = a + t; // ← Metr + Sekund = XATO!

    println!("Olchov<Metr> o'lcham: {} bayt", std::mem::size_of::<Olchov<Metr>>());
    println!("f64 o'lcham:          {} bayt", std::mem::size_of::<f64>());
    // Olchov<Metr> o'lcham: 8 bayt — bir xil! PhantomData zero-cost
    // f64 o'lcham:          8 bayt
}

// ID tizimi — turlar orasida ID ni aralashmaslik
// Система ID — не перепутать ID разных типов
struct Foydalanuvchi;
struct Mahsulot;
struct Buyurtma;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Id<T> {
    qiymat: u64,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    fn new(qiymat: u64) -> Self { Id { qiymat, _marker: PhantomData } }
    fn qiymat(&self) -> u64 { self.qiymat }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.qiymat)
    }
}

// Sodda builder — PhantomData bilan
// Простой builder — с PhantomData
struct Sozlanmagan;
struct Sozlangan;

struct ServerBuilder<Holat> {
    host: String,
    port: Option<u16>,
    workers: Option<u32>,
    _holat: PhantomData<Holat>,
}

impl ServerBuilder<Sozlanmagan> {
    fn new(host: &str) -> Self {
        ServerBuilder {
            host: host.to_string(),
            port: None,
            workers: None,
            _holat: PhantomData,
        }
    }

    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    fn workers(mut self, n: u32) -> Self {
        self.workers = Some(n);
        self
    }

    fn qur(self) -> Result<ServerBuilder<Sozlangan>, String> {
        if self.port.is_none() {
            return Err("Port ko'rsatilmagan!".to_string());
        }
        Ok(ServerBuilder {
            host: self.host,
            port: self.port,
            workers: self.workers,
            _holat: PhantomData,
        })
    }
}

impl ServerBuilder<Sozlangan> {
    fn ishga_tushir(&self) {
        println!("Server: {}:{} ({} worker)",
                 self.host,
                 self.port.unwrap(),
                 self.workers.unwrap_or(4)
        );
    }
}

fn real_hayot_misollari() {

    println!("--- ID Tizimi ---");
    let f_id: Id<Foydalanuvchi> = Id::new(1001);
    let m_id: Id<Mahsulot>      = Id::new(2002);
    let b_id: Id<Buyurtma>      = Id::new(3003);

    println!("Foydalanuvchi ID: {}", f_id);
    println!("Mahsulot ID:      {}", m_id);
    println!("Buyurtma ID:      {}", b_id);
    println!("O'lcham: {} bayt", std::mem::size_of::<Id<Foydalanuvchi>>());

    // Bu KOMPILE BO'LMAYDI:
    // Это НЕ СКОМПИЛИРУЕТСЯ:
    // if f_id == m_id { } // ← Foydalanuvchi != Mahsulot
    // Foydalanuvchi ID: #1001
    // Mahsulot ID:      #2002
    // Buyurtma ID:      #3003
    // O'lcham: 8 bayt

    println!("\n--- Server Builder ---");
    match ServerBuilder::new("localhost").port(8080).workers(8).qur() {
        Ok(server) => server.ishga_tushir(),
        Err(e)     => println!("Xato: {}", e),
    }
    // Server: localhost:8080 (8 worker)

    match ServerBuilder::new("localhost").workers(4).qur() {
        Ok(server) => server.ishga_tushir(),
        Err(e)     => println!("Xato: {}", e),
    }
    // Xato: Port ko'rsatilmagan!

    println!("\n--- RawVec ---");
    raw_vec_misoli();

    println!("\n--- Birlik Tizimi ---");
    birlik_tizimi_misoli();

    println!("\n--- Type State ---");
    type_state_misoli();
}

fn main() {

    println!("=== PHANTOMDATA ASOSIY ===");
    phantomdata_asosiy();

    println!("\n=== VARIANCE ===");
    variance_misollari();

    println!("\n=== ITER BILAN ===");
    iter_misollari();

    println!("\n=== MARKER ===");
    marker_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        PHANTOMDATA ASOSLARI                                                                                  |
// #================================================================================================================================================#
// # |   1 | PhantomData<T>                  | Zero-size, runtime qiymati yo'q            | Нулевой размер, нет значения в runtime                  |
// # |   2 | PhantomData<T>                  | T bo'yicha COVARIANT                       | COVARIANT по T                                          |
// # |   3 | PhantomData<fn(T)>              | T bo'yicha CONTRAVARIANT                   | CONTRAVARIANT по T                                      |
// # |   4 | PhantomData<fn(T)->T>           | T bo'yicha INVARIANT                       | INVARIANT по T                                          |
// # |   5 | PhantomData<*mut T>             | T bo'yicha INVARIANT                       | INVARIANT по T                                          |
// #================================================================================================================================================#
// # |                                        NIMA UCHUN KERAK                                                                                      |
// #================================================================================================================================================#
// # |   6 | Ownership bildirish             | Raw ptr T ni own qilishini bildirish        | Сообщить что raw ptr владеет T                         |
// # |   7 | Drop check                      | T drop bo'lishini bildirish                 | Указать что T может быть dropped                       |
// # |   8 | PhantomData<&'a T>              | 'a lifetime saqlash                         | Хранение lifetime 'a                                   |
// # |   9 | PhantomData<*mut ()>            | !Send + !Sync qilish                        | Сделать !Send + !Sync                                  |
// # |  10 | PhantomData<Rc<()>>             | !Send qilish                                | Сделать !Send                                          |
// # |  11 | PhantomData<UnsafeCell<()>>     | !Sync qilish                                | Сделать !Sync                                          |
// #================================================================================================================================================#
// # |                                        PATTERNLAR                                                                                            |
// #================================================================================================================================================#
// # |  12 | Type State Pattern              | Compile time holat mashinasi                | Машина состояний во время компиляции                   |
// # |  13 | Birlik tizimi                   | Olchov<Metr> vs Olchov<Sekund>              | Olchov<Metr> vs Olchov<Sekund>                         |
// # |  14 | Id<T>                           | Tur-xavfsiz ID tizimi                       | Типобезопасная система ID                              |
// # |  15 | Custom iterator                 | PhantomData<&'a T> bilan lifetime           | Lifetime с PhantomData<&'a T>                          |
// # |  16 | RawVec                          | PhantomData<T> bilan ownership              | Владение с PhantomData<T>                              |
// #================================================================================================================================================#