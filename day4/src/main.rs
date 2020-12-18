use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut valid_passports = 0;

    if let Ok(lines) = read_lines("./input") {
        let mut nofields = 0;
        for line in lines {
            if let Ok(fields)  = line {
                for field in  fields.split(" ") {
                    let parts = field.split(":").collect::<Vec<&str>>();
                    if parts.len() == 2 {
                        validate_field(parts[0],parts[1]);
                    }
                }
            
            }

        }
    }

    println!("{}", valid_passports);
}

fn validate_field(tag: &str, body: &str) -> bool {
    match tag {
        "byr" => {
            let birth = body.parse::<i32>().unwrap();
            birth >= 1920 && birth <= 2002
        },
        "iyr" => {
            let issue = body.parse::<i32>().unwrap();
            issue >= 2010 && issue <= 2020 
        },
        "eyr" => {
            let expire = body.parse::<i32>().unwrap();
            expire >= 2020 && expire <= 2030
        },
        "hgt" => {
            let len = body.len();
            let unit =  &body[len-2..len];
            match unit {
                "in" => {
                    let num = &body[0..len-2].parse::<i32>().unwrap();
                    num >= &59 && num <= &76
                },
                "cm" => {
                    let num = &body[0..len-2].parse::<i32>().unwrap();
                    num >= &150 && num <= &193
                }
                _ => false
            }
        },
        "hcl" => {
            if body.len() != 6 {
                return false
            }
            let c = &body[0..1];
            if c != "#" {
                return false
            }
            for c in body[1..7].chars() {
                match c {
                    'a'..='f' | '0'..='9' => {
                    }
                    _ => return false
                }
            
            }
            true

        },
        "ecl" => {
            match body {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false
            }
        },
        "pid" => {
            if body.len() != 9 {
                return false
            }
            for c in body.chars() {
                if let '0'..='9' = c {
                } else {
                    return false
                }
            }
            true
        },
        "cid" => true,
        _ => true,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
