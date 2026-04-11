// #================================================================================================================================================#
// #                                                                 SHADOWING                                                                      #
// #                  hadowing — bir xil nomli yangi o'zgaruvchi yaratish / Shadowing — создание новой переменной с тем же именем                   #
// #                  mut dan farqi: type o'zgartirish mumkin, mut da mumkin emas / отличие от mut: можно менять тип, с mut нельзя                  #
// #================================================================================================================================================#

fn main() {

    // oddiy shadowing — bir xil nom, yangi qiymat
    // простое затенение — то же имя, новое значение
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("{}", x);
    // 12

    // shadowing — type o'zgartirish mumkin!
    // можно изменить тип!
    let x = 5;
    let x = "salom"; // i32 → &str
    println!("{}", x);
    // salom

    // scope ichida shadowing
    // затенение внутри блока
    let x = 5;
    {
        let x = x * 2; // scope ichida yangi x
        println!("ichida: {}", x); // 10
    }
    println!("tashqarida: {}", x);
    // 5 — eski x qaytdi!

    // shadowing vs mut farqi
    // разница между shadowing и mut
    let x = 5;
    let x = x + 1;     // yangi x yaratildi — shadowing
    println!("{}", x);
    // 6

    let mut y = 5;
    y = y + 1;         // bir xil y o'zgartirildi — mut
    println!("{}", y);
    // 6

    // shadowing — real hayot misoli
    // реальный пример использования
    let son = "42";          // &str
    let son: i32 = son.parse().unwrap(); // i32 ga o'tkazildi
    let son = son * 2;       // 84
    println!("{}", son);
    // 84

    // shadowing — bir nechta o'zgarish
    // несколько преобразований
    let matn = "  salom  ";
    let matn = matn.trim();
    let matn = matn.to_uppercase();
    println!("{}", matn);
    // SALOM

    // const bilan shadowing — fn ichida const yopiladi
    // затенение константы внутри функции
    const X: i32 = 5;
    {
        let y = 10;  // boshqa nom
        println!("{}", y); // 10
    }
    println!("{}", X);
    // 5
}
// #==================================================================================================================================================================#
// # |  №  | Metod / Holat               | Tavsif (UZ)                                                    | Описание (RU)                                             |
// #==================================================================================================================================================================#
// # |   1 | let x = 5; let x = x+1      | Oddiy shadowing — bir xil nom, yangi qiymat                    | Простое затенение — то же имя, новое значение             |
// # |   2 | let x = 5; let x = "salom"  | Type o'zgartirish mumkin (i32 → &str)                          | Можно изменить тип (i32 → &str)                           |
// # |   3 | mut bilan type o'zgartirish | MUMKIN EMAS! Xato beradi                                       | НЕВОЗМОЖНО! Ошибка компиляции                             |
// # |   4 | { let x = x * 2; }          | Scope ichida shadowing — tashqariga ta'sir qilmaydi            | Затенение внутри блока — не влияет снаружи                |
// # |   5 | shadowing vs mut            | Shadowing: yangi o'zgaruvchi / Mut: o'zgaruvchini o'zgartirish | Shadowing: новая переменная / Mut: изменение существующей |
// # |   6 | parse() bilan shadowing     | String → raqamga o'tkazish (real hayot misoli)                 | Преобразование String → число (реальный пример)           |
// # |   7 | trim() + to_uppercase()     | Bir nechta o'zgarishlar zanjiri                                | Цепочка нескольких преобразований                         |
// # |   8 | const bilan shadowing       | Const ni fn ichida yopish mumkin                               | Можно затенять const внутри функции                       |
// #==================================================================================================================================================================#