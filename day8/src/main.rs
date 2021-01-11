use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
struct Vm {
    ip: i64,
    acc: i64,
    instructions: Vec<InsName>,

}

#[derive(Debug, Clone)]
enum InsName {
    Nop(i64),
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
    let mut ind = 0;
    let mut swapped = false;
    loop {
        loop {
            if let InsName::Acc(a) = ins[ind] {
                ind+=1;
            } else {
                break
            }
        }
        let mut machine = Vm {
            ip: 0,
            acc: 0,
            instructions: ins.clone(),
        };
        if does_halt(&machine) {
            println!("{}",&machine.acc);
            break
        }
        if !swapped {
            if let InsName::Jmp(a) = ins[ind] {
                ins[ind] = InsName::Nop(a);
            } 
            if let InsName::Nop(a) = ins[ind] {
                ins[ind] = InsName::Jmp(a);
            }
            swapped = true;
            ind += 1;
        }
    }
}

fn does_halt(mut machine: &Vm) -> bool{
    let mut used = HashMap::new();
    loop {
        if let Some(i) = used.get(&machine.ip) {
            return true
        } else {
            used.insert(machine.ip,1);
        }
        if step(&mut machine) {
            return false
        }
    }
}

fn step(vm : &mut Vm) -> bool {
    let cur = &vm.instructions[vm.ip as usize];
    match cur {
        InsName::Nop(x) => {
            vm.ip = vm.ip + 1;
        },
        InsName::Jmp(x) => {
            vm.ip = vm.ip + x;
        },
        InsName::Acc(x) => { 
            vm.acc = vm.acc + x;
            vm.ip = vm.ip + 1;
        },
    }
    if vm.ip > vm.instructions.len().try_into().unwrap() {
        return true
    }
    false
}


fn parse_ins(string :&str) -> InsName {
    let splits = string.split(" ").collect::<Vec<&str>>();
    match splits[0] {
        "nop" => {
            let num = parse_num(splits[1]);
            InsName::Nop(num)
        },
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
