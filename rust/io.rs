// #================================================================================================================================================#
// #                                                                   STD::IO                                                                      #
// #                        STD::IO — READ, WRITE, SEEK, BUFREAD. STDIN, STDOUT, STDERR. BUFIO. CURSOR. CHAINING.                                   #
// #                        STD::IO — READ, WRITE, SEEK, BUFREAD. STDIN, STDOUT, STDERR. BUFIO. CURSOR. ЦЕПОЧКА.                                    #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::io::{
    self, Read, Write, Seek, SeekFrom, BufRead,
    BufReader, BufWriter, Cursor, Lines,
    stdin, stdout, stderr,
    Error, ErrorKind, Result,
};
use std::fmt;

// std::io nima:
// Что такое std::io:
//
//   Read    — baytlarni o'qish trait
//   Write   — baytlarni yozish trait
//   Seek    — pozitsiyani o'zgartirish trait
//   BufRead — buferli o'qish (satrlar bo'yicha)
//
//   BufReader<R> — Read ni buferlashtirish
//   BufWriter<W> — Write ni buferlashtirish
//   Cursor<T>    — xotira ichida Read/Write/Seek
//
//   stdin()  — standart kirish
//   stdout() — standart chiqish
//   stderr() — standart xato chiqishi

fn read_trait_misollari() {

    // Cursor — xotirada Read/Write/Seek
    // Cursor — Read/Write/Seek в памяти
    let ma_lumot: Vec<u8> = b"Salom Dunyo! Rust tili.".to_vec();
    let mut cursor = Cursor::new(ma_lumot);

    // read() — bufer to'ldirish
    // read() — заполнение буфера
    let mut buf = [0u8; 5];
    let n = cursor.read(&mut buf).unwrap();
    println!("O'qildi ({} bayt): {:?}", n, std::str::from_utf8(&buf[..n]).unwrap());
    // O'qildi (5 bayt): "Salom"

    // read_exact() — aniq miqdor o'qish
    // read_exact() — чтение точного количества
    let mut buf2 = [0u8; 6];
    cursor.read_exact(&mut buf2).unwrap();
    println!("Exact: {:?}", std::str::from_utf8(&buf2).unwrap());
    // Exact: " Dunyo"

    // read_to_end() — oxirigacha o'qish
    // read_to_end() — читать до конца
    let mut qolgan = Vec::new();
    cursor.read_to_end(&mut qolgan).unwrap();
    println!("Qolgan: {:?}", std::str::from_utf8(&qolgan).unwrap());
    // Qolgan: "! Rust tili."

    // read_to_string()
    // read_to_string()
    let mut cursor2 = Cursor::new(b"Unicode matn: salom!");
    let mut matn = String::new();
    cursor2.read_to_string(&mut matn).unwrap();
    println!("{}", matn);
    // Unicode matn: salom!

    // bytes() — baytlar iteratori
    // bytes() — итератор байтов
    let mut cursor3 = Cursor::new(b"ABC");
    let baytlar: Vec<u8> = cursor3.bytes()
        .map(|b| b.unwrap())
        .collect();
    println!("{:?}", baytlar); // [65, 66, 67]
    // [65, 66, 67]

    // chain() — ikki reader birlashtirish
    // chain() — объединение двух reader
    let r1 = Cursor::new(b"Birinchi ");
    let r2 = Cursor::new(b"Ikkinchi");
    let mut zanjir = r1.chain(r2);
    let mut natija = String::new();
    zanjir.read_to_string(&mut natija).unwrap();
    println!("{}", natija); // Birinchi Ikkinchi
    // Birinchi Ikkinchi

    // take() — faqat N bayt o'qish
    // take() — читать только N байт
    let mut cursor4 = Cursor::new(b"Faqat beshta");
    let mut limited = cursor4.take(5);
    let mut s = String::new();
    limited.read_to_string(&mut s).unwrap();
    println!("{}", s); // Faqat
    // Faqat
}

fn write_trait_misollari() {

    // Cursor — xotirada yozish
    // Cursor — запись в память
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    cursor.write_all(b"Salom ").unwrap();
    cursor.write_all(b"Dunyo!").unwrap();
    println!("{:?}", std::str::from_utf8(cursor.get_ref()).unwrap());
    // "Salom Dunyo!"

    // write! makrosi — formatli yozish
    // Макрос write! — форматированная запись
    let mut buf: Vec<u8> = Vec::new();
    write!(buf, "x={}, y={}", 10, 20).unwrap();
    writeln!(buf, " | z={}", 30).unwrap();
    writeln!(buf, "Tugadi").unwrap();
    println!("{}", String::from_utf8(buf).unwrap());
    // x=10, y=20 | z=30
    // Tugadi

    // flush() — buferdagi ma'lumotni yozish
    // flush() — запись данных из буфера
    let mut cursor2: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    cursor2.write_all(b"Bufer").unwrap();
    cursor2.flush().unwrap(); // Cursor uchun no-op
    println!("Flush bajarildi");
    // Flush bajarildi

    // write_vectored() — bir nechta buferdan yozish
    // write_vectored() — запись из нескольких буферов
    use std::io::IoSlice;
    let mut out: Vec<u8> = Vec::new();
    let slice1 = IoSlice::new(b"Birinchi ");
    let slice2 = IoSlice::new(b"Ikkinchi ");
    let slice3 = IoSlice::new(b"Uchinchi");
    out.write_vectored(&[slice1, slice2, slice3]).unwrap();
    println!("{}", String::from_utf8(out).unwrap());
    // Birinchi Ikkinchi Uchinchi

    // stderr() — standart xato chiqishi
    // stderr() — стандартный вывод ошибок
    let mut err = stderr();
    writeln!(err, "[WARN] Bu stderr ga yozildi").unwrap();
    // [WARN] Bu stderr ga yozildi (stderr ga)
}

fn seek_misollari() {

    let ma_lumot = b"0123456789ABCDEF";
    let mut cursor = Cursor::new(ma_lumot.to_vec());

    // SeekFrom::Start — boshidan
    // SeekFrom::Start — от начала
    cursor.seek(SeekFrom::Start(5)).unwrap();
    let mut buf = [0u8; 3];
    cursor.read_exact(&mut buf).unwrap();
    println!("Start(5): {:?}", std::str::from_utf8(&buf).unwrap()); // "567"
    // Start(5): "567"

    // SeekFrom::End — oxiridan
    // SeekFrom::End — от конца
    cursor.seek(SeekFrom::End(-4)).unwrap();
    cursor.read_exact(&mut buf).unwrap();
    println!("End(-4): {:?}", std::str::from_utf8(&buf).unwrap()); // "CDE"
    // End(-4): "CDE"

    // SeekFrom::Current — joriy pozitsiyadan
    // SeekFrom::Current — от текущей позиции
    cursor.seek(SeekFrom::Start(0)).unwrap(); // boshiga qaytish
    cursor.seek(SeekFrom::Current(10)).unwrap();
    cursor.read_exact(&mut buf).unwrap();
    println!("Current(10): {:?}", std::str::from_utf8(&buf).unwrap()); // "ABC"
    // Current(10): "ABC"

    // stream_position() — joriy pozitsiya (Rust 1.51+)
    // stream_position() — текущая позиция (Rust 1.51+)
    cursor.seek(SeekFrom::Start(7)).unwrap();
    let pos = cursor.stream_position().unwrap();
    println!("Pozitsiya: {}", pos); // 7
    // Pozitsiya: 7

    // rewind() — boshiga qaytish (Rust 1.55+)
    // rewind() — возврат в начало (Rust 1.55+)
    cursor.rewind().unwrap();
    let pos2 = cursor.stream_position().unwrap();
    println!("Rewind: {}", pos2); // 0
    // Rewind: 0

    // stream_len() — umumiy uzunlik (Rust 1.51+)
    // stream_len() — общая длина (Rust 1.51+)
    let uzunlik = cursor.seek(SeekFrom::End(0)).unwrap();
    println!("Uzunlik: {}", uzunlik); // 16
    // Uzunlik: 16
}

fn bufreader_misollari() {

    let matn = b"Birinchi satr\nIkkinchi satr\nUchinchi satr\nTo'rtinchi";
    let cursor = Cursor::new(matn);
    let mut reader = BufReader::new(cursor);

    // read_line() — bitta satr o'qish (newline bilan)
    // read_line() — чтение одной строки (с newline)
    let mut satr = String::new();
    let n = reader.read_line(&mut satr).unwrap();
    println!("read_line ({} bayt): {:?}", n, satr.trim_end());
    // read_line (14 bayt): "Birinchi satr"

    // lines() — satrlar iteratori
    // lines() — итератор строк
    let matn2 = b"salom\ndunyo\nrust\ntili";
    let reader2 = BufReader::new(Cursor::new(matn2));
    for (i, satr) in reader2.lines().enumerate() {
        println!("{}: {}", i + 1, satr.unwrap());
    }
    // 1: salom
    // 2: dunyo
    // 3: rust
    // 4: tili

    // split() — delimiter bilan bo'lish
    // split() — разделение по разделителю
    let csv = b"salom,dunyo,rust,tili";
    let reader3 = BufReader::new(Cursor::new(csv));
    let elementlar: Vec<String> = reader3
        .split(b',')
        .map(|s| String::from_utf8(s.unwrap()).unwrap())
        .collect();
    println!("{:?}", elementlar);
    // ["salom", "dunyo", "rust", "tili"]

    // fill_buf() — buferni to'ldirish
    // fill_buf() — заполнение буфера
    let ma_lumot = b"Test ma'lumot";
    let mut reader4 = BufReader::with_capacity(5, Cursor::new(ma_lumot));
    let buf = reader4.fill_buf().unwrap();
    println!("Bufer: {:?}", std::str::from_utf8(buf).unwrap());
    // Bufer: "Test " (faqat 5 bayt)

    // capacity() va buffer() — bufer holati
    // capacity() и buffer() — состояние буфера
    println!("Sig'im: {}", reader4.capacity()); // 5
    // Sig'im: 5
}

fn bufwriter_misollari() {

    // BufWriter — yozishlarni birlashtiradi
    // BufWriter — объединяет операции записи
    let mut buf_out: Vec<u8> = Vec::new();
    {
        let mut writer = BufWriter::new(&mut buf_out);

        writeln!(writer, "Birinchi satr").unwrap();
        writeln!(writer, "Ikkinchi satr").unwrap();
        write!(writer, "Uchinchi (flush kerak)").unwrap();

        // flush() — drop da avtomatik chaqiriladi
        // flush() — вызывается автоматически при drop
        writer.flush().unwrap();
    }
    println!("{}", String::from_utf8(buf_out).unwrap());
    // Birinchi satr
    // Ikkinchi satr
    // Uchinchi (flush kerak)

    // BufWriter::with_capacity — maxsus bufer hajmi
    // BufWriter::with_capacity — особый размер буфера
    let mut out2: Vec<u8> = Vec::new();
    let writer2 = BufWriter::with_capacity(1024, &mut out2);
    println!("BufWriter capacity: {}", writer2.capacity()); // 1024
    drop(writer2);
    // BufWriter capacity: 1024

    // into_inner() — ichki writer olish
    // into_inner() — получение внутреннего writer
    let mut out3: Vec<u8> = Vec::new();
    {
        let mut writer3 = BufWriter::new(&mut out3);
        write!(writer3, "Salom!").unwrap();
        // into_inner — BufWriterError ichida
    }
    println!("{:?}", String::from_utf8(out3).unwrap());
    // "Salom!"
}

fn io_error_misollari() {

    // Error turlari — ErrorKind
    // Виды ошибок — ErrorKind
    let xatolar = vec![
        Error::new(ErrorKind::NotFound, "Fayl topilmadi"),
        Error::new(ErrorKind::PermissionDenied, "Ruxsat yo'q"),
        Error::new(ErrorKind::ConnectionRefused, "Ulanish rad etildi"),
        Error::new(ErrorKind::TimedOut, "Vaqt tugadi"),
        Error::new(ErrorKind::UnexpectedEof, "Kutilmagan fayl oxiri"),
        Error::new(ErrorKind::AlreadyExists, "Allaqachon mavjud"),
        Error::new(ErrorKind::Interrupted, "Uzildi"),
    ];

    for e in &xatolar {
        println!("{:?}: {}", e.kind(), e);
    }
    // NotFound: Fayl topilmadi
    // PermissionDenied: Ruxsat yo'q
    // ...

    // ErrorKind bilan match
    // match с ErrorKind
    fn fayl_o_qi(yo_l: &str) -> Result<String> {
        std::fs::read_to_string(yo_l).map_err(|e| {
            match e.kind() {
                ErrorKind::NotFound => Error::new(ErrorKind::NotFound,
                                                  format!("'{}' fayli topilmadi", yo_l)),
                ErrorKind::PermissionDenied => Error::new(ErrorKind::PermissionDenied,
                                                          format!("'{}' ga ruxsat yo'q", yo_l)),
                _ => e,
            }
        })
    }

    match fayl_o_qi("/mavjud_emas.txt") {
        Ok(s) => println!("O'qildi: {}", s),
        Err(e) => println!("Xato [{:?}]: {}", e.kind(), e),
    }
    // Xato [NotFound]: '/mavjud_emas.txt' fayli topilmadi

    // io::Error dan boshqa xatoga o'tkazish
    // Преобразование из io::Error в другую ошибку
    #[derive(Debug)]
    enum AppXato {
        Io(io::Error),
        Parse(std::num::ParseIntError),
        Boshqa(String),
    }

    impl From<io::Error> for AppXato {
        fn from(e: io::Error) -> Self { AppXato::Io(e) }
    }

    impl From<std::num::ParseIntError> for AppXato {
        fn from(e: std::num::ParseIntError) -> Self { AppXato::Parse(e) }
    }

    fn kompleks_operatsiya() -> std::result::Result<i32, AppXato> {
        let matn = "42abc";
        let n: i32 = matn.parse::<i32>().map_err(AppXato::Parse)?;
        Ok(n)
    }

    println!("{:?}", kompleks_operatsiya());
    // Err(Parse(invalid digit found in string))

    // last_os_error() — tizim xatosini olish
    // last_os_error() — получение системной ошибки
    let tizim_xato = io::Error::last_os_error();
    println!("Tizim xatosi: {:?}", tizim_xato.kind());
}

fn stdin_stdout_misollari() {

    // stdout() — standart chiqish
    // stdout() — стандартный вывод
    let mut out = stdout();
    out.write_all(b"stdout ga to'g'ridan yozish\n").unwrap();
    out.flush().unwrap();
    // stdout ga to'g'ridan yozish

    // write! / writeln! — formatlash bilan
    // write! / writeln! — с форматированием
    writeln!(out, "Formatli: x={}, y={}", 10, 20).unwrap();
    // Formatli: x=10, y=20

    // print! / println! — stdout uchun qisqa
    // print! / println! — краткий для stdout
    println!("println! — stdout bilan bir xil");
    // println! — stdout bilan bir xil

    // stderr
    writeln!(stderr(), "[ERROR] Bu xato xabari").unwrap();
    // [ERROR] Bu xato xabari (stderr ga)

    // stdin — interaktiv (test uchun Cursor ishlatamiz)
    // stdin — интерактивный (используем Cursor для тестов)
    let kirish = b"birinchi satr\nikkinchi satr\n";
    let mut reader = BufReader::new(Cursor::new(kirish));

    let mut satr1 = String::new();
    let mut satr2 = String::new();
    reader.read_line(&mut satr1).unwrap();
    reader.read_line(&mut satr2).unwrap();
    println!("1: {}", satr1.trim());
    println!("2: {}", satr2.trim());
    // 1: birinchi satr
    // 2: ikkinchi satr

    // lock() — qulflangan stdin/stdout (tezroq)
    // lock() — заблокированный stdin/stdout (быстрее)
    let stdout_lock = io::stdout();
    let mut locked = stdout_lock.lock();
    writeln!(locked, "Qulflangan stdout ga yozish").unwrap();
    // Qulflangan stdout ga yozish
}

// O'z Read implementatsiyamiz
// Наша реализация Read
struct SonlarReader {
    sonlar: Vec<u8>,
    pozitsiya: usize,
}

impl SonlarReader {
    fn new(boshlanish: u8, n: usize) -> Self {
        SonlarReader {
            sonlar: (boshlanish..boshlanish.saturating_add(n as u8)).collect(),
            pozitsiya: 0,
        }
    }
}

impl Read for SonlarReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let qolgan = &self.sonlar[self.pozitsiya..];
        let n = qolgan.len().min(buf.len());
        buf[..n].copy_from_slice(&qolgan[..n]);
        self.pozitsiya += n;
        Ok(n)
    }
}

// O'z Write implementatsiyamiz — statistika bilan
// Наша реализация Write — со статистикой
struct StatistikWriter {
    ichki: Vec<u8>,
    yozilgan_baytlar: usize,
    yozilgan_satrlar: usize,
}

impl StatistikWriter {
    fn new() -> Self {
        StatistikWriter {
            ichki: Vec::new(),
            yozilgan_baytlar: 0,
            yozilgan_satrlar: 0,
        }
    }

    fn statistika(&self) -> (usize, usize) {
        (self.yozilgan_baytlar, self.yozilgan_satrlar)
    }

    fn matn(&self) -> &str {
        std::str::from_utf8(&self.ichki).unwrap_or("")
    }
}

impl Write for StatistikWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.yozilgan_satrlar += buf.iter().filter(|&&b| b == b'\n').count();
        self.ichki.extend_from_slice(buf);
        self.yozilgan_baytlar += buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> { Ok(()) }
}

fn custom_read_write_misollari() {

    // Custom Read
    let mut reader = SonlarReader::new(65, 10); // A..J
    let mut natija = String::new();
    reader.read_to_string(&mut natija).unwrap();
    println!("{}", natija); // ABCDEFGHIJ
    // ABCDEFGHIJ

    // BufReader bilan
    let reader2 = SonlarReader::new(97, 5); // a..e
    let mut buf_reader = BufReader::new(reader2);
    let mut s = Vec::new();
    buf_reader.read_to_end(&mut s).unwrap();
    println!("{:?}", std::str::from_utf8(&s).unwrap()); // "abcde"
    // "abcde"

    // Custom Write
    let mut writer = StatistikWriter::new();
    writeln!(writer, "Birinchi satr").unwrap();
    writeln!(writer, "Ikkinchi satr").unwrap();
    write!(writer, "Oxirgi").unwrap();

    let (baytlar, satrlar) = writer.statistika();
    println!("Yozildi: {} bayt, {} satr", baytlar, satrlar);
    print!("{}", writer.matn());
    // Yozildi: 35 bayt, 2 satr
    // Birinchi satr
    // Ikkinchi satr
    // Oxirgi
}

// CSV parser — BufReader bilan
// CSV парсер — с BufReader
fn csv_parser_misoli() {

    let csv_data = b"ism,yosh,shahar\nDilshod,22,Toshkent\nAli,25,Samarqand\nVali,20,Buxoro";
    let reader = BufReader::new(Cursor::new(csv_data));
    let mut satrlari = reader.lines();

    // Sarlavhalar
    let sarlavhalar: Vec<String> = satrlari.next()
        .unwrap().unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect();

    println!("Sarlavhalar: {:?}", sarlavhalar);

    // Ma'lumotlar
    for satr in satrlari {
        let satr = satr.unwrap();
        let qiymatlar: Vec<&str> = satr.split(',').collect();
        for (sarlavha, qiymat) in sarlavhalar.iter().zip(qiymatlar.iter()) {
            print!("{}: {} | ", sarlavha, qiymat);
        }
        println!();
    }
    // Sarlavhalar: ["ism", "yosh", "shahar"]
    // ism: Dilshod | yosh: 22 | shahar: Toshkent |
    // ism: Ali | yosh: 25 | shahar: Samarqand |
    // ism: Vali | yosh: 20 | shahar: Buxoro |
}

// Log yozuvchi — BufWriter bilan
// Записыватель логов — с BufWriter
struct LogYozuvchi {
    ichki: BufWriter<Vec<u8>>,
    satr_soni: usize,
}

impl LogYozuvchi {
    fn new() -> Self {
        LogYozuvchi {
            ichki: BufWriter::new(Vec::new()),
            satr_soni: 0,
        }
    }

    fn yoz(&mut self, daraja: &str, xabar: &str) {
        self.satr_soni += 1;
        writeln!(self.ichki, "[{:>5}] #{} {}", daraja, self.satr_soni, xabar).unwrap();
    }

    fn info(&mut self, xabar: &str)  { self.yoz("INFO",  xabar); }
    fn warn(&mut self, xabar: &str)  { self.yoz("WARN",  xabar); }
    fn error(&mut self, xabar: &str) { self.yoz("ERROR", xabar); }

    fn chiqar(mut self) -> String {
        self.ichki.flush().unwrap();
        let buf = self.ichki.into_inner().unwrap();
        String::from_utf8(buf).unwrap()
    }
}

fn real_hayot_misollari() {

    println!("--- CSV Parser ---");
    csv_parser_misoli();

    println!("\n--- Log Yozuvchi ---");
    let mut log = LogYozuvchi::new();
    log.info("Dastur boshlandi");
    log.info("Konfiguratsiya yuklandi");
    log.warn("Xotira 80% to'la");
    log.error("Fayl topilmadi: config.toml");
    log.info("Dastur tugadi");
    print!("{}", log.chiqar());
    // [ INFO] #1 Dastur boshlandi
    // [ INFO] #2 Konfiguratsiya yuklandi
    // [ WARN] #3 Xotira 80% to'la
    // [ERROR] #4 Fayl topilmadi: config.toml
    // [ INFO] #5 Dastur tugadi

    println!("\n--- Xotira bufer pipeline ---");
    // Read → transform → Write
    let kirish = b"salom\ndunyo\nrust\ntili";
    let mut chiqish: Vec<u8> = Vec::new();

    {
        let reader = BufReader::new(Cursor::new(kirish));
        let mut writer = BufWriter::new(&mut chiqish);

        for satr in reader.lines() {
            let satr = satr.unwrap().to_uppercase();
            writeln!(writer, ">> {}", satr).unwrap();
        }
    }

    print!("{}", String::from_utf8(chiqish).unwrap());
    // >> SALOM
    // >> DUNYO
    // >> RUST
    // >> TILI
}

fn main() {

    println!("=== READ TRAIT ===");
    read_trait_misollari();

    println!("\n=== WRITE TRAIT ===");
    write_trait_misollari();

    println!("\n=== SEEK ===");
    seek_misollari();

    println!("\n=== BUFREADER ===");
    bufreader_misollari();

    println!("\n=== BUFWRITER ===");
    bufwriter_misollari();

    println!("\n=== IO ERROR ===");
    io_error_misollari();

    println!("\n=== STDIN/STDOUT ===");
    stdin_stdout_misollari();

    println!("\n=== CUSTOM READ/WRITE ===");
    custom_read_write_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        READ TRAIT                                                                                            |
// #================================================================================================================================================#
// # |   1 | r.read(&mut buf)                | Bufer to'ldirish (0..n bayt)               | Заполнение буфера (0..n байт)                           |
// # |   2 | r.read_exact(&mut buf)          | Aniq miqdor o'qish                         | Чтение точного количества                               |
// # |   3 | r.read_to_end(&mut v)           | Oxirigacha Vec ga o'qish                   | Читать до конца в Vec                                   |
// # |   4 | r.read_to_string(&mut s)        | Oxirigacha String ga o'qish                | Читать до конца в String                                |
// # |   5 | r.bytes()                       | Baytlar iteratori                          | Итератор байтов                                         |
// # |   6 | r.chain(other)                  | Ikki reader birlashtirish                  | Объединение двух reader                                 |
// # |   7 | r.take(n)                       | Faqat N bayt o'qish                        | Читать только N байт                                    |
// #================================================================================================================================================#
// # |                                        WRITE TRAIT                                                                                           |
// #================================================================================================================================================#
// # |   8 | w.write_all(buf)                | Barchasini yozish                          | Записать всё                                            |
// # |   9 | write!(w, "...")                | Formatli yozish                            | Форматированная запись                                  |
// # |  10 | writeln!(w, "...")              | Satr bilan yozish                          | Запись со строкой                                       |
// # |  11 | w.flush()                       | Buferni tozalash                           | Очистка буфера                                          |
// # |  12 | w.write_vectored(&slices)       | Ko'p buferdan yozish                       | Запись из нескольких буферов                            |
// #================================================================================================================================================#
// # |                                        SEEK                                                                                                  |
// #================================================================================================================================================#
// # |  13 | SeekFrom::Start(n)              | Boshidan n bayt                            | n байт от начала                                        |
// # |  14 | SeekFrom::End(n)                | Oxiridan n bayt (manfiy)                   | n байт от конца (отрицательный)                         |
// # |  15 | SeekFrom::Current(n)            | Joriy pozitsiyadan                         | От текущей позиции                                      |
// # |  16 | s.stream_position()             | Joriy pozitsiyani olish                    | Получить текущую позицию                                |
// # |  17 | s.rewind()                      | Boshiga qaytish                            | Вернуться в начало                                      |
// #================================================================================================================================================#
// # |                                        BUFER                                                                                                 |
// #================================================================================================================================================#
// # |  18 | BufReader::new(r)               | Read ni buferlashtirish                    | Буферизация Read                                        |
// # |  19 | BufReader::with_capacity(n, r)  | Maxsus bufer hajmi                         | Особый размер буфера                                    |
// # |  20 | br.read_line(&mut s)            | Bitta satr o'qish                          | Чтение одной строки                                     |
// # |  21 | br.lines()                      | Satrlar iteratori                          | Итератор строк                                          |
// # |  22 | BufWriter::new(w)               | Write ni buferlashtirish                   | Буферизация Write                                       |
// # |  23 | Cursor<T>                       | Xotirada Read/Write/Seek                   | Read/Write/Seek в памяти                                |
// #================================================================================================================================================#