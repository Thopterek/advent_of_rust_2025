use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/*
 * One dimensional array used as 2nd
 * through usage that row * c_width -> first index
*/
fn main() {
    let mut rolls: usize = 0;
    if let Ok(lines) = reader("../input/example.txt") {
        let mut counter_h: usize = 0;
        let mut c_width: usize = 0;
        for line in lines.map_while(Result::ok) {
            println!("Line -> {}", line);
            counter_h += 1;
            if counter_h == 1 {
                c_width = line.len();
            }
        }
        println!("Height -> {counter_h}, Width -> {c_width}");
        let mut arr: Vec<bool> = vec![false; counter_h * c_width];
        if let Ok(lines) = reader("../input/example.txt") {
            let mut row: usize = 0;
            println!("Compare to filled out arr");
            for line in lines.map_while(Result::ok) {
                arr = fillout_arr(line, row, arr, c_width);
                row += 1;
            }
            rolls = count_rolls(arr, c_width, counter_h);
        }
    }
    else {
        println!("Run from src folder, should be fine");
    }
    println!("");
    println!("Rolls to be picked up -> {rolls}");
}

fn count_adjecent(arr: Vec<bool>, index: usize, max_index: usize, row: usize, width: usize, height: usize) -> usize {
    let mut count: usize = 0;
    let row_max: usize = (row + 1) * width;
    if index - 1 <= 0 {
        if arr[index - 1] == true {
            count += 1;
        }
    }
    if index + 1 <= row_max {
        if arr[index + 1] == true {
            count += 1;
        }
    }
    if row > 0 {
        if arr[index - (width)] == true {
            count += 1;
        }
        if index - (width - 1) >= 0 && arr[index - (width - 1)] == true {
            count += 1;
        }
        if index + 1 - width < row_max && arr[index + 1 - width] == true {
            count += 1;
        }
    }
    if row < height {
        if index + width < max_index && arr[index + width] == true {
            count += 1;
        }
        if index + width + 1 <= max_index && arr[index + width + 1] == true {
            count += 1;
        }
        if index + width - 1 <= (row * width) && arr[index + width - 1] == true {
            count += 1;
        }
    }
    count
}

fn free_to_take(counted: usize) -> usize {
    if counted < 4 {
        let found: usize = 1;
        return found;
    }
    let roll: usize = 0;
    roll
}

fn count_rolls(arr: Vec<bool>, width: usize, height: usize) -> usize {
    let mut rolls: usize = 0;
    let mut row: usize = 0;
    let mut index: usize = 0;
    println!("");
    println!("------------------");
    println!("Checking the array");
    println!("------------------");
    for roll_or_not in arr.clone() {
        if roll_or_not == true {
            let last = rolls;
            rolls += free_to_take(count_adjecent(arr.clone(), index, width * height, row, width, height));
            if last != rolls {
                print!("X");
            }
            else {
                print!("@");
            }
        }
        else {
            print!(".");
        }
        index += 1;
        if index == width * (row + 1) {
            println!("");
            row += 1;
        }
    }
    rolls
}

fn fillout_arr(s: String, row: usize, mut arr: Vec<bool>, width: usize) -> Vec<bool> {
    let bytes = s.as_bytes();
    println!("");
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'@' {
            arr[(row * width) + i] = true;
            print!("@");
        }
        else {
            print!(".");
        }
    }
    arr
}

