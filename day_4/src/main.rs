use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = reader("../input/example.txt") {
        for line in lines.map_while(Result::ok) {
            println!("Line -> {}", line);
        }
    }
    else {
        println!("Run from src folder, should be fine");
    }
}
