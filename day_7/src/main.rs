use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut result: usize = 0;
    if let Ok(lines) = reader("../input/example.txt") {
        let mut arr: Vec<bool> = vec![];
        let mut start: usize = 0;
        let mut collumn: usize = 0;
        for line in lines.map_while(Result::ok) {
            if collumn == 0 {
                collumn = line.len();
            }
            let bytes = line.as_bytes();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b'.' {
                    arr.push(false);
                }
                else if item == b'^' {
                    arr.push(true);
                }
                else if item == b'S' {
                    start = i;
                    println!("Start at -> {}", start);
                }
            }
        }
    }
    else {
        println!("Where is the file?");
    }
    println!("Result is -> {}", result);
}
