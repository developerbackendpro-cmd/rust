// #================================================================================================================================================#
// #                                              MANTIQIY OPERATORLAR INTEGER TURARI UCHUN                                                         #
// #                                               ЛОГИЧЕСКИЕ ОПЕРАТОРЫ ЦЕЛОЧИСЛЕННЫХ ТИПОВ                                                         #
// #================================================================================================================================================#

fn main() {

    // 1. AND (&&) - HAMMA shart bajarilsa
    // 1. AND (&&) - ЕСЛИ ВСЕ условия выполнены
    let yosh = 20;
    let pul = 500;
    let mashina_oladi = yosh > 18 && pul >= 1000;
    println!("1) -► {yosh} yosh va {pul} pul: {mashina_oladi}");
    // false

    // 2. OR (||) - BITTA shart bajarilsa
    // 2. OR (||) - ЕСЛИ ХОТЯ БЫ ОДНО условие выполнено
    let a = 18;
    let b = 1500;
    let mashina_oladi = a > 20 || b > 1000;
    println!("2) -► {a} yosh yoki {b} pul: {mashina_oladi}");
    // true

    // 3. NOT (!) - SHARTNI teskarisiga o'giradi
    // 3. NOT (!) - ИНВЕРСИЯ условия
    let talaba = true;
    let a = !talaba;
    println!("3) -► Talaba emas: {a}");
    // false

    // 4. XOR (^^) - FAQAT BITTA shart bajarilsa (Rustda yo'q, lekin mantiq bor)
    // 4. XOR (^^) - ЕСЛИ ТОЛЬКО ОДНО условие выполнено (нет в Rust)
    // Rustda XOR uchun != (teng emas) ishlatiladi
    let x = true;
    let y = false;
    let xor_natija = x != y;
    println!("4) -► XOR (true != false): {xor_natija}");
    // true

    // 5. NAND - AND dan keyin NOT (HAMMA shart bajarilsa, teskarisi)
    // 5. NAND - AND затем NOT (ЕСЛИ ВСЕ условия выполнены, инверсия)
    let a = true;
    let b = true;
    let nand_natija = !(a && b);
    println!("5) -► NAND (true && true dan keyin !): {nand_natija}");
    // false
}
// #================================================================================================================================================#
// # |  №  | Operator                | Tavsif (UZ)                                          | Описание (RU)                                         |
// #================================================================================================================================================#
// # |   1 | &&                      | AND (HAMMA shart bajarilsa)                          | И (ЕСЛИ ВСЕ условия выполнены)                        |
// # |   2 | ||                      | OR (BITTA shart bajarilsa)                           | ИЛИ (ЕСЛИ ХОТЯ БЫ ОДНО условие выполнено)             |
// # |   3 | !                       | NOT (teskarisiga o'giradi)                           | НЕ (инверсия)                                         |
// # |   4 | !=                      | XOR sifatida ishlatiladi (faqat bittasi true bo'lsa) | Используется как XOR (если только одно true)          |
// #================================================================================================================================================#
