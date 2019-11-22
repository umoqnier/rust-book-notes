use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    /*println! es una macros por el signo de admiración*/
    println!("¡Adivina el número entre 1 y 100!");
    
    /* Generación de números aleatorios. La función thread_rnd() da el método
     * de generación de números (usando el hilo actual del sistema) y el método
     * gen_range() toma un rango y devuelve un número aleatorio en ese rango.*/
    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        println!("Ingresa tu estimación: ");
        
    /* Creación de variables con let. Las variables ne rust son INMUTABLES por
     * defecto. Para crear una variable mutable se utiliza mut. String está
     * definido en la biblioteca estandar. Codifica bits de texto UTF-8. La
     * función asociada new (se conoce como método estático en otros lenguajes)
     * crea una cadena vacía*/
        let mut guess = String::new();

    /* Funcionalidad de entrada/salida (I/O) a través de la
     * función io que está en la biblioteca estandar definida al inicio de este
     * archivo. Se llama a la función stdin() que maneja la entrada de la
     * usuaria a través del "standard input terminal". read_line() lee la
     * entrada y la guarda en un string. En este caso es la variable guess.
     * El signo & indica que se está pasando una *referencia*. Las referencias
     * permiten al código acceder a datos son necesidad de copiarlos en memoria
     * varias veces. Las referencias al igual que las variables son inmutables
     * por defecto. Por esta razón tambien se agrega mut al inicio.
     *
     * read_line() ademas de guardar la entrada retorna una *enumeración* del
     * tipo io::Result. Una enumeración es un tipo de dato que contiene un
     * conjunto de valores fijos y estos valores son conocidos como enum's
     * variants. Para Result se tienen dos variantes que son Ok (la operación
     * fué exitosa) y Err (la operación falló). Result tiene el método expect()
     * y si la instancia de Result es un Err el programa parará y mostrara el 
     * mensaje que se le pase al expect(). Por otra parte, si la instancia es
     * un Ok expect() regresará el valor que contiene Ok. En este caso el
     * número de bytes que introdujo la usuaria. Si no se llama .expect() el 
     * programa compilará pero mostrará una advertencia indicando que no se
     * está manejando un posible error.*/
        io::stdin().read_line(&mut guess)
            .expect("ERROR al leer la línea");

    /* La función trim() remueve espacios en blanco y caracteres especiales
     * (como el salto de línea \n). La función parse() convierte la cadena a
     * un número entero sin signo de 32 bits. parse() retorna la enumeración
     * del tipo Result (como read_line). Por lo tanto debe manejarse un
     * posible error. En este caso se utiliza match para el manejo. Si la
     * entrada es valida se regresa el número. Si es invalida se ignora
     * con la palabra reservada continue.*/ 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    /* Impresión del número introducido por la usuaria. Las llaves {} es un
     * marcador de posición y pueden verse como si fueran las tenasas de un
     * cangrejito que mantienen el valor en su lugar.*/
        println!("Tu estimación>> {}", guess);

    /* Ordering es otra enumeración que tiene las variantes Less, Greater y
     * Equal que son las posibilidades cuando se comparan dos números. La
     * llamada al método cmp puede ser realizada por cualquier cosa que pueda
     * ser comparada. Se le pasa una referencia (que es la del número
     * aleatorio). cmp() retorna una variante de la enumeración Ordering. Se
     * utiliza la expresión match para decidir que se hará después dependiendo
     * del tipo de variante de Ordering que se tenga.*/
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muy bajo"),
            Ordering::Greater => println!("Muy alto"),
            Ordering::Equal => {
                println!("Ganaste!!! :-)");
                break;
            }
        }
    }
}


