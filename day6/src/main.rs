use std::collections::HashMap;
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;



fn main() {
    let mut answers = HashMap::new();
    let mut noanswers = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(answer) = line {
                let length = answer.len();
                if length != 0 {
                    for c in answer.chars() {
                        answers.insert(c,1);
                    }
                } else {
                    noanswers += answers.len();
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
