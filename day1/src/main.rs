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

    // Step 2: loop over lines and print them.
    for line in reader.lines() {
        if line.as_ref().unwrap() == "" {
            if most < sum {
                most = sum;
            }
            sum = 0;
        }
        else {
            let to_int : i32 =  line.as_ref().expect("omg le rust c'est penible en fait").parse::<i32>().unwrap();
            sum += to_int;
        }
    }

    println!("MOST: {}", most);
}
