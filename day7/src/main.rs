use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::collections::HashMap;

struct Bag {
    can: Vec<String>,
    name: String
}


fn main() {
    let mut bags:HashMap<String,Bag> = HashMap::new();
    if let Ok(lines) = read_lines("./test") {
        for line in lines {
            if let Ok(rule) = line {
                let bag = parse_rule(&rule);
                let insname = bag.name.clone();
                bags.insert(insname,bag);
            }
        }
    }
}

fn parse_rule (rule: &str) -> Bag {
    let parts = rule.split("contain").collect::<Vec<&str>>();
    let part2 = parts[1].split(",").collect::<Vec<&str>>();
    let mut contains = Vec::new();
    contains.push(*part2[0]);
    contains.push(*part2[1]);

    Bag {
        can: contains,
        name: "hi".to_string()
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
