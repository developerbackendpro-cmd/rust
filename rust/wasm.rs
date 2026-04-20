// #================================================================================================================================================#
// #                                                                     WASM                                                                       #
// #                            WASM — WEB ASSEMBLY BILAN RUST. WASM-BINDGEN, WEB-SYS, JS-SYS. BROWSER VA NODE.JS.                                  #
// #                            WASM — RUST С WEB ASSEMBLY. WASM-BINDGEN, WEB-SYS, JS-SYS. БРАУЗЕР И NODE.JS.                                       #
// #================================================================================================================================================#

#![allow(dead_code, unused)]

use std::fmt;

// WASM nima:
// Что такое WASM:
//
//   WebAssembly — binary instruction format
//   WebAssembly — формат бинарных инструкций
//   Browser da native tezlikda ishlaydi
//   Работает в браузере с native скоростью
//
//   Rust → WASM yo'llari:
//   Пути Rust → WASM:
//   1. wasm-bindgen — JS bilan integratsiya (to'liq)
//      wasm-bindgen — интеграция с JS (полная)
//   2. wasm-pack — paket va publish qilish
//      wasm-pack — упаковка и публикация
//   3. no_std + no_main — minimal WASM
//      no_std + no_main — минимальный WASM
//
//   Target:
//   cargo build --target wasm32-unknown-unknown
//   cargo build --target wasm32-wasi

fn wasm_bindgen_haqiqiy() {

    println!("=== WASM-BINDGEN HAQIQIY KOD ===\n");

    println!(r#"// Cargo.toml:
// [lib]
// crate-type = ["cdylib", "rlib"]
//
// [dependencies]
// wasm-bindgen = "0.2"
// web-sys = {{ version = "0.3", features = [
//     "Window", "Document", "Element", "HtmlElement",
//     "console", "CanvasRenderingContext2d", "HtmlCanvasElement"
// ]}}
// js-sys = "0.3"
// console_error_panic_hook = "0.1"
//
// [dev-dependencies]
// wasm-bindgen-test = "0.3"

use wasm_bindgen::prelude::*;
use web_sys::{{Document, Window, console}};
use js_sys::{{Array, Promise, Date}};

// 1. ODDIY FUNKSIYA — JS ga eksport
#[wasm_bindgen]
pub fn faktorial(n: u32) -> u32 {{
    (1..=n).product()
}}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {{
    let (mut a, mut b) = (0u64, 1);
    for _ in 0..n {{ (a, b) = (b, a + b); }}
    a
}}

// 2. STRUCT — JS class sifatida
#[wasm_bindgen]
pub struct Hisoblash {{
    history: Vec<f64>,
}}

#[wasm_bindgen]
impl Hisoblash {{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {{
        Hisoblash {{ history: Vec::new() }}
    }}

    pub fn qo_shish(&mut self, a: f64, b: f64) -> f64 {{
        let r = a + b;
        self.history.push(r);
        r
    }}

    pub fn oxirgi(&self) -> Option<f64> {{
        self.history.last().copied()
    }}

    pub fn tarix_uzunlik(&self) -> usize {{
        self.history.len()
    }}
}}

// 3. WEB-SYS — Browser API
#[wasm_bindgen]
pub fn dom_yangilash(id: &str, matn: &str) -> Result<(), JsValue> {{
    let window: Window = web_sys::window().ok_or("window yo'q")?;
    let document: Document = window.document().ok_or("document yo'q")?;
    let element = document.get_element_by_id(id).ok_or("element yo'q")?;
    element.set_inner_html(matn);
    Ok(())
}}

// 4. CONSOLE LOG
#[wasm_bindgen]
pub fn log_yozish(xabar: &str) {{
    console::log_1(&xabar.into());
}}

// 5. JS CALLBACK
#[wasm_bindgen]
pub fn asinxron_hisoblash(n: u32, callback: &js_sys::Function) {{
    let natija = faktorial(n);
    let this = JsValue::null();
    let arg = JsValue::from(natija);
    callback.call1(&this, &arg).unwrap();
}}

// 6. JS-SYS — JavaScript ob'ektlari
#[wasm_bindgen]
pub fn massiv_yaratish(n: u32) -> Array {{
    let arr = Array::new();
    for i in 0..n {{
        arr.push(&JsValue::from(i * i));
    }}
    arr
}}

// 7. PROMISE
#[wasm_bindgen]
pub fn sekinlashtirilgan_hisoblash(n: u32) -> Promise {{
    let future = async move {{
        // Async hisoblash
        Ok(JsValue::from(faktorial(n)))
    }};
    wasm_bindgen_futures::future_to_promise(future)
}}

// 8. CANVAS — Grafika
#[wasm_bindgen]
pub fn fraktal_chizish(canvas_id: &str) -> Result<(), JsValue> {{
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(canvas_id)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let w = canvas.width() as f64;
    let h = canvas.height() as f64;

    for py in 0..canvas.height() {{
        for px in 0..canvas.width() {{
            let x = (px as f64 / w) * 3.5 - 2.5;
            let y = (py as f64 / h) * 2.0 - 1.0;
            let rang = mandelbrot(x, y);
            ctx.set_fill_style(&format!("hsl({{}},100%,50%)", rang).into());
            ctx.fill_rect(px as f64, py as f64, 1.0, 1.0);
        }}
    }}
    Ok(())
}}

fn mandelbrot(cx: f64, cy: f64) -> u32 {{
    let (mut x, mut y) = (0.0f64, 0.0);
    for i in 0..255u32 {{
        if x*x + y*y > 4.0 {{ return i * 360 / 255; }}
        let xn = x*x - y*y + cx;
        y = 2.0*x*y + cy;
        x = xn;
    }}
    0
}}"#);
}

fn wasm_pack_ishlatish() {

    println!("\n=== WASM-PACK ISHLATISH ===\n");

    println!("# O'rnatish:");
    println!("cargo install wasm-pack");
    println!();

    println!("# Build (web uchun):");
    println!("wasm-pack build --target web");
    println!("wasm-pack build --target bundler   # webpack/rollup");
    println!("wasm-pack build --target nodejs     # Node.js");
    println!("wasm-pack build --target deno       # Deno");
    println!();

    println!("# NPM publish:");
    println!("wasm-pack publish");
    println!();

    println!("# JavaScript da ishlatish (web target):");
    println!(r#"// index.html:
// <script type="module">
//   import init, {{ faktorial, Hisoblash }} from './pkg/mening_wasm.js';
//
//   async function main() {{
//     await init(); // WASM yuklanadi
//
//     // Funksiya chaqirish
//     console.log(faktorial(10)); // 3628800
//
//     // Class ishlatish
//     const h = new Hisoblash();
//     console.log(h.qo_shish(3, 4)); // 7
//     console.log(h.oxirgi());       // 7
//     console.log(h.tarix_uzunlik()); // 1
//     h.free(); // Xotirani bo'shatish
//   }}
//   main();
// </script>"#);
    println!();

    println!("# Node.js da:");
    println!(r#"// const {{ faktorial }} = require('./pkg');
// console.log(faktorial(5)); // 120"#);
}

fn minimal_wasm_misol() {

    println!("\n=== MINIMAL WASM (no_std) ===\n");

    println!(r#"// src/lib.rs — minimal, wasm-bindgen siz
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {{
    core::arch::wasm32::unreachable()
}}

#[no_mangle]
pub extern "C" fn faktorial(n: u32) -> u32 {{
    (1..=n).product()
}}

#[no_mangle]
pub extern "C" fn qo_shish(a: f64, b: f64) -> f64 {{
    a + b
}}

#[no_mangle]
pub extern "C" fn fibonacci(n: u32) -> u64 {{
    let (mut a, mut b) = (0u64, 1);
    for _ in 0..n {{ (a, b) = (b, a + b); }}
    a
}}

// Cargo.toml:
// [lib]
// crate-type = ["cdylib"]
//
// Build:
// cargo build --target wasm32-unknown-unknown --release
// wasm2wat target/wasm32-unknown-unknown/release/mylib.wasm

// JavaScript:
// const response = await fetch('mylib.wasm');
// const bytes = await response.arrayBuffer();
// const {{ instance }} = await WebAssembly.instantiate(bytes);
// const {{ faktorial, qo_shish }} = instance.exports;
// console.log(faktorial(5));    // 120
// console.log(qo_shish(3, 4)); // 7"#);
}

fn wasm_test_misol() {

    println!("\n=== WASM TESTING ===\n");

    println!(r#"// tests/wasm_test.rs
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn faktorial_test() {{
    assert_eq!(faktorial(0), 1);
    assert_eq!(faktorial(1), 1);
    assert_eq!(faktorial(5), 120);
    assert_eq!(faktorial(10), 3628800);
}}

#[wasm_bindgen_test]
fn fibonacci_test() {{
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(10), 55);
}}

#[wasm_bindgen_test]
fn hisoblash_class_test() {{
    let mut h = Hisoblash::new();
    assert_eq!(h.qo_shish(3.0, 4.0), 7.0);
    assert_eq!(h.tarix_uzunlik(), 1);
    assert_eq!(h.oxirgi(), Some(7.0));
    h.free();
}}

// Browser testini ishga tushirish:
// wasm-pack test --headless --firefox
// wasm-pack test --headless --chrome
// wasm-pack test --node"#);
}

fn wasm_performance() {

    println!("\n=== WASM PERFORMANCE ===\n");

    println!(r#"// 1. JS ↔ WASM chegarasi optimallashtirish
// Har bir chaqiruv overhead bor — batch qiling!

// YOMON: Ko'p kichik chaqiruvlar
for (let i = 0; i < 1000000; i++) {{
    faktorial(i); // Har birida JS→WASM overhead
}}

// YAXSHI: Bir marta katta operatsiya
massiv_faktoriallari(1000000); // WASM ichida loop

// 2. WASM xotira — buffer orqali
#[wasm_bindgen]
pub fn ma_lumot_qaytarish() -> Vec<u8> {{
    let mut v = Vec::with_capacity(1000);
    for i in 0..1000u8 {{ v.push(i); }}
    v // JS ga sifatida Uint8Array
}}

// 3. Shared memory (SharedArrayBuffer)
// JS:
// const memory = new WebAssembly.Memory({{
//     initial: 256, maximum: 512, shared: true
// }});
// Rust:
// extern "C" fn set_memory(ptr: *mut u8) {{ ... }}

// 4. wee_alloc — kichik allocator
// [dependencies]
// wee_alloc = "0.4"
// [profile.release]
// lto = true
// opt-level = 'z'  ← o'lchamni minimallashtirish
//
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 5. wasm-opt bilan optimallashtirish
// wasm-pack build --release
// wasm-opt -Os output.wasm -o output_opt.wasm"#);

    println!();
    println!("Benchmark — Rust WASM vs JavaScript:");
    println!("  Fibonacci(40): Rust 2ms vs JS 800ms → 400x tez");
    println!("  Matrix mult:   Rust 10ms vs JS 500ms → 50x tez");
    println!("  Mandelbrot:    Rust 50ms vs JS 2000ms → 40x tez");
}

// std muhitida WASM mantiqini simulyatsiya
// Симуляция логики WASM в среде std

fn faktorial(n: u32) -> u32 { (1..=n).product() }
fn fibonacci(n: u32) -> u64 {
    let (mut a, mut b) = (0u64, 1);
    for _ in 0..n { (a, b) = (b, a + b); }
    a
}
fn mandelbrot_iter(cx: f64, cy: f64) -> u32 {
    let (mut x, mut y) = (0.0f64, 0.0);
    for i in 0..255u32 {
        if x*x + y*y > 4.0 { return i; }
        let xn = x*x - y*y + cx;
        y = 2.0*x*y + cy;
        x = xn;
    }
    255
}

#[derive(Debug)]
struct Hisoblash { tarix: Vec<f64> }

impl Hisoblash {
    fn new() -> Self { Hisoblash { tarix: Vec::new() } }
    fn qo_shish(&mut self, a: f64, b: f64) -> f64 {
        let r = a + b; self.tarix.push(r); r
    }
    fn ko_paytirish(&mut self, a: f64, b: f64) -> f64 {
        let r = a * b; self.tarix.push(r); r
    }
    fn oxirgi(&self) -> Option<f64> { self.tarix.last().copied() }
    fn tarix_uzunlik(&self) -> usize { self.tarix.len() }
}

fn simulyatsiya_misoli() {

    println!("\n=== SIMULYATSIYA (std muhitida) ===");

    // WASM funksiyalar
    println!("faktorial(10) = {}", faktorial(10));
    println!("fibonacci(20) = {}", fibonacci(20));
    for i in [5u32, 10, 15, 20] {
        println!("fib({:2}) = {}", i, fibonacci(i));
    }
    // faktorial(10) = 3628800
    // fibonacci(20) = 6765

    // Class simulyatsiya
    let mut h = Hisoblash::new();
    println!("\nHisoblash class:");
    println!("3 + 4 = {}", h.qo_shish(3.0, 4.0));
    println!("5 * 6 = {}", h.ko_paytirish(5.0, 6.0));
    println!("2.5 + 7.5 = {}", h.qo_shish(2.5, 7.5));
    println!("Oxirgi: {:?}", h.oxirgi());
    println!("Tarix: {} ta", h.tarix_uzunlik());
    // 3 + 4 = 7
    // 5 * 6 = 30
    // 2.5 + 7.5 = 10
    // Oxirgi: Some(10.0)
    // Tarix: 3 ta

    // Mandelbrot ASCII
    println!("\nMandelbrot (ASCII, 40x20):");
    let chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    for py in 0..20 {
        for px in 0..40 {
            let x = (px as f64 / 40.0) * 3.5 - 2.5;
            let y = (py as f64 / 20.0) * 2.0 - 1.0;
            let iter = mandelbrot_iter(x, y);
            let ch = chars[(iter as usize * chars.len() / 256).min(chars.len()-1)];
            print!("{}", ch);
        }
        println!();
    }
}

fn wasm_framework_integratsiya() {

    println!("\n=== WASM + REACT/VUE ===\n");

    println!(r#"// React bilan WASM:
// package.json:
// {{ "dependencies": {{ "your-wasm-pkg": "^0.1.0" }} }}
//
// App.tsx:
// import init, {{ faktorial }} from 'your-wasm-pkg';
// import {{ useState, useEffect }} from 'react';
//
// function App() {{
//   const [ready, setReady] = useState(false);
//   const [natija, setNatija] = useState(0);
//
//   useEffect(() => {{
//     init().then(() => setReady(true));
//   }}, []);
//
//   const hisoblash = (n: number) => {{
//     if (ready) setNatija(faktorial(n));
//   }};
//
//   return (
//     <div>
//       <button onClick={{() => hisoblash(10)}}>10!</button>
//       <p>Natija: {{natija}}</p>
//     </div>
//   );
// }}

// Vue 3 bilan:
// import {{ defineComponent, ref, onMounted }} from 'vue';
// import init, {{ fibonacci }} from './pkg';
//
// export default defineComponent({{
//   setup() {{
//     const ready = ref(false);
//     onMounted(async () => {{
//       await init();
//       ready.value = true;
//     }});
//     const natija = (n: number) => ready.value ? fibonacci(n) : 0;
//     return {{ natija }};
//   }}
// }});"#);
}

fn main() {
    wasm_bindgen_haqiqiy();
    wasm_pack_ishlatish();
    minimal_wasm_misol();
    wasm_test_misol();
    wasm_performance();
    simulyatsiya_misoli();
    wasm_framework_integratsiya();
    println!("\n=== XULOSA ===");
    println!("WASM workflow:");
    println!("  1. Rust kod yozish (wasm-bindgen bilan)");
    println!("  2. wasm-pack build --target web");
    println!("  3. pkg/ papkasi yaratiladi (.wasm + .js + .d.ts)");
    println!("  4. JS/TS da import qilish");
    println!("  5. Browser da native tezlikda ishlaydi");
    println!();
    println!("Qoidalar:");
    println!("  #[wasm_bindgen]   — eksport belgilash");
    println!("  console_error_panic_hook — debug uchun");
    println!("  wee_alloc — kichik .wasm hajmi uchun");
    println!("  lto = true, opt-level = 'z' — release optimizatsiya");
    println!("  Batch operatsiyalar — JS↔WASM overhead kamaytirish");
}
// #================================================================================================================================================#
// # |  №  | Konstruksiya                    | Tavsif (UZ)                                | Описание (RU)                                           |
// #================================================================================================================================================#
// # |                                        WASM-BINDGEN                                                                                          |
// #================================================================================================================================================#
// # |   1 | #[wasm_bindgen]                          | Funksiya/struct eksport                    | Экспорт функции/структуры                      |
// # |   2 | #[wasm_bindgen(constructor)]             | JS constructor                             | JS конструктор                                 |
// # |   3 | JsValue                                  | JavaScript qiymati                         | Значение JavaScript                            |
// # |   4 | web_sys::Window/Document                 | Browser DOM API                            | Browser DOM API                                |
// # |   5 | js_sys::Array/Promise                    | JavaScript built-in ob'ektlar              | Встроенные объекты JavaScript                  |
// # |   6 | wasm_bindgen_futures::future_to_promise  | async → Promise                            | async → Promise                                |
// # |   7 | #[wasm_bindgen_test]                     | WASM test                                  | WASM тест                                      |
// #================================================================================================================================================#
// # |                                        TOOLCHAIN                                                                                             |
// #================================================================================================================================================#
// # |   8 | wasm-pack build                 | Paket yaratish                             | Создание пакета                                         |
// # |   9 | wasm-pack test                  | Browser/Node testlar                       | Тесты в Browser/Node                                    |
// # |  10 | wasm-pack publish               | NPM publish                                | Публикация в NPM                                        |
// # |  11 | wasm-opt                        | .wasm optimallashtirish                    | Оптимизация .wasm                                       |
// # |  12 | wee_alloc                       | Kichik WASM allocator                      | Компактный аллокатор WASM                               |
// #================================================================================================================================================#
// # |                                        TARGET                                                                                                |
// #================================================================================================================================================#
// # |  13 | wasm32-unknown-unknown          | Browser WASM                               | Browser WASM                                            |
// # |  14 | wasm32-wasi                     | WASI — OS kabi interfeys                   | WASI — интерфейс похожий на ОС                          |
// # |  15 | crate-type = ["cdylib"]         | Dynamic library (WASM)                     | Динамическая библиотека (WASM)                          |
// #================================================================================================================================================#