use std::collections::HashMap;
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;



fn main() {
    let mut answers = HashMap::new();
    let mut noanswers = 0;
    let mut nopeople = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(answer) = line {
                let length = answer.len();
                if length != 0 {
                    for c in answer.chars() {
                        let count = answers.entry(c).or_insert(0);
                        *count += 1;
                    }
                    nopeople += 1;
                } else {
                    for (_, n) in &answers {
                        if *n == nopeople {
                            noanswers += 1;
                        }
                    }
                    nopeople = 0;
                    answers = HashMap::new();
                }
            }
        }
    }
    println!("{}",noanswers);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
