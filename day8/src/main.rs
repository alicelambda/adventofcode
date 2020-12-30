use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

struct Vm {
    ip: i64,
    acc: i64,
    instructions: Vec<InsName>
}

#[derive(Debug)]
enum InsName {
    Nop,
    Acc(i64),
    Jmp(i64)
}
fn main() {
    let mut ins = Vec::new();
    if let Ok(lines) = read_lines("./test") {
        for line in lines {
            if let Ok(instruction) = line {
                let splits = instruction.split(" ").collect::<Vec<&str>>();
                ins.push(parse_ins(&instruction));
            }
        }
    }
    let machine = Vm {
        ip: 0,
        acc: 0,
        instructions: ins,
    };
}

fn parse_ins(string :&str) -> InsName {
    let splits = string.split(" ").collect::<Vec<&str>>();
    match splits[0] {
        "nop" => InsName::Nop,
        "acc" => {
            let num = parse_num(splits[1]);
            InsName::Acc(num)
        },
        "jmp" => {
            let num = parse_num(splits[1]);
            InsName::Jmp(num)
        },
        _ => {
            println!("{}",splits[0]);
            panic!("couldn't parse instruction")
        }
    }
}

fn parse_num(string: &str) -> i64 {
    let mut num = string.to_string();
    let sign = num.remove(0);
    match sign {
        '+' => num.parse::<i64>().unwrap(),
        '-' => -num.parse::<i64>().unwrap(),
        _ => panic!("couln't parse ins"),
    }
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())

}