use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("¡Adivina el número entre 1 y 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Ingresa tu estimación: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("ERROR al leer la línea");
   
    let guess: u32 = guess.trim().parse()
        .expect("Solo se aceptan numeros. ¿Me quieres ver la cara de estúpida?");

    println!("Tu estimación>> {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Muy bajo"),
        Ordering::Greater => println!("Muy alto"),
        Ordering::Equal => println!("Ganaste!!! :-)"),
    }
}


