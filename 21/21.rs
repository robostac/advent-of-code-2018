// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::io::prelude::*;
#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
struct Runner {
    reg: [usize; 6],
}

impl Runner {
    fn new() -> Runner {
        Runner { reg: [0usize; 6] }
    }

    fn addi(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = self.reg[a] + b;
    }

    fn addr(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = self.reg[a] + self.reg[b];
    }

    fn muli(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = self.reg[a] * b;
    }

    fn mulr(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = self.reg[a] * self.reg[b];
    }

    fn bani(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = self.reg[a] & b;
    }

    fn banr(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = self.reg[a] & self.reg[b];
    }

    fn bori(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = self.reg[a] | b;
    }

    fn borr(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = self.reg[a] | self.reg[b];
    }

    fn seti(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = a;
    }

    fn setr(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = self.reg[a];
    }

    fn gtir(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = if a > self.reg[b] { 1 } else { 0 };
    }

    fn gtri(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = if self.reg[a] > b { 1 } else { 0 };
    }

    fn gtrr(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = if self.reg[a] > self.reg[b] { 1 } else { 0 };
    }

    fn eqir(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = if a == self.reg[b] { 1 } else { 0 };
    }

    fn eqri(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = if self.reg[a] == b { 1 } else { 0 };
    }

    fn eqrr(&mut self, a: usize, b: usize, c: usize) {
        self.reg[c] = if self.reg[a] == self.reg[b] { 1 } else { 0 };
    }

    fn run_op(&mut self, op: usize, a: usize, b: usize, c: usize) {
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

    fn op_is_valid(&mut self, target: &Runner, op: usize, params: &[usize; 3]) -> bool {
        self.run_op(op, params[0], params[1], params[2]);
        *target == *self
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
    let ip: usize;
    stdin.read_line(&mut text);
    scan!(text.bytes() => "#ip {}",ip);
    let values: Vec<(usize, [usize; 3])> = stdin
        .lock()
        .lines()
        .map(|x| {
            let mut tempv = [0usize; 3];
            let mut text = String::new();
            let v = x.unwrap();
            scan!(v.bytes() => "{} {} {} {}",text, tempv[0], tempv[1], tempv[2]);
            (ops[text.as_str()], tempv)
        }).collect();

    let mut runner = Runner::new();
    let mut curinst = 0;
    runner.reg[0] = 0;

    let mut v = HashSet::new();
    let mut prev = 0;
    let mut first = true;
    while curinst < (values.len()) {
        runner.reg[ip] = curinst;
        let inst = &values[curinst];
        if inst.0 == 15 {
            if (first) {
                first = false;
                println!("{} {:?}", runner.reg[(inst.1)[0]], runner,);
            }
            if v.contains(&runner.reg[(inst.1)[0]]) {
                println!("{} {:?} {}", runner.reg[(inst.1)[0]], runner, prev);
                break;
            }
            v.insert(runner.reg[(inst.1)[0]]);
            prev = runner.reg[(inst.1)[0]];
        }
        runner.run_op((inst.0), (inst.1)[0], (inst.1)[1], (inst.1)[2]);
        curinst = runner.reg[ip] + 1;
    }
}
