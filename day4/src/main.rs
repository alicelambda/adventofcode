use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut valid_passports = 0;
    let mut nofields = 0;

    
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(fields)  = line {
                for field in  fields.split(" ") {
                    let parts = field.split(":").collect::<Vec<&str>>();
                    if parts.len() == 2 {
                        if validate_field(parts[0],parts[1])  {
                            nofields += 1;
                        }
                    } else {
                        if nofields == 7 {
                            valid_passports += 1;
                            println!("valid passport {}",nofields);
                        } else {
                            println!("invalid passport {}",nofields);
                        }
                        nofields = 0;
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
            if body.len() != 7 {
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
        "cid" => { 
            false
        },
        _ => false,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(validate_field("byr","2002"), true);
        assert_eq!(validate_field("byr","2003"), false);
    }

    #[test]
    fn test_height() {
        assert_eq!(validate_field("hgt","60in"), true);
        assert_eq!(validate_field("hgt","190cm"), true);
        assert_eq!(validate_field("hgt","190in"), false);
        assert_eq!(validate_field("hgt","190"), false);
    }

    #[test]
    fn test_eye() {
        assert_eq!(validate_field("ecl","brn"), true);
        assert_eq!(validate_field("ecl","wat"), false);
    }

    #[test]
    fn test_hair() {
        assert_eq!(validate_field("hcl","dab227"), false);
        assert_eq!(validate_field("hcl","#602927"), true);
        assert_eq!(validate_field("hcl","74454a"), false);
    }
}
