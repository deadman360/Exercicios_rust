use std::{cmp::Ordering, io};
fn main() {
    saudacoes();
}
fn saudacoes() {
    let mut opcao: String = String::new();
    println!(
        "Olá, qual código voce gostaria de executar ?
    \n1-Inverte String
    \n2-Palindromo?
    \n3-Remover Duplicadas
    \n4-Contador de palavra
    \n5-Bubble sort"
    );
    io::stdin().read_line(&mut opcao).expect("Deu ruim");

    match opcao.trim() {
        "1" => inverte_string(),
        "2" => println!("{}", palindromo()),
        "3" => remove_duplicadas(),
        "4" => conta_palavra(),
        "5" => bubble_sort(),
        _ => println!("Todo"),
    }
}
fn inverte_string() {
    let mut input: String = String::new();
    println!("Qual frase voce gostaria de inverter?");
    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao receber input!");
    let mut tupni: Vec<char> = Vec::with_capacity(input.len());
    for c in input.chars() {
        tupni.insert(0, c);
    } //O(n)
    println!("{}", tupni.into_iter().collect::<String>());
}
fn palindromo() -> bool {
    let mut input: String = String::new();
    println!("Qual palavra voce gostaria de saber se é um palindromo?");
    io::stdin().read_line(&mut input).expect("Falha!!");

    let mut tupni: Vec<char> = Vec::new();
    for c in input.chars() {
        tupni.insert(0, c);
    }
    match tupni
        .into_iter()
        .collect::<String>()
        .trim()
        .cmp(&input.trim())
    {
        Ordering::Equal => true,
        _ => false,
    }
}
fn remove_duplicadas() {
    let mut input: String = String::new();
    println!("Me informe qual lista voce gostaria de remover as duplicadas");
    io::stdin().read_line(&mut input).expect("Erro!");
    let lista: Vec<&str> = input.trim().split(",").collect();
    let mut nova_lista: Vec<&str> = Vec::new();
    for st in lista {
        if !nova_lista.contains(&st.trim()) {
            nova_lista.push(st);
        } else {
            continue;
        }
    }
    println!("{}", nova_lista.join(" | "))
}
fn conta_palavra() {
    let mut input: String = String::new();
    let mut target: String = String::new();
    println!("Qual texto voce gostaria que eu contasse as palavras?");
    io::stdin().read_line(&mut input).expect("Erro!");
    println!("Qual palavra gostaria que eu contasse?");
    io::stdin().read_line(&mut target).expect("Erro!!");
    let mut contador: i32 = 0;
    for word in input.split(" ").collect::<Vec<&str>>() {
        match word
            .to_lowercase()
            .trim()
            .cmp(&target.to_lowercase().trim())
        {
            Ordering::Equal => contador += 1,
            _ => continue,
        }
    }
    println!("Esse texto tem a palavra {} {}x vezes", target, contador);
}
fn bubble_sort() {
    let mut input: String = String::new();
    println!("Que lista voce gostaria que fosse ordenada?");
    io::stdin().read_line(&mut input).expect("Erro!");
    let mut lista: Vec<&str> = input.trim().split(",").collect();
    let mut trocado: bool = true;
    while trocado {
        trocado = false;
        for index in 0..lista.len() - 1 {
            if lista[index].cmp(&lista[index + 1]) == Ordering::Greater {
                lista.swap(index, index + 1);
                trocado = true
            }
        }
    }
    println!("lista arrumada = {}", lista.join(", "));
}
