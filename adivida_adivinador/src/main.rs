use std::io;


fn main() {
    println!("¡Adivina el número!");

    println!("Ingresa tu estimación: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("ERROR al leer la línea");
    
    println!("Tu estimación>> {}", guess);
}
