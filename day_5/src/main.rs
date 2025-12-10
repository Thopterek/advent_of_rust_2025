use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
// use std::collections::HashSet;

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
        // let mut start_checking: bool = false;
        for line in lines.map_while(Result::ok) {
            if line.is_empty() == false {
                let range = line.split_once("-");
                let start: usize = range.unwrap().0.parse().expect("A NUMBER");
                let end: usize = range.unwrap().1.parse().expect("A NUMBER");
                ranges.push(start);
                ranges.push(end);
                // ranges.push(start);
                // ranges.push(end);
            }
            // if start_checking == true {
                // nevermind I don't think it will be needed
            //    let mut index: usize = 0;
            //    while index < ranges.len() {
            //        fresh += (ranges[index + 1] + 1) - ranges[index];
            //        println!("fresh now is {fresh}");
            //        index += 2;
            //    }
            //    break;
            //}
            if line.is_empty() == true {
                // start_checking = true;
                // fresh += remove_dups(ranges);
                let mut cleaned: Vec<usize> = vec![];
                cleaned.push(ranges[0]);
                cleaned.push(ranges[1]);
                let mut index: usize = 2;
                while index < ranges.len() {
                    let mut check: usize = 0;
                    let mut keep_first_lower: bool = false;
                    let mut keep_first_higher: bool = false;
                    let mut keep_second_lower: bool = false;
                    let mut keep_second_higher: bool = false;
                    for i in &cleaned {
                        if ranges[index] < *i {
                            check += 1;
                        }
                        if check == cleaned.len() {
                            keep_first_lower = true;
                        }
                    }
                    check = 0;
                    for i in &cleaned {
                        if ranges[index] > *i {
                            check += 1;
                        }
                        if check == cleaned.len() {
                            keep_first_higher = true;
                        }
                    }
                    check = 0;
                    for i in &cleaned {
                        if ranges[index + 1] < *i {
                            check += 1;
                        }
                        if check == cleaned.len() {
                            keep_second_lower = true;
                        }
                    }
                    check = 0;
                    for i in &cleaned {
                        if ranges[index + 1] > *i {
                            check += 1;
                        }
                        if check == cleaned.len() {
                            keep_second_higher = true;
                        }
                    }
                    // case if we are adding completly new ranges to the thing
                    if (keep_first_lower == true && keep_second_lower == true) || (keep_first_higher == true && keep_second_higher == true) {
                        cleaned.push(ranges[index]);
                        cleaned.push(ranges[index + 1])
                    }
                    // case if we are replacing everything that was saved before
                    else if keep_first_lower == true && keep_second_higher == true {
                        cleanded.clear();
                        cleaned.push(ranges[index]);
                        cleaned.push(ranges[index + 1]);
                    }
                    index += 2;
                }
                for i in cleaned {
                    println!("Values in cleaned {i}");
                }
                break;
            }
        }
    }
    println!("Fresh ingridents -> {}", fresh);
}

// fn remove_dups(data: Vec<usize>) -> usize {
//    let set: HashSet<_> = data.into_iter().collect();
//   let total: usize = set.len();
//    total
//}

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
