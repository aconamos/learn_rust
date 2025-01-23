use std::io;

fn main() {
    do_fib();
    do_temp();
}

fn do_temp() {
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

    let temp = celsius_to_fahrenheit(user_temperature_in);

    println!("{user_temperature_in} C is {temp} fahrenheit.");
    println!("Going back: {} C", fahrenheit_to_celsius(temp));

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

    let temp = fahrenheit_to_celsius(user_temperature_in);

    println!("{user_temperature_in} fahrenheit is {temp} celsius.");
    println!("Going back: {} fahrenheit", celsius_to_fahrenheit(temp));
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
        println!("fib({x}): {}", fib(x));
    }
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32f32) * 5f32 / 9f32
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    c * 9f32 / 5f32 + 32f32
}

fn fib(n: u8) -> u128 {
    let mut ab = (0, 1);
    let mut c = 0;

    for _ in 1..n {
        c = ab.0 + ab.1;
        ab.0 = ab.1;
        ab.1 = c;
    }
    c
}
