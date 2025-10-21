use std::io;
use rand::Rng;
fn main() {
    println!("Adivina el número!");
    let num_aleatorio:u32 = rand::thread_rng().gen_range(1..=5);
    let mut my_num = String::new();
    io::stdin().read_line(&mut my_num)
    .expect("Failed to read line");
    let my_num:u32 = match my_num.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("Error");return;}
    };
    println!("El numero aleatorio era: {num_aleatorio}");
    if num_aleatorio==my_num {
        println!("El numero es correcto!");
    } else{
        println!("El numero es incorrecto!");
    }
}
