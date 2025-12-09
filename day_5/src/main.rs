use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn reader<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    println!("FRESH WESH");
    let mut fresh: usize = 0;
    if let Ok(lines) = reader("../input/example.txt") {
        let mut ranges: Vec<u32>;
        let mut start_checking: bool = false;
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
            if start_checking == false {
                let range = line.split("")
            }
            if line.is_empty() {
                start_checking = true;
            }
        }
    }
    println!("Fresh ingridents -> {}", fresh);
}
