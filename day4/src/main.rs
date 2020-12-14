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
                        print!("{:?}",parts);
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
            println!("expiration year"),
        "hgt" => println!("height"),
        "hcl" => println!("hair color"),
        "ecl" => println!("eye color"),
        "pid" => println!("passport id"),
        "cid" => println!("country id"),
        _ => (),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
