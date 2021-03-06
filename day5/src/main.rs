use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn main() {
    let mut ids = Vec::new();
    let mut max = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(seat) = line {
                let rowstr = &seat[0..=6];
                let colstr = &seat[6..=9];
                let row = seat_row(&seat[0..=6]);
                let col = seat_col(&seat[7..=9]);
                let id = calc_seat_id(row,col);
                ids.push(id);
            }
        }
    }
    ids.sort();
    for (i,n) in ids.iter().enumerate() {
        if ids[i] != 89 + i as i32{
            println!("{}",89+i);
        }
    }
    println!("{:?}",ids);
}


fn seat_row(seat: &str) -> i32 {
    let mut add = 64;
    let mut num = 0;
    for c in seat.chars() {
        if c == 'B' {
            num += add;
        }
        add /= 2;
    }
    num
}

fn seat_col(seat: &str) -> i32 {
    let mut add = 4;
    let mut num = 0;
    for c in seat.chars() {
        if c == 'R' {
            num += add;
        }
        add /= 2;
    }
    num
}

fn calc_seat_id(row: i32, col: i32) -> i32 {
    row *8 + col
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn seat_test() {
        assert_eq!(seat_row("FBFBBFF"),44)
    }

    #[test]
    fn seat_testcol() {
        assert_eq!(seat_col("RLR"),5);
    }

    #[test]
    fn test_id() {
        assert_eq!(calc_seat_id(44,5),357);
    }




}
