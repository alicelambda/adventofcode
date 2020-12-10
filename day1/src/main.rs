use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;



fn main() {

    let mut nums: Vec<i32> = Vec::new();




    if let Ok(lines) = read_lines("./input") {
        
        for line in lines {
            if let Ok(num) = line {
                let int = num.parse::<i32>().unwrap(); 
                nums.push(int);
            }
        }
    }


    //inefficent
    for i in nums.iter() {
        for j in nums.iter() {
            if i + j == 2020  {
                println!("{}, {} =  {}",i,j , i*j);
            }
        }
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
