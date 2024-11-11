use std::io;
fn main() {
    saudacoes();
}
struct Retangulo {
    base: u32,
    altura: u32,
}
impl Retangulo {
    fn area(&self) -> u32 {
        self.base * self.altura
    }
    fn perimetro(&self) -> u32 {
        self.base + self.altura
    }
}
fn saudacoes() {
    let mut opcao: String = String::new();
    println!(
        "Olá, qual código voce gostaria de executar ?
    \n1-Calcular area de triangulo"
    );
    io::stdin().read_line(&mut opcao).expect("Deu ruim");

    match opcao.trim() {
        "1" => menu_retangulo(),
        _ => println!("Todo"),
    }
}
fn menu_retangulo() {
    let mut opcao: String = String::new();
    println!(
        "Olá, qual código voce gostaria de executar ?
    \n1-Area
    \n2-Perimetro"
    );
    io::stdin().read_line(&mut opcao).expect("Deu ruim");
    let mut retangulo_input: String = String::new();
    println!("Agora me informe a base e a altura do seu triangulo no formato \"base, altura \"");
    io::stdin()
        .read_line(&mut retangulo_input)
        .expect("Deu ruim");
    let retangulo_arr: Vec<&str> = retangulo_input.split(",").collect();
    let retangulo: Retangulo = Retangulo {
        base: retangulo_arr[0].trim().parse().expect("base invalida"),
        altura: retangulo_arr[1].trim().parse().expect("altura invalida"),
    };
    match opcao.trim() {
        "1" => println!("{}", retangulo.area()),
        "2" => println!("{}", retangulo.perimetro()),
        _ => println!("Todo"),
    }
}
