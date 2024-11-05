fn main() {
    println!("{}", ex1('x', String::from("ext4 is awesomeX")));
    ex2();
    println!("{}",ex3(String::from("n to gritando")));
    println!("{:?}",ex4(String::from("HEllo, World")));
    ex5();
    println!("{}",ex6(String::from("Bbabuble")));
    println!("{}", ex7(String::from("S t ri n g com espa ço e tt oo da f orma ta da")));
    ex8();
    println!("{}",ex9(String::from("    isso aqui deve estar com as primeiras maiusculas")));   // ex10();
    println!("{}", ex10(String::from("babaluebir"), 'i').unwrap());
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
    //função quer recebe uma string e retorna uma nova string sem as vogais
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
fn ex8() {
    /*Escreva um programa que declare uma String com o texto "  Rust é  incrivel  " 
    e utilize metodos para remover os espaços em branco do inicio e do fim */
    let frase: String = String::from("  Rust é incrível  ");
    println!("{}", frase.trim());
}
fn ex9(s:  String) -> String { 
    //Implemente uma função que receba um String e retorne uma nova String com a primeira letra de cada palavra em maiuscula
    let mut ns: String= String::new();
    for (i, c) in s.trim().chars().enumerate(){
        if i == 0 {
            ns = ns + c.to_string().to_uppercase().as_str();
        }else if s.trim().chars().nth(i - 1).unwrap() == ' '{  
            ns = ns + c.to_string().to_uppercase().as_str();
        }else {
            ns = ns + c.to_string().as_str()
        }
    }   
    ns
}
fn ex10(s: String, c: char) -> Option<usize> {
    //funçao que recebe uma string e um char e retorna a posicção do char
    for (index, character) in s.chars().enumerate(){
        if c == character {
            return Some(index)
        }
    }
    None
}