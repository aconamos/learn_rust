use std::{collections::HashMap, io};

pub fn median_mode(nums: &Vec<i32>) -> (i32, i32) {
    let mut occurrences = HashMap::new();

    for x in nums {
        occurrences.entry(x).and_modify(|x| *x += 1).or_insert(1);
    }

    let mode = *occurrences.into_keys().max().expect("Weird");

    let mut v = nums.clone();

    v.sort();

    let median = if v.len() % 2 == 0 {
        (v.get(v.len() / 2).expect("Weird") + v.get(v.len() / 2 + 1).expect("Weird")) / 2
    } else {
        *(v.get(v.len() / 2).expect("Weird"))
    };

    (median, mode)
}

pub fn pig_latin(words: &String) -> String {
    let mut new_sentence = String::new();

    for word in words.split_whitespace() {
        match word.chars().next().expect("Weird") {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                new_sentence.push_str(&word);
                new_sentence.push_str("-hay ");
            }
            c => {
                new_sentence.push_str(&word[1..word.len()]); // Potential panic here because of strings/bytes issue
                new_sentence.push_str(format!("-{}ay ", c).as_str());
            }
        };
    }

    new_sentence
}
enum Operation {
    Add(String, String),
    Remove(String, String),
    Query(String),
    List,
}

pub fn department() {
    let mut map = HashMap::<String, Vec<String>>::new();

    loop {
        println!("Enter operation (Valid operations: add user to dept, remove user from dept, query dept, list): ");

        let mut user_input = String::new();

        let op = match match io::stdin().read_line(&mut user_input) {
            Ok(_) => parse_op(&user_input),
            Err(_) => continue,
        } {
            Ok(op) => op,
            Err(msg) => {
                println!("ERROR: {}", msg);
                continue;
            }
        };

        match op {
            Operation::Add(user, dept) => {
                map.entry(dept)
                    .and_modify(|v| {
                        let ind = match v.binary_search(&user) {
                            Ok(val) => val,
                            Err(val) => val,
                        };
                        v.insert(ind, user.clone())
                    })
                    .or_insert(vec![user]);
            }
            Operation::Remove(user, dept) => {
                let map = map
                    .get_mut(&dept)
                    .expect("Department should be valid! Giving up...");

                if let Ok(i) = map.binary_search(&user) {
                    map.remove(i);
                }
            }
            Operation::Query(dept) => {
                println!(
                    "{:?}",
                    map.get(&dept)
                        .unwrap_or(&vec!["No such department!".to_string()])
                );
            }
            Operation::List => {
                println!("{:#?}", map)
            }
        }
    }
}

fn parse_op(inp: &String) -> Result<Operation, String> {
    let mut parts = inp.split_whitespace();

    let op = parts
        .next()
        .ok_or("Error parsing operation!")?
        .to_lowercase();

    let op = op.as_str();

    match op {
        "add" | "remove" | "rm" => {
            let person = parts
                .next()
                .ok_or("Error parsing person for add or remove operation!")?
                .to_string();

            parts.next();

            let department = parts
                .next()
                .ok_or("Error parsing department for add or remove operation!")?
                .to_string();

            return match op {
                "add" => Ok(Operation::Add(person, department)),
                "remove" | "rm" => Ok(Operation::Remove(person, department)),
                _ => panic!("This shouldn't be reached!"),
            };
        }
        "query" => {
            let department = parts
                .next()
                .ok_or("Error parsing department for query operation!")?
                .to_string();

            return Ok(Operation::Query(department));
        }
        "list" => Ok(Operation::List),
        typ => return Err(format!("Error parsing operation type {typ}!").to_string()),
    }
}
