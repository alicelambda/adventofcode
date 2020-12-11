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
                println!("{:?}",firstlast);
                let min = firstlast[0].parse::<i32>().unwrap();
                let max = firstlast[1].parse::<i32>().unwrap();

                let mut numchars = 0;

                for c in split[1].chars() {
                    if c == letter {
                        numchars+=1;
                    }

                }

                if numchars >= min && numchars <= max {
                    meetspolicy +=1;
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
