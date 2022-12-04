use std::env;
use std::fs::File;
use std::io::*;

fn par1() {
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
        let sections: Vec<&str> = temp.split(",").collect();

        let e1 = get_and_parse(sections[0]);
        let e2 = get_and_parse(sections[1]);

        if e1[0] <= e2[0] && e1[1] >= e2[1] ||
            e2[0] <= e1[0] && e2[1] >= e1[1] {
            score += 1;
        }
    }

    println!("score : {}", score);
}

fn get_and_parse(sec: &str) -> Vec<i32> {
    let mut to_int: Vec<i32> = Vec::new();
    for i in sec.split("-") {
        to_int.push(i.parse::<i32>().unwrap());
    }
    return to_int;
}

// part 2
fn main() {
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
        let sections: Vec<&str> = temp.split(",").collect();

        let e1 = get_and_parse_and_generate(sections[0]);
        let e2 = get_and_parse_and_generate(sections[1]);

        for i in e2.iter() {
            if e1.contains(i) {
                score +=1;
                break;
            }
        }
    }

    println!("score : {}", score);

}

fn get_and_parse_and_generate(sec: &str) -> Vec<i32> {
    let mut to_int: Vec<i32> = Vec::new();
    for i in sec.split("-") {
        to_int.push(i.parse::<i32>().unwrap());
    }
    return (to_int[0]..to_int[1]+1).collect();
}