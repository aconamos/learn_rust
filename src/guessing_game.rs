use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing game!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Input a guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Fuck you!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("That's it! You win!");
                break;
            }
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
        }
    }
}
