fn main() {
    println!("{}", ex1('x', String::from("ext4 is awesomeX")));
    ex2();
    println!("{}",ex3(String::from("n to gritando")));
    println!("{:?}",ex4(String::from("HEllo, World")));
    ex5();
    println!("{}",ex6(String::from("Bbabuble")));
    println!("{}", ex7(String::from("S t ri n g com espa ço e tt oo da f orma ta da")));
    // ex8();
    // ex9();
    // ex10();
}
fn ex1(c: char, s: String) -> u8{
    /*1. Crie uma função que receba uma String e um char como parâmetros
    e retorne a quantidade de vezes que o caractere aparece*/ 
    let mut contador: u8 = 0;
    for i in s.chars() {
        if i.to_lowercase().next().unwrap() == c || i.to_uppercase().next().unwrap() == c{
            contador += 1;
        }
        // nesse caso usei o metodo next que volta o proximo caracter do iterador
        //funciona só para chars que são um unico byte

    }
    contador
}
fn ex2() {
    /*Escreva um programa que declare uma String com o texto "APRENDER RUST É DIVERTIDO" 
    e imprima seu comprimento*/
    let frase: String = String::from("APRENDER RUST É DIVERTIDO");
    println!("{}", frase.capacity())
    //ele retorna 26 devido ao acento no e que conta como caracter
}
fn ex3(s: String) -> String {
    /*3. Implemente uma função que receba uma String e retorne uma nova
    String com todas as letras Maíusculas */
    s.to_uppercase()

}
fn ex4(s: String) -> Vec<char> {
    let vogais: String = String::from("aeiouAEIOU");
    return s.chars().filter(|c| !vogais.contains(*c)).collect::<Vec<char>>();
    
}
fn ex5(){
    /*Escreva um programa que declare uma string vazia e concatene-a com A
    a String "Olá," e com a String "Mundo!". */
    let vazia: String = String::new();
    println!("{}", vazia + "Olá,"/*slice of string , enunciado errado */ +" Mundo!" );
}
fn ex6(s: String) ->String {
    /*Implemente uma funçãp que receba uma String
    e retorne uma nova String com os caracteres invertidos */
    s.chars().rev().collect()
}
fn ex7(s: String) -> String {
    /*Crie um função que receba uma String e retorne uma nova String com todos os espaços removidos */
    String::from(s.replace(" ", ""))
}