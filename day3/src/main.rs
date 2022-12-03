use std::env;
use std::fs::File;
use std::io::*;


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
        let size = temp.len()/2;
        let first = &temp[..size];
        let last =  &temp[size..];

        let mut com: u32 = 0;
        let mut find:char='&';

        for i in first.chars(){
            for j in last.chars(){
                if i == j {
                    com = i as u32;
                    find = i;
                }
            }
        }

        if find.is_lowercase(){
            score += com - 96;
        }
        else {
            com = find.to_ascii_lowercase() as u32;
            score += com - 70;
        }

    }
    println!("{}",score);

}
