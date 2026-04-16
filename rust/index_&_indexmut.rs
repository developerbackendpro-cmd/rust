// #================================================================================================================================================#
// #                                                            INDEX  |  INDEXMUT                                                                  #
// #                            INDEX — [] OPERATORI ORQALI O'QISH. INDEXMUT — [] OPERATORI ORQALI O'ZGARTIRISH.                                    #
// #                     I      NDEX — ЧТЕНИЕ ЧЕРЕЗ ОПЕРАТОР []. INDEXMUT — ИЗМЕНЕНИЕ ЧЕРЕЗ ОПЕРАТОР [].                                            #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::ops::{Index, IndexMut};
use std::fmt;

// Index    — v[i] o'qish → &T qaytaradi
// Index    — чтение v[i] → возвращает &T
// IndexMut — v[i] o'zgartirish → &mut T qaytaradi
// IndexMut — изменение v[i] → возвращает &mut T
//
// std da:
//   Vec<T>:     Index<usize, Output=T>
//   [T]:        Index<usize, Output=T>
//   HashMap<K,V>: Index<K, Output=V>
//   String:     Index<Range<usize>, Output=str>

fn built_in_index_misollari() {

    // Vec — Index<usize>
    // Vec — Index<usize>
    let v: Vec<i32> = vec![10, 20, 30, 40, 50];
    let birinchi: &i32 = &v[0];
    let uchinchi: &i32 = &v[2];
    let oxirgi: &i32 = &v[4];
    println!("{}", birinchi);
    println!("{}", uchinchi);
    println!("{}", oxirgi);
    // 10
    // 30
    // 50

    // array — Index<usize>
    // массив — Index<usize>
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let ikkinchi: &i32 = &arr[1];
    println!("{}", ikkinchi);
    // 2

    // String — Index<Range>
    // String — Index<Range>
    let s: String = String::from("salom dunyo");
    let qism: &str = &s[0..5];
    let oxirgi_qism: &str = &s[6..];
    println!("{}", qism);
    println!("{}", oxirgi_qism);
    // salom
    // dunyo

    // HashMap — Index<K>
    // HashMap — Index<K>
    use std::collections::HashMap;
    let mut xarita: HashMap<&str, i32> = HashMap::new();
    xarita.insert("bir", 1);
    xarita.insert("ikki", 2);
    xarita.insert("uch", 3);
    let qiymat: &i32 = &xarita["bir"];
    println!("{}", qiymat);
    // 1

    // Vec — Range bilan
    // Vec — с Range
    let v2: Vec<i32> = vec![10, 20, 30, 40, 50];
    let qism_vec: &[i32] = &v2[1..4];
    println!("{:?}", qism_vec);
    // [20, 30, 40]
}

fn built_in_indexmut_misollari() {

    // Vec — IndexMut<usize>
    // Vec — IndexMut<usize>
    let mut v: Vec<i32> = vec![10, 20, 30, 40, 50];
    v[0] = 99;
    v[4] = 100;
    println!("{:?}", v);
    // [99, 20, 30, 40, 100]

    // array — IndexMut<usize>
    // массив — IndexMut<usize>
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[2] = 33;
    println!("{:?}", arr);
    // [1, 2, 33, 4, 5]

    // Vec — Range bilan IndexMut
    // Vec — IndexMut с Range
    let mut v2: Vec<i32> = vec![1, 2, 3, 4, 5];
    let qism: &mut [i32] = &mut v2[1..4];
    qism[0] = 20;
    qism[1] = 30;
    println!("{:?}", v2);
    // [1, 20, 30, 4, 5]

    // += operatori bilan IndexMut
    // IndexMut с оператором +=
    let mut v3: Vec<i32> = vec![10, 20, 30];
    v3[0] += 5;
    v3[1] *= 2;
    v3[2] -= 10;
    println!("{:?}", v3);
    // [15, 40, 20]
}

// Matritsa — 2D array uchun Index
// Матрица — Index для 2D массива
#[derive(Debug, Clone)]
struct Matritsa {
    qatorlar: usize,
    ustunlar: usize,
    ichki: Vec<f64>,
}

impl Matritsa {
    fn new(qatorlar: usize, ustunlar: usize) -> Self {
        Matritsa {
            qatorlar,
            ustunlar,
            ichki: vec![0.0; qatorlar * ustunlar],
        }
    }

    fn to_ldirish(mut self, qiymatlar: Vec<f64>) -> Self {
        self.ichki = qiymatlar;
        self
    }
}

impl fmt::Display for Matritsa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for q in 0..self.qatorlar {
            write!(f, "[")?;
            for u in 0..self.ustunlar {
                if u > 0 { write!(f, ", ")?; }
                write!(f, "{:.1}", self.ichki[q * self.ustunlar + u])?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

// (qator, ustun) tuple bilan Index
// Index через кортеж (строка, столбец)
impl Index<(usize, usize)> for Matritsa {
    type Output = f64;

    fn index(&self, (q, u): (usize, usize)) -> &f64 {
        assert!(q < self.qatorlar, "Qator indeksi chegaradan oshdi");
        assert!(u < self.ustunlar, "Ustun indeksi chegaradan oshdi");
        &self.ichki[q * self.ustunlar + u]
    }
}

impl IndexMut<(usize, usize)> for Matritsa {
    fn index_mut(&mut self, (q, u): (usize, usize)) -> &mut f64 {
        assert!(q < self.qatorlar, "Qator indeksi chegaradan oshdi");
        assert!(u < self.ustunlar, "Ustun indeksi chegaradan oshdi");
        &mut self.ichki[q * self.ustunlar + u]
    }
}

// Rang enum bilan Index
// Index с enum цвета
#[derive(Debug)]
enum Rang { Qizil, Yashil, Moviy }

#[derive(Debug)]
struct RgbQiymati {
    qizil: u8,
    yashil: u8,
    moviy: u8,
}

impl Index<Rang> for RgbQiymati {
    type Output = u8;

    fn index(&self, rang: Rang) -> &u8 {
        match rang {
            Rang::Qizil  => &self.qizil,
            Rang::Yashil => &self.yashil,
            Rang::Moviy  => &self.moviy,
        }
    }
}

impl IndexMut<Rang> for RgbQiymati {
    fn index_mut(&mut self, rang: Rang) -> &mut u8 {
        match rang {
            Rang::Qizil  => &mut self.qizil,
            Rang::Yashil => &mut self.yashil,
            Rang::Moviy  => &mut self.moviy,
        }
    }
}

// Konfiguratsiya — &str bilan Index
// Конфигурация — Index через &str
#[derive(Debug)]
struct Konfiguratsiya {
    port: u16,
    host: String,
    timeout: u64,
    debug: bool,
}

impl Konfiguratsiya {
    fn new() -> Self {
        Konfiguratsiya {
            port: 8080,
            host: String::from("localhost"),
            timeout: 30,
            debug: false,
        }
    }
}

impl Index<&str> for Konfiguratsiya {
    type Output = String;

    fn index(&self, kalit: &str) -> &String {
        match kalit {
            "host" => &self.host,
            _      => panic!("Noma'lum kalit: {}", kalit),
        }
    }
}

// Xavfsiz Vec — chegaradan oshmaydigan
// Безопасный Vec — без выхода за границы
#[derive(Debug)]
struct XavfsizVec<T> {
    ichki: Vec<T>,
}

impl<T: Default + Clone> XavfsizVec<T> {
    fn new() -> Self {
        XavfsizVec { ichki: Vec::new() }
    }

    fn qosh(&mut self, qiymat: T) {
        self.ichki.push(qiymat);
    }

    // Xavfsiz o'qish — Option qaytaradi
    // Безопасное чтение — возвращает Option
    fn ol(&self, i: usize) -> Option<&T> {
        self.ichki.get(i)
    }
}

// Index — panic bilan (standart xulq)
// Index — с паникой (стандартное поведение)
impl<T> Index<usize> for XavfsizVec<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.ichki[i]
    }
}

impl<T> IndexMut<usize> for XavfsizVec<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        &mut self.ichki[i]
    }
}

// BitArray — bitlar uchun Index
// BitArray — Index для битов
#[derive(Debug)]
struct BitArray {
    ichki: Vec<u8>,
    uzunlik: usize,
}

impl BitArray {
    fn new(uzunlik: usize) -> Self {
        let bayt_soni: usize = (uzunlik + 7) / 8;
        BitArray {
            ichki: vec![0u8; bayt_soni],
            uzunlik,
        }
    }
}

// bool qaytaradi — bit qiymati
// возвращает bool — значение бита
impl Index<usize> for BitArray {
    type Output = bool;

    fn index(&self, i: usize) -> &bool {
        assert!(i < self.uzunlik, "Indeks chegaradan oshdi");
        let bayt: u8 = self.ichki[i / 8];
        let bit: u8 = (bayt >> (i % 8)) & 1;
        if bit == 1 { &true } else { &false }
    }
}

// RingBuffer — modulo indekslash
// RingBuffer — индексация по модулю
#[derive(Debug)]
struct RingBuffer<T> {
    ichki: Vec<T>,
    hajm: usize,
}

impl<T: Default + Clone> RingBuffer<T> {
    fn new(hajm: usize) -> Self {
        RingBuffer {
            ichki: vec![T::default(); hajm],
            hajm,
        }
    }
}

// Har qanday indeks ishlaydi — modulo bilan
// Любой индекс работает — через модуль
impl<T> Index<usize> for RingBuffer<T> {
    type Output = T;

    fn index(&self, i: usize) -> &T {
        &self.ichki[i % self.hajm]
    }
}

impl<T> IndexMut<usize> for RingBuffer<T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        let hajm: usize = self.hajm;
        &mut self.ichki[i % hajm]
    }
}

fn main() {

    built_in_index_misollari();

    built_in_indexmut_misollari();

    // 3x3 matritsa yaratish
    // создание матрицы 3x3
    let mut m = Matritsa::new(3, 3).to_ldirish(vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 8.0, 9.0,
    ]);

    // Index — o'qish
    // Index — чтение
    println!("m[0,0] = {}", m[(0, 0)]);
    println!("m[1,1] = {}", m[(1, 1)]);
    println!("m[2,2] = {}", m[(2, 2)]);
    // m[0,0] = 1
    // m[1,1] = 5
    // m[2,2] = 9

    // IndexMut — o'zgartirish
    // IndexMut — изменение
    m[(0, 0)] = 99.0;
    m[(1, 1)] = 55.0;
    println!("{}", m);
    // [99.0, 2.0, 3.0]
    // [4.0, 55.0, 6.0]
    // [7.0, 8.0, 9.0]

    // RgbQiymati — Rang enum bilan indekslash
    // RgbQiymati — индексация через enum Rang
    let mut rang = RgbQiymati {
        qizil: 255,
        yashil: 128,
        moviy: 0,
    };

    // Index — o'qish
    // Index — чтение
    println!("Qizil:  {}", rang[Rang::Qizil]);
    println!("Yashil: {}", rang[Rang::Yashil]);
    println!("Moviy:  {}", rang[Rang::Moviy]);
    // Qizil:  255
    // Yashil: 128
    // Moviy:  0

    // IndexMut — o'zgartirish
    // IndexMut — изменение
    rang[Rang::Qizil] = 200;
    rang[Rang::Moviy] = 100;
    println!("Yangi qizil: {}", rang[Rang::Qizil]);
    println!("Yangi moviy: {}", rang[Rang::Moviy]);
    // Yangi qizil: 200
    // Yangi moviy: 100

    // Konfiguratsiya — &str bilan indekslash
    // Конфигурация — индексация через &str
    let config = Konfiguratsiya::new();
    println!("Host: {}", config["host"]);
    // Host: localhost

    // XavfsizVec — Index va xavfsiz ol()
    // XavfsizVec — Index и безопасный ol()
    let mut xv: XavfsizVec<i32> = XavfsizVec::new();
    xv.qosh(10);
    xv.qosh(20);
    xv.qosh(30);

    // Index — to'g'ridan o'qish
    // Index — прямое чтение
    println!("{}", xv[0]);
    println!("{}", xv[1]);
    // 10
    // 20

    // IndexMut — o'zgartirish
    // IndexMut — изменение
    xv[0] = 99;
    println!("{}", xv[0]);
    // 99

    // xavfsiz o'qish — chegaradan oshsa None
    // безопасное чтение — None при выходе за границы
    println!("{:?}", xv.ol(0));
    println!("{:?}", xv.ol(100));
    // Some(99)
    // None

    // BitArray — bit qiymatlarini o'qish
    // BitArray — чтение значений битов
    let mut bits = BitArray::new(16);

    // 5-bit ni yoqamiz
    // включаем 5-й бит
    bits.ichki[0] = 0b00100000;

    println!("bit[0] = {}", bits[0]);
    println!("bit[5] = {}", bits[5]);
    println!("bit[7] = {}", bits[7]);
    // bit[0] = false
    // bit[5] = true
    // bit[7] = false

    // RingBuffer — modulo indekslash
    // RingBuffer — индексация по модулю
    let mut rb: RingBuffer<i32> = RingBuffer::new(4);
    rb[0] = 10;
    rb[1] = 20;
    rb[2] = 30;
    rb[3] = 40;

    // Normal indekslar
    // Обычные индексы
    println!("{}", rb[0]);
    println!("{}", rb[3]);
    // 10
    // 40

    // Modulo — har qanday indeks ishlaydi
    // Модуль — любой индекс работает
    println!("{}", rb[4]);
    println!("{}", rb[7]);
    println!("{}", rb[100]);
    // 10  (4 % 4 = 0)
    // 40  (7 % 4 = 3)
    // 10  (100 % 4 = 0... wait 100%4=0 → rb[0]=10)

    // T: Index<usize> — generic bound
    // T: Index<usize> — generic ограничение
    fn birinchi_elementni_ol<T>(kolleksiya: &T) -> &T::Output
    where
        T: Index<usize>,
    {
        &kolleksiya[0]
    }

    let v: Vec<i32> = vec![10, 20, 30];
    let arr: [i32; 3] = [100, 200, 300];

    println!("{}", birinchi_elementni_ol(&v));
    println!("{}", birinchi_elementni_ol(&arr));
    // 10
    // 100

    // IndexMut bilan yozish
    // запись с IndexMut
    fn birinchisini_ozgartir<T>(kolleksiya: &mut T, yangi: T::Output)
    where
        T: IndexMut<usize>,
        T::Output: Sized,
    {
        kolleksiya[0] = yangi;
    }

    let mut v2: Vec<i32> = vec![1, 2, 3];
    birinchisini_ozgartir(&mut v2, 99);
    println!("{:?}", v2);
    // [99, 2, 3]
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya             | Tavsif (UZ)                                          | Описание (RU)                                        |
// #================================================================================================================================================#
// # |                                          INDEX TRAIT                                                                                         |
// #================================================================================================================================================#
// # |   1 | impl Index<Idx> for T    | T uchun [] operatorini o'qish                        | Оператор [] для чтения T                             |
// # |   2 | type Output = V;         | [] qaytaradigan tur (majburiy)                       | Тип возвращаемый [] (обязательно)                    |
// # |   3 | fn index(&self, i: Idx)  | &self.Output qaytaradi                               | Возвращает &self.Output                              |
// # |   4 | v[i]                     | Index::index(&v, i) ga ekvivalent                    | Эквивалентно Index::index(&v, i)                     |
// #================================================================================================================================================#
// # |                                        INDEXMUT TRAIT                                                                                        |
// #================================================================================================================================================#
// # |   5 | impl IndexMut<Idx> for T | T uchun [] operatorini o'zgartirish                  | Оператор [] для изменения T                          |
// # |   6 | fn index_mut(&mut self)  | &mut self.Output qaytaradi                           | Возвращает &mut self.Output                          |
// # |   7 | v[i] = x                 | IndexMut::index_mut(&mut v, i) ga ekvivalent         | Эквивалентно IndexMut::index_mut(&mut v, i)          |
// # |   8 | v[i] += 1                | index_mut orqali o'zgartirish                        | Изменение через index_mut                            |
// #================================================================================================================================================#
// # |                                    BUILT-IN INDEX TRAITLAR                                                                                   |
// #================================================================================================================================================#
// # |   9 | Vec<T>[usize]            | O'lchovi aniqlanadi, panic bo'lishi mumkin            | Размер известен, может паниковать                   |
// # |  10 | [T;N][usize]             | Array indekslash                                     | Индексация массива                                   |
// # |  11 | String[Range]            | String qismini olish                                 | Получение части String                               |
// # |  12 | HashMap<K,V>[K]          | Kalit bilan qiymat olish (mavjud bo'lmasa panic)     | Получение значения по ключу (паника если нет)        |
// #================================================================================================================================================#
// # |                                    CUSTOM INDEX TURLARI                                                                                      |
// #================================================================================================================================================#
// # |  13 | Index<(usize,usize)>     | 2D matritsa — tuple indeks                           | 2D матрица — индекс через кортеж                     |
// # |  14 | Index<Enum>              | Enum bilan indekslash                                | Индексация через enum                                |
// # |  15 | Index<&str>              | String kalit bilan indekslash                        | Индексация через строковый ключ                      |
// # |  16 | Index<usize> modulo      | RingBuffer — halqali indekslash                      | RingBuffer — кольцевая индексация                    |
// #================================================================================================================================================#
// # |                                    GENERIC BILAN                                                                                             |
// #================================================================================================================================================#
// # |  17 | T: Index<usize>          | Generic Index bound                                  | Generic ограничение Index                            |
// # |  18 | T: IndexMut<usize>       | Generic IndexMut bound                               | Generic ограничение IndexMut                         |
// # |  19 | T::Output                | Index chiqish turi                                   | Тип выхода Index                                     |
// #================================================================================================================================================#
// # |                                    MUHIM QOIDALAR                                                                                            |
// #================================================================================================================================================#
// # |  20 | Index kerak → IndexMut   | IndexMut implement uchun Index ham kerak             | Для IndexMut нужен и Index                           |
// # |  21 | Panic xavfi              | Chegaradan oshsa panic — .get() xavfsizroq           | Паника при выходе за границу — .get() безопаснее     |
// # |  22 | &v[i] va v[i]            | Index &T, lekin Deref orqali T ga ham kirish         | Index &T, но доступ к T через Deref                  |
// #================================================================================================================================================#