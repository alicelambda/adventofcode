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
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(rule) = line {
                let bag = parse_rule(&rule);
                let insname = bag.name.clone();
                bags.insert(insname,bag);
            }
        }
    }
    println!("{}",traverse(&bags));
}

fn traverse (bags: &HashMap<String,Bag>) -> i64{
    let mut cancontain = 0;
    for (bag, _) in bags {
        if bag != "shiny gold bag" {
            cancontain += traverse_help(bag,bags);
        }
    }
    cancontain
}

fn traverse_help(curbag: &str, bags: &HashMap<String,Bag>) -> i64 {
    let con = bags.get(curbag).unwrap();
    for i in &con.can {
        if i.name == "shiny gold bag" {
            return 1
        } else {
            if traverse_help(&i.name, bags) == 1 {
                return 1
            }
        }
    }
    0
}

fn parse_rule (rule: &str) -> Bag {
    let bagcontain= rule.split("contain").collect::<Vec<&str>>();
    let part2 = bagcontain[1].split(",").collect::<Vec<&str>>();
    let mut chunks = part2.chunks(1).peekable();
    let mut children = Vec::new();
    while let Some(rule) = chunks.next() {
        if chunks.peek().is_some() {
            let contains = parse_contains(&rule[0].to_string());
            match contains {
                Some(rule) => children.push(rule),
                None => ()
            }
        } else {
            let mut bag = rule[0].to_string();
            bag.pop();
            let contains = parse_contains(&bag);
            match contains {
                Some(rule) => children.push(rule),
                None => ()
            }
        
        }
    }
    let mut name = bagcontain[0].to_string();
    name.pop();
    println!("name {}",name);
    Bag {
        can: children,
        name: name,
    }
}

fn parse_contains( bag: &str) -> Option<Contains> {
    let mut bagparse = bag.to_string();
    bagparse.remove(0);
    if bagparse == "no other bags" {
        return None
    }
    let (num,bag) = bagparse.split_once(" ").unwrap();
    let nobags = num.parse::<i32>().unwrap();
    let mut name = bag.to_string();
    if nobags > 1 {
        name.pop();
    }
    Some(
        Contains {
            name:name,
            num:nobags,

        })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
