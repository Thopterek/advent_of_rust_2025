use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use num_traits::Pow;

fn reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("Day 3!");
    let mut sum: usize = 0;
    if let Ok(lines) = reader("../input/puzzle.txt") {
        for line in lines.map_while(Result::ok) {
            println!("Line -> {}", line);
            sum += joltage_part_two(line);
        }
    }
    else {
        println!("Damn file...");
    }
    println!("Total! {}", sum);
}

fn joltage_part_two(line: String) -> usize {
    let max: usize = line.len().try_into().unwrap();
    let mut all_twelve: [usize; 12] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut position: usize = 0;
    let mut i: usize = 0;
    while i < 12 {
        let single_nums = line.split("");
        let mut counter: usize = 0;
        for num in single_nums {
            let check: usize = match num.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if i != 0 {
                if all_twelve[i] < check && counter < max - (11 - i) && counter > position {
                    all_twelve[i] = check;
                    position = counter;
                }
            }
            else {
                if all_twelve[i] < check && counter < max - (11 - i) {
                    all_twelve[i] = check;
                    position = counter;
                }
            }
            counter += 1;
        }
        i += 1;
    }
    let mut sum: usize = 0;
    i = 0;
    let ten: usize = 10;
    while i < 12 {
        let to_power: usize = 11 - i;
        let times: usize = Pow::pow(ten, to_power);
        sum += all_twelve[i] * times;
        i += 1;
    }
    println!("Sum of the damn thing -> {}", sum);
    sum
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
