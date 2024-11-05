use std::io;
fn main() {
    saudacoes();
    calculadora();
}
fn saudacoes(){
    println!("Hello, World");
}
fn calculadora(){
    let mut input: String = String::new();
    let mut input2: String = String::new();
    let mut operacao: String = String::new();
    println!("Digite um número");
    io::stdin()
    .read_line(&mut input)
    .expect("Falha ao ler o arquivo");    
        
    println!("Digite outro número");
    io::stdin()
            .read_line(&mut input2)
            .expect("Falaha ao ler sua entrada");
        
    println!("\t Qual operação desejada(1,2,3,4) ?\n\t\t1-Adicionar\n\t\t2-Dubtrair\n\t\t3-Dividir\n\t\t4-Multiplicar");
    io::stdin()
            .read_line(&mut operacao)
            .expect("Falaha ao ler sua entrada");
        
    let n1: i32 = input.trim().parse().expect("Falha interna");
    let n2: i32 = input2.trim().parse().expect("Falha interna");
    match operacao.trim() {
        "1" => println!("{}",n1 + n2,),
        "2" => println!("{}",n1 - n2),
        "3" => println!("{}",n1 / n2),
        "4" => println!("{}",n1 * n2),
        _ => println!("Operação invalida")
    }
}
