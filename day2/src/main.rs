use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    let mut meetspolicy = 0;

    if let Ok(lines) = read_lines("./input") {

        for line in lines {
            if let Ok(passwordpolicy) = line {
                let split = passwordpolicy.split(":").collect::<Vec<&str>>();
                let policy = split[0].split(" ").collect::<Vec<&str>>();
                let letter = policy[1].chars().collect::<Vec<char>>()[0];
                let range = policy[0];

                let firstlast = range.split("-").collect::<Vec<&str>>();
                let min = firstlast[0].parse::<usize>().unwrap();
                let max = firstlast[1].parse::<usize>().unwrap();

                if min >= max {

                    panic!("hewwow");
                }

                let mut m = 0;    
                
                let chars = split[1].as_bytes();
                if chars[min] as char == letter {
                    println!("{} {} {:?}", chars[min-1] as char, letter,chars);
                    m+=1;
                }

                if chars[max] as char == letter {
                    m+=1;
                }

                if m  == 1 {
                    meetspolicy += 1;
                }
            }
        }
    }

    println!("{}",meetspolicy);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
