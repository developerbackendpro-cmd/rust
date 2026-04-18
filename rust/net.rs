// #================================================================================================================================================#
// #                                                                    STD::NET                                                                    #
// #                        STD::NET — TCP, UDP, IP MANZIL, SOCKET. SERVER VA CLIENT. NON-BLOCKING. THREAD-PER-CONNECTION.                          #
// #                        STD::NET — TCP, UDP, IP АДРЕС, SOCKET. СЕРВЕР И КЛИЕНТ. NON-BLOCKING. THREAD-PER-CONNECTION.                            #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::net::{
    TcpListener, TcpStream,
    UdpSocket,
    IpAddr, Ipv4Addr, Ipv6Addr,
    SocketAddr, SocketAddrV4, SocketAddrV6,
    ToSocketAddrs,
};
use std::io::{Read, Write, BufRead, BufReader, BufWriter};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// std::net nima:
// Что такое std::net:
//
//   TcpListener  — TCP server (bog'lanishlarni qabul qilish)
//   TcpListener  — TCP сервер (принятие соединений)
//   TcpStream    — TCP ulanish (o'qish/yozish)
//   TcpStream    — TCP соединение (чтение/запись)
//   UdpSocket    — UDP soket (connectionless)
//   UdpSocket    — UDP сокет (без соединения)
//   IpAddr       — IP manzil (v4 yoki v6)
//   IpAddr       — IP адрес (v4 или v6)
//   SocketAddr   — IP + port
//   SocketAddr   — IP + порт

fn ip_addr_misollari() {

    // Ipv4Addr — IPv4 manzil
    // Ipv4Addr — адрес IPv4
    let v4: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    println!("{}", v4);            // 127.0.0.1
    println!("{}", v4.is_loopback());   // true
    println!("{}", v4.is_private());    // false
    println!("{}", v4.is_unspecified());// false

    let v4_2: Ipv4Addr = "192.168.1.1".parse().unwrap();
    println!("{}", v4_2.is_private());  // true
    // 127.0.0.1
    // true
    // false
    // false
    // true

    // Ipv6Addr — IPv6 manzil
    // Ipv6Addr — адрес IPv6
    let v6: Ipv6Addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    println!("{}", v6);            // ::1
    println!("{}", v6.is_loopback());   // true
    // ::1
    // true

    // IpAddr — v4 yoki v6
    // IpAddr — v4 или v6
    let ip1: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let ip2: IpAddr = IpAddr::V6(Ipv6Addr::LOCALHOST);
    println!("{} {}", ip1, ip2);
    println!("{}", ip1.is_loopback());
    println!("{}", ip1.is_ipv4());
    println!("{}", ip2.is_ipv6());
    // 127.0.0.1 ::1
    // true
    // true
    // true

    // Parse
    let ip3: IpAddr = "10.0.0.1".parse().unwrap();
    let ip4: IpAddr = "::1".parse().unwrap();
    println!("{} {}", ip3, ip4);
    // 10.0.0.1 ::1

    // SocketAddr — IP + port
    // SocketAddr — IP + порт
    let sa: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("{}", sa);            // 127.0.0.1:8080
    println!("{}", sa.ip());       // 127.0.0.1
    println!("{}", sa.port());     // 8080
    println!("{}", sa.is_ipv4()); // true

    let sa2: SocketAddrV4 = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 3000);
    println!("{}", sa2);           // 127.0.0.1:3000

    // ToSocketAddrs — turli manzil formatlarini qabul qilish
    // ToSocketAddrs — принятие различных форматов адресов
    fn birinchi_addr(manzil: impl ToSocketAddrs) -> Option<SocketAddr> {
        manzil.to_socket_addrs().ok()?.next()
    }

    println!("{:?}", birinchi_addr("127.0.0.1:9090"));
    println!("{:?}", birinchi_addr(("127.0.0.1", 9090u16)));
    // Some(127.0.0.1:9090)
    // Some(127.0.0.1:9090)
}

fn tcp_echo_server_misoli() {

    println!("--- TCP Echo Server ---");

    // Server: bog'lanishlarni qabul qilish
    // Server: принятие соединений
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    println!("Server: {}", addr);

    // Server thread
    let server = thread::spawn(move || {
        // accept() — yangi ulanishni kutish (blocking)
        // accept() — ожидание нового соединения (blocking)
        if let Ok((mut stream, client_addr)) = listener.accept() {
            println!("[Server] Ulanish: {}", client_addr);

            let mut buf = [0u8; 1024];
            loop {
                match stream.read(&mut buf) {
                    Ok(0)    => { println!("[Server] Client uzildi"); break; }
                    Ok(n)    => {
                        let xabar = String::from_utf8_lossy(&buf[..n]);
                        println!("[Server] Qabul: '{}'", xabar);
                        // Echo — xuddi shu xabarni qaytarish
                        // Echo — вернуть то же сообщение
                        stream.write_all(&buf[..n]).unwrap();
                    }
                    Err(e)   => { println!("[Server] Xato: {}", e); break; }
                }
            }
        }
    });

    thread::sleep(Duration::from_millis(10));

    // Client
    let mut client = TcpStream::connect(addr).unwrap();
    println!("[Client] Ulandi: {}", addr);

    let xabarlar = ["Salom!", "Rust tili", "Tarmoq dasturlash"];
    for xabar in &xabarlar {
        client.write_all(xabar.as_bytes()).unwrap();
        let mut buf = [0u8; 1024];
        let n = client.read(&mut buf).unwrap();
        println!("[Client] Echo: '{}'", String::from_utf8_lossy(&buf[..n]));
    }
    drop(client); // Ulanishni yopish
    server.join().unwrap();
    // [Server] Ulanish: 127.0.0.1:XXXXX
    // [Server] Qabul: 'Salom!'
    // [Client] Echo: 'Salom!'
    // ...
}

fn tcp_buffered_misoli() {

    println!("--- TCP Buffered I/O ---");

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = thread::spawn(move || {
        if let Ok((stream, _)) = listener.accept() {
            // BufReader — satrlar bo'yicha o'qish
            // BufReader — чтение по строкам
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut writer = BufWriter::new(stream);

            let mut satr = String::new();
            loop {
                satr.clear();
                match reader.read_line(&mut satr) {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        let javob = format!("ECHO: {}", satr.trim_end());
                        println!("[Server] '{}'", javob);
                        writeln!(writer, "{}", javob).unwrap();
                        writer.flush().unwrap();
                    }
                }
            }
        }
    });

    thread::sleep(Duration::from_millis(10));

    let stream = TcpStream::connect(addr).unwrap();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream);

    let satrlar = ["birinchi satr", "ikkinchi satr", "uchinchi satr"];
    for satr in &satrlar {
        writeln!(writer, "{}", satr).unwrap();
        writer.flush().unwrap();

        let mut javob = String::new();
        reader.read_line(&mut javob).unwrap();
        println!("[Client] '{}'", javob.trim_end());
    }
    server.join().unwrap();
    // [Client] 'ECHO: birinchi satr'
    // [Client] 'ECHO: ikkinchi satr'
    // [Client] 'ECHO: uchinchi satr'
}

fn tcp_thread_per_connection() {

    println!("--- Thread-Per-Connection Server ---");

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let hisob = Arc::new(Mutex::new(0u32));

    // Server — har ulanish uchun yangi thread
    // Server — новый поток для каждого соединения
    let hisob_clone = Arc::clone(&hisob);
    let server = thread::spawn(move || {
        // set_nonblocking — qabul qilish bo'lmasa darhol qaytadi
        listener.set_nonblocking(true).unwrap();

        let mut ulanishlar: Vec<thread::JoinHandle<()>> = vec![];
        let boshlanish = std::time::Instant::now();

        while boshlanish.elapsed() < Duration::from_millis(200) {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("[Server] Yangi ulanish: {}", addr);
                    let h = Arc::clone(&hisob_clone);
                    ulanishlar.push(thread::spawn(move || {
                        let mut buf = [0u8; 256];
                        if let Ok(n) = stream.read(&mut buf) {
                            let client_id: u32 = String::from_utf8_lossy(&buf[..n])
                                .trim().parse().unwrap_or(0);
                            let mut hisob = h.lock().unwrap();
                            *hisob += client_id;
                            stream.write_all(format!("Client {} qabul qilindi", client_id).as_bytes()).unwrap();
                        }
                    }));
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
        for h in ulanishlar { h.join().ok(); }
    });

    thread::sleep(Duration::from_millis(20));

    // 3 ta client parallel ulanadi
    let mut client_handlar = vec![];
    for id in 1u32..=3 {
        let a = addr;
        client_handlar.push(thread::spawn(move || {
            if let Ok(mut stream) = TcpStream::connect(a) {
                stream.write_all(format!("{}", id).as_bytes()).unwrap();
                let mut buf = [0u8; 256];
                if let Ok(n) = stream.read(&mut buf) {
                    println!("[Client {}] {}", id, String::from_utf8_lossy(&buf[..n]));
                }
            }
        }));
    }

    for h in client_handlar { h.join().unwrap(); }
    server.join().unwrap();

    println!("Jami hisob: {}", hisob.lock().unwrap()); // 1+2+3 = 6
    // [Client X] Client X qabul qilindi
    // Jami hisob: 6
}

fn tcp_sozlamalar_misoli() {

    println!("--- TCP Sozlamalar ---");

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = thread::spawn(move || {
        if let Ok((stream, _)) = listener.accept() {
            // TcpStream sozlamalari
            // Настройки TcpStream

            // nodelay — Nagle algoritmini o'chirish (kichik paketlar uchun)
            // nodelay — отключение алгоритма Nagle (для малых пакетов)
            stream.set_nodelay(true).unwrap();
            println!("[Server] nodelay: {}", stream.nodelay().unwrap());

            // read_timeout — o'qish timeout
            // read_timeout — таймаут чтения
            stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

            // write_timeout — yozish timeout
            stream.set_write_timeout(Some(Duration::from_secs(5))).unwrap();

            // ttl — Time To Live
            stream.set_ttl(64).unwrap();
            println!("[Server] TTL: {}", stream.ttl().unwrap());

            // peer_addr — remote manzil
            println!("[Server] Client: {}", stream.peer_addr().unwrap());
            // local_addr — lokal manzil
            println!("[Server] Local: {}", stream.local_addr().unwrap());
        }
    });

    thread::sleep(Duration::from_millis(10));
    let _client = TcpStream::connect(addr).unwrap();
    server.join().unwrap();
    // [Server] nodelay: true
    // [Server] TTL: 64
    // [Server] Client: 127.0.0.1:XXXXX
    // [Server] Local: 127.0.0.1:XXXXX
}

fn udp_misollari() {

    println!("--- UDP Socket ---");

    // UdpSocket — soket yaratish
    // UdpSocket — создание сокета
    let server = UdpSocket::bind("127.0.0.1:0").unwrap();
    let server_addr = server.local_addr().unwrap();
    println!("UDP Server: {}", server_addr);

    let client = UdpSocket::bind("127.0.0.1:0").unwrap();
    let client_addr = client.local_addr().unwrap();
    println!("UDP Client: {}", client_addr);

    // Client → Server xabar yuborish
    // Client → Server отправка сообщения
    let xabarlar = ["UDP xabar 1", "UDP xabar 2", "UDP xabar 3"];

    let server_thread = thread::spawn(move || {
        let mut buf = [0u8; 1024];
        let mut qabul_qilingan = vec![];
        for _ in 0..xabarlar.len() {
            let (n, client_addr) = server.recv_from(&mut buf).unwrap();
            let xabar = String::from_utf8_lossy(&buf[..n]).to_string();
            println!("[UDP Server] '{}' dan: {}", xabar, client_addr);
            qabul_qilingan.push(xabar.clone());
            // Echo qaytarish
            server.send_to(format!("OK: {}", xabar).as_bytes(), client_addr).unwrap();
        }
        qabul_qilingan
    });

    thread::sleep(Duration::from_millis(10));

    for xabar in &xabarlar {
        // send_to — manzil ko'rsatib yuborish
        // send_to — отправка с указанием адреса
        client.send_to(xabar.as_bytes(), server_addr).unwrap();

        let mut buf = [0u8; 1024];
        let (n, _) = client.recv_from(&mut buf).unwrap();
        println!("[UDP Client] Echo: '{}'", String::from_utf8_lossy(&buf[..n]));
    }

    let qabul = server_thread.join().unwrap();
    println!("Server qabul qildi: {:?}", qabul);
    // [UDP Server] 'UDP xabar 1' dan: 127.0.0.1:XXXXX
    // [UDP Client] Echo: 'OK: UDP xabar 1'
    // ...
}

fn udp_connect_misoli() {

    println!("--- UDP Connect ---");

    let server = UdpSocket::bind("127.0.0.1:0").unwrap();
    let server_addr = server.local_addr().unwrap();

    let server_thread = thread::spawn(move || {
        let mut buf = [0u8; 1024];
        for _ in 0..3 {
            let (n, addr) = server.recv_from(&mut buf).unwrap();
            println!("[UDP] {}: {}", addr, String::from_utf8_lossy(&buf[..n]));
        }
    });

    thread::sleep(Duration::from_millis(10));

    // connect — manzilni oldindan o'rnatish (send/recv oddiy bo'ladi)
    // connect — предварительная установка адреса (send/recv становятся простыми)
    let client = UdpSocket::bind("127.0.0.1:0").unwrap();
    client.connect(server_addr).unwrap();

    // Endi send_to o'rniga send ishlatiladi
    // Теперь вместо send_to используется send
    client.send(b"Birinchi").unwrap();
    client.send(b"Ikkinchi").unwrap();
    client.send(b"Uchinchi").unwrap();

    server_thread.join().unwrap();
    // [UDP] 127.0.0.1:XXXXX: Birinchi
    // [UDP] 127.0.0.1:XXXXX: Ikkinchi
    // [UDP] 127.0.0.1:XXXXX: Uchinchi
}

// Oddiy HTTP-like server
// Простой HTTP-подобный сервер
fn http_like_server() {

    println!("--- HTTP-like Server ---");

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    let server = thread::spawn(move || {
        let boshlanish = std::time::Instant::now();
        listener.set_nonblocking(true).unwrap();

        while boshlanish.elapsed() < Duration::from_millis(300) {
            match listener.accept() {
                Ok((stream, _)) => {
                    thread::spawn(move || {
                        let mut reader = BufReader::new(stream.try_clone().unwrap());
                        let mut writer = BufWriter::new(stream);

                        // So'rovni o'qish
                        let mut satr = String::new();
                        reader.read_line(&mut satr).unwrap();
                        let satr = satr.trim_end().to_string();

                        // Routing
                        let javob = if satr.starts_with("GET /users") {
                            r#"{"users": ["Dilshod", "Ali", "Vali"]}"#
                        } else if satr.starts_with("GET /status") {
                            r#"{"status": "ok", "version": "1.0"}"#
                        } else {
                            r#"{"error": "not found"}"#
                        };

                        let http_javob = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                            javob.len(), javob
                        );
                        writer.write_all(http_javob.as_bytes()).unwrap();
                        writer.flush().unwrap();
                        println!("[Server] '{}' → {}", satr, javob);
                    });
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(5));
                }
                Err(_) => break,
            }
        }
    });

    thread::sleep(Duration::from_millis(20));

    // HTTP-like clientlar
    let so_rovlar = ["GET /users HTTP/1.1", "GET /status HTTP/1.1", "GET /unknown HTTP/1.1"];

    for so_rov in &so_rovlar {
        let mut stream = TcpStream::connect(addr).unwrap();
        writeln!(stream, "{}", so_rov).unwrap();
        stream.flush().unwrap();

        let mut reader = BufReader::new(stream);
        let mut javob = String::new();

        // Header qatorlarni o'tkazish
        loop {
            let mut satr = String::new();
            reader.read_line(&mut satr).unwrap();
            if satr.trim().is_empty() { break; }
        }

        // Body o'qish
        reader.read_line(&mut javob).unwrap();
        println!("[Client] {}: {}", so_rov.split(' ').nth(1).unwrap_or("?"), javob.trim());
    }

    server.join().unwrap();
    // [Client] /users: {"users": ["Dilshod", "Ali", "Vali"]}
    // [Client] /status: {"status": "ok", "version": "1.0"}
    // [Client] /unknown: {"error": "not found"}
}

// Chat server simulyatsiyasi
// Симуляция чат-сервера
fn chat_server_simulyatsiya() {

    println!("--- Chat Server ---");

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();

    // Barcha ulanishlarga xabar yuborish uchun
    // Для отправки сообщений всем соединениям
    let clients: Arc<Mutex<Vec<std::net::TcpStream>>> = Arc::new(Mutex::new(vec![]));

    let clients_clone = Arc::clone(&clients);
    let server = thread::spawn(move || {
        listener.set_nonblocking(true).unwrap();
        let boshlanish = std::time::Instant::now();

        while boshlanish.elapsed() < Duration::from_millis(500) {
            match listener.accept() {
                Ok((stream, addr)) => {
                    println!("[Chat] Yangi ishtirokchi: {}", addr);
                    stream.set_nonblocking(true).unwrap();
                    clients_clone.lock().unwrap().push(stream.try_clone().unwrap());

                    let c = Arc::clone(&clients_clone);
                    thread::spawn(move || {
                        let mut reader = BufReader::new(stream);
                        loop {
                            let mut xabar = String::new();
                            match reader.read_line(&mut xabar) {
                                Ok(0)  => break,
                                Ok(_)  => {
                                    let xabar = xabar.trim_end().to_string();
                                    println!("[Chat] Xabar: '{}'", xabar);
                                    // Barcha clientlarga yuborish
                                    let mut clients = c.lock().unwrap();
                                    clients.retain_mut(|c| {
                                        writeln!(c, "{}", xabar).is_ok()
                                    });
                                }
                                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                    thread::sleep(Duration::from_millis(5));
                                }
                                Err(_) => break,
                            }
                        }
                    });
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(_) => break,
            }
        }
    });

    thread::sleep(Duration::from_millis(30));

    // 2 ta chat client
    let mut c1 = BufWriter::new(TcpStream::connect(addr).unwrap());
    thread::sleep(Duration::from_millis(20));

    writeln!(c1, "Salom hammaga!").unwrap();
    c1.flush().unwrap();
    thread::sleep(Duration::from_millis(20));

    writeln!(c1, "Rust tarmoq dasturlash!").unwrap();
    c1.flush().unwrap();
    thread::sleep(Duration::from_millis(100));

    server.join().unwrap();
    // [Chat] Yangi ishtirokchi: ...
    // [Chat] Xabar: 'Salom hammaga!'
    // [Chat] Xabar: 'Rust tarmoq dasturlash!'
}

fn main() {

    println!("=== IP ADDR VA SOCKET ===");
    ip_addr_misollari();

    println!("\n=== TCP ECHO SERVER ===");
    tcp_echo_server_misoli();

    println!("\n=== TCP BUFFERED I/O ===");
    tcp_buffered_misoli();

    println!("\n=== THREAD-PER-CONNECTION ===");
    tcp_thread_per_connection();

    println!("\n=== TCP SOZLAMALAR ===");
    tcp_sozlamalar_misoli();

    println!("\n=== UDP ===");
    udp_misollari();

    println!("\n=== UDP CONNECT ===");
    udp_connect_misoli();

    println!("\n=== HTTP-LIKE SERVER ===");
    http_like_server();

    println!("\n=== CHAT SERVER ===");
    chat_server_simulyatsiya();
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                     | Tavsif (UZ)                                | Описание (RU)                                          |
// #================================================================================================================================================#
// # |                                        IP VA SOCKET                                                                                          |
// #================================================================================================================================================#
// # |   1 | Ipv4Addr::new(a,b,c,d)           | IPv4 manzil yaratish                       | Создание адреса IPv4                                   |
// # |   2 | "1.2.3.4".parse::<IpAddr>()      | Satrdan parse                              | Парсинг из строки                                      |
// # |   3 | SocketAddr                       | IP + port                                  | IP + порт                                              |
// # |   4 | ToSocketAddrs                    | Turli format manzillarni qabul qilish      | Принятие адресов в разных форматах                     |
// #================================================================================================================================================#
// # |                                        TCP                                                                                                   |
// #================================================================================================================================================#
// # |   5 | TcpListener::bind(addr)          | Server: portni bog'lash                    | Сервер: привязка порта                                 |
// # |   6 | listener.accept()                | Yangi ulanishni qabul qilish               | Принятие нового соединения                             |
// # |   7 | TcpStream::connect(addr)         | Client: serverga ulanish                   | Клиент: подключение к серверу                          |
// # |   8 | stream.read(&mut buf)            | Ma'lumot o'qish                            | Чтение данных                                          |
// # |   9 | stream.write_all(buf)            | Ma'lumot yozish                            | Запись данных                                          |
// # |  10 | stream.try_clone()               | Streamni klonlash (read+write ajratish)    | Клонирование (разделение read+write)                   |
// # |  11 | BufReader / BufWriter            | Satr bo'yicha o'qish/yozish                | Чтение/запись по строкам                               |
// # |  12 | set_nonblocking(true)            | Non-blocking rejim                         | Режим без блокирования                                 |
// # |  13 | set_read_timeout(dur)            | O'qish timeout                             | Таймаут чтения                                         |
// # |  14 | set_nodelay(true)                | Nagle algoritmini o'chirish                | Отключение алгоритма Nagle                             |
// # |  15 | set_ttl(n)                       | Time To Live o'rnatish                     | Установка Time To Live                                 |
// #================================================================================================================================================#
// # |                                        UDP                                                                                                   |
// #================================================================================================================================================#
// # |  16 | UdpSocket::bind(addr)            | UDP soket yaratish                         | Создание UDP сокета                                    |
// # |  17 | socket.send_to(buf, addr)        | Manzil ko'rsatib yuborish                  | Отправка с указанием адреса                            |
// # |  18 | socket.recv_from(&mut buf)       | Yuboruvchi manzili bilan qabul             | Приём с адресом отправителя                            |
// # |  19 | socket.connect(addr)             | Default manzil o'rnatish                   | Установка адреса по умолчанию                          |
// # |  20 | socket.send / socket.recv        | connect dan keyin oddiy send/recv          | Простые send/recv после connect                        |
// #================================================================================================================================================#