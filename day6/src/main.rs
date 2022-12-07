use std::env;
use std::fs;
use std::io::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let line = read_file_string(file_path);

    let mut cpt = 0;
    let mut msg = String::new();
    for char in line.unwrap().chars() {
        if msg.len() == 14 {
            if find_duplicates(msg.as_ref()) == false {
                msg.remove(0);
            } else {
                println!("{} - {}", cpt, msg);
                break;
            }
        }
        msg.push(char);
        cpt += 1;
    }
}


fn read_file_string(filepath: &str) -> std::result::Result<String, Error> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}



fn find_duplicates(msg: & str) -> bool {
    return msg.chars().enumerate().find_map(|(i, c)| {
        msg.chars()
            .enumerate()
            .skip(i + 1)
            .find(|(_, other)| c == *other)
            .map(|(j, _)| (i, j, c))
    }).is_none();
}