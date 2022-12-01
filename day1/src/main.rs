use std::env;
use std::fs::File;
use std::io::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;
    let mut most = 0;
    let mut array: [i32; 3] = [0; 3];
    let mut min_index: usize = 0;

    for line in reader.lines() {
        if line.as_ref().unwrap() == "" {
            for (i, _) in array.into_iter().enumerate() {
                if array[i] < array[min_index] {
                    min_index = i;
                }
            }
            if sum > array[min_index] {
                array[min_index] = sum;
            }
            sum = 0;
        } else {
            let to_int: i32 = line.as_ref().expect("omg le rust c'est penible en fait").parse::<i32>().unwrap();
            sum += to_int;
        }
    }
    for (_, x) in array.iter().enumerate() {
        most += x;
    }
    println!("total: {}", most);
}

