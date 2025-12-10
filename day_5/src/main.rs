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
        let mut ranges: Vec<usize> = vec![];
        let mut start_checking: bool = false;
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
            if start_checking == false && line.is_empty() == false {
                let range = line.split_once("-");
                let start: usize = range.unwrap().0.parse().expect("A NUMBER");
                let end: usize = range.unwrap().1.parse().expect("A NUMBER");
                ranges.push(start);
                ranges.push(end);
            }
            if start_checking == true {
                let mut index: usize = 0;
            }
            if line.is_empty() == true {
                start_checking = true;
            }
        }
    }
    println!("Fresh ingridents -> {}", fresh);
}
// part used in start_checking for part 1
//            if start_checking == true {
//                let converted: usize = line.parse().expect("A NUMBER");
//                let mut index = 0;
//                while index < ranges.len() {
//                    if converted >= ranges[index] && converted <= ranges[index + 1] {
//                        fresh += 1;
//                        break;
//                   }
//                   index += 2;
