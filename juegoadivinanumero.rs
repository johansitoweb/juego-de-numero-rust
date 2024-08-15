use rand::Rng;
use std::io;

fn main() {
    println!("¡Bienvenido al juego de adivinanza de números!");

    let secreto = rand::thread_rng().gen_range(1..101);
    let mut intentos = 0;

    loop {
        println!("Por favor, ingresa tu adivinanza (entre 1 y 100):");

        let mut adivinanza = String::new();
        io::stdin()
            .read_line(&mut adivinanza)
            .expect("Error al leer la línea");

        let adivinanza: u32 = match adivinanza.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor, ingresa un número válido.");
                continue;
            }
        };

        intentos += 1;

        if adivinanza < secreto {
            println!("Demasiado bajo. Intenta de nuevo.");
        } else if adivinanza > secreto {
            println!("Demasiado alto. Intenta de nuevo.");
        } else {
            println!("¡Felicidades! Adivinaste el número en {} intentos.", intentos);
            break;
        }
    }
}
