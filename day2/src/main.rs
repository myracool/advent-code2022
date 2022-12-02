use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::*;

fn part1() {
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

// part 2

fn main() {
    let mut reference: HashMap<&str, i32> = HashMap::new();

    reference.insert("A", 1);
    reference.insert("B", 2);
    reference.insert("C", 3);

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
        let my_play = plays_letter[1];

        let player1 = reference.get(plays_letter[0]).unwrap();
        let player2;

        if my_play == "Y" {
            score +=  *player1 + 3;
        } else if my_play == "X" {
            if *player1 == 1 {
                player2 = 3;
            } else {
                player2 = *player1 - 1;
            }
            score += player2;
        } else if my_play == "Z" {
            if *player1 == 3 {
                player2 = 1;
            } else {
                player2 = *player1 + 1;
            }
            score += player2 + 6;
        }
    }

    println!("total : {}", score);
}