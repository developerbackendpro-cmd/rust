// #================================================================================================================================================#
// #                                                                  STD::MEM                                                                      #
// #                    STD::MEM — XOTIRA OPERATSIYALARI. SIZE_OF, ALIGN_OF, SWAP, REPLACE, TAKE, TRANSMUTE, FORGET, ZEROED.                        #
// #                    STD::MEM — ОПЕРАЦИИ С ПАМЯТЬЮ. SIZE_OF, ALIGN_OF, SWAP, REPLACE, TAKE, TRANSMUTE, FORGET, ZEROED.                           #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::mem;
use std::fmt;

// std::mem nima:
// Что такое std::mem:
//
//   size_of::<T>()      — T ning baytlardagi o'lchami
//   align_of::<T>()     — T ning hizalanishi
//   size_of_val(&val)   — qiymat o'lchami
//   swap(&mut a, &mut b)— ikkisini almashtirish
//   replace(&mut a, b)  — yangi qiymat qo'yib eski qaytarish
//   take(&mut a)        — Default qo'yib qiymat olish
//   transmute(val)      — bir turni boshqaga aylantirish (UNSAFE!)
//   forget(val)         — Drop chaqirmasdan yo'qotish
//   zeroed::<T>()       — nol baytlar bilan T (UNSAFE!)
//   uninitialized::<T>()— initsializatsiyasiz T (UNSAFE!)
//   needs_drop::<T>()   — T Drop implement qilganmi?
//   discriminant(&val)  — enum variant diskriminanti
//   offset_of!(T, f)    — struct field offseti

fn size_align_misollari() {

    // size_of::<T>() — kompilyatsiya vaqtida aniqlanadi
    // size_of::<T>() — определяется во время компиляции
    println!("=== O'lchamlar ===");
    println!("bool:    {} bayt", mem::size_of::<bool>());
    println!("u8:      {} bayt", mem::size_of::<u8>());
    println!("u16:     {} bayt", mem::size_of::<u16>());
    println!("u32:     {} bayt", mem::size_of::<u32>());
    println!("u64:     {} bayt", mem::size_of::<u64>());
    println!("u128:    {} bayt", mem::size_of::<u128>());
    println!("usize:   {} bayt", mem::size_of::<usize>());
    println!("f32:     {} bayt", mem::size_of::<f32>());
    println!("f64:     {} bayt", mem::size_of::<f64>());
    println!("char:    {} bayt", mem::size_of::<char>());
    println!("&str:    {} bayt", mem::size_of::<&str>());      // fat pointer: ptr+len
    println!("String:  {} bayt", mem::size_of::<String>());   // ptr+len+cap
    println!("Vec<i32>:{} bayt", mem::size_of::<Vec<i32>>()); // ptr+len+cap
    // bool:    1 bayt
    // u8:      1 bayt
    // ...
    // &str:    16 bayt (fat pointer)
    // String:  24 bayt
    // Vec<i32>:24 bayt

    // Struct o'lchamlari — padding bilan
    // Размеры структур — с выравниванием
    println!("\n=== Struct o'lchamlari ===");

    #[repr(C)]
    struct S1 { a: u8, b: u32, c: u8 }  // padding: 12 bayt
    struct S2 { a: u32, b: u8, c: u8 }  // optimized: 8 bayt
    struct S3 { a: u8, b: u8, c: u32 }  // 8 bayt

    #[repr(packed)]
    struct SPacked { a: u8, b: u32, c: u8 } // 6 bayt — padding yo'q

    println!("S1 (u8,u32,u8):   {} bayt", mem::size_of::<S1>());  // 12
    println!("S2 (u32,u8,u8):   {} bayt", mem::size_of::<S2>());  // 8
    println!("S3 (u8,u8,u32):   {} bayt", mem::size_of::<S3>());  // 8
    println!("SPacked:          {} bayt", mem::size_of::<SPacked>()); // 6

    // Enum o'lchamlari — diskriminant + kattasi
    // Размеры enum — дискриминант + наибольший вариант
    enum E1 { A, B, C }                    // 1 bayt
    enum E2 { A(u8), B(u32), C }           // 8 bayt (diskriminant + u32)
    enum E3 { A(Box<i32>), B }             // 8 bayt (NonNull pointer)
    enum E4 { A(u8), B(u8), C(u8) }        // 2 bayt (diskriminant+u8)

    println!("E1 (A,B,C):           {} bayt", mem::size_of::<E1>());
    println!("E2 (A(u8),B(u32),C):  {} bayt", mem::size_of::<E2>());
    println!("E3 (A(Box),B):        {} bayt", mem::size_of::<E3>());
    println!("Option<Box<i32>>:     {} bayt", mem::size_of::<Option<Box<i32>>>()); // 8 (null opt)

    // align_of::<T>() — hizalanish talabi
    // align_of::<T>() — требование выравнивания
    println!("\n=== Hizalanish ===");
    println!("u8  align: {}", mem::align_of::<u8>());
    println!("u16 align: {}", mem::align_of::<u16>());
    println!("u32 align: {}", mem::align_of::<u32>());
    println!("u64 align: {}", mem::align_of::<u64>());
    println!("f64 align: {}", mem::align_of::<f64>());

    // size_of_val — runtime qiymat o'lchami
    // size_of_val — размер значения в runtime
    println!("\n=== size_of_val ===");
    let s: &str = "salom dunyo";
    let v: Vec<i32> = vec![1, 2, 3, 4, 5];
    let arr: [u8; 10] = [0; 10];

    println!("str '{}': {} bayt", s, mem::size_of_val(s)); // 11 (baytlar soni)
    println!("Vec<i32>[5]: {} bayt", mem::size_of_val(&v)); // 24 (Vec struct o'lchami)
    println!("[u8; 10]: {} bayt", mem::size_of_val(&arr));  // 10
    // str 'salom dunyo': 11 bayt
    // Vec<i32>[5]: 24 bayt
    // [u8; 10]: 10 bayt
}

fn swap_replace_take_misollari() {

    // mem::swap — ikkisini almashtirish
    // mem::swap — обмен двух значений
    let mut a = String::from("salom");
    let mut b = String::from("dunyo");
    println!("Avval: a='{}' b='{}'", a, b);
    mem::swap(&mut a, &mut b);
    println!("Keyin: a='{}' b='{}'", a, b);
    // Avval: a='salom' b='dunyo'
    // Keyin: a='dunyo' b='salom'

    // swap — primitiv turlar
    let mut x = 10i32;
    let mut y = 20i32;
    mem::swap(&mut x, &mut y);
    println!("{} {}", x, y); // 20 10
    // 20 10

    // mem::replace — yangi qiymat qo'yib eski qaytarish
    // mem::replace — поставить новое значение и вернуть старое
    let mut holat = String::from("boshlangich");
    let eski = mem::replace(&mut holat, String::from("yangi"));
    println!("Eski: '{}', Yangi: '{}'", eski, holat);
    // Eski: 'boshlangich', Yangi: 'yangi'

    // replace — enum holat o'zgartirish
    // replace — изменение состояния enum
    #[derive(Debug)]
    enum Holat { Faol(String), Nofaol }

    let mut h = Holat::Faol("ish".to_string());
    let prev = mem::replace(&mut h, Holat::Nofaol);
    println!("Avvalgi: {:?}, Hozirgi: {:?}", prev, h);
    // Avvalgi: Faol("ish"), Hozirgi: Nofaol

    // replace — Option almashtirish
    let mut opt: Option<Vec<i32>> = Some(vec![1, 2, 3]);
    let olindi = mem::replace(&mut opt, None); // = opt.take() bilan bir xil
    println!("{:?} {:?}", olindi, opt);
    // Some([1, 2, 3]) None

    // mem::take — Default qo'yib qiymat olish
    // mem::take — взять значение и заменить Default
    let mut s = String::from("salom");
    let olindi = mem::take(&mut s);
    println!("Olindi: '{}', s: '{}'", olindi, s); // s = "" (Default)
    // Olindi: 'salom', s: ''

    // take — Vec
    let mut v = vec![1, 2, 3, 4, 5];
    let olindi_v = mem::take(&mut v);
    println!("{:?} {:?}", olindi_v, v); // [1,2,3,4,5] []
    // [1, 2, 3, 4, 5] []

    // take — Option
    let mut opt2: Option<i32> = Some(42);
    let val = mem::take(&mut opt2);
    println!("{:?} {:?}", val, opt2); // Some(42) None
    // Some(42) None
}

fn forget_misollari() {

    // mem::forget — Drop chaqirilmaydi
    // mem::forget — Drop не вызывается
    struct DropLog { nomi: String }
    impl Drop for DropLog {
        fn drop(&mut self) { println!("'{}' tushirildi", self.nomi); }
    }

    let d1 = DropLog { nomi: "oddiy".to_string() };
    let d2 = DropLog { nomi: "forgotten".to_string() };

    println!("forget qilinmoqda...");
    mem::forget(d2); // Drop chaqirilmaydi!
    println!("forget tugadi");
    // d1 scope tugaganda tushiriladi, d2 emas
    drop(d1); // qo'lda drop
    // forget qilinmoqda...
    // forget tugadi
    // 'oddiy' tushirildi  ← d1 drop
    // 'd2' TUSHIRILMAYDI  ← forget qilindi!

    // forget — ManuallyDrop bilan bir xil effekt
    // forget — тот же эффект что и ManuallyDrop
    use std::mem::ManuallyDrop;
    let md = ManuallyDrop::new(DropLog { nomi: "manually".to_string() });
    // Drop chaqirilmaydi — ManuallyDrop bilan bir xil
    println!("ManuallyDrop ham Drop chaqirmaydi");

    // forget — FFI da ishlatish (C ga ownership berish)
    // forget — использование в FFI (передача владения в C)
    fn c_ga_berish<T>(val: T) -> *mut T {
        let ptr = Box::into_raw(Box::new(val));
        ptr // C xotirani o'zi boshqaradi
    }

    let ptr = c_ga_berish(42i32);
    unsafe {
        println!("C qiymati: {}", *ptr);
        drop(Box::from_raw(ptr)); // qaytarib olish
    }
    // C qiymati: 42
}

fn transmute_misollari() {

    // mem::transmute — bir turni boshqaga aylantirish
    // mem::transmute — преобразование одного типа в другой
    // XAVFLI: bir xil o'lchamdagi turlar bo'lishi shart!
    // ОПАСНО: типы должны иметь одинаковый размер!

    // u32 → f32 (bir xil: 4 bayt)
    // u32 → f32 (одинаковый: 4 байта)
    let bits: u32 = 0x3F800000;
    let f: f32 = f32::from_bits(bits);
    println!("{}", f); // 1.0
    // 1.0

    // f32 → u32 — bit pattern ko'rish
    // f32 → u32 — просмотр битового представления
    let pi: f32 = std::f32::consts::PI;
    let pi_bits: u32 = pi.to_bits();
    println!("π bits: {:#010X}", pi_bits); // 0x40490FDB
    // π bits: 0x40490FDB

    // &str → &[u8] — byte ko'rish (xavfsiz yo'li: .as_bytes())
    // &str → &[u8] — просмотр байтов (безопасный способ: .as_bytes())
    let s = "salom";
    let bytes: &[u8] = unsafe { mem::transmute::<&str, &[u8]>(s) };
    println!("{:?}", bytes); // [115, 97, 108, 111, 109]
    // [115, 97, 108, 111, 109]

    // Vec<i32> → Vec<u32> — bir xil o'lcham
    // Vec<i32> → Vec<u32> — одинаковый размер
    let signed: Vec<i32> = vec![-1, 0, 1, 127];
    let unsigned: Vec<u32> = unsafe { mem::transmute::<Vec<i32>, Vec<u32>>(signed) };
    println!("{:?}", unsigned); // [4294967295, 0, 1, 127]
    // [4294967295, 0, 1, 127]

    // Xavfsizroq alternativ: from_bits
    // Более безопасная альтернатива: from_bits
    let bits2: u32 = 1065353216; // 1.0f32
    let f2 = f32::from_bits(bits2);
    println!("{}", f2); // 1.0
    // 1.0

    // transmute_copy — o'lcham tekshirmasdan
    // transmute_copy — без проверки размера
    let big: u64 = 0x0102030405060708;
    let small: u32 = unsafe { mem::transmute_copy::<u64, u32>(&big) };
    println!("transmute_copy: {:#010X}", small);
    // transmute_copy: 0x05060708 (little-endian) yoki 0x01020304

    // Xavfsiz transmute pattern — assurance bilan
    // Безопасный паттерн transmute — с проверкой
    fn xavfsiz_transmute<F, T>(val: F) -> T {
        assert_eq!(mem::size_of::<F>(), mem::size_of::<T>(),
                   "Tur o'lchamlari mos emas!");
        unsafe { mem::transmute_copy::<F, T>(&val) }
    }

    let x: i32 = -1;
    let y: u32 = xavfsiz_transmute::<i32, u32>(x);
    println!("i32(-1) → u32: {}", y); // 4294967295
    // i32(-1) → u32: 4294967295
}

fn yordamchi_funksiyalar() {

    // needs_drop::<T>() — T Drop implement qilganmi?
    // needs_drop::<T>() — реализует ли T Drop?
    println!("=== needs_drop ===");
    println!("i32:    {}", mem::needs_drop::<i32>());       // false
    println!("String: {}", mem::needs_drop::<String>());    // true
    println!("Vec:    {}", mem::needs_drop::<Vec<i32>>()); // true
    println!("&str:   {}", mem::needs_drop::<&str>());      // false
    println!("Box<i32>: {}", mem::needs_drop::<Box<i32>>()); // true
    // i32:    false
    // String: true
    // Vec:    true
    // &str:   false
    // Box<i32>: true

    // discriminant — enum variant identifikatori
    // discriminant — идентификатор варианта enum
    println!("\n=== discriminant ===");
    #[derive(Debug)]
    enum Rang { Qizil, Yashil(u8), Kok { r: u8, g: u8, b: u8 } }

    let r = Rang::Qizil;
    let y = Rang::Yashil(128);
    let k = Rang::Kok { r: 0, g: 0, b: 255 };

    let d_r = mem::discriminant(&r);
    let d_y = mem::discriminant(&y);
    let d_k = mem::discriminant(&k);
    let d_r2 = mem::discriminant(&Rang::Qizil);

    println!("Qizil == Qizil: {}", d_r == d_r2); // true
    println!("Qizil == Yashil: {}", d_r == d_y);  // false
    println!("Yashil == Ko'k: {}", d_y == d_k);   // false
    // Qizil == Qizil: true
    // Qizil == Yashil: false
    // Yashil == Ko'k: false

    // Bir xil variant tekshirish — ma'lumotga qaramay
    // Проверка одного варианта — независимо от данных
    fn bir_xil_variant<T>(a: &T, b: &T) -> bool
    where T: std::fmt::Debug
    {
        mem::discriminant(a) == mem::discriminant(b)
    }

    println!("{}", bir_xil_variant(&Rang::Yashil(0), &Rang::Yashil(255))); // true — variant bir xil
    println!("{}", bir_xil_variant(&Rang::Qizil, &Rang::Yashil(0)));       // false
    // true
    // false

    // offset_of! — struct field ning offseti (Rust 1.77+)
    // offset_of! — смещение поля структуры (Rust 1.77+)
    println!("\n=== offset_of! ===");
    #[repr(C)]
    struct Nuqta { x: f32, y: f32, z: f32 }

    println!("x offset: {}", mem::offset_of!(Nuqta, x)); // 0
    println!("y offset: {}", mem::offset_of!(Nuqta, y)); // 4
    println!("z offset: {}", mem::offset_of!(Nuqta, z)); // 8
    // x offset: 0
    // y offset: 4
    // z offset: 8

    #[repr(C)]
    struct Paket { version: u8, flags: u8, id: u16, payload: u32 }
    println!("version offset: {}", mem::offset_of!(Paket, version)); // 0
    println!("flags offset:   {}", mem::offset_of!(Paket, flags));   // 1
    println!("id offset:      {}", mem::offset_of!(Paket, id));      // 2
    println!("payload offset: {}", mem::offset_of!(Paket, payload)); // 4
    // version offset: 0
    // flags offset:   1
    // id offset:      2
    // payload offset: 4
}

fn zeroed_misollari() {

    // mem::zeroed() — barcha baytlar nol (UNSAFE!)
    // mem::zeroed() — все байты нули (UNSAFE!)
    // Faqat POD (Plain Old Data) turlar uchun xavfsiz
    // Безопасно только для POD (Plain Old Data) типов

    let z_i32: i32 = unsafe { mem::zeroed() };
    let z_f64: f64 = unsafe { mem::zeroed() };
    let z_bool: bool = unsafe { mem::zeroed() };
    println!("zeroed i32:  {}", z_i32);   // 0
    println!("zeroed f64:  {}", z_f64);   // 0
    println!("zeroed bool: {}", z_bool);  // false
    // zeroed i32:  0
    // zeroed f64:  0
    // zeroed bool: false

    // zeroed — C dan xotira olish (FFI da keng ishlatiladi)
    // zeroed — получение памяти из C (широко используется в FFI)
    #[repr(C)]
    #[derive(Debug)]
    struct CStruct { a: i32, b: f64, c: u8 }

    let mut cs: CStruct = unsafe { mem::zeroed() };
    cs.a = 42;
    cs.b = 3.14;
    cs.c = 255;
    println!("{:?}", cs); // CStruct { a: 42, b: 3.14, c: 255 }
    // CStruct { a: 42, b: 3.14, c: 255 }

    // XAVFLI zeroed:
    // ОПАСНЫЙ zeroed:
    // let z_str: String = unsafe { mem::zeroed() }; // ← UNDEFINED BEHAVIOR!
    // String ichida null pointer bo'ladi!
    // Внутри String будет null pointer!

    // MaybeUninit — xavfsiz alternativ
    // MaybeUninit — безопасная альтернатива
    use std::mem::MaybeUninit;
    let mut mu: MaybeUninit<i32> = MaybeUninit::uninit();
    mu.write(100);
    let val = unsafe { mu.assume_init() };
    println!("MaybeUninit: {}", val); // 100
    // MaybeUninit: 100
}

// Samarali swap — mem::swap orqali
// Эффективный обмен — через mem::swap
fn parallel_qayta_ishlash() {

    // Double buffer pattern — swap bilan
    // Паттерн двойного буфера — через swap
    struct DoubleBuffer<T> {
        faol: Vec<T>,
        yashirin: Vec<T>,
    }

    impl<T: Clone + fmt::Debug> DoubleBuffer<T> {
        fn new() -> Self {
            DoubleBuffer { faol: Vec::new(), yashirin: Vec::new() }
        }

        fn yashiringa_yoz(&mut self, data: Vec<T>) {
            self.yashirin = data;
        }

        fn almashtir(&mut self) {
            mem::swap(&mut self.faol, &mut self.yashirin);
        }

        fn faolni_o_qi(&self) -> &[T] {
            &self.faol
        }
    }

    let mut buf: DoubleBuffer<i32> = DoubleBuffer::new();

    buf.yashiringa_yoz(vec![1, 2, 3]);
    buf.almashtir();
    println!("Faol: {:?}", buf.faolni_o_qi()); // [1, 2, 3]

    buf.yashiringa_yoz(vec![4, 5, 6, 7]);
    buf.almashtir();
    println!("Faol: {:?}", buf.faolni_o_qi()); // [4, 5, 6, 7]
    // Faol: [1, 2, 3]
    // Faol: [4, 5, 6, 7]
}

fn enum_holat_mashinasi() {

    // replace bilan enum holat mashinasi
    // Машина состояний enum через replace
    #[derive(Debug)]
    enum Vazifa {
        Kutmoqda,
        Bajarilmoqda { id: u32, progress: u8 },
        Tugadi(String),
        Xato(String),
    }

    impl Vazifa {
        fn boshlash(&mut self, id: u32) {
            let eski = mem::replace(self, Vazifa::Bajarilmoqda { id, progress: 0 });
            if let Vazifa::Kutmoqda = eski {
                println!("Vazifa {} boshlandi", id);
            }
        }

        fn progress_yangilash(&mut self, yangi: u8) {
            if let Vazifa::Bajarilmoqda { progress, .. } = self {
                *progress = yangi;
            }
        }

        fn tugatish(&mut self) {
            let eski = mem::replace(self, Vazifa::Tugadi("Muvaffaqiyat!".into()));
            if let Vazifa::Bajarilmoqda { id, .. } = eski {
                println!("Vazifa {} tugadi", id);
            }
        }
    }

    let mut v = Vazifa::Kutmoqda;
    v.boshlash(42);
    v.progress_yangilash(50);
    v.progress_yangilash(100);
    v.tugatish();
    println!("{:?}", v);
    // Vazifa 42 boshlandi
    // Vazifa 42 tugadi
    // Tugadi("Muvaffaqiyat!")
}

fn size_optimallashtirish() {

    // O'lcham tekshiruvi — const assert
    // Проверка размера — const assert
    const _: () = assert!(mem::size_of::<i32>() == 4);
    const _: () = assert!(mem::size_of::<u8>() == 1);

    // Struct packing strategiyasi
    // Стратегия упаковки структур
    #[repr(C)]
    struct NOptimal { a: u8, b: u64, c: u8 } // 24 bayt (padding bilan)

    #[repr(C)]
    struct Optimal { b: u64, a: u8, c: u8 }   // 16 bayt (katta avval)

    println!("NOptimal: {} bayt", mem::size_of::<NOptimal>());
    println!("Optimal:  {} bayt", mem::size_of::<Optimal>());
    // NOptimal: 24 bayt
    // Optimal:  16 bayt

    // Nul-pointer optimallashtirish
    // Оптимизация нулевого указателя
    println!("Option<Box<i32>>: {} bayt", mem::size_of::<Option<Box<i32>>>());
    println!("Box<i32>:         {} bayt", mem::size_of::<Box<i32>>());
    // Bir xil! — NPO (Null Pointer Optimization)
    // Одинаковый! — NPO

    // discriminant ni hisoblash uchun transmute (needs_drop bilan)
    println!("\nNeeds drop tekshiruvi:");
    println!("i32: {}", mem::needs_drop::<i32>());
    println!("String: {}", mem::needs_drop::<String>());

    // Samarali mass initialization
    // Эффективная массовая инициализация
    let mut arr: [i32; 5] = unsafe { mem::zeroed() };
    for (i, x) in arr.iter_mut().enumerate() {
        *x = i as i32 * i as i32;
    }
    println!("{:?}", arr); // [0, 1, 4, 9, 16]
    // [0, 1, 4, 9, 16]
}

fn main() {

    println!("=== SIZE_OF VA ALIGN_OF ===");
    size_align_misollari();

    println!("\n=== SWAP, REPLACE, TAKE ===");
    swap_replace_take_misollari();

    println!("\n=== FORGET ===");
    forget_misollari();

    println!("\n=== TRANSMUTE ===");
    transmute_misollari();

    println!("\n=== YORDAMCHI FUNKSIYALAR ===");
    yordamchi_funksiyalar();

    println!("\n=== ZEROED ===");
    zeroed_misollari();

    println!("\n=== REAL HAYOT ===");
    parallel_qayta_ishlash();
    enum_holat_mashinasi();
    size_optimallashtirish();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        O'LCHAM VA HIZALANISH                                                                                  |
// #================================================================================================================================================#
// # |   1 | mem::size_of::<T>()             | T ning baytlardagi o'lchami                | Размер T в байтах                                       |
// # |   2 | mem::align_of::<T>()            | T ning hizalanish talabi                   | Требование выравнивания T                               |
// # |   3 | mem::size_of_val(&val)          | Runtime qiymat o'lchami                    | Размер значения в runtime                               |
// # |   4 | mem::offset_of!(T, field)       | Struct field offseti (1.77+)               | Смещение поля структуры (1.77+)                         |
// #================================================================================================================================================#
// # |                                        QIYMAT OPERATSIYALARI                                                                                 |
// #================================================================================================================================================#
// # |   5 | mem::swap(&mut a, &mut b)       | Ikkisini almashtirish                      | Обмен двух значений                                     |
// # |   6 | mem::replace(&mut a, b)         | Yangi qo'yib eski qaytarish                | Поставить новое и вернуть старое                        |
// # |   7 | mem::take(&mut a)               | Default qo'yib qiymat olish                | Взять значение и поставить Default                      |
// # |   8 | mem::forget(val)                | Drop chaqirmasdan yo'qotish                | Уничтожить без вызова Drop                              |
// #================================================================================================================================================#
// # |                                        XAVFLI OPERATSIYALAR (unsafe)                                                                         |
// #================================================================================================================================================#
// # |   9 | mem::transmute::<F, T>(val)     | F → T tur o'zgartirish (bir xil o'lcham)   | Преобразование F → T (одинаковый размер)                |
// # |  10 | mem::transmute_copy::<F,T>(&val)| O'lcham tekshirmasdan nusxa                | Копия без проверки размера                              |
// # |  11 | mem::zeroed::<T>()              | Barcha baytlar nol                         | Все байты нули                                          |
// # |  12 | MaybeUninit<T>                  | zeroed dan xavfsiz alternativ              | Безопасная альтернатива zeroed                          |
// #================================================================================================================================================#
// # |                                        DIAGNOSTIKA                                                                                           |
// #================================================================================================================================================#
// # |  13 | mem::needs_drop::<T>()          | T Drop implement qilganmi?                 | Реализует ли T Drop?                                    |
// # |  14 | mem::discriminant(&val)         | Enum variant ID si                         | ID варианта enum                                        |
// # |  15 | NPO                             | Option<Box<T>> == Box<T> o'lcham           | Option<Box<T>> == Box<T> размер                         |
// #================================================================================================================================================#