use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
struct Bag {
    can: Vec<String>,
    name: String
}

struct Contains {
    name: String,
    num: i32,
}


fn main() {
    let mut bags:HashMap<String,Bag> = HashMap::new();
    if let Ok(lines) = read_lines("./test") {
        for line in lines {
            if let Ok(rule) = line {
                let bag = parse_rule(&rule);
                println!("{:?}",bag);
                let insname = bag.name.clone();
                bags.insert(insname,bag);
            }
        }
    }
}

fn parse_rule (rule: &str) -> Bag {
    let parts = rule.split("contain").collect::<Vec<&str>>();
    let part2 = parts[1].split(",").collect::<Vec<&str>>();
    let mut contains : Vec<String> = Vec::new();
    if part2.len() == 2 {
        contains.push(part2[0].to_string());
        let mut s1 = part2[1].to_string();
        s1.pop();
        contains.push(s1);
    } else {
        let mut s1 = part2[0].to_string();
        s1.pop();
        contains.push(s1);
    }
    Bag {
        can: contains,
        name: parts[0].to_string()
    }
}

fn parse_bag_num (mut bag: &str) {
    bag.remove(0);
    bag.split_once(" ");

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
