// #================================================================================================================================================#
// #                                                            STD::ENV + STD::PROCESS                                                             #
// #                    STD::ENV — MUHIT O'ZGARUVCHILARI. STD::PROCESS — JARAYON BOSHQARUVI. ARGS, EXIT, COMMAND.                                   #
// #                    STD::ENV — ПЕРЕМЕННЫЕ СРЕДЫ. STD::PROCESS — УПРАВЛЕНИЕ ПРОЦЕССОМ. ARGS, EXIT, COMMAND.                                      #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::env;
use std::process::{self, Command, ExitCode, ExitStatus, Stdio};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fmt;

// std::env nima:
// Что такое std::env:
//
//   args()           — buyruq qatori argumentlari
//   args_os()        — OsString formatida argumentlar
//   var("KEY")       — muhit o'zgaruvchisi qiymati
//   var_os("KEY")    — OsString formatida
//   vars()           — barcha muhit o'zgaruvchilari
//   set_var("K","V") — o'rnatish (UNSAFE! thread-safe emas!)
//   remove_var("K")  — o'chirish (UNSAFE!)
//   current_dir()    — joriy katalog
//   set_current_dir()— katalogni o'zgartirish
//   home_dir()       — foydalanuvchi home katalogi (deprecated)
//   temp_dir()       — vaqtinchalik katalog
//   current_exe()    — joriy bajariladigan fayl
//
// std::process nima:
// Что такое std::process:
//
//   exit(code)       — dasturni to'xtatish
//   abort()          — birdaniga to'xtatish (signal)
//   id()             — joriy jarayon IDsi
//   Command::new()   — tashqi buyruq ishlatish

fn args_misollari() {

    // args() — argumentlar iteratori
    // args() — итератор аргументов
    let barcha_args: Vec<String> = env::args().collect();

    println!("Argumentlar soni: {}", barcha_args.len());
    for (i, arg) in barcha_args.iter().enumerate() {
        println!("  args[{}]: {}", i, arg);
    }
    // Argumentlar soni: 1 (odatda faqat dastur nomi)
    // args[0]: ./target/debug/hello_rust

    // args().skip(1) — dastur nomini o'tkazib yuborish
    // args().skip(1) — пропуск имени программы
    let faqat_args: Vec<String> = env::args().skip(1).collect();
    println!("Foydalanuvchi argumentlari: {:?}", faqat_args);

    // Argument parsing simulyatsiyasi
    // Симуляция парсинга аргументов
    fn argumentlarni_parse(args: Vec<String>) -> HashMap<String, String> {
        let mut xarita = HashMap::new();
        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];
            if arg.starts_with("--") {
                let kalit = arg.trim_start_matches('-').to_string();
                if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                    xarita.insert(kalit, args[i + 1].clone());
                    i += 2;
                } else {
                    xarita.insert(kalit, "true".to_string());
                    i += 1;
                }
            } else if arg.starts_with('-') {
                let kalit = arg.trim_start_matches('-').to_string();
                xarita.insert(kalit, "true".to_string());
                i += 1;
            } else {
                xarita.insert(format!("arg_{}", i), arg.clone());
                i += 1;
            }
        }
        xarita
    }

    let test_args = vec![
        "--host".to_string(), "localhost".to_string(),
        "--port".to_string(), "8080".to_string(),
        "--debug".to_string(),
        "-v".to_string(),
    ];

    let parsed = argumentlarni_parse(test_args);
    println!("host: {:?}", parsed.get("host"));   // Some("localhost")
    println!("port: {:?}", parsed.get("port"));   // Some("8080")
    println!("debug: {:?}", parsed.get("debug")); // Some("true")
    println!("v: {:?}", parsed.get("v"));         // Some("true")
    // host: Some("localhost")
    // port: Some("8080")
    // debug: Some("true")
    // v: Some("true")
}

fn env_var_misollari() {

    // var("KEY") — qiymat olish
    // var("KEY") — получение значения
    match env::var("PATH") {
        Ok(v)  => println!("PATH (qisqartirilgan): {}...", &v[..50.min(v.len())]),
        Err(e) => println!("PATH topilmadi: {}", e),
    }
    // PATH (qisqartirilgan): /usr/local/sbin:/usr/local/bin:...

    // var_os() — OsString formatida (Unicode bo'lmasa ham)
    // var_os() — в формате OsString (даже если не Unicode)
    if let Some(path) = env::var_os("HOME") {
        println!("HOME: {:?}", path);
    }
    // HOME: "/home/username"

    // Mavjud emas — VarError
    // Не существует — VarError
    match env::var("MAVJUD_EMAS_XYZ_123") {
        Ok(v)  => println!("Topildi: {}", v),
        Err(env::VarError::NotPresent)   => println!("Mavjud emas"),
        Err(env::VarError::NotUnicode(s)) => println!("Unicode emas: {:?}", s),
    }
    // Mavjud emas

    // unwrap_or — standart qiymat bilan
    // unwrap_or — со значением по умолчанию
    let host = env::var("APP_HOST").unwrap_or_else(|_| "localhost".to_string());
    let port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    println!("host: {}, port: {}", host, port);
    // host: localhost, port: 8080

    // vars() — barcha muhit o'zgaruvchilari
    // vars() — все переменные среды
    let mut sistemaviy_vars: Vec<(String, String)> = env::vars()
        .filter(|(k, _)| k.starts_with("RUST") || k == "HOME" || k == "USER")
        .collect();
    sistemaviy_vars.sort_by_key(|(k, _)| k.clone());
    println!("\nRust va tizim o'zgaruvchilari:");
    for (k, v) in &sistemaviy_vars {
        println!("  {}={}", k, if v.len() > 50 { &v[..50] } else { v });
    }

    // set_var va remove_var — thread-safe EMAS!
    // set_var и remove_var — НЕ потокобезопасны!
    // Faqat single-thread, test da ishlatish
    // Использовать только в single-thread, тестах
    unsafe {
        env::set_var("MENING_O_ZGARUVCHIM", "Rust_2024");
        println!("\nO'rnatildi: {:?}", env::var("MENING_O_ZGARUVCHIM"));
        env::remove_var("MENING_O_ZGARUVCHIM");
        println!("O'chirildi: {:?}", env::var("MENING_O_ZGARUVCHIM"));
    }
    // O'rnatildi: Ok("Rust_2024")
    // O'chirildi: Err(NotPresent)
}

fn env_katalog_misollari() {

    // current_dir() — joriy ishchi katalog
    // current_dir() — текущий рабочий каталог
    match env::current_dir() {
        Ok(dir) => println!("Joriy katalog: {}", dir.display()),
        Err(e)  => println!("Xato: {}", e),
    }
    // Joriy katalog: /home/user/hello_rust

    // current_exe() — bajariladigan fayl yo'li
    // current_exe() — путь к исполняемому файлу
    match env::current_exe() {
        Ok(exe) => println!("Bajariladigan: {}", exe.display()),
        Err(e)  => println!("Xato: {}", e),
    }
    // Bajariladigan: /home/user/hello_rust/target/debug/hello_rust

    // temp_dir() — vaqtinchalik katalog
    // temp_dir() — временный каталог
    let tmp = env::temp_dir();
    println!("Temp katalog: {}", tmp.display());
    // Temp katalog: /tmp

    // Yo'llarni birlashtirish
    // Объединение путей
    let konfig_yo_l = tmp.join("myapp_config.toml");
    println!("Konfig: {}", konfig_yo_l.display());
    // Konfig: /tmp/myapp_config.toml

    // join_paths va split_paths
    // join_paths и split_paths
    let path_var = env::var("PATH").unwrap_or_default();
    let paths: Vec<PathBuf> = env::split_paths(&path_var).collect();
    println!("\nPATH da {} katalog bor", paths.len());
    for p in paths.iter().take(3) {
        println!("  {}", p.display());
    }
    println!("  ...");

    // join_paths — yo'llarni birlashtirish
    // join_paths — объединение путей
    let qo_shimcha_yo_l = vec![
        PathBuf::from("/usr/local/bin"),
        PathBuf::from("/home/user/.local/bin"),
    ];
    if let Ok(yangi_path) = env::join_paths(qo_shimcha_yo_l) {
        println!("Yangi PATH qo'shimcha: {:?}", yangi_path);
    }
}

fn process_asosiy_misollari() {

    // process::id() — joriy PID
    // process::id() — текущий PID
    println!("Joriy PID: {}", process::id());
    // Joriy PID: XXXX

    // ExitCode — chiqish kodi
    // ExitCode — код выхода
    println!("ExitCode::SUCCESS: {:?}", ExitCode::SUCCESS);
    println!("ExitCode::FAILURE: {:?}", ExitCode::FAILURE);
    // ExitCode::SUCCESS: 0
    // ExitCode::FAILURE: 1

    // process::exit() — dasturni to'xtatish
    // process::exit() — завершение программы
    // process::exit(0);  // muvaffaqiyat
    // process::exit(1);  // xato
    // Biz bu yerda chaqirmaymiz — dastur to'xtab qoladi!
    println!("process::exit() — Drop ham chaqirilmaydi!");

    // process::abort() — birdaniga to'xtatish (signal 6)
    // process::abort() — немедленное завершение (сигнал 6)
    // process::abort(); // core dump yaratadi
    println!("process::abort() — hech qanday tozalanish yo'q");

    // ExitCode — main() dan qaytarish
    // ExitCode — возврат из main()
    // fn main() -> ExitCode {
    //     if xato_bor() { return ExitCode::FAILURE; }
    //     ExitCode::SUCCESS
    // }
}

fn command_misollari() {

    // Command::new — tashqi buyruq yaratish
    // Command::new — создание внешней команды

    // 1. Oddiy buyruq — echo
    // 1. Простая команда — echo
    let chiqish = Command::new("echo")
        .arg("Salom Rust!")
        .output()
        .expect("echo bajarilib bo'lmadi");

    println!("stdout: {}", String::from_utf8_lossy(&chiqish.stdout));
    println!("status: {}", chiqish.status);
    // stdout: Salom Rust!
    // status: exit status: 0

    // 2. Ko'p argumentli — ls
    // 2. С несколькими аргументами — ls
    let chiqish2 = Command::new("ls")
        .args(["-la", "/tmp"])
        .output();

    match chiqish2 {
        Ok(out) => {
            let matn = String::from_utf8_lossy(&out.stdout);
            let qatorlar: Vec<&str> = matn.lines().take(3).collect();
            for q in qatorlar { println!("{}", q); }
        }
        Err(e) => println!("Xato: {}", e),
    }
    // total XXX
    // drwxrwxrwt XX root root ...
    // drwxr-xr-x XX root root ...

    // 3. Muhit o'zgaruvchisi bilan
    // 3. С переменной среды
    let chiqish3 = Command::new("sh")
        .arg("-c")
        .arg("echo $MENING_VAR")
        .env("MENING_VAR", "Rust_Ajoyib")
        .output()
        .unwrap();
    println!("env: {}", String::from_utf8_lossy(&chiqish3.stdout).trim());
    // env: Rust_Ajoyib

    // 4. status() — faqat chiqish kodi
    // 4. status() — только код выхода
    let status = Command::new("true").status().unwrap();
    println!("true status: {}", status.success()); // true

    let status2 = Command::new("false").status().unwrap();
    println!("false status: {}", status2.success()); // false
    // true status: true
    // false status: false

    // 5. stdin ga yozish
    // 5. Запись в stdin
    use std::io::Write;
    let mut proc = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(mut stdin) = proc.stdin.take() {
        stdin.write_all(b"Salom stdin!\n").unwrap();
    }

    let out = proc.wait_with_output().unwrap();
    println!("cat: {}", String::from_utf8_lossy(&out.stdout).trim());
    // cat: Salom stdin!

    // 6. stderr alohida
    // 6. stderr отдельно
    let chiqish4 = Command::new("sh")
        .arg("-c")
        .arg("echo stdout; echo stderr >&2")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();

    println!("stdout: {}", String::from_utf8_lossy(&chiqish4.stdout).trim());
    println!("stderr: {}", String::from_utf8_lossy(&chiqish4.stderr).trim());
    // stdout: stdout
    // stderr: stderr
}

// Konfiguratsiya — muhit o'zgaruvchilari bilan
// Конфигурация — с переменными среды
#[derive(Debug)]
struct AppKonfig {
    host: String,
    port: u16,
    debug: bool,
    log_daraja: String,
    ma_lumotlar_bazasi_url: String,
    max_ulanishlar: u32,
}

impl AppKonfig {
    fn muhitdan_yukla() -> Self {
        AppKonfig {
            host: env::var("APP_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("APP_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            debug: env::var("APP_DEBUG")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(false),
            log_daraja: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
            ma_lumotlar_bazasi_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://./app.db".to_string()),
            max_ulanishlar: env::var("MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .unwrap_or(10),
        }
    }

    fn tekshir(&self) -> Result<(), Vec<String>> {
        let mut xatolar = Vec::new();
        if self.port == 0 { xatolar.push("Port 0 bo'lishi mumkin emas".to_string()); }
        if self.host.is_empty() { xatolar.push("Host bo'sh".to_string()); }
        if self.max_ulanishlar == 0 { xatolar.push("Max ulanishlar 0".to_string()); }
        if xatolar.is_empty() { Ok(()) } else { Err(xatolar) }
    }
}

// Buyruq ishga tushiruvchi — xavfsiz wrapper
// Безопасный обёртчик запуска команд
struct BuyruqIshga {
    dastur: String,
    argumentlar: Vec<String>,
    muhit_o_zgaruvchilari: HashMap<String, String>,
    ishchi_katalog: Option<PathBuf>,
}

impl BuyruqIshga {
    fn new(dastur: &str) -> Self {
        BuyruqIshga {
            dastur: dastur.to_string(),
            argumentlar: Vec::new(),
            muhit_o_zgaruvchilari: HashMap::new(),
            ishchi_katalog: None,
        }
    }

    fn arg(mut self, a: &str) -> Self {
        self.argumentlar.push(a.to_string());
        self
    }

    fn env(mut self, k: &str, v: &str) -> Self {
        self.muhit_o_zgaruvchilari.insert(k.to_string(), v.to_string());
        self
    }

    fn katalog(mut self, d: impl Into<PathBuf>) -> Self {
        self.ishchi_katalog = Some(d.into());
        self
    }

    fn bajar(self) -> Result<String, String> {
        let mut cmd = Command::new(&self.dastur);
        cmd.args(&self.argumentlar);
        for (k, v) in &self.muhit_o_zgaruvchilari {
            cmd.env(k, v);
        }
        if let Some(dir) = &self.ishchi_katalog {
            cmd.current_dir(dir);
        }

        match cmd.output() {
            Ok(out) => {
                if out.status.success() {
                    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
                } else {
                    Err(String::from_utf8_lossy(&out.stderr).trim().to_string())
                }
            }
            Err(e) => Err(format!("Buyruq bajarilib bo'lmadi: {}", e)),
        }
    }
}

fn real_hayot_misollari() {

    println!("--- AppKonfig ---");
    let konfig = AppKonfig::muhitdan_yukla();
    println!("{:#?}", konfig);

    match konfig.tekshir() {
        Ok(())   => println!("Konfig to'g'ri ✅"),
        Err(xatolar) => {
            for x in xatolar { println!("❌ {}", x); }
        }
    }
    // AppKonfig {
    //     host: "0.0.0.0",
    //     port: 8080,
    //     debug: false,
    //     ...
    // }
    // Konfig to'g'ri ✅

    println!("\n--- BuyruqIshga ---");
    let natija = BuyruqIshga::new("echo")
        .arg("Rust")
        .arg("ajoyib!")
        .bajar();
    println!("Natija: {:?}", natija);
    // Natija: Ok("Rust ajoyib!")

    let git_natija = BuyruqIshga::new("git")
        .arg("--version")
        .bajar();
    println!("Git: {:?}", git_natija);
    // Git: Ok("git version X.X.X")

    println!("\n--- Tizim ma'lumotlari ---");
    let uname = BuyruqIshga::new("uname")
        .arg("-s")
        .bajar();
    let hostname = BuyruqIshga::new("hostname")
        .bajar();

    println!("OS: {:?}", uname);
    println!("Hostname: {:?}", hostname);
    println!("PID: {}", process::id());
    // OS: Ok("Linux")
    // Hostname: Ok("username-pc")
    // PID: XXXX
}

fn main() {

    println!("=== ARGS ===");
    args_misollari();

    println!("\n=== ENV VAR ===");
    env_var_misollari();

    println!("\n=== KATALOG VA YO'LLAR ===");
    env_katalog_misollari();

    println!("\n=== PROCESS ASOSIY ===");
    process_asosiy_misollari();

    println!("\n=== COMMAND ===");
    command_misollari();

    println!("\n=== REAL HAYOT ===");
    real_hayot_misollari();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        STD::ENV                                                                                              |
// #================================================================================================================================================#
// # |   1 | env::args()                     | Buyruq qatori argumentlari                 | Аргументы командной строки                              |
// # |   2 | env::var("KEY")                 | Muhit o'zgaruvchisi qiymati                | Значение переменной среды                               |
// # |   3 | env::var_os("KEY")              | OsString formatida                         | В формате OsString                                      |
// # |   4 | env::vars()                     | Barcha muhit o'zgaruvchilari               | Все переменные среды                                    |
// # |   5 | env::set_var("K","V")           | O'rnatish (UNSAFE! thread-safe emas)       | Установка (UNSAFE! не потокобезопасно)                  |
// # |   6 | env::remove_var("K")            | O'chirish (UNSAFE!)                        | Удаление (UNSAFE!)                                      |
// # |   7 | env::current_dir()              | Joriy ishchi katalog                       | Текущий рабочий каталог                                 |
// # |   8 | env::current_exe()              | Bajariladigan fayl yo'li                   | Путь к исполняемому файлу                               |
// # |   9 | env::temp_dir()                 | Vaqtinchalik katalog                       | Временный каталог                                       |
// # |  10 | env::split_paths(&s)            | PATH o'zgaruvchisini ajratish              | Разбивка переменной PATH                                |
// # |  11 | env::join_paths(paths)          | Yo'llarni PATH formatiga                   | Пути в формат PATH                                      |
// #================================================================================================================================================#
// # |                                        STD::PROCESS                                                                                          |
// #================================================================================================================================================#
// # |  12 | process::exit(code)             | Dasturni to'xtatish (Drop chaqiriladi)     | Завершение (Drop вызывается)                            |
// # |  13 | process::abort()                | Birdaniga to'xtatish (Drop chaqirilmaydi)  | Немедленное завершение (Drop не вызывается)             |
// # |  14 | process::id()                   | Joriy jarayon IDsi (PID)                   | ID текущего процесса (PID)                              |
// # |  15 | ExitCode::SUCCESS/FAILURE       | Chiqish kodlari                            | Коды выхода                                             |
// #================================================================================================================================================#
// # |                                        COMMAND                                                                                               |
// #================================================================================================================================================#
// # |  16 | Command::new("cmd")             | Tashqi buyruq yaratish                     | Создание внешней команды                                |
// # |  17 | .arg("x") / .args([...])        | Argumentlar qo'shish                       | Добавление аргументов                                   |
// # |  18 | .env("K","V")                   | Muhit o'zgaruvchisi                        | Переменная среды                                        |
// # |  19 | .stdin/.stdout/.stderr(Stdio::) | I/O yo'naltirish                           | Перенаправление I/O                                     |
// # |  20 | .output()                       | stdout+stderr+status olish                 | Получить stdout+stderr+status                           |
// # |  21 | .status()                       | Faqat chiqish kodi                         | Только код выхода                                       |
// # |  22 | .spawn()                        | Asinxron ishga tushirish                   | Асинхронный запуск                                      |
// # |  23 | .current_dir(path)              | Ishchi katalog o'rnatish                   | Установка рабочего каталога                             |
// #================================================================================================================================================#