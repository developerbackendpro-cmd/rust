// #================================================================================================================================================#
// #                                                               STD::PATH                                                                        #
// #                    STD::PATH — FAYL TIZIMI YO'LLARI. PATH, PATHBUF, COMPONENTS, ANCESTORS. CROSS-PLATFORM. OsStr.                              #
// #                    STD::PATH — ПУТИ ФАЙЛОВОЙ СИСТЕМЫ. PATH, PATHBUF, COMPONENTS, ANCESTORS. CROSS-PLATFORM. OsStr.                             #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::path::{Path, PathBuf, Component, Prefix};
use std::ffi::OsStr;
use std::fmt;

// std::path nima:
// Что такое std::path:
//
//   Path    — immutable yo'l (&str ga o'xshash, borrowed)
//   Path    — неизменяемый путь (как &str, borrowed)
//   PathBuf — owned, o'zgaruvchan yo'l (String ga o'xshash)
//   PathBuf — владеющий, изменяемый путь (как String)
//
//   String  → PathBuf   (owned → owned)
//   &str    → &Path     (borrowed → borrowed)
//   PathBuf → &Path     (Deref)
//
//   Cross-platform:
//   Linux/macOS: separator = '/'
//   Windows:     separator = '\'
//   std::path har ikki platformada ishlaydi
//   std::path работает на обеих платформах

fn path_yaratish_misollari() {

    // PathBuf yaratish
    // Создание PathBuf
    let p1: PathBuf = PathBuf::new();
    let p2: PathBuf = PathBuf::from("/home/dilshod/projects");
    let p3: PathBuf = PathBuf::from("src/main.rs");
    let p4: PathBuf = ["home", "dilshod", "rust"].iter().collect();

    println!("{:?}", p1); // ""
    println!("{:?}", p2); // "/home/dilshod/projects"
    println!("{:?}", p3); // "src/main.rs"
    println!("{:?}", p4); // "home/dilshod/rust"
    // ""
    // "/home/dilshod/projects"
    // "src/main.rs"
    // "home/dilshod/rust"

    // Path yaratish — borrowed
    // Создание Path — borrowed
    let path: &Path = Path::new("/etc/config.toml");
    println!("{:?}", path); // "/etc/config.toml"
    // "/etc/config.toml"

    // &str → &Path
    let s: &str = "/usr/bin/rustc";
    let p: &Path = Path::new(s);
    println!("{}", p.display()); // /usr/bin/rustc
    // /usr/bin/rustc

    // PathBuf → &Path — Deref orqali
    // PathBuf → &Path — через Deref
    let pb: PathBuf = PathBuf::from("/home/user");
    let pr: &Path = &pb; // avtomatik Deref
    println!("{}", pr.display()); // /home/user
    // /home/user

    // PathBuf — from turli manbalardan
    // PathBuf — из разных источников
    let from_string: PathBuf = PathBuf::from(String::from("/tmp/file.txt"));
    let from_osstr: PathBuf = PathBuf::from(OsStr::new("/var/log"));
    println!("{}", from_string.display()); // /tmp/file.txt
    println!("{}", from_osstr.display());  // /var/log
    // /tmp/file.txt
    // /var/log
}

fn path_qismlar_misollari() {
    let path = Path::new("/home/dilshod/projects/rust/src/main.rs");

    // file_name() — fayl nomi (oxirgi komponent)
    // file_name() — имя файла (последний компонент)
    println!("{:?}", path.file_name()); // Some("main.rs")
    // Some("main.rs")

    // file_stem() — kengaytirgichsiz fayl nomi
    // file_stem() — имя файла без расширения
    println!("{:?}", path.file_stem()); // Some("main")
    // Some("main")

    // extension() — fayl kengaytmasi
    // extension() — расширение файла
    println!("{:?}", path.extension()); // Some("rs")
    // Some("rs")

    // parent() — ota katalog
    // parent() — родительский каталог
    println!("{:?}", path.parent()); // Some("/home/dilshod/projects/rust/src")
    // Some("/home/dilshod/projects/rust/src")

    // file_prefix() — birinchi nuqtacha oldidagi nom (Rust 1.87+)
    // file_prefix() — имя до первой точки (Rust 1.87+)
    // println!("{:?}", path.file_prefix()); // Some("main")

    // is_absolute() va is_relative()
    // is_absolute() и is_relative()
    println!("absolute: {}", path.is_absolute()); // true
    println!("relative: {}", path.is_relative()); // false
    // absolute: true
    // relative: false

    let rel = Path::new("src/main.rs");
    println!("rel absolute: {}", rel.is_absolute()); // false
    println!("rel relative: {}", rel.is_relative()); // true
    // rel absolute: false
    // rel relative: true

    // starts_with va ends_with
    // starts_with и ends_with
    println!("{}", path.starts_with("/home/dilshod")); // true
    println!("{}", path.ends_with("main.rs"));         // true
    println!("{}", path.ends_with("src/main.rs"));     // true
    // true
    // true
    // true

    // has_root() — ildiz bor (/)
    // has_root() — есть корень (/)
    println!("{}", path.has_root()); // true
    println!("{}", rel.has_root());  // false
    // true
    // false

    // to_str() — &str ga aylantirish
    // to_str() — преобразование в &str
    if let Some(s) = path.to_str() {
        println!("{}", s); // /home/dilshod/projects/rust/src/main.rs
    }
    // /home/dilshod/projects/rust/src/main.rs

    // to_string_lossy() — Unicode bo'lmasa ? bilan
    // to_string_lossy() — с ? если не Unicode
    println!("{}", path.to_string_lossy());
    // /home/dilshod/projects/rust/src/main.rs

    // display() — yozib chiqarish uchun
    // display() — для отображения
    println!("{}", path.display());
    // /home/dilshod/projects/rust/src/main.rs
}

fn pathbuf_ozgartirish_misollari() {

    // push() — komponent qo'shish
    // push() — добавление компонента
    let mut path = PathBuf::from("/home");
    path.push("dilshod");
    path.push("projects");
    path.push("rust");
    println!("{}", path.display()); // /home/dilshod/projects/rust
    // /home/dilshod/projects/rust

    // MUHIM: agar absolut yo'l push qilinsa — almashtiradi!
    // ВАЖНО: если добавить абсолютный путь — заменяет!
    let mut p2 = PathBuf::from("/home/user");
    p2.push("/etc"); // absolut — almashtirildi!
    println!("{}", p2.display()); // /etc
    // /etc

    // pop() — oxirgi komponentni olib tashlash
    // pop() — удаление последнего компонента
    let mut p3 = PathBuf::from("/home/dilshod/projects");
    p3.pop();
    println!("{}", p3.display()); // /home/dilshod
    p3.pop();
    println!("{}", p3.display()); // /home
    // /home/dilshod
    // /home

    // set_file_name() — fayl nomini o'zgartirish
    // set_file_name() — изменение имени файла
    let mut p4 = PathBuf::from("/home/user/old_file.txt");
    p4.set_file_name("new_file.txt");
    println!("{}", p4.display()); // /home/user/new_file.txt
    // /home/user/new_file.txt

    // set_extension() — kengaytmani o'zgartirish
    // set_extension() — изменение расширения
    let mut p5 = PathBuf::from("/home/user/script.py");
    p5.set_extension("rs");
    println!("{}", p5.display()); // /home/user/script.rs
    p5.set_extension(""); // kengaytmani olib tashlash
    println!("{}", p5.display()); // /home/user/script
    // /home/user/script.rs
    // /home/user/script

    // join() — yangi path yaratish
    // join() — создание нового пути
    let base = Path::new("/home/dilshod");
    let full = base.join("projects").join("rust").join("src");
    println!("{}", full.display()); // /home/dilshod/projects/rust/src
    // /home/dilshod/projects/rust/src

    // with_file_name() va with_extension() — yangi PathBuf
    // with_file_name() и with_extension() — новый PathBuf
    let original = Path::new("/home/user/config.toml");
    let backup = original.with_extension("toml.bak");
    let renamed = original.with_file_name("settings.toml");
    println!("{}", backup.display());  // /home/user/config.toml.bak
    println!("{}", renamed.display()); // /home/user/settings.toml
    // /home/user/config.toml.bak
    // /home/user/settings.toml
}

fn components_misollari() {

    let path = Path::new("/home/dilshod/projects/../rust/./src");

    // components() — yo'l qismlarini iteratsiya
    // components() — итерация по частям пути
    println!("Komponentlar:");
    for comp in path.components() {
        match comp {
            Component::RootDir     => println!("  RootDir: /"),
            Component::Normal(s)   => println!("  Normal: {:?}", s),
            Component::ParentDir   => println!("  ParentDir: .."),
            Component::CurDir      => println!("  CurDir: ."),
            Component::Prefix(p)   => println!("  Prefix: {:?}", p),
        }
    }
    // Komponentlar:
    //   RootDir: /
    //   Normal: "home"
    //   Normal: "dilshod"
    //   Normal: "projects"
    //   ParentDir: ..
    //   Normal: "rust"
    //   CurDir: .
    //   Normal: "src"

    // iter() — &OsStr iteratsiyasi
    // iter() — итерация &OsStr
    println!("\nIter:");
    for part in path.iter() {
        print!("{:?} ", part);
    }
    println!();
    // "/" "home" "dilshod" "projects" ".." "rust" "." "src"

    // ancestors() — barcha ota yo'llar
    // ancestors() — все родительские пути
    let p2 = Path::new("/home/dilshod/rust/src");
    println!("\nAncestors:");
    for ancestor in p2.ancestors() {
        println!("  {}", ancestor.display());
    }
    // Ancestors:
    //   /home/dilshod/rust/src
    //   /home/dilshod/rust
    //   /home/dilshod
    //   /home
    //   /

    // strip_prefix() — prefixni olib tashlash
    // strip_prefix() — удаление префикса
    let full_path = Path::new("/home/dilshod/projects/rust/src/main.rs");
    let base = Path::new("/home/dilshod");

    match full_path.strip_prefix(base) {
        Ok(rel)  => println!("Nisbiy yo'l: {}", rel.display()),
        Err(e)   => println!("Xato: {}", e),
    }
    // Nisbiy yo'l: projects/rust/src/main.rs
}

fn fayl_tizimi_tekshiruv() {

    // exists() — yo'l mavjudmi?
    // exists() — существует ли путь?
    let p1 = Path::new("/tmp");
    let p2 = Path::new("/mavjud_emas_katalog_xyz");

    println!("/tmp mavjud: {}", p1.exists());
    println!("/mavjud_emas: {}", p2.exists());
    // /tmp mavjud: true
    // /mavjud_emas: false

    // is_file() va is_dir()
    // is_file() и is_dir()
    println!("/tmp is_dir: {}", p1.is_dir());
    println!("/tmp is_file: {}", p1.is_file());
    // /tmp is_dir: true
    // /tmp is_file: false

    // is_symlink() — simvolik havolami?
    // is_symlink() — это символическая ссылка?
    println!("/tmp is_symlink: {}", p1.is_symlink());
    // /tmp is_symlink: false (odatda)

    // metadata() — fayl ma'lumotlari
    // metadata() — метаданные файла
    if let Ok(meta) = p1.metadata() {
        println!("/tmp kattaroq: {}", meta.is_dir());
        println!("/tmp o'lcham: {} bayt", meta.len());
    }

    // canonicalize() — absolut yo'l (symlink hal qilinadi)
    // canonicalize() — абсолютный путь (symlinks разрешаются)
    if let Ok(abs) = p1.canonicalize() {
        println!("/tmp canonical: {}", abs.display());
    }
    // /tmp canonical: /tmp
}

// Fayl yo'llarini qayta ishlash
// Обработка путей к файлам
fn yo_l_yordamchisi() {

    // Kengaytma bo'yicha filtrlash simulyatsiyasi
    // Симуляция фильтрации по расширению
    fn kengaytma_tekshir(path: &Path, kengaytma: &str) -> bool {
        path.extension()
            .and_then(|e| e.to_str())
            .map(|e| e == kengaytma)
            .unwrap_or(false)
    }

    let fayllar: Vec<&str> = vec![
        "main.rs", "lib.rs", "config.toml", "README.md",
        "build.py", "test.rs", "data.json", "Cargo.toml",
    ];

    let rust_fayllar: Vec<&str> = fayllar.iter()
        .filter(|&&f| kengaytma_tekshir(Path::new(f), "rs"))
        .copied()
        .collect();

    let toml_fayllar: Vec<&str> = fayllar.iter()
        .filter(|&&f| kengaytma_tekshir(Path::new(f), "toml"))
        .copied()
        .collect();

    println!("Rust fayllar: {:?}", rust_fayllar);
    println!("TOML fayllar: {:?}", toml_fayllar);
    // Rust fayllar: ["main.rs", "lib.rs", "test.rs"]
    // TOML fayllar: ["config.toml", "Cargo.toml"]

    // Yo'llarni birlashtirish
    // Объединение путей
    fn loyiha_yo_li(ildiz: &Path, qism: &str) -> PathBuf {
        ildiz.join(qism)
    }

    let ildiz = PathBuf::from("/home/dilshod/projects/rust");
    println!("{}", loyiha_yo_li(&ildiz, "src/main.rs").display());
    println!("{}", loyiha_yo_li(&ildiz, "Cargo.toml").display());
    // /home/dilshod/projects/rust/src/main.rs
    // /home/dilshod/projects/rust/Cargo.toml

    // Nisbiy va absolut yo'l o'tkazish
    // Преобразование относительного и абсолютного пути
    fn nisbiy_qil(full: &Path, base: &Path) -> Option<PathBuf> {
        full.strip_prefix(base).ok().map(|p| p.to_path_buf())
    }

    let full = Path::new("/home/dilshod/projects/rust/src/lib.rs");
    let base = Path::new("/home/dilshod/projects");
    println!("{:?}", nisbiy_qil(full, base));
    // Some("rust/src/lib.rs")
}

// Konfiguratsiya yo'l menejeri
// Менеджер путей конфигурации
struct KonfigYollar {
    ildiz: PathBuf,
}

impl KonfigYollar {
    fn new(ildiz: impl Into<PathBuf>) -> Self {
        KonfigYollar { ildiz: ildiz.into() }
    }

    fn konfig_fayli(&self) -> PathBuf {
        self.ildiz.join("config.toml")
    }

    fn log_katalogi(&self) -> PathBuf {
        self.ildiz.join("logs")
    }

    fn ma_lumotlar_bazasi(&self) -> PathBuf {
        self.ildiz.join("data").join("app.db")
    }

    fn kesh_katalogi(&self) -> PathBuf {
        self.ildiz.join(".cache")
    }

    fn barcha_yo_llar(&self) -> Vec<(&str, PathBuf)> {
        vec![
            ("Konfig",    self.konfig_fayli()),
            ("Loglar",    self.log_katalogi()),
            ("Ma'lumot",  self.ma_lumotlar_bazasi()),
            ("Kesh",      self.kesh_katalogi()),
        ]
    }
}

fn konfig_yo_llar_misoli() {

    let km = KonfigYollar::new("/home/dilshod/.myapp");

    for (nom, yo_l) in km.barcha_yo_llar() {
        println!("{:<12}: {}", nom, yo_l.display());
    }
    // Konfig     : /home/dilshod/.myapp/config.toml
    // Loglar     : /home/dilshod/.myapp/logs
    // Ma'lumot   : /home/dilshod/.myapp/data/app.db
    // Kesh       : /home/dilshod/.myapp/.cache

    // Yo'l validatsiyasi
    // Валидация пути
    fn yo_l_xavfsizmi(path: &Path) -> bool {
        // Path traversal hujumidan himoya
        // Защита от атаки Path traversal
        !path.components().any(|c| c == Component::ParentDir)
    }

    let xavfsiz   = Path::new("docs/guide.html");
    let xavfsiz2  = Path::new("images/logo.png");
    let xavfli    = Path::new("../../etc/passwd"); // path traversal!
    let xavfli2   = Path::new("docs/../../../secret");

    println!("\nYo'l xavfsizligi:");
    println!("docs/guide.html:       {}", yo_l_xavfsizmi(xavfsiz));
    println!("images/logo.png:       {}", yo_l_xavfsizmi(xavfsiz2));
    println!("../../etc/passwd:      {}", yo_l_xavfsizmi(xavfli));
    println!("docs/../../../secret:  {}", yo_l_xavfsizmi(xavfli2));
    // docs/guide.html:       true
    // images/logo.png:       true
    // ../../etc/passwd:      false
    // docs/../../../secret:  false
}

// Yo'llarni normalizatsiya qilish
// Нормализация путей
fn normalizatsiya_misoli() {

    // . va .. ni hal qilish (std da canonicalize() file system kerak)
    // Разрешение . и .. (canonicalize() требует file system в std)
    // O'zimiz normalizatsiya qilamiz
    // Нормализуем сами
    fn normalize(path: &Path) -> PathBuf {
        let mut komponentlar: Vec<Component> = Vec::new();

        for comp in path.components() {
            match comp {
                Component::CurDir   => {} // . — o'tkazib yuborish
                Component::ParentDir => {
                    // .. — oldingi komponentni olib tashlash
                    if matches!(komponentlar.last(), Some(Component::Normal(_))) {
                        komponentlar.pop();
                    } else {
                        komponentlar.push(comp);
                    }
                }
                c => komponentlar.push(c),
            }
        }

        komponentlar.iter().collect()
    }

    let p1 = Path::new("/home/dilshod/./projects/../rust/src");
    let p2 = Path::new("./src/../lib/./utils");
    let p3 = Path::new("/home/user/docs/../../admin");

    println!("Normalize misollari:");
    println!("{} → {}", p1.display(), normalize(p1).display());
    println!("{} → {}", p2.display(), normalize(p2).display());
    println!("{} → {}", p3.display(), normalize(p3).display());
    // /home/dilshod/./projects/../rust/src → /home/dilshod/rust/src
    // ./src/../lib/./utils → lib/utils
    // /home/user/docs/../../admin → /admin
}

fn main() {

    println!("=== PATH YARATISH ===");
    path_yaratish_misollari();

    println!("\n=== PATH QISMLAR ===");
    path_qismlar_misollari();

    println!("\n=== PATHBUF O'ZGARTIRISH ===");
    pathbuf_ozgartirish_misollari();

    println!("\n=== COMPONENTS ===");
    components_misollari();

    println!("\n=== FAYL TIZIMI TEKSHIRUV ===");
    fayl_tizimi_tekshiruv();

    println!("\n=== YO'L YORDAMCHISI ===");
    yo_l_yordamchisi();

    println!("\n=== KONFIG YO'LLAR ===");
    konfig_yo_llar_misoli();

    println!("\n=== NORMALIZATSIYA ===");
    normalizatsiya_misoli();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                               | Описание (RU)                                            |
// #================================================================================================================================================#
// # |                                        PATH VA PATHBUF                                                                                       |
// #================================================================================================================================================#
// # |   1 | Path::new(s)                    | Borrowed yo'l yaratish                    | Создание заимствованного пути                            |
// # |   2 | PathBuf::from(s)                | Owned yo'l yaratish                       | Создание владеющего пути                                 |
// # |   3 | PathBuf — Deref → &Path         | PathBuf → &Path avtomatik                 | PathBuf → &Path автоматически                            |
// # |   4 | path.display()                  | Yozib chiqarish uchun                     | Для отображения                                          |
// # |   5 | path.to_str()                   | &OsStr → Option<&str>                     | &OsStr → Option<&str>                                    |
// # |   6 | path.to_string_lossy()          | Unicode bo'lmasa ? bilan                  | С ? если не Unicode                                      |
// #================================================================================================================================================#
// # |                                        PATH QISMLARI                                                                                         |
// #================================================================================================================================================#
// # |   7 | path.file_name()                | Fayl nomi (oxirgi komponent)              | Имя файла (последний компонент)                          |
// # |   8 | path.file_stem()                | Kengaytirgichsiz nom                      | Имя без расширения                                       |
// # |   9 | path.extension()                | Kengaytma                                 | Расширение                                               |
// # |  10 | path.parent()                   | Ota katalog                               | Родительский каталог                                     |
// # |  11 | path.is_absolute()              | Absolut yo'lmi?                           | Абсолютный ли путь?                                      |
// # |  12 | path.starts_with(p)             | p bilan boshlanadimi?                     | Начинается ли с p?                                       |
// # |  13 | path.ends_with(p)               | p bilan tugaydimi?                        | Заканчивается ли на p?                                   |
// # |  14 | path.has_root()                 | Ildiz bor (/)                             | Есть корень (/)                                          |
// #================================================================================================================================================#
// # |                                        PATHBUF O'ZGARTIRISH                                                                                  |
// #================================================================================================================================================#
// # |  15 | pb.push(p)                      | Komponent qo'shish                        | Добавление компонента                                    |
// # |  16 | pb.pop()                        | Oxirgi komponentni olib tashlash          | Удаление последнего компонента                           |
// # |  17 | pb.set_file_name(n)             | Fayl nomini o'zgartirish                  | Изменение имени файла                                    |
// # |  18 | pb.set_extension(e)             | Kengaytmani o'zgartirish                  | Изменение расширения                                     |
// # |  19 | path.join(p)                    | Yangi PathBuf yaratish                    | Создание нового PathBuf                                  |
// # |  20 | path.with_extension(e)          | Kengaytma bilan yangi                     | Новый с расширением                                      |
// # |  21 | path.strip_prefix(base)         | Prefiksni olib tashlash                   | Удаление префикса                                        |
// # |  22 | path.components()               | Yo'l qismlarini iteratsiya                | Итерация по частям пути                                  |
// # |  23 | path.ancestors()                | Barcha ota yo'llar                        | Все родительские пути                                    |
// #================================================================================================================================================#