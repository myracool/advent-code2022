use std::env;
use std::fs::File;
use std::io::*;

fn part1() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score: u32 = 0;

    for line in reader.lines() {
        let temp = line.unwrap();
        if temp == " " {
            break;
        }
        let size = temp.len() / 2;
        let first = &temp[..size];
        let last = &temp[size..];

        let mut com: u32 = 0;
        let mut find: char = '&';

        for i in first.chars() {
            for j in last.chars() {
                if i == j {
                    com = i as u32;
                    find = i;
                }
            }
        }

        if find.is_lowercase() {
            score += com - 96;
        } else {
            com = find.to_ascii_lowercase() as u32;
            score += com - 70;
        }
    }
    println!("{}", score);
}


//part 2
fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score: u32 = 0;
    let mut cpt = 0;
    let mut big_line: Vec<String> = Vec::new();

    for line in reader.lines() {
        let temp = line.unwrap();


        if cpt == 2  || temp == "\n" {
            big_line.push(temp.to_string());
            let mut val: char = '&';
            let mut com: u32 = 0;

            for c1 in big_line[0].chars() {
                for c2 in big_line[1].chars() {
                    if c1 == c2 {
                        for c3 in big_line[2].chars() {
                            if c1 == c3 {
                                val = c1;
                                com = val as u32;
                            }
                        }
                    }
                }
            }

            if val.is_lowercase() {
                score += com - 96;
            } else {
                com = val.to_ascii_lowercase() as u32;
                score += com - 70;
            }

            cpt = 0;
            big_line.clear();
        } else {
            big_line.push(temp.to_string());
            cpt += 1;
        }
    }
    println!("{}", score);
}