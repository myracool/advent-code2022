use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::*;

fn main() {
    let mut reference: HashMap<&str, i32> = HashMap::new();

    reference.insert("A", 1);
    reference.insert("B", 2);
    reference.insert("C", 3);
    reference.insert("X", 1);
    reference.insert("Y", 2);
    reference.insert("Z", 3);

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score: i32 = 0;

    for line in reader.lines() {
        let temp = line.unwrap();
        if temp == " " {
            break;
        }

        let plays_letter: Vec<&str> = temp.split_whitespace().collect();
        let mut plays_int: Vec<&i32> = Vec::new();

        for player in plays_letter {
            plays_int.push(reference.get(player).unwrap());
        }

        score += plays_int[1];

        if *plays_int[0] == 1 && *plays_int[1] == 3 {
            continue;
        } else if *plays_int[0] == 3 && *plays_int[1] == 1 {
            score += 6;
        } else if plays_int[0] < plays_int[1] {
            score += 6;
        } else if plays_int[0] == plays_int[1] {
            score += 3;
        }

    }

    println!("total : {}", score);
}
