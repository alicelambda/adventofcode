#![feature(str_split_once)]
use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
struct Bag {
    can: Vec<Contains>,
    name: String
}

#[derive(Debug)]
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
                let insname = bag.name.clone();
                bags.insert(insname,bag);
            }
        }
    }
    println!("{:?}",bags);
}

fn parse_rule (rule: &str) -> Bag {
    let parts = rule.split("contain").collect::<Vec<&str>>();
    let part2 = parts[1].split(",").collect::<Vec<&str>>();
    let mut contains : Vec<Contains> = Vec::new();
    if part2.len() == 2 {
        match  parse_bag_num(&part2[0].to_string()) {
            Some(rule) => contains.push(rule),
            None => {}
        }
        
        let mut s1 = part2[1].to_string();
        s1.pop();
        match parse_bag_num(&s1) {
            Some(rule) => contains.push(rule),
            None => {}
        }
    } else {
        let mut s1 = part2[0].to_string();
        s1.pop();
        match parse_bag_num(&s1) {
            Some(rule) => contains.push(rule),
            None => {}
        }

    }
    let mut name = parts[0].to_string();
    name.pop();
    Bag {
        can: contains,
        name: name
    }
}

fn parse_bag_num ( bag: &str) -> Option<Contains> {
    let mut bagparse = bag.to_string();
    bagparse.remove(0);
    if bagparse == "no other bags" {
        return None
    }
    let (a,b) = bagparse.split_once(" ").unwrap();
    let nobags = a.parse::<i32>().unwrap();
    Some(
        Contains {
            name:b.to_string(),
            num:nobags,

        })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
