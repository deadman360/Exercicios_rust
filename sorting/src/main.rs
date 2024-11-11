use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::time::Instant;
fn main() {
    let mut lista_desordenada: Vec<i32> = (1..10000).collect::<Vec<i32>>();
    lista_desordenada.shuffle(&mut thread_rng());

    let inicio = Instant::now();
    bubble_sort(&mut lista_desordenada.clone());
    println!("Bubble sort = {:?}", inicio.elapsed());

    insertion_sort(&mut lista_desordenada.clone());
}
fn bubble_sort(lista: &mut Vec<i32>) {
    let mut trocado: bool = true;
    let mut iter_num: i32 = 0;
    while trocado {
        trocado = false;
        for index in 0..lista.len() - 1 {
            if lista[index].cmp(&lista[index + 1]) == Ordering::Greater {
                lista.swap(index, index + 1);
                trocado = true;
            }
        }
        iter_num += 1;
    }
    println!(
        "{:?}, {} iterações para {} elementos",
        lista.is_sorted(),
        iter_num,
        lista.len()
    );
}
fn insertion_sort(lista: Vec<i32>) {}
