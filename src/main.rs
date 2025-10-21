use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el numero!");

    println!("Dificultad: Fácil= F; Normal= N; Difícil= D");
    let mut dificultad = String::new();
    io::stdin().read_line(&mut dificultad)
        .expect("Failed to read line");

    let cantidad = if dificultad.trim().eq_ignore_ascii_case("F") {
        5
    } else if dificultad.trim().eq_ignore_ascii_case("N") {
        25
    } else if dificultad.trim().eq_ignore_ascii_case("D") {
        100
    } else {
        10
    };

    let mut vidas = if cantidad == 5{
        100
    }else if cantidad==10 {
        10
    }else if cantidad==100{
        5
    }else {
        4
    };

    let num_aleatorio = rand::thread_rng().gen_range(1..=cantidad);
    println!("Dificultad elegida: {}", dificultad.trim());
    println!("Máximo número posible: {}", cantidad);
    println!("Número de vidas: {}", vidas);

    let mut repetir = true;

    while repetir && vidas>0 {
        println!("\nIngresa tu numero: ");

        let mut tu_numero = String::new();
        io::stdin().read_line(&mut tu_numero)
            .expect("Error al leer la línea");


        let tu_numero:u32 = match tu_numero.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ingrese un valor válido, por favor");
                continue;
            }
        };

        if tu_numero > cantidad {
            println!("Tu número está fuera de los límites (1-{}", cantidad);
            continue;
        }

        let diferencia = if tu_numero > num_aleatorio {
            tu_numero - num_aleatorio
        } else {
            num_aleatorio - tu_numero
        };
        let porcentaje = (diferencia as f32 / num_aleatorio as f32) * 100.0;

        //Herramienta para que diga la aproximación
        match tu_numero.cmp(&num_aleatorio) {
            Ordering::Less => {
                if porcentaje <= 10.0 {
                    println!("¡Muy cerca! (+)");
                } else {
                    println!("Demasiado pequeño");
                }
            },
            Ordering::Greater => {
                if porcentaje <= 10.0 {
                    println!("¡Muy cerca! (-)");
                } else {
                    println!("Demasiado grande");
                }
            },
            Ordering::Equal => {
                println!("¡Ganaste!");
                println!("Fin del juego");
                break;
            }
        }

        vidas-=1;
        println!("Vidas restantes: {}",{vidas});
        if vidas == 0 {
            println!("Te quedaste sin vidas. El número era {}. Fin del juego.", num_aleatorio);
            break;
        }

        // Preguntar si quiere continuar
        println!("¿Quieres volver a intentarlo? (Si/No)");
        let mut duda = String::new();
        io::stdin().read_line(&mut duda)
            .expect("Error al leer la línea");

        if duda.trim().eq_ignore_ascii_case("No") {
            println!("Sin resentimientos. Fin del juego.");
            repetir = false;
        } else if duda.trim().eq_ignore_ascii_case("Si") {
            println!("¡Intenta otra vez!");
        } else {
            println!("Respuesta no válida. Terminando el juego.");
            repetir = false;
        }
    }
}