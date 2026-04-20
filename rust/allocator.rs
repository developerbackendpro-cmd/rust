// #================================================================================================================================================#
// #                                                            ALLOCATOR_API                                                                       #
// #                        ALLOCATOR_API — CUSTOM ALLOCATOR. GLOBAL ALLOCATOR. BUMP ALLOCATOR. ARENA. POOL.                                        #
// #                        ALLOCATOR_API — КАСТОМНЫЙ АЛЛОКАТОР. ГЛОБАЛЬНЫЙ АЛЛОКАТОР. BUMP АЛЛОКАТОР. ARENA. POOL.                                 #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::alloc::{GlobalAlloc, Layout, System, alloc, dealloc, realloc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::ptr::NonNull;
use std::cell::UnsafeCell;
use std::fmt;

// Allocator API nima:
// Что такое Allocator API:
//
//   Rust da xotira ajratishni boshqarish
//   Управление выделением памяти в Rust
//
//   Turlar:
//   Виды:
//   #[global_allocator] — dastur uchun global allocator
//   #[global_allocator] — глобальный аллокатор для программы
//
//   GlobalAlloc trait:
//     unsafe fn alloc(&self, layout: Layout) -> *mut u8
//     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
//     unsafe fn realloc(...) -> *mut u8
//     unsafe fn alloc_zeroed(...) -> *mut u8
//
//   Nima uchun:
//   Зачем:
//   - Embedded/no_std (heap yo'q → soddal allocator)
//   - Performance (pool, arena — tez ajratish)
//   - Debugging (leak detection, usage tracking)
//   - Security (memory zeroing)
//   - Game dev (frame allocator)


// Xotira foydalanishini kuzatuvchi allocator
// Аллокатор отслеживающий использование памяти
struct TrackingAllocator {
    jami_ajratilgan: AtomicUsize,
    joriy_ajratilgan: AtomicUsize,
    ajratishlar_soni: AtomicUsize,
    bo_shatishlar_soni: AtomicUsize,
    maksimal: AtomicUsize,
}

impl TrackingAllocator {
    const fn new() -> Self {
        TrackingAllocator {
            jami_ajratilgan:   AtomicUsize::new(0),
            joriy_ajratilgan:  AtomicUsize::new(0),
            ajratishlar_soni:  AtomicUsize::new(0),
            bo_shatishlar_soni: AtomicUsize::new(0),
            maksimal:          AtomicUsize::new(0),
        }
    }

    fn hisobot(&self) {
        println!("=== Xotira Hisobot ===");
        println!("  Jami ajratilgan:    {} bayt", self.jami_ajratilgan.load(Ordering::Relaxed));
        println!("  Joriy ishlatish:    {} bayt", self.joriy_ajratilgan.load(Ordering::Relaxed));
        println!("  Maksimal:           {} bayt", self.maksimal.load(Ordering::Relaxed));
        println!("  Ajratishlar:        {} marta", self.ajratishlar_soni.load(Ordering::Relaxed));
        println!("  Bo'shatishlar:      {} marta", self.bo_shatishlar_soni.load(Ordering::Relaxed));
        let ajr = self.ajratishlar_soni.load(Ordering::Relaxed);
        let bos = self.bo_shatishlar_soni.load(Ordering::Relaxed);
        if ajr > bos {
            println!("  ⚠️ Ehtimol leak:    {} ta ajratish bo'shatilmagan", ajr - bos);
        } else {
            println!("  ✅ Leak yo'q");
        }
    }

    fn joriy(&self) -> usize { self.joriy_ajratilgan.load(Ordering::Relaxed) }
    fn jami(&self) -> usize { self.jami_ajratilgan.load(Ordering::Relaxed) }
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            let olcham = layout.size();
            self.jami_ajratilgan.fetch_add(olcham, Ordering::Relaxed);
            self.ajratishlar_soni.fetch_add(1, Ordering::Relaxed);
            let joriy = self.joriy_ajratilgan.fetch_add(olcham, Ordering::Relaxed) + olcham;
            let mut maks = self.maksimal.load(Ordering::Relaxed);
            while joriy > maks {
                match self.maksimal.compare_exchange_weak(maks, joriy, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_)  => break,
                    Err(v) => maks = v,
                }
            }
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        self.joriy_ajratilgan.fetch_sub(layout.size(), Ordering::Relaxed);
        self.bo_shatishlar_soni.fetch_add(1, Ordering::Relaxed);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, yangi_o_lcham: usize) -> *mut u8 {
        let yangi_ptr = unsafe { System.realloc(ptr, layout, yangi_o_lcham) };
        if !yangi_ptr.is_null() {
            let eski = layout.size();
            if yangi_o_lcham > eski {
                let qoshimcha = yangi_o_lcham - eski;
                self.jami_ajratilgan.fetch_add(qoshimcha, Ordering::Relaxed);
                self.joriy_ajratilgan.fetch_add(qoshimcha, Ordering::Relaxed);
            } else {
                let kamayish = eski - yangi_o_lcham;
                self.joriy_ajratilgan.fetch_sub(kamayish, Ordering::Relaxed);
            }
        }
        yangi_ptr
    }
}

#[global_allocator]
static KUZATUVCHI: TrackingAllocator = TrackingAllocator::new();

fn tracking_allocator_misoli() {

    println!("--- Tracking Allocator ---");
    let boshlangich = KUZATUVCHI.joriy();

    {
        let v: Vec<i32> = (0..1000).collect();
        let s = String::from("Salom Rust allocator!");
        let _m: std::collections::HashMap<i32, i32> = (0..100).map(|i| (i, i*i)).collect();
        println!("Vec, String, HashMap ajratildi");
        println!("  Qo'shimcha xotira: {} bayt", KUZATUVCHI.joriy() - boshlangich);
    }

    println!("  Scope tugagandan keyin: {} bayt", KUZATUVCHI.joriy() - boshlangich);
    KUZATUVCHI.hisobot();
}

// Bump allocator — eng tez allocator
// Bump allocator — самый быстрый аллокатор
// Bo'shatish individual emas — hammasi birda
// Нет индивидуального освобождения — всё сразу

struct BumpAllocator {
    bufer: UnsafeCell<Vec<u8>>,
    uch: AtomicUsize,
    sig_im: usize,
}

unsafe impl Sync for BumpAllocator {}
unsafe impl Send for BumpAllocator {}

impl BumpAllocator {
    fn new(sig_im: usize) -> Self {
        BumpAllocator {
            bufer: UnsafeCell::new(vec![0u8; sig_im]),
            uch: AtomicUsize::new(0),
            sig_im,
        }
    }

    fn ajrat(&self, o_lcham: usize, hizalanish: usize) -> Option<*mut u8> {
        let mut joriy = self.uch.load(Ordering::Relaxed);
        loop {
            // Hizalanish
            let hizalangan = (joriy + hizalanish - 1) & !(hizalanish - 1);
            let yangi_uch = hizalangan + o_lcham;

            if yangi_uch > self.sig_im {
                return None; // To'la
            }

            match self.uch.compare_exchange_weak(
                joriy, yangi_uch, Ordering::Relaxed, Ordering::Relaxed
            ) {
                Ok(_) => {
                    let bufer = unsafe { &mut *self.bufer.get() };
                    return Some(bufer.as_mut_ptr().wrapping_add(hizalangan));
                }
                Err(v) => joriy = v,
            }
        }
    }

    fn reset(&self) {
        self.uch.store(0, Ordering::Relaxed);
    }

    fn ishlatilgan(&self) -> usize {
        self.uch.load(Ordering::Relaxed)
    }

    fn sig_im(&self) -> usize { self.sig_im }

    fn ajrat_typed<T>(&self) -> Option<*mut T> {
        let ptr = self.ajrat(std::mem::size_of::<T>(), std::mem::align_of::<T>())?;
        Some(ptr as *mut T)
    }
}

fn bump_allocator_misoli() {

    println!("\n--- Bump Allocator ---");
    let alloc = BumpAllocator::new(1024 * 1024); // 1 MB

    let t = std::time::Instant::now();

    // 10000 ta ajratish — super tez
    for i in 0..10000 {
        let ptr: Option<*mut i32> = alloc.ajrat_typed::<i32>();
        if let Some(p) = ptr {
            unsafe { *p = i; }
        }
    }

    let vaqt = t.elapsed();
    println!("10000 ta ajratish: {:?}", vaqt);
    println!("Ishlatilgan: {} / {} bayt", alloc.ishlatilgan(), alloc.sig_im());

    // Reset — hammasi birda bo'shatiladi
    alloc.reset();
    println!("Reset dan keyin: {} bayt", alloc.ishlatilgan());

    // Turli turlar ajratish
    let i_ptr: *mut i32 = alloc.ajrat_typed().unwrap();
    let f_ptr: *mut f64 = alloc.ajrat_typed().unwrap();
    let arr_ptr: Option<*mut u8> = alloc.ajrat(100, 8);

    unsafe {
        *i_ptr = 42;
        *f_ptr = 3.14;
        if let Some(p) = arr_ptr {
            for j in 0..10 { *p.add(j) = j as u8; }
            println!("Array[0..5]: {:?}", std::slice::from_raw_parts(p, 5));
        }
        println!("i32: {}, f64: {}", *i_ptr, *f_ptr);
    }
    // i32: 42, f64: 3.14
    // Array[0..5]: [0, 1, 2, 3, 4]
}

// Pool allocator — bir xil o'lchamli ob'ektlar uchun
// Pool allocator — для объектов одинакового размера
struct PoolAllocator<const BLOK_O_LCHAM: usize> {
    bufer: UnsafeCell<Vec<u8>>,
    erkin: Mutex<Vec<*mut u8>>,
    sig_im: usize,
}

unsafe impl<const N: usize> Send for PoolAllocator<N> {}
unsafe impl<const N: usize> Sync for PoolAllocator<N> {}

impl<const BLOK_O_LCHAM: usize> PoolAllocator<BLOK_O_LCHAM> {
    fn new(blok_soni: usize) -> Self {
        let jami = BLOK_O_LCHAM * blok_soni;
        let mut bufer = vec![0u8; jami];
        let mut erkin = Vec::with_capacity(blok_soni);

        // Barcha bloklarni erkin ro'yxatga qo'shish
        for i in 0..blok_soni {
            let ptr = unsafe { bufer.as_mut_ptr().add(i * BLOK_O_LCHAM) };
            erkin.push(ptr);
        }

        PoolAllocator {
            bufer: UnsafeCell::new(bufer),
            erkin: Mutex::new(erkin),
            sig_im: blok_soni,
        }
    }

    fn ajrat(&self) -> Option<*mut u8> {
        self.erkin.lock().unwrap().pop()
    }

    fn qaytarish(&self, ptr: *mut u8) {
        self.erkin.lock().unwrap().push(ptr);
    }

    fn erkin_soni(&self) -> usize {
        self.erkin.lock().unwrap().len()
    }

    fn band_soni(&self) -> usize {
        self.sig_im - self.erkin_soni()
    }
}

#[derive(Debug)]
struct Obekt { id: u32, qiymat: f64, nomi: [u8; 16] }

fn pool_allocator_misoli() {

    println!("\n--- Pool Allocator ---");
    const OBJ_O_LCHAM: usize = std::mem::size_of::<Obekt>();
    let pool: PoolAllocator<OBJ_O_LCHAM> = PoolAllocator::new(100);

    println!("Blok o'lcham: {} bayt", OBJ_O_LCHAM);
    println!("Pool sig'im: {} ta", 100);
    println!("Erkin: {}", pool.erkin_soni());

    // Bir nechta ob'ekt ajratish
    let mut ob_ektlar: Vec<*mut Obekt> = Vec::new();
    for i in 0..5u32 {
        if let Some(ptr) = pool.ajrat() {
            let obj = ptr as *mut Obekt;
            unsafe {
                (*obj).id = i;
                (*obj).qiymat = i as f64 * 1.5;
                let nomi = format!("obj_{}", i);
                let bytes = nomi.as_bytes();
                let n = bytes.len().min(15);
                (&mut (*obj).nomi)[..n].copy_from_slice(&bytes[..n]);
                (*obj).nomi[n] = 0;
            }
            ob_ektlar.push(obj);
        }
    }

    println!("Band: {}", pool.band_soni());

    // O'qish
    for &ptr in &ob_ektlar {
        unsafe {
            let obj = &*ptr;
            let nomi = std::str::from_utf8(&obj.nomi)
                .unwrap_or("?").trim_end_matches('\0');
            println!("  id={}, qiymat={:.1}, nomi={}", obj.id, obj.qiymat, nomi);
        }
    }

    // Qaytarish
    for ptr in ob_ektlar {
        pool.qaytarish(ptr as *mut u8);
    }
    println!("Qaytarildi, erkin: {}", pool.erkin_soni());
    // Band: 5
    // id=0, qiymat=0.0, nomi=obj_0
    // ...
    // Qaytarildi, erkin: 100
}

// Arena — bir guruh ob'ekt, hammasi birda bo'shatiladi
// Arena — группа объектов, освобождаются все вместе
struct Arena {
    bump: BumpAllocator,
    ajratishlar: AtomicUsize,
}

impl Arena {
    fn new(sig_im: usize) -> Self {
        Arena {
            bump: BumpAllocator::new(sig_im),
            ajratishlar: AtomicUsize::new(0),
        }
    }

    fn ajrat<T: Default>(&self) -> Option<&mut T> {
        let ptr = self.bump.ajrat_typed::<T>()?;
        self.ajratishlar.fetch_add(1, Ordering::Relaxed);
        unsafe {
            std::ptr::write(ptr, T::default());
            Some(&mut *ptr)
        }
    }

    fn ajrat_qiymat<T>(&self, qiymat: T) -> Option<&mut T> {
        let ptr = self.bump.ajrat_typed::<T>()?;
        self.ajratishlar.fetch_add(1, Ordering::Relaxed);
        unsafe {
            std::ptr::write(ptr, qiymat);
            Some(&mut *ptr)
        }
    }

    fn ajrat_slice<T: Copy>(&self, uzunlik: usize, to_ldirish: T) -> Option<&mut [T]> {
        let ptr = self.bump.ajrat(
            uzunlik * std::mem::size_of::<T>(),
            std::mem::align_of::<T>(),
        )? as *mut T;
        unsafe {
            for i in 0..uzunlik { std::ptr::write(ptr.add(i), to_ldirish); }
            Some(std::slice::from_raw_parts_mut(ptr, uzunlik))
        }
    }

    fn hisobot(&self) {
        println!("Arena: {} ta ajratish, {} / {} bayt",
                 self.ajratishlar.load(Ordering::Relaxed),
                 self.bump.ishlatilgan(),
                 self.bump.sig_im(),
        );
    }

    fn tozalash(&self) {
        self.bump.reset();
        self.ajratishlar.store(0, Ordering::Relaxed);
    }
}

#[derive(Debug, Default, Clone)]
struct Tugun { qiymat: i32, uzunlik: usize }

fn arena_misoli() {

    println!("\n--- Arena Allocator ---");
    let arena = Arena::new(1024 * 64); // 64 KB

    // Turli ob'ektlar ajratish — barchasi arena da
    let n1: &mut i32 = arena.ajrat_qiymat(42).unwrap();
    let n2: &mut f64 = arena.ajrat_qiymat(3.14).unwrap();
    let t1: &mut Tugun = arena.ajrat_qiymat(Tugun { qiymat: 100, uzunlik: 5 }).unwrap();
    let slice: &mut [i32] = arena.ajrat_slice(10, 0).unwrap();

    // Qiymatlar
    *n1 += 8;
    for (i, x) in slice.iter_mut().enumerate() { *x = i as i32 * i as i32; }

    println!("n1: {}", n1);
    println!("n2: {}", n2);
    println!("t1: {:?}", t1);
    println!("slice: {:?}", slice);
    arena.hisobot();

    // Frame allocator — har frame tozalanadi
    println!("\nFrame allocator simulyatsiya:");
    for frame in 0..3 {
        println!("  Frame {}:", frame);
        arena.tozalash();

        for i in 0..5 {
            let t: &mut Tugun = arena.ajrat_qiymat(Tugun {
                qiymat: frame * 100 + i,
                uzunlik: i as usize,
            }).unwrap();
            print!("    {:?} ", t);
        }
        println!();
    }
    // Frame 0: Tugun{0} Tugun{1} ... Tugun{4}
    // Frame 1: Tugun{100} ... (reset + yangi)
}

// Xavfsizlik uchun — bo'shatilinayotgan xotirani nol qilish
// Для безопасности — обнуление освобождаемой памяти
struct ZeroingAllocator;

unsafe impl GlobalAlloc for ZeroingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { System.alloc_zeroed(layout) } // nol bilan to'ldirish
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // Avval nol qilish, keyin bo'shatish
        unsafe {
            std::ptr::write_bytes(ptr, 0, layout.size());
            System.dealloc(ptr, layout);
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, yangi_o_lcham: usize) -> *mut u8 {
        unsafe { System.realloc(ptr, layout, yangi_o_lcham) }
    }
}

// (ZeroingAllocator global ishlatilmaydi — TrackingAllocator o'rnatilgan)
// Bu faqat tushuntirish uchun

fn zeroing_tushuntirish() {

    println!("\n--- Zeroing Allocator ---");
    println!("Xavfsizlik uchun nol qilish:");
    println!("  alloc_zeroed() — ajratganda nol");
    println!("  write_bytes(0) — bo'shatganda nol");
    println!("  Parol, kalit kabi maxfiy ma'lumotlar uchun muhim");
    println!();

    // Hozir System allocator bilan nol qilish
    let layout = Layout::from_size_align(16, 8).unwrap();
    let ptr = unsafe { System.alloc_zeroed(layout) };
    if !ptr.is_null() {
        let slice = unsafe { std::slice::from_raw_parts(ptr, 16) };
        println!("alloc_zeroed natija: {:?}", slice);
        // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        unsafe { System.dealloc(ptr, layout); }
    }
}

fn performance_taqqoslash() {

    println!("\n--- Allocator Performance Taqqoslash ---");
    let n = 100_000;

    // System allocator (standart)
    let t1 = std::time::Instant::now();
    let mut ptrs: Vec<*mut i32> = Vec::with_capacity(n);
    let layout = Layout::new::<i32>();
    for _ in 0..n {
        let p = unsafe { alloc(layout) as *mut i32 };
        ptrs.push(p);
    }
    for p in &ptrs {
        unsafe { dealloc(*p as *mut u8, layout); }
    }
    let vaqt_system = t1.elapsed();

    // Bump allocator
    let bump = BumpAllocator::new(n * std::mem::size_of::<i32>() + 1024);
    let t2 = std::time::Instant::now();
    for _ in 0..n {
        bump.ajrat_typed::<i32>();
    }
    let vaqt_bump = t2.elapsed();
    bump.reset();

    println!("System alloc ({} ta): {:?}", n, vaqt_system);
    println!("Bump alloc   ({} ta): {:?}", n, vaqt_bump);
    if vaqt_bump.as_nanos() > 0 {
        println!("Tezlashuv: ~{:.0}x", vaqt_system.as_nanos() as f64 / vaqt_bump.as_nanos() as f64);
    }
    // Bump odatda 10-100x tezroq
}

fn main() {

    println!("=== TRACKING ALLOCATOR ===");
    tracking_allocator_misoli();

    println!("\n=== BUMP ALLOCATOR ===");
    bump_allocator_misoli();

    println!("\n=== POOL ALLOCATOR ===");
    pool_allocator_misoli();

    println!("\n=== ARENA ALLOCATOR ===");
    arena_misoli();

    zeroing_tushuntirish();

    println!("\n=== PERFORMANCE ===");
    performance_taqqoslash();

    println!("\n=== YAKUNIY HISOBOT ===");
    KUZATUVCHI.hisobot();

    println!("\n=== XULOSA ===");
    println!("Allocator turlari:");
    println!("  System     — standart, universal, sekin");
    println!("  Tracking   — debug, leak aniqlash");
    println!("  Bump       — eng tez, reset bilan bo'shatish");
    println!("  Pool       — bir o'lcham, tez qayta ishlatish");
    println!("  Arena      — guruh, frame allocator");
    println!("  Zeroing    — xavfsizlik, maxfiy ma'lumot");
    println!();
    println!("#[global_allocator] — dastur miqyosida o'zgartirish");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        GLOBAL ALLOCATOR                                                                                      |
// #================================================================================================================================================#
// # |   1 | #[global_allocator]             | Global allocator o'rnatish                 | Установка глобального аллокатора                        |
// # |   2 | unsafe impl GlobalAlloc         | GlobalAlloc implement qilish               | Реализация GlobalAlloc                                  |
// # |   3 | fn alloc(&self, layout) -> *u8  | Xotira ajratish                            | Выделение памяти                                        |
// # |   4 | fn dealloc(&self, ptr, layout)  | Xotira bo'shatish                          | Освобождение памяти                                     |
// # |   5 | Layout::new::<T>()              | T uchun Layout                             | Layout для T                                            |
// # |   6 | Layout::from_size_align(s, a)   | Maxsus Layout yaratish                     | Создание произвольного Layout                           |
// # |   7 | System.alloc_zeroed(layout)     | Nol bilan to'ldirilgan ajratish            | Выделение заполненное нулями                            |
// #================================================================================================================================================#
// # |                                        ALLOCATOR TURLARI                                                                                     |
// #================================================================================================================================================#
// # |   8 | Tracking Allocator              | Ajratish/bo'shatish kuzatish               | Отслеживание выделений/освобождений                     |
// # |   9 | Bump Allocator                  | Pointer oshirish, reset bilan bo'shatish   | Инкремент указателя, сброс для освобождения             |
// # |  10 | Pool Allocator                  | Bir o'lcham bloklari, qayta ishlatish      | Блоки одного размера, повторное использование           |
// # |  11 | Arena Allocator                 | Guruh ajratish, hammasi birda bo'shatish   | Групповое выделение, массовое освобождение              |
// # |  12 | Zeroing Allocator               | Bo'shatganda nol qilish                    | Обнуление при освобождении                              |
// #================================================================================================================================================#
// # |                                        QACHON NIMA                                                                                           |
// #================================================================================================================================================#
// # |  13 | System (default)                | Universal maqsad                           | Универсальное назначение                                |
// # |  14 | Bump                            | Tez bir martalik ajratish (parser, game)   | Быстрое одноразовое выделение                           |
// # |  15 | Pool                            | Ko'p bir xil ob'ekt (network, game)        | Много одинаковых объектов                               |
// # |  16 | Arena                           | Frame, request, transaction chegarasi      | Граница frame/запроса/транзакции                        |
// #================================================================================================================================================#