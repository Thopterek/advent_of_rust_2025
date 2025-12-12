use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut result: usize = 0;
    if let Ok(lines) = reader("../input/puzzle.txt") {
        let mut next_index: Vec<usize> = vec![];
        let mut even: bool = true;
        for line in lines.map_while(Result::ok) {
            let bytes = line.as_bytes();
            if even == false {
                // print!(" -> added {}", next_index.len());
                println!("");
                // result += next_index.len();
            }
            for (i, &item) in bytes.iter().enumerate() {
                if item == b'.' {
                    print!(".");
                }
                else if item == b'^' && next_index.contains(&i) {
                    next_index.retain(|value| *value != i);
                    if next_index.contains(&(i - 1)) == false {
                        next_index.push(i - 1);
                    }
                    if next_index.contains(&(i + 1)) == false {
                        next_index.push(i + 1);
                    }
                    print!("^");
                    result += 1;
                }
                else if item == b'S' {
                    next_index.push(i);
                    print!("S");
                }
            }
            if even == true {
                even = false;
            }
            else {
                even = true;
            }
            println!("");
        }
    }
    else {
        println!("Where is the file?");
    }
    println!("Result is -> {}", result);
}
