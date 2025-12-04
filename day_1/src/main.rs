use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// left from 0 -> one click 99
// right from 99 -> one click 0
fn main() {
    println!("Day 1 start!");
    let mut dial: i32 = 50;
    let mut count: i32 = 0;
    if let Ok(lines) = read_lines("../input/rotation.txt") {
        for line in lines.map_while(Result::ok) {
            dial = move_dial(dial, line);
            if dial == 0 {
                count += 1;
            }
        }
    }
    else {
        println!("Where is the damn file");
    }
}

fn move_dial(current_dial: i32, next_move: str) -> i32 {
    if next_move.starts_with('R') {
    }
}

