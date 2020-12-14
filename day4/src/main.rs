use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut valid_passports = 0;

    if let Ok(lines) = read_lines("./input") {
        let mut nofields = 0;
        for line in lines {
            if let Ok(field)  = line {
                if field.chars().count() == 0 {
                    if nofields == 7 {
                        valid_passports += 1;
                    }
                    nofields = 0;
                }
                if field.contains("byr") {
                    nofields += 1;
                }
                if field.contains("iyr") {
                    nofields += 1;
                }
                if field.contains("eyr") {
                    nofields += 1;
                }
                if field.contains("hgt") {
                    nofields += 1;
                }
                if field.contains("hcl") {
                    nofields += 1;
                }
                if field.contains("ecl") {
                    nofields += 1;
                }
                if field.contains("pid") {
                    nofields += 1;
                }
            
            }

        }
    }

    println!("{}", valid_passports);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
