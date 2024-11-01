fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();
    ex6();
    ex7();
    ex8();
    ex9();
    ex10();
}
fn ex1() {
    // 1. Declare uma variável do tipo i64 com o valor 100_000 e outra do tipo u32 com o valor 50. Imprima a soma das duas.
    let num: i64 = 100_000;
    let num2: u32 = 50;
    println!("{}", num + num2 as i64);
}
fn ex2() {
    // 2. Crie uma variável f64 com o valor 3.14 e outra i32 com o valor 7. Multiplique-as e imprima o resultado.
    let pi: f64 = 3.14;
    let mult: i32 = 7;
    println!("{}", pi * mult as f64);
}
fn ex3() {
    /* 3. Escreva um programa que declare uma variável booleana
    com valor true e outra com valor false. Imprima o resultado da operação lógica AND entre elas. */
    let verd: bool = true;
    let fal: bool = false;
    println!("{}", verd && fal);
}
fn ex4() {
    /*Declare uma variável String com o texto "Hello, World!"
    e outra char com o valor 'x'. Concatene-as e imprima o resultado. */
    let hello: String = String::from("Hello, World!");
    let letra: char = 'x';
    println!("{}", hello + letra.to_string().as_str());
}
fn ex5() {
    /*Crie uma variável i16 com valor -20 e outra u16 com valor 30. Subtraia a segunda da primeira e imprima
    o resultado. */
    let negative: i16 = -20;
    let positive: u16 = 30;
    println!("{}", negative - positive as i16);
}
fn ex6() {
    /*Escreva um programa que declare uma variável f32 com valor 2.5 e outra f32 com valor 3.7.
    Divida a primeira pela segunda e imprima o resultado com 3 casas decimais. */
    let n1: f32 = 2.5;
    let n2: f32 = 3.7;
    println!("{:.3}", n1 / n2);
}
fn ex7() {
    /*Declare uma variável u8 com valor 250 e outra i8 com valor -120.
    Imprima o resultado da operação de módulo entre elas. */
    let unsigned: u8 = 250;
    let signed: i8 = -120;
    println!("{}", unsigned as i8 % signed);
}
fn ex8() {
    /*Crie uma variável String com o texto "Rust é awesome!" e outra char com o valor 'a'.
    Conte quantas vezes o caractere aparece na String e imprima o resultado. */
    let letra: char = 'a';
    let frase: String = String::from("Rust is awesome!");
    let mut contador: u8 = 0;
    for i in frase.chars() {
        if i == letra {
            contador += 1
        }
    }
    println!("{}", contador);
}
fn ex9() {
    /*Declare uma variável i64 com valor 1_000_000 e outra u64 com valor 2_000_000. Imprima a soma das duas. */
    let milhao: i64 = 1_000_000;
    let milhao2: u64 = 2_000_000;
    println!("{}", milhao as u64 + milhao2);
}
fn ex10() {
    /*Escreva um programa que declare uma variável bool com valor false e outra bool com valor true.
    Imprima o resultado da operação lógica OR entre elas. */
    let tr: bool = true;
    let fl: bool = false;
    println!("{}", tr || fl);
}
