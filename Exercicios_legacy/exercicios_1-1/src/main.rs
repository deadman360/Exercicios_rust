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
    // 1. Crie um programa que declare uma variável mutável do tipo i32 e a incremente em 3.
    let mut num: i32 = 0;
    num += 3;
    println!("{}", num);
}
fn ex2() {
    // 2. Declare duas variáveis imutáveis, uma do tipo f64 e outra do tipo bool. Imprima seus valores.
    let num: f64 = 10f64;
    let var: bool = true;
    println!("f64: {}, bool: {}", num, var);
}
fn ex3() {
    // 3. Escreva um programa que declare uma variável String e a imprima.
    let st: String = String::from("Hello, Rust");
    println!("{}", st);
}
fn ex4() {
    // 4. Crie uma variável mutable chamada "contador" e incremente seu valor em 5.
    let mut contador: u8 = 0;
    contador += 5;
    println!("{}", contador);
}
fn ex5() {
    // 5. Declare uma variável mutável do tipo u8 e atribua o valor máximo possível para este tipo.
    let uns: u8 = 255;
    println!("{}", uns);
    // variavel nao precisou ser mutavel
}
fn ex6() {
    // 6. Escreva um programa que declare uma variável do tipo char e a imprima.
    let letra: char = 'a';
    println!("{}", letra)
}
fn ex7() {
    // 7. Crie uma variável mutável do tipo i16 e decremente seu valor em 2.
    let mut dezesseis: i16 = 0;
    dezesseis -= 2;
    println!("{}", dezesseis);
}
fn ex8() {
    // 8. Declare duas variáveis do tipo f32, multiplique-as e imprima o resultado.
    let n1: f32 = 10f32;
    let n2: f32 = 4f32;
    println!("{}", n1 * n2);
}
fn ex9() {
    // 9. Escreva um programa que declare uma variável String com valor vazio.
    let st: String = String::new();
    println!("{}", st);
}
fn ex10() {
    // 10. Crie uma variável mutável do tipo usize e incremente seu valor em 4.
    let mut tamanho: usize = 4;
    tamanho += 4;
    println!("{}", tamanho);
}
