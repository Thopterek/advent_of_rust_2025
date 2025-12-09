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
    let mut total_rolls: usize = 0;
    if let Ok(lines) = reader("../input/puzzle.txt") {
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
        if let Ok(lines) = reader("../input/puzzle.txt") {
            let mut row: usize = 0;
            println!("Compare to filled out arr");
            for line in lines.map_while(Result::ok) {
                arr = fillout_arr(line, row, arr, c_width);
                row += 1;
            }
            loop {
                let tuple: (usize, Vec<bool>) = count_rolls(arr.clone(), c_width, counter_h);
                let rolls = tuple.0;
                arr = tuple.1;
                if rolls == 0 {
                    break;
                }
                total_rolls += rolls;
            }
        }
    }
    else {
        println!("Run from src folder, should be fine");
    }
    println!("");
    println!("Rolls to be picked up -> {total_rolls}");
}

fn count_adjecent(arr: Vec<bool>, index: usize, row: usize, width: usize, height: usize) -> usize {
    let mut count: usize = 0;
    let row_max: usize = (row + 1) * width;
    let row_min: isize = (row * width).try_into().unwrap();
    /*
     * The easy checks
     * left <---> right
    */
    let convert_index: isize = (index).try_into().unwrap();
    let check_left: isize = convert_index - 1;
    if check_left >= row_min {
        if arr[index - 1] == true {
            count += 1;
        }
    }
    if index + 1 < row_max {
        if arr[index + 1] == true {
            count += 1;
        }
    }
    if row > 0 {
        // checking up
        let up_max: isize = (((row - 1) * width) + width).try_into().unwrap();
        let up_min: isize = ((row - 1) * width).try_into().unwrap();
        let check_up: isize = (index - width).try_into().unwrap();
        if arr[index - width] == true {
            count += 1;
        }
        // up left
        if up_min <= check_up - 1 && arr[index - width - 1] == true {
            count += 1;
        }
        // up right
        if check_up + 1 < up_max && arr[index + 1 - width] == true {
            count += 1;
        }
    }
    if row < height - 1 {
        // checking down
        let down_max: isize = (((row + 1) * width) + width).try_into().unwrap();
        let check_down: isize = (index + width).try_into().unwrap();
        let down_min: isize = ((row + 1) * width).try_into().unwrap();
        if arr[index + width] == true {
            count += 1;
        }
        // down right
        if check_down + 1 < down_max && arr[index + width + 1] == true {
            count += 1;
        }
        // down left
        if check_down - 1 >= down_min && arr[index + width - 1] == true {
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

fn count_rolls(arr: Vec<bool>, width: usize, height: usize) -> (usize, Vec<bool>) {
    let mut rolls: usize = 0;
    let mut row: usize = 0;
    let mut index: usize = 0;
    println!("");
    println!("------------------");
    println!("Checking the array");
    println!("------------------");
    let mut new_arr: Vec<bool> = arr.clone();
    for roll_or_not in arr.clone() {
        if roll_or_not == true {
            let last = rolls;
            rolls += free_to_take(count_adjecent(arr.clone(), index, row, width, height));
            if last != rolls {
                new_arr[index] = false;
                print!("X");
            }
            else {
                new_arr[index] = true;
                print!("@");
            }
        }
        else {
            new_arr[index] = false;
            print!(".");
        }
        index += 1;
        if index == width * (row + 1) {
            println!("");
            row += 1;
        }
    }
    (rolls, new_arr)
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

