fn main() {
    println!("{}",ex1(8));
    println!("{}", ex2('0'));
    println!("{}",ex3(20193846));
    println!("{}",ex4(String::from("")));

}
fn ex1(dia: u8) -> &'static str{
    /*receba uma função que receba um numero inteiro e use um 
    match para retornar o dia correspondente */
    match dia{
        1 => "domingo",
        2 => "segunda",
        3 => "terça",
        4 => "quarta",
        5 => "quinta",
        6 => "sexta",
        7 => "sábado",
        _ => " dia desconhecido"
    }

}
fn ex2(c: char) -> &'static str{
    /*Crie um programa que declare uma variaável char e use match
     para imprimir se o caractere é uma vogal, consoante ou outro
     tipo de caracter*/
     
    match c.to_string().to_lowercase().chars().next().unwrap(){
        'a' | 'e' | 'i' | 'o' | 'u' => "vogal",
        x if x.is_alphabetic() => "Consoante",
        _ => "outro caractere"
    }
}
fn ex3(n: i64) -> &'static str{
    /*Impleme uma função que receba um número inteiro e use match para retornar
     se o número é par, impár ou zero ? */
     match n {
         0 => "Zero",
        x if x%2 == 0 => "par",
        x if x%2 == 1 => "ìmpar",
        _ => "numero desconhecido"
     }
}
fn ex4(s: String) ->&'static str {
    /*Escreva um programa que declare uma varável String
    e use match para imprimir se a String esá vaiz, tem
    um único caractere ou tem mpultimplos caracteres*/
    match s.len(){
        0 => "vazia",
        1 => " um unico carater",
        _ => "multiplos caracteres"
    }
}