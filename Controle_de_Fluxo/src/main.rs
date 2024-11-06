use std::{cmp::Ordering, io};
use rand::random;
fn main() {
    saudacoes();
}
fn saudacoes(){
    let mut opcao: String =  String::new();
    println!("Olá, qual código voce gostaria de executar ?
    \n1-Sequenica Fibonacci
    \n2-Contador de Vogais
    \n3-Jogo de advinhação
    \n4-Fatorial
    \n5-Contador de ocorrências");
    io::stdin()
        .read_line(&mut opcao)
        .expect("Deu ruim");
    
    match opcao.trim(){
        "1" => fibonacci(),
        "2" => conta_vogal(),
        "3" => guessing_game(),
        "4" => fatorial(),
        "5" => conta_ocorrencias(),
        _ => println!("Todo")
    }
}
fn fibonacci() {
    let (mut n1, mut n2):  (u128, u128) = (0, 1);

    let mut opt: String = String::new();
    println!("Qual posição de fibonacci voce gostaria de saber?");
    io::stdin()
        .read_line(&mut opt)
        .expect("Erro!");
    let target: i32 = opt.trim().parse().expect("Numero inválido!!!");
    for _ in 1..=target{
        let n3: u128 = n1+n2;
        n1 = n2;
        n2 = n3;
    }
    println!("{}", n1);
}
fn conta_vogal() {
    let vogais: [char; 5] = ['a','e','i','o','u']; 
    let mut input: String = String::new();
    let mut contador: u16 = 0;
    println!("Qual frase você gostaria de contar as vogais?");
    //Pega input
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a string!!!");
    
    for c in input.to_lowercase().chars(){
        if vogais.contains(&c){
            contador += 1;
        }   
    }
    println!("{} vogais", contador);
    
    
}
fn guessing_game() {
    let numero_premiado: u8 = random::<u8>();
    let mut input: String = String::new();
    let mut opt: String = String::new();
    
    println!("Olá, Vamos Jogar ?
    \n estou pensando em um número de 1 a 255, tente advinhar");
    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao receber input");
    
    let chute: u8 = input.trim().parse().expect("Número inválildo!!!");
    match chute.cmp(&numero_premiado) {
        Ordering::Equal => println!("Acertou!!!!!"),
        _ => println!("Não foi dessa vez, o numero que estava pensando era {}", numero_premiado)
    }
    println!("Deseja tentar novamente ?(S/N)");
    io::stdin()
        .read_line(&mut opt)
        .expect("Falha ao ler a opção!");

    match opt.to_lowercase().trim() {
        "s" => guessing_game(),
         _ => println!("saindo...")
    }
}
fn fatorial(){
    let mut input: String = String::new();
    println!("Me informe qual número gostaria de saber o valor do fatorial?");
    io::stdin()
        .read_line(&mut input)
        .expect("Erro!");
    
    let num: u64 = input.trim().parse().expect("Numero inválido!!!");
    let mut fatorial: u64 = 1;
    for i in 1..=num{
        fatorial *= i;
    }
    println!("o fatorial de {} é = {}",num, fatorial)
}
fn conta_ocorrencias(){
    let mut input: String = String::new();
    println!("Me informe uma lista de numeros, separados por virgulas");
    io::stdin()
        .read_line(&mut input)
        .expect("Erro na conversão");

    let lista: Vec<&str> = input.trim().split(",").collect();
    println!("Agora me informe qual número voce gostaria que eu contasse as ocorrencias na lista");
    let mut target: String = String::new();
    io::stdin()
        .read_line(&mut target)
        .expect("Erro!");

    let mut contador: i32 = 0;
    for num in lista{
        match num.cmp(&target.trim()) {
            Ordering::Equal => contador +=1,
            _ => continue            
        }
    }
    println!("O número {} aparece {}x vezes", target, contador);
}
