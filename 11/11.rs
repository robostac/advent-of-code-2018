// cargo-deps: text_io
#[macro_use]
extern crate text_io;

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
fn power(x: i64, y: i64, serial: i64) -> i64 {
    let rack_id = x + 10;
    let mut plvl = y * rack_id;
    plvl += serial;
    plvl *= rack_id;
    ((plvl % 1000) / 100) - 5
}
struct solver {
    ma: HashMap<(usize, usize, usize), i64>,
    max: i64,
    best: (usize, usize, usize),
    serial: i64,
}

impl solver {
    fn solve(&mut self, x: usize, y: usize, size: usize) -> i64 {
        if 1 == size {
            *self
                .ma
                .entry((x, y, size))
                .or_insert(power(x as i64, y as i64, self.serial))
        } else {
            let v: i64;
            if self.ma.contains_key(&(x, y, size)) {
                v = self.ma[&(x, y, size)]
            } else {
                let mut m = self.solve(x, y, 1);
                m += self.solve(x + 1, y, size - 1);
                self.ma.insert((x, y, size), m);
                v = m;
            }
            v
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let serial: i64;
    let mut s = String::new();
    stdin.read_line(&mut s);
    scan!(s.bytes() => "{}", serial);
    let mut ma: HashMap<(usize, usize, usize), i64> = HashMap::new();

    let mut max_pwr = -100000;
    let mut slv = solver {
        ma: ma,
        max: -100000,
        best: (0, 0, 0),
        serial,
    };
    for sz in 1..=300 {
        for x in 1..=(301 - sz) {
            for y in 1..=(301 - sz) {
                let mut v = 0;
                for yy in y..y + sz {
                    v += slv.solve(x, yy, sz);
                }
                if v > slv.max {
                    slv.max = v;
                    slv.best = (x, y, sz);
                    println!("{:?} {}", slv.best, slv.max);
                }
            }
        }
    }
}
