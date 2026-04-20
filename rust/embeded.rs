// #================================================================================================================================================#
// #                                                               EMBEDDED RUST                                                                    #
// #                    EMBEDDED — MCU DASTURLASH. REGISTER, GPIO, UART, I2C, SPI, TIMER. HAL, PAC. SAFE ABSTRAKTSIYA.                              #
// #                    EMBEDDED — ПРОГРАММИРОВАНИЕ MCU. REGISTER, GPIO, UART, I2C, SPI, TIMER. HAL, PAC. SAFE АБСТРАКЦИЯ.                          #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::fmt;

// Embedded Rust nima:
// Что такое Embedded Rust:
//
//   Microcontroller dasturlash — Rust bilan
//   Программирование микроконтроллеров — на Rust
//
//   Qatlamlar:
//   Уровни:
//   PAC  — Peripheral Access Crate (Register darajasi)
//   HAL  — Hardware Abstraction Layer (Qulay interfeys)
//   BSP  — Board Support Package (Platfor spesifik)
//   APP  — Ilova darajasi
//
//   Mashhur targetlar:
//   Популярные цели:
//   - ARM Cortex-M (STM32, nRF52, RP2040)
//   - RISC-V (ESP32-C3, GD32VF)
//   - AVR (Arduino — avr-gcc)
//
//   embedded-hal — universal abstraktsiya traitleri
//   embedded-hal — универсальные абстракционные трейты

// Real PAC (svd2rust tomonidan generatsiya):
// Настоящий PAC (генерируется svd2rust):
//
// #[repr(C)]
// pub struct GPIOA {
//     pub moder: MODER,   // Mode register
//     pub otyper: OTYPER, // Output type register
//     pub ospeedr: OSPEEDR,
//     pub pupdr: PUPDR,
//     pub idr: IDR,       // Input data register
//     pub odr: ODR,       // Output data register
//     pub bsrr: BSRR,     // Bit set/reset register
//     pub lckr: LCKR,
//     pub afrl: AFRL,     // Alternate function low
//     pub afrh: AFRH,     // Alternate function high
// }

// Register simulyatsiya — volatile read/write
// Симуляция регистра — volatile чтение/запись
use std::cell::UnsafeCell;

struct Register(UnsafeCell<u32>);

impl Register {
    const fn new(val: u32) -> Self { Register(UnsafeCell::new(val)) }

    fn o_qi(&self) -> u32 {
        // Real kodda: core::ptr::read_volatile(self.0.get())
        unsafe { *self.0.get() }
    }

    fn yoz(&self, val: u32) {
        // Real kodda: core::ptr::write_volatile(self.0.get(), val)
        unsafe { *self.0.get() = val; }
    }

    fn bit_o_rnat(&self, bit: u8) {
        let joriy = self.o_qi();
        self.yoz(joriy | (1u32 << bit));
    }

    fn bit_o_chir(&self, bit: u8) {
        let joriy = self.o_qi();
        self.yoz(joriy & !(1u32 << bit));
    }

    fn bit_tekshir(&self, bit: u8) -> bool {
        (self.o_qi() >> bit) & 1 != 0
    }

    fn maydon_o_rnat(&self, offset: u8, width: u8, val: u32) {
        let mask = (1u32 << width) - 1;
        let joriy = self.o_qi() & !(mask << offset);
        self.yoz(joriy | ((val & mask) << offset));
    }
}

unsafe impl Sync for Register {}

// GPIO Register bloki simulyatsiyasi
// Симуляция блока GPIO регистров
struct GpioRegisters {
    moder:  Register, // Pin rejimi (input/output/af/analog)
    otyper: Register, // Output turi (push-pull/open-drain)
    odr:    Register, // Output data
    idr:    Register, // Input data
    bsrr:   Register, // Bit set/reset
}

impl GpioRegisters {
    fn new() -> Self {
        GpioRegisters {
            moder:  Register::new(0xABFF_FFFF), // Default: analog
            otyper: Register::new(0),
            odr:    Register::new(0),
            idr:    Register::new(0),
            bsrr:   Register::new(0),
        }
    }
}

fn register_darajasi_misoli() {

    println!("--- Register Darajasi (PAC) ---");

    let gpio = GpioRegisters::new();

    // Pin 5 ni output rejimiga o'rnatish
    // MODER bits [11:10] = 01 (output)
    gpio.moder.maydon_o_rnat(10, 2, 0b01);
    println!("MODER: {:#034b}", gpio.moder.o_qi());

    // Pin 5 ni yuqori (high) qilish — BSRR orqali
    gpio.bsrr.yoz(1 << 5); // BS5 biti
    gpio.odr.bit_o_rnat(5);
    println!("ODR (high): {:#018b}", gpio.odr.o_qi());

    // Pin 5 ni past (low) qilish
    gpio.bsrr.yoz(1 << (5 + 16)); // BR5 biti
    gpio.odr.bit_o_chir(5);
    println!("ODR (low):  {:#018b}", gpio.odr.o_qi());

    // Pin 5 ni o'qish
    gpio.idr.bit_o_rnat(3); // 3-pin high
    println!("IDR pin3:   {}", gpio.idr.bit_tekshir(3));
    println!("IDR pin5:   {}", gpio.idr.bit_tekshir(5));
    // MODER: 0b...
    // ODR (high): 0b0000000000100000
    // ODR (low):  0b0000000000000000
    // IDR pin3:   true
    // IDR pin5:   false
}

// embedded-hal traitleri simulyatsiyasi
// Симуляция трейтов embedded-hal

trait OutputPin {
    type Error: fmt::Debug;
    fn high(&mut self) -> Result<(), Self::Error>;
    fn low(&mut self) -> Result<(), Self::Error>;
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

trait InputPin {
    type Error: fmt::Debug;
    fn is_high(&self) -> Result<bool, Self::Error>;
    fn is_low(&self) -> Result<bool, Self::Error>;
}

// GPIO pin holatlari — TypeState bilan
// Состояния GPIO pin — с TypeState
struct Input;
struct Output;
struct AlternateFunksiya;

struct GpioPin<Holat> {
    port: char,
    pin: u8,
    qiymat: bool,
    _holat: std::marker::PhantomData<Holat>,
}

impl GpioPin<Output> {
    fn new_output(port: char, pin: u8) -> Self {
        println!("[GPIO] {}{}:  Output rejimi", port, pin);
        GpioPin { port, pin, qiymat: false, _holat: std::marker::PhantomData }
    }

    fn into_input(self) -> GpioPin<Input> {
        println!("[GPIO] {}{}: Input rejimiga o'tdi", self.port, self.pin);
        GpioPin { port: self.port, pin: self.pin, qiymat: false, _holat: std::marker::PhantomData }
    }
}

impl GpioPin<Input> {
    fn new_input(port: char, pin: u8) -> Self {
        println!("[GPIO] {}{}: Input rejimi", port, pin);
        GpioPin { port, pin, qiymat: false, _holat: std::marker::PhantomData }
    }

    fn simulyatsiya_qiymat(&mut self, val: bool) { self.qiymat = val; }

    fn into_output(self) -> GpioPin<Output> {
        println!("[GPIO] {}{}: Output rejimiga o'tdi", self.port, self.pin);
        GpioPin { port: self.port, pin: self.pin, qiymat: false, _holat: std::marker::PhantomData }
    }
}

#[derive(Debug)]
struct GpioXato;

impl OutputPin for GpioPin<Output> {
    type Error = GpioXato;

    fn high(&mut self) -> Result<(), GpioXato> {
        self.qiymat = true;
        println!("[GPIO] {}{}: HIGH ↑", self.port, self.pin);
        Ok(())
    }

    fn low(&mut self) -> Result<(), GpioXato> {
        self.qiymat = false;
        println!("[GPIO] {}{}: LOW ↓", self.port, self.pin);
        Ok(())
    }

    fn toggle(&mut self) -> Result<(), GpioXato> {
        if self.qiymat { self.low() } else { self.high() }
    }
}

impl InputPin for GpioPin<Input> {
    type Error = GpioXato;
    fn is_high(&self) -> Result<bool, GpioXato> { Ok(self.qiymat) }
    fn is_low(&self)  -> Result<bool, GpioXato> { Ok(!self.qiymat) }
}

fn hal_gpio_misoli() {

    println!("\n--- HAL: GPIO ---");

    // TypeState — compile time xavfsizlik
    // TypeState — безопасность во время компиляции
    let mut led = GpioPin::<Output>::new_output('A', 5);

    led.high().unwrap();
    led.toggle().unwrap();
    led.toggle().unwrap();
    led.low().unwrap();

    let mut tugma = GpioPin::<Input>::new_input('C', 13);
    tugma.simulyatsiya_qiymat(true);
    println!("Tugma bosildi: {}", tugma.is_high().unwrap());

    // input.high() — KOMPILE XATO! Input da high() yo'q
    // input.high() — ОШИБКА КОМПИЛЯЦИИ! Нет high() у Input
    println!("TypeState kafolati: Input pindan yozib bo'lmaydi ✅");
    // [GPIO] A5: Output rejimi
    // [GPIO] A5: HIGH ↑
    // [GPIO] A5: LOW ↓
    // [GPIO] A5: HIGH ↑
    // [GPIO] A5: LOW ↓
    // Tugma bosildi: true
}

trait SerialWrite {
    type Error: fmt::Debug;
    fn yoz_bayt(&mut self, bayt: u8) -> Result<(), Self::Error>;
    fn yoz_str(&mut self, s: &str) -> Result<(), Self::Error>;
    fn flush(&mut self) -> Result<(), Self::Error>;
}

trait SerialRead {
    type Error: fmt::Debug;
    fn o_qi_bayt(&mut self) -> Result<u8, Self::Error>;
    fn o_qishga_tayyor(&self) -> bool;
}

struct UartPort {
    nomi: String,
    baud_rate: u32,
    tx_buf: VecDeque<u8>,
    rx_buf: VecDeque<u8>,
    yoqilgan: bool,
}

#[derive(Debug)]
enum UartXato { BufTola, YoqilmaganPort, BoshBuf }

impl UartPort {
    fn new(nomi: &str, baud: u32) -> Self {
        println!("[UART] {} init: {} baud", nomi, baud);
        UartPort {
            nomi: nomi.to_string(),
            baud_rate: baud,
            tx_buf: VecDeque::new(),
            rx_buf: VecDeque::new(),
            yoqilgan: true,
        }
    }

    fn rx_ga_ma_lumot_qo_sh(&mut self, data: &[u8]) {
        for &b in data { self.rx_buf.push_back(b); }
    }

    fn tx_ni_ol(&mut self) -> Vec<u8> {
        self.tx_buf.drain(..).collect()
    }
}

impl SerialWrite for UartPort {
    type Error = UartXato;

    fn yoz_bayt(&mut self, bayt: u8) -> Result<(), UartXato> {
        if !self.yoqilgan { return Err(UartXato::YoqilmaganPort); }
        self.tx_buf.push_back(bayt);
        Ok(())
    }

    fn yoz_str(&mut self, s: &str) -> Result<(), UartXato> {
        for b in s.bytes() { self.yoz_bayt(b)?; }
        Ok(())
    }

    fn flush(&mut self) -> Result<(), UartXato> {
        let data = self.tx_ni_ol();
        if !data.is_empty() {
            let matn = String::from_utf8_lossy(&data);
            println!("[UART {}] TX: \"{}\"", self.nomi, matn);
        }
        Ok(())
    }
}

impl SerialRead for UartPort {
    type Error = UartXato;

    fn o_qi_bayt(&mut self) -> Result<u8, UartXato> {
        self.rx_buf.pop_front().ok_or(UartXato::BoshBuf)
    }

    fn o_qishga_tayyor(&self) -> bool { !self.rx_buf.is_empty() }
}

fn uart_misoli() {

    println!("\n--- HAL: UART ---");

    let mut uart = UartPort::new("USART1", 115200);

    // Yuborish
    uart.yoz_str("AT\r\n").unwrap();
    uart.yoz_str("AT+VERSION\r\n").unwrap();
    uart.flush().unwrap();

    // Qabul qilish simulyatsiyasi
    uart.rx_ga_ma_lumot_qo_sh(b"OK\r\n");
    uart.rx_ga_ma_lumot_qo_sh(b"V1.0.0\r\n");

    print!("[UART USART1] RX: \"");
    while uart.o_qishga_tayyor() {
        let b = uart.o_qi_bayt().unwrap();
        if b != b'\r' && b != b'\n' { print!("{}", b as char); }
    }
    println!("\"");
    // [UART] USART1 init: 115200 baud
    // [UART USART1] TX: "AT\r\nAT+VERSION\r\n"
    // [UART USART1] RX: "OKV1.0.0"
}

trait I2cWrite {
    type Error: fmt::Debug;
    fn yoz(&mut self, addr: u8, data: &[u8]) -> Result<(), Self::Error>;
}

trait I2cRead {
    type Error: fmt::Debug;
    fn o_qi(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), Self::Error>;
}

trait I2cWriteRead: I2cWrite + I2cRead {
    fn yoz_o_qi(&mut self, addr: u8, yoz_data: &[u8], o_qi_buf: &mut [u8])
                -> Result<(), <Self as I2cWrite>::Error>;
}

struct I2cShina {
    qurilmalar: std::collections::HashMap<u8, Vec<u8>>,
}

#[derive(Debug)]
enum I2cXato { QurilmaTopilmadi(u8), Nack, BufXato }

impl I2cShina {
    fn new() -> Self { I2cShina { qurilmalar: std::collections::HashMap::new() } }

    fn qurilma_qo_sh(&mut self, addr: u8, javob: Vec<u8>) {
        println!("[I2C] Qurilma 0x{:02X} qo'shildi", addr);
        self.qurilmalar.insert(addr, javob);
    }
}

impl I2cWrite for I2cShina {
    type Error = I2cXato;

    fn yoz(&mut self, addr: u8, data: &[u8]) -> Result<(), I2cXato> {
        if !self.qurilmalar.contains_key(&addr) {
            return Err(I2cXato::QurilmaTopilmadi(addr));
        }
        println!("[I2C] → 0x{:02X}: {:?}", addr, data);
        Ok(())
    }
}

impl I2cRead for I2cShina {
    type Error = I2cXato;

    fn o_qi(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), I2cXato> {
        let javob = self.qurilmalar.get(&addr)
            .ok_or(I2cXato::QurilmaTopilmadi(addr))?;
        let n = buf.len().min(javob.len());
        buf[..n].copy_from_slice(&javob[..n]);
        println!("[I2C] ← 0x{:02X}: {:?}", addr, &buf[..n]);
        Ok(())
    }
}

// BME280 sensor simulyatsiyasi
// Симуляция датчика BME280
struct Bme280<I2C> {
    i2c: I2C,
    addr: u8,
}

const BME280_ADDR: u8 = 0x76;
const BME280_ID_REG: u8 = 0xD0;
const BME280_TEMP_REG: u8 = 0xFA;

impl<I2C: I2cWrite<Error = I2cXato> + I2cRead<Error = I2cXato>> Bme280<I2C> {
    fn new(i2c: I2C) -> Self {
        Bme280 { i2c, addr: BME280_ADDR }
    }

    fn chip_id_o_qi(&mut self) -> Result<u8, I2cXato> {
        self.i2c.yoz(self.addr, &[BME280_ID_REG])?;
        let mut buf = [0u8; 1];
        self.i2c.o_qi(self.addr, &mut buf)?;
        Ok(buf[0])
    }

    fn harorat_o_qi(&mut self) -> Result<f32, I2cXato> {
        self.i2c.yoz(self.addr, &[BME280_TEMP_REG])?;
        let mut buf = [0u8; 3];
        self.i2c.o_qi(self.addr, &mut buf)?;
        // Soddalashtirilgan konversiya
        let raw = ((buf[0] as u32) << 12) | ((buf[1] as u32) << 4) | ((buf[2] as u32) >> 4);
        Ok(raw as f32 / 100.0)
    }
}

fn i2c_misoli() {

    println!("\n--- HAL: I2C + BME280 ---");

    let mut i2c = I2cShina::new();
    i2c.qurilma_qo_sh(BME280_ADDR, vec![0x60, 0x19, 0x00, 0x00]); // chip_id=0x60

    let mut sensor = Bme280::new(i2c);

    match sensor.chip_id_o_qi() {
        Ok(id) => println!("BME280 chip ID: 0x{:02X}", id), // 0x60
        Err(e) => println!("Xato: {:?}", e),
    }

    match sensor.harorat_o_qi() {
        Ok(t) => println!("Harorat: {:.1}°C", t),
        Err(e) => println!("Xato: {:?}", e),
    }
    // [I2C] Qurilma 0x76 qo'shildi
    // [I2C] → 0x76: [208]
    // [I2C] ← 0x76: [96, 25, 0, 0]
    // BME280 chip ID: 0x60
}

trait DelayMs {
    fn delay_ms(&mut self, ms: u32);
}

trait DelayUs {
    fn delay_us(&mut self, us: u32);
}

struct SysTimer {
    takt: u32, // Hz
    o_tgan_ms: u64,
}

impl SysTimer {
    fn new(takt_hz: u32) -> Self {
        println!("[Timer] SysTick: {} Hz", takt_hz);
        SysTimer { takt: takt_hz, o_tgan_ms: 0 }
    }

    fn tick_soni(&self) -> u64 { self.o_tgan_ms }
    fn hozirgi_ms(&self) -> u64 { self.o_tgan_ms }
}

impl DelayMs for SysTimer {
    fn delay_ms(&mut self, ms: u32) {
        println!("[Timer] {} ms kutish", ms);
        self.o_tgan_ms += ms as u64;
    }
}

impl DelayUs for SysTimer {
    fn delay_us(&mut self, us: u32) {
        println!("[Timer] {} µs kutish", us);
        self.o_tgan_ms += (us as u64) / 1000;
    }
}

fn timer_misoli() {

    println!("\n--- HAL: Timer/Delay ---");

    let mut timer = SysTimer::new(168_000_000); // 168 MHz (STM32F4)

    // LED blink
    let mut led = GpioPin::<Output>::new_output('A', 5);

    for _ in 0..3 {
        led.high().unwrap();
        timer.delay_ms(500);
        led.low().unwrap();
        timer.delay_ms(500);
    }

    println!("Jami o'tgan: {} ms", timer.hozirgi_ms());
    // [Timer] SysTick: 168000000 Hz
    // [GPIO] A5: HIGH ↑
    // [Timer] 500 ms kutish
    // [GPIO] A5: LOW ↓
    // [Timer] 500 ms kutish
    // ... (3 marta)
    // Jami o'tgan: 3000 ms
}

fn rtic_tushuntirish() {

    println!("\n--- RTIC Framework ---");

    println!(r#"
// RTIC — Rust da real-time embedded dasturlash
// RTIC — программирование real-time embedded на Rust
//
// Cargo.toml:
// cortex-m-rtic = "1.0"
//
// #[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
// mod app {{
//     use super::*;
//
//     #[shared]
//     struct Shared {{ led: LedPin, hisob: u32 }}
//
//     #[local]
//     struct Local {{ timer: Timer<TIM2> }}
//
//     #[init]
//     fn init(cx: init::Context) -> (Shared, Local) {{
//         // Hardware initsializatsiya
//         let dp = cx.device;
//         let gpioa = dp.GPIOA.split();
//         let led = gpioa.pa5.into_push_pull_output();
//         let timer = Timer::new(dp.TIM2, 1.hz(), &clocks);
//         timer.listen(Event::Update);
//
//         (Shared {{ led, hisob: 0 }}, Local {{ timer }})
//     }}
//
//     #[idle]
//     fn idle(_: idle::Context) -> ! {{
//         loop {{ cortex_m::asm::wfi(); }} // Wait For Interrupt
//     }}
//
//     #[task(binds = TIM2, local = [timer], shared = [led, hisob])]
//     fn tim2_interrupt(cx: tim2_interrupt::Context) {{
//         cx.local.timer.clear_interrupt(Event::Update);
//         let hisob = cx.shared.hisob.lock(|h| {{ *h += 1; *h }});
//         cx.shared.led.lock(|l| l.toggle().unwrap());
//     }}
// }}"#);

    println!("\nRTIC afzalliklari:");
    println!("  ✅ Compile-time deadlock xavfsizligi");
    println!("  ✅ Priority-based preemption");
    println!("  ✅ Zero-cost resource sharing");
    println!("  ✅ Async/await support (RTIC 2.0)");
}

fn real_embedded_dastur() {

    println!("\n--- Real Embedded Dastur Simulyatsiyasi ---");
    println!("Platform: STM32F411 (168 MHz, Cortex-M4)");
    println!();

    // Hardware initsializatsiya
    let mut led     = GpioPin::<Output>::new_output('A', 5);  // PA5 — LED
    let mut tugma   = GpioPin::<Input>::new_input('C', 13);   // PC13 — Button
    let mut uart    = UartPort::new("USART2", 115200);
    let mut timer   = SysTimer::new(168_000_000);

    // Sensorlar
    let mut i2c = I2cShina::new();
    i2c.qurilma_qo_sh(0x76, vec![0x60, 0x19, 0x00, 0x00]);
    let mut sensor = Bme280::new(i2c);

    uart.yoz_str("=== Embedded Rust Demo ===\r\n").unwrap();
    uart.flush().unwrap();

    // Asosiy tsikl
    for tsikl in 0..3 {
        uart.yoz_str(&format!("Tsikl {}\r\n", tsikl)).unwrap();

        // Sensor o'qish
        if let Ok(id) = sensor.chip_id_o_qi() {
            uart.yoz_str(&format!("BME280 ID: 0x{:02X}\r\n", id)).unwrap();
        }

        // LED blink
        led.high().unwrap();
        timer.delay_ms(100);
        led.low().unwrap();
        timer.delay_ms(100);

        // Tugma
        tugma.simulyatsiya_qiymat(tsikl % 2 == 0);
        if tugma.is_high().unwrap() {
            uart.yoz_str("Tugma bosildi!\r\n").unwrap();
        }

        uart.flush().unwrap();
    }

    println!("Jami: {} ms", timer.hozirgi_ms());
}

fn main() {

    println!("=== REGISTER DARAJASI ===");
    register_darajasi_misoli();

    println!("\n=== HAL: GPIO ===");
    hal_gpio_misoli();

    println!("\n=== HAL: UART ===");
    uart_misoli();

    println!("\n=== HAL: I2C ===");
    i2c_misoli();

    println!("\n=== HAL: TIMER ===");
    timer_misoli();

    println!("\n=== RTIC ===");
    rtic_tushuntirish();

    println!("\n=== REAL DASTUR ===");
    real_embedded_dastur();
}

// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        QATLAMLAR                                                                                              |
// #================================================================================================================================================#
// # |   1 | PAC (svd2rust)                  | Register darajasi, to'g'ridan hardware     | Уровень регистров, прямой доступ к железу               |
// # |   2 | HAL                             | Qulay abstraktsiya traitleri               | Удобные абстракционные трейты                           |
// # |   3 | BSP                             | Platforma spesifik kutubxona               | Платформо-специфичная библиотека                        |
// #================================================================================================================================================#
// # |                                        HAL TRAITLERI                                                                                         |
// #================================================================================================================================================#
// # |   4 | OutputPin                       | GPIO output boshqaruvi                     | Управление GPIO выходом                                 |
// # |   5 | InputPin                        | GPIO input o'qish                          | Чтение GPIO входа                                       |
// # |   6 | SerialWrite/SerialRead          | UART yozish/o'qish                         | Запись/чтение UART                                      |
// # |   7 | I2cWrite/I2cRead                | I2C protokol                               | Протокол I2C                                            |
// # |   8 | SpiWrite/SpiRead                | SPI protokol                               | Протокол SPI                                            |
// # |   9 | DelayMs/DelayUs                 | Vaqt kutish                                | Ожидание времени                                        |
// #================================================================================================================================================#
// # |                                        PATTERNLAR                                                                                            |
// #================================================================================================================================================#
// # |  10 | TypeState GPIO                  | Compile-time Input/Output xavfsizligi      | Безопасность Input/Output во время компиляции           |
// # |  11 | Generic sensor driver           | impl<I2C: I2cWrite + I2cRead>              | impl<I2C: I2cWrite + I2cRead>                           |
// # |  12 | RTIC                            | Interrupt-driven real-time concurrency     | Параллелизм реального времени на прерываниях            |
// # |  13 | volatile read/write             | Register kirish                            | Доступ к регистрам                                      |
// # |  14 | #[repr(C)] register blocks      | Hardware register mapping                  | Маппинг аппаратных регистров                            |
// #================================================================================================================================================#