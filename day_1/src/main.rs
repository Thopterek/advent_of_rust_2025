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
    let mut wrapper: [i32; 2] = [50, 0];
    // let mut dial: i32 = 50;
    // let mut count: i32 = 0;
    if let Ok(lines) = read_lines("../input/rotation.txt") {
        for line in lines.map_while(Result::ok) {
            wrapper = move_dial(wrapper[0], line, wrapper[1]);
            if wrapper[0] == 0 {
                wrapper[1] += 1;
            }
        }
    }
    else {
        println!("Where is the damn file");
    }
    println!("Password {}", wrapper[1]);
}

fn move_dial(current_dial: i32, mut next_move: String, current_count: i32) -> [i32; 2] {
    let is_right: bool = if next_move.starts_with('R') {true} else {false};
    next_move.remove(0);
    let move_count: i32 = next_move.parse().expect("Not a number");
    let mut new_count: i32 = current_count;
    if is_right == true {
        let mut new_dial: i32 = current_dial + move_count;
        while new_dial > 99 {
            new_dial -= 100;
            new_count += 1;
        }
        let new_wrapper: [i32; 2] = [new_dial, new_count];
        return new_wrapper;
    }
    let mut new_dial: i32 = current_dial - move_count;
    while new_dial < 0 {
        new_dial += 100;
        new_count += 1;
    }
    let new_wrapper: [i32; 2] = [new_dial, new_count];
    new_wrapper
}

