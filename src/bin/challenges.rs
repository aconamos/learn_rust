use std::io;

mod challenges {
    pub mod ch3;
}

use challenges::ch3;

fn main() {
    let mut input = String::new();

    println!(
        "Enter a challenge function: (1-3) 
            1. Celsius to Fahrenheit
            2. Fahrenheit to Celsius
            3. Fibonacci sequence"
    );

    print!("> ");

    match loop {
        match io::stdin().read_line(&mut input) {
            Err(_) => {
                println!("Try again!");
                continue;
            }
            _ => (),
        }

        match input.trim().parse() {
            Ok(num) => match num {
                1 => break num,
                2 => break num,
                3 => break num,
                _ => println!("Please enter a number in range!"),
            },
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        }
    } {
        1 => do_celsius(),
        2 => do_fahrenheit(),
        3 => do_fib(),
        _ => panic!("How did we get here?"),
    }
}

fn do_celsius() {
    let mut user_temperature_in = String::new();

    println!("Input a value in celsius:");

    io::stdin()
        .read_line(&mut user_temperature_in)
        .expect("Please input a valid input!");

    let user_temperature_in = match user_temperature_in.trim().parse() {
        Ok(num) => {
            if num < -273.15 {
                println!("You're weird.");
                -273.15
            } else {
                num
            }
        }
        Err(_) => {
            println!("Error parsing! Giving up.");
            return;
        }
    };

    let temp = ch3::celsius_to_fahrenheit(user_temperature_in);

    println!("{user_temperature_in} C is {temp} fahrenheit.");
    println!("Going back: {} C", ch3::fahrenheit_to_celsius(temp));
}

fn do_fahrenheit() {
    let mut user_temperature_in = String::new();

    println!("Input a value in fahrenheit:");

    io::stdin()
        .read_line(&mut user_temperature_in)
        .expect("Please input a valid input!");

    let user_temperature_in = match user_temperature_in.trim().parse() {
        Ok(num) => {
            if num < -459.66998 {
                println!("You're weird.");
                -459.66998
            } else {
                num
            }
        }
        Err(_) => {
            println!("Error parsing! Giving up.");
            return;
        }
    };

    let temp = ch3::fahrenheit_to_celsius(user_temperature_in);

    println!("{user_temperature_in} fahrenheit is {temp} celsius.");
    println!(
        "Going back: {} fahrenheit",
        ch3::celsius_to_fahrenheit(temp)
    );
}

fn do_fib() {
    let mut user_fib_in = String::new();

    println!("Input number of fibonacci iterations:");

    io::stdin()
        .read_line(&mut user_fib_in)
        .expect("Please input a valid input!");

    let user_fib_in = match user_fib_in.trim().parse() {
        Ok(num) => {
            if num > 186 {
                println!("Iteration counts above 186 are not currently supported...");
                186
            } else {
                num
            }
        }
        Err(_) => {
            println!("Error parsing! Defaulting to 100...");
            100
        }
    };

    for x in 1..=user_fib_in {
        println!("fib({x}): {}", ch3::fib(x));
    }
}
