use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("Hello, world!");
    if let Ok(lines) = reader("../input/example.txt") {
        let mut arr: Vec<usize> = vec![];
        let mut rows: usize = 0;
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
            let individual = line.split(" ");
            if line.contains("+") {
                let mut start: usize = 0;
                let mut index: usize = 0;
                let mut counter: usize = 0;
                let vector_len = arr.len();
                let collumns = vector_len / rows;
                let mut one_coll = vec![];
                while index < vector_len {
                    let number = arr[index];
                    one_coll.push(number);
                    index += collumns;
                    counter += 1;
                    if counter == rows {
                        for i in &one_coll {
                            print!("{} ", i);
                        }
                        println!("<- one collumn");
                        one_coll.clear();
                        counter = 0;
                        start += 1;
                        if start > collumns - 1 {
                            break;
                        }
                        index = start;
                    }
                }
            }
            else {
                for i in individual {
                    let number: usize = match i.trim().parse() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };
                    arr.push(number);
                }
            }
            rows += 1;
        }
    }
    else {
        println!("Where is the file?");
    }
}
