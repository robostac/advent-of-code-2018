// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::prelude::*;
#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
struct Runner {
    reg: [i64; 4],
}

impl Runner {
    fn new() -> Runner {
        Runner { reg: [0i64; 4] }
    }

    fn new_init(a: i64, b: i64, c: i64, d: i64) -> Runner {
        Runner { reg: [a, b, c, d] }
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
fn parser(stdin: &io::Stdin) -> Result<(Runner, usize, [i64; 3], Runner), Box<Error>> {
    let mut a: i64;
    let mut b: i64;
    let mut c: i64;
    let mut d: i64;
    let mut text = String::new();
    let mut tempv = [0i64; 3];
    let mut op: usize;
    stdin.read_line(&mut text);
    try_scan!(text.bytes() => "Before: [{}, {}, {}, {}]",  a, b, c, d );
    let init_run = Runner::new_init(a, b, c, d);
    text = String::new();
    stdin.read_line(&mut text);
    scan!(text.bytes() => "{} {} {} {}", op, tempv[0], tempv[1], tempv[2]);
    text = String::new();
    stdin.read_line(&mut text);
    scan!(text.bytes() => "After:  [{}, {}, {}, {}]",  a, b, c, d );
    let final_run = Runner::new_init(a, b, c, d);
    stdin.read_line(&mut text);
    Ok((init_run, op, tempv, final_run))
}
fn main() {
    let stdin = io::stdin();
    let mut invalid = [0; 16];
    let mut total = 0;
    loop {
        //println!("{:?}", parser(&s));
        let v = parser(&stdin);
        match v {
            Ok(a) => {
                let mut count = 0;
                for x in 0..16 {
                    if (a.0).op_is_valid(&a.3, x, &a.2) {
                        count += 1;
                    } else {
                        invalid[a.1] |= (1u16 << x);
                    }
                }
                if count >= 3 {
                    total += 1;
                }
            }
            _ => break,
        }
    }
    println!("{}", total);
    let mut mapping = [0usize; 16];
    for _ in 0..16 {
        let mut pos = 0;
        let mut target = 0;
        for x in 0..16 {
            let possiblities: Vec<(usize, &u16)> = invalid
                .iter()
                .enumerate()
                .filter(|y| (y.1 & (1 << x)) == 0)
                .collect();
            if (possiblities.len() == 1) {
                pos = possiblities[0].0;
                target = x;
                break;
            }
        }
        invalid[pos] = 0xFFFF;
        mapping[pos] = target;
    }
    let mut runner = Runner::new();
    let mut s = String::new();
    stdin.read_line(&mut s);
    let values: Vec<[i64; 4]> = stdin
        .lock()
        .lines()
        .map(|x| {
            let mut tempv = [0i64; 4];
            let v = x.unwrap();
            scan!(v.bytes() => "{} {} {} {}", tempv[0], tempv[1], tempv[2], tempv[3]);
            tempv
        }).collect();
    for v in values.iter() {
        runner = runner.run_op(mapping[v[0] as usize], v[1], v[2], v[3]);
    }
    println!("{:?}", runner);
}
