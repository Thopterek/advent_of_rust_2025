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
    if let Ok(lines) = reader("../input/puzzle.txt") {
        for line in lines.map_while(Result::ok) {
            println!("Line -> {}", line);
            sum += joltage(line);
        }
    }
    else {
        println!("Damn file...");
    }
    println!("Total! {}", sum);
}

fn joltage(line: String) -> u64 {
    let max: u64 = line.len().try_into().unwrap();
    let single_nums = line.split("");
    let repeat = single_nums.clone();
    let mut first: u64 = 0;
    let mut second: u64 = 0;
    let mut counter: u64 = 0;
    let mut position: u64 = 0;
    for num in single_nums {
        let check: u64 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if first < check && counter < max - 1 {
            first = check;
            position = counter;
        }
        counter += 1;
    }
    counter = 0;
    for num in repeat {
        let check: u64 = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if counter > position && second < check {
            second = check;
        }
        counter += 1;
    }
    let sum: u64 = (first * 10) + second;
    println!("Sum -> {}", sum);
    sum
}
