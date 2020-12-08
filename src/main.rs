use clap::Clap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[macro_use]
extern crate lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
enum Operation {
    NOP(i32),
    ACC(i32),
    JMP(i32),
}

#[derive(Default, Debug)]
struct Computer {
    operations: Vec<(Operation, bool)>,
    current: usize,
    accumulator: i32,
}

impl Computer {
    fn run_till_loop(&mut self) -> (i32, bool) {
        loop {
            println!("Op[{}] = {:?}", self.current, self.operations[self.current]);
            self.operations[self.current].1 = true;
            match self.operations[self.current].0 {
                Operation::NOP(_) => {
                    self.operations[self.current].1 = true;
                    self.current += 1;
                }
                Operation::ACC(d) => {
                    self.accumulator += d;
                    self.current += 1;
                }
                Operation::JMP(d) => {
                    self.current = (self.current as i32 + d) as usize;
                }
            }
            if self.current == self.operations.len() {
                return (self.accumulator, true);
            }
            if self.operations[self.current].1 {
                return (self.accumulator, false);
            }
        }
    }
}

#[derive(Clap)]
struct Opts {
    part: i32,
    input: String,
}

fn main() {
    let opts: Opts = Opts::parse();
    let mut computer = boot_computer(opts.input);
    if opts.part == 1 {
        println!("Output {}", computer.run_till_loop().0);
    } else {
        for ii in 0..computer.operations.len() {
            let mut new_ops = computer.operations.to_vec();
            match new_ops[ii].0 {
                Operation::NOP(d) => {
                    new_ops[ii].0 = Operation::JMP(d);
                }
                Operation::JMP(d) => {
                    new_ops[ii].0 = Operation::NOP(d);
                }
                _ => continue,
            }

            let (val, term) = Computer {
                operations: new_ops,
                ..Default::default()
            }
            .run_till_loop();

            if term {
                println!("Val {}", val);
                break;
            }
        }
    }
}

fn boot_computer(filename: String) -> Computer {
    let mut operations = vec![];

    lazy_static! {
        static ref OP_RE: Regex = Regex::new(r"^(\w{3}) (\S)(\d+)$").unwrap();
    }

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(line_as_string) = line {
                let op_cap = OP_RE.captures_iter(&line_as_string).next().unwrap();
                let d = op_cap[3].parse::<i32>().unwrap() * if &op_cap[2] == "+" { 1 } else { -1 };
                let op = match &op_cap[1] {
                    "nop" => Operation::NOP(d),
                    "acc" => Operation::ACC(d),
                    "jmp" => Operation::JMP(d),
                    invalid => panic!("Invalid opcode {}", invalid),
                };
                operations.push((op, false));
            }
        }
    }

    Computer {
        operations,
        ..Default::default()
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
