use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
    let mut trees  = Vec::new();

    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(tree) = line {
                let mut treeline  =  Vec::new();
                for i in tree.chars() {
                    if i == '#' {
                        treeline.push(true);
                    } else {
                        treeline.push(false);
                    }
                }
                trees.push(treeline);
            }
        }
    }

    println!("{}",trees[0].len());
    let yslopes = vec![1,1,1,1,2];
    let xslopes = vec![1,3,5,7,1];
    let mut slopindex = 0;
    let mut twees = Vec::new();
    println!("{}", trees.len());
    loop {
        let mut x = 0;
        let mut y = 0;
        let mut ntrees = 0;
        loop {
            x = (x +  xslopes[slopindex]) % trees[0].len();
            y = (y + yslopes[slopindex]);

            if y >= trees.len()  {
                twees.push(ntrees);
                break;
            }
            if trees[y][x] {
                ntrees += 1;
            }
            println!("{} {} {}",x,y,ntrees);
        }
        slopindex+=1;
        println!("{:?}",twees);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
