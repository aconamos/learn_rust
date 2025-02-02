pub const TURBOFISH_HEAD: &'static str = "::<>";

#[derive(Debug)]
enum LinkedList<T> {
    Node(T, Box<LinkedList<T>>),
    Nil,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    pub fn new(val: T) -> LinkedList<T> {
        LinkedList::Node(val, Box::<LinkedList<T>>::new(LinkedList::Nil))
    }

    pub fn new_empty() -> LinkedList<T> {
        LinkedList::Nil
    }

    pub fn add(&mut self, val: T) {
        let mut pointer = self;

        while let LinkedList::Node(_, link) = pointer {
            pointer = link;
        }

        if let LinkedList::Nil = pointer {
            *pointer = LinkedList::Node(val, Box::new(LinkedList::Nil));
        }
    }
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    let mut linkylist = LinkedList::new_empty();

    for val in 1..=100 {
        linkylist.add(val);
    }

    println!("{linkylist:?}");
}
