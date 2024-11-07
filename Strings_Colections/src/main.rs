use std::{cmp::Ordering, io};
fn main() {
    saudacoes();
}
fn saudacoes(){
    let mut opcao: String =  String::new();
    println!("Olá, qual código voce gostaria de executar ?
    \n1-Inverte String
    \n2-Palindromo?");
    io::stdin()
        .read_line(&mut opcao)
        .expect("Deu ruim");
    
    match opcao.trim(){
        "1" => inverte_string(),
        "2" => println!("{}", palindromo()),
        _ => println!("Todo")
    }
}
fn inverte_string(){
    let mut input: String = String::new();
    println!("Qual frase voce gostaria de inverter?");
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao receber input!");
    let mut tupni: Vec<char> = Vec::with_capacity(input.len());
    for c in input.chars(){
        tupni.insert(0, c);
    }//O(n)
    println!("{}", tupni.into_iter().collect::<String>());
}
fn palindromo() -> bool{
    let mut input: String = String::new();
    println!("Qual palavra voce gostaria de saber se é um palindromo?");
    io::stdin()
        .read_line(&mut input)
        .expect("Falha!!");
        
    let mut tupni: Vec<char> = Vec::new();
    for c in input.chars(){
        tupni.insert(0, c);
    }
    match tupni.into_iter().collect::<String>().trim().cmp(&input.trim()){
        Ordering::Equal => true,
        _ => false
    }
}