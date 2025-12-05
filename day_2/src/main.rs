use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use fancy_regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("Day 2, lets goooo");
    let mut sum: u64 = 0;
    if let Ok(lines) = read_lines("../input/test.txt") {
        for line in lines.map_while(Result::ok) {
            let parts = line.split(",");
            for sequence in parts {
                sum += get_repeat(sequence);
                println!("Sum is now -> {}", sum);
            }
        }
    }
    else {
        println!("Where is that file?");
    }
    println!("Answer -> {}", sum);
}

fn get_repeat(sequence: &str) -> u64 {
    let (range_s, r_end) = sequence.split_once("-").expect("Invalid format");
    let start: u64 = range_s.parse().expect("Not a number?");
    let end: u64 = r_end.parse().expect("Not a number?");
    let mut sum: u64 = 0;
    let regex = Regex::new(r"^(\w+) (\1)$").unwrap();
    let mut counter = start;
    while counter <= end {
        let to_check: &str = &counter.to_string();
        let result = regex.is_match(to_check);
        if result.is_ok() {
            let add: u64 = to_check.parse().expect("Not a num");
            println!("To add value is -> {add}");
            sum += add;
        }
        counter += 1;
    }
    sum
}
