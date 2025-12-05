use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("Day 3!");
    let mut sum: u64 = 0;
    if let Ok(lines) = reader("../input/example.txt") {
        for line in lines.map_while(Result::ok) {
            sum += joltage(line);
        }
    }
    else {
        println!("Damn file...");
    }
    println!("Sum {}", sum);
}

fn joltage(line: String) -> u64 {
    let mut sum: u64 = 0;
    let single_nums = line.split("");
    let mut first: u64 = 0;
    let mut second: u64 = 0;
    for num in single_nums {
        println!("{}", num);
    }
    sum
}
