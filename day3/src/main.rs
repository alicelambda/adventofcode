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
    let mut x = 0;
    let mut y = 0;
    let mut ntrees = 0;
    loop {
        x = (x +  3) % trees[0].len();
        y += 1;

        if trees[y][x] {
            ntrees += 1;
        }

        println!("{} {} {}",x,y,ntrees);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
