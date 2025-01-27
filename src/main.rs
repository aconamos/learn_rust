use std::fmt::Display;

struct Shit<'a> {
    poop_size: u32,
    poop_type: &'a str,
}

impl Display for Shit<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Shit {}, size {}", self.poop_type, self.poop_size)
    }
}

fn main() {
    let my_poopie = Shit {
        poop_size: 5,
        poop_type: "Shitty poopy shit",
    };

    println!("{}", my_poopie);
}
