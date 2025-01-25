pub fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32f32) * 5f32 / 9f32
}

pub fn celsius_to_fahrenheit(c: f32) -> f32 {
    c * 9f32 / 5f32 + 32f32
}

pub fn fib(n: u8) -> u128 {
    let mut ab = (0, 1);
    let mut c = 0;

    for _ in 1..n {
        c = ab.0 + ab.1;
        ab.0 = ab.1;
        ab.1 = c;
    }
    c
}
