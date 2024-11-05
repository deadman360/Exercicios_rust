use std::io;
fn main() {
    saudacoes();
}
fn saudacoes(){
    let mut opcao: String =  String::new();
    println!("Olá, qual código voce gostaria de executar ?
    \n1-Calculadora
    \n2-Conversor de Temperatura
    \n3-Par ou Impar ?
    \n4- Gerar Tabuada");
    io::stdin()
        .read_line(&mut opcao)
        .expect("Deu ruim");
    
    match opcao.trim(){
        "1" => calculadora(),
        "2" => conversor_temperatura(),
        "3" => par_impar(),
        "4" => tabuada(),
        _ => println!("Todo")
    }
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
        
    let n1: f64 = input.trim().parse().expect("Falha interna");
    let n2: f64 = input2.trim().parse().expect("Falha interna");
    match operacao.trim() {
        "1" => println!("{:.2}",n1 + n2,),
        "2" => println!("{:.2}",n1 - n2),
        "3" if n2 !=0f64 => println!("{:.2}",n1 / n2),
        "4" => println!("{:.2}",n1 * n2),
        _ => println!("Operação invalida")
         
    }
}
fn conversor_temperatura(){
    let mut opt: String = String::new();
    let mut tmp: String = String::new();

    println!("Olá, qual operecação deseja realizar
    \n 1- Celsius -> Farenheit
    \n 2- Farenheit -> Celsius");
    io::stdin()
        .read_line(&mut opt)
        .expect("Erro");
    
    println!("Quanto é a temperatura?");
    io::stdin()
        .read_line(&mut tmp)
        .expect("Erro!");
    let tmp_n: f64 = tmp.trim().parse().expect("Erro");
    match opt.trim() {
        "1" => println!("Temperatura em Farenheit = {:.3}º", (tmp_n*9.0/5.0)+32f64),
        "2" => println!("Temperatura em Celsius = {:.3}º",(tmp_n - 32f64) * 5.0/9.0),
        _ => println!("Operação inválida")

    }
}
fn par_impar(){
    println!("Quer número gostaria de verificar ?");
    let mut opt: String = String::new();
    io::stdin()
        .read_line(&mut opt)
        .expect("Erro!");
    let num: i32 = opt.trim().parse().expect("Numero Invalido!!");
    match num%2 {
        1 => println!("Impar"),
        0 => println!("Par"),
        _ => println!("Unknown")
    } 
}
fn tabuada(){
    let mut opt: String = String::new();
    println!("De qual numero gostaria de saber a tabuada ?");
    io::stdin()
        .read_line(&mut opt)
        .expect("Erro!");

    let num: i32 = opt.trim().parse().expect("Numero Invalido!!!!");
    for i in 0..=10{
        println!("{} x {} = {}", num, i, num * i);
    }
}