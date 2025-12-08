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

fn count_rolls(arr: Vec<bool>, width: usize, height: usize) -> usize {
    let mut rolls: usize = 0;
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

