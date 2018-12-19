// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::prelude::*;
#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
struct Runner {
    reg: [i64; 6],
}

impl Runner {
    fn new() -> Runner {
        Runner { reg: [0i64; 6] }
    }

    fn addi(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = newrun.reg[a as usize] + b;
        newrun
    }

    fn addr(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = newrun.reg[a as usize] + newrun.reg[b as usize];
        newrun
    }

    fn muli(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = newrun.reg[a as usize] * b;
        newrun
    }

    fn mulr(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = newrun.reg[a as usize] * newrun.reg[b as usize];
        newrun
    }

    fn bani(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = newrun.reg[a as usize] & b;
        newrun
    }

    fn banr(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = newrun.reg[a as usize] & newrun.reg[b as usize];
        newrun
    }

    fn bori(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = newrun.reg[a as usize] | b;
        newrun
    }

    fn borr(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = newrun.reg[a as usize] | newrun.reg[b as usize];
        newrun
    }

    fn seti(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = a;
        newrun
    }

    fn setr(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = newrun.reg[a as usize];
        newrun
    }

    fn gtir(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = if a > newrun.reg[b as usize] { 1 } else { 0 };
        newrun
    }

    fn gtri(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = if newrun.reg[a as usize] > b { 1 } else { 0 };
        newrun
    }

    fn gtrr(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = if newrun.reg[a as usize] > newrun.reg[b as usize] {
            1
        } else {
            0
        };
        newrun
    }

    fn eqir(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = if a == newrun.reg[b as usize] { 1 } else { 0 };
        newrun
    }

    fn eqri(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = if newrun.reg[a as usize] == b { 1 } else { 0 };
        newrun
    }

    fn eqrr(&self, a: i64, b: i64, c: i64) -> Runner {
        let mut newrun = self.clone();
        newrun.reg[c as usize] = if newrun.reg[a as usize] == newrun.reg[b as usize] {
            1
        } else {
            0
        };
        newrun
    }

    fn run_op(&self, op: usize, a: i64, b: i64, c: i64) -> Runner {
        match op {
            0 => self.addi(a, b, c),
            1 => self.addr(a, b, c),
            2 => self.muli(a, b, c),
            3 => self.mulr(a, b, c),
            4 => self.bori(a, b, c),
            5 => self.borr(a, b, c),
            6 => self.bani(a, b, c),
            7 => self.banr(a, b, c),
            8 => self.seti(a, b, c),
            9 => self.setr(a, b, c),
            10 => self.gtir(a, b, c),
            11 => self.gtri(a, b, c),
            12 => self.gtrr(a, b, c),
            13 => self.eqir(a, b, c),
            14 => self.eqri(a, b, c),
            15 => self.eqrr(a, b, c),
            _ => panic!("UNKNOWN OP {}", op),
        }
    }

    fn op_is_valid(&self, target: &Runner, op: usize, params: &[i64; 3]) -> bool {
        let next = self.run_op(op, params[0], params[1], params[2]);
        *target == next
    }
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

fn main() {
    let ops = hashmap!["addi" => 0, "addr" => 1, "muli" => 2, "mulr" => 3, "bori" => 4, "borr" => 5, "bani" => 6, "banr" => 7, "seti" => 8, "setr" => 9, "gtir" => 10, "gtri" => 11, "gtrr" => 12, "eqir" => 13, "eqri" => 14, "eqrr" => 15];
    let stdin = io::stdin();
    let mut text = String::new();
    let mut tempv = [0i64; 3];
    let mut op: usize;
    let ip: usize;
    stdin.read_line(&mut text);
    scan!(text.bytes() => "#ip {}",ip);
    let values: Vec<(String, [i64; 3])> = stdin
        .lock()
        .lines()
        .map(|x| {
            let mut tempv = [0i64; 3];
            let mut text = String::new();
            let v = x.unwrap();
            scan!(v.bytes() => "{} {} {} {}",text, tempv[0], tempv[1], tempv[2]);
            (text, tempv)
        }).collect();
    let mut runner = Runner::new();
    let mut curinst = 0;

    while curinst >= 0 && curinst < (values.len() as i64) {
        runner.reg[ip] = curinst;
        let inst = &values[curinst as usize];
        runner = runner.run_op(
            ops[(inst.0).as_str()],
            (inst.1)[0],
            (inst.1)[1],
            (inst.1)[2],
        );
        curinst = runner.reg[ip] + 1;
    }

    println!("{:?}", runner);
    let target = runner.reg[1];
    let result: i64 = (1..=target).filter(|x| (target % x) == 0).sum();
    println!("{:?} {}", result, target);
    let mut runner = Runner::new();
    let mut curinst = 0;
    runner.reg[0] = 1;
    for _ in 0..100 {
        runner.reg[ip] = curinst;
        let inst = &values[curinst as usize];
        runner = runner.run_op(
            ops[(inst.0).as_str()],
            (inst.1)[0],
            (inst.1)[1],
            (inst.1)[2],
        );
        curinst = runner.reg[ip] + 1;
    }
    let target = runner.reg[1];
    let result: i64 = (1..=target).filter(|x| target % x == 0).sum();
    println!("{:?} {}", result, target);
}
