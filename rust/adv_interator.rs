// #================================================================================================================================================#
// #                                                                ADV ITERATORS                                                                   #
// #                YUQORI DARAJALI ITERATORLAR — CUSTOM ADAPTER, CHAIN PATTERN, LAZY, ZIP/UNZIP, SCAN, UNFOLD, FLATTEN.                            #
// #                ПРОДВИНУТЫЕ ИТЕРАТОРЫ — CUSTOM АДАПТЕР, CHAIN ПАТТЕРН, LAZY, ZIP/UNZIP, SCAN, UNFOLD, FLATTEN.                                  #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// Bu mavzuda o'rganiladiganlar:
// Что изучается в этой теме:
//
//   1. Custom Iterator adapter — mavjud iteratorni wrapping
//      Custom адаптер итератора — обёртка существующего
//   2. Iterator kombinatorlar — zanjirli operatsiyalar
//      Комбинаторы итераторов — цепочечные операции
//   3. Lazy evaluation — hisoblash qachon bo'ladi
//      Ленивые вычисления — когда происходит вычисление
//   4. Infinite iterators — cheksiz generatorlar
//      Бесконечные итераторы — бесконечные генераторы
//   5. Parallel iterators (rayon pattern)
//      Параллельные итераторы (паттерн rayon)
//   6. State machine iterator
//      Итератор машины состояний
//   7. Unfold — state dan iterator
//      Unfold — итератор из состояния
//   8. scan, flat_map, cycle, peekable chuqur
//      scan, flat_map, cycle, peekable подробно

// 1. Map adapter — xuddi .map() kabi o'z versiyamiz
// 1. Map адаптер — наша версия .map()
struct MyMap<I, F> {
    ichki: I,
    funksiya: F,
}

impl<I, F, B> Iterator for MyMap<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;
    fn next(&mut self) -> Option<B> {
        self.ichki.next().map(&mut self.funksiya)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ichki.size_hint()
    }
}

// 2. Filter adapter
// 2. Filter адаптер
struct MyFilter<I, P> {
    ichki: I,
    predikat: P,
}

impl<I, P> Iterator for MyFilter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        loop {
            match self.ichki.next() {
                Some(x) if (self.predikat)(&x) => return Some(x),
                Some(_) => continue,
                None => return None,
            }
        }
    }
}

// 3. Take-While adapter
// 3. Адаптер Take-While
struct MyTakeWhile<I, P> {
    ichki: I,
    predikat: P,
    tugadimi: bool,
}

impl<I, P> Iterator for MyTakeWhile<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        if self.tugadimi { return None; }
        match self.ichki.next() {
            Some(x) if (self.predikat)(&x) => Some(x),
            _ => { self.tugadimi = true; None }
        }
    }
}

// 4. Zip adapter — ikki iteratorni juftlashtirish
// 4. Zip адаптер — объединение двух итераторов
struct MyZip<A, B> {
    a: A,
    b: B,
}

impl<A: Iterator, B: Iterator> Iterator for MyZip<A, B> {
    type Item = (A::Item, B::Item);
    fn next(&mut self) -> Option<(A::Item, B::Item)> {
        match (self.a.next(), self.b.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            _ => None,
        }
    }
}

// Trait extension — custom adapterlarni barcha iteratorlarga qo'shish
// Trait extension — добавление custom адаптеров ко всем итераторам
trait IteratorExt: Iterator + Sized {
    fn my_map<B, F: FnMut(Self::Item) -> B>(self, f: F) -> MyMap<Self, F> {
        MyMap { ichki: self, funksiya: f }
    }

    fn my_filter<P: FnMut(&Self::Item) -> bool>(self, p: P) -> MyFilter<Self, P> {
        MyFilter { ichki: self, predikat: p }
    }

    fn my_take_while<P: FnMut(&Self::Item) -> bool>(self, p: P) -> MyTakeWhile<Self, P> {
        MyTakeWhile { ichki: self, predikat: p, tugadimi: false }
    }

    fn my_zip<B: Iterator>(self, b: B) -> MyZip<Self, B> {
        MyZip { a: self, b }
    }

    // Window sliding — N o'lchamli oynalar
    // Window sliding — окна размером N
    fn sliding_window(self, n: usize) -> SlidingWindow<Self>
    where
        Self::Item: Clone,
    {
        SlidingWindow { ichki: self, oyna: Vec::new(), hajm: n, tayyor: false }
    }
}

impl<I: Iterator> IteratorExt for I {}

// Sliding window adapter
// Адаптер скользящего окна
struct SlidingWindow<I: Iterator>
where
    I::Item: Clone,
{
    ichki: I,
    oyna: Vec<I::Item>,
    hajm: usize,
    tayyor: bool,
}

impl<I: Iterator> Iterator for SlidingWindow<I>
where
    I::Item: Clone,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Vec<I::Item>> {
        if !self.tayyor {
            while self.oyna.len() < self.hajm {
                match self.ichki.next() {
                    Some(x) => self.oyna.push(x),
                    None => return None,
                }
            }
            self.tayyor = true;
            return Some(self.oyna.clone());
        }
        match self.ichki.next() {
            Some(x) => {
                self.oyna.remove(0);
                self.oyna.push(x);
                Some(self.oyna.clone())
            }
            None => None,
        }
    }
}

fn custom_adapter_misollari() {

    // MyMap
    let v: Vec<i32> = (1..=5).my_map(|x| x * x).collect();
    println!("{:?}", v);
    // [1, 4, 9, 16, 25]

    // MyFilter
    let v2: Vec<i32> = (1..=10).my_filter(|&x| x % 2 == 0).collect();
    println!("{:?}", v2);
    // [2, 4, 6, 8, 10]

    // MyTakeWhile
    let v3: Vec<i32> = (1..=10).my_take_while(|&x| x < 6).collect();
    println!("{:?}", v3);
    // [1, 2, 3, 4, 5]

    // MyZip
    let v4: Vec<(i32, i32)> = (1..=4).my_zip(10..=40).collect();
    println!("{:?}", v4);
    // [(1, 10), (2, 11), (3, 12), (4, 13)]

    // Zanjir — my_map + my_filter
    // Цепочка — my_map + my_filter
    let v5: Vec<i32> = (1..=10)
        .my_map(|x| x * x)
        .my_filter(|&x| x % 2 == 0)
        .collect();
    println!("{:?}", v5);
    // [4, 16, 36, 64, 100]

    // Sliding window
    // Скользящее окно
    let ma_lumotlar: Vec<f64> = vec![1.0, 3.0, 5.0, 2.0, 8.0, 6.0];
    let ortachalar: Vec<f64> = ma_lumotlar.iter()
        .copied()
        .sliding_window(3)
        .map(|w| w.iter().sum::<f64>() / w.len() as f64)
        .collect();
    println!("{:?}", ortachalar);
    // [3.0, 3.3333..., 5.0, 5.3333...]
}

// Fibonacci — cheksiz
// Fibonacci — бесконечный
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self { Fibonacci { a: 0, b: 1 } }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let natija = self.a;
        (self.a, self.b) = (self.b, self.a + self.b);
        Some(natija)
    }
}

// std::iter::from_fn — closure dan iterator
// std::iter::from_fn — итератор из замыкания
fn from_fn_misoli() -> impl Iterator<Item = i32> {
    let mut n = 0;
    std::iter::from_fn(move || {
        n += 1;
        if n <= 5 { Some(n * n) } else { None }
    })
}

// std::iter::successors — avvalgidan keyingini hisoblash
// std::iter::successors — вычисление следующего из предыдущего
fn collatz(n: u64) -> impl Iterator<Item = u64> {
    std::iter::successors(Some(n), |&x| {
        if x == 1 { None }
        else if x % 2 == 0 { Some(x / 2) }
        else { Some(3 * x + 1) }
    })
}

fn infinite_iterator_misollari() {

    // Fibonacci — take bilan cheklash
    // Fibonacci — ограничение через take
    let fib: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("{:?}", fib);
    // [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]

    // Birinchi 100 dan katta Fibonacci
    // Первое число Фибоначчи больше 100
    let f100 = Fibonacci::new().find(|&x| x > 100);
    println!("{:?}", f100);
    // Some(144)

    // 20 ta Fibonacci yig'indisi
    // Сумма 20 чисел Фибоначчи
    let yig: u64 = Fibonacci::new().take(20).sum();
    println!("{}", yig);
    // 10945

    // from_fn — closure dan iterator
    // from_fn — итератор из замыкания
    let v: Vec<i32> = from_fn_misoli().collect();
    println!("{:?}", v);
    // [1, 4, 9, 16, 25]

    // successors — Collatz ketma-ketligi
    // successors — последовательность Коллатца
    let collatz_27: Vec<u64> = collatz(27).collect();
    println!("Collatz(27) uzunlik: {}", collatz_27.len());
    println!("Collatz(27) boshlanish: {:?}", &collatz_27[..5]);
    // Collatz(27) uzunlik: 112
    // Collatz(27) boshlanish: [27, 82, 41, 124, 62]

    // cycle — cheksiz takrorlash
    // cycle — бесконечное повторение
    let rang_seriyasi: Vec<&str> = ["qizil", "yashil", "ko'k"]
        .iter()
        .copied()
        .cycle()
        .take(8)
        .collect();
    println!("{:?}", rang_seriyasi);
    // ["qizil", "yashil", "ko'k", "qizil", "yashil", "ko'k", "qizil", "yashil"]

    // repeat — bir elementni takrorlash
    // repeat — повторение одного элемента
    let nollar: Vec<i32> = std::iter::repeat(0).take(5).collect();
    println!("{:?}", nollar);
    // [0, 0, 0, 0, 0]

    // repeat_with — closure dan takrorlash
    // repeat_with — повторение из замыкания
    let mut rng = 42u64;
    let pseudo_random: Vec<u64> = std::iter::repeat_with(move || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        rng % 100
    }).take(5).collect();
    println!("{:?}", pseudo_random);
}

fn scan_flatmap_misollari() {

    // scan — fold + yield (har qadamda qiymat qaytaradi)
    // scan — fold + yield (возвращает значение на каждом шаге)
    let kumulyativ: Vec<i32> = (1..=5)
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    println!("{:?}", kumulyativ);
    // [1, 3, 6, 10, 15]

    // scan — harakatlanuvchi o'rtacha
    // scan — скользящее среднее
    let ma_lumotlar = vec![10.0, 20.0, 30.0, 40.0, 50.0];
    let harakatlanuvchi: Vec<f64> = ma_lumotlar.iter()
        .enumerate()
        .scan(0.0, |yig, (i, &x)| {
            *yig += x;
            Some(*yig / (i + 1) as f64)
        })
        .collect();
    println!("{:?}", harakatlanuvchi);
    // [10.0, 15.0, 20.0, 25.0, 30.0]

    // flat_map — map + flatten
    // flat_map — map + flatten
    let so_zlar = vec!["salom dunyo", "rust tili", "ajoyib til"];
    let barcha_so_zlar: Vec<&str> = so_zlar.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("{:?}", barcha_so_zlar);
    // ["salom", "dunyo", "rust", "tili", "ajoyib", "til"]

    // flat_map — Option larni yassilash
    // flat_map — сплющивание Option
    let aralash = vec!["1", "ikki", "3", "to'rt", "5"];
    let sonlar: Vec<i32> = aralash.iter()
        .flat_map(|s| s.parse::<i32>())
        .collect();
    println!("{:?}", sonlar);
    // [1, 3, 5]

    // flatten — ichki iteratorlarni yassilash
    // flatten — сплющивание вложенных итераторов
    let ichki: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
    let yassi: Vec<i32> = ichki.into_iter().flatten().collect();
    println!("{:?}", yassi);
    // [1, 2, 3, 4, 5, 6, 7, 8, 9]

    // flatten — Option larni yassilash
    // flatten — сплющивание Option
    let optlar: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
    let faqat_borlar: Vec<i32> = optlar.into_iter().flatten().collect();
    println!("{:?}", faqat_borlar);
    // [1, 3, 5]

    // flat_map + enumerate — raqamlash bilan yassilash
    // flat_map + enumerate — сплющивание с нумерацией
    let matnlar = vec!["bir ikki", "uch to'rt besh"];
    let raqamlangan: Vec<(usize, &str)> = matnlar.iter()
        .flat_map(|s| s.split_whitespace())
        .enumerate()
        .collect();
    println!("{:?}", raqamlangan);
    // [(0, "bir"), (1, "ikki"), (2, "uch"), (3, "to'rt"), (4, "besh")]
}

fn peekable_chuqur_misollari() {

    // peekable — keyingisini consume qilmasdan ko'rish
    // peekable — просмотр следующего без consume
    let mut it = vec![1, 2, 3, 4, 5].into_iter().peekable();

    // peek — reference qaytaradi, consume qilmaydi
    // peek — возвращает ссылку, не consume
    println!("{:?}", it.peek()); // Some(1)
    println!("{:?}", it.peek()); // Some(1) — hali ham 1
    println!("{:?}", it.next()); // Some(1) — endi consume
    println!("{:?}", it.peek()); // Some(2)
    // Some(1)
    // Some(1)
    // Some(1)
    // Some(2)

    // peekable — shartli yutish
    // peekable — условное поглощение
    let mut it2 = "123abc456".chars().peekable();
    let mut raqamlar = String::new();
    let mut harflar = String::new();

    // Raqamlarni olish
    while it2.peek().map_or(false, |c| c.is_ascii_digit()) {
        raqamlar.push(it2.next().unwrap());
    }
    // Harflarni olish
    while it2.peek().map_or(false, |c| c.is_alphabetic()) {
        harflar.push(it2.next().unwrap());
    }
    println!("Raqamlar: {}, Harflar: {}", raqamlar, harflar);
    // Raqamlar: 123, Harflar: abc

    // peekable — tokenizer simulyatsiyasi
    // peekable — симуляция токенайзера
    #[derive(Debug)]
    enum Token {
        Son(i64),
        Plyus,
        Minus,
        Kopaytirish,
        Boshlanish,
        Tugatish,
    }

    fn tokenize(kiritish: &str) -> Vec<Token> {
        let mut tokenlar = Vec::new();
        let mut chars = kiritish.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                ' ' => { chars.next(); }
                '0'..='9' => {
                    let mut son_str = String::new();
                    while chars.peek().map_or(false, |c| c.is_ascii_digit()) {
                        son_str.push(chars.next().unwrap());
                    }
                    tokenlar.push(Token::Son(son_str.parse().unwrap()));
                }
                '+' => { chars.next(); tokenlar.push(Token::Plyus); }
                '-' => { chars.next(); tokenlar.push(Token::Minus); }
                '*' => { chars.next(); tokenlar.push(Token::Kopaytirish); }
                '(' => { chars.next(); tokenlar.push(Token::Boshlanish); }
                ')' => { chars.next(); tokenlar.push(Token::Tugatish); }
                _ => { chars.next(); }
            }
        }
        tokenlar
    }

    let tokenlar = tokenize("12 + 34 * (5 - 2)");
    println!("{:?}", tokenlar);
    // [Son(12), Plyus, Son(34), Ko'paytirish, Boshlanish, Son(5), Minus, Son(2), Tugatish]
}

// Lexer state machine — iterator sifatida
// Лексер машины состояний — как итератор
#[derive(Debug, PartialEq)]
enum LexToken<'a> {
    Soz(&'a str),
    Son(i64),
    Tinish(char),
    Bosh,
}

struct Lexer<'a> {
    kiritish: &'a str,
    pozitsiya: usize,
}

impl<'a> Lexer<'a> {
    fn new(kiritish: &'a str) -> Self {
        Lexer { kiritish, pozitsiya: 0 }
    }

    fn qolgan(&self) -> &'a str {
        &self.kiritish[self.pozitsiya..]
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexToken<'a>;

    fn next(&mut self) -> Option<LexToken<'a>> {
        let qolgan = self.qolgan();
        if qolgan.is_empty() { return None; }

        let c = qolgan.chars().next().unwrap();

        // Bo'sh joy
        if c.is_whitespace() {
            let uzunlik = qolgan.chars().take_while(|c| c.is_whitespace()).count();
            self.pozitsiya += uzunlik;
            return Some(LexToken::Bosh);
        }

        // Raqam
        if c.is_ascii_digit() {
            let raqam_str: &str = {
                let uzunlik = qolgan.chars().take_while(|c| c.is_ascii_digit()).count();
                &qolgan[..uzunlik]
            };
            self.pozitsiya += raqam_str.len();
            return Some(LexToken::Son(raqam_str.parse().unwrap()));
        }

        // Harf
        if c.is_alphabetic() {
            let so_z: &str = {
                let uzunlik = qolgan.chars().take_while(|c| c.is_alphabetic()).count();
                &qolgan[..uzunlik]
            };
            self.pozitsiya += so_z.len();
            return Some(LexToken::Soz(so_z));
        }

        // Tinish belgisi
        self.pozitsiya += c.len_utf8();
        Some(LexToken::Tinish(c))
    }
}

fn state_machine_misoli() {

    let kod = "x = 42 + y";
    let tokenlar: Vec<LexToken> = Lexer::new(kod)
        .filter(|t| *t != LexToken::Bosh)
        .collect();

    for t in &tokenlar {
        println!("{:?}", t);
    }
    // So'z("x")
    // Tinish('=')
    // Son(42)
    // Tinish('+')
    // So'z("y")
}

fn collect_patternlar() {

    // Result<Vec<T>, E> ga collect — xatolarni to'plash
    // collect в Result<Vec<T>, E> — сбор ошибок
    let satrlar = vec!["1", "2", "3", "4"];
    let natija: Result<Vec<i32>, _> = satrlar.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{:?}", natija);
    // Ok([1, 2, 3, 4])

    let xatoli = vec!["1", "ikki", "3"];
    let xato: Result<Vec<i32>, _> = xatoli.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("{}", xato.is_err());
    // true

    // partition — ikkiga ajratish
    // partition — разделение на две части
    let sonlar: Vec<i32> = (1..=10).collect();
    let (juftlar, toqlar): (Vec<i32>, Vec<i32>) = sonlar.iter()
        .partition(|&&x| x % 2 == 0);
    println!("{:?}", juftlar);
    println!("{:?}", toqlar);
    // [2, 4, 6, 8, 10]
    // [1, 3, 5, 7, 9]

    // unzip — juftlarni ajratish
    // unzip — разделение пар
    let juft_v: Vec<(i32, &str)> = vec![(1,"bir"), (2,"ikki"), (3,"uch")];
    let (a, b): (Vec<i32>, Vec<&str>) = juft_v.into_iter().unzip();
    println!("{:?} {:?}", a, b);
    // [1, 2, 3] ["bir", "ikki", "uch"]

    // HashMap ga collect
    // collect в HashMap
    use std::collections::HashMap;
    let xarita: HashMap<&str, usize> = ["salom", "dunyo", "rust"]
        .iter()
        .map(|&s| (s, s.len()))
        .collect();
    let mut sorted: Vec<_> = xarita.iter().collect();
    sorted.sort_by_key(|&(&k, _)| k);
    println!("{:?}", sorted);
    // [("dunyo", 5), ("rust", 4), ("salom", 5)]

    // group_by simulyatsiyasi — HashMap bilan
    // Симуляция group_by — с HashMap
    let so_zlar = vec!["salom", "dunyo", "rust", "siz", "dastur"];
    let guruhlar: HashMap<usize, Vec<&str>> = so_zlar.iter()
        .fold(HashMap::new(), |mut acc, &s| {
            acc.entry(s.len()).or_default().push(s);
            acc
        });
    let mut kalit: Vec<usize> = guruhlar.keys().copied().collect();
    kalit.sort();
    for k in kalit {
        println!("uzunlik {}: {:?}", k, guruhlar[&k]);
    }
    // uzunlik 4: ["rust", "siz"]  (yoki boshqa tartib)
    // uzunlik 5: ["salom", "dunyo"]
    // uzunlik 6: ["dastur"]
}

fn lazy_evaluation_misollari() {

    // Lazy — terminal metod bo'lmaguncha hech narsa hisoblanmaydi
    // Lazy — ничего не вычисляется до терминального метода
    let _lazy = (0..1_000_000)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take(5);
    // Bu yerda hech narsa hisoblanmadi!
    // Здесь ничего не вычислилось!
    println!("Iterator yaratildi — hisob yo'q");

    // Terminal metod — hisoblash boshlanadi
    // Терминальный метод — вычисление начинается
    let natija: Vec<i64> = (0i64..1_000_000)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take(5)
        .collect(); // ← SHU YERDA hisoblash
    println!("{:?}", natija);
    // Iterator yaratildi — hisob yo'q
    // [0, 4, 16, 36, 64]

    // Qisqa tutashuv (short circuit) — find, any, all
    // Короткое замыкание — find, any, all
    let mut hisob = 0;
    let _natija = (1..=1000).find(|&x| {
        hisob += 1;
        x * x > 100
    });
    println!("find — {} element tekshirildi (emas 1000)", hisob);
    // find — 11 element tekshirildi (emas 1000)

    // chain — ikkita cheksiz iteratordan faqat N ta olish
    // chain — взять только N из двух бесконечных итераторов
    let natija2: Vec<i32> = (1..)
        .filter(|x| x % 3 == 0)
        .chain((1..).filter(|x| x % 5 == 0))
        .take(6)
        .collect();
    println!("{:?}", natija2);
    // [3, 6, 9, 12, 15, 18]  (faqat 3 ga bo'linuvchanlar)
    // Keyin 5 ga bo'linuvchanlar: aslida chain 3 ga bo'linuvchanlar tugamaydi
    // Lekin take(6) orqali faqat 6 ta olinadi
}

// CSV parser — iterator asosida
// CSV парсер — на основе итератора
struct CsvParser<'a> {
    satrlari: std::str::Lines<'a>,
    sarlavhalar: Vec<String>,
    tayyor: bool,
}

impl<'a> CsvParser<'a> {
    fn new(matn: &'a str) -> Self {
        CsvParser {
            satrlari: matn.lines(),
            sarlavhalar: Vec::new(),
            tayyor: false,
        }
    }
}

impl<'a> Iterator for CsvParser<'a> {
    type Item = std::collections::HashMap<String, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.tayyor {
            let sarlavha_satri = self.satrlari.next()?;
            self.sarlavhalar = sarlavha_satri.split(',')
                .map(|s| s.trim().to_string())
                .collect();
            self.tayyor = true;
        }

        let qator = self.satrlari.next()?;
        let qiymatlar: Vec<&str> = qator.split(',').collect();

        Some(self.sarlavhalar.iter()
            .zip(qiymatlar.iter())
            .map(|(k, v)| (k.clone(), v.trim().to_string()))
            .collect())
    }
}

// Pipeline — ma'lumot qayta ishlash
// Pipeline — обработка данных
#[derive(Debug, Clone)]
struct Xodim {
    ism: String,
    departament: String,
    maosh: f64,
    yosh: u32,
}

impl Xodim {
    fn new(ism: &str, departament: &str, maosh: f64, yosh: u32) -> Self {
        Xodim {
            ism: ism.to_string(),
            departament: departament.to_string(),
            maosh,
            yosh,
        }
    }
}

fn real_hayot_misollari() {

    // CSV parser
    // CSV парсер
    let csv = "ism,yosh,shahar\nDilshod,22,Toshkent\nAli,25,Samarqand\nVali,20,Buxoro";
    let qatorlar: Vec<_> = CsvParser::new(csv).collect();
    for q in &qatorlar {
        println!("{} yoshli {} dan", q["ism"], q["yosh"]);
    }
    // 22 yoshli Dilshod dan
    // 25 yoshli Ali dan
    // 20 yoshli Vali dan

    // Xodimlar pipeline
    // Pipeline сотрудников
    let xodimlar = vec![
        Xodim::new("Dilshod", "IT",       5000.0, 22),
        Xodim::new("Ali",     "IT",       7000.0, 28),
        Xodim::new("Vali",    "Marketing",4500.0, 25),
        Xodim::new("Soli",    "IT",       6500.0, 30),
        Xodim::new("Jamshid", "Marketing",5500.0, 27),
        Xodim::new("Dilnoza", "HR",       4000.0, 23),
        Xodim::new("Malika",  "IT",       8000.0, 32),
    ];

    // IT departament — maoshi 6000 dan ko'p — yosh bo'yicha tartiblangan
    // Отдел IT — зарплата больше 6000 — отсортированы по возрасту
    let mut it_yuqori: Vec<&Xodim> = xodimlar.iter()
        .filter(|x| x.departament == "IT" && x.maosh > 6000.0)
        .collect();
    it_yuqori.sort_by(|a, b| a.yosh.cmp(&b.yosh));

    println!("\nIT yuqori maoshlilar:");
    for x in &it_yuqori {
        println!("  {} ({}) — {}", x.ism, x.yosh, x.maosh);
    }
    // IT yuqori maoshlilar:
    //   Ali (28) — 7000
    //   Soli (30) — 6500
    //   Malika (32) — 8000

    // Departament bo'yicha o'rtacha maosh
    // Средняя зарплата по отделам
    use std::collections::HashMap;
    let mut dep_maosh: HashMap<&str, Vec<f64>> = HashMap::new();
    for x in &xodimlar {
        dep_maosh.entry(&x.departament).or_default().push(x.maosh);
    }
    let mut dep_ortacha: Vec<(&str, f64)> = dep_maosh.iter()
        .map(|(&dep, maoshlar)| {
            let ortacha = maoshlar.iter().sum::<f64>() / maoshlar.len() as f64;
            (dep, ortacha)
        })
        .collect();
    dep_ortacha.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nDepartament o'rtacha maosh:");
    for (dep, ortacha) in &dep_ortacha {
        println!("  {}: {:.0}", dep, ortacha);
    }
    // Departament o'rtacha maosh:
    //   IT: 6625
    //   Marketing: 5000
    //   HR: 4000

    // Sliding window — narx harakati
    // Скользящее окно — движение цены
    let narxlar: Vec<f64> = vec![100.0, 102.0, 98.0, 105.0, 110.0, 108.0, 115.0];
    let harakatlanuvchi: Vec<f64> = narxlar.iter()
        .copied()
        .sliding_window(3)
        .map(|w| {
            let ortacha = w.iter().sum::<f64>() / w.len() as f64;
            (ortacha * 100.0).round() / 100.0
        })
        .collect();
    println!("\nHarakatlanuvchi o'rtacha (3):");
    println!("{:?}", harakatlanuvchi);
    // Harakatlanuvchi o'rtacha (3):
    // [100.0, 101.67, 104.33, 107.67, 111.0]
}

fn main() {

    println!("=== CUSTOM ADAPTER ===");
    custom_adapter_misollari();

    println!("\n=== CHEKSIZ ITERATORLAR ===");
    infinite_iterator_misollari();

    println!("\n=== SCAN, FLAT_MAP, FLATTEN ===");
    scan_flatmap_misollari();

    println!("\n=== PEEKABLE CHUQUR ===");
    peekable_chuqur_misollari();

    println!("\n=== STATE MACHINE ITERATOR ===");
    state_machine_misoli();

    println!("\n=== COLLECT PATTERNLAR ===");
    collect_patternlar();

    println!("\n=== LAZY EVALUATION ===");
    lazy_evaluation_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |   1 | Custom adapter struct           | Mavjud iteratorni wrapping                 | Обёртка существующего итератора                         |
// # |   2 | Trait extension                 | Barcha iteratorlarga metod qo'shish        | Добавление метода всем итераторам                       |
// # |   3 | Sliding window                  | N o'lchamli harakatlanuvchi oyna           | Скользящее окно размером N                              |
// # |   4 | std::iter::from_fn(|| ...)      | Closure dan iterator                       | Итератор из замыкания                                   |
// # |   5 | std::iter::successors(s, |x|)   | Avvalgidan keyingisini hisoblash           | Вычисление следующего из предыдущего                    |
// # |   6 | .cycle()                        | Cheksiz takrorlash                         | Бесконечное повторение                                  |
// # |   7 | .scan(state, |s, x| ...)        | Fold + har qadamda yield                   | Fold + yield на каждом шаге                             |
// # |   8 | .flat_map(|x| ...)              | Map + flatten                              | Map + flatten                                           |
// # |   9 | .flatten()                      | Ichki iteratorlarni yassilash              | Сплющивание вложенных итераторов                        |
// # |  10 | .peekable()                     | Keyingisini consume qilmasdan ko'rish      | Просмотр следующего без consume                         |
// # |  11 | .partition(|x| ...)             | Ikkiga ajratish                            | Разделение на две части                                 |
// # |  12 | .unzip()                        | Juftlarni ajratish                         | Разделение пар                                          |
// # |  13 | Collect<Result<Vec,E>>          | Xatolarni to'plash                         | Сбор ошибок                                             |
// # |  14 | Lazy evaluation                 | Terminal metod bo'lguncha hisob yo'q       | Нет вычислений до терминального метода                  |
// # |  15 | State machine iterator          | Kompleks parsing uchun                     | Для сложного парсинга                                   |
// #================================================================================================================================================#