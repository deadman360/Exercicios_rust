use std::io;
fn main() {
    saudacoes();
}
fn saudacoes(){
    let mut opcao: String =  String::new();
    println!("Olá, qual código voce gostaria de executar ?
    \n1-Sequenica Fibonacci");
    io::stdin()
        .read_line(&mut opcao)
        .expect("Deu ruim");
    
    match opcao.trim(){
        "1" => fibonacci(),
        _ => println!("Todo")
    }
}
fn fibonacci() {
    let (mut n1, mut n2):  (i64, i64) = (0, 1);

    let mut opt: String = String::new();
    println!("Qual posição de fibonacci voce gostaria de saber?");
    io::stdin()
        .read_line(&mut opt)
        .expect("Erro!");
    let target: i32 = opt.trim().parse().expect("Numero inválido!!!");
    for _ in 0..=target{
        let n3: i64 = n1+n2;
        n1 = n2;
        n2 = n3;
    }
    println!("{}", n2);
}